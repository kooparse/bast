use crate::models::{
    schema::{pageviews, stats, websites},
    CmpStat, Pageview, Stat, Website,
};
use crate::utils::{is_belongs_to_domain, is_from_localhost, Db};
use actix_web::{
    error::BlockingError, error::Error as ActixError, web, HttpRequest,
    HttpResponse,
};
use chrono::{Datelike, Utc};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::Error as DbError;
use serde::Deserialize;
use sha2::{Digest, Sha256};

// Error wrapper to send actix errors on other threads.
#[derive(Debug)]
struct SendError {
    pub inner: HttpResponse,
}

impl From<HttpResponse> for SendError {
    fn from(response: HttpResponse) -> Self {
        Self { inner: response }
    }
}

unsafe impl Send for SendError {}

// User means number of unique visitors.
// If someone refresh 10 times the page, it will still be one user.
//
// Session means the number of pageviews that start after 30 minutes of inactivity.
#[derive(Deserialize, Insertable, Debug, Clone)]
#[table_name = "pageviews"]
pub struct Data {
    website_id: i32,
    href: String,
    #[serde(default)]
    hostname: String,
    #[serde(default)]
    pathname: String,
    #[serde(default)]
    referrer: String,

    // Get by headers.
    #[serde(skip)]
    user_agent: String,
    #[serde(skip)]
    location: Option<String>,
    #[serde(skip)]
    is_new_session: bool,
    #[serde(skip)]
    is_new_user: bool,
    #[serde(skip)]
    u_id: String,
}

pub async fn collect(
    req: HttpRequest,
    params: web::Query<Data>,
    data: web::Data<Db>,
) -> Result<HttpResponse, ActixError> {
    let conn = data.conn_pool()?;
    let mut params = params.into_inner();
    let req_info = req.connection_info();

    // Do not collect if DNT is here.
    if req.headers().contains_key("DNT") {
        return Ok(HttpResponse::Ok().finish());
    }

    // Do not collect if it comes from localhost.
    if is_from_localhost(&params.hostname) {
        return Ok(HttpResponse::Ok().finish());
    }

    // Do not collect if it comes from prefetch/preview.
    if let Some(h) = req.headers().get("X-Moz") {
        if h == "prefetch" {
            return Ok(HttpResponse::Ok().finish());
        }
    }

    // Same (for safari).
    if let Some(h) = req.headers().get("X-Purpose") {
        if h == "preview" {
            return Ok(HttpResponse::Ok().finish());
        }
    }

    // Construct user id from ip address and user agent.
    if let Some(ua) = req.headers().get("User-Agent") {
        params.user_agent = ua
            .to_str()
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?
            .to_owned();

        // Hash made from concatenation of host with user agent.
        let hash = Sha256::new()
            // Hash host/ip.
            .chain(req_info.host().as_bytes())
            // Hash user agent.
            .chain(&params.user_agent)
            .finalize();

        // Encode 16 first characters.
        params.u_id = hex::encode(&hash)[..16].to_owned();
    } else {
        eprintln!("No User-Agent found.");
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let mut is_new_user = false;
    let mut is_new_session = false;
    let mut duration = 0.;

    web::block(move || -> Result<_, SendError> {
        let mut website: Website = websites::table
            .find(params.website_id)
            .get_result(&conn)
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::NotFound().body("Website does not exists.")
            })?;

        let is_belongs = is_belongs_to_domain(&website.domain, &params.href)
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        if !is_belongs {
            return Err(SendError::from(HttpResponse::Forbidden().body(
                "Current request doesn't belongs to associated website.",
            )));
        }

        let last_pageview = pageviews::table
            .filter(
                pageviews::u_id
                    .eq(&params.u_id)
                    .and(pageviews::website_id.eq(params.website_id))
                    .and(not(pageviews::is_done)),
            )
            .order(pageviews::created_at.desc())
            .get_result::<Pageview>(&conn);

        let new_date = match last_pageview {
            // If we found an pageview, we have to compute
            Ok(mut pageview) => {
                let elapsed = Utc::now()
                    .naive_utc()
                    .signed_duration_since(pageview.created_at)
                    .to_std()
                    .expect("Error whild converting chrono time to std.");
                // 1800 secs means 30 minutes.
                is_new_session = elapsed.as_secs() >= 1800;
                is_new_user = false;

                if !is_new_session {
                    duration = elapsed.as_secs_f32();
                    pageview.duration = duration;
                }

                pageview.is_done = true;
                update(pageviews::table)
                    .filter(pageviews::id.eq(pageview.id))
                    .set(&pageview)
                    .execute(&conn)
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    })?;

                params.is_new_session = is_new_session;
                params.is_new_user = is_new_user;

                insert_into(pageviews::table)
                    .values(&params)
                    .get_result(&conn)
                    .map(|pv: Pageview| pv.created_at)
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    })
            }
            // Not found means that it's a new user, so we just add it with
            // default values.
            Err(DbError::NotFound) => {
                is_new_session = true;
                is_new_user = true;

                params.is_new_user = is_new_user;
                params.is_new_session = is_new_session;

                insert_into(pageviews::table)
                    .values(&params)
                    .get_result(&conn)
                    .map(|pv: Pageview| pv.created_at)
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    })
            }

            _ => Err(HttpResponse::InternalServerError().finish()),
        }
        .map_err(SendError::from)?;

        // Now we want to compute analytics over time.
        //
        // All global analytics are stored directly on the website table,
        // so we firstly computes and updates it.
        // We're gonna do the same for the daily ones.
        //
        // Website and Stat implements all the CmpStat trait.
        //
        // TODO: We should parallelize database calls.
        website.cmp(is_new_user, is_new_session, duration);
        update(websites::table)
            .filter(websites::id.eq(website.id))
            .set(&website)
            .execute(&conn)
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        // We want to fetch the latest day stored in the database
        // Then checked if the current pageview is in the same day. If so,
        // we update it with new data, otherwise we create a new one.
        let last_day: Option<Stat> = stats::table
            .filter(stats::website_id.eq(website.id))
            .order(stats::created_at.desc())
            .first(&conn)
            .ok()
            .filter(|day: &Stat| {
                day.created_at.year() == new_date.year()
                    && day.created_at.month() == new_date.month()
                    && day.created_at.day() == new_date.day()
            })
            .map(|mut day| {
                day.cmp(is_new_user, is_new_session, duration);
                day
            });

        if let Some(day) = last_day {
            update(stats::table)
                .filter(stats::id.eq(day.id))
                .set(day)
                .execute(&conn)
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
        } else {
            insert_into(stats::table)
                .values((
                    stats::website_id.eq(website.id),
                    stats::pageviews.eq(1),
                    stats::users.eq(1),
                    stats::sessions.eq(1),
                ))
                .execute(&conn)
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
        }

        Ok(())
    })
    .await
    .map_err(|err| match err {
        BlockingError::Error(e) => e.inner,
        BlockingError::Canceled => HttpResponse::InternalServerError().finish(),
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use actix_web::{http::StatusCode, test, web, App};

    fn make_collect_uri(
        website_id: i32,
        href: &str,
        pathname: &str,
        hostname: &str,
        referrer: &str,
    ) -> String {
        format!(
            "/collect?website_id={}&href={}&pathname={}&hostname={}&referrer={}", 
            website_id,
            href,
            pathname,
            hostname,
            referrer
        )
    }

    fn config(cfg: &mut web::ServiceConfig) {
        let db = Db::new();
        let conn = db.conn_pool().expect("Failed to connect to database.");
        utils::seed_database(&conn);

        cfg.data(Db::new());
        cfg.route("/collect", web::get().to(collect));
    }

    #[actix_web::test]
    async fn check_collect() {
        let db = Db::new();
        let conn = db.conn_pool().expect("Failed to connect to database.");

        let user_agent = "supertest";
        let uri = make_collect_uri(
            1,
            "https://google.com",
            "/about",
            "google.com",
            "https://duckduckgo.com",
        );

        let mut app = test::init_service(App::new().configure(config)).await;

        //
        // Check when totally new user.
        //
        let req = test::TestRequest::get()
            .header("User-Agent", user_agent)
            .uri(&uri)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let pageview: Result<Pageview, _> = pageviews::table
            .filter(pageviews::website_id.eq(1))
            .get_result(&conn);

        assert_eq!(pageview.is_ok(), true);

        let website: Website =
            websites::table.find(1).get_result(&conn).unwrap();

        assert_eq!(website.users, 1);
        assert_eq!(website.sessions, 1);
        assert_eq!(website.pageviews, 1);
        assert_eq!(website.time_counter, 0);

        //
        // Check when same_user but <= 30min.
        //
        let req = test::TestRequest::get()
            .header("User-Agent", user_agent)
            .uri(&uri)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let website: Website =
            websites::table.find(1).get_result(&conn).unwrap();

        assert_eq!(website.users, 1);
        assert_eq!(website.sessions, 1);
        assert_eq!(website.pageviews, 2);
        // Because < 5 secs.
        assert_eq!(website.time_counter, 0);
        assert_eq!(website.avg_time, 190.);

        //
        // Check when same_user but >= 30min.
        //
        // To trick this, we're gonna change the created_at from the
        // latest pageview, and subtract 30min.
        let pageview: Pageview = pageviews::table
            // Currently, there is just two pageview, and the latest is not
            // done yet.
            .filter(
                pageviews::website_id
                    .eq(1)
                    .and(pageviews::is_done.eq(false)),
            )
            .get_result(&conn)
            .unwrap();

        update(pageviews::table)
            .filter(pageviews::id.eq(pageview.id))
            .set(
                pageviews::created_at
                    .eq(pageview.created_at - chrono::Duration::minutes(30)),
            )
            .execute(&conn)
            .expect("Failed to trick time.");

        let req = test::TestRequest::get()
            .header("User-Agent", user_agent)
            .uri(&uri)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let website: Website =
            websites::table.find(1).get_result(&conn).unwrap();

        assert_eq!(website.users, 1);
        assert_eq!(website.sessions, 2);
        assert_eq!(website.pageviews, 3);
        assert_eq!(website.time_counter, 0);
        assert_eq!(website.avg_time, 190.);

        //
        // Check when different user.
        //
        let req = test::TestRequest::get()
            .header("User-Agent", format!("{}_2", user_agent))
            .uri(&uri)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let website: Website =
            websites::table.find(1).get_result(&conn).unwrap();

        assert_eq!(website.users, 2);
        assert_eq!(website.sessions, 3);
        assert_eq!(website.pageviews, 4);
        assert_eq!(website.time_counter, 0);
        assert_eq!(website.avg_time, 190.);
    }
}

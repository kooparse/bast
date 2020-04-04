use crate::models::{
    schema::{day_stats, month_stats, pageviews, websites},
    DayStat, MonthStat, Pageview, Website, 
};
use crate::utils::Db;
use crate::utils::{to_client, UserError};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Datelike, Utc};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Deserialize;

// User means number of unique visitors.
// If someone refresh 10 times the page, it will still be one user.
//
// Session means the number of pageviews that start after 30 minutes of inactivity.
#[derive(Deserialize, Insertable, Debug, Clone)]
#[table_name = "pageviews"]
pub struct Data {
    website_id: i32,
    href: String,
    hostname: String,
    pathname: String,
    referrer: String,
    #[serde(skip)]
    is_new_session: bool,
    #[serde(skip)]
    is_new_user: bool,
    #[serde(skip)]
    is_bounce: bool,
    #[serde(skip)]
    u_id: String,
}

pub async fn collect(
    req: HttpRequest,
    params: Option<web::Query<Data>>,
    data: web::Data<Db>,
) -> Result<HttpResponse, UserError> {
    let c_info = req.connection_info();
    let host = c_info.host();
    // TODO: Remove unwrap.
    let agent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let u_id = format!("{}_{}", &host, agent);

    if params.is_none() {
        return Err(UserError::Unauthorized);
    }

    let res = web::block(move || -> Result<(), UserError> {
        let conn = &data.conn_pool()?;
        let mut params = params.unwrap().into_inner();
        // TODO: Salt u_id.
        params.u_id = u_id;

        let mut website: Website = websites::table
            .find(params.website_id)
            .get_result(conn)
            .map_err(|_| UserError::NotFound)?;

        // Get the last unfinished pageview.
        let result = pageviews::table
            .filter(
                pageviews::u_id
                    .eq(&params.u_id)
                    .and(pageviews::website_id.eq(params.website_id))
                    .and(not(pageviews::is_done)),
            )
            .order(pageviews::created_at.desc())
            .get_result::<Pageview>(conn);

        let new_date = match result {
            // If we found an pageview, we have to compute
            Ok(mut pageview) => {
                let elapsed = Utc::now()
                    .naive_utc()
                    .signed_duration_since(pageview.created_at)
                    .to_std()
                    .expect("Error whild converting chrono time to std.");
                // 1800 secs means 30 minutes.
                let is_new_session = elapsed.as_secs() >= 1800;

                if !is_new_session {
                    pageview.duration = elapsed.as_secs_f32();
                    pageview.is_bounce = false;

                    website.known_time_counter += 1;
                    website.avg_time = (website.avg_time + pageview.duration)
                        / website.known_time_counter as f32;
                } else {
                    website.sessions += 1;
                }

                pageview.is_done = true;
                update(pageviews::table)
                    .filter(pageviews::id.eq(pageview.id))
                    .set(&pageview)
                    .execute(conn)
                    .map_err(|_| UserError::InternalServerError)?;

                params.is_new_session = is_new_session;
                params.is_bounce = is_new_session;
                params.is_new_user = false;
                let r: Pageview = insert_into(pageviews::table)
                    .values(&params)
                    .get_result(conn)
                    .map_err(|_| UserError::InternalServerError)?;

                Ok(r.created_at)
            }
            // Not found means that it's a new user, so we just add it with
            // default values.
            Err(Error::NotFound) => {
                website.users += 1;
                website.sessions += 1;

                params.is_new_user = true;
                params.is_new_session = true;
                params.is_bounce = true;

                let r: Pageview = insert_into(pageviews::table)
                    .values(&params)
                    .get_result(conn)
                    .map_err(|_| UserError::InternalServerError)?;

                Ok(r.created_at)
            }

            _ => Err(UserError::InternalServerError),
        }?;

        update(websites::table)
            .filter(websites::id.eq(website.id))
            .set(&website)
            .execute(conn)
            .map_err(|_| UserError::InternalServerError)?;

        let mut last_month: Option<MonthStat> = month_stats::table
            .filter(month_stats::website_id.eq(website.id))
            .order(month_stats::created_at.desc())
            .first(conn)
            .ok()
            .filter(|last: &MonthStat| {
                last.created_at.year() == new_date.year()
                    && last.created_at.month() == new_date.month()
            });

        let mut last_day: Option<DayStat> = day_stats::table
            .filter(day_stats::website_id.eq(website.id))
            .order(day_stats::created_at.desc())
            .first(conn)
            .ok()
            .filter(|last: &DayStat| {
                last.created_at.year() == new_date.year()
                    && last.created_at.month() == new_date.month()
                    && last.created_at.day() == new_date.day()
            });

        // TODO: Cleanup this ugly logic...
        if let Some(mut last) = last_month.take() {
            last.users = website.users;
            last.sessions = website.sessions;
            last.avg_time = website.avg_time;
            last.bounce_rate = website.bounce_rate;

            update(month_stats::table)
                .filter(month_stats::id.eq(last.id))
                .set(last)
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;

            return Ok(());
        } else {
            insert_into(month_stats::table)
                .values((
                    month_stats::website_id.eq(website.id),
                    month_stats::users.eq(website.users),
                    month_stats::sessions.eq(website.sessions),
                    month_stats::avg_time.eq(website.avg_time),
                    month_stats::bounce_rate.eq(website.bounce_rate),
                ))
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        }

        if let Some(mut last) = last_day.take() {
            last.users = website.users;
            last.sessions = website.sessions;
            last.avg_time = website.avg_time;
            last.bounce_rate = website.bounce_rate;

            update(day_stats::table)
                .filter(day_stats::id.eq(last.id))
                .set(last)
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        } else {
            insert_into(day_stats::table)
                .values((
                    day_stats::website_id.eq(website.id),
                    day_stats::users.eq(website.users),
                    day_stats::sessions.eq(website.sessions),
                    day_stats::avg_time.eq(website.avg_time),
                    day_stats::bounce_rate.eq(website.bounce_rate),
                ))
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        }

        Ok(())
    })
    .await;

    to_client(res)
}

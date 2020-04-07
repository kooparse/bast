use crate::models::{
    schema::{day_stats, month_stats, pageviews, websites},
    DayStat, MonthStat, Pageview, CmpStat, Website,
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
    params: web::Query<Data>,
    data: web::Data<Db>,
) -> Result<HttpResponse, UserError> {
    let mut params = params.into_inner();

    let c_info = req.connection_info();
    let host = c_info.host();
    // TODO: Remove unwrap.
    let agent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    // TODO: Salt u_id.
    params.u_id = format!("{}_{}", &host, agent);

    let mut is_new_user = false;
    let mut is_new_session = false;
    let mut is_bounce = true;
    let mut duration = 0.;

    let res = web::block(move || -> Result<(), UserError> {
        let conn = &data.conn_pool()?;

        // Find the coresponding website.
        let mut website: Website = websites::table
            .find(params.website_id)
            .get_result(conn)
            .map_err(|_| UserError::NotFound)?;

        // Get the last unfinished pageview for this "user".
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
                is_new_session = elapsed.as_secs() >= 1800;
                is_bounce = is_new_session;
                is_new_user = false;

                if !is_new_session {
                    duration = elapsed.as_secs_f32();
                    pageview.duration = duration;
                    pageview.is_bounce = false;
                }

                pageview.is_done = true;
                update(pageviews::table)
                    .filter(pageviews::id.eq(pageview.id))
                    .set(&pageview)
                    .execute(conn)
                    .map_err(|_| UserError::InternalServerError)?;

                params.is_new_session = is_new_session;
                params.is_bounce = is_new_session;
                params.is_new_user = is_new_user;
                let r: Pageview = insert_into(pageviews::table)
                    .values(&params)
                    .get_result(conn)
                    .map_err(|_| UserError::InternalServerError)?;

                Ok(r.created_at)
            }
            // Not found means that it's a new user, so we just add it with
            // default values.
            Err(Error::NotFound) => {
                is_new_session = true;
                is_bounce = true;
                is_new_user = true;

                params.is_new_user = is_new_user;
                params.is_new_session = is_new_session;
                params.is_bounce = is_bounce;

                let r: Pageview = insert_into(pageviews::table)
                    .values(&params)
                    .get_result(conn)
                    .map_err(|_| UserError::InternalServerError)?;

                Ok(r.created_at)
            }

            _ => Err(UserError::InternalServerError),
        }?;

        // Now we want to compute analytics over time.
        //
        // All global analytics are stored directly on the website table, 
        // so we firstly computes and updates it. 
        // We're gonna do the same for monthly analytics and for the daily ones.
        //
        // Website, MonthStat and DayStat implements all the CmpStat trait.
        //
        // TODO: We should reconstruct the monthly data from the daily analytics.
        // TODO: Even so, we should parallelize those database call.
        //
        website.cmp(is_new_user, is_new_session, is_bounce, duration);
        update(websites::table)
            .filter(websites::id.eq(website.id))
            .set(&website)
            .execute(conn)
            .map_err(|_| UserError::InternalServerError)?;

        // We want to fetch the latest month stored in the database
        // Then checked if the current pageview is in the same month. If so, 
        // we update it with new data, otherwise we create a new one.
        let last_month: Option<MonthStat> = month_stats::table
            .filter(month_stats::website_id.eq(website.id))
            // Get the latest one.
            .order(month_stats::created_at.desc())
            // Get only one result.
            .first(conn)
            // Transform Result into Option type (Err = None).
            .ok()
            // If Option isn't None and don't pass this condition, we set it to None.
            .filter(|month: &MonthStat| {
                month.created_at.year() == new_date.year()
                    && month.created_at.month() == new_date.month()
            })
            // If even after previous filter, it's still Some, we mutate it to reflects our compute.
            .map(|mut month| {
                month.cmp(is_new_user, is_new_session, is_bounce, duration);
                month
            });

        // If we get something (correctly computed), we updates it.
        if let Some(month) = last_month {
            update(month_stats::table)
                .filter(month_stats::id.eq(month.id))
                .set(month)
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        } else {
            // If not, we insert a new one, we default values.
            insert_into(month_stats::table)
                .values((
                    month_stats::website_id.eq(website.id),
                    month_stats::pageviews.eq(1),
                    month_stats::users.eq(1),
                    month_stats::sessions.eq(1),
                ))
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        }

        //
        // Exact same logic for compute, update and insert days.
        // In a near future, we could only rely on days. 
        let last_day: Option<DayStat> = day_stats::table
            .filter(day_stats::website_id.eq(website.id))
            .order(day_stats::created_at.desc())
            .first(conn)
            .ok()
            .filter(|day: &DayStat| {
                day.created_at.year() == new_date.year()
                    && day.created_at.month() == new_date.month()
                    && day.created_at.day() == new_date.day()
            })
            .map(|mut day| {
                day.cmp(is_new_user, is_new_session, is_bounce, duration);
                day
            });

        if let Some(day) = last_day {
            update(day_stats::table)
                .filter(day_stats::id.eq(day.id))
                .set(day)
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        } else {
            insert_into(day_stats::table)
                .values((
                    day_stats::website_id.eq(website.id),
                    day_stats::pageviews.eq(1),
                    day_stats::users.eq(1),
                    day_stats::sessions.eq(1),
                ))
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        }

        Ok(())
    })
    .await;

    to_client(res)
}

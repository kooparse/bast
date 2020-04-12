use crate::models::{
    schema::{month_stats, pageviews, websites},
    AuthUser, MonthStat, Pageview, Website,
};
use crate::utils::{to_client, UserError};
use crate::Db;
use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Query {
    website_id: i32,
    start: i64,
    end: i64,
    by: String,
}

#[derive(Debug, Serialize)]
pub struct Referrer {
    name: String,
    count: i32,
}

#[derive(Debug, Serialize)]
pub struct Page {
    name: String,
    sessions: i32,
    users: i32,
}

#[derive(Serialize)]
pub struct Stats {
    website: Website,
    stats: Vec<MonthStat>,
    pages: Vec<Page>,
    referrers: Vec<Referrer>,
}

pub async fn get_stat(
    params: web::Query<Query>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<Stats, UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        if !["month", "day"].contains(&params.by.as_str()) {
            return Err(UserError::BadRequest);
        }

        let website: Website = websites::table
            .filter(
                websites::user_id
                    .eq(user_id)
                    .and(websites::id.eq(params.website_id)),
            )
            .first::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        let start = NaiveDateTime::from_timestamp(params.start, 0);
        let end = NaiveDateTime::from_timestamp(params.end, 0);

        let months: Vec<MonthStat> = month_stats::table
            .filter(
                month_stats::website_id
                    .eq(website.id)
                    .and(month_stats::created_at.gt(start))
                    .and(month_stats::created_at.lt(end)),
            )
            .get_results(&conn)
            .map_err(|_| UserError::InternalServerError)?;

        let pageview_list: Vec<Pageview> = pageviews::table
            .filter(
                pageviews::website_id
                    .eq(website.id)
                    .and(pageviews::created_at.gt(start))
                    .and(pageviews::created_at.lt(end)),
            )
            .get_results(&conn)
            .map_err(|_| UserError::InternalServerError)?;

        let mut referrers: Vec<Referrer> = vec![];
        let mut pages: Vec<Page> = vec![];

        pageview_list.iter().for_each(|pv| {
            let sessions = if pv.is_new_session { 1 } else { 0 };
            let users = if pv.is_new_user { 1 } else { 0 };

            if !pv.referrer.is_empty() {
                if let Some(mut r) =
                    referrers.iter_mut().find(|r| r.name == *pv.referrer)
                {
                    r.count += 1;
                } else {
                    referrers.push(Referrer {
                        name: pv.referrer.clone(),
                        count: 1,
                    })
                }
            }

            if let Some(mut p) =
                pages.iter_mut().find(|p| p.name == pv.pathname)
            {
                p.sessions += sessions;
                p.users += users;
            } else {
                pages.push(Page {
                    name: pv.pathname.clone(),
                    sessions,
                    users,
                })
            }
        });

        Ok(Stats {
            website,
            stats: months,
            pages,
            referrers,
        })
    })
    .await;

    to_client(result)
}

use crate::models::{
    schema::{pageviews, stats, websites},
    AuthUser, Page, Pageview, Referrer, SlimStat, Stat, Stats, Website,
};
use crate::utils::Db;
use actix_web::{error::Error as ActixError, web, HttpResponse};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::result::Error as DbError;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct Query {
    website_id: i32,
    start: i64,
    end: i64,
    by: String,
}

pub async fn get_stat(
    params: web::Query<Query>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, ActixError> {
    let conn = data.conn_pool()?;
    let user_id = auth_user.get_id()?;

    if !["month", "day"].contains(&params.by.as_str()) {
        return Ok(HttpResponse::BadRequest()
            .body("Parameter 'by' should contains 'month' or 'day'."));
    }

    let start = NaiveDateTime::from_timestamp(params.start, 0);
    let end = NaiveDateTime::from_timestamp(params.end, 0);

    let (website, days, pageviews): (Website, Vec<Stat>, Vec<Pageview>) =
        web::block(move || -> Result<_, DbError> {
            let website: Website = websites::table
                .filter(
                    websites::user_id
                        .eq(user_id)
                        .and(websites::id.eq(params.website_id)),
                )
                .get_result(&conn)?;

            let days = stats::table
                .filter(
                    stats::website_id
                        .eq(website.id)
                        .and(stats::created_at.gt(start))
                        .and(stats::created_at.lt(end)),
                )
                .get_results(&conn)?;

            // Retrieving all pages and referrers.
            let pageviews = pageviews::table
                .filter(
                    pageviews::website_id
                        .eq(website.id)
                        .and(pageviews::created_at.gt(start))
                        .and(pageviews::created_at.lt(end)),
                )
                .get_results(&conn)?;

            Ok((website, days, pageviews))
        })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError()
        })?;

    let mut referrers: Vec<Referrer> = vec![];
    let mut pages: Vec<Page> = vec![];
    let format_str = "%Y-%m";

    let mut current_month =
        NaiveDate::from_ymd(end.year(), end.month(), 1).and_hms(12, 00, 00);

    let mut months: BTreeMap<String, SlimStat> = vec![(
        current_month.format(format_str).to_string(),
        SlimStat::default(),
    )]
    .into_iter()
    .collect();

    for _ in 1..11 {
        let year = current_month.year();
        let next_month = current_month.month() as i32 - 1;

        if next_month.is_negative() || next_month == 0 {
            current_month =
                NaiveDate::from_ymd(year - 1, 12, 1).and_hms(12, 00, 00);
        } else {
            current_month = NaiveDate::from_ymd(year, next_month as u32, 1)
                .and_hms(12, 00, 00);
        }

        months.insert(
            current_month.format(format_str).to_string(),
            SlimStat::default(),
        );
    }

    days.iter().for_each(|d| {
        let date = d.created_at.format(format_str).to_string();

        if let Some(m) = months.get_mut(&date) {
            m.users += d.users;
            m.sessions += d.sessions;
            m.pageviews += d.pageviews;
            m.known_time_counter += d.known_time_counter;
            m.bounce_counter += d.bounce_counter;
            m.avg_time += d.avg_time;
            m.bounce_rate += d.bounce_rate;
        }
    });

    // Iterate over all pageviews in the given range.
    pageviews.iter().for_each(|pv: &Pageview| {
        let users = if pv.is_new_user { 1 } else { 0 };
        let sessions = if pv.is_new_session { 1 } else { 0 };

        if !pv.referrer.is_empty() {
            if let Some(mut r) =
                referrers.iter_mut().find(|r| r.name == *pv.referrer)
            {
                r.count += 1;
            } else {
                referrers.push(Referrer::new(&pv.referrer));
            }
        }

        if let Some(mut p) = pages.iter_mut().find(|p| p.name == pv.pathname) {
            p.sessions += sessions;
            p.users += users;
            p.pageviews += 1;
        } else {
            pages.push(Page::new(&pv.pathname, users, sessions));
        }
    });

    pages.sort_by(|a, b| b.pageviews.cmp(&a.pageviews));
    referrers.sort_by(|a, b| b.count.cmp(&a.count));

    Ok(HttpResponse::Ok().json(Stats {
        website,
        stats: months,
        pages,
        referrers,
    }))
}

use crate::models::{
    schema::{pageviews, stats, websites},
    AuthUser, Browser, DeviceCategory, OperatingSystem, Page, Pageview,
    Referrer, Stat, Stats, SystemStats, Website,
};
use crate::utils::{get_days, get_months, Db, DAILY_FORMAT, MONTHLY_FORMAT};
use actix_web::{error::Error as ActixError, web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DbError;
use serde::Deserialize;
use woothee::parser::Parser as UaParser;

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
    let ua_parser = UaParser::new();

    if !["month", "day"].contains(&params.by.as_str()) {
        return Ok(HttpResponse::BadRequest()
            .body("Parameter 'by' should contains 'month' or 'day'."));
    }

    let start = NaiveDateTime::from_timestamp(params.start, 0);
    let end = NaiveDateTime::from_timestamp(params.end, 0);
    let is_monthly = params.by == "month";

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
                .filter(pageviews::website_id.eq(website.id))
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

    let mut results = if is_monthly {
        get_months(start, end)
    } else {
        get_days(start, end)
    };

    days.iter().for_each(|d| {
        let date = if is_monthly {
            d.created_at.format(MONTHLY_FORMAT).to_string()
        } else {
            d.created_at.format(DAILY_FORMAT).to_string()
        };

        if let Some(r) = results.get_mut(&date) {
            r.users += d.users;
            r.sessions += d.sessions;
            r.pageviews += d.pageviews;
            r.time_counter += d.time_counter;
            r.avg_time += d.avg_time;
        }
    });

    let mut systems = SystemStats::default();

    // Iterate over all pageviews in the given range.
    pageviews.iter().for_each(|pv: &Pageview| {
        let users = if pv.is_new_user { 1 } else { 0 };
        let sessions = if pv.is_new_session { 1 } else { 0 };

        if let Some(r) = ua_parser.parse(&pv.user_agent) {
            if let Some(operating_system) = systems
                .operating_systems
                .iter_mut()
                .find(|os| os.name == r.os)
            {
                operating_system.counter += 1;
            } else {
                systems.operating_systems.push(OperatingSystem {
                    name: r.os.into(),
                    counter: 1,
                });
            }

            if let Some(browser) = systems
                .browsers
                .iter_mut()
                .find(|browser| browser.name == r.name)
            {
                browser.counter += 1;
            } else {
                systems.browsers.push(Browser {
                    name: r.name.into(),
                    counter: 1,
                });
            }

            if let Some(category) = systems
                .categories
                .iter_mut()
                .find(|category| category.name == r.category)
            {
                category.counter += 1;
            } else {
                systems.categories.push(DeviceCategory {
                    name: r.category.into(),
                    counter: 1,
                });
            }
        }

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
    systems
        .operating_systems
        .sort_by(|a, b| b.counter.cmp(&a.counter));
    systems.browsers.sort_by(|a, b| b.counter.cmp(&a.counter));

    Ok(HttpResponse::Ok().json(Stats {
        website,
        stats: results,
        pages,
        referrers,
        systems,
    }))
}

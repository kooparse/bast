use crate::models::{
    schema::{ghosts, pages, websites},
    AuthUser, Ghost, Page, SlimGhost, Website,
};
use crate::utils::{to_client, UserError};
use crate::Db;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

#[derive(Deserialize)]
pub struct Query {
    website_id: i32,
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Serialize)]
pub struct Stats {
    website: Website,
    pages: Vec<Page>,
    ghosts: Vec<SlimGhost>,
}

pub async fn stats(
    params: web::Query<Query>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<Stats, UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        let website: Website = websites::table
            .filter(
                websites::user_id
                    .eq(user_id)
                    .and(websites::id.eq(params.website_id)),
            )
            .first::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        let pages: Vec<Page> = pages::table
            .filter(pages::website_id.eq(&website.id))
            .order_by(pages::visitors.desc())
            .get_results::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        let list = if params.start.is_some() && params.end.is_some() {
            let start = params.start.unwrap();
            let end = params.end.unwrap();

            // TODO: We should use gt/lt here...
            let list: Vec<Ghost> = ghosts::table
                .filter(
                    ghosts::website_id
                        .eq(website.id)
                        .and(ghosts::user_id.eq(user_id)),
                )
                .get_results::<_>(&conn)
                .map_err(|_| UserError::BadRequest)?;

            // TODO: Remove unwrap().
            list.into_iter()
                .filter_map(|ghost| {
                    let created_at = ghost
                        .created_at
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        as u64;

                    if created_at >= start && created_at <= end {
                        Some(SlimGhost::from(ghost))
                    } else {
                        None
                    }
                })
                .collect::<_>()
        } else {
            vec![]
        };

        Ok(Stats {
            website,
            pages,
            ghosts: list,
        })
    })
    .await;

    to_client(result)
}

use crate::models::{schema::ghosts, AuthUser, Ghost};
use crate::utils::{to_client, UserError};
use crate::Db;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use std::time::UNIX_EPOCH;

#[derive(Deserialize)]
pub struct Params {
    start: u64,
    end: u64,
    website_id: i32,
}

pub async fn from_range(
    params: web::Query<Params>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<Vec<Ghost>, UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        // TODO: We should use gt/lt here...
        let ghosts: Vec<Ghost> = ghosts::table
            .filter(
                ghosts::website_id
                    .eq(params.website_id)
                    .and(ghosts::user_id.eq(user_id)),
            )
            .get_results::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        // TODO: Remove unwrap().
        let ghosts: Vec<Ghost> = ghosts
            .into_iter()
            .filter(|ghost| {
                let created_at = ghost
                    .created_at
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                created_at >= params.start && created_at <= params.end
            })
            .collect::<_>();

        Ok(ghosts)
    })
    .await;

    to_client(result)
}

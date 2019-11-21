use crate::models::{
    schema::{pages, websites},
    AuthUser, Page, Website,
};
use crate::utils::UserError;
use crate::Db;
use actix_web::{error::ResponseError, web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Query {
    website_id: i32,
}

#[derive(Serialize)]
pub struct Stats {
    website: Website,
    pages: Vec<Page>,
}

pub fn stats(
    params: web::Query<Query>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<Stats, UserError> {
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
            .get_results::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        Ok(Stats { website, pages })
    })
    .then(move |res| match res {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Ok(err.error_response()),
    })
}

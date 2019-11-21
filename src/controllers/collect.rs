use crate::diesel::dsl::*;
use crate::models::{
    schema::{pages, websites},
    SlimPage, Website,
};
use crate::utils::UserError;
use crate::Db;
use actix_web::{error::ResponseError, web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    user_id: i32,
    website_id: i32,
    #[serde(rename(deserialize = "isNewSession"))]
    is_new_session: bool,
    href: String,
    hostname: String,
    origin: String,
    pathname: String,
}

pub fn collect(
    params: web::Query<Data>,
    data: web::Data<Db>,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<(), UserError> {
        let conn = &data.conn_pool()?;

        let website: Website = update(websites::table)
            .filter(
                websites::id
                    .eq(&params.website_id)
                    .and(websites::user_id.eq(&params.user_id)),
            )
            .set((
                websites::visitors.eq(websites::visitors + 1),
                websites::sessions.eq(if params.is_new_session {
                    websites::sessions + 1
                } else {
                    websites::sessions + 0
                }),
            ))
            .get_result::<_>(conn)
            .map_err(|_| UserError::InternalServerError)?;

        // Upsert page.
        let upsert_page = SlimPage {
            website_id: website.id,
            pathname: params.pathname.clone(),
            visitors: 1,
            sessions: if params.is_new_session { 1 } else { 0 },
        };

        insert_into(pages::table)
            .values(upsert_page)
            .on_conflict(pages::pathname)
            .do_update()
            .set((
                pages::visitors.eq(pages::visitors + 1),
                pages::sessions.eq(if params.is_new_session {
                    pages::sessions + 1
                } else {
                    pages::sessions + 0
                }),
            ))
            .execute(&data.conn_pool()?)
            .map_err(|_| UserError::InternalServerError)?;

        Ok(())
    })
    .then(move |res| match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Ok(err.error_response()),
    })
}

use crate::diesel::QueryDsl;
use crate::models::{
    schema::{pages, websites},
    Page, SlimPage, Website,
};
use crate::utils::UserError;
use crate::Db;
use actix_web::{error::ResponseError, web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    tid: String,
    #[serde(rename(deserialize = "isNewSession"))]
    is_new_session: bool,
    href: String,
    hostname: String,
    origin: String,
    pathname: String,
}

pub fn tracker(
    params: web::Query<Data>,
    data: web::Data<Db>,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<(), UserError> {
        let conn = &data.conn_pool()?;
        let tid: Vec<&str> = params.tid.split("::").collect();

        let cid = tid[0].parse::<i32>().unwrap();
        let wid = tid[1].parse::<i32>().unwrap();

        let mut website = websites::table
            .filter(websites::id.eq(&wid).and(websites::user_id.eq(&cid)))
            .first::<Website>(conn)
            .map_err(|_| UserError::NotFound)?;

        website.visitors += 1;

        if params.is_new_session {
            website.sessions += 1;
        }

        diesel::update(websites::table)
            .filter(websites::id.eq(&wid).and(websites::user_id.eq(&cid)))
            .set((
                websites::visitors.eq(website.visitors),
                websites::sessions.eq(website.sessions),
            ))
            .execute(conn)
            .map_err(|_| UserError::InternalServerError)?;

        // should update website here.

        let page: Option<Page> = pages::table
            .filter(
                pages::website_id
                    .eq(&website.id)
                    .and(pages::pathname.eq(&params.pathname)),
            )
            .get_result::<_>(conn)
            .optional()
            .map_err(|_| UserError::NotFound)?;

        // If the page exist, we update it.
        if let Some(mut p) = page {
            p.visitors += 1;
            if params.is_new_session {
                p.sessions += 1;
            }

            diesel::update(pages::table)
                .filter(pages::id.eq(p.id))
                .set((
                    pages::visitors.eq(p.visitors),
                    pages::sessions.eq(p.sessions),
                ))
                .execute(conn)
                .map_err(|_| UserError::InternalServerError)?;
        } else {
            // insert a new page.
            let mut p = SlimPage {
                website_id: website.id,
                pathname: params.pathname.clone(),
                visitors: 1,
                sessions: 0,
            };

            if params.is_new_session {
                p.sessions += 1;
            }

            diesel::insert_into(pages::table)
                .values(p)
                .execute(&data.conn_pool()?)
                .map_err(|_| UserError::InternalServerError)?;
        }

        Ok(())
    })
    .then(move |res| match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Ok(err.error_response()),
    })
}

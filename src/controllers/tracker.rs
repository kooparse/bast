use crate::diesel::QueryDsl;
use crate::models::{
    schema::{users, websites},
    User, Website,
};
use crate::utils::UserError;
use crate::Db;
use actix_web::{error::ResponseError, web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    tid: String,
    uuid: String,
    puuid: String,
    #[serde(rename(deserialize = "isNewSession"))]
    is_new_session: bool,
    href: String,
    hostname: String,
    origin: String,
    pathname: String,
    pages: Vec<String>
}

pub fn tracker(
    params: web::Query<Data>,
    data: web::Data<Db>,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    dbg!(&params);

    web::block(move || -> Result<(), UserError> {
        let conn = &data.conn_pool()?;
        let tid: Vec<&str> = params.tid.split("::").collect();

        let cid = tid[0].parse::<i32>().unwrap();
        let wid = tid[1].parse::<i32>().unwrap();

        // TODO: make those requests in parallel.
        // Get the corresponding owner id.
        let owner = users::table
            .filter(users::id.eq(&cid))
            .first::<User>(conn)
            .map_err(|_| UserError::NotFound)?;

        let website = websites::table
            .filter(websites::id.eq(&wid))
            .first::<Website>(conn)
            .map_err(|_| UserError::NotFound)?;


        dbg!(params);

        Ok(())
    })
    .then(move |res| match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Ok(err.error_response()),
    })
}

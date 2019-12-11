use crate::models::{
    schema::{users, websites},
    AuthUser, User, Website,
};
use crate::utils::UserError;
use crate::Db;
use actix_web::{error::ResponseError, web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[table_name = "websites"]
pub struct WebsiteFormData {
    #[serde(skip_deserializing)]
    user_id: i32,
    domain: String,
}

pub fn get_all(
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<Vec<Website>, UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        // Check if user is found.
        let user: User = users::table
            .find(user_id)
            .first::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        let list: Vec<Website> = websites::table
            .filter(websites::user_id.eq(user.id))
            .get_results::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        Ok(list)
    })
    .then(move |res| match res {
        Ok(list) => Ok(HttpResponse::Ok().json(list)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn create(
    mut form: web::Json<WebsiteFormData>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<Website, UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        // Check if user is found.
        let user: User = users::table
            .find(user_id)
            .first::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        form.user_id = user.id;

        let website: Website = diesel::insert_into(websites::table)
            .values(form.into_inner())
            .get_result(&conn)
            .map_err(|_| UserError::InternalServerError)?;

        Ok(website)
    })
    .then(move |res| match res {
        Ok(website) => Ok(HttpResponse::Ok().json(website)),
        Err(err) => Ok(err.error_response()),
    })
}

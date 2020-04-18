use crate::models::{
    schema::{users, websites},
    AuthUser, User, Website,
};
use crate::Db;
use actix_web::{error::Error as ActixError, web, HttpResponse};
use diesel::prelude::*;
use diesel::result::Error as DbError;
use serde::Deserialize;

/// Get user's websites.
pub async fn list(
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, ActixError> {
    let conn = data.conn_pool()?;
    let user_id = auth_user.get_id()?;

    let list: Vec<Website> = web::block(move || -> Result<_, DbError> {
        // Find is user exists.
        let user: User = users::table.find(user_id).get_result(&conn)?;

        let list = websites::table
            .filter(websites::user_id.eq(user.id))
            .order(websites::created_at.desc())
            .get_results(&conn)?;

        Ok(list)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::Unauthorized()
    })?;

    Ok(HttpResponse::Ok().json(list))
}

#[derive(Deserialize, Insertable)]
#[table_name = "websites"]
pub struct WebsiteFormData {
    #[serde(skip_deserializing)]
    user_id: i32,
    domain: String,
}

/// Create new website for authenticated user.
pub async fn create(
    mut form: web::Json<WebsiteFormData>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, ActixError> {
    let conn = data.conn_pool()?;
    let user_id = auth_user.get_id()?;

    let website: Website = web::block(move || -> Result<_, DbError> {
        let user: User = users::table.find(&user_id).get_result(&conn)?;
        form.user_id = user.id;

        let website = diesel::insert_into(websites::table)
            .values(form.into_inner())
            .get_result(&conn)?;

        Ok(website)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError()
    })?;

    Ok(HttpResponse::Ok().json(website))
}

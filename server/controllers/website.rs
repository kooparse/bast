use crate::models::{
    schema::{users, websites},
    AuthUser, User, Website,
};
use crate::utils::{to_client, UserError};
use crate::Db;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

pub async fn get_all(
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<Vec<Website>, UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        // Check if user is found.
        let user: User = users::table
            .find(user_id)
            .first::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        let list: Vec<Website> = websites::table
            .filter(websites::user_id.eq(user.id))
            .order(websites::created_at.desc())
            .get_results::<_>(&conn)
            .map_err(|_| UserError::BadRequest)?;

        Ok(list)
    })
    .await;

    to_client(result)
}

#[derive(Deserialize, Insertable)]
#[table_name = "websites"]
pub struct WebsiteFormData {
    #[serde(skip_deserializing)]
    user_id: i32,
    domain: String,
}

pub async fn create(
    mut form: web::Json<WebsiteFormData>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<Website, UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        dbg!(&form.domain);

        // Check if user is found.
        let user: User = users::table
            .find(user_id)
            .first(&conn)
            .map_err(|_| UserError::BadRequest)?;

        form.user_id = user.id;

        let website: Website = diesel::insert_into(websites::table)
            .values(form.into_inner())
            .get_result(&conn)
            .map_err(|e| {
                dbg!(e);
                return UserError::InternalServerError;
            })?;

        Ok(website)
    })
    .await;

    to_client(result)
}

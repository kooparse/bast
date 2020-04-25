use crate::models::{
    schema::{pageviews, stats, users, websites},
    AuthUser, User, Website,
};
use crate::utils::{is_valid_domain, Db};
use actix_web::{error::Error as ActixError, web, HttpRequest, HttpResponse};
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
        // Find if user exists.
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

// Delete all pageviews, stats and finally websites
// related to given website_id.
pub async fn delete(
    req: HttpRequest,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, ActixError> {
    let user_id = auth_user.get_id()?;
    let conn = data.conn_pool()?;
    let path = req
        .match_info()
        .get("website_id")
        .ok_or_else(|| HttpResponse::Forbidden())?;

    let website_id: i32 = path.parse().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::Forbidden()
    })?;

    web::block(move || -> Result<(), DbError> {
        diesel::delete(stats::table)
            .filter(stats::id.eq(website_id))
            .execute(&conn)?;

        diesel::delete(pageviews::table)
            .filter(pageviews::website_id.eq(website_id))
            .execute(&conn)?;

        diesel::delete(websites::table)
            .filter(
                websites::id
                    .eq(website_id)
                    .and(websites::user_id.eq(user_id)),
            )
            .execute(&conn)?;

        Ok(())
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError()
    })?;

    Ok(HttpResponse::Ok().finish())
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

    if !is_valid_domain(&form.domain) {
        return Ok(HttpResponse::Forbidden().body("Invalid domain name."));
    }

    let website: Website = web::block(move || -> Result<_, DbError> {
        let user: User = users::table.find(&user_id).get_result(&conn)?;
        form.user_id = user.id;
        form.domain = form.domain.to_ascii_lowercase();

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

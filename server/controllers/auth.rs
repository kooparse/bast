use crate::models::{schema::users, AuthUser, SlimUser, User, UserWithToken};
use crate::utils::JWTPayload;
use crate::Db;
use actix_web::{error::Error as ActixError, web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use dotenv;
use jsonwebtoken::{encode, Header};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SignInData {
    email: String,
    password: String,
}

/// Handle login from given email and password.
/// Return light information about the user and token.
pub async fn login(
    form: web::Json<SignInData>,
    data: web::Data<Db>,
) -> Result<HttpResponse, ActixError> {
    let conn = data.conn_pool()?;
    // TODO: Change timeout.
    let jwt_timeout = 10_000_000_000;
    let jwt_secret = dotenv::var("JWT_SECRET").map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError()
    })?;

    let form = form.into_inner();
    // Value moved after...
    let password = form.password.clone().to_ascii_lowercase();

    let user: User = web::block(move || {
        users::table
            .filter(users::email.eq(&form.email))
            .first(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError()
    })?;

    let is_valid = verify(password, &user.password).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError()
    })?;

    if !is_valid {
        return Ok(HttpResponse::Unauthorized().body("Password is not valid."));
    }

    let payload = JWTPayload::new(user.id, jwt_timeout);
    let token = encode(&Header::default(), &payload, jwt_secret.as_ref())
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError()
        })?;

    let user_with_token = UserWithToken {
        user: SlimUser::from(user),
        token,
    };

    Ok(HttpResponse::Ok().json(user_with_token))
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct RegisterFormData {
    email: String,
    password: String,
}

/// Create new user from given email and password.
pub async fn register(
    mut form: web::Json<RegisterFormData>,
    data: web::Data<Db>,
) -> Result<HttpResponse, ActixError> {
    let conn = data.conn_pool()?;
    // Store only lowercase version of email address.
    form.email = form.email.to_ascii_lowercase();
    form.password = hash(&form.password, DEFAULT_COST)
        .map_err(|_| HttpResponse::InternalServerError())?;

    let user: User = web::block(move || {
        diesel::insert_into(users::table)
            .values(form.into_inner())
            .get_result(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().body("Email already exists.")
    })?;

    Ok(HttpResponse::Ok().json(SlimUser::from(user)))
}

/// Find information from authenticated user (token).
pub async fn user(
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, ActixError> {
    let conn = data.conn_pool()?;
    let user_id = auth_user.get_id()?;

    let user: User =
        web::block(move || users::table.find(&user_id).get_result(&conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError()
            })?;

    Ok(HttpResponse::Ok().json(SlimUser::from(user)))
}

use crate::models::{schema::users, AuthUser, SlimUser, User};
use crate::utils::{to_client, JWTPayload, UserError};
use crate::Db;
use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use dotenv;
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

pub async fn user(
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<SlimUser, UserError> {
        let user_id = auth_user.get_id()?;

        let user: User = users::table
            .find(&user_id)
            .get_result::<_>(&data.conn_pool()?)
            .map_err(|_| UserError::BadRequest)?;

        Ok(SlimUser::from(user))
    })
    .await;

    to_client(result)
}

#[derive(Deserialize)]
pub struct SignInData {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserWithToken {
    user: SlimUser,
    token: String,
}

pub async fn login(
    form: web::Json<SignInData>,
    data: web::Data<Db>,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<UserWithToken, UserError> {
        let jwt_secret = dotenv::var("JWT_SECRET").unwrap();
        let jwt_timeout = 10_000_000_000;

        let user: User = users::table
            .filter(users::email.eq(&form.email))
            .first::<_>(&data.conn_pool()?)
            .map_err(|_| UserError::Unauthorized)?;

        let is_valid = verify(&form.password, &user.password)
            .map_err(|_| UserError::Unauthorized)?;

        if !is_valid {
            return Err(UserError::Unauthorized);
        }

        let payload = JWTPayload::new(user.id, jwt_timeout);
        let token = encode(&Header::default(), &payload, jwt_secret.as_ref())
            .map_err(|_| UserError::BadRequest)?;

        let result = UserWithToken {
            user: SlimUser::from(user),
            token,
        };

        Ok(result)
    })
    .await;

    to_client(result)
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct RegisterFormData {
    email: String,
    password: String,
}

/// Create a new user from email and password.
pub async fn register(
    mut form: web::Json<RegisterFormData>,
    data: web::Data<Db>,
) -> Result<HttpResponse, UserError> {
    let result = web::block(move || -> Result<SlimUser, UserError> {
        form.password = hash(&form.password, DEFAULT_COST)
            .map_err(|_| UserError::InternalServerError)?;

        let user: User = diesel::insert_into(users::table)
            .values(form.into_inner())
            .get_result(&data.conn_pool()?)
            .map_err(|_| UserError::InternalServerError)?;

        Ok(SlimUser::from(user))
    })
    .await;

    to_client(result)
}

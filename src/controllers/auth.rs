use crate::models::{schema::users, SlimUser, User};
use crate::utils::{JWTPayload, UserError};
use crate::Db;
use actix_web::{error::ResponseError, web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use dotenv;
use futures::Future;
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignInData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserWithToken {
    user: SlimUser,
    token: String,
}

pub fn login(
    form: web::Json<SignInData>,
    data: web::Data<Db>,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<UserWithToken, UserError> {
        let jwt_secret = dotenv::var("JWT_SECRET").unwrap();
        let jwt_timeout = 10000000000;

        let user: User = users::table
            .filter(users::username.eq(&form.username))
            .first::<_>(&data.conn_pool()?)
            .map_err(|_| UserError::BadRequest)?;

        let is_valid = verify(&form.password, &user.password)
            .map_err(|_| UserError::BadRequest)?;

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
    .then(move |res| match res {
        Ok(user_with_token) => Ok(HttpResponse::Ok().json(user_with_token)),
        Err(err) => Ok(err.error_response()),
    })
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct RegisterFormData {
    username: String,
    password: String,
    email: String,
}

pub fn register(
    mut form: web::Json<RegisterFormData>,
    data: web::Data<Db>,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<SlimUser, UserError> {
        form.password = hash(&form.password, DEFAULT_COST)
            .map_err(|_| UserError::InternalServerError)?;

        let user: User = diesel::insert_into(users::table)
            .values(form.into_inner())
            .get_result(&data.conn_pool()?)
            .map_err(|_| UserError::InternalServerError)?;

        Ok(SlimUser::from(user))
    })
    .then(move |res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => Ok(err.error_response()),
    })
}

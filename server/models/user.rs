use crate::utils::{check_auth, Ready};
use actix_web::{
    dev::Payload, error::Error as ActixError, FromRequest, HttpRequest,
    HttpResponse,
};
use diesel::Queryable;
use serde::Serialize;
use std::time::SystemTime;

/// Full user object with all database information.
#[derive(Serialize, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
}

/// Serialiazed user struct before sending to
/// the client.
#[derive(Default, Serialize)]
pub struct SlimUser {
    pub id: i32,
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

/// Send from login endpoint.
#[derive(Serialize)]
pub struct UserWithToken {
    pub user: SlimUser,
    pub token: String,
}

/// Used only to restrict some calls to authenticated users.
/// We do not fetch the logged user here.
pub struct AuthUser {
    pub id: Option<i32>,
}

impl AuthUser {
    pub fn get_id(&self) -> Result<i32, ActixError> {
        if let Some(id) = self.id {
            return Ok(id);
        }

        Err(ActixError::from(HttpResponse::Unauthorized()))
    }
}

impl FromRequest for AuthUser {
    type Error = ActixError;
    type Config = ();
    type Future = Ready<Result<AuthUser, ActixError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match check_auth(&req) {
            Ok(id) => Ready(Some(Ok(AuthUser { id }))),
            Err(err) => Ready(Some(Err(err))),
        }
    }
}

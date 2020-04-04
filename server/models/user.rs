use crate::utils::{check_auth, Ready, UserError};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
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

/// Used only to restrict some calls to authenticated users.
/// We do not fetch the logged user here.
pub struct AuthUser {
    pub id: Option<i32>,
}

impl AuthUser {
    pub fn get_id(&self) -> Result<i32, UserError> {
        if let Some(id) = self.id {
            return Ok(id);
        }

        Err(UserError::Unauthorized)
    }
}

impl FromRequest for AuthUser {
    type Error = UserError;
    type Config = ();
    type Future = Ready<Result<AuthUser, UserError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match check_auth(&req) {
            Ok(id) => Ready(Some(Ok(AuthUser { id }))),
            Err(err) => Ready(Some(Err(err))),
        }
    }
}

use crate::utils::{check_auth, UserError};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use diesel::Queryable;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Queryable, PartialEq, Debug)]
/// Full user object with all database information.
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
}

#[derive(Default, Serialize)]
/// Serialiazed user struct before sending to
/// the client.
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
    type Future = Result<AuthUser, UserError>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let id = check_auth(&req)?;
        Ok(AuthUser { id })
    }
}

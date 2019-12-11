use actix_web::{error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Enum storing all error types.
#[derive(Debug, Deserialize, Serialize)]
pub enum UserError {
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            UserError::BadRequest => write!(f, "Bad Request"),
            UserError::Unauthorized => write!(f, "Unauthorized"),
            UserError::NotFound => write!(f, "Not Found"),
            UserError::InternalServerError => write!(f, "Internal Error"),
        }
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let mut response = match *self {
            UserError::BadRequest => HttpResponse::BadRequest(),
            UserError::Unauthorized => HttpResponse::Unauthorized(),
            UserError::NotFound => HttpResponse::NotFound(),
            UserError::InternalServerError => {
                HttpResponse::InternalServerError()
            }
        };

        response.finish()
    }
}

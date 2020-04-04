use actix_web::{error::BlockingError, HttpResponse};
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

mod db;
mod error;
mod seed;
mod token;

pub use db::*;
pub use error::*;
pub use seed::*;
pub use token::*;

pub fn to_client<T: Serialize>(
    result: Result<T, BlockingError<UserError>>,
) -> Result<HttpResponse, UserError> {
    match result {
        Ok(body) => Ok(HttpResponse::Ok().json(body)),
        Err(err) => match err {
            BlockingError::Error(e) => Err(e),
            BlockingError::Canceled => Err(UserError::InternalServerError),
        },
    }
}

pub struct Ready<T>(pub Option<T>);

impl<T> Unpin for Ready<T> {}
impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().unwrap())
    }
}

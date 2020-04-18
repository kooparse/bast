use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

mod db;
mod seed;
mod token;

pub use db::*;
pub use seed::*;
pub use token::*;

pub struct Ready<T>(pub Option<T>);

impl<T> Unpin for Ready<T> {}
impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().unwrap())
    }
}

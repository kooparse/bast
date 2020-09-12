use regex::{Error as RegexError, Regex};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

mod db;
mod seed;
mod token;
mod views;

pub use db::*;
pub use seed::*;
pub use token::*;
pub use views::*;

pub struct Ready<T>(pub Option<T>);

impl<T> Unpin for Ready<T> {}
impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().unwrap())
    }
}

/// Safety check against collect from faulty domains.
pub fn is_belongs_to_domain(
    domain: &str,
    path: &str,
) -> Result<bool, RegexError> {
    let regex = Regex::new(&format!(
        "{}{}{}{}",
        "^(https?://)?", "(?:[^/:]+.)?", domain, "(/.*$)?$"
    ))?;

    Ok(regex.is_match(path))
}

// Check if it's a valid domain name.
pub fn is_valid_domain(domain: &str) -> bool {
    lazy_static! {
        static ref DOMAIN_REGEX: Regex = Regex::new(
            r"^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]\.[a-zA-Z]{2,}$"
        )
        .unwrap();
    }

    DOMAIN_REGEX.is_match(domain)
}

// Check if hostname is localhost.
pub fn is_from_localhost(hostname: &str) -> bool {
    lazy_static! {
        static ref LOCALHOST_REGEX: Regex = Regex::new(
            r"^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$"
        )
        .unwrap();
    }

    hostname == "localhost"
        || hostname == "[::1]"
        || LOCALHOST_REGEX.is_match(hostname)
}

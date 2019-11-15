use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    id: String,
    pid: String,
    is_new: bool,
    href: String,
    website_id: String,
    hostname: String,
    origin: String,
    pathname: String,
}

pub fn tracker(params: web::Query<Data>) -> impl Responder {
    dbg!(params);
    dbg!("Hello World")
}

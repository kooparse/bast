mod controllers;
mod db;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use controllers::health;
use db::Db;
use dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Data {
    id: String,
    pid: Option<String>,
    href: String,
    website_id: String,
    hostname: String,
    origin: String,
    pathname: String,
}

fn tracker(params: web::Query<Data>) -> impl Responder {
    dbg!(params);
    dbg!("Hello World")
}

/// All routes are here.
fn main() -> std::io::Result<()> {
    // Load all environement's variables.
    dotenv::dotenv().ok();

    let server_address =
        dotenv::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let server_port = dotenv::var("SERVER_PORT").unwrap_or("3333".to_string());
    let bind_address = format!("{}:{}", server_address, server_port);

    HttpServer::new(move || {
        App::new().data(Db::new()).wrap(Cors::default()).service(
            // TODO: Should be "api".
            web::scope("/")
                .route("/tracker", web::get().to(tracker))
                .route("/health", web::get().to(health)),
        )
    })
    .bind(bind_address)?
    .run()
}

mod controllers;
mod db;
mod models;
mod utils;

#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use controllers::{health, login, register, tracker, website};
use db::Db;
use dotenv;
use env_logger;

/// All routes are here.
fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Load all environement's variables.
    dotenv::dotenv().ok();

    let server_address =
        dotenv::var("SERVER_URL").unwrap_or("127.0.0.1".to_string());
    let server_port = dotenv::var("SERVER_PORT").unwrap_or("3333".to_string());
    let bind_address = format!("{}:{}", server_address, server_port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Db::new())
            .wrap(Cors::default())
            .service(
                // TODO: Should be "api".
                web::scope("/")
                    .route("/register", web::post().to_async(register))
                    .route("/login", web::post().to_async(login))
                    .route("/tracker", web::get().to_async(tracker))
                    .route("/website", web::post().to_async(website::create))
                    .route("/health", web::get().to(health)),
            )
    })
    .bind(bind_address)?
    .run()
}

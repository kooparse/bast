mod controllers;
mod db;
mod models;
mod utils;

#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use controllers::{collect, health, login, register, stats, website};
use db::Db;
use dotenv::{dotenv, var};
use env_logger;

fn script() -> Result<NamedFile> {
    Ok(NamedFile::open("./website/script.js")?)
}

/// All routes are here.
fn main() -> std::io::Result<()> {
    // Load all environement's variables.
    dotenv().ok();
    // Load logger env info
    env_logger::init();

    let bind_address = {
        format!(
            "{}:{}",
            var("SERVER_URL").unwrap_or("0.0.0.0".into()),
            var("PORT").unwrap_or("3333".into())
        )
    };

    println!("Starting server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Db::new())
            .wrap(Cors::default())
            .service(
                web::scope("/api")
                    .route("/register", web::post().to_async(register))
                    .route("/login", web::post().to_async(login))
                    .route("/collect", web::get().to_async(collect))
                    .route("/stats", web::get().to_async(stats))
                    .route("/website", web::post().to_async(website::create))
                    .route("/health", web::get().to(health)),
            )
            .route("/script.js", web::get().to(script))
    })
    .bind(bind_address)?
    .run()
}

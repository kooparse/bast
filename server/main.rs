mod controllers;
mod db;
mod models;
mod utils;

#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use controllers::{
    collect, file, from_range, health, login, register, stats, user, website,
};
use db::Db;
use dotenv::{dotenv, var};
use env_logger;

/// All routes are here.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load all environement's variables.
    dotenv().ok();
    // Load logger env info
    env_logger::init();

    let bind_address = {
        format!(
            "{}:{}",
            var("SERVER_URL").unwrap_or_else(|_| "0.0.0.0".into()),
            var("PORT").unwrap_or_else(|_| "3333".into())
        )
    };

    println!("Starting server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .data(Db::new())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .service(
                web::scope("/api")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login))
                    .route("/user", web::get().to(user))
                    .route("/stats", web::get().to(stats))
                    .route("/ghosts", web::get().to(from_range))
                    .route("/websites", web::get().to(website::get_all))
                    .route("/website", web::post().to(website::create))
                    .route("/health", web::get().to(health)),
            )
            // Serving the script file.
            .route("/script.js", web::get().to(file::script))
            // Collect data from client websites.
            .route("/ghost.png", web::get().to(collect))
            // Serving the front static app.
            .route("/register", web::get().to(file::front_register))
            .route("/login", web::get().to(file::front_login))
            .route("/create_website", web::get().to(file::front_create_website))
            .service(
                fs::Files::new("/", "./static/front")
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}

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
    collect, file, health, login, register, stats, user, website,
};
use db::Db;
use dotenv::{dotenv, var};
use env_logger;

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
            .data(Db::new())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .service(
                web::scope("/api")
                    .route("/register", web::post().to_async(register))
                    .route("/login", web::post().to_async(login))
                    .route("/collect", web::get().to_async(collect))
                    .route("/stats", web::get().to_async(stats))
                    .route("/user", web::get().to_async(user))
                    .route("/websites", web::get().to_async(website::get_all))
                    .route("/website", web::post().to_async(website::create))
                    .route("/health", web::get().to(health)),
            )
            // Serving the script file.
            .route("/script.js", web::get().to(file::script))
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
}

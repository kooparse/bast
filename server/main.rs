mod controllers;
mod models;
mod utils;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use controllers::{collect, file, get_stat, login, register, user, website};
use dotenv::{dotenv, var};
use env_logger;
use utils::Db;

embed_migrations!();

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load all environement's variables.
    dotenv().ok();
    // Load logger env info
    env_logger::init();

    let db = Db::new();
    let conn = db
        .conn_pool()
        .expect("Failed to connect throught the dabatabase pool.");

    // Run pending migrations before starting our server.
    embedded_migrations::run(&conn).expect("Failed to run migrations.");

    let bind_address = {
        format!(
            "{}:{}",
            var("HOSTNAME").unwrap_or("0.0.0.0".into()),
            var("PORT").unwrap_or("3333".into())
        )
    };

    let server = HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(Logger::default())
            .wrap(Cors::new().send_wildcard().finish())
            .service(
                web::scope("/api")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login))
                    .route("/collect", web::get().to(collect))
                    .route("/user", web::get().to(user))
                    .route("/stats", web::get().to(get_stat))
                    .route("/websites", web::get().to(website::list))
                    .route(
                        "/websites/{website_id}",
                        web::delete().to(website::delete),
                    )
                    .route("/websites", web::post().to(website::create)),
            )
            // Serving the script file.
            .route("/script.js", web::get().to(file::script))
            // Collect data from client websites.
            .route("/ghost.png", web::get().to(collect))
            // Serving the front static app.
            .route("/register", web::get().to(file::front_register))
            .route("/login", web::get().to(file::front_login))
            .route("/settings", web::get().to(file::front_settings))
            .service(
                fs::Files::new("/", "./static/front")
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind(&bind_address)?
    .run();

    println!("Starting server on {}", bind_address);
    server.await
}

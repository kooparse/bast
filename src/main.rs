mod controllers;
mod db;

use actix_web::{web, App, HttpServer};
use controllers::health;
use db::Db;
use dotenv;

/// All routes are here.
fn main() -> std::io::Result<()> {
    // Load all environement's variables.
    dotenv::dotenv().ok();

    let server_address =
        dotenv::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let server_port = dotenv::var("SERVER_PORT").unwrap_or("3333".to_string());
    let bind_address = format!("{}:{}", server_address, server_port);

    HttpServer::new(|| {
        App::new()
            .data(Db::new())
            .service(web::resource("/health").to(health))
    })
    .bind(bind_address)?
    .run()
}

use actix_web::Responder;

pub fn health() -> impl Responder {
    dbg!("Hello World")
}

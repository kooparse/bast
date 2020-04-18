use actix_web::{error::Error as ActixError, HttpRequest, HttpResponse};
use dotenv;
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

pub fn check_auth(req: &HttpRequest) -> Result<Option<i32>, ActixError> {
    req.headers()
        .get("Authorization")
        .ok_or_else(|| HttpResponse::InternalServerError())
        .and_then(|bearer| {
            let token = bearer
                .to_str()
                .expect("Failed to cast bearer header to &str")
                .replace("Bearer ", "");

            let jwt_secret = dotenv::var("JWT_SECRET").map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError()
            })?;

            let decoded = decode::<JWTPayload>(
                &token,
                jwt_secret.as_ref(),
                &Validation::default(),
            )
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError()
            })?;

            return Ok(Some(decoded.claims.id));
        })
        .map_err(|e| ActixError::from(e))
}
#[derive(Deserialize, Serialize, Debug)]
pub struct JWTPayload {
    pub id: i32,
    pub exp: usize,
}

impl JWTPayload {
    pub fn new(id: i32, exp: usize) -> Self {
        Self { id, exp }
    }
}

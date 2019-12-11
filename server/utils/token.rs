use crate::utils::UserError;
use actix_web::HttpRequest;
use dotenv;
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

pub fn check_auth(req: &HttpRequest) -> Result<Option<i32>, UserError> {
    if let Some(bearer) = req.headers().get("Authorization") {
        let token = bearer
            .to_str()
            .expect("Failed to cast bearer header to &str")
            .replace("Bearer ", "");

        let jwt_secret = dotenv::var("JWT_SECRET")
            .map_err(|_| UserError::InternalServerError)?;

        let decoded = decode::<JWTPayload>(
            &token,
            jwt_secret.as_ref(),
            &Validation::default(),
        )
        .map_err(|_| UserError::Unauthorized)?;

        return Ok(Some(decoded.claims.id));
    }

    Err(UserError::Unauthorized)
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

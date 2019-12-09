use crate::models::{schema::ghosts, AuthUser, Ghost};
use crate::utils::UserError;
use crate::Db;
use actix_web::{error::ResponseError, web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use std::time::SystemTime;

pub fn from_range(
    params: web::Query<(SystemTime, SystemTime)>,
    data: web::Data<Db>,
    auth_user: AuthUser,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    web::block(move || -> Result<(), UserError> {
        let user_id = auth_user.get_id()?;
        let conn = data.conn_pool()?;

        Ok(())
    })
    .then(move |res| match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Ok(err.error_response()),
    })
}

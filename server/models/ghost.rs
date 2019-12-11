use super::schema::ghosts;
use crate::models::{User, Website};
use diesel::Queryable;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Associations, Queryable, PartialEq, Debug)]
#[belongs_to(Website)]
#[belongs_to(User)]
pub struct Ghost {
    pub id: i32,
    pub user_id: i32,
    pub website_id: i32,
    pub is_new_session: bool,
    pub pathname: String,
    pub hostname: String,
    pub created_at: SystemTime,
}

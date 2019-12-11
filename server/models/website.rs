use super::schema::websites;
use crate::models::User;
use diesel::Queryable;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Associations, Queryable, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Website {
    pub id: i32,
    #[serde(skip_serializing)]
    pub user_id: i32,
    pub visitors: i32,
    pub sessions: i32,
    pub domain: String,
    #[serde(skip_serializing)]
    pub created_at: SystemTime,
}

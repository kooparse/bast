use super::schema::websites;
use crate::models::User;
use diesel::Queryable;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Associations, AsChangeset, Queryable, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Website {
    pub id: i32,
    #[serde(skip_serializing)]
    pub user_id: i32,
    pub domain: String,
    pub pageviews: i32,
    pub users: i32,
    pub sessions: i32,
    #[serde(rename(serialize = "avgTime"))]
    pub avg_time: f32,
    #[serde(rename(serialize = "bounceRate"))]
    pub bounce_rate: f32,
    pub known_time_counter: i32,
    #[serde(skip_serializing)]
    pub created_at: SystemTime,
}

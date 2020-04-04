use super::schema::{day_stats, month_stats};
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(
    Serialize, Associations, AsChangeset, Queryable, PartialEq, Clone, Debug,
)]
pub struct MonthStat {
    pub id: i32,
    #[serde(rename(serialize = "websiteId"))]
    pub website_id: i32,
    pub users: i32,
    pub sessions: i32,
    #[serde(rename(serialize = "avgTime"))]
    pub avg_time: f32,
    #[serde(rename(serialize = "bounceRate"))]
    pub bounce_rate: f32,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: NaiveDateTime,
}

#[derive(
    Serialize, Associations, AsChangeset, Queryable, PartialEq, Clone, Debug,
)]
pub struct DayStat {
    pub id: i32,
    #[serde(rename(serialize = "websiteId"))]
    pub website_id: i32,
    pub users: i32,
    pub sessions: i32,
    #[serde(rename(serialize = "avgTime"))]
    pub avg_time: f32,
    #[serde(rename(serialize = "bounceRate"))]
    pub bounce_rate: f32,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: NaiveDateTime,
}

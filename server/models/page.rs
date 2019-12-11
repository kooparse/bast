use super::schema::pages;
use crate::models::Website;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Associations, Queryable, PartialEq, Debug)]
#[belongs_to(Website)]
pub struct Page {
    pub id: i32,
    #[serde(skip_serializing)]
    pub website_id: i32,
    pub pathname: String,
    pub visitors: i32,
    pub sessions: i32,
    #[serde(skip_serializing)]
    pub created_at: SystemTime,
}

#[derive(Serialize, Insertable, PartialEq, Debug)]
#[table_name = "pages"]
pub struct SlimPage {
    pub website_id: i32,
    pub pathname: String,
    pub visitors: i32,
    pub sessions: i32,
}

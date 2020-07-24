use super::schema::pageviews;
use crate::models::Website;
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Associations, AsChangeset, Queryable, PartialEq, Debug)]
#[belongs_to(Website)]
pub struct Pageview {
    pub id: Uuid,
    pub website_id: i32,
    pub u_id: String,
    pub pathname: String,
    pub href: String,
    pub hostname: String,
    pub referrer: String,
    // user agent in order to find browser version and
    // operating system.
    pub user_agent: String,
    // I don't know yet for this field,
    // but we're gonna use it to find the geo.
    // It's a string containing (probably) an ip address.
    pub location: Option<String>,
    pub is_new_session: bool,
    pub is_new_user: bool,
    pub duration: f32,
    pub is_done: bool,
    pub created_at: NaiveDateTime,
}

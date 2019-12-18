use super::schema::ghosts;
use crate::models::{User, Website};
use diesel::Queryable;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

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

impl Ghost {
    fn date_to_timestamp(&self) -> u64 {
        self.created_at
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct SlimGhost {
    pub id: i32,
    #[serde(rename(serialize = "isNewSession"))]
    pub is_new_session: bool,
    pub pathname: String,
    pub hostname: String,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: u64,
}

impl From<Ghost> for SlimGhost {
    fn from(ghost: Ghost) -> Self {
        let created_at = ghost.date_to_timestamp();

        Self {
            id: ghost.id,
            is_new_session: ghost.is_new_session,
            pathname: ghost.pathname,
            hostname: ghost.hostname,
            created_at,
        }
    }
}

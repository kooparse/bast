use super::schema::stats;
use super::Website;
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;
use std::collections::BTreeMap;

pub trait CmpStat {
    fn cmp(&mut self, is_new_user: bool, is_new_session: bool, duration: f32);
}

#[derive(
    Serialize, Associations, AsChangeset, Queryable, PartialEq, Clone, Debug,
)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub id: i32,
    pub website_id: i32,
    pub pageviews: i32,
    pub users: i32,
    pub sessions: i32,
    pub avg_time: f32,
    pub time_counter: i32,
    pub created_at: NaiveDateTime,
}

impl CmpStat for Stat {
    fn cmp(&mut self, is_new_user: bool, is_new_session: bool, duration: f32) {
        self.pageviews += 1;

        if is_new_user {
            self.users += 1;
        }

        if is_new_session {
            self.sessions += 1;
        }

        if duration > 5. && duration < 1800. {
            self.time_counter += 1;
            self.avg_time =
                (self.avg_time + duration) / self.time_counter as f32;
        }
    }
}

#[derive(Default, Serialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimStat {
    pub pageviews: i32,
    pub users: i32,
    pub sessions: i32,
    pub avg_time: f32,
    pub time_counter: i32,
}

#[derive(Serialize)]
pub struct Stats {
    pub website: Website,
    pub stats: BTreeMap<String, SlimStat>,
    pub pages: Vec<Page>,
    pub referrers: Vec<Referrer>,
    pub systems: SystemStats,
}

#[derive(Debug, Serialize)]
pub struct Referrer {
    pub name: String,
    pub count: i32,
}

impl Referrer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            count: 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub name: String,
    pub sessions: i32,
    pub users: i32,
    pub pageviews: i32,
}

impl Page {
    pub fn new(name: &str, users: i32, sessions: i32) -> Self {
        Self {
            name: name.to_owned(),
            pageviews: 1,
            sessions,
            users,
        }
    }
}

#[derive(Serialize)]
pub struct OperatingSystem {
    pub name: String,
    pub counter: i32,
}

#[derive(Serialize)]
pub struct Browser {
    pub name: String,
    pub counter: i32,
}

#[derive(Serialize)]
pub struct DeviceCategory {
    pub name: String,
    pub counter: i32,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStats {
    pub operating_systems: Vec<OperatingSystem>,
    pub browsers: Vec<Browser>,
    pub categories: Vec<DeviceCategory>,
}

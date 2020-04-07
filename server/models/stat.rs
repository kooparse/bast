use super::schema::{day_stats, month_stats};
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

pub trait CmpStat {
    fn cmp(
        &mut self,
        is_new_user: bool,
        is_new_session: bool,
        is_bounce: bool,
        duration: f32,
    );
}

#[derive(PartialEq, Clone, Debug)]
pub struct Stat {
    pub id: i32,
    pub website_id: i32,
    pub pageviews: i32,
    pub users: i32,
    pub sessions: i32,
    pub avg_time: f32,
    pub bounce_rate: f32,
    pub known_time_counter: i32,
    pub bounce_counter: i32,
    pub created_at: NaiveDateTime,
}

#[derive(
    Serialize, Associations, AsChangeset, Queryable, PartialEq, Clone, Debug,
)]
pub struct MonthStat {
    pub id: i32,
    #[serde(rename(serialize = "websiteId"))]
    pub website_id: i32,
    pub pageviews: i32,
    pub users: i32,
    pub sessions: i32,
    #[serde(rename(serialize = "avgTime"))]
    pub avg_time: f32,
    #[serde(rename(serialize = "bounceRate"))]
    pub bounce_rate: f32,
    #[serde(skip)]
    pub known_time_counter: i32,
    #[serde(skip)]
    pub bounce_counter: i32,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: NaiveDateTime,
}

impl CmpStat for MonthStat {
    fn cmp(
        &mut self,
        is_new_user: bool,
        is_new_session: bool,
        is_bounce: bool,
        duration: f32,
    ) {
        self.pageviews += 1;

        if is_new_user {
            self.users += 1;
        }

        if is_new_session {
            self.sessions += 1;
        }

        if duration > 0. {
            self.known_time_counter += 1;
            self.avg_time =
                (self.avg_time + duration) / self.known_time_counter as f32;
        }

        if is_bounce {
            self.bounce_counter += 1;
            self.bounce_rate = (self.bounce_counter / self.pageviews) as f32;
        }
    }
}

#[derive(
    Serialize, Associations, AsChangeset, Queryable, PartialEq, Clone, Debug,
)]
pub struct DayStat {
    pub id: i32,
    #[serde(rename(serialize = "websiteId"))]
    pub website_id: i32,
    pub pageviews: i32,
    pub users: i32,
    pub sessions: i32,
    #[serde(rename(serialize = "avgTime"))]
    pub avg_time: f32,
    #[serde(rename(serialize = "bounceRate"))]
    pub bounce_rate: f32,
    #[serde(skip)]
    pub known_time_counter: i32,
    #[serde(skip)]
    pub bounce_counter: i32,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: NaiveDateTime,
}

impl CmpStat for DayStat {
    fn cmp(
        &mut self,
        is_new_user: bool,
        is_new_session: bool,
        is_bounce: bool,
        duration: f32,
    ) {
        self.pageviews += 1;

        if is_new_user {
            self.users += 1;
        }

        if is_new_session {
            self.sessions += 1;
        }

        if duration > 0. {
            self.known_time_counter += 1;
            self.avg_time =
                (self.avg_time + duration) / self.known_time_counter as f32;
        }

        if is_bounce {
            self.bounce_counter += 1;
            self.bounce_rate = (self.bounce_counter / self.pageviews) as f32;
        }
    }
}

use super::db::Conn;
use crate::models::{
    schema::{month_stats, pageviews, users, websites},
    User, Website,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;

pub fn seed_database(conn: &Conn) {
    delete(pageviews::table)
        .execute(conn)
        .expect("Error while clearing pageviews table.");
    delete(month_stats::table)
        .execute(conn)
        .expect("Error while clearing month stats table.");
    delete(websites::table)
        .execute(conn)
        .expect("Error while clearing website table.");
    delete(users::table)
        .execute(conn)
        .expect("Error while clearing users table.");

    let user_password =
        hash("password", DEFAULT_COST).expect("Failed to hash user password.");

    let users: Vec<User> = insert_into(users::table)
        .values(&vec![
            (
                users::id.eq(1),
                users::email.eq("test_123@test.com"),
                users::password.eq(&user_password),
            ),
            (
                users::id.eq(2),
                users::email.eq("test_321@test.com"),
                users::password.eq(&user_password),
            ),
        ])
        .get_results(conn)
        .expect("Error while seeding users.");

    let sites: Vec<Website> = insert_into(websites::table)
        .values(&vec![
            (
                websites::id.eq(1),
                websites::user_id.eq(users[0].id),
                websites::domain.eq("google.com"),
                websites::pageviews.eq(230),
                websites::users.eq(24),
                websites::sessions.eq(16),
            ),
            (
                websites::id.eq(2),
                websites::user_id.eq(users[0].id),
                websites::domain.eq("protonmail.com"),
                websites::pageviews.eq(20),
                websites::users.eq(2),
                websites::sessions.eq(4),
            ),
            (
                websites::id.eq(3),
                websites::user_id.eq(users[1].id),
                websites::domain.eq("kooparse.com"),
                websites::pageviews.eq(23),
                websites::users.eq(3),
                websites::sessions.eq(6),
            ),
        ])
        .get_results(conn)
        .expect("Error while seeding websites.");

    let months: Vec<NaiveDateTime> = {
        let months: [u8; 12] = [0; 12];
        months
            .iter()
            .enumerate()
            .map(|(i, _)| {
                NaiveDateTime::parse_from_str(
                    &format!("2019-{}-15 22:00:00", i + 1),
                    "%Y-%m-%d %H:%M:%S",
                )
                .expect("Error when parsing date.")
            })
            .collect()
    };

    let (old_dt, recent_dt) = {
        (
            NaiveDateTime::parse_from_str(
                "2020-01-23 23:56:04",
                "%Y-%m-%d %H:%M:%S",
            )
            .expect("Error when parsing date."),
            NaiveDateTime::parse_from_str(
                "2020-02-20 23:56:04",
                "%Y-%m-%d %H:%M:%S",
            )
            .expect("Error when parsing date."),
        )
    };

    insert_into(month_stats::table)
        .values(&vec![
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(20),
                month_stats::sessions.eq(23),
                month_stats::created_at.eq(&months[0]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(24),
                month_stats::sessions.eq(43),
                month_stats::created_at.eq(&months[1]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(36),
                month_stats::sessions.eq(1),
                month_stats::created_at.eq(&months[2]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(20),
                month_stats::sessions.eq(23),
                month_stats::created_at.eq(&months[3]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(220),
                month_stats::sessions.eq(53),
                month_stats::created_at.eq(&months[4]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(26),
                month_stats::sessions.eq(73),
                month_stats::created_at.eq(&months[5]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(2),
                month_stats::sessions.eq(45),
                month_stats::created_at.eq(&months[6]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(20),
                month_stats::sessions.eq(23),
                month_stats::created_at.eq(&months[7]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(20),
                month_stats::sessions.eq(23),
                month_stats::created_at.eq(&months[8]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(20),
                month_stats::sessions.eq(23),
                month_stats::created_at.eq(&months[9]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(20),
                month_stats::sessions.eq(23),
                month_stats::created_at.eq(&months[10]),
            ),
            (
                month_stats::website_id.eq(sites[0].id),
                month_stats::users.eq(20),
                month_stats::sessions.eq(23),
                month_stats::created_at.eq(&months[11]),
            ),
        ])
        .execute(conn)
        .expect("Error while seeding month stats.");

    insert_into(pageviews::table)
        .values(&vec![
            (
                pageviews::website_id.eq(sites[0].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/search"),
                pageviews::href.eq("https://google.com/"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://duckduckgo.com/"),
                pageviews::is_new_session.eq(true),
                pageviews::is_new_user.eq(true),
                pageviews::created_at.eq(&old_dt),
            ),
            (
                pageviews::website_id.eq(sites[0].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/search"),
                pageviews::href.eq("https://google.com/"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://duckduckgo.com/"),
                pageviews::is_new_session.eq(true),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&old_dt),
            ),
            (
                pageviews::website_id.eq(sites[0].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/found"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&recent_dt),
            ),
        ])
        .execute(conn)
        .expect("Error while seeding websites.");
}

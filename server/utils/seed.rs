use super::db::Conn;
use crate::models::{
    schema::{stats, pageviews, users, websites},
    User, Website,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;

#[allow(dead_code)]
pub fn reset_databse(conn: &Conn) {
    delete(pageviews::table)
        .execute(conn)
        .expect("Error while clearing pageviews table.");
    delete(stats::table)
        .execute(conn)
        .expect("Error while clearing stats table.");
    delete(websites::table)
        .execute(conn)
        .expect("Error while clearing website table.");
    delete(users::table)
        .execute(conn)
        .expect("Error while clearing users table.");
}

#[allow(dead_code)]
pub fn seed_database(conn: &Conn) {
    reset_databse(conn);
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
                websites::pageviews.eq(0),
                websites::users.eq(0),
                websites::sessions.eq(0),
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
    insert_into(stats::table)
        .values(&vec![
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(20),
                stats::sessions.eq(23),
                stats::created_at.eq(&old_dt),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(2),
                stats::sessions.eq(3),
                stats::created_at.eq(&recent_dt),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(10),
                stats::sessions.eq(13),
                stats::created_at.eq(&recent_dt),
            ),
        ])
        .execute(conn)
        .expect("Error while seeding month stats.");

    insert_into(pageviews::table)
        .values(&vec![
            (
                pageviews::website_id.eq(sites[1].id),
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
                pageviews::website_id.eq(sites[1].id),
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
                pageviews::website_id.eq(sites[1].id),
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

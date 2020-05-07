use super::db::Conn;
use crate::models::{
    schema::{pageviews, stats, users, websites},
    User, Website,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
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
                websites::avg_time.eq(190.),
            ),
            (
                websites::id.eq(2),
                websites::user_id.eq(users[0].id),
                websites::domain.eq("protonmail.com"),
                websites::pageviews.eq(20),
                websites::users.eq(124),
                websites::sessions.eq(170),
                websites::avg_time.eq(190.),
            ),
            (
                websites::id.eq(3),
                websites::user_id.eq(users[1].id),
                websites::domain.eq("kooparse.com"),
                websites::pageviews.eq(23),
                websites::users.eq(125),
                websites::sessions.eq(193),
                websites::avg_time.eq(190.),
            ),
        ])
        .get_results(conn)
        .expect("Error while seeding websites.");

    let mut dates: Vec<NaiveDateTime> = vec![];
    let mut current_date = Utc::now().naive_utc();
    dates.push(current_date);

    for _ in 1..11 {
        let year = current_date.year();
        let next_date = current_date.month() as i32 - 1;

        if next_date.is_negative() || next_date == 0 {
            current_date =
                NaiveDate::from_ymd(year - 1, 12, 1).and_hms(12, 00, 00);
        } else {
            current_date = NaiveDate::from_ymd(year, next_date as u32, 1)
                .and_hms(12, 00, 00);
        }

        dates.push(current_date);
    }

    insert_into(stats::table)
        .values(&vec![
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(23),
                stats::sessions.eq(12),
                stats::created_at.eq(&dates[0]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(8),
                stats::sessions.eq(23),
                stats::created_at.eq(&dates[1]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(10),
                stats::sessions.eq(16),
                stats::created_at.eq(&dates[2]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(20),
                stats::sessions.eq(10),
                stats::created_at.eq(&dates[3]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(10),
                stats::sessions.eq(2),
                stats::created_at.eq(&dates[4]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(8),
                stats::sessions.eq(22),
                stats::created_at.eq(&dates[5]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(9),
                stats::sessions.eq(13),
                stats::created_at.eq(&dates[6]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(6),
                stats::sessions.eq(16),
                stats::created_at.eq(&dates[7]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(32),
                stats::sessions.eq(18),
                stats::created_at.eq(&dates[8]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(27),
                stats::sessions.eq(20),
                stats::created_at.eq(&dates[9]),
            ),
            (
                stats::website_id.eq(sites[1].id),
                stats::users.eq(10),
                stats::sessions.eq(9),
                stats::created_at.eq(&dates[10]),
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
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:76.0) Gecko/20100101 Firefox/76.0"),
                pageviews::href.eq("https://google.com/"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://duckduckgo.com/"),
                pageviews::is_new_session.eq(true),
                pageviews::is_new_user.eq(true),
                pageviews::created_at.eq(&dates[0]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/search"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1 Safari/605.1.15"),
                pageviews::href.eq("https://google.com/"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://duckduckgo.com/"),
                pageviews::is_new_session.eq(true),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[1]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/ddd"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/aaaa"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/dfg/vdcg"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/sdfjkh"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/sdjkfh"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/sdjkfhvfv"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/ffser"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
            (
                pageviews::website_id.eq(sites[1].id),
                pageviews::u_id.eq("localhost:3000_Mozilla"),
                pageviews::pathname.eq("/ffser"),
                pageviews::user_agent.eq("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"),
                pageviews::href.eq("https://google.com/found"),
                pageviews::hostname.eq("www.google.com"),
                pageviews::referrer.eq("https://kooparse.com/"),
                pageviews::is_new_session.eq(false),
                pageviews::is_new_user.eq(false),
                pageviews::created_at.eq(&dates[2]),
            ),
        ])
        .execute(conn)
        .expect("Error while seeding websites.");
}

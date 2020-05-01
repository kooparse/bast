use crate::models::SlimStat;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use std::collections::BTreeMap;

pub static MONTHLY_FORMAT: &str = "%Y-%m";
pub static DAILY_FORMAT: &str = "%Y-%m-%d";

pub fn get_months(
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> BTreeMap<String, SlimStat> {
    let mut current_month =
        NaiveDate::from_ymd(start.year(), start.month(), 1).and_hms(12, 00, 00);

    let mut months: BTreeMap<String, SlimStat> = BTreeMap::new();
    let month_diff = end.signed_duration_since(start).num_days() / 30;

    for _ in 0..month_diff {
        let year = current_month.year();
        let next_month = current_month.month() + 1;

        if current_month.with_month(next_month).is_some() {
            current_month = NaiveDate::from_ymd(year, next_month as u32, 1)
                .and_hms(12, 00, 00);
        } else {
            current_month =
                NaiveDate::from_ymd(year + 1, 1, 1).and_hms(12, 00, 00);
        }

        months.insert(
            current_month.format(MONTHLY_FORMAT).to_string(),
            SlimStat::default(),
        );
    }

    months
}

pub fn get_days(
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> BTreeMap<String, SlimStat> {
    let mut current_day =
        NaiveDate::from_ymd(start.year(), start.month(), start.day())
            .and_hms(12, 00, 00);

    let mut days: BTreeMap<String, SlimStat> = BTreeMap::new();
    let day_diff = end.signed_duration_since(start).num_days();

    for _ in 0..=day_diff {
        let year = current_day.year();
        let month = current_day.month();
        let next_day = current_day.day() + 1;

        if current_day.with_day(next_day as u32).is_some() {
            current_day = NaiveDate::from_ymd(year, month, next_day as u32)
                .and_hms(12, 00, 00);
        } else if current_day.with_month(month + 1).is_some() {
            current_day =
                NaiveDate::from_ymd(year, month + 1, 1).and_hms(12, 00, 00);
        } else {
            current_day =
                NaiveDate::from_ymd(year + 1, 1, 1).and_hms(12, 00, 00);
        }

        dbg!(&current_day);

        days.insert(
            current_day.format(DAILY_FORMAT).to_string(),
            SlimStat::default(),
        );
    }

    days
}

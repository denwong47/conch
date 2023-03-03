use crate::{config, func, LastWeekdayOfMonth, NextWeekdayFromDate, RegionMarker};
use chrono::{Duration, NaiveDate, Weekday};

/// State struct for marking regions.
pub struct England;
impl RegionMarker for England {
    fn starts_week_with() -> Option<Weekday> {
        Some(Weekday::Mon)
    }

    fn list_holidays(year: i32) -> Vec<chrono::NaiveDate> {
        let weekdays = Vec::from_iter(config::WEEKDAYS.into_iter());

        let easter_day = func::get_easter_date(year);
        let xmas_day = NaiveDate::from_ymd_opt(year, 12, 25)
            .and_then(|date| date.next_weekday_from(&weekdays));
        let boxing_day = (xmas_day.unwrap() + Duration::days(1)).next_weekday_from(&weekdays);

        Option::from_iter(
            vec![
                NaiveDate::from_ymd_opt(year, 1, 1)
                    .and_then(|date| date.next_weekday_from(&weekdays)), // New Year's Day
                Some(easter_day - Duration::days(2)), // Good Friday
                Some(easter_day + Duration::days(1)), // Easter Monday
                NaiveDate::from_weekday_of_month_opt(year, 5, Weekday::Mon, 1), // Early May Bank Holiday
                NaiveDate::last_weekday_of_month_opt(year, 5, Weekday::Mon), // Spring Bank Holiday
                NaiveDate::last_weekday_of_month_opt(year, 8, Weekday::Mon), // Summer Bank Holiday
                xmas_day,                                                    // Christmas Day
                boxing_day,                                                  // Boxing Day
            ]
            .into_iter()
            .filter(|o| o.is_some()),
        )
        .unwrap_or(vec![])
    }
}

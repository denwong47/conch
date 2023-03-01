use std::marker::PhantomData;

use chrono::{Duration, NaiveDate, Weekday};
// use chrono::offset::Local;

use crate::{
    config, func, regions, HolidayList, LastWeekdayOfMonth, NextWeekdayFromDate, RegionMarker,
};

/// Structs for static method to list public holidays of a region in a specific year.
pub struct Holidays<Region, const YEAR: i32>
where
    Region: RegionMarker,
{
    region: PhantomData<Region>,
}

/// Generate list of bank holidays for a given year.
impl<const YEAR: i32> HolidayList for Holidays<regions::England, YEAR> {
    fn list() -> Vec<NaiveDate> {
        let weekdays = Vec::from_iter(config::WEEKDAYS.into_iter());

        let easter_day = func::get_easter_date(YEAR);
        let xmas_day = NaiveDate::from_ymd_opt(YEAR, 12, 25)
            .and_then(|date| date.next_weekday_from(&weekdays));
        let boxing_day = (xmas_day.unwrap() + Duration::days(1)).next_weekday_from(&weekdays);

        Option::from_iter(
            vec![
                NaiveDate::from_ymd_opt(YEAR, 1, 1)
                    .and_then(|date| date.next_weekday_from(&weekdays)), // New Year's Day
                Some(easter_day - Duration::days(2)), // Good Friday
                Some(easter_day + Duration::days(1)), // Easter Monday
                NaiveDate::from_weekday_of_month_opt(YEAR, 5, Weekday::Mon, 1), // Early May Bank Holiday
                NaiveDate::last_weekday_of_month_opt(YEAR, 5, Weekday::Mon), // Spring Bank Holiday
                NaiveDate::last_weekday_of_month_opt(YEAR, 8, Weekday::Mon), // Summer Bank Holiday
                xmas_day,                                                    // Christmas Day
                boxing_day,                                                  // Boxing Day
            ]
            .into_iter()
            .filter(|o| o.is_some()),
        )
        .unwrap_or(vec![])
    }
}

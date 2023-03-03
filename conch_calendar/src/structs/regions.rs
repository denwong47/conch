use crate::{config, func, LastWeekdayOfMonth, NextWeekdayFromDate, RegionMarker};
use chrono::{Datelike, Duration, NaiveDate, Weekday};

/// State Struct representing he [Region] of England, part of the United Kingdom.
///
/// Due to devolved powers, England does not necessarily share its public holidays
/// called Bank Holidays with Scotland, Wales and Northern Ireland. This struct is only
/// usable for England.
///
/// Most Bank Holidays in England are calculated by first/last of a certain weekday
/// in a given month, hence implementation of [`LastWeekdayOfMonth`] is required.
///
/// Whenever a Bank Holiday coincides with a Saturday or Sunday, a substitute Bank
/// Holiday is issued on the following Monday. Such calculations are facilitated by
/// the [`NextWeekdayFromDate`] trait.
///
/// [Region]: RegionMarker
pub struct England {
    _private: bool, // Prevent instantiation.
}
impl RegionMarker for England {
    /// Defines that England starts a week on Monday.
    fn starts_week_with() -> Option<Weekday> {
        Some(Weekday::Mon)
    }

    /// List all the Bank Holidays in England for a given year.
    ///
    /// Bank Holidays Act came into force in 1871; thus no years prior will return
    /// any dates at all.
    fn list_holidays(year: i32) -> Vec<NaiveDate> {
        // Bank Holidays Act 1871
        if year >= 1871 {
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
                    Self::get_early_may_bank_holiday(year), // Early May Bank Holiday
                    Self::get_spring_bank_holiday(year),  // Spring Bank Holiday
                    Self::get_summer_bank_holiday(year),  // Summer Bank Holiday
                    xmas_day,                             // Christmas Day
                    boxing_day,                           // Boxing Day
                ]
                .into_iter()
                .filter(|o| o.is_some())
                .chain(Self::get_special_holidays(year)),
            )
            .unwrap_or(vec![])
        } else {
            // Before we have bank holidays!
            vec![]
        }
    }
}
impl England {
    fn get_early_may_bank_holiday(year: i32) -> Option<NaiveDate> {
        match year {
            // VE Day
            1995 => NaiveDate::from_ymd_opt(1995, 5, 8),

            // VE Day
            2020 => NaiveDate::from_ymd_opt(2020, 5, 8),
            year if year >= 1978 => NaiveDate::from_weekday_of_month_opt(year, 5, Weekday::Mon, 1),
            _ => None,
        }
    }

    fn get_spring_bank_holiday(year: i32) -> Option<NaiveDate> {
        match year {
            // Moved due to Golden Jubilee bank holiday
            2002 => NaiveDate::from_ymd_opt(2002, 6, 4),

            // Moved due to Diamond Jubilee bank holiday
            2012 => NaiveDate::from_ymd_opt(2012, 6, 4),

            // Moved due to Platinum Jubilee bank holiday
            2022 => NaiveDate::from_ymd_opt(2022, 6, 2),

            // Banking and Financial Dealings Act 1971
            year if year >= 1965 => NaiveDate::last_weekday_of_month_opt(year, 5, Weekday::Mon),

            // Traditionally, Monday after Pentecost.
            year if year >= 1871 => (func::get_easter_date(year) + Duration::days(49))
                .next_weekday_from(&vec![Weekday::Mon]),

            // Before Bank Holidays Act 1871
            _ => None,
        }
    }

    fn get_summer_bank_holiday(year: i32) -> Option<NaiveDate> {
        match year {
            // Weird calculations before the current method was standardised in 1971
            1968 => NaiveDate::from_ymd_opt(1968, 9, 2),
            1969 => NaiveDate::from_ymd_opt(1969, 9, 1),

            // Banking and Financial Dealings Act 1971
            year if year >= 1965 => NaiveDate::last_weekday_of_month_opt(year, 8, Weekday::Mon),

            // Traditionally, First Monday of August
            year if year >= 1871 => NaiveDate::from_weekday_of_month_opt(year, 8, Weekday::Mon, 1),

            // Before Bank Holidays Act 1871
            _ => None,
        }
    }

    fn get_special_holidays(year: i32) -> impl Iterator<Item = Option<NaiveDate>> {
        macro_rules! list_dates {
            ($(($year:literal, $month:literal, $day:literal, $desc:literal),)+) => {
                vec![
                    $(
                        NaiveDate::from_ymd_opt($year, $month, $day),
                    )*
                ]
            };
        }

        list_dates!(
            (
                2023,
                5,
                8,
                "Bank holiday for the coronation of King Charles III"
            ),
            (
                2022,
                9,
                19,
                "Bank Holiday for the State Funeral of Queen Elizabeth II"
            ),
            (2022, 6, 3, "Platinum Jubilee bank holiday"),
            (2012, 6, 5, "Diamond Jubilee of Elizabeth II"),
            (2002, 6, 3, "Golden Jubilee of Elizabeth II"),
        )
        .into_iter()
        .filter(move |date_opt| {
            date_opt
                .and_then(|date| if date.year() == year { Some(()) } else { None })
                .is_some()
        })
    }
}

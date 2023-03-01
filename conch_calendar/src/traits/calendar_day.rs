use crate::{CalendarMonth, RegionMarker};
use chrono::{NaiveDate, Weekday};

/// Trait for printing out a day in a Calendar.
pub(crate) trait DisplayCalendarDay {
    fn to_display_on_calendar<Region: RegionMarker>(
        &self,
        calendar: &CalendarMonth<Region>,
    ) -> String;
}

//

use chrono::offset::Local;
use chrono::{Datelike, NaiveDate};

use crate::{config, CalendarMonth, ContainsDate, RegionMarker};
use conch_ansi::Modifier;
use conch_base_models::StringWrapper;

/// Trait for printing out a day in a Calendar.
pub(crate) trait DisplayCalendarDay {
    fn get_modifier<'a, Region: RegionMarker>(
        &self,
        calendar: &'a CalendarMonth<Region>,
    ) -> &'a Modifier;

    fn to_display_on_calendar<Region: RegionMarker>(
        &self,
        calendar: &CalendarMonth<Region>,
    ) -> String;
}

impl DisplayCalendarDay for NaiveDate {
    fn get_modifier<'a, Region: RegionMarker>(
        &self,
        calendar: &'a CalendarMonth<Region>,
    ) -> &'a Modifier {
        // Check for today.
        if (self == &Local::now().date_naive()) && calendar.today_modifier.is_some() {
            return calendar.today_modifier.as_ref().unwrap();
        }

        // Check for other months.
        if !calendar.contains(self) {
            return &calendar.other_month_modifier;
        }

        // Check for decorations.
        if calendar.decorated_days.contains_key(self) {
            return calendar.decorated_days.get(self).unwrap();
        }

        // Check for Sundays or public holidays.
        if config::SUNDAYS.contains(&self.weekday()) || calendar.holidays.contains(self) {
            return &calendar.holiday_modifier;
        }

        &calendar.weekday_modifier
    }

    fn to_display_on_calendar<Region: RegionMarker>(
        &self,
        calendar: &CalendarMonth<Region>,
    ) -> String {
        if !calendar.show_other_months && !calendar.contains(self) {
            // If the month is wrong and the calendar does not display it, just return
            // a couple of spaces.
            format!("{:2}", "")
        } else {
            let modifier = self.get_modifier(calendar);
            let number_str = format!("{:2}", self.day());

            modifier.wraps(&number_str)
        }
    }
}

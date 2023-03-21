use chrono::offset::Local;
use chrono::{Datelike, NaiveDate};

use crate::{config, CalendarMonth, ContainsDate, RegionMarker};
use conch_ansi::Modifier;
use conch_base_models::StringWrapper;

/// Trait for printing out a day in a Calendar.
pub trait DisplayCalendarDay {
    fn get_modifier<'a, Region: RegionMarker>(
        &self,
        calendar: &'a CalendarMonth<Region>,
    ) -> &'a Modifier;

    fn to_display_on_calendar<Region: RegionMarker>(
        &self,
        calendar: &CalendarMonth<Region>,
    ) -> String;

    fn calendar_col_row_of<Region: RegionMarker>(
        &self,
        calendar: &CalendarMonth<Region>,
    ) -> Option<(u32, u32)>;
}

impl DisplayCalendarDay for NaiveDate {
    /// Get the appropriate modifier for this date on specified calendar.
    ///
    /// It evaluates this date in sequence:
    ///
    /// - If it is today, and [today_modifier] is defined, then return a
    ///   reference to that. Otherwise, continue on.
    /// - If the date is not inside the calendar, returns a reference to
    ///   the [other_month_modifier], regardless of whether the calendar
    ///   [show_other_months].
    /// - If the date is decorated by the calendar, returns a reference
    ///   to its decoration modifier.
    /// - If the date is a sunday or a public holiday, returns a
    ///   reference to the the [holiday_modifier].
    /// - Otherwise, returns a reference to the [weekday_modifier].
    ///
    /// [today_modifier]: CalendarMonth<Region>::today_modifier
    /// [other_month_modifier]: CalendarMonth<Region>::other_month_modifier
    /// [show_other_months]: CalendarMonth<Region>::show_other_months
    /// [holiday_modifier]: CalendarMonth<Region>::holiday_modifier
    /// [weekday_modifier]: CalendarMonth<Region>::weekday_modifier
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

    /// Returns the formatted and decorated string to be displayed
    /// on the calendar.
    ///
    /// This does not include the cursor shifting to the position
    /// required.
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

    /// Find the column and row number of the specified date.
    ///
    /// Returns [`None`] if the date is not in `calendar`.
    fn calendar_col_row_of<Region: RegionMarker>(
        &self,
        calendar: &CalendarMonth<Region>,
    ) -> Option<(u32, u32)> {
        calendar.week_number_of(self).map(|row| {
            let col = calendar.num_days_from_start_of_week(self);

            (col, row)
        })
    }
}

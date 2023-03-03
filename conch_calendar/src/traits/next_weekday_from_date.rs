use chrono::{Datelike, Duration, NaiveDate, Weekday};

/// Trait for adding `last_weekday_of_month_opt` method to `NaiveDate`.
pub trait NextWeekdayFromDate
where
    Self: Sized,
{
    /// Get the next weekday from the specified date.
    ///
    /// If the day itself is the given weekday, it will return itself.
    fn next_weekday_from(&self, weekdays: &Vec<Weekday>) -> Option<Self>;
}

impl NextWeekdayFromDate for NaiveDate {
    /// Get the next weekday from the specified date.
    ///
    /// If the day itself is the given weekday, it will return itself.
    /// If `weekdays` is empty, return [`None`].
    fn next_weekday_from(&self, weekdays: &Vec<Weekday>) -> Option<Self> {
        if weekdays.len() == 0 {
            None
        } else {
            let mut cur_date = self.clone();

            while !weekdays.contains(&cur_date.weekday()) {
                cur_date += Duration::days(1);
            }

            Some(cur_date)
        }
    }
}

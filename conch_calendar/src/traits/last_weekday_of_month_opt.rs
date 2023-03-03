use chrono::{NaiveDate, Weekday};

/// Trait for adding `last_weekday_of_month_opt` method to `NaiveDate`.
pub trait LastWeekdayOfMonth {
    /// Get the last weekday of the specified month.
    ///
    /// As there always is a last weekday of a month, this method is guaranteed to
    /// succeed despite returning an [`Option<NaiveDate>`]; this is to match the syntax
    /// of [`NaiveDate::from_weekday_of_month_opt()`].
    fn last_weekday_of_month_opt(year: i32, month: u32, weekday: Weekday) -> Option<NaiveDate>;

    /// Get the last weekday of the specified month.
    ///
    /// As there always is a last weekday of a month, this method is guaranteed to
    /// succeed.
    fn last_weekday_of_month(year: i32, month: u32, weekday: Weekday) -> NaiveDate {
        Self::last_weekday_of_month_opt(year, month, weekday).unwrap()
    }
}

impl LastWeekdayOfMonth for NaiveDate {
    /// Get the last weekday of the specified month.
    ///
    /// As there always is a last weekday of a month, this method is guaranteed to
    /// succeed despite returning an [`Option<NaiveDate>`]; this is to match the syntax
    /// of [`NaiveDate::from_weekday_of_month_opt()`].
    fn last_weekday_of_month_opt(year: i32, month: u32, weekday: Weekday) -> Option<NaiveDate> {
        (1..6)
            .into_iter()
            .map(|n| NaiveDate::from_weekday_of_month_opt(year, month, weekday, n))
            .fold(None, |lhs, rhs| if rhs.is_some() { rhs } else { lhs })
    }
}

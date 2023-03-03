use chrono::{NaiveDate, Weekday};

/// Marker Trait for a Region.
pub trait RegionMarker {
    /// Defines the [`Weekday`] that a week begins in.
    fn starts_week_with() -> Option<Weekday>;

    /// List the holidays in a given year.
    fn list_holidays(year: i32) -> Vec<NaiveDate>;
}

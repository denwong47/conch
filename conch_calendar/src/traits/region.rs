use chrono::NaiveDate;

/// Marker Trait for a Region.
pub trait RegionMarker {
    fn list_holidays(year: i32) -> Vec<NaiveDate>;
}

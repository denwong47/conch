use chrono::NaiveDate;

/// Trait for structs that can output a structure
pub trait HolidayList {
    fn list(year: i32) -> Vec<NaiveDate>;
}

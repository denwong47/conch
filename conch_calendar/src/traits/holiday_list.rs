use chrono::NaiveDate;

/// Trait for structs that can output a structure
pub trait HolidayList {
    fn list() -> Vec<NaiveDate>;
}

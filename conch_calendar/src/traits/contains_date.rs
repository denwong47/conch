use chrono::NaiveDate;

/// Trait to check if a date is within another struct of object, presumably a range
/// of dates.
pub trait ContainsDate {
    fn contains(&self, date: &NaiveDate) -> bool;
}

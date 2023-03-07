use chrono::NaiveDate;

/// Trait to check if a date is within another struct of object, presumably a range
/// of dates.
pub trait ContainsDate {
    fn contains(&self, date: &NaiveDate) -> bool;
}

/// Trait to check if a date is within another struct of object, presumably a range
/// of dates.
pub trait ContainsDateMut {
    fn contains(&mut self, date: &NaiveDate) -> bool;
}

impl<'a, T> ContainsDate for Vec<T>
where
    T: ContainsDate,
{
    fn contains(&self, date: &NaiveDate) -> bool {
        self.iter().any(|item| item.contains(date))
    }
}

impl<'a, I, T> ContainsDateMut for I
where
    I: Iterator<Item = T>,
    T: ContainsDate,
{
    fn contains(&mut self, date: &NaiveDate) -> bool {
        self.any(|item| item.contains(date))
    }
}

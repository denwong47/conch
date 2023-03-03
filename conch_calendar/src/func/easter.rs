use chrono::NaiveDate;

/// Calculate Easter Sunday date of a given year.
///
/// Reference: <https://stackoverflow.com/questions/2510383/how-can-i-calculate-what-date-good-friday-falls-on-given-a-year>
pub fn get_easter_date(year: i32) -> NaiveDate {
    let g = year % 19;
    let c = year / 100;
    let h = (c - c / 4 - ((8 * c + 13) / 25) + 19 * g + 15) % 30;
    let i = h - (h / 28) * (1 - (h / 28) * (29 / (h + 1)) * ((21 - g) / 11));

    let mut day: i32 = i - ((year + (year / 4) + i + 2 - c + (c / 4)) % 7) + 28;
    let mut month: i32 = 3;

    if day > 31 {
        month += 1;
        day -= 31;
    }

    NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
}

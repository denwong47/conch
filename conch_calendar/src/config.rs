use chrono::Weekday;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SUNDAYS: [Weekday; 1] = [Weekday::Sun];
    pub static ref WEEKENDS: [Weekday; 2] = [Weekday::Sat, Weekday::Sun];
    pub static ref WEEKDAYS: [Weekday; 5] = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
    ];
}

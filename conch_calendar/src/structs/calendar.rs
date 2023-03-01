use std::collections::HashMap;
use std::marker::PhantomData;

use lazy_static::lazy_static;

#[allow(unused_imports)]
use chrono::{NaiveDate, NaiveWeek, Weekday};

use crate::RegionMarker;
use conch_ansi::Modifier;

lazy_static! {
    pub static ref DEFAULT_WEEK_STARTS_WITH: Weekday = Weekday::Mon;
    pub static ref DEFAULT_WEEKEND_MODIFIER: Modifier =
        Modifier::colour("BrightRed").unwrap() + Modifier::intensity("Bold").unwrap();
    pub static ref DEFAULT_WEEKDAY_MODIFIER: Modifier = Modifier::colour("White").unwrap();
    pub static ref DEFAULT_OTHER_MONTH_MODIFIER: Modifier =
        Modifier::colour("Grayscale13").unwrap();
}

/// A struct to display
pub struct CalendarMonth<Region>
where
    Region: RegionMarker,
{
    pub month: NaiveDate,
    region: PhantomData<Region>,

    // Date related presentation settings
    pub week_starts_with: Weekday,

    // Modifiers
    pub other_month_modifier: Modifier,
    pub weekday_modifier: Modifier,
    pub weekend_modifier: Modifier,
    pub today_modifier: Option<Modifier>,

    // Decorated Days
    pub decorated_days: HashMap<NaiveDate, Modifier>,
}
impl<Region> CalendarMonth<Region>
where
    Region: RegionMarker,
{
    /// Create a new [`CalendarMonth`] from a [`NaiveDate`] provided.
    pub fn new(month: NaiveDate) -> Self {
        Self {
            month,
            region: PhantomData,

            other_month_modifier: DEFAULT_OTHER_MONTH_MODIFIER.clone(),
            week_starts_with: DEFAULT_WEEK_STARTS_WITH.clone(),
            weekday_modifier: DEFAULT_WEEKDAY_MODIFIER.clone(),
            weekend_modifier: DEFAULT_WEEKEND_MODIFIER.clone(),
            today_modifier: None,

            decorated_days: HashMap::new(),
        }
    }
}

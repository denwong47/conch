use std::collections::HashMap;
use std::iter;
use std::marker::PhantomData;

use lazy_static::lazy_static;

#[allow(unused_imports)]
use chrono::{Datelike, Duration, NaiveDate, NaiveWeek, Weekday};

use crate::{
    ContainsDate, DisplayCalendarDay, HolidayList, Holidays, IterRangeByDuration, RegionMarker,
};
use conch_ansi::Modifier;
use conch_base_models::StringWrapper;

lazy_static! {
    pub static ref DEFAULT_TITLE_MODIFIER: Modifier = Modifier::intensity("Bold").unwrap();
    pub static ref DEFAULT_WEEK_STARTS_WITH: Weekday = Weekday::Mon;
    pub static ref DEFAULT_WEEKDAY_MODIFIER: Modifier = Modifier::Nothing;
    pub static ref DEFAULT_HOLIDAY_MODIFIER: Modifier =
        Modifier::colour("BrightRed").unwrap() + Modifier::intensity("Bold").unwrap();
    pub static ref DEFAULT_OTHER_MONTH_MODIFIER: Modifier =
        Modifier::colour("Grayscale13").unwrap();
}

/// A struct to display
pub struct CalendarMonth<Region>
where
    Region: RegionMarker,
{
    pub date: NaiveDate,
    region: PhantomData<Region>,

    pub(crate) holidays: Vec<NaiveDate>,

    // Date related presentation settings
    pub week_starts_with: Weekday,
    pub show_title: bool,
    pub show_other_months: bool,
    pub captialize_title: bool,

    // Modifiers
    pub title_modifier: Modifier,
    pub other_month_modifier: Modifier,
    pub weekday_modifier: Modifier,
    pub holiday_modifier: Modifier,
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
            date: month - Duration::days((month.day() - 1) as i64),
            region: PhantomData,

            holidays: Holidays::<Region>::list(month.year()),

            week_starts_with: DEFAULT_WEEK_STARTS_WITH.clone(),
            show_title: true,
            show_other_months: false,

            captialize_title: true,

            title_modifier: DEFAULT_TITLE_MODIFIER.clone(),
            other_month_modifier: DEFAULT_OTHER_MONTH_MODIFIER.clone(),
            weekday_modifier: DEFAULT_WEEKDAY_MODIFIER.clone(),
            holiday_modifier: DEFAULT_HOLIDAY_MODIFIER.clone(),
            today_modifier: None,

            decorated_days: HashMap::new(),
        }
    }

    /// Add a special [`Modifier`] to a single date.
    pub fn decorate_day(mut self, date: NaiveDate, modifier: Modifier) -> Self {
        // Swap with a placeholder value to avoid messing with hash table
        let existing_modifier = self
            .decorated_days
            .insert(date, Modifier::Nothing)
            .unwrap_or(Modifier::Nothing);

        // Swap out the placeholder value
        self.decorated_days
            .insert(date, existing_modifier + modifier);

        self
    }

    /// Generate the title string for the calendar month.
    pub(crate) fn title(&self) -> String {
        let mut weekday = self.week_starts_with;

        self.title_modifier.wraps(
            &(0..7)
                .map(|_| {
                    let s = format!("{:>2}", weekday.to_string().chars().next().unwrap());
                    weekday = weekday.succ();

                    s
                })
                .reduce(|lhs, rhs| lhs + " " + &rhs)
                .unwrap(),
        )
    }
}

macro_rules! expand_params {
    (
        $((
            $name:ident,
            $param:ident,
            $type:ty
        )),+
        $(,)?
    ) => {
        $(
            impl<Region> CalendarMonth<Region>
            where
                Region: RegionMarker,
            {
                #[doc = "Chained method to change the [`Self::"]
                #[doc = stringify!($param)]
                #[doc = "`] of a [`CalendarMonth`] instance."]
                pub fn $name(mut self, value: $type) -> Self {
                    self.$param = value;
                    self
                }
            }
        )*
    }
}
expand_params!(
    (modify_title, title_modifier, Modifier),
    (modify_weekdays, weekday_modifier, Modifier),
    (modify_holidays, holiday_modifier, Modifier),
    (modify_today, today_modifier, Option<Modifier>),
    (modify_other_months, other_month_modifier, Modifier),
    (starts_week_with, week_starts_with, Weekday),
    (captialize_title, captialize_title, bool),
    (show_title, show_title, bool),
    (show_other_months, show_other_months, bool),
);

impl<Region> ContainsDate for CalendarMonth<Region>
where
    Region: RegionMarker,
{
    /// Check if a date is inside the calendar month.
    fn contains(&self, date: &NaiveDate) -> bool {
        return self.date.month() == date.month() && self.date.year() == date.year();
    }
}

impl<Region> From<&CalendarMonth<Region>> for Vec<String>
where
    Region: RegionMarker,
{
    /// Parse a calendar into display strings.
    fn from(value: &CalendarMonth<Region>) -> Self {
        let weeks: Vec<NaiveWeek> = Option::from_iter(
            (0..6)
                .map(
                    // Get the weeks we need to print.
                    |week_no| {
                        let week =
                            (value.date + Duration::days(7 * week_no)).week(value.week_starts_with);

                        if value.contains(&week.first_day()) || value.contains(&week.last_day()) {
                            Some(week)
                        } else {
                            None
                        }
                    },
                )
                .filter(|week| week.is_some()),
        )
        .unwrap();

        let week_rows = weeks.iter().map(
            // For each week, we gather the days and print each one out.
            |week| {
                week.days()
                    .into_iter_by_duration(Duration::days(1))
                    .map(|date| date.to_display_on_calendar(&value))
                    .reduce(|lhs, rhs| lhs + " " + &rhs)
                    .unwrap_or(String::new())
            },
        );

        if value.show_title {
            iter::once(value.title()).chain(week_rows).collect()
        } else {
            week_rows.collect()
        }
    }
}

impl<Region> From<CalendarMonth<Region>> for Vec<String>
where
    Region: RegionMarker,
{
    fn from(value: CalendarMonth<Region>) -> Self {
        Self::from(&value)
    }
}

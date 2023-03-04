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
use conch_split::Lines;

lazy_static! {
    pub static ref DEFAULT_TITLE_MODIFIER: Modifier = Modifier::intensity("Bold").unwrap();
    pub static ref DEFAULT_WEEK_STARTS_WITH: Weekday = Weekday::Mon;
    pub static ref DEFAULT_WEEKDAY_MODIFIER: Modifier = Modifier::Nothing;
    pub static ref DEFAULT_HOLIDAY_MODIFIER: Modifier =
        Modifier::colour("BrightRed").unwrap() + Modifier::intensity("Bold").unwrap();
    pub static ref DEFAULT_OTHER_MONTH_MODIFIER: Modifier =
        Modifier::colour("Grayscale13").unwrap();
}

/// A struct to display a month on a calendar in stdout.
///
/// Allows special highlighting to
/// - public holidays,
/// - days outside of the current month, or
/// - any arbitrary days.
///
/// Example:
///
/// ```rust
/// use chrono::NaiveDate;
///
/// use conch::{
///     s, // macro for combining string literals, for assertion only.
///     Modifier, // ANSI styling tool
///     Lines, // Helper struct to format a [`Vec<String>`] for stdout.
///     CalendarMonth,
///     regions // Defining the region of [`CalendarMonth`].
/// };
///
/// let today_highlight =
///     Modifier::colour("BrightBlue").unwrap()
///     + Modifier::intensity("Bold").unwrap()
/// ;
/// let today = NaiveDate::from_ymd_opt(2023, 3, 3).unwrap();
///
/// let calendar: CalendarMonth<regions::England> =
///     CalendarMonth::new(today)
///     .modify_today(Some(today_highlight))
/// ;
///
/// // Works with both owned and `&calendar`
/// let lines = Lines::from(calendar);
///
/// println!("{}", &lines);
/// assert_eq!(lines.to_string(), s!(
///     "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m\n"
///     "       1  2 \u{1b}[38;5;12m\u{1b}[1m 3\u{1b}[22m\u{1b}[39m  4 \u{1b}[38;5;9m\u{1b}[1m 5\u{1b}[22m\u{1b}[39m\n"
///     " 6  7  8  9 10 11 \u{1b}[38;5;9m\u{1b}[1m12\u{1b}[22m\u{1b}[39m\n"
///     "13 14 15 16 17 18 \u{1b}[38;5;9m\u{1b}[1m19\u{1b}[22m\u{1b}[39m\n"
///     "20 21 22 23 24 25 \u{1b}[38;5;9m\u{1b}[1m26\u{1b}[22m\u{1b}[39m\n"
///     "27 28 29 30 31      "
/// ));
/// ```
///
/// This will produce a printout like:
/// ```text
///  M  T  W  T  F  S  S
///        1  2 *3* 4 *5*
///  6  7  8  9 10 11*12*
/// 13 14 15 16 17 18*19*
/// 20 21 22 23 24 25*26*
/// 27 28 29 30 31
/// ```
/// with `*\d+*` denoting a colour.
///
/// Assuming today is the 3rd, then `3` will be in
/// bright blue as dictated by `today_highlight` above.
///
/// A number of chained modifiers are available. For example, the above
/// example is equivalent to:
///
/// ```rust
/// use chrono::{NaiveDate, Weekday};
///
/// use conch::{
///     Modifier, // ANSI styling tool
///     CalendarMonth,
///     regions // Defining the region of [`CalendarMonth`].
/// };
///
/// let today_highlight =
///     Modifier::colour("BrightBlue").unwrap()
///     + Modifier::intensity("Bold").unwrap()
/// ;
/// let today = NaiveDate::from_ymd_opt(2023, 3, 3).unwrap();
///
/// let calendar: CalendarMonth<regions::England> =
///     CalendarMonth::new(today)
///     .starts_week_with(Weekday::Mon)
///     .show_title(true)
///     .show_other_months(false)
///     .capitalize_title(true)
///     .modify_title(
///         Modifier::colour("Grayscale13").unwrap()
///     )
///     .modify_weekdays(Modifier::Nothing)
///     .modify_holidays(
///         Modifier::colour("BrightRed").unwrap()
///         + Modifier::intensity("Bold").unwrap()
///     )
///     .modify_today(Some(today_highlight))
/// ;
/// ```
pub struct CalendarMonth<Region>
where
    Region: RegionMarker,
{
    /// [`NaiveDate`] that this calendar is based on. The [day] will
    /// be discarded upon instantiation, and replaced with the first day
    /// of the month.
    ///
    /// [day]: Datelike::day()
    pub date: NaiveDate,

    region: PhantomData<Region>,

    /// Cache the holidays relevant to us.
    pub(crate) holidays: Vec<NaiveDate>,

    /// Cache the total number of weeks in this month.
    /// This excludes the title.
    pub(crate) weeks_count: u32,

    // Date related presentation settings
    /// The [`Weekday`] to starts each week with.
    /// Each region can set its own [default] which is used unless overridden
    /// in this [`CalendarMonth`] instance.
    ///
    /// Use [`Self::starts_week_with()`] to change this.
    ///
    /// [default]: RegionMarker::starts_week_with
    pub week_starts_with: Weekday,

    /// Defines whether the weekday initials like `M  T  W  T  F  S  S`
    /// should be shown.
    ///
    /// Use [`Self::show_title()`] to change this.
    pub show_title: bool,

    /// Defines whether to show the days from another month, if the
    /// starting and ending weeks of the month extends
    /// into another month.
    ///
    /// Use [`Self::show_other_months()`] to change this.
    pub show_other_months: bool,

    /// Defines whether to capitalize the title row, so that instead of
    /// `M  T  W  T  F  S  S`, `m  t  w  t  f  s  s` will be displayed.
    ///
    /// Use [`Self::capitalize_title()`] to change this.
    pub capitalize_title: bool,

    // Modifiers
    /// Modifier for the title row.
    ///
    /// Use [`Self::modify_title()`] to change this.
    pub title_modifier: Modifier,

    /// Modifier for the days from other months.
    /// This will only be applied if [`Self::show_other_months`] is `true`.
    ///
    /// Use [`Self::modify_other_months()`] to change this.
    pub other_month_modifier: Modifier,

    /// Modifier for any undecorated weekdays within the month.
    ///
    /// Use [`Self::modify_weekdays()`] to change this.
    pub weekday_modifier: Modifier,

    /// Modifier for any Sundays and Public holidays within the month.
    ///
    /// Use [`Self::modify_holidays()`] to change this.
    pub holiday_modifier: Modifier,

    /// Modifier for today, if present within the month.
    /// This differs from other modifiers in that an [`Option<Modifier>`]
    /// is expected instead; this is due to the fact that if this is [`None`],
    /// then [`Self::weekday_modifier`] or [`Self::holiday_modifier`] will be
    /// used instead, whichever relevant.
    ///
    /// Use [`Self::modify_today()`] to change this.
    pub today_modifier: Option<Modifier>,

    // Decorated Days
    /// A hashmap of days that requires special modifiers.
    ///
    /// Each calendar can register special arbitrary days within itself,
    /// and use a special modifier on each of them. This allows you to
    /// customise the calendar to the highlighting needs of your app.
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

            holidays: vec![],
            weeks_count: 0,

            week_starts_with: Region::starts_week_with()
                .unwrap_or(DEFAULT_WEEK_STARTS_WITH.clone()),
            show_title: true,
            show_other_months: false,

            capitalize_title: true,

            title_modifier: DEFAULT_TITLE_MODIFIER.clone(),
            other_month_modifier: DEFAULT_OTHER_MONTH_MODIFIER.clone(),
            weekday_modifier: DEFAULT_WEEKDAY_MODIFIER.clone(),
            holiday_modifier: DEFAULT_HOLIDAY_MODIFIER.clone(),
            today_modifier: None,

            decorated_days: HashMap::new(),
        }
        .generate_relevant_holidays()
        .generate_weeks_count()
    }

    /// Chained private method to populate holidays of this month.
    fn generate_relevant_holidays(mut self) -> Self {
        self.holidays = Holidays::<Region>::list(self.date.year())
            .into_iter()
            .filter(|date| self.contains(date))
            .collect();

        self
    }

    /// Chained private method to pre-calculate the calendar's number of weeks.
    fn generate_weeks_count(mut self) -> Self {
        self.weeks_count = (0..6)
            .map(
                // Get the weeks we need to print.
                |week_no| {
                    let week =
                        (self.date + Duration::days(7 * week_no)).week(self.week_starts_with);

                    if self.contains(&week.first_day()) || self.contains(&week.last_day()) {
                        Some(week)
                    } else {
                        None
                    }
                },
            )
            .filter(|week| week.is_some())
            .count() as u32;

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

                    if !self.capitalize_title {
                        s.to_ascii_lowercase()
                    } else {
                        s
                    }
                })
                .reduce(|lhs, rhs| lhs + " " + &rhs)
                .unwrap(),
        )
    }

    /// Add a special [`Modifier`] to a single date.
    pub fn decorate_day(mut self, date: NaiveDate, modifier: Modifier) -> Self {
        // Ignore day if its outside of range
        if self.contains(&date) {
            // Swap with a placeholder value to avoid messing with hash table
            let existing_modifier = self
                .decorated_days
                .insert(date, Modifier::Nothing)
                .unwrap_or(Modifier::Nothing);

            // Swap out the placeholder value
            self.decorated_days
                .insert(date, existing_modifier + modifier);
        }

        self
    }

    /// Use [`Self::week_starts_with`] to determine the number of days that had
    /// passed since the local start of week in this calendar.
    ///
    /// Example
    /// ```rust
    /// use chrono::{NaiveDate, Weekday};
    /// use conch::{CalendarMonth, regions};
    ///
    /// let today = NaiveDate::from_ymd_opt(2023, 3, 3).unwrap();
    ///
    /// // Lets start our week with Thursday just for fun.
    /// let calendar: CalendarMonth<regions::England> =
    ///     CalendarMonth::new(today)
    ///     .starts_week_with(Weekday::Thu);
    /// ;
    ///
    /// let my_friday = NaiveDate::from_isoywd_opt(2023, 1, Weekday::Fri).unwrap();
    ///
    /// assert_eq!(
    ///     calendar.num_days_from_start_of_week(&my_friday),
    ///     1, // Friday - Thursday = 1
    /// )
    /// ```
    pub fn num_days_from_start_of_week(&self, date: &NaiveDate) -> u32 {
        (7 + date.weekday().num_days_from_monday() - self.week_starts_with.num_days_from_monday())
            % 7
    }

    /// Get the week number of a certain date.
    ///
    /// This number is 0-indexed; the first week is `0`.
    ///
    /// Example
    /// ```rust
    /// use chrono::{NaiveDate, Weekday};
    /// use conch::{CalendarMonth, regions};
    ///
    /// let today = NaiveDate::from_ymd_opt(2023, 3, 3).unwrap();
    ///
    /// // Week starts with Monday.
    /// let calendar: CalendarMonth<regions::England> =
    ///     CalendarMonth::new(today)
    /// ;
    ///
    /// assert_eq!(
    ///     calendar.week_number_of(
    ///         &NaiveDate::from_ymd_opt(2023, 3, 7).unwrap()
    ///     ),
    ///     Some(1), // Second Week
    /// );
    ///
    /// // Week starts with Wednesday.
    /// let calendar = calendar.starts_week_with(Weekday::Wed);
    ///
    /// assert_eq!(
    ///     calendar.week_number_of(
    ///         &NaiveDate::from_ymd_opt(2023, 3, 7).unwrap()
    ///     ),
    ///     Some(0), // First Week
    /// );
    ///
    /// // Week starts with Thursday.
    /// let calendar = calendar.starts_week_with(Weekday::Thu);
    ///
    /// assert_eq!(
    ///     calendar.week_number_of(
    ///         &NaiveDate::from_ymd_opt(2023, 3, 9).unwrap() // 9th this time
    ///     ),
    ///     Some(2), // Third Week
    /// );
    ///
    /// // Date out of range.
    ///
    /// assert_eq!(
    ///     calendar.week_number_of(
    ///         &NaiveDate::from_ymd_opt(2023, 4, 1).unwrap() // 9th this time
    ///     ),
    ///     None,
    /// );
    /// ```
    ///
    /// For reference:
    /// ```text
    /// March 2023 Week
    ///
    ///  M  T  W  T  F  S  S
    ///        1  2  3  4  5
    ///  6  7  8  9 10 11 12
    /// 13 14 15 16 17 18 19
    /// 20 21 22 23 24 25 26
    /// 27 28 29 30 31
    /// ```
    ///
    pub fn week_number_of(&self, date: &NaiveDate) -> Option<u32> {
        if self.contains(&date) {
            // Assuming our week starts with Sunday, and we are searching for 10th.
            //
            //  S  M  T  W  T  F  S
            // 26 27 28 29 30 31  1
            //  2  3  4  5  6  7  8
            //  9[10]11 12 13 14 15
            // 16 17 18 19 20 21 22
            // 23 24 25 26 27 28 29
            // 30 31  1  2  3  4  5
            //
            // local_weekday_of_1st = 6
            //
            // result = (10+6-1) / 7 = 2
            Some({
                let local_weekday_of_1st = self.num_days_from_start_of_week(
                    &NaiveDate::from_ymd_opt(self.date.year(), self.date.month(), 1).unwrap(),
                );

                (date.day() + local_weekday_of_1st - 1) / 7
            })
        } else {
            None
        }
    }

    /// Return the number of weeks in this month.
    pub fn weeks_count(&self) -> u32 {
        self.weeks_count
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
                #[doc = "Chained method to change the `"]
                #[doc = stringify!($param)]
                #[doc = "` of a [`CalendarMonth`] instance."]
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
    (capitalize_title, capitalize_title, bool),
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

impl<Region> From<&CalendarMonth<Region>> for Lines
where
    Region: RegionMarker,
{
    fn from(value: &CalendarMonth<Region>) -> Self {
        Self::new(value.into())
    }
}

impl<Region> From<CalendarMonth<Region>> for Lines
where
    Region: RegionMarker,
{
    fn from(value: CalendarMonth<Region>) -> Self {
        Self::from(&value)
    }
}

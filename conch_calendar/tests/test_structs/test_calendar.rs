use chrono::NaiveDate;
use conch_ansi::Modifier;
use conch_calendar::{regions, CalendarMonth};
use conch_split::Lines;

#[cfg(test)]
mod test_to_vec_strings {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $date:expr,
            $transformer:expr,
            $expected:expr
            $(,)?
        ) => {
            #[test]
            fn $name() {
                let mut calendar = CalendarMonth::<regions::England>::new($date);

                calendar = $transformer(calendar);

                let strings: Vec<String> = calendar.into();

                println!("{}", Lines::new(strings.clone()));

                assert_eq!(strings, $expected);
            }
        };
    }

    test_factory!(
        no_modifiers_short_month,
        NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
        |calendar| { calendar },
        vec![
            "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m",
            "       1  2  3  4 \u{1b}[38;5;9m\u{1b}[1m 5\u{1b}[22m\u{1b}[39m",
            " 6  7  8  9 10 11 \u{1b}[38;5;9m\u{1b}[1m12\u{1b}[22m\u{1b}[39m",
            "13 14 15 16 17 18 \u{1b}[38;5;9m\u{1b}[1m19\u{1b}[22m\u{1b}[39m",
            "20 21 22 23 24 25 \u{1b}[38;5;9m\u{1b}[1m26\u{1b}[22m\u{1b}[39m",
            "27 28               "
        ]
    );

    test_factory!(
        no_modifiers_long_month,
        NaiveDate::from_ymd_opt(2023, 3, 1).unwrap(),
        |calendar| { calendar },
        vec![
            "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m",
            "       1  2  3  4 \u{1b}[38;5;9m\u{1b}[1m 5\u{1b}[22m\u{1b}[39m",
            " 6  7  8  9 10 11 \u{1b}[38;5;9m\u{1b}[1m12\u{1b}[22m\u{1b}[39m",
            "13 14 15 16 17 18 \u{1b}[38;5;9m\u{1b}[1m19\u{1b}[22m\u{1b}[39m",
            "20 21 22 23 24 25 \u{1b}[38;5;9m\u{1b}[1m26\u{1b}[22m\u{1b}[39m",
            "27 28 29 30 31      "
        ]
    );

    test_factory! (
        no_modifiers_starts_monday_bank_holidays,
        NaiveDate::from_ymd_opt(2023,5,1).unwrap(),
        | calendar |  {
            calendar
        },
        vec![
            "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m",
            "\u{1b}[38;5;9m\u{1b}[1m 1\u{1b}[22m\u{1b}[39m  2  3  4  5  6 \u{1b}[38;5;9m\u{1b}[1m 7\u{1b}[22m\u{1b}[39m",
            " 8  9 10 11 12 13 \u{1b}[38;5;9m\u{1b}[1m14\u{1b}[22m\u{1b}[39m",
            "15 16 17 18 19 20 \u{1b}[38;5;9m\u{1b}[1m21\u{1b}[22m\u{1b}[39m",
            "22 23 24 25 26 27 \u{1b}[38;5;9m\u{1b}[1m28\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;9m\u{1b}[1m29\u{1b}[22m\u{1b}[39m 30 31            "
        ]
    );

    test_factory! (
        no_modifiers_ends_monday_bank_holidays,
        NaiveDate::from_ymd_opt(2023,12,1).unwrap(),
        | calendar |  {
            calendar
        },
        vec![
            "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m",
            "             1  2 \u{1b}[38;5;9m\u{1b}[1m 3\u{1b}[22m\u{1b}[39m",
            " 4  5  6  7  8  9 \u{1b}[38;5;9m\u{1b}[1m10\u{1b}[22m\u{1b}[39m",
            "11 12 13 14 15 16 \u{1b}[38;5;9m\u{1b}[1m17\u{1b}[22m\u{1b}[39m",
            "18 19 20 21 22 23 \u{1b}[38;5;9m\u{1b}[1m24\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;9m\u{1b}[1m25\u{1b}[22m\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m26\u{1b}[22m\u{1b}[39m 27 28 29 30 \u{1b}[38;5;9m\u{1b}[1m31\u{1b}[22m\u{1b}[39m"
        ]
    );

    test_factory! (
        blue_weekdays_modifiers,
        NaiveDate::from_ymd_opt(2023,12,1).unwrap(),
        | calendar: CalendarMonth<regions::England> | -> CalendarMonth<regions::England> {
            calendar
            .modify_weekdays(Modifier::colour("Blue").unwrap())
        },
        vec![
            "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m",
            "            \u{1b}[38;5;4m 1\u{1b}[39m \u{1b}[38;5;4m 2\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m 3\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m 4\u{1b}[39m \u{1b}[38;5;4m 5\u{1b}[39m \u{1b}[38;5;4m 6\u{1b}[39m \u{1b}[38;5;4m 7\u{1b}[39m \u{1b}[38;5;4m 8\u{1b}[39m \u{1b}[38;5;4m 9\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m10\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m11\u{1b}[39m \u{1b}[38;5;4m12\u{1b}[39m \u{1b}[38;5;4m13\u{1b}[39m \u{1b}[38;5;4m14\u{1b}[39m \u{1b}[38;5;4m15\u{1b}[39m \u{1b}[38;5;4m16\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m17\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m18\u{1b}[39m \u{1b}[38;5;4m19\u{1b}[39m \u{1b}[38;5;4m20\u{1b}[39m \u{1b}[38;5;4m21\u{1b}[39m \u{1b}[38;5;4m22\u{1b}[39m \u{1b}[38;5;4m23\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m24\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;9m\u{1b}[1m25\u{1b}[22m\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m26\u{1b}[22m\u{1b}[39m \u{1b}[38;5;4m27\u{1b}[39m \u{1b}[38;5;4m28\u{1b}[39m \u{1b}[38;5;4m29\u{1b}[39m \u{1b}[38;5;4m30\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m31\u{1b}[22m\u{1b}[39m"
        ]
    );

    test_factory! (
        blue_weekdays_modifiers_special_days,
        NaiveDate::from_ymd_opt(2023,12,1).unwrap(),
        | calendar: CalendarMonth<regions::England> | -> CalendarMonth<regions::England> {
            calendar
            .modify_weekdays(Modifier::colour("Blue").unwrap())
            .decorate_day(NaiveDate::from_ymd_opt(2023,12,1).unwrap(), Modifier::colour("BrightGreen").unwrap())
            .decorate_day(NaiveDate::from_ymd_opt(2023,12,30).unwrap(), Modifier::colour("BrightYellow").unwrap())
            .decorate_day(NaiveDate::from_ymd_opt(2023,12,15).unwrap(), Modifier::background("BrightYellow").unwrap())
        },
        vec![
            "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m",
            "            \u{1b}[38;5;10m 1\u{1b}[39m \u{1b}[38;5;4m 2\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m 3\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m 4\u{1b}[39m \u{1b}[38;5;4m 5\u{1b}[39m \u{1b}[38;5;4m 6\u{1b}[39m \u{1b}[38;5;4m 7\u{1b}[39m \u{1b}[38;5;4m 8\u{1b}[39m \u{1b}[38;5;4m 9\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m10\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m11\u{1b}[39m \u{1b}[38;5;4m12\u{1b}[39m \u{1b}[38;5;4m13\u{1b}[39m \u{1b}[38;5;4m14\u{1b}[39m \u{1b}[48;5;11m15\u{1b}[49m \u{1b}[38;5;4m16\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m17\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m18\u{1b}[39m \u{1b}[38;5;4m19\u{1b}[39m \u{1b}[38;5;4m20\u{1b}[39m \u{1b}[38;5;4m21\u{1b}[39m \u{1b}[38;5;4m22\u{1b}[39m \u{1b}[38;5;4m23\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m24\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;9m\u{1b}[1m25\u{1b}[22m\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m26\u{1b}[22m\u{1b}[39m \u{1b}[38;5;4m27\u{1b}[39m \u{1b}[38;5;4m28\u{1b}[39m \u{1b}[38;5;4m29\u{1b}[39m \u{1b}[38;5;11m30\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m31\u{1b}[22m\u{1b}[39m"
        ]
    );

    test_factory! (
        show_other_months,
        NaiveDate::from_ymd_opt(2023,11,1).unwrap(),
        | calendar: CalendarMonth<regions::England> | -> CalendarMonth<regions::England> {
            calendar
            .modify_weekdays(Modifier::colour("Blue").unwrap())
            .show_other_months(true)
        },
        vec![
            "\u{1b}[1m M  T  W  T  F  S  S\u{1b}[22m",
            "\u{1b}[38;5;245m30\u{1b}[39m \u{1b}[38;5;245m31\u{1b}[39m \u{1b}[38;5;4m 1\u{1b}[39m \u{1b}[38;5;4m 2\u{1b}[39m \u{1b}[38;5;4m 3\u{1b}[39m \u{1b}[38;5;4m 4\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m 5\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m 6\u{1b}[39m \u{1b}[38;5;4m 7\u{1b}[39m \u{1b}[38;5;4m 8\u{1b}[39m \u{1b}[38;5;4m 9\u{1b}[39m \u{1b}[38;5;4m10\u{1b}[39m \u{1b}[38;5;4m11\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m12\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m13\u{1b}[39m \u{1b}[38;5;4m14\u{1b}[39m \u{1b}[38;5;4m15\u{1b}[39m \u{1b}[38;5;4m16\u{1b}[39m \u{1b}[38;5;4m17\u{1b}[39m \u{1b}[38;5;4m18\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m19\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m20\u{1b}[39m \u{1b}[38;5;4m21\u{1b}[39m \u{1b}[38;5;4m22\u{1b}[39m \u{1b}[38;5;4m23\u{1b}[39m \u{1b}[38;5;4m24\u{1b}[39m \u{1b}[38;5;4m25\u{1b}[39m \u{1b}[38;5;9m\u{1b}[1m26\u{1b}[22m\u{1b}[39m",
            "\u{1b}[38;5;4m27\u{1b}[39m \u{1b}[38;5;4m28\u{1b}[39m \u{1b}[38;5;4m29\u{1b}[39m \u{1b}[38;5;4m30\u{1b}[39m \u{1b}[38;5;245m 1\u{1b}[39m \u{1b}[38;5;245m 2\u{1b}[39m \u{1b}[38;5;245m 3\u{1b}[39m"
        ]
    );
}

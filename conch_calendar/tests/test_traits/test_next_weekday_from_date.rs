use chrono::{NaiveDate, Weekday};
use conch_calendar::NextWeekdayFromDate;

macro_rules! test_factory {
    (
        $name:ident,
        ($ystart:literal, $mstart:literal, $dstart:literal),
        $weekdays:expr,
        ($yexpected:literal, $mexpected:literal, $dexpected:literal)
        $(,)?
    ) => {
        #[test]
        fn $name() {
            let start_date = NaiveDate::from_ymd_opt($ystart, $mstart, $dstart).unwrap();

            assert_eq!(
                start_date.next_weekday_from(&$weekdays),
                NaiveDate::from_ymd_opt($yexpected, $mexpected, $dexpected)
            );
        }
    };
}

test_factory!(
    mon_from_20240101,
    (2024, 1, 1),
    vec![Weekday::Mon],
    (2024, 1, 1)
);

test_factory!(
    nothing_from_20240101,
    (2024, 1, 1),
    vec![],
    (2024, 0, 0) // Force a None
);

test_factory!(
    tue_from_20240101,
    (2024, 1, 1),
    vec![Weekday::Tue],
    (2024, 1, 2)
);

test_factory!(
    sun_from_20240101,
    (2024, 1, 1),
    vec![Weekday::Sun],
    (2024, 1, 7)
);

test_factory!(
    wed_sun_from_20240101,
    (2024, 1, 1),
    vec![Weekday::Sun, Weekday::Wed],
    (2024, 1, 3)
);

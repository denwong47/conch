use std::ops::{Range, RangeInclusive};

use chrono::{Duration, NaiveDate};
use conch_calendar::IterRangeByDuration;

macro_rules! test_factory {
    (
        $name:ident,
        $range:expr,
        $step:expr,
        (
            $(($yexpected:literal, $mexpected:literal, $dexpected:literal),)*
        )
        $(,)?
    ) => {
        #[test]
        fn $name() {
            assert_eq!(
                $range.into_iter_by_duration($step).collect::<Vec<NaiveDate>>(),
                vec![
                    $(
                        NaiveDate::from_ymd_opt($yexpected, $mexpected, $dexpected).unwrap()
                    ),*
                ]
            );
        }
    };
}

test_factory!(
    simple_range,
    Range::<NaiveDate> {
        start: NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
        end: NaiveDate::from_ymd_opt(2023, 01, 05).unwrap(),
    },
    Duration::days(1),
    (
        (2023, 01, 01),
        (2023, 01, 02),
        (2023, 01, 03),
        (2023, 01, 04),
    )
);

test_factory!(
    simple_range_inclusive,
    RangeInclusive::<NaiveDate>::new(
        NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
        NaiveDate::from_ymd_opt(2023, 01, 05).unwrap(),
    ),
    Duration::days(1),
    (
        (2023, 01, 01),
        (2023, 01, 02),
        (2023, 01, 03),
        (2023, 01, 04),
        (2023, 01, 05),
    )
);

test_factory!(
    negative_range,
    Range::<NaiveDate> {
        start: NaiveDate::from_ymd_opt(2023, 01, 05).unwrap(),
        end: NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
    },
    Duration::days(1),
    ()
);

test_factory!(
    negative_range_inclusive,
    RangeInclusive::<NaiveDate>::new(
        NaiveDate::from_ymd_opt(2023, 01, 05).unwrap(),
        NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
    ),
    Duration::days(1),
    ()
);

test_factory!(
    step_range_by_2_days,
    Range::<NaiveDate> {
        start: NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
        end: NaiveDate::from_ymd_opt(2023, 01, 05).unwrap(),
    },
    Duration::days(2),
    ((2023, 01, 01), (2023, 01, 03),)
);

test_factory!(
    step_range_inclusive_by_2_days,
    RangeInclusive::<NaiveDate>::new(
        NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
        NaiveDate::from_ymd_opt(2023, 01, 05).unwrap(),
    ),
    Duration::days(2),
    ((2023, 01, 01), (2023, 01, 03), (2023, 01, 05),)
);

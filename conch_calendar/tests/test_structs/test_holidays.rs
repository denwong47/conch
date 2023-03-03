use chrono::NaiveDate;

use conch_calendar::regions;
use conch_calendar::{HolidayList, Holidays};

mod test_england {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $year:literal,
            $(
                ($month:literal, $day:literal)
            ),*
        ) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Holidays::<regions::England>::list($year),
                    vec![
                        $(NaiveDate::from_ymd_opt($year, $month, $day).unwrap(),)*
                    ]
                )
            }
        }
    }

    test_factory!(
        year_2021,
        2021,
        (1, 1),
        (4, 2),
        (4, 5),
        (5, 3),
        (5, 31),
        (8, 30),
        (12, 27),
        (12, 28)
    );

    // 2022 is Platinum Jubilee so its all messed up

    test_factory!(
        year_2023,
        2023,
        (1, 2),
        (4, 7),
        (4, 10),
        (5, 1),
        (5, 29),
        (8, 28),
        (12, 25),
        (12, 26),
        (5, 8)
    );

    test_factory!(
        year_2024,
        2024,
        (1, 1),
        (3, 29),
        (4, 1),
        (5, 6),
        (5, 27),
        (8, 26),
        (12, 25),
        (12, 26)
    );
}

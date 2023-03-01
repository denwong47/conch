mod test_easter {
    use chrono::NaiveDate;

    use conch_calendar::func::get_easter_date;

    macro_rules! test_factory {
        (
            $name:ident,
            ($year:literal,
            $month:literal,
            $day:literal)
            $(,)?
        ) => {
            #[test]
            fn $name() {
                assert_eq!(
                    get_easter_date($year),
                    NaiveDate::from_ymd_opt($year, $month, $day).unwrap()
                );
            }
        };
    }

    test_factory!(year_2023, (2023, 4, 9));
    test_factory!(year_2024, (2024, 3, 31));
    test_factory!(year_2025, (2025, 4, 20));
}

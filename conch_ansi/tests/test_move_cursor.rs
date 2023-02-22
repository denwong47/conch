//! Test [`MoveCursor`] enum.
//!
use conch_ansi::*;
use conch_base_models::*;

mod test_try_from {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $text:literal,
            $expected:expr
        ) => {
            #[test]
            fn $name() {
                let parsed: Result<MoveCursor, ModifierError> = MoveCursor::try_from($text);

                if let Ok(variant) = parsed {
                    assert_eq!(variant, $expected.unwrap());
                } else {
                    if $expected.is_ok() {
                        panic!("{}", parsed.unwrap_err())
                    } else {
                        assert_eq!(
                            $expected.unwrap_err().to_string(),
                            parsed.unwrap_err().to_string(),
                        )
                    }
                }
            }
        };
    }

    test_factory!(
        simple_up,
        "\x1b[1A",
        Ok::<_, ModifierError>(MoveCursor::Up(1))
    );
    test_factory!(
        simple_down,
        "\x1b[2B",
        Ok::<_, ModifierError>(MoveCursor::Down(2))
    );
    test_factory!(
        simple_right,
        "\x1b[3C",
        Ok::<_, ModifierError>(MoveCursor::Right(3))
    );
    test_factory!(
        simple_left,
        "\x1b[4D",
        Ok::<_, ModifierError>(MoveCursor::Left(4))
    );
    test_factory!(
        simple_origin,
        "\x1b[0;0H",
        Ok::<_, ModifierError>(MoveCursor::Absolute(0, 0))
    );
    test_factory!(
        simple_absolute,
        "\x1b[10;20H",
        Ok::<_, ModifierError>(MoveCursor::Absolute(10, 20))
    );

    test_factory!(
        negative_up,
        "\x1b[-1A",
        Ok::<_, ModifierError>(MoveCursor::Up(-1))
    );

    test_factory!(
        zero_up,
        "\x1b[0A",
        Ok::<_, ModifierError>(MoveCursor::Up(0))
    );

    test_factory!(
        negative_absolute,
        "\x1b[-2;-60H",
        Ok::<_, ModifierError>(MoveCursor::Absolute(-2, -60))
    );
}

mod test_partial_eq {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $lhs:expr,
            $rhs:expr,
            $result:literal,
        ) => {
            #[test]
            fn $name() {
                if $result {
                    assert_eq!($lhs, $rhs)
                } else {
                    assert_ne!($lhs, $rhs)
                }
            }
        };
    }

    test_factory!(up_and_up, MoveCursor::Up(30), MoveCursor::Up(30), true,);

    test_factory!(
        up_and_down_negative,
        MoveCursor::Up(30),
        MoveCursor::Down(-30),
        true,
    );

    test_factory!(up_and_down, MoveCursor::Up(30), MoveCursor::Down(30), false,);

    test_factory!(
        down_and_up_negative,
        MoveCursor::Down(30),
        MoveCursor::Up(-30),
        true,
    );

    test_factory!(
        up_and_right,
        MoveCursor::Up(30),
        MoveCursor::Right(30),
        false,
    );

    test_factory!(
        right_and_up,
        MoveCursor::Right(30),
        MoveCursor::Up(30),
        false,
    );

    test_factory!(
        right_and_left,
        MoveCursor::Right(30),
        MoveCursor::Left(30),
        false,
    );

    test_factory!(
        right_and_left_negative,
        MoveCursor::Right(30),
        MoveCursor::Left(-30),
        true,
    );

    test_factory!(
        left_and_right_negative,
        MoveCursor::Left(30),
        MoveCursor::Right(-30),
        true,
    );

    test_factory!(
        absolute_and_absolute,
        MoveCursor::Absolute(123, 456),
        MoveCursor::Absolute(123, 456),
        true,
    );

    test_factory!(
        absolute_and_absolute_different,
        MoveCursor::Absolute(123, 456),
        MoveCursor::Absolute(0, 456),
        false,
    );

    test_factory!(
        absolute_and_origin,
        MoveCursor::Absolute(0, 0),
        MoveCursor::Origin,
        true,
    );

    test_factory!(
        origin_and_absolute,
        MoveCursor::Origin,
        MoveCursor::Absolute(0, 0),
        true,
    );

    test_factory!(
        absolute_and_up,
        MoveCursor::Absolute(30, 0),
        MoveCursor::Up(30),
        false,
    );
}

mod test_resetter {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $apply:expr,
            $reset:expr
        ) => {
            #[test]
            fn $name() {
                assert_eq!($apply.resetter(Some("Hello, World!")), $reset)
            }
        };
    }

    test_factory!(simple_up, MoveCursor::Up(5), MoveCursor::Down(5));

    test_factory!(simple_down, MoveCursor::Down(5), MoveCursor::Up(5));

    // test_factory!(
    //     simple_right,
    //     MoveCursor::Right(5),
    //     MoveCursor::Left(5+13)
    // );
}

mod manual_tests {
    use super::*;

    // For manual testing. Run using `cargo test -- --ignored --nocapture
    #[test]
    #[ignore]
    fn run() {
        // print 20 dots
        print!(
            "{}",
            (0..20).into_iter().fold(String::new(), |lhs, _| lhs + ".")
        );
        print!(
            "{}",
            (0..20).into_iter().fold(String::new(), |lhs, _| lhs + ":")
        );
        print!(
            "{}",
            (0..20).into_iter().fold(String::new(), |lhs, _| lhs + ".")
        );
        print!(
            "{}",
            (0..20).into_iter().fold(String::new(), |lhs, _| lhs + ":")
        );
        println!();

        println!(
            "Original{}{}{}Right",
            Modifier::right(12).wraps("Shifted16"),
            (Modifier::right(32) + Modifier::background("BrightRed").unwrap()).wraps("Shifted36"),
            (Modifier::left(6) + Modifier::colour("BrightRed").unwrap()).wraps("LongWord"),
        );
    }
}

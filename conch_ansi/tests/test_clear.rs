//! Test [`Clear`] enum.
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
            $(,)?
        ) => {
            #[test]
            fn $name() {
                let parsed: Result<Clear, ModifierError> = Clear::try_from($text);
                let modifier: Result<Modifier, ModifierError> = Modifier::try_from($text);

                if let Ok(variant) = parsed {
                    assert_eq!(variant, $expected.unwrap());
                    assert_eq!(modifier.unwrap(), Modifier::Clear($expected.unwrap()));
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
        entire_display,
        "\x1b[2J",
        Ok::<_, ModifierError>(Clear::EntireDisplay)
    );

    // Currently not supported
    // test_factory!(
    //     display_below_cursor_no_param,
    //     "\x1b[J",
    //     Ok::<_, ModifierError>(Clear::DisplayBelowCursor)
    // );

    test_factory!(
        display_below_cursor,
        "\x1b[0J",
        Ok::<_, ModifierError>(Clear::DisplayBelowCursor)
    );

    test_factory!(
        display_above_cursor,
        "\x1b[1J",
        Ok::<_, ModifierError>(Clear::DisplayAboveCursor)
    );

    test_factory!(
        entire_line,
        "\x1b[2K",
        Ok::<_, ModifierError>(Clear::EntireLine)
    );

    // Currently not supported
    // test_factory!(
    //     line_after_cursor_no_param,
    //     "\x1b[K",
    //     Ok::<_, ModifierError>(Clear::LineAfterCursor)
    // );

    test_factory!(
        line_after_cursor,
        "\x1b[0K",
        Ok::<_, ModifierError>(Clear::LineAfterCursor)
    );

    test_factory!(
        line_before_cursor,
        "\x1b[1K",
        Ok::<_, ModifierError>(Clear::LineBeforeCursor)
    );
}

mod test_to_string {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $lhs:expr,
            $rhs:literal
            $(,)?
        ) => {
            #[test]
            fn $name() {
                assert_eq!($lhs.to_string().as_str(), $rhs)
            }
        };
    }

    test_factory!(entire_display, Clear::EntireDisplay, "\x1b[2J",);
    test_factory!(display_below_cursor, Clear::DisplayBelowCursor, "\x1b[0J",);
    test_factory!(display_above_cursor, Clear::DisplayAboveCursor, "\x1b[1J",);
    test_factory!(entire_line, Clear::EntireLine, "\x1b[2K",);
    test_factory!(line_after_cursor, Clear::LineAfterCursor, "\x1b[0K",);
    test_factory!(line_before_cursor, Clear::LineBeforeCursor, "\x1b[1K",);
}

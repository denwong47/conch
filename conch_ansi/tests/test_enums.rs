//! Test base modifier enums.
//!
//! This does not include tests for `unified::Modifier`.
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
                let parsed: Result<Colour, ModifierError> = Colour::try_from($text);

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
        simple_word,
        "\x1b[38:5:125m",
        Ok::<_, ModifierError>(Colour::R3G0B1)
    );
    test_factory!(
        simple_reset,
        "\x1b[39m",
        Ok::<_, ModifierError>(Colour::Reset)
    );
    test_factory!(
        mismatched_apply_command,
        "\x1b[40:5:125m",
        Err::<Colour, _>(ModifierError::MismatchedANSICode(
            String::from("Colour"),
            40,
            38,
        ))
    );
    test_factory!(
        mismatched_reset_command,
        "\x1b[40m",
        Err::<Colour, _>(ModifierError::MismatchedANSICode(
            String::from("Colour"),
            40,
            39,
        ))
    );
    test_factory!(
        simple_apply_with_nondigit,
        "\x1b[38:a:125m",
        Err::<Colour, _>(ModifierError::ValueIsNotAModifier(
            String::from("\x1b[38:a:125m"),
            String::from("Unmatchable pattern."),
        ))
    );
    test_factory!(
        simple_apply_with_wrong_end_digit,
        "\x1b[38:5:125n",
        Err::<Colour, _>(ModifierError::UnexpectedEndCharacter(
            String::from("Colour"),
            String::from("n"),
        ))
    );
    test_factory!(
        reset_with_extra_codes,
        "\x1b[39:5:125m",
        Err::<Colour, _>(ModifierError::ValueNotRecognised(
            String::from("Colour"),
            format!("{:?}:{:?}", Some(39), vec![5, 125]),
            String::from("Wrong combination of codes.")
        ))
    );
    test_factory!(
        apply_with_wrong_code,
        "\x1b[38:6:125m",
        Err::<Colour, _>(ModifierError::ValueNotRecognised(
            String::from("Colour"),
            format!("{:?}", vec![6, 125]),
            String::from("Non 5 codes are not allowed.")
        ))
    );
    test_factory!(
        simple_word_with_trailing_text,
        "\x1b[38:5:125mThis is some extra text added",
        Ok::<_, ModifierError>(Colour::R3G0B1)
    );
    test_factory!(
        simple_word_with_leading_text,
        "This is some extra text added\x1b[38:5:125m",
        Err::<Colour, _>(ModifierError::ValueIsNotAModifier(
            String::from("This is some extra text added\u{1b}[38:5:125m"),
            String::from("Unmatchable pattern."),
        ))
    );
}

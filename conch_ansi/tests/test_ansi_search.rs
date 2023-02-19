//! Test to search for [`ANSIEscapeCode`] in [`str`].

// use conch_base_models::*;
use conch_ansi::*;

mod test_iter {
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
                let modifiers: Vec<Modifier> = $text.iter_modifiers().collect();

                assert_eq!(modifiers, $expected);
            }
        };
    }

    test_factory!(
        simple_word,
        "\x1b[38:5:69m",
        vec![Modifier::Colour(Colour::R1G2B5),]
    );

    test_factory!(incomplete_pattern, "\x1b[38:5:69", vec![]);

    test_factory!(
        consecutive_patterns,
        "\x1b[38:5:69m\x1b[48:5:42m",
        vec![
            Modifier::Colour(Colour::R1G2B5),
            Modifier::Background(Background::R0G4B2),
        ]
    );

    test_factory!(
        multiple_patterns_with_extras,
        "Hello, \x1b[38:5:69mWorld\x1b[48:5:42m!\x1b[49m\x1b[39m",
        vec![
            Modifier::Colour(Colour::R1G2B5),
            Modifier::Background(Background::R0G4B2),
            Modifier::Background(Background::Reset),
            Modifier::Colour(Colour::Reset),
        ]
    );

    test_factory!(
        multiple_patterns_with_one_being_bad,
        // Missing : between 5 and 42
        "Hello, \x1b[38:5:69mWorld\x1b[48:542m!\x1b[49m\x1b[39m",
        vec![
            Modifier::Colour(Colour::R1G2B5),
            Modifier::Background(Background::Reset),
            Modifier::Colour(Colour::Reset),
        ]
    );
}

mod test_len {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $text:literal,
            $expected:literal
            $(,)?
        ) => {
            #[test]
            fn $name() {
                let length: usize = $text.len_without_modifiers();

                assert_eq!(length, $expected);
            }
        };
    }

    test_factory!(simple_word, "\x1b[38:5:69m", 0);

    test_factory!(incomplete_pattern, "\x1b[38:5:69", 9);

    test_factory!(consecutive_patterns, "\x1b[38:5:69m\x1b[48:5:42m", 0);

    test_factory!(
        multiple_patterns_with_extras,
        "Hello, \x1b[38:5:69mWorld\x1b[48:5:42m!\x1b[49m\x1b[39m",
        13
    );

    test_factory!(
        multiple_patterns_with_one_being_bad,
        // Missing : between 5 and 42
        "Hello, \x1b[38:5:69mWorld\x1b[48:542m!\x1b[49m\x1b[39m",
        22
    );
}

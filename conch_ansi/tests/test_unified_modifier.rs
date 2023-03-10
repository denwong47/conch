//! Test [`Modifier`].

use conch_ansi::*;
use conch_base_models::*;

mod test_from_name {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $method:ident,
            $text:literal,
            $expected:expr
        ) => {
            #[test]
            fn $name() {
                let result: Option<Modifier> = Modifier::$method($text);

                assert_eq!(result, $expected)
            }
        };
    }

    test_factory!(
        simple_colour,
        colour,
        "Black",
        Some(Modifier::Colour(Colour::Black))
    );

    test_factory!(
        simple_background,
        background,
        "BrightBlue",
        Some(Modifier::Background(Background::BrightBlue))
    );

    test_factory!(
        simple_intensity,
        intensity,
        "Bold",
        Some(Modifier::Intensity(Intensity::Bold))
    );

    test_factory!(non_existent_colour, colour, "BadChoice", None);

    test_factory!(non_existent_background, background, "BadChoice", None);

    test_factory!(non_existent_intensity, intensity, "BadChoice", None);
}

mod test_parsing {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $text:literal,
            $expected:expr
        ) => {
            #[test]
            fn $name() {
                let parsed: Result<Modifier, ModifierError> = Modifier::try_from($text);

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
        simple_colour,
        "\x1b[38:5:125m",
        Ok::<_, ModifierError>(Modifier::Colour(Colour::R3G0B1))
    );

    test_factory!(
        simple_background,
        "\x1b[48:5:125m",
        Ok::<_, ModifierError>(Modifier::Background(Background::R3G0B1))
    );

    test_factory!(
        simple_intensity_1,
        "\x1b[1m",
        Ok::<_, ModifierError>(Modifier::Intensity(Intensity::Bold))
    );

    test_factory!(
        simple_intensity_2,
        "\x1b[2m",
        Ok::<_, ModifierError>(Modifier::Intensity(Intensity::Faint))
    );

    test_factory!(
        simple_intensity_22,
        "\x1b[22m",
        Ok::<_, ModifierError>(Modifier::Intensity(Intensity::Normal))
    );
}

mod test_add {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            (
                $base:expr,
                $($modifier:expr),+
                $(,)?
            ),
            $expected:expr
        ) => {
            mod $name {
                use super::*;

                #[test]
                fn add() {
                    let mut result = $base;
                    $(result = result + $modifier;)+

                    println!("  > Testing + for {}: {}", stringify!($name), result.wraps("Hello, World!"));
                    assert_eq!(result, Modifier::Combo($expected));
                }

                #[test]
                fn add_assign() {
                    let mut result = $base;
                    $(result += $modifier;)+

                    println!("  > Testing += for {}: {}", stringify!($name), result.wraps("Hello, World!"));
                    assert_eq!(result, Modifier::Combo($expected));
                }
            }
        }
    }

    test_factory!(
        colour_background,
        (
            Modifier::Colour(Colour::BrightYellow),
            Modifier::Background(Background::BrightRed)
        ),
        vec![
            Modifier::Colour(Colour::BrightYellow),
            Modifier::Background(Background::BrightRed),
        ]
    );

    test_factory!(
        colour_background_intensity,
        (
            Modifier::Colour(Colour::BrightYellow),
            Modifier::Background(Background::BrightRed),
            Modifier::Intensity(Intensity::Bold)
        ),
        vec![
            Modifier::Colour(Colour::BrightYellow),
            Modifier::Background(Background::BrightRed),
            Modifier::Intensity(Intensity::Bold)
        ]
    );

    test_factory!(
        colour_background_colour,
        (
            Modifier::Colour(Colour::BrightYellow),
            Modifier::Colour(Colour::BrightGreen),
            Modifier::Background(Background::BrightRed)
        ),
        vec![
            Modifier::Colour(Colour::BrightYellow),
            Modifier::Colour(Colour::BrightGreen),
            Modifier::Background(Background::BrightRed)
        ]
    );
}

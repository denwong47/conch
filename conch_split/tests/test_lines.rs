use conch_ansi::Modifier;
use conch_macros::s;
use conch_split::Lines;

#[cfg(test)]
mod test_display {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $lines:expr,
            $expected:expr
            $(,)?
        ) => {
            #[test]
            fn $name() {
                let lines = $lines;
                println!("Test '{}':\n{}", stringify!($name), lines);

                assert_eq!(lines.to_string().as_str(), $expected);
            }
        };
    }

    test_factory!(
        simple_3_lines,
        Lines::from(vec!["veni", "vidi", "vici",]),
        s!(
            "veni\n"
            "vidi\n"
            "vici"
        )
    );

    test_factory!(
        prefix,
        Lines::from(vec!["veni", "vidi", "vici",]).prefix("> "),
        s!(
            "> veni\n"
            "> vidi\n"
            "> vici"
        )
    );

    test_factory!(
        spacing,
        Lines::from(vec!["veni", "vidi", "vici",]).spacing(2),
        s!(
            "veni\n\n"
            "vidi\n\n"
            "vici"
        )
    );

    test_factory!(
        prefix_and_spacing,
        Lines::from(vec!["veni", "vidi", "vici",])
            .prefix("> ")
            .spacing(2),
        s!(
            "> veni\n\n"
            "> vidi\n\n"
            "> vici"
        )
    );

    test_factory!(
        title,
        Lines::from(vec!["veni", "vidi", "vici",]).title("Julius Caesar"),
        s!(
            "Julius Caesar\n"
            "veni\n"
            "vidi\n"
            "vici"
        )
    );

    test_factory!(
        title_prefix,
        Lines::from(vec!["veni", "vidi", "vici",])
            .title("Julius Caesar")
            .title_prefix("Quotes of "),
        s!(
            "Quotes of Julius Caesar\n"
            "veni\n"
            "vidi\n"
            "vici"
        )
    );

    test_factory!(
        lines_modifier,
        Lines::from(vec!["veni", "vidi", "vici",])
            .title("Julius Caesar")
            .title_prefix("Quotes of ")
            .modifier(Modifier::colour("Grayscale13").unwrap()),
        s!(
            "\u{1b}[38;5;245mQuotes of Julius Caesar\u{1b}[39m\n"
            "\u{1b}[38;5;245mveni\u{1b}[39m\n"
            "\u{1b}[38;5;245mvidi\u{1b}[39m\n"
            "\u{1b}[38;5;245mvici\u{1b}[39m"
        )
    );

    test_factory!(
        lines_and_title_modifier,
        Lines::from(vec!["veni", "vidi", "vici",])
            .title("Julius Caesar")
            .title_prefix("Quotes of ")
            .modifier(Modifier::colour("Grayscale13").unwrap())
            .title_modifier(
                Modifier::colour("BrightRed").unwrap() + Modifier::intensity("Bold").unwrap()
            ),
        s!(
            "\u{1b}[38;5;9m\u{1b}[1mQuotes of Julius Caesar\u{1b}[22m\u{1b}[39m\n"
            "\u{1b}[38;5;245mveni\u{1b}[39m\n"
            "\u{1b}[38;5;245mvidi\u{1b}[39m\n"
            "\u{1b}[38;5;245mvici\u{1b}[39m"
        )
    );
}

use conch_macros::s;

#[cfg(test)]
mod test_s_macro {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            (
                $($text:expr )*
            ),
            $expected:literal
            $(,)?
        ) => {
            #[test]
            fn $name() {
                assert_eq!(
                    s!($($text )*),
                    $expected
                );
            }
        }
    }

    test_factory! (
        hello_world,
        (
            "Hello"
            ", "
            "World"
            "!"
        ),
        "Hello, World!"
    );

    test_factory! (
        hello_world_with_strings,
        (
            String::from("Hello")
            String::from(", ")
            "World".to_string()
            "!".to_string()
        ),
        "Hello, World!"
    );

    test_factory! (
        hello_world_with_mixture,
        (
            String::from("Hello")
            ", "
            "World".to_owned()
            "!"
        ),
        "Hello, World!"
    );

    test_factory! (
        hello_world_multiline,
        (
            String::from("Hello")
            ",\n"
            "World".to_owned()
            "!"
        ),
        "Hello,
World!"
    );

    test_factory! (
        single_uint,
        (
            12345_u32
        ),
        "12345"
    );

    test_factory! (
        mix_uint_str,
        (
            "30624700 "
            30624770_u32
            " 534202 "
            13_u8
            942_i16
            " "
            4314_u16
            String::from("0624")
        ),
        "30624700 30624770 534202 13942 43140624"
    );
}
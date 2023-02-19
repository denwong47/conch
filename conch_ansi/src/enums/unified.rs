//! The primary enum, containing all the modifier enum structs as well as providing
//! a method to combine modifiers in sequence.

use std::{fmt, ops};

use enum_index::VariantByName;

use crate::{Background, Colour, Intensity};
use conch_base_models::{ANSIEscapeCode, HasLength, ModifierError, Resetter, StringWrapper};

/// Unified [`Modifier`] enum type.
///
/// For each of the enum types of [`Background`], [`Colour`] and [`Intensity`], this
/// enum has a corresponding variant, bringing them under the same struct. Each of
/// these variant takes a single-element tuple value of the corresponding enum type.
///
/// Most important methods are implemented and passed through to the underlying
/// enum variant, such as [`Self::wraps()`] and [`Self::len()`], so in most cases they
/// can be used interchangably in syntax terms:
///
/// ```rust
/// use conch::*;
///
/// assert_eq!(
///     Modifier::Colour(Colour::BrightRed).wraps("Hello, World!"),
///     Colour::BrightRed.wraps("Hello, World!"),
/// );
///
/// assert_eq!(
///     Modifier::Intensity(Intensity::Bold).wraps("Hello, World!"),
///     Intensity::Bold.wraps("Hello, World!"),
/// );
/// ```
///
/// To make instantiation easier, [`Modifier`] also allows convenient methods to
/// get a certain variant by [`str`]:
///
/// ```rust
/// use conch::*;
///
/// assert_eq!(
///     Modifier::colour("BrightRed"),
///     Some(Modifier::Colour(Colour::BrightRed))
/// );
///
/// assert_eq!(
///     Modifier::intensity("Bold"),
///     Some(Modifier::Intensity(Intensity::Bold))
/// )
/// ```
///
/// [`Modifier`] also has the special variant of [`Modifier::Combo`], allowing multiple
/// [`Modifier`] to be applied in sequence when [wrapping].
///
/// [`Modifier::Combo`] can be built from using `+` and `+=` operators:
///
/// [wrapping]: Modifier::wraps()
///
/// ```rust
/// use conch::*;
///
/// assert_eq!(
///     Modifier::colour("BrightRed").unwrap() + Modifier::intensity("Bold").unwrap(),
///     Modifier::Combo(
///         vec![
///             Modifier::Colour(Colour::BrightRed),
///             Modifier::Intensity(Intensity::Bold),
///         ]
///     )
/// )
/// ```
///
/// [`Modifier`] can also be used for [`std::fmt::Display`] directly:
///
/// ```rust
/// use conch::*;
///
/// assert_eq!(
///     (Modifier::colour("BrightRed").unwrap() + Modifier::intensity("Bold").unwrap())
///     .wraps("Hello, world!"),
///     "\u{1b}[38;5;9m\u{1b}[1mHello, world!\u{1b}[22m\u{1b}[39m"
/// )
/// ```
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    Intensity(Intensity),
    Colour(Colour),
    Background(Background),

    Combo(Vec<Self>),
}

macro_rules! expand_variants {
    ($(($variant:ident, $base_enum:ident, $method:ident)),+$(,)?) => {
        impl Modifier {
            $(
                #[allow(dead_code)]
                pub fn $method(name: &str) -> Option<Self> {
                    $base_enum::by_name(name)
                    .map(
                        | modifier | {
                            Self::$variant(modifier)
                        }
                    )
                }
            )*
        }
    };
}

expand_variants!(
    (Intensity, Intensity, intensity),
    (Colour, Colour, colour),
    (Background, Background, background),
);

impl HasLength for Modifier {
    /// String Length of the [`Modifier`] upon conversion.
    fn len(&self) -> usize {
        macro_rules! expand_variants {
            ($($variant:ident),+) => {
                match self {
                    $(Self::$variant(modifier) => modifier.len(),)+
                    Self::Combo(modifiers) => {
                        // For [`Modifier::Combo`], sequentially format all the modifiers.
                        modifiers
                        .iter()
                        .fold(
                            0,
                            | lhs, modifier | {
                                lhs + modifier.len()
                            }
                        )
                    },
                }
            };
        }

        expand_variants!(Intensity, Colour, Background)
    }
}

/// Allow all Modifiers to have a resetter.
/// For all single types, just return its own resetter.
/// For [`Modifier::Combo`], returns another [`Modifier::Combo`] with the resetters in reversed order.
impl Resetter for Modifier {
    fn resetter(&self, input: Option<&str>) -> Self {
        macro_rules! expand_variants {
            ($($variant:ident),+) => {
                match self {
                    $(Self::$variant(modifier) => Self::$variant(modifier.resetter(input)),)+
                    Self::Combo(modifiers) => {
                        // For [`Modifier::Combo`], sequentially format all the modifiers.
                        Self::Combo(
                            modifiers
                            .iter()
                            .rev()
                            .map(
                                | modifier | {
                                    modifier.resetter(input)
                                }
                            )
                            .collect()
                        )
                    },
                }
            };
        }

        expand_variants!(Intensity, Colour, Background)
    }
}

/// Allow two [`Modifier`] to be added together; so that when wrapping, the modifiers
/// will be applied in reversed sequence.
impl ops::Add for Modifier {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs_mods = match self {
            Self::Combo(mods) => mods,
            other => vec![other],
        };

        let mut rhs_mods = match rhs {
            Self::Combo(mods) => mods,
            other => vec![other],
        };

        lhs_mods.append(&mut rhs_mods);

        Self::Combo(lhs_mods)
    }
}

/// Allow a mutable [`Modifier`] to add another [`Modifier`] to itself.
impl ops::AddAssign for Modifier {
    fn add_assign(&mut self, other: Self) {
        if let Self::Combo(_) = self {
        } else {
            *self = Self::Combo(vec![self.clone()]);
        };

        match self {
            Self::Combo(mods) => mods.push(other),
            _ => unreachable!("`self` should always be `Self::Combo`."),
        }
    }
}

/// Allow the use Modifier enum variants directly in `println!()` or `format!()`.
impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! expand_variants {
            ($($variant:ident),+) => {
                match self {
                    $(Self::$variant(modifier) => modifier.fmt(f),)+
                    Self::Combo(modifiers) => {
                        // For [`Modifier::Combo`], sequentially format all the modifiers.
                        Result::from_iter(
                            modifiers.iter().map(
                                | modifier | modifier.fmt(f)
                            )
                        )
                    },
                }
            };
        }

        expand_variants!(Intensity, Colour, Background)
    }
}

/// Allows a `Modifier` to wrap a `str`, decorating it and resetting itself afterwards.
impl StringWrapper for Modifier {
    /// Enclose the text with the modifier.
    fn wraps(&self, text: &str) -> String {
        match self {
            Self::Combo(mods) => mods
                .iter()
                .rev()
                .fold(String::from(text), |text, modifier| modifier.wraps(&text)),
            Self::Intensity(modifier) => modifier.wraps(text),
            Self::Colour(modifier) => modifier.wraps(text),
            Self::Background(modifier) => modifier.wraps(text),
        }
    }
}

/// Try to parse an [`ANSIEscapeCode`] into a known [`Modifier`].
impl TryFrom<&ANSIEscapeCode> for Modifier {
    type Error = ModifierError;
    fn try_from(value: &ANSIEscapeCode) -> Result<Self, Self::Error> {
        macro_rules! expand_base_enums {
            ($(($variant:ident, $base_enum:ident, $code:pat, $end_char:literal)),+) => {
                match (value.code, value.end_char) {
                    $(
                        ($code, $end_char) => {
                            let modifier = $base_enum::try_from(value)?;

                            Ok(Self::$variant(modifier))
                        },
                    )+
                    (Some(code), _) => Err(
                        ModifierError::UnsupportedANSICode(code),
                    ),
                    (_, chr) => Err(
                        ModifierError::UnsupportedEndChar(chr),
                    ),
                }
            };
        }

        expand_base_enums!(
            (Intensity, Intensity, Some(1), 'm'),
            (Intensity, Intensity, Some(2), 'm'),
            (Intensity, Intensity, Some(22), 'm'),
            (Colour, Colour, Some(38), 'm'),
            (Colour, Colour, Some(39), 'm'),
            (Background, Background, Some(48), 'm'),
            (Background, Background, Some(49), 'm')
        )
    }
}

/// Also supports owning the [`ANSIEscapeCode`] outright and drop it afterwards.
impl TryFrom<ANSIEscapeCode> for Modifier {
    type Error = ModifierError;

    fn try_from(value: ANSIEscapeCode) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// Try to parse a [`str`] into a known [`Modifier`].
///
/// This is simply chaining together
///
/// - [`TryFrom<&str>`] of [`ANSIEscapeCode`], and
/// - [`TryFrom<ANSIEscapeCode>`] of [`Modifier`]
impl TryFrom<&str> for Modifier {
    type Error = ModifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ANSIEscapeCode::try_from(value).and_then(|ansi| Modifier::try_from(ansi))
    }
}

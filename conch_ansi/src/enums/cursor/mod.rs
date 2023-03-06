use std::fmt;
use strum_macros::EnumIter;

use conch_base_models::{ANSIEscapeCode, IntoANSIEscapeCode, ModifierError, Resetter};
use conch_macros::ansi_enum_builder as builder;

use crate::traits::*;
use crate::Modifier;

/// Move the position of the current cursor in the terminal.
#[allow(dead_code)]
#[derive(Clone, Debug, EnumIter)]
pub enum MoveCursor {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
    Origin,
    Absolute(i32, i32),
}

impl PartialEq for MoveCursor {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Direct comparison
            (Self::Up(m), Self::Up(n)) if m == n => true,
            (Self::Down(m), Self::Down(n)) if m == n => true,
            (Self::Right(m), Self::Right(n)) if m == n => true,
            (Self::Left(m), Self::Left(n)) if m == n => true,

            // Opposite directions and negative amount
            (Self::Up(m), Self::Down(n)) if &-m == n => true,
            (Self::Down(m), Self::Up(n)) if &-m == n => true,
            (Self::Right(m), Self::Left(n)) if &-m == n => true,
            (Self::Left(m), Self::Right(n)) if &-m == n => true,

            // Direct comparison
            (Self::Origin, Self::Origin) => true,
            (Self::Absolute(x1, y1), Self::Absolute(x2, y2)) if x1 == x2 && y1 == y2 => true,

            // Origin and (0, 0)
            (Self::Origin, Self::Absolute(0, 0)) => true,
            (Self::Absolute(0, 0), Self::Origin) => true,

            _ => false,
        }
    }
}

impl IntoANSIEscapeCode for MoveCursor {
    /// Convert any [`MoveCursor`] into a [`ANSIEscapeCode`].
    ///
    /// This also converts negative values to the opposite modifier:
    ///
    /// ```rust
    /// use conch_ansi::MoveCursor;
    /// use conch_base_models::ANSIEscapeCode;
    ///
    /// assert_eq!(
    ///     ANSIEscapeCode::from(&MoveCursor::Right(-1)),
    ///     ANSIEscapeCode::new(
    ///         None,
    ///         Some(vec![1]),
    ///         'D', // Left
    ///     )
    /// )
    /// ```
    fn into_ansi_escape_code(&self) -> ANSIEscapeCode {
        match self {
            Self::Up(n) if *n < 0 => ANSIEscapeCode::new(None, Some(vec![n.abs()]), 'B'),
            Self::Up(n) => ANSIEscapeCode::new(None, Some(vec![*n]), 'A'),
            Self::Down(n) if *n < 0 => ANSIEscapeCode::new(None, Some(vec![n.abs()]), 'A'),
            Self::Down(n) => ANSIEscapeCode::new(None, Some(vec![*n]), 'B'),
            Self::Right(n) if *n < 0 => ANSIEscapeCode::new(None, Some(vec![n.abs()]), 'D'),
            Self::Right(n) => ANSIEscapeCode::new(None, Some(vec![*n]), 'C'),
            Self::Left(n) if *n < 0 => ANSIEscapeCode::new(None, Some(vec![n.abs()]), 'C'),
            Self::Left(n) => ANSIEscapeCode::new(None, Some(vec![*n]), 'D'),
            Self::Origin => ANSIEscapeCode::new(None, None, 'H'),
            Self::Absolute(x, y) => ANSIEscapeCode::new(None, Some(vec![*y, *x]), 'H'),
        }
    }
}

impl Resetter for MoveCursor {
    /// Attempt to reset any settings to before this modifier was applied.
    ///
    /// Compared to the other enums, moving cursors are sometimes impossible to reset
    /// unless the original cursor location is known. In particular,
    ///
    /// - [`Self::Up`], [`Self::Down`] simply returns the opposite modifier, i.e.
    ///   [`Self::Up(5)`] will return [`Self::Down(5)`];
    /// - [`Self::Right`] and [`Self::Left`] will produce an opposite modifier with
    ///   added [`Self::Left`] equal to the number of string characters in `input`.
    /// - [`Self::Origin`] and [`Self::Absolute`], being absolute positions, will return
    ///   themselves as the resetter.
    ///
    #[allow(unused_variables)]
    fn resetter(&self, input: Option<&str>) -> Self {
        // This is going to be a nasty one...
        match self {
            Self::Up(n) => Self::Down(*n), // TODO Take in account \n counts?
            Self::Down(n) => Self::Up(*n), // TODO Take in account \n counts?
            Self::Right(n) => {
                Self::Left(n + input.map(|s| s.len_without_modifiers()).unwrap_or(0) as i32)
            }
            Self::Left(n) => {
                Self::Right(n - input.map(|s| s.len_without_modifiers()).unwrap_or(0) as i32)
            }
            Self::Origin => Self::Origin,
            Self::Absolute(x, y) => self.clone(),
        }
    }
}

impl TryFrom<&ANSIEscapeCode> for MoveCursor {
    type Error = ModifierError;

    fn try_from(value: &ANSIEscapeCode) -> Result<Self, Self::Error> {
        if !"ABCDH".contains(value.end_char) {
            return Err(ModifierError::UnexpectedEndCharacter(
                stringify!($enum_name).to_string(),
                value.end_char.to_string(),
            ));
        }

        macro_rules! expand_variants {
            (
                $(
                    (
                    $name:ident,
                    $modifier_count:expr,
                    $end_char:literal,
                    $variant_builder:expr
                    $(,)?)
                ),+
            ) => {
                match (value.code, &value.modifiers, value.end_char) {
                    $(
                        (None, modifiers, $end_char) => {
                            if modifiers.len() == $modifier_count {
                                Ok($variant_builder(modifiers))
                            } else {
                                Err(ModifierError::ValueNotRecognised(
                                    stringify!($enum_name).to_string(),
                                    format!("{:?}:{:?}", value.code, value.modifiers),
                                    format!(
                                        "{} command accepts {} argument(s), but {:?} found.",
                                        stringify!($name),
                                        $modifier_count,
                                        modifiers
                                    ),
                                ))
                            }
                        }
                    ),*
                    _ => Err(ModifierError::ValueNotRecognised(
                        stringify!($enum_name).to_string(),
                        format!("{:?}:{:?}", value.code, value.modifiers),
                        String::from("Unrecognised pattern for MoveCursor."),
                    ))
                }
            };
        }

        expand_variants!(
            (Up, 1, 'A', |mods: &Vec<i32>| Self::Up(mods[0])),
            (Down, 1, 'B', |mods: &Vec<i32>| Self::Down(mods[0])),
            (Right, 1, 'C', |mods: &Vec<i32>| Self::Right(mods[0])),
            (Left, 1, 'D', |mods: &Vec<i32>| Self::Left(mods[0])),
            // Because we put the modifiers.len() check at case level instead of match,
            // Origin here will unfortunately absorb anything that should've gone to
            // Absolute!
            // (Origin, 0, 'H', | _ | Self::Origin),
            (Absolute, 2, 'H', |mods: &Vec<i32>| {
                if mods[0] == 0 && mods[1] == 0 {
                    Self::Origin
                } else {
                    Self::Absolute(mods[0], mods[1])
                }
            })
        )
    }
}

builder!(MoveCursor, MoveCursor);

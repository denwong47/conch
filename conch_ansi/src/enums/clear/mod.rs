use std::fmt;
use strum_macros::EnumIter;

use conch_base_models::{ANSIEscapeCode, IntoANSIEscapeCode, ModifierError};
use conch_macros::ansi_enum_builder as builder;

use crate::Modifier;

/// Move the position of the current cursor in the terminal.
#[allow(dead_code)]
#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum Clear {
    EntireDisplay,
    DisplayBelowCursor,
    DisplayAboveCursor,
    EntireLine,
    LineAfterCursor,
    LineBeforeCursor,
}

impl IntoANSIEscapeCode for Clear {
    /// Convert any [`Clear`] into a [`ANSIEscapeCode`].
    fn into_ansi_escape_code(&self) -> ANSIEscapeCode {
        match self {
            Self::EntireDisplay => ANSIEscapeCode::new(None, Some(vec![2]), 'J'),
            Self::DisplayBelowCursor => ANSIEscapeCode::new(None, Some(vec![0]), 'J'),
            Self::DisplayAboveCursor => ANSIEscapeCode::new(None, Some(vec![1]), 'J'),
            Self::EntireLine => ANSIEscapeCode::new(None, Some(vec![2]), 'K'),
            Self::LineAfterCursor => ANSIEscapeCode::new(None, Some(vec![0]), 'K'),
            Self::LineBeforeCursor => ANSIEscapeCode::new(None, Some(vec![1]), 'K'),
        }
    }
}

impl TryFrom<&ANSIEscapeCode> for Clear {
    type Error = ModifierError;

    fn try_from(value: &ANSIEscapeCode) -> Result<Self, Self::Error> {
        if !"JK".contains(value.end_char) {
            return Err(ModifierError::UnexpectedEndCharacter(
                stringify!($enum_name).to_string(),
                value.end_char.to_string(),
            ));
        }

        match (value.code, value.modifiers.as_slice(), value.end_char) {
            (None, [2], 'J') => Ok(Self::EntireDisplay),
            // Currently not supported
            (None, slice, 'J') if slice.len() == 0 => Ok(Self::DisplayBelowCursor),
            (None, [0], 'J') => Ok(Self::DisplayBelowCursor),
            (None, [1], 'J') => Ok(Self::DisplayAboveCursor),
            (None, [2], 'K') => Ok(Self::EntireLine),
            // Currently not supported
            (None, slice, 'K') if slice.len() == 0 => Ok(Self::LineAfterCursor),
            (None, [0], 'K') => Ok(Self::LineAfterCursor),
            (None, [1], 'K') => Ok(Self::LineBeforeCursor),
            _ => Err(ModifierError::ValueNotRecognised(
                String::from("Clear"),
                format!("{:?}:{:?}", value.code, value.modifiers),
                String::from("Unrecognised pattern for Clear."),
            )),
        }
    }
}

builder!(Clear, Clear);

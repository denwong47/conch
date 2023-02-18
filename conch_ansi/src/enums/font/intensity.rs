use std::{default, fmt};
use strum_macros::EnumIter;

use enum_index::*;

use conch_base_models::{ANSIEscapeCode, IntoANSIEscapeCode, ModifierError, Resetter};
use conch_macros::ansi_enum_builder as builder;

/// Intensity modifier
#[derive(Clone, Debug, EnumIter, EnumIndex, PartialEq)]
#[index_type(u16)]
pub enum Intensity {
    #[index(22)]
    Normal,

    #[index(1)]
    Bold,

    #[index(2)]
    Faint,
}
impl default::Default for Intensity {
    fn default() -> Self {
        Self::Normal
    }
}

impl IntoANSIEscapeCode for Intensity {
    fn into_ansi_escape_code(&self) -> ANSIEscapeCode {
        ANSIEscapeCode::new(Some(self.index()), None, 'm')
    }
}

impl TryFrom<&ANSIEscapeCode> for Intensity {
    type Error = ModifierError;

    fn try_from(value: &ANSIEscapeCode) -> Result<Self, Self::Error> {
        if value.end_char != 'm' {
            return Err(ModifierError::UnexpectedEndCharacter(
                stringify!($enum_name).to_string(),
                value.end_char.to_string(),
            ));
        }

        if value.modifiers.len() > 0 {
            return Err(ModifierError::ValueNotRecognised(
                stringify!($enum_name).to_string(),
                format!("{:?}:{:?}", value.code, value.modifiers),
                String::from("This code does not accept modifiers."),
            ));
        }

        if let Some(code) = value.code {
            Self::try_from(&code).or(Err(ModifierError::MismatchedANSICode(
                stringify!($enum_name).to_string(),
                code,
                1, // For the lack of a better code
            )))
        } else {
            Err(ModifierError::MissingANSICode(
                stringify!($enum_name).to_string(),
                1, // For the lack of a better code
            ))
        }
    }
}

builder!(Intensity);

/// An internal helper macro that builds all the auto implementations for enum classes
/// in `conch_ansi`.
#[macro_export]
macro_rules! ansi_enum_builder {
    (
        $enum_name:ident,
        $modifier_variant:ident
        $(,)?
    ) => {
        use conch_base_models::*;

        impl fmt::Display for $enum_name {
            /// Transform the object into ANSIEscapeCode, then use that to generate
            /// a String.
            ///
            /// This also implements Display.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let ansi: ANSIEscapeCode = self.into();

                write!(f, "{}", ansi.to_string())
            }
        }

        impl HasLength for $enum_name {
            fn len(&self) -> usize {
                self.to_string().len()
            }
        }

        impl<'a> TryFrom<ANSIEscapeCode> for $enum_name {
            type Error = ModifierError;

            /// Global Implementation for all the base enums, which implements
            /// [`TryFrom<&ANSIEscapeCode>`], to try from the owned version as well.
            fn try_from(value: ANSIEscapeCode) -> Result<Self, Self::Error> {
                Self::try_from(&value)
            }
        }

        impl TryFrom<&str> for $enum_name {
            type Error = ModifierError;

            /// Use ANSIEscapeCode to parse the str first, then select variant of itself if successful.
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                ANSIEscapeCode::try_from(value).and_then(|ansi| $enum_name::try_from(&ansi))
            }
        }

        impl From<$enum_name> for Modifier {
            fn from(value: $enum_name) -> Modifier {
                Modifier::$modifier_variant(value)
            }
        }
    };
}

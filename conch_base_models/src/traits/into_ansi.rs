use crate::ANSIEscapeCode;

/// Custom Trait for anything that can be converted into [`ANSIEscapeCode`].
///
/// This is necessary because we want to have the conversion code to reside within
/// the struct U, but if we just implement [`Into<ANSIEscapeCode>`] there,
/// [`From<U>`] won't be implemented for [`ANSIEscapeCode`] then. Hence an
/// intermediary trait is required.
pub trait IntoANSIEscapeCode {
    fn into_ansi_escape_code(&self) -> ANSIEscapeCode;
}

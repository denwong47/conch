use crate::{IntoANSIEscapeCode, Resetter};

/// Auto trait for structs and enums that can wrap a string.
pub trait StringWrapper {
    fn wraps(&self, text: &str) -> String;
}

impl<U> StringWrapper for U
where
    U: IntoANSIEscapeCode + Resetter,
{
    /// Enclose the text with the modifier.
    fn wraps(&self, text: &str) -> String {
        format!(
            "{}{}{}",
            self.into_ansi_escape_code(),
            text,
            self.resetter(Some(text)).into_ansi_escape_code(),
        )
    }
}

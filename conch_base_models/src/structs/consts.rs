//! Constants for all base models.

use regex::Regex;

use lazy_static::lazy_static;

#[cfg(doc)]
use crate::ANSIEscapeCode;

/// Default `sep` to use for [`ANSIEscapeCode`].
pub const DEFAULT_SEPARATOR: char = ';';

// In principle, there is one code that we won't match here, which is `\x1b[H`.
// However this "origin position" code can be easily expressed as `\x1b[0;0H`.
const BASE_CODE_PATTERN: &str = r#"\x1b\[(?P<codes>(?:\-?\d+[;:])*\-?\d+)(?P<end_char>[A-Za-z])"#;

lazy_static! {
    /// [`Regex`] pattern for an escape code anywhere within a given text.
    pub static ref ESCAPE_CODE_PATTERN: Regex = Regex::new(BASE_CODE_PATTERN).unwrap();

    /// [`Regex`] pattern for an escape code leading a given text.
    pub static ref ESCAPE_CODE_START_PATTERN: Regex =
        Regex::new((String::from(r"^") + BASE_CODE_PATTERN).as_str()).unwrap();

    /// [`Regex`] pattern for any valid separators - namely `:` and `;`.
    pub static ref SEP_PATTERN: Regex = Regex::new(r"[:;]").unwrap();
}

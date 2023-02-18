use regex::Regex;

use lazy_static::lazy_static;

/// Default `sep` to use for [`ANSIEscapeCode`].
pub const DEFAULT_SEPARATOR: char = ';';

// In principle, there is one code that we won't match here, which is `\x1b[H`.
// However this "origin position" code can be easily expressed as `\x1b[0;0H`.
const BASE_CODE_PATTERN: &str = r#"\x1b\[(?P<codes>(?:\-?\d+[;:])*\-?\d+)(?P<end_char>[A-Za-z])"#;

lazy_static! {
    pub static ref ESCAPE_CODE_PATTERN: Regex = Regex::new(BASE_CODE_PATTERN).unwrap();
    pub static ref ESCAPE_CODE_START_PATTERN: Regex =
        Regex::new((String::from(r"^") + BASE_CODE_PATTERN).as_str()).unwrap();
    pub static ref SEP_PATTERN: Regex = Regex::new(r"[:;]").unwrap();
}

/// A simple macro allowing string literals to be broken across lines without
/// affecting indentation, like Python strings.
///
/// All members need to `impl` [`std::fmt::Display`] trait, as they will be passed into
/// [`format`] macro.
///
/// Example:
///
/// ```rust
/// use conch_macros::s;
///
/// assert_eq!(
///     s!(
///         "The quick brown fox "
///         "jumps over the "
///         "lazy dog"
///     ),
///     "The quick brown fox jumps over the lazy dog"
/// );
///
/// let name = String::from("John Doe");
/// assert_eq!(
///     s!(
///         "Hello, my name is "
///         name
///         "! Nice to meet you."
///     ),
///     "Hello, my name is John Doe! Nice to meet you."
/// );
/// ```
#[macro_export]
macro_rules! s {
    ($shead:tt $($sseg:tt)*) => {
        format!("{}", $shead) $(+&format!("{}", $sseg))*
    }
}

use std::fmt::Display;

use conch_ansi::Modifier;
use conch_base_models::StringWrapper;

/// A wrapper around [`Vec<String>`] to provide more control over display.
pub struct Lines {
    pub title: Option<String>,
    pub lines: Vec<String>,
    title_prefix: Option<String>,
    prefix: String,
    title_modifier: Option<Modifier>,
    lines_modifier: Modifier,
    spacing: u8,
}
impl Lines {
    /// Create a new instance of [`Lines`] with default empty settings.
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            title: None,
            lines,
            title_prefix: None,
            prefix: String::new(),
            title_modifier: None,
            lines_modifier: Modifier::Nothing,
            spacing: 1,
        }
    }
}

impl From<Lines> for Vec<String> {
    fn from(value: Lines) -> Self {
        value.lines
    }
}

impl Display for Lines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // SPACER BLOCK
        let spacer = (0..self.spacing).fold(String::new(), |s, _| s + "\n");

        // TITLE BLOCK
        self.title
            .as_ref()
            .map(|title| {
                let mut title = self.title_prefix.as_ref().unwrap_or(&self.prefix).clone() + title;

                title = self
                    .title_modifier
                    .as_ref()
                    .unwrap_or(&self.lines_modifier)
                    .wraps(&title);

                title
            })
            .map(|title| {
                write!(f, "{}", title).and_then(|res| {
                    if self.lines.len() > 0 {
                        write!(f, "{}", &spacer)
                    } else {
                        Ok(res)
                    }
                })
            })
            // Ensure that the iterator is consumed, and all lazy evaluations completed.
            .unwrap_or(Ok(()))?;

        // TEXT BLOCK
        Result::from_iter(self.lines.iter().map(|line| line.as_str()).map(|line| {
            write!(
                f,
                "{}",
                self.lines_modifier.wraps(&(self.prefix.to_string() + line))
            )
        }))
    }
}

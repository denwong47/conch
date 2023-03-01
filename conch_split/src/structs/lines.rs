use std::fmt::Display;

use conch_ansi::Modifier;
use conch_base_models::StringWrapper;

/// A wrapper around [`Vec<String>`] to provide more control over display.
#[derive(Clone, Debug)]
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

    /// Create a new instance of [`Lines`] by splitting a block of text
    /// into lines.
    pub fn from_text<S>(text: S, max_len: u16) -> Self
    where
        S: ToString,
    {
        let _ = text.to_string();
        drop(max_len);

        todo!("Not implemented yet.")
    }

    /// Extend the lines in an instance of [`Lines`].
    pub fn extend<S>(mut self, lines: Vec<S>) -> Self
    where
        S: ToString,
    {
        self.lines.extend(lines.iter().map(|s| s.to_string()));
        self
    }

    /// Append a line to an instance of [`Lines`].
    pub fn extend_one<S>(mut self, line: S) -> Self
    where
        S: ToString,
    {
        self.lines.append(&mut vec![line.to_string()]);
        self
    }

    /// A chained function to set [`Lines::title`]
    /// on an instance.
    pub fn title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());
        self
    }

    /// A chained function to set [`Lines::title_prefix`]
    /// on an instance.
    pub fn title_prefix<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title_prefix = Some(value.to_string());
        self
    }

    /// A chained function to set [`Lines::title_modifier`]
    /// on an instance.
    pub fn title_modifier(mut self, value: Modifier) -> Self {
        self.title_modifier = Some(value);
        self
    }

    /// A chained function to set [`Lines::prefix`]
    /// on an instance.
    pub fn prefix<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.prefix = value.to_string();
        self
    }

    /// A chained function to set [`Lines::prefix`]
    /// on an instance.
    pub fn modifier(mut self, value: Modifier) -> Self {
        self.lines_modifier = value;
        self
    }

    /// A chained function to set [`Lines::spacing`]
    /// on an instance.
    pub fn spacing(mut self, value: u8) -> Self {
        self.spacing = value;
        self
    }
}

impl From<Lines> for Vec<String> {
    fn from(value: Lines) -> Self {
        value.lines
    }
}

impl<S> From<Vec<S>> for Lines
where
    S: ToString,
{
    /// Convert a [`Vec<S>`] of any `Item` that `impl` [`ToString`] into [`Lines`].
    /// Note that if you have a [`Vec<String>`], using [`Lines::new()`] will be more
    /// performant.
    fn from(value: Vec<S>) -> Self {
        Lines::new(value.into_iter().map(|s| s.to_string()).collect())
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
                        write!(
                            f,
                            "{}{}{}",
                            &spacer,
                            self.lines_modifier.wraps(&self.prefix),
                            &spacer
                        )
                    } else {
                        Ok(res)
                    }
                })
            })
            // Ensure that the iterator is consumed, and all lazy evaluations completed.
            .unwrap_or(Ok(()))?;

        // TEXT BLOCK
        let text = self.lines.iter().fold(String::new(), |s, line| {
            let sep = {
                if s.len() > 0 {
                    &spacer
                } else {
                    ""
                }
            };
            s + sep
                + self
                    .lines_modifier
                    .wraps(&(self.prefix.to_string() + line))
                    .as_str()
        });

        write!(f, "{}", text)
    }
}

//! Search for [`ANSIEscapeCode`] in a [`str`].

use std::cmp::min;
use std::ops::Range;

use regex::{Match, Matches};

use crate::Modifier;
use conch_base_models::{ANSIEscapeCode, HasLength, ESCAPE_CODE_PATTERN};

pub struct ModifiersInText<'r, 't>(Matches<'r, 't>);
impl<'r, 't> ModifiersInText<'r, 't> {
    pub fn new(s: &'t str) -> Self {
        return Self(ESCAPE_CODE_PATTERN.find_iter(s));
    }

    pub fn next_match(&mut self) -> Option<Match> {
        self.0.next()
    }
}
impl<'r, 't> Iterator for ModifiersInText<'r, 't> {
    type Item = Modifier;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(text) = self.0.next().map(|m| m.as_str()) {
                let result = ANSIEscapeCode::try_from(text);

                if let Ok(ansi) = result {
                    if let Ok(modifier) = Modifier::try_from(ansi) {
                        return Some(modifier);
                    }
                }
                // If its not valid, continue searching.
            } else {
                return None;
            }
        }
    }
}

/// Trait for [`str`] and [`String`] to iterate its modifiers.
///
/// `len` is just wrapper around its respective `len` functions, to provide a guarantee
/// that anything that `impl FindModifiers` will have a `len` function.
pub trait FindModifiers {
    fn iter_modifiers(&self) -> ModifiersInText;
    fn len(&self) -> usize;
}
impl FindModifiers for &str {
    fn iter_modifiers(&self) -> ModifiersInText {
        ModifiersInText::new(self)
    }

    fn len(&self) -> usize {
        return str::len(self);
    }
}
impl FindModifiers for String {
    fn iter_modifiers(&self) -> ModifiersInText {
        ModifiersInText::new(&self)
    }

    fn len(&self) -> usize {
        return String::len(self);
    }
}

pub trait LengthWithoutModifiers: FindModifiers {
    fn len_without_modifiers(&self) -> usize;
}
impl<T> LengthWithoutModifiers for T
where
    T: FindModifiers,
{
    fn len_without_modifiers(&self) -> usize {
        let modifier_count = {
            self.iter_modifiers()
                .fold(0_usize, |count, matched| count + matched.len())
        };

        return self.len() - modifier_count;
    }
}

#[allow(dead_code)]
pub struct RangeWithoutModifiers<'t> {
    text: &'t str,
    modifier_ranges: Vec<(usize, usize)>,
}
impl<'t> RangeWithoutModifiers<'t> {
    pub fn new(text: &'t str) -> Self {
        Self {
            text,
            modifier_ranges: {
                let mut v = vec![];
                let mut iter = text.iter_modifiers();

                loop {
                    match iter.next_match() {
                        Some(m) => v.push((m.start(), m.end())),
                        None => break,
                    }
                }

                v
            },
        }
    }

    #[allow(dead_code)]
    pub fn index_without_modifiers(&self, idx: usize) -> usize {
        self.modifier_ranges
            .iter()
            .map(|(start, end)| {
                if *start >= idx {
                    0
                } else {
                    min(idx, *end) - start
                }
            })
            .fold(idx, |lhs, rhs| lhs - rhs)
    }

    pub fn index_with_modifiers(&self, idx: usize) -> usize {
        self.modifier_ranges
            .iter()
            .map(|(start, end)| {
                if *start >= idx {
                    0
                } else {
                    min(idx, *end) - start
                }
            })
            .fold(idx, |lhs, rhs| lhs + rhs)
    }

    #[allow(dead_code)]
    pub fn range_without_modifiers(&self, range: Range<usize>) -> Range<usize> {
        self.index_without_modifiers(range.start)..self.index_without_modifiers(range.end)
    }

    #[allow(dead_code)]
    pub fn range_with_modifiers(&self, range: Range<usize>) -> Range<usize> {
        self.index_with_modifiers(range.start)..self.index_with_modifiers(range.end)
    }
}

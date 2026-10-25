//! Rendering checked sentences back into ordinary text.

use std::borrow::Cow;
use std::fmt;

use crate::lexicon::capitalize;

/// One sentence: a subject, a finite verb that agrees with it, and whatever
/// follows. `Display` capitalises it, spaces it and ends it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sentence {
    subject: String,
    verb: Cow<'static, str>,
    rest: Vec<Cow<'static, str>>,
    end: char,
}

impl Sentence {
    pub(crate) fn runtime(subject: String, verb: impl Into<Cow<'static, str>>) -> Self {
        Sentence {
            subject,
            verb: verb.into(),
            rest: Vec::new(),
            end: '.',
        }
    }

    /// Used by `english!`.
    #[doc(hidden)]
    pub fn from_macro(
        subject: String,
        verb: &'static str,
        rest: &[&'static str],
        end: char,
    ) -> Self {
        let rest = rest.iter().map(|&t| Cow::Borrowed(t)).collect();
        Sentence {
            subject,
            verb: verb.into(),
            rest,
            end,
        }
    }

    /// Append words after the verb: `She.verb("write").then("Rust")`.
    pub fn then(mut self, words: impl fmt::Display) -> Self {
        let words = words.to_string();
        if !words.is_empty() {
            self.rest.push(words.into());
        }
        self
    }

    /// End with `?` instead of `.`.
    pub fn question(mut self) -> Self {
        self.end = '?';
        self
    }

    /// End with `!` instead of `.`.
    pub fn exclaim(mut self) -> Self {
        self.end = '!';
        self
    }
}

/// Tokens that attach to the word before them.
fn closes(token: &str) -> bool {
    matches!(
        token,
        "." | "," | "!" | "?" | ";" | ":" | ")" | "]" | "%" | "-" | "..." | ".."
    ) || token.starts_with('\'')
}

/// Tokens that attach to the word after them.
fn opens(token: &str) -> bool {
    matches!(token, "(" | "[" | "-" | "$")
}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = capitalize(&self.subject);
        let mut previous = self.subject.as_str();
        for token in std::iter::once(&self.verb).chain(&self.rest) {
            if !closes(token) && !opens(previous) {
                out.push(' ');
            }
            out.push_str(token);
            previous = token;
        }
        if !matches!(out.chars().last(), Some('.' | '!' | '?')) {
            out.push(self.end);
        }
        f.write_str(&out)
    }
}

/// Several sentences.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text(pub Vec<Sentence>);

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, sentence) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            sentence.fmt(f)?;
        }
        Ok(())
    }
}

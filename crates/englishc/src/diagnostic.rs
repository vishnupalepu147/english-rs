//! What a lint reports.

use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warning,
}

#[derive(Clone, Debug)]
pub struct Label {
    pub span: Range<usize>,
    pub message: String,
    pub primary: bool,
}

#[derive(Clone, Debug)]
pub struct Suggestion {
    pub span: Range<usize>,
    pub replacement: String,
    /// Safe for `--fix` to apply without a human looking at it.
    pub applicable: bool,
}

#[derive(Clone, Debug)]
pub struct Help {
    pub message: String,
    pub suggestion: Option<Suggestion>,
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub level: Level,
    /// `E0001` for errors.
    pub code: Option<&'static str>,
    /// The lint name, which `#![allow(..)]` and `-A` accept.
    pub lint: &'static str,
    pub message: String,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
    pub helps: Vec<Help>,
}

impl Diagnostic {
    pub fn error(code: &'static str, lint: &'static str, message: impl Into<String>) -> Self {
        Diagnostic {
            level: Level::Error,
            code: Some(code),
            lint,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
        }
    }

    pub fn warning(lint: &'static str, message: impl Into<String>) -> Self {
        Diagnostic {
            level: Level::Warning,
            code: None,
            lint,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
        }
    }

    pub fn primary(mut self, span: Range<usize>, message: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
            primary: true,
        });
        self
    }

    pub fn secondary(mut self, span: Range<usize>, message: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
            primary: false,
        });
        self
    }

    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// A help line with no code suggestion.
    pub fn help(mut self, message: impl Into<String>) -> Self {
        self.helps.push(Help {
            message: message.into(),
            suggestion: None,
        });
        self
    }

    /// A help line that `--fix` may apply.
    pub fn fix(
        mut self,
        message: impl Into<String>,
        span: Range<usize>,
        replacement: impl Into<String>,
    ) -> Self {
        self.helps.push(Help {
            message: message.into(),
            suggestion: Some(Suggestion {
                span,
                replacement: replacement.into(),
                applicable: true,
            }),
        });
        self
    }

    /// A help line with a suggestion that needs a human to choose it.
    pub fn suggest(
        mut self,
        message: impl Into<String>,
        span: Range<usize>,
        replacement: impl Into<String>,
    ) -> Self {
        self.helps.push(Help {
            message: message.into(),
            suggestion: Some(Suggestion {
                span,
                replacement: replacement.into(),
                applicable: false,
            }),
        });
        self
    }

    /// Where the diagnostic points.
    pub fn span(&self) -> Range<usize> {
        self.labels
            .iter()
            .find(|l| l.primary)
            .or(self.labels.first())
            .map_or(0..0, |l| l.span.clone())
    }

    pub fn fixes(&self) -> impl Iterator<Item = &Suggestion> {
        self.helps
            .iter()
            .filter_map(|h| h.suggestion.as_ref())
            .filter(|s| s.applicable)
    }
}

//! rustc's diagnostic layout, for prose.

use std::ops::Range;

use crate::diagnostic::{Diagnostic, Label, Level, Suggestion};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Plain,
    Bold,
    Error,
    Warning,
    /// Line numbers, `|` and `-->`.
    Gutter,
    /// `-` underlines and their labels.
    Secondary,
    Note,
    Help,
    Added,
    Removed,
    Success,
}

/// How styled text is written out: plain, ANSI escapes, or HTML spans.
pub trait Painter {
    fn paint(&self, style: Style, text: &str) -> String;
}

pub struct Plain;

impl Painter for Plain {
    fn paint(&self, _: Style, text: &str) -> String {
        text.to_string()
    }
}

pub struct Ansi;

impl Painter for Ansi {
    fn paint(&self, style: Style, text: &str) -> String {
        let code = match style {
            Style::Plain => return text.to_string(),
            Style::Bold => "1",
            Style::Error => "1;91",
            Style::Warning => "1;93",
            Style::Gutter | Style::Secondary => "1;94",
            Style::Note | Style::Success => "1;92",
            Style::Help => "1;96",
            Style::Added => "92",
            Style::Removed => "91",
        };
        format!("\x1b[{code}m{text}\x1b[0m")
    }
}

/// `<span class="…">` with the text escaped. Used by the web playground.
pub struct Html;

impl Painter for Html {
    fn paint(&self, style: Style, text: &str) -> String {
        let escaped = text
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        let class = match style {
            Style::Plain => return escaped,
            Style::Bold => "b",
            Style::Error => "err",
            Style::Warning => "warn",
            Style::Gutter | Style::Secondary => "gut",
            Style::Note => "note",
            Style::Success => "ok",
            Style::Help => "help",
            Style::Added => "add",
            Style::Removed => "del",
        };
        format!("<span class=\"{class}\">{escaped}</span>")
    }
}

pub struct Renderer<'a> {
    src: &'a str,
    file: &'a str,
    painter: &'a dyn Painter,
    /// Byte range of each line, without its newline.
    lines: Vec<Range<usize>>,
}

/// One cell of an annotation line.
type Cell = (char, Style);

impl<'a> Renderer<'a> {
    pub fn new(src: &'a str, file: &'a str, painter: &'a dyn Painter) -> Self {
        let mut lines = Vec::new();
        let mut start = 0;
        for (i, c) in src.char_indices() {
            if c == '\n' {
                lines.push(start..i);
                start = i + 1;
            }
        }
        lines.push(start..src.len());
        Renderer {
            src,
            file,
            painter,
            lines,
        }
    }

    fn p(&self, style: Style, text: &str) -> String {
        self.painter.paint(style, text)
    }

    /// Zero-based line index of a byte offset.
    fn line_of(&self, byte: usize) -> usize {
        self.lines
            .partition_point(|l| l.end < byte)
            .min(self.lines.len() - 1)
    }

    /// One-based column, in characters.
    fn col_of(&self, line: usize, byte: usize) -> usize {
        let start = self.lines[line].start;
        self.src[start..byte.max(start)].chars().count() + 1
    }

    fn line_text(&self, line: usize) -> String {
        self.src[self.lines[line].clone()].replace('\t', " ")
    }

    fn level_style(level: Level) -> Style {
        match level {
            Level::Error => Style::Error,
            Level::Warning => Style::Warning,
        }
    }

    pub fn diagnostic(&self, d: &Diagnostic) -> String {
        let mut out = String::new();
        let level_style = Self::level_style(d.level);
        let head = match (d.level, d.code) {
            (Level::Error, Some(code)) => format!("error[{code}]"),
            (Level::Error, None) => "error".to_string(),
            (Level::Warning, _) => "warning".to_string(),
        };
        out += &self.p(level_style, &head);
        out += &self.p(Style::Bold, &format!(": {}", d.message));
        out.push('\n');

        let mut shown: Vec<usize> = d
            .labels
            .iter()
            .map(|l| self.line_of(l.span.start))
            .collect();
        for s in d.helps.iter().filter_map(|h| h.suggestion.as_ref()) {
            shown.push(self.line_of(s.span.end));
        }
        let width = shown
            .iter()
            .map(|&l| (l + 1).to_string().len())
            .max()
            .unwrap_or(1);
        let pad = " ".repeat(width);
        let gutter = |s: &str| self.p(Style::Gutter, s);

        let span = d.span();
        let line = self.line_of(span.start);
        out += &format!(
            "{pad}{} {}:{}:{}\n",
            gutter("-->"),
            self.file,
            line + 1,
            self.col_of(line, span.start)
        );
        out += &format!("{pad} {}\n", gutter("|"));

        let mut label_lines: Vec<usize> = d
            .labels
            .iter()
            .map(|l| self.line_of(l.span.start))
            .collect();
        label_lines.sort_unstable();
        label_lines.dedup();
        let mut previous: Option<usize> = None;
        for &line in &label_lines {
            if let Some(prev) = previous {
                if line == prev + 2 {
                    out += &self.source_line(prev + 1, width);
                } else if line > prev + 2 {
                    out += &format!("{}\n", gutter("..."));
                }
            }
            out += &self.source_line(line, width);
            let labels: Vec<&Label> = d
                .labels
                .iter()
                .filter(|l| self.line_of(l.span.start) == line)
                .collect();
            for annotation in self.annotations(line, &labels, d.level) {
                out += &format!("{pad} {} {}\n", gutter("|"), annotation);
            }
            previous = Some(line);
        }

        let inline_helps: Vec<&str> = d
            .helps
            .iter()
            .filter(|h| h.suggestion.is_none())
            .map(|h| h.message.as_str())
            .collect();
        if !d.notes.is_empty()
            || !inline_helps.is_empty()
            || d.helps.iter().any(|h| h.suggestion.is_some())
        {
            out += &format!("{pad} {}\n", gutter("|"));
        }
        for note in &d.notes {
            out += &format!(
                "{pad} {} {}: {}\n",
                gutter("="),
                self.p(Style::Note, "note"),
                self.p(Style::Plain, note)
            );
        }
        for help in inline_helps {
            out += &format!(
                "{pad} {} {}: {}\n",
                gutter("="),
                self.p(Style::Help, "help"),
                self.p(Style::Plain, help)
            );
        }
        for help in &d.helps {
            if let Some(s) = &help.suggestion {
                out += &format!(
                    "{}: {}\n",
                    self.p(Style::Help, "help"),
                    self.p(Style::Plain, &help.message)
                );
                out += &self.suggestion(s, width);
            }
        }
        out
    }

    fn source_line(&self, line: usize, width: usize) -> String {
        let number = format!("{:>width$} |", line + 1);
        format!(
            "{} {}\n",
            self.p(Style::Gutter, &number),
            self.p(Style::Plain, &self.line_text(line))
        )
    }

    /// The underline row and the label rows beneath it, laid out the way
    /// rustc does: the rightmost label sits on the underline row, and the
    /// rest hang below on `|` connectors.
    fn annotations(&self, line: usize, labels: &[&Label], level: Level) -> Vec<String> {
        let line_end = self.lines[line].end;
        let mut marks: Vec<(usize, usize, Style, char, &str)> = labels
            .iter()
            .map(|l| {
                let start = self.col_of(line, l.span.start) - 1;
                let end = self.col_of(line, l.span.end.min(line_end)) - 1;
                let (style, ch) = if l.primary {
                    (Self::level_style(level), '^')
                } else {
                    (Style::Secondary, '-')
                };
                (start, end.max(start + 1), style, ch, l.message.as_str())
            })
            .collect();
        marks.sort_by_key(|m| m.0);

        let mut underline: Vec<Cell> = Vec::new();
        for &(start, end, style, ch, _) in &marks {
            if underline.len() < end {
                underline.resize(end, (' ', Style::Plain));
            }
            for cell in &mut underline[start..end] {
                *cell = (ch, style);
            }
        }
        let mut rows = Vec::new();
        let (last, rest) = marks.split_last().expect("at least one label");
        if !last.4.is_empty() {
            underline.push((' ', Style::Plain));
            underline.extend(last.4.chars().map(|c| (c, last.2)));
        }
        rows.push(self.cells(&underline));

        let hanging: Vec<_> = rest.iter().filter(|m| !m.4.is_empty()).collect();
        if hanging.is_empty() {
            return rows;
        }
        let connectors = |upto: usize| {
            let mut row: Vec<Cell> = Vec::new();
            for m in hanging.iter().take(upto) {
                row.resize(m.0, (' ', Style::Plain));
                row.push(('|', m.2));
            }
            row
        };
        rows.push(self.cells(&connectors(hanging.len())));
        for (i, m) in hanging.iter().enumerate().rev() {
            let mut row = connectors(i);
            row.resize(m.0, (' ', Style::Plain));
            row.extend(m.4.chars().map(|c| (c, m.2)));
            rows.push(self.cells(&row));
        }
        rows
    }

    fn cells(&self, cells: &[Cell]) -> String {
        let mut out = String::new();
        let mut run = String::new();
        let mut style = Style::Plain;
        for &(c, s) in cells {
            if s != style && !run.is_empty() {
                out += &self.p(style, &run);
                run.clear();
            }
            style = s;
            run.push(c);
        }
        out += &self.p(style, &run);
        out.trim_end().to_string()
    }

    /// A suggestion as a diff of the affected lines.
    fn suggestion(&self, s: &Suggestion, width: usize) -> String {
        let pad = " ".repeat(width);
        let gutter = |t: &str| self.p(Style::Gutter, t);
        let first = self.line_of(s.span.start);
        let last = self.line_of(s.span.end.max(s.span.start));
        let region = self.lines[first].start..self.lines[last].end;
        let before = &self.src[region.start..s.span.start];
        let removed = &self.src[s.span.clone()];
        let after = &self.src[s.span.end..region.end];

        let mut out = format!("{pad} {}\n", gutter("|"));
        let old = format!(
            "{}{}{}",
            self.p(Style::Plain, before),
            self.p(Style::Removed, removed),
            self.p(Style::Plain, after)
        );
        let new = format!(
            "{}{}{}",
            self.p(Style::Plain, before),
            self.p(Style::Added, &s.replacement),
            self.p(Style::Plain, after)
        );
        for (i, text) in old.split('\n').enumerate() {
            out += &format!(
                "{} {}\n",
                gutter(&format!("{:>width$}", first + i + 1)),
                self.diff_line('-', text)
            );
        }
        for (i, text) in new.split('\n').enumerate() {
            out += &format!(
                "{} {}\n",
                gutter(&format!("{:>width$}", first + i + 1)),
                self.diff_line('+', text)
            );
        }
        out += &format!("{pad} {}\n", gutter("|"));
        out
    }

    fn diff_line(&self, sign: char, text: &str) -> String {
        let style = if sign == '+' {
            Style::Added
        } else {
            Style::Removed
        };
        format!(
            "{} {}",
            self.p(style, &sign.to_string()),
            text.replace('\t', " ")
        )
    }

    /// `file:line:col: error[E0001]: message`, for `--error-format short`.
    pub fn short(&self, d: &Diagnostic) -> String {
        let span = d.span();
        let line = self.line_of(span.start);
        let head = match (d.level, d.code) {
            (Level::Error, Some(code)) => format!("error[{code}]"),
            (Level::Error, None) => "error".to_string(),
            (Level::Warning, _) => "warning".to_string(),
        };
        format!(
            "{}:{}:{}: {}: {}\n",
            self.file,
            line + 1,
            self.col_of(line, span.start),
            self.p(Self::level_style(d.level), &head),
            self.p(Style::Plain, &d.message)
        )
    }
}

/// The closing line: "could not compile" or "generated N warnings".
pub fn summary(
    file: &str,
    errors: usize,
    warnings: usize,
    fixable: usize,
    painter: &dyn Painter,
) -> Option<String> {
    let plural = |n: usize, word: &str| {
        if n == 1 {
            format!("{n} {word}")
        } else {
            format!("{n} {word}s")
        }
    };
    let fix_hint = if fixable > 0 {
        format!(
            " (run `englishc --fix {file}` to apply {})",
            plural(fixable, "suggestion")
        )
    } else {
        String::new()
    };
    if errors > 0 {
        let warned = if warnings > 0 {
            format!("; {} emitted", plural(warnings, "warning"))
        } else {
            String::new()
        };
        Some(format!(
            "{}: could not compile `{file}` due to {} previous {}{warned}{fix_hint}\n",
            painter.paint(Style::Error, "error"),
            errors,
            if errors == 1 { "error" } else { "errors" },
        ))
    } else if warnings > 0 {
        Some(format!(
            "{}: `{file}` generated {}{fix_hint}\n",
            painter.paint(Style::Warning, "warning"),
            plural(warnings, "warning")
        ))
    } else {
        None
    }
}

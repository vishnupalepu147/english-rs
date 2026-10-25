//! Turns prose into tokens and sentences.
//!
//! Words keep their internal apostrophes and hyphens (`don't`, `well-known`),
//! abbreviations keep their dots (`e.g.`), and code spans, fenced blocks and
//! URLs become opaque `Code` tokens that no lint looks inside. Markdown list
//! markers and headings start new sentences. Lines of the form
//! `#![allow(lint)]` and `#[allow(lint)]` are attributes, as in Rust.

use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Word,
    Punct,
    Code,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub kind: Kind,
    pub text: &'a str,
    /// Lower-cased, with curly apostrophes straightened.
    pub lower: String,
    pub span: Range<usize>,
    pub sentence: usize,
}

impl Token<'_> {
    pub fn is_word(&self) -> bool {
        self.kind == Kind::Word
    }
}

/// Why a sentence started where it did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    /// The first sentence of the document.
    Document,
    /// After `.`, `!` or `?`.
    Terminal,
    /// After a blank line.
    Paragraph,
    /// After a list marker or heading, where lower case is normal.
    Structure,
    /// After an ellipsis, which may or may not have ended a sentence.
    Ellipsis,
}

/// An `allow` attribute: a lint name or error code, and the bytes it covers
/// (`None` for the whole file).
#[derive(Clone, Debug)]
pub struct Allow {
    pub lint: String,
    pub range: Option<Range<usize>>,
}

pub struct Doc<'a> {
    pub src: &'a str,
    pub tokens: Vec<Token<'a>>,
    /// How each sentence started, indexed by `Token::sentence`.
    pub starts: Vec<Start>,
    pub allows: Vec<Allow>,
}

const ABBREVIATIONS: &[&str] = &[
    "e.g", "i.e", "etc", "vs", "mr", "mrs", "ms", "dr", "st", "jr", "sr", "prof", "inc", "ltd",
    "approx", "cf", "al", "fig", "dept", "est", "u.s", "a.m", "p.m",
];

fn is_apostrophe(c: char) -> bool {
    c == '\'' || c == '\u{2019}'
}

pub fn normalize(word: &str) -> String {
    word.chars()
        .map(|c| if is_apostrophe(c) { '\'' } else { c })
        .collect::<String>()
        .to_lowercase()
}

impl<'a> Doc<'a> {
    pub fn parse(src: &'a str) -> Doc<'a> {
        Lexer::new(src).run()
    }

    /// The token at `i`, if it is a word.
    pub fn word(&self, i: usize) -> Option<&Token<'a>> {
        self.tokens.get(i).filter(|t| t.is_word())
    }

    /// The index of the word right after `i`, with nothing but whitespace
    /// between them and in the same sentence.
    pub fn next_word(&self, i: usize) -> Option<usize> {
        let next = self.word(i + 1)?;
        (next.sentence == self.tokens[i].sentence).then_some(i + 1)
    }

    /// The index of the word right before `i`, in the same sentence.
    pub fn prev_word(&self, i: usize) -> Option<usize> {
        let prev = self.word(i.checked_sub(1)?)?;
        (prev.sentence == self.tokens[i].sentence).then_some(i - 1)
    }

    /// The previous token of any kind in the same sentence.
    pub fn prev(&self, i: usize) -> Option<&Token<'a>> {
        let prev = self.tokens.get(i.checked_sub(1)?)?;
        (prev.sentence == self.tokens[i].sentence).then_some(prev)
    }

    /// Is `i` the first word of its sentence (ignoring opening punctuation)?
    pub fn starts_sentence(&self, i: usize) -> bool {
        let sentence = self.tokens[i].sentence;
        !self.tokens[..i]
            .iter()
            .rev()
            .take_while(|t| t.sentence == sentence)
            .any(|t| t.kind != Kind::Punct)
    }

    pub fn lower(&self, i: usize) -> &str {
        &self.tokens[i].lower
    }
}

struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    tokens: Vec<Token<'a>>,
    starts: Vec<Start>,
    allows: Vec<Allow>,
    sentence: usize,
    /// A sentence break waiting for the next word.
    pending: Option<Start>,
    at_line_start: bool,
    /// An `#[allow]` waiting for the next non-blank line.
    pending_allows: Vec<String>,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Lexer {
            src,
            pos: 0,
            tokens: Vec::new(),
            starts: vec![Start::Document],
            allows: Vec::new(),
            sentence: 0,
            pending: None,
            at_line_start: true,
            pending_allows: Vec::new(),
        }
    }

    fn rest(&self) -> &'a str {
        &self.src[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn line_end(&self) -> usize {
        self.rest()
            .find('\n')
            .map_or(self.src.len(), |n| self.pos + n)
    }

    fn break_sentence(&mut self, why: Start) {
        // Paragraphs and structure outrank a terminal that came first.
        match self.pending {
            Some(Start::Paragraph | Start::Structure) => {}
            _ => self.pending = Some(why),
        }
    }

    fn push(&mut self, kind: Kind, span: Range<usize>) {
        // `"Hello, world!" program`, `(thanks to SemVer!) and`: a terminal
        // inside quotes or brackets, followed by lower case, was not the end.
        let lowercase = self.src[span.clone()].starts_with(char::is_lowercase);
        if kind == Kind::Word && lowercase && self.pending == Some(Start::Terminal) {
            let n = self.tokens.len();
            let closed = n >= 2
                && matches!(
                    self.tokens[n - 1].text,
                    "\"" | "\u{201d}" | "'" | "\u{2019}" | ")" | "]"
                )
                && matches!(self.tokens[n - 2].text, "." | "!" | "?");
            if closed {
                self.pending = None;
            }
        }
        if kind != Kind::Punct
            && let Some(why) = self.pending.take()
        {
            if self
                .tokens
                .last()
                .is_some_and(|t| t.sentence == self.sentence)
            {
                self.sentence += 1;
                self.starts.push(why);
            } else if let Some(last) = self.starts.last_mut() {
                // Nothing in the current sentence yet: just say why it starts.
                if *last != Start::Document {
                    *last = why;
                }
            }
        }
        let text = &self.src[span.clone()];
        self.tokens.push(Token {
            kind,
            text,
            lower: normalize(text),
            span,
            sentence: self.sentence,
        });
    }

    fn run(mut self) -> Doc<'a> {
        while let Some(c) = self.peek() {
            if c == '\n' {
                self.pos += 1;
                self.at_line_start = true;
                let line = &self.src[self.pos..self.line_end()];
                if line.trim().is_empty() {
                    self.break_sentence(Start::Paragraph);
                }
                continue;
            }
            if c.is_whitespace() {
                self.pos += c.len_utf8();
                continue;
            }
            if self.at_line_start {
                self.at_line_start = false;
                self.apply_pending_allows();
                if self.line_structure() {
                    continue;
                }
            }
            if self.rest().starts_with("```") {
                self.fenced_code();
            } else if c == '`' {
                self.inline_code();
            } else if ["http://", "https://", "www."]
                .iter()
                .any(|p| self.rest().starts_with(p))
            {
                let end = self
                    .rest()
                    .find(char::is_whitespace)
                    .map_or(self.src.len(), |n| self.pos + n);
                let end = self.src[..end]
                    .trim_end_matches(['.', ',', ')', ';', ':'])
                    .len()
                    .max(self.pos + 1);
                self.push(Kind::Code, self.pos..end);
                self.pos = end;
            } else if c.is_alphanumeric() {
                self.word();
            } else {
                self.punct(c);
            }
        }
        Doc {
            src: self.src,
            tokens: self.tokens,
            starts: self.starts,
            allows: self.allows,
        }
    }

    /// Attributes, headings and list markers at the start of a line. Returns
    /// whether anything was consumed.
    fn line_structure(&mut self) -> bool {
        let rest = self.rest();
        if rest.starts_with("#![") || rest.starts_with("#[") {
            let end = self.line_end();
            let line = &self.src[self.pos..end];
            let inner = line.starts_with("#![");
            if let Some(lints) = parse_allow(line) {
                if inner {
                    self.allows
                        .extend(lints.into_iter().map(|lint| Allow { lint, range: None }));
                } else {
                    self.pending_allows.extend(lints);
                }
                self.pos = end;
                return true;
            }
        }
        let marker = rest
            .find(|c: char| c != '#')
            .filter(|&n| n > 0 && rest[n..].starts_with(' '))
            .or_else(|| {
                let bullet = ["- ", "* ", "+ ", "> "].iter().any(|b| rest.starts_with(b));
                let digits = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
                let numbered = digits > 0
                    && (rest[digits..].starts_with(". ") || rest[digits..].starts_with(") "));
                if bullet {
                    Some(1)
                } else if numbered {
                    Some(digits + 1)
                } else {
                    None
                }
            });
        if let Some(n) = marker {
            self.pending = Some(Start::Structure);
            self.pos += n;
            return true;
        }
        false
    }

    fn apply_pending_allows(&mut self) {
        if self.pending_allows.is_empty() {
            return;
        }
        let line = self.pos..self.line_end();
        for lint in self.pending_allows.drain(..) {
            self.allows.push(Allow {
                lint,
                range: Some(line.clone()),
            });
        }
    }

    fn fenced_code(&mut self) {
        let start = self.pos;
        let after = start + 3;
        let end = self.src[after..]
            .find("```")
            .map_or(self.src.len(), |n| after + n + 3);
        self.push(Kind::Code, start..end);
        self.break_sentence(Start::Paragraph);
        self.pos = end;
    }

    fn inline_code(&mut self) {
        let start = self.pos;
        let line_end = self.line_end();
        let end = self.src[start + 1..line_end]
            .find('`')
            .map_or(line_end, |n| start + 1 + n + 1);
        self.push(Kind::Code, start..end);
        self.pos = end;
    }

    fn word(&mut self) {
        let start = self.pos;
        let mut chars = self.rest().char_indices().peekable();
        let mut end = start;
        let mut all_digits = true;
        while let Some((offset, c)) = chars.next() {
            let next = chars.peek().map(|&(_, c)| c);
            let joins = |n: Option<char>| n.is_some_and(char::is_alphanumeric);
            if c.is_alphanumeric() {
                all_digits &= c.is_ascii_digit();
                end = start + offset + c.len_utf8();
            } else if offset > 0 && (is_apostrophe(c) || c == '-') && joins(next) {
                all_digits = false;
                end = start + offset + c.len_utf8();
            } else if offset > 0
                && (c == '.' && joins(next)
                    || c == ',' && all_digits && next.is_some_and(|n| n.is_ascii_digit()))
            {
                // "e.g", "3.14", "1,000".
                end = start + offset + 1;
            } else {
                break;
            }
        }
        if self.src[end..].starts_with('.')
            && ABBREVIATIONS.contains(&normalize(&self.src[start..end]).as_str())
        {
            end += 1;
        }
        self.push(Kind::Word, start..end);
        self.pos = end;
    }

    fn punct(&mut self, c: char) {
        let start = self.pos;
        if c == '.' && self.rest().starts_with("...") {
            self.push(Kind::Punct, start..start + 3);
            self.pos += 3;
            self.break_sentence(Start::Ellipsis);
            return;
        }
        if c == '\u{2026}' {
            self.push(Kind::Punct, start..start + c.len_utf8());
            self.pos += c.len_utf8();
            self.break_sentence(Start::Ellipsis);
            return;
        }
        self.push(Kind::Punct, start..start + c.len_utf8());
        self.pos += c.len_utf8();
        // A terminal ends a sentence only when it is attached to what came
        // before: "Press ? to quit" and "the .rs extension" go on.
        let attached = self.src[..start]
            .chars()
            .next_back()
            .is_some_and(|p| !p.is_whitespace());
        // "~/.cargo", "?query": a terminal glued to the next word is not one.
        let glued = self
            .rest()
            .chars()
            .next()
            .is_some_and(char::is_alphanumeric);
        if matches!(c, '.' | '!' | '?') && attached && !glued {
            self.break_sentence(Start::Terminal);
        }
    }
}

/// `#![allow(a, b)]` or `#[allow(a)]` to `["a", "b"]`.
fn parse_allow(line: &str) -> Option<Vec<String>> {
    let line = line.trim();
    let inner = line
        .strip_prefix("#![")
        .or_else(|| line.strip_prefix("#["))?
        .strip_suffix(']')?;
    let args = inner
        .trim()
        .strip_prefix("allow")?
        .trim()
        .strip_prefix('(')?
        .strip_suffix(')')?;
    Some(
        args.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(src: &str) -> Vec<(String, usize)> {
        Doc::parse(src)
            .tokens
            .iter()
            .filter(|t| t.is_word())
            .map(|t| (t.text.to_string(), t.sentence))
            .collect()
    }

    #[test]
    fn words_and_sentences() {
        let w = words("I don't know, e.g. this. It's well-known! Pi is 3.14 and 1,000 too");
        let texts: Vec<_> = w.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(
            texts,
            [
                "I",
                "don't",
                "know",
                "e.g.",
                "this",
                "It's",
                "well-known",
                "Pi",
                "is",
                "3.14",
                "and",
                "1,000",
                "too"
            ]
        );
        let sentences: Vec<_> = w.iter().map(|(_, s)| *s).collect();
        assert_eq!(sentences, [0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2]);
    }

    #[test]
    fn code_and_urls_are_opaque() {
        let doc =
            Doc::parse("Run `cargo build` at https://example.com. Then\n```\nhe are\n```\ndone");
        let kinds: Vec<_> = doc.tokens.iter().map(|t| (t.kind, t.text)).collect();
        assert!(kinds.contains(&(Kind::Code, "`cargo build`")));
        assert!(kinds.contains(&(Kind::Code, "https://example.com")));
        assert!(!kinds.iter().any(|(_, t)| *t == "he"));
    }

    #[test]
    fn structure_and_attributes() {
        let doc = Doc::parse(
            "#![allow(repeated_word)]\n# Title\n- item one\n- item two\n\n#[allow(E0002)]\na apple",
        );
        assert_eq!(doc.allows.len(), 2);
        assert!(doc.allows[0].range.is_none());
        assert_eq!(
            doc.starts,
            [
                Start::Document,
                Start::Structure,
                Start::Structure,
                Start::Paragraph
            ]
        );
    }
}

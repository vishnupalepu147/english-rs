//! The lints. Each module adds diagnostics for one family of mistakes.

mod agreement;
mod article;
mod dictionary;
mod homophones;
mod morphology;
mod pronoun;
mod style;

use crate::diagnostic::{Diagnostic, Level};
use crate::lexer::Doc;

pub struct LintInfo {
    pub name: &'static str,
    pub code: Option<&'static str>,
    pub level: Level,
    pub summary: &'static str,
}

const fn error(code: &'static str, name: &'static str, summary: &'static str) -> LintInfo {
    LintInfo {
        name,
        code: Some(code),
        level: Level::Error,
        summary,
    }
}

const fn warning(name: &'static str, summary: &'static str) -> LintInfo {
    LintInfo {
        name,
        code: None,
        level: Level::Warning,
        summary,
    }
}

pub const LINTS: &[LintInfo] = &[
    error(
        "E0001",
        "subject_verb_agreement",
        "a verb form that does not agree with its subject",
    ),
    error(
        "E0002",
        "indefinite_article",
        "`a` before a vowel sound or `an` before a consonant sound",
    ),
    error(
        "E0003",
        "of_for_have",
        "`of` written for `have` after a modal (`could of`)",
    ),
    error(
        "E0004",
        "regularized_verb",
        "`-ed` on an irregular verb (`goed`)",
    ),
    error(
        "E0005",
        "wrong_verb_form",
        "past tense where a participle belongs, or the reverse",
    ),
    error(
        "E0006",
        "regularized_plural",
        "`-s` on an irregular noun (`childs`)",
    ),
    error("E0007", "its_confusion", "`its` and `it's` swapped"),
    error("E0008", "your_confusion", "`your` and `you're` swapped"),
    error(
        "E0009",
        "there_confusion",
        "`there`, `their` and `they're` swapped",
    ),
    error("E0010", "then_for_than", "`then` after a comparative"),
    error(
        "E0011",
        "unknown_word",
        "a word not in the dictionary, with a known correction",
    ),
    error(
        "E0012",
        "lowercase_i",
        "the pronoun `I` written in lower case",
    ),
    error(
        "E0013",
        "objective_case_subject",
        "an object pronoun as a subject (`me and him went`)",
    ),
    warning(
        "ambiguous_pronoun",
        "`it` or `they` with two possible antecedents",
    ),
    warning("double_negative", "two negatives in one clause"),
    warning("repeated_word", "the same word twice in a row"),
    warning(
        "absolute_adjective",
        "an intensifier on an adjective that cannot be graded",
    ),
    warning("unused_literally", "`literally`, which has no effect"),
    warning(
        "non_capitalized_sentences",
        "a sentence that starts in lower case",
    ),
];

pub fn info(name_or_code: &str) -> Option<&'static LintInfo> {
    LINTS
        .iter()
        .find(|l| l.name == name_or_code || l.code == Some(name_or_code))
}

pub fn run(doc: &Doc) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    agreement::check(doc, &mut out);
    agreement::check_case(doc, &mut out);
    article::check(doc, &mut out);
    morphology::check(doc, &mut out);
    homophones::check(doc, &mut out);
    dictionary::check(doc, &mut out);
    pronoun::check(doc, &mut out);
    style::check(doc, &mut out);
    out
}

/// Spell `replacement` with the capitalisation of `original`.
pub(crate) fn like(original: &str, replacement: &str) -> String {
    english::lexicon::match_case(original, replacement)
}

/// Words that can come between a subject and its verb without changing the
/// agreement: "he always goes".
pub(crate) const ADVERBS: &[&str] = &[
    "always",
    "never",
    "often",
    "usually",
    "sometimes",
    "rarely",
    "seldom",
    "really",
    "also",
    "just",
    "still",
    "only",
    "even",
    "actually",
    "probably",
    "definitely",
    "certainly",
    "already",
    "simply",
    "truly",
    "generally",
    "totally",
    "finally",
    "recently",
    "ever",
    "all",
];

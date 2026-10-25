//! Warnings: things that compile but probably should not.

use english::lexicon;

use crate::diagnostic::Diagnostic;
use crate::lexer::{Doc, Kind, Start};

/// Doubled words that are grammatical: "had had", "that that", "what it is
/// is", "there, there".
const DOUBLES: &[&str] = &[
    "had", "that", "is", "do", "bye", "ha", "haha", "no", "yeah", "yes", "there", "knock", "very",
    "so", "far", "really", "now",
];

const ABSOLUTES: &[&str] = &[
    "unique",
    "perfect",
    "infinite",
    "unanimous",
    "impossible",
    "eternal",
    "universal",
    "absolute",
];

const INTENSIFIERS: &[&str] = &[
    "very",
    "more",
    "most",
    "extremely",
    "really",
    "quite",
    "so",
    "totally",
    "somewhat",
    "rather",
    "fairly",
    "slightly",
    "less",
];

/// Negative words that would cancel an earlier negation.
const NEGATIVE: &[(&str, &str)] = &[
    ("nothing", "anything"),
    ("nobody", "anybody"),
    ("none", "any"),
    ("nowhere", "anywhere"),
    ("never", "ever"),
    ("neither", "either"),
    ("no", "any"),
];

/// Words that end the clause a negation applies to.
const CLAUSE_BREAKS: &[&str] = &[
    "and", "but", "or", "because", "if", "when", "which", "who", "that", "while", "so", "than",
];

fn negator(word: &str) -> bool {
    word.ends_with("n't")
        || matches!(
            word,
            "not" | "never" | "cannot" | "nobody" | "nothing" | "none"
        )
}

pub fn check(doc: &Doc, out: &mut Vec<Diagnostic>) {
    repeated_words(doc, out);
    double_negatives(doc, out);
    absolute_adjectives(doc, out);
    literally(doc, out);
    sentence_case(doc, out);
}

fn repeated_words(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, first) in doc.tokens.iter().enumerate() {
        let Some(j) = doc.next_word(i) else { continue };
        if !first.is_word()
            || first.lower != doc.tokens[j].lower
            || DOUBLES.contains(&first.lower.as_str())
        {
            continue;
        }
        let second = &doc.tokens[j];
        if first.lower.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        out.push(
            Diagnostic::warning("repeated_word", format!("repeated word `{}`", first.lower))
                .secondary(first.span.clone(), "first written here")
                .primary(second.span.clone(), "written again here")
                .fix(
                    format!("remove the second `{}`", first.lower),
                    first.span.end..second.span.end,
                    "",
                ),
        );
    }
}

fn double_negatives(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, first) in doc.tokens.iter().enumerate() {
        if !first.is_word() || !negator(&first.lower) {
            continue;
        }
        let mut k = i;
        for _ in 0..4 {
            let Some(next) = doc.next_word(k) else { break };
            k = next;
            let word = doc.lower(k);
            if CLAUSE_BREAKS.contains(&word) {
                break;
            }
            let Some(&(_, positive)) = NEGATIVE.iter().find(|(neg, _)| *neg == word) else {
                continue;
            };
            // Idioms: "say no", "take no for an answer", "no matter what",
            // "never say never", "no one".
            let prev = doc.prev_word(k).map(|p| doc.lower(p));
            let after = doc.next_word(k).map(|n| doc.lower(n));
            if matches!(prev, Some("say" | "said" | "take" | "took"))
                || matches!(after, Some("matter" | "doubt" | "longer"))
            {
                break;
            }
            let second = &doc.tokens[k];
            let (span, positive) = if word == "no" && after == Some("one") {
                (second.span.start..doc.tokens[k + 1].span.end, "anyone")
            } else {
                (second.span.clone(), positive)
            };
            out.push(
                Diagnostic::warning("double_negative", "double negative")
                    .secondary(first.span.clone(), "this negation...")
                    .primary(second.span.clone(), "...is cancelled by this one")
                    .note("two negatives make a positive: `!!x == x`")
                    .fix(
                        format!("use `{positive}`"),
                        span,
                        lexicon::match_case(second.text, positive),
                    ),
            );
            break;
        }
    }
}

fn absolute_adjectives(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, modifier) in doc.tokens.iter().enumerate() {
        if !modifier.is_word() || !INTENSIFIERS.contains(&modifier.lower.as_str()) {
            continue;
        }
        let Some(j) = doc.next_word(i) else { continue };
        let adjective = &doc.tokens[j];
        if !ABSOLUTES.contains(&adjective.lower.as_str()) {
            continue;
        }
        out.push(
            Diagnostic::warning(
                "absolute_adjective",
                format!("`{}` cannot be graded", adjective.lower),
            )
            .primary(modifier.span.clone(), "this has no effect")
            .secondary(adjective.span.clone(), "something either is this or is not")
            .note(format!(
                "`{}` is a `bool`; it does not implement `PartialOrd`",
                adjective.lower
            ))
            .fix(
                format!("remove `{}`", modifier.lower),
                modifier.span.start..adjective.span.start,
                "",
            ),
        );
    }
}

fn literally(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, token) in doc.tokens.iter().enumerate() {
        if !token.is_word() || token.lower != "literally" {
            continue;
        }
        // `_literally` opts out, as in Rust.
        if i > 0 && doc.tokens[i - 1].text == "_" && doc.tokens[i - 1].span.end == token.span.start
        {
            continue;
        }
        let end = doc
            .tokens
            .get(i + 1)
            .filter(|t| t.is_word())
            .map_or(token.span.end, |t| t.span.start);
        out.push(
            Diagnostic::warning("unused_literally", "unused word: `literally`")
                .primary(token.span.clone(), "this word has no effect")
                .help("if this is intentional, prefix it with an underscore: `_literally`")
                .fix("remove it", token.span.start..end, ""),
        );
    }
}

fn sentence_case(doc: &Doc, out: &mut Vec<Diagnostic>) {
    let mut seen = vec![false; doc.starts.len()];
    for (i, token) in doc.tokens.iter().enumerate() {
        if token.kind == Kind::Code {
            seen[token.sentence] = true;
        }
        if !token.is_word() || seen[token.sentence] {
            continue;
        }
        seen[token.sentence] = true;
        if matches!(
            doc.starts[token.sentence],
            Start::Structure | Start::Ellipsis
        ) {
            continue;
        }
        let mut chars = token.text.chars();
        let Some(first) = chars.next() else { continue };
        // "iPhone", "eBay", "e.g.", "npm", and the lower-case `i` lint's turf.
        let unusual = chars
            .clone()
            .any(|c| c.is_uppercase() || !c.is_alphabetic() && c != '\'' && c != '-');
        if !first.is_lowercase() || unusual || token.lower == "i" || token.lower.starts_with("i'") {
            continue;
        }
        // Inside quotes the sentence may continue: `"yes," she said`.
        if doc
            .prev(i)
            .is_some_and(|p| matches!(p.text, "\"" | "\u{201c}" | "'" | "("))
        {
            continue;
        }
        let letter = token.span.start..token.span.start + first.len_utf8();
        out.push(
            Diagnostic::warning(
                "non_capitalized_sentences",
                "sentence does not start with a capital letter",
            )
            .primary(letter.clone(), "lower-case letter")
            .fix(
                "capitalize it",
                letter,
                first.to_uppercase().collect::<String>(),
            ),
        );
    }
}

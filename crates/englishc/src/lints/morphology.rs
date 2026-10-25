//! E0004, E0005, E0006: irregular verbs and nouns.
//!
//! The mistakes are predicted, not listed: for every irregular verb we work
//! out what a learner's `-ed` would produce, and for every irregular noun
//! what `-s` would produce, and flag exactly those spellings.

use std::collections::HashMap;
use std::sync::OnceLock;

use english::lexicon::{self, IRREGULAR_PLURALS, IRREGULAR_VERBS, alternatives_contain, preferred};

use super::{ADVERBS, like};
use crate::diagnostic::Diagnostic;
use crate::lexer::Doc;

/// Regularised spellings that happen to be real words, and so must not be
/// flagged: "seed" (see), "winded" (wind), "lied" (lie, to fib), "putted"
/// (golf), "leaded" (petrol), "bended" (knee).
const REAL_WORDS: &[&str] = &[
    "seed", "winded", "leaded", "bended", "lied", "putted", "leaved", "flied", "shined", "binded",
];

/// Naive plurals that are also verbs ("he mans the desk", "she foots the
/// bill") or accepted plurals ("peoples", "datums").
const REAL_PLURALS: &[&str] = &[
    "mans",
    "foots",
    "louses",
    "knifes",
    "leafs",
    "wolfs",
    "loafs",
    "hypothesises",
    "datums",
    "peoples",
    "bases",
];

/// Dialect pasts that the `-ed` rule does not predict.
const EXTRA_PASTS: &[(&str, &str)] = &[
    ("brang", "bring"),
    ("brung", "bring"),
    ("drownded", "drown"),
];

/// `goed` → `go`.
fn regularized_verbs() -> &'static HashMap<String, &'static str> {
    static MAP: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = HashMap::new();
        for &(base, past, participle) in IRREGULAR_VERBS {
            if base == "be" {
                continue;
            }
            let wrong = lexicon::regular_past(base);
            let accepted =
                alternatives_contain(past, &wrong) || alternatives_contain(participle, &wrong);
            if !accepted && !REAL_WORDS.contains(&wrong.as_str()) {
                map.insert(wrong, base);
            }
        }
        for &(wrong, base) in EXTRA_PASTS {
            map.insert(wrong.to_string(), base);
        }
        map
    })
}

/// `childs` → (`child`, `children`, false); `childrens` → (`child`, `children`, true).
fn regularized_plurals() -> &'static HashMap<String, (&'static str, &'static str, bool)> {
    static MAP: OnceLock<HashMap<String, (&'static str, &'static str, bool)>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = HashMap::new();
        for &(singular, plurals) in IRREGULAR_PLURALS {
            let plural = preferred(plurals);
            let wrong = lexicon::regular_plural(singular);
            if !alternatives_contain(plurals, &wrong) && !REAL_PLURALS.contains(&wrong.as_str()) {
                map.insert(wrong, (singular, plural, false));
            }
            if plural != singular {
                let doubled = lexicon::regular_plural(plural);
                if !alternatives_contain(plurals, &doubled)
                    && !REAL_PLURALS.contains(&doubled.as_str())
                {
                    map.entry(doubled).or_insert((singular, plural, true));
                }
            }
        }
        map
    })
}

const HAVE: &[&str] = &[
    "have",
    "has",
    "had",
    "having",
    "haven't",
    "hasn't",
    "hadn't",
    "i've",
    "you've",
    "we've",
    "they've",
    "would've",
    "could've",
    "should've",
    "might've",
    "must've",
];

/// Pasts that are also common nouns after "have": "I have rose bushes".
const NOUN_PASTS: &[&str] = &["rose", "lay", "bore"];

fn after_have(doc: &Doc, i: usize) -> bool {
    let mut j = i;
    for _ in 0..2 {
        let Some(p) = doc.prev_word(j) else {
            return false;
        };
        let w = doc.lower(p);
        if HAVE.contains(&w) {
            return true;
        }
        // "could of went" is "could have went", which is E0003 and this.
        if w == "of" {
            return doc.prev_word(p).is_some_and(|m| {
                matches!(
                    doc.lower(m),
                    "could" | "should" | "would" | "must" | "might" | "may"
                )
            });
        }
        if !ADVERBS.contains(&w) && w != "not" {
            return false;
        }
        j = p;
    }
    false
}

pub fn check(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, token) in doc.tokens.iter().enumerate() {
        if !token.is_word() {
            continue;
        }
        let word = token.lower.as_str();

        if let Some(&base) = regularized_verbs().get(word) {
            let (_, past, participle) = lexicon::irregular(base).expect("from the table");
            let (form, which) = if after_have(doc, i) {
                (preferred(participle), "past participle")
            } else {
                (preferred(past), "past tense")
            };
            out.push(
                Diagnostic::error(
                    "E0004",
                    "regularized_verb",
                    format!("`{base}` is an irregular verb"),
                )
                .primary(token.span.clone(), format!("`{base}` has no `-ed` form"))
                .note(format!("the {which} of `{base}` is `{form}`"))
                .fix(
                    format!("use `{form}`"),
                    token.span.clone(),
                    like(token.text, form),
                ),
            );
            continue;
        }

        if let Some(&(singular, plural, doubled)) = regularized_plurals().get(word) {
            let d = if doubled {
                Diagnostic::error(
                    "E0006",
                    "regularized_plural",
                    format!("`{plural}` is already plural"),
                )
                .primary(token.span.clone(), "one `-s` too many")
            } else {
                Diagnostic::error(
                    "E0006",
                    "regularized_plural",
                    format!("`{singular}` is an irregular noun"),
                )
                .primary(
                    token.span.clone(),
                    format!("`{singular}` has no `-s` plural"),
                )
                .note(format!("the plural of `{singular}` is `{plural}`"))
            };
            out.push(d.fix(
                format!("use `{plural}`"),
                token.span.clone(),
                like(token.text, plural),
            ));
            continue;
        }

        // "had went": a past tense where the participle belongs.
        if after_have(doc, i)
            && !NOUN_PASTS.contains(&word)
            && let Some(&(base, _, participle)) = IRREGULAR_VERBS
                .iter()
                .find(|(_, past, _)| alternatives_contain(past, word))
            && !alternatives_contain(participle, word)
            && base != "be"
        {
            let want = preferred(participle);
            out.push(
                Diagnostic::error(
                    "E0005",
                    "wrong_verb_form",
                    "expected past participle, found past tense",
                )
                .primary(
                    token.span.clone(),
                    format!("`{word}` is the past tense of `{base}`"),
                )
                .note(format!("after `have`, use the past participle: `{want}`"))
                .fix(
                    format!("use `{want}`"),
                    token.span.clone(),
                    like(token.text, want),
                ),
            );
            continue;
        }

        // "I seen it": a participle with no auxiliary.
        if let Some(p) = doc.prev_word(i) {
            let subject = doc.lower(p);
            let is_subject = matches!(subject, "i" | "he" | "she" | "we" | "they")
                || (matches!(subject, "you" | "it") && doc.starts_sentence(p));
            if is_subject {
                let entry = IRREGULAR_VERBS.iter().find(|(base, past, participle)| {
                    alternatives_contain(participle, word)
                        && !alternatives_contain(past, word)
                        && !lexicon::eq(base, word)
                });
                if let Some(&(base, past, _)) = entry
                    && base != "be"
                {
                    let want = preferred(past);
                    let have = if matches!(subject, "he" | "she" | "it") {
                        "has"
                    } else {
                        "have"
                    };
                    out.push(
                        Diagnostic::error(
                            "E0005",
                            "wrong_verb_form",
                            "expected past tense, found past participle",
                        )
                        .primary(
                            token.span.clone(),
                            format!("`{word}` is the past participle of `{base}`"),
                        )
                        .secondary(
                            doc.tokens[p].span.clone(),
                            format!("a participle needs `{have}` between it and its subject"),
                        )
                        .help(format!(
                            "or keep `{word}` and add `{have}`: `{} {have} {word}`",
                            doc.tokens[p].text
                        ))
                        .fix(
                            format!("use the past tense `{want}`"),
                            token.span.clone(),
                            like(token.text, want),
                        ),
                    );
                }
            }
        }
    }
}

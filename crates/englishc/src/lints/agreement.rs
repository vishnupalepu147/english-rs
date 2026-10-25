//! E0001: subject-verb agreement for pronoun subjects.

use english::lexicon;

use super::{ADVERBS, like};
use crate::diagnostic::Diagnostic;
use crate::lexer::{Doc, Kind};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Agreement {
    FirstSingular,
    Second,
    ThirdSingular,
    Plural,
}

fn agreement(word: &str) -> Option<Agreement> {
    Some(match word {
        "i" => Agreement::FirstSingular,
        "you" => Agreement::Second,
        "he" | "she" | "it" => Agreement::ThirdSingular,
        "we" | "they" => Agreement::Plural,
        _ => return None,
    })
}

/// Words after which a pronoun starts a new clause.
const CLAUSE_STARTERS: &[&str] = &[
    "and", "but", "or", "so", "because", "if", "when", "while", "since", "although", "though",
    "that", "unless", "until", "whether", "where", "after", "before", "once", "then", "yet",
    "whenever",
];

/// Words after which a pronoun is not the subject of the next verb: "does he
/// have", "let it go", "taller than he is".
const LICENSORS: &[&str] = &[
    "do", "does", "did", "can", "could", "will", "would", "shall", "should", "may", "might",
    "must", "let", "lets", "make", "makes", "made", "help", "helps", "helped", "watch", "watched",
    "see", "sees", "saw", "hear", "hears", "heard", "feel", "feels", "felt", "have", "has", "had",
    "than", "as", "to",
];

/// Words that license the subjunctive "were": "if I were you".
const SUBJUNCTIVE: &[&str] = &[
    "if", "wish", "wished", "though", "unless", "suppose", "imagine", "as",
];

/// The form of `verb` that agrees with `agreement`, if it is a verb we know.
fn expected(agreement: Agreement, verb: &str) -> Option<String> {
    use Agreement::*;
    let third = agreement == ThirdSingular;
    let singular_past = matches!(agreement, FirstSingular | ThirdSingular);
    let form = match verb {
        "am" | "is" | "are" => match agreement {
            FirstSingular => "am",
            ThirdSingular => "is",
            _ => "are",
        },
        "isn't" | "aren't" => match agreement {
            FirstSingular => "am not",
            ThirdSingular => "isn't",
            _ => "aren't",
        },
        "was" | "were" if singular_past => "was",
        "was" | "were" => "were",
        "wasn't" | "weren't" if singular_past => "wasn't",
        "wasn't" | "weren't" => "weren't",
        "has" | "have" if third => "has",
        "has" | "have" => "have",
        "hasn't" | "haven't" if third => "hasn't",
        "hasn't" | "haven't" => "haven't",
        "does" | "do" if third => "does",
        "does" | "do" => "do",
        "doesn't" | "don't" if third => "doesn't",
        "doesn't" | "don't" => "don't",
        _ if third => {
            let looks_past = lexicon::IRREGULAR_VERBS
                .iter()
                .any(|(_, past, _)| lexicon::alternatives_contain(past, verb));
            if lexicon::is_verb_base(verb) && !looks_past {
                return Some(lexicon::third_person(verb));
            }
            return None;
        }
        _ => return lexicon::base_of_third_person(verb).map(str::to_string),
    };
    Some(form.to_string())
}

/// The `Takes*` trait from the `english` crate that the found form needs.
fn trait_for(found: &str) -> &'static str {
    match found.trim_end_matches("n't") {
        "am" => "TakesAm",
        "is" => "TakesIs",
        "are" => "TakesAre",
        "was" => "TakesWas",
        "were" => "TakesWere",
        "has" | "does" => "TakesS",
        "have" | "do" => "TakesBase",
        _ if found.ends_with('s') => "TakesS",
        _ => "TakesBase",
    }
}

fn describe(agreement: Agreement, written: &str) -> String {
    match agreement {
        Agreement::FirstSingular => "`I` is first-person singular".to_string(),
        Agreement::Second => format!("`{written}` takes plural agreement, even for one person"),
        Agreement::ThirdSingular => format!("`{written}` is third-person singular"),
        Agreement::Plural => format!("`{written}` is plural"),
    }
}

pub fn check(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, token) in doc.tokens.iter().enumerate() {
        if !token.is_word() {
            continue;
        }
        let Some(agreement) = agreement(&token.lower) else {
            continue;
        };
        let prev = doc.prev(i);
        let prev_word = doc.prev_word(i).map(|p| doc.lower(p));
        if prev_word.is_some_and(|w| LICENSORS.contains(&w) || w.ends_with("n't")) {
            continue;
        }
        // "World War I", "Henry I".
        if token.lower == "i"
            && doc.prev_word(i).is_some_and(|p| {
                !doc.starts_sentence(p) && doc.tokens[p].text.starts_with(char::is_uppercase)
            })
        {
            continue;
        }
        // `you` and `it` are objects as often as subjects, so only trust them
        // at the start of a clause.
        if matches!(token.lower.as_str(), "you" | "it") {
            let boundary = doc.starts_sentence(i)
                || prev.is_some_and(|p| {
                    p.kind == Kind::Punct
                        && matches!(
                            p.text,
                            "," | ";" | ":" | "(" | "\"" | "\u{201c}" | "\u{2014}"
                        )
                })
                || prev_word.is_some_and(|w| CLAUSE_STARTERS.contains(&w));
            if !boundary {
                continue;
            }
        }
        let Some(mut v) = doc.next_word(i) else {
            continue;
        };
        if ADVERBS.contains(&doc.lower(v)) {
            let Some(next) = doc.next_word(v) else {
                continue;
            };
            v = next;
        }
        let verb = &doc.tokens[v];
        // "he need not", "she dare not".
        if matches!(verb.lower.as_str(), "need" | "dare")
            && doc.next_word(v).is_some_and(|n| doc.lower(n) == "not")
        {
            continue;
        }
        let Some(want) = expected(agreement, &verb.lower) else {
            continue;
        };
        if want == verb.lower {
            continue;
        }
        // "he and I were": a compound subject takes the plural, which is what
        // was written.
        let compound = matches!(prev_word, Some("and" | "or" | "nor"))
            && expected(Agreement::Plural, &verb.lower).as_deref() == Some(verb.lower.as_str());
        if compound {
            continue;
        }
        if verb.lower.starts_with("were")
            && agreement != Agreement::Second
            && agreement != Agreement::Plural
        {
            let start = i.saturating_sub(4);
            if doc.tokens[start..i]
                .iter()
                .any(|t| SUBJUNCTIVE.contains(&t.lower.as_str()))
            {
                continue;
            }
        }
        let subject_type = match token.lower.as_str() {
            "i" => "I".to_string(),
            other => lexicon::capitalize(other),
        };
        let replacement = like(verb.text, &want);
        out.push(
            Diagnostic::error("E0001", "subject_verb_agreement", "mismatched verb form")
                .primary(verb.span.clone(), format!("expected `{want}`, found `{}`", verb.lower))
                .secondary(token.span.clone(), describe(agreement, token.text))
                .note(format!(
                    "in English (Rust edition) this is a compile error: the trait bound `{subject_type}: {}` is not satisfied",
                    trait_for(&verb.lower)
                ))
                .fix(format!("use `{want}`"), verb.span.clone(), replacement),
        );
    }
}

/// Object pronouns and their subject forms.
const OBJECTIVE: &[(&str, &str)] = &[
    ("me", "I"),
    ("him", "he"),
    ("her", "she"),
    ("us", "we"),
    ("them", "they"),
];

const POSSESSIVES: &[&str] = &["my", "the", "a", "an", "his", "her", "our", "their", "your"];

fn subjective(word: &str) -> Option<&'static str> {
    OBJECTIVE.iter().find(|(o, _)| *o == word).map(|&(_, s)| s)
}

/// Could this word be the finite verb of a clause?
fn finite_verb(word: &str) -> bool {
    matches!(
        word,
        "am" | "is" | "are" | "was" | "were" | "has" | "have" | "had" | "do" | "does" | "did"
    ) || [
        "can", "could", "will", "would", "should", "might", "must", "may",
    ]
    .contains(&word)
        || word.ends_with("ed")
        || lexicon::base_of_third_person(word).is_some()
        || lexicon::is_verb_base(word)
        || lexicon::IRREGULAR_VERBS
            .iter()
            .any(|(_, past, _)| lexicon::alternatives_contain(past, word))
}

fn clause_start(doc: &Doc, i: usize) -> bool {
    doc.starts_sentence(i)
        || doc
            .prev(i)
            .is_some_and(|p| p.kind == Kind::Punct && matches!(p.text, "," | ";" | ":"))
        || doc
            .prev_word(i)
            .is_some_and(|p| CLAUSE_STARTERS.contains(&doc.lower(p)))
}

/// E0013: "me and my brother went", "my brother and me went".
pub fn check_case(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, token) in doc.tokens.iter().enumerate() {
        if !token.is_word() || !clause_start(doc, i) {
            continue;
        }
        // Parse `A and B` from here: each side is a pronoun, or a noun with an
        // optional determiner.
        let side = |start: usize| -> Option<usize> {
            let word = doc.lower(start);
            if subjective(word).is_some() || agreement(word).is_some() {
                return Some(start);
            }
            if POSSESSIVES.contains(&word) {
                return doc.next_word(start);
            }
            word.chars().all(char::is_alphabetic).then_some(start)
        };
        let Some(left_end) = side(i) else { continue };
        let Some(and) = doc.next_word(left_end).filter(|&a| doc.lower(a) == "and") else {
            continue;
        };
        let Some(right_start) = doc.next_word(and) else {
            continue;
        };
        let Some(right_end) = side(right_start) else {
            continue;
        };
        let Some(verb) = doc.next_word(right_end) else {
            continue;
        };
        if !finite_verb(doc.lower(verb)) {
            continue;
        }

        let left_pronoun = (left_end == i).then(|| doc.lower(i));
        let right_pronoun = (right_end == right_start).then(|| doc.lower(right_start));
        let left_obj = left_pronoun.and_then(subjective);
        let right_obj = right_pronoun.and_then(subjective);
        let (culprit, culprit_word) = match (left_obj, right_obj) {
            (Some(_), _) => (i, doc.lower(i)),
            (None, Some(_)) => (right_start, doc.lower(right_start)),
            _ => continue,
        };

        let text = |a: usize, b: usize| {
            doc.src[doc.tokens[a].span.start..doc.tokens[b].span.end].to_string()
        };
        let fix_side =
            |start: usize, end: usize, pronoun: Option<&str>| match pronoun.and_then(subjective) {
                Some(s) => s.to_string(),
                None => match pronoun.and_then(|p| agreement(p).map(|_| p)) {
                    Some("i") => "I".to_string(),
                    Some(p) => p.to_string(),
                    None => text(start, end).to_lowercase_first(),
                },
            };
        let left = fix_side(i, left_end, left_pronoun);
        let right = fix_side(right_start, right_end, right_pronoun);
        // Politeness is part of the grammar: "my brother and I", not "I and
        // my brother".
        let mut phrase = if left == "I" {
            format!("{right} and I")
        } else {
            format!("{left} and {right}")
        };
        if doc.starts_sentence(i) {
            phrase = lexicon::capitalize(&phrase);
        }
        let type_name = lexicon::capitalize(culprit_word);
        out.push(
            Diagnostic::error(
                "E0013",
                "objective_case_subject",
                format!("`{culprit_word}` cannot be the subject of a sentence"),
            )
            .primary(
                doc.tokens[culprit].span.clone(),
                "this is in the objective case",
            )
            .secondary(doc.tokens[verb].span.clone(), "the subject of this verb")
            .note(format!(
                "in English (Rust edition), the trait bound `{type_name}: Subject` is not satisfied"
            ))
            .fix(
                "use the subjective case",
                token.span.start..doc.tokens[right_end].span.end,
                phrase,
            ),
        );
    }
}

trait LowercaseFirst {
    fn to_lowercase_first(&self) -> String;
}

impl LowercaseFirst for String {
    /// "My brother" becomes "my brother" when it moves out of first place.
    fn to_lowercase_first(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            Some(c) => c.to_lowercase().chain(chars).collect(),
            None => String::new(),
        }
    }
}

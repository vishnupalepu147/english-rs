//! E0003, E0007–E0010: words that sound alike and type-check differently.
//!
//! Each rule looks only at a word and its neighbour, and fires only where the
//! neighbour makes one reading impossible: "its a" can only mean "it is a".

use super::like;
use crate::diagnostic::Diagnostic;
use crate::lexer::Doc;

const MODALS: &[&str] = &[
    "could",
    "should",
    "would",
    "must",
    "might",
    "may",
    "couldn't",
    "shouldn't",
    "wouldn't",
    "mustn't",
];

/// Words that cannot follow a possessive, so a possessive before them must be
/// a contraction: "its a", "your not", "their is".
const NEEDS_VERB: &[&str] = &[
    "a", "an", "the", "not", "been", "going", "gonna", "getting", "so", "too", "really", "always",
    "never", "also", "still", "about", "here", "ok", "okay", "alright",
];

/// More words that cannot follow `its`, but can follow `your` ("your just
/// reward").
const NEEDS_VERB_AFTER_ITS: &[&str] = &["just", "me", "you", "him", "us", "them"];

const BE: &[&str] = &[
    "is", "are", "was", "were", "isn't", "aren't", "wasn't", "weren't",
];

const COMPARATIVES: &[&str] = &[
    "more", "less", "fewer", "better", "worse", "rather", "other", "bigger", "smaller", "larger",
    "greater", "higher", "lower", "faster", "slower", "older", "younger", "longer", "shorter",
    "easier", "harder", "cheaper", "stronger", "weaker", "smarter", "richer", "poorer", "happier",
    "safer", "nicer", "colder", "hotter", "warmer", "cooler", "heavier", "lighter", "darker",
    "brighter", "further", "farther", "taller", "wider", "deeper", "louder", "quieter", "simpler",
];

struct Swap {
    code: &'static str,
    lint: &'static str,
    message: &'static str,
    label: &'static str,
    want: &'static str,
    note: &'static str,
}

const ITS_FOR_ITS: Swap = Swap {
    code: "E0007",
    lint: "its_confusion",
    message: "possessive `its` used where `it's` belongs",
    label: "`its` means \"belonging to it\"",
    want: "it's",
    note: "`it's` is short for \"it is\" or \"it has\"; the possessive has no apostrophe, like `his`",
};

const ITS_FOR_POSSESSIVE: Swap = Swap {
    code: "E0007",
    lint: "its_confusion",
    message: "`it's` used where the possessive `its` belongs",
    label: "`it's` means \"it is\"",
    want: "its",
    note: "the possessive has no apostrophe, like `his` and `hers`",
};

const YOUR_FOR_YOURE: Swap = Swap {
    code: "E0008",
    lint: "your_confusion",
    message: "possessive `your` used where `you're` belongs",
    label: "`your` means \"belonging to you\"",
    want: "you're",
    note: "`you're` is short for \"you are\"",
};

const YOURE_FOR_YOUR: Swap = Swap {
    code: "E0008",
    lint: "your_confusion",
    message: "`you're` used where the possessive `your` belongs",
    label: "`you're` means \"you are\"",
    want: "your",
    note: "the possessive is `your`, with no apostrophe",
};

const THEIR_FOR_THERE: Swap = Swap {
    code: "E0009",
    lint: "there_confusion",
    message: "possessive `their` used where `there` belongs",
    label: "`their` means \"belonging to them\"",
    want: "there",
    note: "\"there is\" and \"there are\" use `there`, the word for a place",
};

const THEYRE_FOR_THERE: Swap = Swap {
    code: "E0009",
    lint: "there_confusion",
    message: "`they're` used where `there` belongs",
    label: "`they're` means \"they are\"",
    want: "there",
    note: "\"there is\" and \"there are\" use `there`, the word for a place",
};

const THEIR_FOR_THEYRE: Swap = Swap {
    code: "E0009",
    lint: "there_confusion",
    message: "possessive `their` used where `they're` belongs",
    label: "`their` means \"belonging to them\"",
    want: "they're",
    note: "`they're` is short for \"they are\"",
};

const THERE_FOR_THEIR: Swap = Swap {
    code: "E0009",
    lint: "there_confusion",
    message: "`there` used where the possessive `their` belongs",
    label: "`there` is a place",
    want: "their",
    note: "the possessive is `their`",
};

const THERE_FOR_THEYRE: Swap = Swap {
    code: "E0009",
    lint: "there_confusion",
    message: "`there` used where `they're` belongs",
    label: "`there` is a place",
    want: "they're",
    note: "`they're` is short for \"they are\"",
};

fn push(doc: &Doc, out: &mut Vec<Diagnostic>, i: usize, j: usize, swap: &Swap, why: String) {
    let token = &doc.tokens[i];
    out.push(
        Diagnostic::error(swap.code, swap.lint, swap.message)
            .primary(token.span.clone(), swap.label)
            .secondary(doc.tokens[j].span.clone(), why)
            .note(swap.note)
            .fix(
                format!("use `{}`", swap.want),
                token.span.clone(),
                like(token.text, swap.want),
            ),
    );
}

pub fn check(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, token) in doc.tokens.iter().enumerate() {
        if !token.is_word() {
            continue;
        }
        let Some(j) = doc.next_word(i) else { continue };
        let next = doc.lower(j);
        let after_next = doc.next_word(j).map(|k| doc.lower(k));
        let prev = doc.prev_word(i).map(|p| doc.lower(p));
        let next_is_last = doc.tokens.get(j + 1).is_none_or(|t| !t.is_word());

        match token.lower.as_str() {
            "of" => {
                if let Some(p) = doc.prev_word(i)
                    && MODALS.contains(&doc.lower(p))
                    && next != "course"
                {
                    let modal = &doc.tokens[p];
                    out.push(
                        Diagnostic::error("E0003", "of_for_have", "expected `have`, found `of`")
                            .primary(token.span.clone(), "`of` is a preposition, not a verb")
                            .secondary(
                                modal.span.clone(),
                                format!("`{}` needs a verb after it", modal.lower),
                            )
                            .note(format!(
                                "`{}'ve` sounds like `{} of`, but the compiler can read",
                                modal.lower, modal.lower
                            ))
                            .fix("use `have`", token.span.clone(), like(token.text, "have")),
                    );
                }
            }
            "its" => {
                let time_to = next == "time" && after_next == Some("to");
                if NEEDS_VERB.contains(&next) || NEEDS_VERB_AFTER_ITS.contains(&next) || time_to {
                    push(
                        doc,
                        out,
                        i,
                        j,
                        &ITS_FOR_ITS,
                        format!("`{next}` needs a verb before it"),
                    );
                }
            }
            "it's" if next == "own" => {
                push(
                    doc,
                    out,
                    i,
                    j,
                    &ITS_FOR_POSSESSIVE,
                    "`own` needs a possessive before it".into(),
                );
            }
            "your" => {
                let welcome = next == "welcome" && next_is_last;
                if NEEDS_VERB.contains(&next) || welcome {
                    push(
                        doc,
                        out,
                        i,
                        j,
                        &YOUR_FOR_YOURE,
                        format!("`{next}` needs a verb before it"),
                    );
                }
            }
            "you're" if next == "own" => {
                push(
                    doc,
                    out,
                    i,
                    j,
                    &YOURE_FOR_YOUR,
                    "`own` needs a possessive before it".into(),
                );
            }
            "their" if next == "gonna" || next == "going" && after_next == Some("to") => {
                push(
                    doc,
                    out,
                    i,
                    j,
                    &THEIR_FOR_THEYRE,
                    format!("`{next}` needs a subject and a verb"),
                );
            }
            "their" if BE.contains(&next) => {
                push(
                    doc,
                    out,
                    i,
                    j,
                    &THEIR_FOR_THERE,
                    format!("`{next}` needs a subject, not a possessive"),
                );
            }
            "they're" if BE.contains(&next) => {
                push(
                    doc,
                    out,
                    i,
                    j,
                    &THEYRE_FOR_THERE,
                    format!("`they're` already contains a verb, so `{next}` is one too many"),
                );
            }
            "there" if next == "own" => {
                push(
                    doc,
                    out,
                    i,
                    j,
                    &THERE_FOR_THEIR,
                    "`own` needs a possessive before it".into(),
                );
            }
            "there" if matches!(next, "going" | "gonna" | "doing" | "getting" | "coming") => {
                let inverted = prev.is_some_and(|p| {
                    BE.contains(&p)
                        || matches!(
                            p,
                            "will" | "would" | "could" | "should" | "might" | "may" | "'s"
                        )
                });
                if !inverted {
                    push(
                        doc,
                        out,
                        i,
                        j,
                        &THERE_FOR_THEYRE,
                        format!("`{next}` needs a subject and a verb"),
                    );
                }
            }
            "then" => {
                let Some(p) = doc.prev_word(i) else { continue };
                let comparative = doc.lower(p);
                if COMPARATIVES.contains(&comparative) {
                    out.push(
                        Diagnostic::error(
                            "E0010",
                            "then_for_than",
                            "expected `than`, found `then`",
                        )
                        .primary(token.span.clone(), "`then` is about time")
                        .secondary(doc.tokens[p].span.clone(), "a comparison needs `than`")
                        .fix(
                            "use `than`",
                            token.span.clone(),
                            like(token.text, "than"),
                        ),
                    );
                }
            }
            _ => {}
        }
    }
}

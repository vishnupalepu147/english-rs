//! `ambiguous_pronoun`: an `it` or `they` that could point at two things.
//!
//! English (C edition) lets a pronoun alias several antecedents and leaves
//! the reader to resolve it at run time. We look for the classic shape: a
//! sentence that opens with a noun phrase and its verb, has a second noun
//! phrase of the same number after the verb, and then starts a clause with
//! `it` or `they`.
//!
//! > The dog chased the ball until it stopped.
//! > The council refused the protesters a permit because they feared violence.
//!
//! Elsewhere, readers resolve pronouns reliably enough that a warning would be
//! noise.

use english::lexicon;

use crate::diagnostic::Diagnostic;
use crate::lexer::{Doc, Kind};

const DETERMINERS: &[&str] = &[
    "the", "a", "an", "this", "that", "my", "your", "his", "her", "our", "their", "every", "each",
    "another",
];

const ADJECTIVES: &[&str] = &[
    "big",
    "small",
    "little",
    "large",
    "old",
    "new",
    "young",
    "good",
    "bad",
    "great",
    "long",
    "short",
    "tall",
    "red",
    "blue",
    "green",
    "black",
    "white",
    "brown",
    "yellow",
    "gray",
    "grey",
    "orange",
    "pink",
    "purple",
    "hot",
    "cold",
    "warm",
    "cool",
    "fast",
    "slow",
    "quick",
    "heavy",
    "light",
    "dark",
    "bright",
    "happy",
    "sad",
    "angry",
    "tired",
    "hungry",
    "empty",
    "full",
    "open",
    "closed",
    "broken",
    "pretty",
    "nice",
    "strange",
    "huge",
    "tiny",
    "whole",
    "entire",
    "same",
    "first",
    "last",
    "next",
    "only",
    "own",
    "best",
    "worst",
    "lazy",
    "brave",
    "clever",
    "loud",
    "quiet",
    "soft",
    "hard",
    "wet",
    "dry",
    "clean",
    "dirty",
    "cheap",
    "expensive",
    "rich",
    "poor",
    "busy",
    "free",
    "real",
    "fake",
    "wooden",
    "metal",
    "glass",
    "plastic",
    "golden",
    "silver",
    "favorite",
    "favourite",
    "local",
    "strong",
    "weak",
    "sparse",
    "simple",
    "complex",
    "random",
    "single",
    "different",
    "certain",
    "particular",
    "specific",
    "main",
    "common",
    "special",
    "final",
    "previous",
    "following",
    "early",
    "exact",
    "basic",
    "current",
    "whole",
    "actual",
    "general",
    "public",
    "private",
    "outer",
    "inner",
];

const ADJECTIVE_SUFFIXES: &[&str] = &["ful", "ous", "ive", "less", "ish", "able", "ible", "est"];

/// Nouns for people, which take "he", "she" or "they" rather than "it".
const PEOPLE: &[&str] = &[
    "man",
    "woman",
    "boy",
    "girl",
    "person",
    "friend",
    "mother",
    "father",
    "mom",
    "dad",
    "mum",
    "brother",
    "sister",
    "son",
    "daughter",
    "teacher",
    "doctor",
    "student",
    "child",
    "kid",
    "baby",
    "wife",
    "husband",
    "king",
    "queen",
    "boss",
    "author",
    "user",
    "customer",
    "player",
    "driver",
    "neighbor",
    "neighbour",
    "lawyer",
    "nurse",
    "chef",
    "manager",
    "owner",
    "engineer",
    "developer",
    "guy",
    "lady",
    "parent",
    "partner",
    "colleague",
    "officer",
    "judge",
    "president",
    "swimmer",
    "runner",
    "writer",
    "reader",
    "singer",
    "dancer",
    "worker",
    "leader",
    "member",
    "winner",
    "speaker",
    "baker",
    "farmer",
    "hunter",
    "painter",
    "stranger",
    "visitor",
    "uncle",
    "aunt",
    "cousin",
    "grandmother",
    "grandfather",
    "girlfriend",
    "boyfriend",
    "programmer",
];

/// Nouns that are almost never what a later "it" means.
const UNLIKELY: &[&str] = &[
    "way", "time", "day", "moment", "end", "rest", "lot", "bit", "kind", "sort", "fact", "while",
    "number", "couple", "most", "other", "one", "week", "year", "morning", "evening", "night",
    "minute", "hour", "idea", "reason", "point", "problem",
];

/// Words that end a noun phrase before it has a head.
const FUNCTION_WORDS: &[&str] = &[
    "of", "to", "in", "on", "at", "for", "with", "by", "from", "and", "or", "but", "is", "are",
    "was", "were", "be", "been", "has", "have", "had", "it", "its", "that", "which", "who", "not",
    "very", "so", "as", "than", "then", "one", "more", "most", "same", "until", "into", "onto",
    "over", "under", "through", "after", "before", "while", "because", "if", "when", "where",
    "about", "around", "behind", "beside", "between", "near", "off", "up", "down", "out", "across",
    "against", "along", "among", "without", "within", "during", "toward", "towards", "upon",
    "like", "whose", "whom", "what", "how", "why", "will", "would", "can", "could", "should",
    "may", "might", "must", "shall", "did", "does", "do", "this", "these", "those", "there",
    "here", "all", "both", "each", "every", "some", "any", "no", "such", "only", "also", "just",
    "still", "even", "too", "again", "i", "you", "he", "she", "we", "they", "me", "him", "us",
    "them", "instead", "bit", "the", "a", "an", "my", "your", "his", "her", "our", "their",
];

/// Verbs after which a subject `it` is a dummy: "it seems", "it rained".
const DUMMY_VERBS: &[&str] = &[
    "seems", "seemed", "appears", "appeared", "rains", "rained", "snows", "snowed", "happens",
    "happened", "depends", "depended", "matters", "mattered", "follows", "turns", "turned",
    "takes", "took", "looks", "looked", "sounds", "sounded", "feels", "felt", "occurred", "means",
];

/// Words that, soon after "it is", mark a dummy subject: "it is likely that",
/// "it is raining", "it was time to".
const DUMMY_MARKERS: &[&str] = &[
    "to",
    "that",
    "whether",
    "raining",
    "snowing",
    "sunny",
    "windy",
    "cloudy",
    "o'clock",
    "late",
    "early",
    "time",
    "likely",
    "clear",
    "obvious",
    "true",
    "possible",
    "important",
    "necessary",
    "worth",
    "okay",
    "ok",
    "fine",
    "unclear",
    "impossible",
    "hard",
    "easy",
];

const CLAUSE_STARTERS: &[&str] = &[
    "and", "but", "or", "so", "because", "if", "when", "while", "since", "although", "though",
    "until", "unless", "after", "before", "once", "then", "whereas",
];

struct Candidate {
    token: usize,
    word: String,
}

fn adjective(word: &str) -> bool {
    ADJECTIVES.contains(&word)
        || (word.len() > 5 && ADJECTIVE_SUFFIXES.iter().any(|s| word.ends_with(s)))
}

/// Does this word look like the verb after a subject ("the dog barks", "the
/// ball rolled", "the trophy doesn't")?
fn verb_like(word: &str) -> bool {
    word.ends_with("ed")
        || word.ends_with("n't")
        || (word.ends_with('s') && !word.ends_with("ss"))
        || [
            "is", "was", "has", "had", "can", "could", "will", "would", "did", "does", "must",
            "might",
        ]
        .contains(&word)
        || lexicon::is_verb_base(word)
        || lexicon::IRREGULAR_VERBS
            .iter()
            .any(|(_, past, _)| lexicon::alternatives_contain(past, word))
}

/// The noun phrase that starts at determiner `k`, if it has a plausible head.
fn noun_phrase(doc: &Doc, k: usize, end: usize, plural: bool) -> Option<Candidate> {
    if !DETERMINERS.contains(&doc.lower(k)) {
        return None;
    }
    let mut h = k;
    for _ in 0..4 {
        h = doc.next_word(h).filter(|&n| n < end)?;
        if !adjective(doc.lower(h)) {
            break;
        }
    }
    let word = doc.lower(h).trim_end_matches("'s").to_string();
    let plausible = word.chars().all(char::is_alphabetic)
        && word.len() > 1
        && !FUNCTION_WORDS.contains(&word.as_str())
        && !UNLIKELY.contains(&word.as_str())
        && !word.ends_with("ing")
        && lexicon::is_plural(&word) == plural
        // People are "he", "she" or "they", never "it".
        && (plural || !PEOPLE.contains(&word.as_str()));
    plausible.then_some(Candidate { token: h, word })
}

fn is_dummy(doc: &Doc, i: usize) -> bool {
    let Some(j) = doc.next_word(i) else {
        return false;
    };
    let next = doc.lower(j);
    if DUMMY_VERBS.contains(&next) {
        return true;
    }
    let be_follows = doc.lower(i) == "it's"
        || matches!(next, "is" | "was" | "will" | "would" | "isn't" | "wasn't");
    if !be_follows {
        return false;
    }
    let sentence = doc.tokens[i].sentence;
    doc.tokens[i + 1..]
        .iter()
        .take_while(|t| t.sentence == sentence)
        .filter(|t| t.kind == Kind::Word)
        .take(4)
        .any(|t| DUMMY_MARKERS.contains(&t.lower.as_str()))
}

pub fn check(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, token) in doc.tokens.iter().enumerate() {
        let plural = match token.lower.as_str() {
            "it" | "it's" => false,
            "they" | "they're" => true,
            _ => continue,
        };
        if !token.is_word()
            || !doc
                .prev_word(i)
                .is_some_and(|p| CLAUSE_STARTERS.contains(&doc.lower(p)))
        {
            continue;
        }
        if !plural && is_dummy(doc, i) {
            continue;
        }
        // The subject: a noun phrase that opens the sentence, then its verb.
        let sentence = token.sentence;
        let first = doc
            .tokens
            .iter()
            .position(|t| t.sentence == sentence && t.is_word())
            .unwrap_or(i);
        let Some(subject) = noun_phrase(doc, first, i, plural) else {
            continue;
        };
        let Some(verb) = doc
            .next_word(subject.token)
            .filter(|&v| v < i && verb_like(doc.lower(v)))
        else {
            continue;
        };
        // The object: the last matching noun phrase between that verb and the
        // pronoun's clause.
        let object = (verb + 1..i - 1)
            .rev()
            .find_map(|k| noun_phrase(doc, k, i, plural).filter(|c| c.word != subject.word));
        let Some(object) = object else { continue };

        let pronoun = token.lower.as_str();
        let replacement = |c: &Candidate| {
            let phrase = format!("the {}", c.word);
            match pronoun {
                "it's" | "they're" => format!("{phrase} {}", if plural { "are" } else { "is" }),
                _ => phrase,
            }
        };
        out.push(
            Diagnostic::warning(
                "ambiguous_pronoun",
                format!("ambiguous pronoun: `{pronoun}` could refer to `{}` or `{}`", subject.word, object.word),
            )
            .secondary(doc.tokens[subject.token].span.clone(), format!("could be the `{}`", subject.word))
            .secondary(doc.tokens[object.token].span.clone(), format!("could be the `{}`", object.word))
            .primary(token.span.clone(), format!("`{pronoun}` is used here"))
            .note(format!(
                "in English (Rust edition), a pronoun borrows exactly one noun: `let {} = {}.pronoun();`",
                if plural { "they" } else { "it" },
                object.word
            ))
            .suggest(format!("if you meant the `{}`, say so", object.word), token.span.clone(), replacement(&object))
            .suggest(format!("if you meant the `{}`, say so", subject.word), token.span.clone(), replacement(&subject)),
        );
    }
}

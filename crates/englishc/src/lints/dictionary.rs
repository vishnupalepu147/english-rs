//! E0011 and E0012: words that are not words, and a lower-case `i`.

use super::like;
use crate::diagnostic::Diagnostic;
use crate::lexer::Doc;

/// Misspellings common enough to have their own correction.
const CORRECTIONS: &[(&str, &str)] = &[
    ("irregardless", "regardless"),
    ("alot", "a lot"),
    ("definately", "definitely"),
    ("definatly", "definitely"),
    ("seperate", "separate"),
    ("seperately", "separately"),
    ("recieve", "receive"),
    ("recieved", "received"),
    ("occured", "occurred"),
    ("occurence", "occurrence"),
    ("untill", "until"),
    ("wich", "which"),
    ("beleive", "believe"),
    ("noone", "no one"),
    ("thier", "their"),
    ("truely", "truly"),
    ("goverment", "government"),
    ("arguement", "argument"),
    ("begining", "beginning"),
    ("wierd", "weird"),
    ("acheive", "achieve"),
    ("accomodate", "accommodate"),
    ("publically", "publicly"),
    ("tommorow", "tomorrow"),
    ("tommorrow", "tomorrow"),
    ("neccessary", "necessary"),
    ("necessery", "necessary"),
    ("existance", "existence"),
    ("persistant", "persistent"),
    ("independant", "independent"),
    ("embarass", "embarrass"),
    ("concious", "conscious"),
    ("enviroment", "environment"),
    ("grammer", "grammar"),
    ("rythm", "rhythm"),
    ("supercede", "supersede"),
    ("pronounciation", "pronunciation"),
    ("mispell", "misspell"),
    ("calender", "calendar"),
    ("buisness", "business"),
    ("foriegn", "foreign"),
    ("freind", "friend"),
    ("happend", "happened"),
    ("knowlege", "knowledge"),
    ("libary", "library"),
    ("posession", "possession"),
    ("prefered", "preferred"),
    ("refered", "referred"),
    ("succesful", "successful"),
    ("sucessful", "successful"),
    ("tounge", "tongue"),
    ("vaccum", "vacuum"),
    ("wether", "whether"),
    ("dont", "don't"),
    ("doesnt", "doesn't"),
    ("didnt", "didn't"),
    ("isnt", "isn't"),
    ("wasnt", "wasn't"),
    ("arent", "aren't"),
    ("werent", "weren't"),
    ("couldnt", "couldn't"),
    ("wouldnt", "wouldn't"),
    ("shouldnt", "shouldn't"),
    ("hasnt", "hasn't"),
    ("havent", "haven't"),
    ("youre", "you're"),
    ("theyre", "they're"),
    ("thats", "that's"),
    ("whats", "what's"),
    ("shes", "she's"),
    ("youve", "you've"),
    ("theyve", "they've"),
    ("youll", "you'll"),
    ("theyll", "they'll"),
    ("couldve", "could've"),
    ("wouldve", "would've"),
    ("shouldve", "should've"),
    ("im", "I'm"),
    ("ive", "I've"),
];

/// Real words that are usually a missing apostrophe, but only after a
/// pronoun: "I cant" is "I can't", while "political cant" is fine.
const AFTER_PRONOUN: &[(&str, &str)] = &[("cant", "can't"), ("wont", "won't")];

pub fn check(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, token) in doc.tokens.iter().enumerate() {
        if !token.is_word() {
            continue;
        }
        let word = token.lower.as_str();

        let after_pronoun = doc.prev_word(i).is_some_and(|p| {
            matches!(
                doc.lower(p),
                "i" | "you" | "he" | "she" | "it" | "we" | "they"
            )
        });
        let correction = CORRECTIONS
            .iter()
            .chain(AFTER_PRONOUN.iter().filter(|_| after_pronoun))
            .find(|(wrong, _)| *wrong == word)
            .map(|&(_, right)| right);
        if let Some(right) = correction {
            let replacement = if right.starts_with("I'") {
                right.to_string()
            } else {
                like(token.text, right)
            };
            out.push(
                Diagnostic::error(
                    "E0011",
                    "unknown_word",
                    format!("cannot find word `{}` in this dictionary", token.text),
                )
                .primary(token.span.clone(), "not found in this dictionary")
                .fix(
                    format!("a word with a similar spelling exists: `{right}`"),
                    token.span.clone(),
                    replacement,
                ),
            );
            continue;
        }

        // A lower-case `i`, but not a list numeral like "(i)".
        let is_i = word == "i" || ["i'm", "i've", "i'll", "i'd"].contains(&word);
        if is_i && token.text.starts_with('i') {
            let numeral = doc.tokens.get(i + 1).is_some_and(|t| t.text == ")");
            if !numeral {
                let fixed = format!("I{}", &token.text[1..]);
                out.push(
                    Diagnostic::error("E0012", "lowercase_i", "the pronoun `I` is always capitalized")
                        .primary(token.span.clone(), "lower-case `i`")
                        .note("this is a stable feature of English and will not be removed in a future edition")
                        .fix("capitalize it", token.span.clone(), fixed),
                );
            }
        }
    }
}

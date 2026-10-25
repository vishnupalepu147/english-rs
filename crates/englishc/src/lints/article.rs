//! E0002: `a` versus `an`, decided by sound rather than spelling.

use english::lexicon;

use super::like;
use crate::diagnostic::Diagnostic;
use crate::lexer::{Doc, Kind};

fn shouting(word: &str) -> bool {
    word.len() > 1 && word.chars().all(|c| !c.is_lowercase())
}

pub fn check(doc: &Doc, out: &mut Vec<Diagnostic>) {
    for (i, article) in doc.tokens.iter().enumerate() {
        if !article.is_word() || !matches!(article.lower.as_str(), "a" | "an") {
            continue;
        }
        // "Plan A", "vitamin A": a capital letter mid-sentence is a name.
        if article.text == "A" && !doc.starts_sentence(i) || article.text == "AN" {
            continue;
        }
        // Look past opening quotes and brackets: an "apple".
        let mut j = i + 1;
        while doc.tokens.get(j).is_some_and(|t| {
            t.kind == Kind::Punct
                && matches!(
                    t.text,
                    "\"" | "\u{201c}" | "'" | "\u{2018}" | "(" | "*" | "_"
                )
        }) {
            j += 1;
        }
        let Some(noun) = doc.word(j) else { continue };
        if noun.sentence != article.sentence {
            continue;
        }
        // ALL CAPS TEXT is not a run of initialisms.
        if shouting(noun.text) && doc.word(j + 1).is_some_and(|t| shouting(t.text)) {
            continue;
        }
        let want = lexicon::indefinite_article(noun.text);
        if want == article.lower {
            continue;
        }
        let sound = if want == "an" {
            "a vowel sound"
        } else {
            "a consonant sound"
        };
        let letter_misleads = noun.lower.starts_with(['a', 'e', 'i', 'o', 'u']) != (want == "an");
        let mut d = Diagnostic::error("E0002", "indefinite_article", "wrong indefinite article")
            .primary(
                article.span.clone(),
                format!("expected `{want}`, found `{}`", article.lower),
            )
            .secondary(
                noun.span.clone(),
                format!("`{}` starts with {sound}", noun.text),
            );
        if letter_misleads {
            d = d.note("the article follows the sound, not the letter: an hour, a unicorn, an MBA, a one-off");
        }
        out.push(d.fix(
            format!("use `{want}`"),
            article.span.clone(),
            like(article.text, want),
        ));
    }
}

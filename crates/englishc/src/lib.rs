//! `englishc`, the English compiler.
//!
//! English (Rust edition) checks agreement in the type system, but there are
//! about fifteen centuries of English (C edition) in production. `englishc`
//! is the migration tool: it reads ordinary prose and reports what the new
//! edition would reject, in rustc's diagnostic format, with machine-applicable
//! fixes where the right answer is unambiguous.
//!
//! ```
//! let diagnostics = englishc::check("He are late.", &englishc::Config::default());
//! assert_eq!(diagnostics[0].code, Some("E0001"));
//!
//! let (fixed, count) = englishc::fix("She have a apple.", &englishc::Config::default());
//! assert_eq!(fixed, "She has an apple.");
//! assert_eq!(count, 2);
//! ```

pub mod diagnostic;
pub mod explain;
pub mod lexer;
pub mod lints;
pub mod render;
#[cfg(target_arch = "wasm32")]
mod web;

use std::collections::HashSet;

pub use diagnostic::{Diagnostic, Level};
use lexer::Doc;

/// Lint levels from the command line. Entries are lint names, error codes,
/// or `warnings` for every warning.
#[derive(Clone, Debug, Default)]
pub struct Config {
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

impl Config {
    fn names(d: &Diagnostic) -> [&str; 3] {
        let group = if d.level == Level::Warning {
            "warnings"
        } else {
            ""
        };
        [d.lint, d.code.unwrap_or(""), group]
    }

    fn allows(&self, d: &Diagnostic) -> bool {
        Self::names(d)
            .iter()
            .any(|n| !n.is_empty() && self.allow.iter().any(|a| a == n))
    }

    fn denies<'d>(&self, d: &'d Diagnostic) -> Option<&'d str> {
        Self::names(d)
            .into_iter()
            .find(|n| !n.is_empty() && self.deny.iter().any(|a| a == n))
    }
}

/// Every diagnostic for `src`, in source order.
pub fn check(src: &str, config: &Config) -> Vec<Diagnostic> {
    let doc = Doc::parse(src);
    let mut diagnostics = lints::run(&doc);
    diagnostics.retain(|d| {
        let start = d.span().start;
        let in_source = doc.allows.iter().any(|a| {
            (a.lint == d.lint
                || Some(a.lint.as_str()) == d.code
                || (a.lint == "warnings" && d.level == Level::Warning))
                && a.range.as_ref().is_none_or(|r| r.contains(&start))
        });
        !in_source && !config.allows(d)
    });
    diagnostics.sort_by_key(|d| (d.span().start, d.level));

    let mut seen = HashSet::new();
    for d in &mut diagnostics {
        if d.level != Level::Warning {
            continue;
        }
        if let Some(flag) = config.denies(d).map(str::to_string) {
            d.level = Level::Error;
            d.notes
                .insert(0, format!("requested on the command line with `-D {flag}`"));
        } else if seen.insert(d.lint) {
            d.notes
                .insert(0, format!("`#[warn({})]` on by default", d.lint));
        }
    }
    diagnostics
}

/// Apply every machine-applicable suggestion, repeating until nothing
/// changes. Returns the new text and how many fixes went in.
pub fn fix(src: &str, config: &Config) -> (String, usize) {
    let mut text = src.to_string();
    let mut total = 0;
    for _ in 0..4 {
        let diagnostics = check(&text, config);
        let mut fixes: Vec<_> = diagnostics
            .iter()
            .flat_map(Diagnostic::fixes)
            .filter(|s| text[s.span.clone()] != s.replacement)
            .collect();
        fixes.sort_by_key(|s| (s.span.start, s.span.end));
        let mut chosen = Vec::new();
        let mut end = 0;
        for s in fixes {
            if s.span.start >= end
                && !(chosen
                    .last()
                    .is_some_and(|c: &&diagnostic::Suggestion| c.span == s.span))
            {
                end = s.span.end;
                chosen.push(s);
            }
        }
        if chosen.is_empty() {
            break;
        }
        for s in chosen.iter().rev() {
            text.replace_range(s.span.clone(), &s.replacement);
        }
        total += chosen.len();
    }
    (text, total)
}

/// How many suggestions `--fix` would apply in one pass.
pub fn fixable(diagnostics: &[Diagnostic]) -> usize {
    diagnostics
        .iter()
        .filter(|d| d.fixes().next().is_some())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codes(src: &str) -> Vec<&'static str> {
        check(src, &Config::default())
            .iter()
            .map(|d| d.code.unwrap_or(d.lint))
            .collect()
    }

    fn fixed(src: &str) -> String {
        fix(src, &Config::default()).0
    }

    #[test]
    fn agreement() {
        assert_eq!(
            codes("He are late. They is here. I has two. She don't care."),
            ["E0001"; 4]
        );
        assert_eq!(
            fixed("He are late. They is here. I has two. She don't care."),
            "He is late. They are here. I have two. She doesn't care."
        );
        assert_eq!(
            fixed("He always go home. They runs fast."),
            "He always goes home. They run fast."
        );
        assert!(
            codes("Does he have a car? Let it go. If I were you, I would stay. Make it work.")
                .is_empty()
        );
        assert!(codes("World War I is over. He put it down. He need not worry.").is_empty());
    }

    #[test]
    fn articles() {
        assert_eq!(
            fixed("A apple, an banana, a hour, an unicorn, a MBA and a owl."),
            "An apple, a banana, an hour, a unicorn, an MBA and an owl."
        );
        assert!(codes("An hour, a university, a European, an X-ray, plan A is fine.").is_empty());
    }

    #[test]
    fn morphology() {
        assert_eq!(
            fixed("I goed home and we runned. The childs have teeths."),
            "I went home and we ran. The children have teeth."
        );
        assert_eq!(
            fixed("She had went. They have ate. I seen it. He done it."),
            "She had gone. They have eaten. I saw it. He did it."
        );
        assert_eq!(
            fixed("Two childrens and their mouses."),
            "Two children and their mice."
        );
        assert!(
            codes("The seed was planted. He was winded. She lied. I have seen it. He has done it.")
                .is_empty()
        );
    }

    #[test]
    fn homophones() {
        assert_eq!(
            fixed(
                "Its a trap. Your welcome. Their is a cat. There going home. It's own way. Bigger then me. I could of won."
            ),
            "It's a trap. You're welcome. There is a cat. They're going home. Its own way. Bigger than me. I could have won."
        );
        assert_eq!(
            fixed("Their going to the park. I could of went."),
            "They're going to the park. I could have gone."
        );
        assert!(codes("Its tail wagged. Your welcome speech was great. Is there going to be cake? Then we left. Of course you could, of course.").is_empty());
    }

    #[test]
    fn objective_case() {
        assert_eq!(
            fixed("Me and my brother went home."),
            "My brother and I went home."
        );
        assert_eq!(
            fixed("Yesterday, him and me were late."),
            "Yesterday, he and I were late."
        );
        assert_eq!(
            fixed("My sister and me play chess."),
            "My sister and I play chess."
        );
        assert!(codes("Between you and me, it was fine. Tell my brother and me.").is_empty());
    }

    #[test]
    fn dictionary_and_case() {
        assert_eq!(
            fixed("Irregardless, i dont think alot of it."),
            "Regardless, I don't think a lot of it."
        );
    }

    #[test]
    fn warnings() {
        assert_eq!(fixed("I don't have no money."), "I don't have any money.");
        assert_eq!(
            fixed("It was the the best and very unique."),
            "It was the best and unique."
        );
        assert_eq!(fixed("I literally died. the end."), "I died. The end.");
        assert!(codes("Had had. That that is. I can't say no. No matter what.").is_empty());
        assert_eq!(
            codes("The dog chased the ball until it stopped."),
            ["ambiguous_pronoun"]
        );
        assert_eq!(
            codes("The trophy doesn't fit in the suitcase because it is too big."),
            ["ambiguous_pronoun"]
        );
        assert_eq!(
            codes(
                "The councilmen refused the demonstrators a permit because they feared violence."
            ),
            ["ambiguous_pronoun"]
        );
        assert!(
            codes("The dog chased the ball. It was fast. The dog wagged its tail when it saw me.")
                .is_empty()
        );
        assert!(
            codes("I bought a car. It is red. It seems fine. It is likely that it rains.")
                .is_empty()
        );
    }

    #[test]
    fn allow_attributes() {
        assert!(codes("#![allow(double_negative)]\nI can't get no satisfaction.").is_empty());
        assert_eq!(codes("#[allow(E0002)]\nA apple.\nA apple."), ["E0002"]);
        let config = Config {
            allow: vec!["warnings".into()],
            deny: vec![],
        };
        assert!(check("the the", &config).is_empty());
        let config = Config {
            allow: vec![],
            deny: vec!["repeated_word".into()],
        };
        assert_eq!(check("The the end.", &config)[0].level, Level::Error);
    }

    #[test]
    fn code_is_not_prose() {
        assert!(
            codes("Run `he are` or see https://a.example/he-are.\n\n```\nI is\n```\n").is_empty()
        );
    }
}

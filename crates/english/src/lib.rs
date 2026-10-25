//! # English, rewritten in Rust
//!
//! English (C edition) has served for about fifteen centuries, but it is
//! unsafe in the ways old systems languages are. Subjects and verbs are not
//! type-checked, so "he don't" ships to production. Pronouns are raw pointers
//! that can dangle or alias. Plurals and past tenses are a mess of legacy
//! special cases that every speaker has to memorise. This crate is the rewrite.
//!
//! ## Agreement is a trait bound
//!
//! The [`english!`] macro takes sentences as Rust tokens, classifies each verb
//! at compile time and requires the subject to implement the matching
//! `Takes*` trait. A sentence that type-checks is rendered back into text.
//!
//! ```
//! use english::english;
//!
//! let text = english!(The dog barks. I am happy! These mice are small. She doesn 't care.);
//! assert_eq!(text.to_string(), "The dog barks. I am happy! These mice are small. She doesn't care.");
//! ```
//!
//! Getting it wrong is a compile error, with a message that says why:
//!
//! ```compile_fail,E0277
//! english::english!(He don 't care); // `He` cannot take the base form of `do`
//! ```
//!
//! ```compile_fail,E0277
//! english::english!(I is happy); // subject-verb disagreement: `I` cannot take `is`
//! ```
//!
//! ```compile_fail,E0277
//! english::english!(Me am happy); // `Me` cannot be the subject of a sentence
//! ```
//!
//! ```compile_fail,E0277
//! english::english!(A dogs barks); // `A` cannot come before a plural noun
//! ```
//!
//! ```compile_fail,E0277
//! english::english!(A apple is red); // `A` cannot come before a vowel sound; write `An`
//! ```
//!
//! ## Contractions are lifetimes
//!
//! Since Rust 2021, `don't` is a reserved prefix, and rustc will suggest that
//! you write `don 't` instead. Inside `english!`, `'t`, `'s`, `'re`, `'m`, `'ve`,
//! `'ll` and `'d` are lifetimes that attach to the word before them when the
//! sentence is rendered.
//!
//! ## Pronouns are borrows
//!
//! A [`Pronoun`] holds a shared reference to its antecedent, so a pronoun
//! cannot outlive the thing it refers to, and that thing cannot be moved out
//! of the story while the pronoun is in use. See [`Pronoun`] for the
//! `compile_fail` example. Ambiguous pronouns cannot be written at all,
//! because a pronoun borrows exactly one noun.
//!
//! ## Inflection is computed, never written
//!
//! Outside the macro you write dictionary forms and let the crate inflect
//! them, so overregularised forms like "goed" and "mouses" are unrepresentable.
//!
//! ```
//! use english::prelude::*;
//!
//! assert_eq!(She.verb("go").then("home").to_string(), "She goes home.");
//! assert_eq!(They.past("go").then("home").to_string(), "They went home.");
//! assert_eq!(It.perfect("begin").to_string(), "It has begun.");
//! assert_eq!(I.be("ready").to_string(), "I am ready.");
//! assert_eq!(Noun::new("child").plural().the().to_string(), "the children");
//! ```
//!
//! ## What the macro understands
//!
//! Each sentence is a subject, a finite verb, and anything at all after it.
//! The subject is a personal pronoun (`I`, `You`, `He`, `She`, `It`, `We`,
//! `They`), a demonstrative (`This`, `That`, `These`, `Those`, `There`), a
//! determiner followed by one noun (`The dog`, `An hour`, `These mice`), or a
//! variable holding a [`Noun`], a [`Person`] or a [`Pronoun`]. Sentences end
//! at `.`, `!` or `?`. Everything after the verb is passed through unchecked,
//! which is also how English (C edition) handles it.

pub mod det;
pub mod grammar;
pub mod lexicon;
pub mod noun;
pub mod pronoun;
pub mod sentence;

pub use noun::{Noun, Person};
pub use pronoun::Pronoun;
pub use sentence::{Sentence, Text};

/// Everything you need to write English in Rust.
pub mod prelude {
    pub use crate::english;
    pub use crate::grammar::{Plural, Singular, Subject};
    pub use crate::noun::{Noun, Person};
    pub use crate::pronoun::{He, Her, Him, I, It, Me, Pronoun, She, Them, They, Us, We, You};
}

/// Write English sentences as Rust tokens, checked at compile time.
///
/// Returns a [`Text`], which displays as ordinary prose. See the crate
/// documentation for the grammar it accepts.
#[macro_export]
macro_rules! english {
    ($($tokens:tt)+) => {
        $crate::__munch!([] [] $($tokens)+)
    };
}

/// Splits the input into sentences at `.`, `!` and `?`.
#[doc(hidden)]
#[macro_export]
macro_rules! __munch {
    ([$($done:expr,)*] [$($current:tt)+] . $($rest:tt)*) => {
        $crate::__munch!([$($done,)* $crate::__sentence!('.'; $($current)+),] [] $($rest)*)
    };
    ([$($done:expr,)*] [$($current:tt)+] ! $($rest:tt)*) => {
        $crate::__munch!([$($done,)* $crate::__sentence!('!'; $($current)+),] [] $($rest)*)
    };
    ([$($done:expr,)*] [$($current:tt)+] ? $($rest:tt)*) => {
        $crate::__munch!([$($done,)* $crate::__sentence!('?'; $($current)+),] [] $($rest)*)
    };
    ([$($done:expr,)*] [$($current:tt)*] $token:tt $($rest:tt)*) => {
        $crate::__munch!([$($done,)*] [$($current)* $token] $($rest)*)
    };
    ([$($done:expr,)*] [$($current:tt)+]) => {
        $crate::__munch!([$($done,)* $crate::__sentence!('.'; $($current)+),] [])
    };
    ([$($done:expr,)*] []) => {
        $crate::Text(::std::vec![$($done),*])
    };
}

/// Finds the subject of one sentence.
#[doc(hidden)]
#[macro_export]
macro_rules! __sentence {
    // Personal pronouns, which need no imports.
    ($end:literal; I $($rest:tt)*) => { $crate::__pronoun!($end; I; $($rest)*) };
    ($end:literal; You $($rest:tt)*) => { $crate::__pronoun!($end; You; $($rest)*) };
    ($end:literal; He $($rest:tt)*) => { $crate::__pronoun!($end; He; $($rest)*) };
    ($end:literal; She $($rest:tt)*) => { $crate::__pronoun!($end; She; $($rest)*) };
    ($end:literal; It $($rest:tt)*) => { $crate::__pronoun!($end; It; $($rest)*) };
    ($end:literal; We $($rest:tt)*) => { $crate::__pronoun!($end; We; $($rest)*) };
    ($end:literal; They $($rest:tt)*) => { $crate::__pronoun!($end; They; $($rest)*) };
    ($end:literal; Me $($rest:tt)*) => { $crate::__pronoun!($end; Me; $($rest)*) };
    ($end:literal; Him $($rest:tt)*) => { $crate::__pronoun!($end; Him; $($rest)*) };
    ($end:literal; Us $($rest:tt)*) => { $crate::__pronoun!($end; Us; $($rest)*) };
    ($end:literal; Them $($rest:tt)*) => { $crate::__pronoun!($end; Them; $($rest)*) };
    ($end:literal; There $($rest:tt)*) => { $crate::__pronoun!($end; demonstrative::There; $($rest)*) };
    // Words that are a pronoun before a verb and a determiner before a noun.
    ($end:literal; This $($rest:tt)*) => { $crate::__det_or_pronoun!($end; This demonstrative::This; $($rest)*) };
    ($end:literal; That $($rest:tt)*) => { $crate::__det_or_pronoun!($end; That demonstrative::That; $($rest)*) };
    ($end:literal; These $($rest:tt)*) => { $crate::__det_or_pronoun!($end; These demonstrative::These; $($rest)*) };
    ($end:literal; Those $($rest:tt)*) => { $crate::__det_or_pronoun!($end; Those demonstrative::Those; $($rest)*) };
    ($end:literal; Her $($rest:tt)*) => { $crate::__det_or_pronoun!($end; Her Her; $($rest)*) };
    // Determiners.
    ($end:literal; The $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; The $noun; $($rest)*) };
    ($end:literal; A $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; A $noun; $($rest)*) };
    ($end:literal; An $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; An $noun; $($rest)*) };
    ($end:literal; My $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; My $noun; $($rest)*) };
    ($end:literal; Your $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Your $noun; $($rest)*) };
    ($end:literal; His $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; His $noun; $($rest)*) };
    ($end:literal; Its $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Its $noun; $($rest)*) };
    ($end:literal; Our $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Our $noun; $($rest)*) };
    ($end:literal; Their $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Their $noun; $($rest)*) };
    ($end:literal; Every $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Every $noun; $($rest)*) };
    ($end:literal; Each $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Each $noun; $($rest)*) };
    ($end:literal; Some $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Some $noun; $($rest)*) };
    ($end:literal; No $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; No $noun; $($rest)*) };
    ($end:literal; Many $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Many $noun; $($rest)*) };
    ($end:literal; Several $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Several $noun; $($rest)*) };
    ($end:literal; Both $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Both $noun; $($rest)*) };
    ($end:literal; Few $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; Few $noun; $($rest)*) };
    ($end:literal; All $noun:ident $($rest:tt)*) => { $crate::__noun_phrase!($end; All $noun; $($rest)*) };
    // Anything else is a variable in scope: a `Noun`, a `Person` or a `Pronoun`.
    ($end:literal; $subject:ident $($rest:tt)*) => {{
        let subject = $crate::grammar::subject(&$subject);
        $crate::__predicate!($end; subject; $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __pronoun {
    ($end:literal; $($pronoun:ident)::+; $($rest:tt)*) => {{
        let subject = $crate::grammar::subject(&$crate::pronoun::$($pronoun)::+);
        $crate::__predicate!($end; subject; $($rest)*)
    }};
}

/// `This is` versus `This dog is`: decided by whether a verb comes next.
#[doc(hidden)]
#[macro_export]
macro_rules! __det_or_pronoun {
    (@am $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@is $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@are $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@was $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@were $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@has $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@have $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@does $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@isn $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@aren $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@wasn $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@weren $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@hasn $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@haven $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@doesn $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@don $($t:tt)*) => { $crate::__det_or_pronoun!(@verb $($t)*) };
    (@verb $end:literal; $det:ident $($pronoun:ident)::+; $($rest:tt)*) => {
        $crate::__pronoun!($end; $($pronoun)::+; $($rest)*)
    };
    (@$other:ident $end:literal; $det:ident $($pronoun:ident)::+; $noun:ident $($rest:tt)*) => {
        $crate::__noun_phrase!($end; $det $noun; $($rest)*)
    };

    ($end:literal; $det:ident $($pronoun:ident)::+; $verb:ident $($rest:tt)*) => {
        $crate::__det_or_pronoun!(@$verb $end; $det $($pronoun)::+; $verb $($rest)*)
    };
    ($end:literal; $det:ident $($pronoun:ident)::+; $($rest:tt)*) => {
        $crate::__pronoun!($end; $($pronoun)::+; $($rest)*)
    };
}

/// `Det Noun`, with the noun's number and first sound worked out at compile
/// time from its spelling.
#[doc(hidden)]
#[macro_export]
macro_rules! __noun_phrase {
    ($end:literal; $det:ident $noun:ident; $($rest:tt)*) => {{
        $crate::grammar::Sound::<{ $crate::lexicon::wants_an(stringify!($noun)) }>::check(&$crate::det::$det);
        let subject = &$crate::Noun::<
            <$crate::grammar::NumberOf<{ $crate::lexicon::is_plural(stringify!($noun)) }> as $crate::grammar::SelectNumber>::Number,
        >::with_determiner(&$crate::det::$det, stringify!($noun));
        $crate::__predicate!($end; subject; $($rest)*)
    }};
}

/// Finds the verb and checks it against the subject.
#[doc(hidden)]
#[macro_export]
macro_rules! __predicate {
    ($end:literal; $s:ident; 'm $($rest:tt)*) => { $crate::__finish!($end; $s; $crate::lexicon::FormClass::Am; "'m"; $($rest)*) };
    ($end:literal; $s:ident; 're $($rest:tt)*) => { $crate::__finish!($end; $s; $crate::lexicon::FormClass::Are; "'re"; $($rest)*) };
    ($end:literal; $s:ident; 's $($rest:tt)*) => { $crate::__finish!($end; $s; $crate::lexicon::FormClass::Is; "'s"; $($rest)*) };
    ($end:literal; $s:ident; 've $($rest:tt)*) => { $crate::__finish!($end; $s; $crate::lexicon::FormClass::Base; "'ve"; $($rest)*) };
    ($end:literal; $s:ident; 'll $($rest:tt)*) => { $crate::__finish!($end; $s; $crate::lexicon::FormClass::Any; "'ll"; $($rest)*) };
    ($end:literal; $s:ident; 'd $($rest:tt)*) => { $crate::__finish!($end; $s; $crate::lexicon::FormClass::Any; "'d"; $($rest)*) };
    ($end:literal; $s:ident; $verb:ident 't $($rest:tt)*) => {
        $crate::__finish!($end; $s; $crate::lexicon::form_class(stringify!($verb), true); concat!(stringify!($verb), "'t"); $($rest)*)
    };
    ($end:literal; $s:ident; $verb:ident $($rest:tt)*) => {
        $crate::__finish!($end; $s; $crate::lexicon::form_class(stringify!($verb), false); stringify!($verb); $($rest)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __finish {
    ($end:literal; $s:ident; $class:expr; $verb:expr; $($rest:tt)*) => {{
        $crate::grammar::Verb::<<$crate::grammar::FormOf<{ $class as u8 }> as $crate::grammar::SelectForm>::Form>::new()
            .agrees_with($s);
        $crate::Sentence::from_macro(
            ::std::string::ToString::to_string($s),
            $verb,
            &[$(stringify!($rest)),*],
            $end,
        )
    }};
}

//! Agreement, as trait bounds.
//!
//! Every subject type implements the `Takes*` traits for the verb forms it can
//! take, and nothing else. The `english!` macro classifies each verb at compile
//! time and asks for the matching bound, so `He run` is a type error rather
//! than a typo. The traits carry `#[diagnostic::on_unimplemented]` messages so
//! the error reads like a grammar note instead of a trait-solver dump.

use crate::lexicon;
use crate::lexicon::FormClass;
use crate::sentence::Sentence;

/// Grammatical person.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Person {
    First,
    Second,
    Third,
}

/// Grammatical number, as a type: `Noun<Singular>` and `Noun<Plural>` are
/// different types, so a noun can never be ambiguously countable.
#[derive(Clone, Copy, Debug, Default)]
pub struct Singular;

/// See [`Singular`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Plural;

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be the subject of a sentence",
    label = "this is in the objective case",
    note = "object pronouns (me, him, her, us, them) cannot be subjects; use I, he, she, we or they"
)]
/// Anything that can be the subject of a clause.
pub trait Subject {
    const PERSON: Person;
    const PLURAL: bool;

    /// How the subject is written, before sentence capitalisation.
    fn render(&self) -> String;

    /// `I.be("happy")` is "I am happy."; `They.be("late")` is "They are late."
    fn be(&self, complement: impl std::fmt::Display) -> Sentence {
        Sentence::runtime(self.render(), be_present(Self::PERSON, Self::PLURAL)).then(complement)
    }

    /// The present tense of `base`, inflected for this subject: `She.verb("run")`
    /// is "She runs."
    fn verb(&self, base: &str) -> Sentence {
        let form = if lexicon::eq(base, "be") {
            be_present(Self::PERSON, Self::PLURAL).to_string()
        } else if third_singular(Self::PERSON, Self::PLURAL) {
            lexicon::third_person(base)
        } else {
            base.to_string()
        };
        Sentence::runtime(self.render(), form)
    }

    /// The simple past: `We.past("go")` is "We went."
    fn past(&self, base: &str) -> Sentence {
        let form = if lexicon::eq(base, "be") {
            match (Self::PERSON, Self::PLURAL) {
                (Person::Second, _) | (_, true) => "were".to_string(),
                _ => "was".to_string(),
            }
        } else {
            lexicon::past(base)
        };
        Sentence::runtime(self.render(), form)
    }

    /// The present perfect: `It.perfect("go")` is "It has gone."
    fn perfect(&self, base: &str) -> Sentence {
        let have = if third_singular(Self::PERSON, Self::PLURAL) {
            "has"
        } else {
            "have"
        };
        Sentence::runtime(
            self.render(),
            format!("{have} {}", lexicon::participle(base)),
        )
    }
}

fn third_singular(person: Person, plural: bool) -> bool {
    person == Person::Third && !plural
}

fn be_present(person: Person, plural: bool) -> &'static str {
    match (person, plural) {
        (Person::First, false) => "am",
        (Person::Third, false) => "is",
        _ => "are",
    }
}

/// Used by `english!` to check that its subject is in the subjective case.
#[doc(hidden)]
pub fn subject<S: Subject>(subject: &S) -> &S {
    subject
}

#[diagnostic::on_unimplemented(
    message = "subject-verb disagreement: `{Self}` cannot take `am`",
    label = "only `I` takes `am`",
    note = "`is` goes with he, she, it and singular nouns; `are` goes with you, we, they and plurals"
)]
pub trait TakesAm {}

#[diagnostic::on_unimplemented(
    message = "subject-verb disagreement: `{Self}` cannot take `is`",
    label = "`is` needs a third-person singular subject",
    note = "`I` takes `am`; you, we, they and plural nouns take `are`"
)]
pub trait TakesIs {}

#[diagnostic::on_unimplemented(
    message = "subject-verb disagreement: `{Self}` cannot take `are`",
    label = "`are` needs you, we, they or a plural subject",
    note = "`I` takes `am`; he, she, it and singular nouns take `is`"
)]
pub trait TakesAre {}

#[diagnostic::on_unimplemented(
    message = "subject-verb disagreement: `{Self}` cannot take `was`",
    label = "`was` needs I, he, she, it or a singular subject",
    note = "you, we, they and plural nouns take `were`"
)]
pub trait TakesWas {}

#[diagnostic::on_unimplemented(
    message = "subject-verb disagreement: `{Self}` cannot take `were`",
    label = "`were` needs you, we, they or a plural subject",
    note = "I, he, she, it and singular nouns take `was` (the subjunctive is not stable in this edition)"
)]
pub trait TakesWere {}

#[diagnostic::on_unimplemented(
    message = "subject-verb disagreement: `{Self}` needs the -s form of this verb",
    label = "third-person singular subjects take -s",
    note = "he runs, she does, it has, the dog barks"
)]
pub trait TakesBase {}

#[diagnostic::on_unimplemented(
    message = "subject-verb disagreement: `{Self}` cannot take the -s form of this verb",
    label = "only third-person singular subjects take -s",
    note = "I run, you do, we have, the dogs bark"
)]
pub trait TakesS {}

/// The finite verb forms, as types. `english!` turns a [`FormClass`] into one
/// of these, then calls [`Verb::agrees_with`], whose bound depends on the form.
#[doc(hidden)]
pub mod form {
    pub struct BaseForm;
    pub struct SForm;
    pub struct Am;
    pub struct Is;
    pub struct Are;
    pub struct Was;
    pub struct Were;
    /// Past tense and modals, which agree with anything.
    pub struct Unmarked;
}

#[doc(hidden)]
pub struct FormOf<const CLASS: u8>;

#[doc(hidden)]
pub trait SelectForm {
    type Form;
}

macro_rules! select_form {
    ($($class:ident => $form:ident,)+) => {$(
        impl SelectForm for FormOf<{ FormClass::$class as u8 }> {
            type Form = form::$form;
        }
    )+};
}

select_form! {
    Base => BaseForm,
    ThirdSingular => SForm,
    Am => Am,
    Is => Is,
    Are => Are,
    Was => Was,
    Were => Were,
    Any => Unmarked,
}

/// A verb in a known form.
#[doc(hidden)]
pub struct Verb<F>(std::marker::PhantomData<F>);

impl<F> Verb<F> {
    pub const fn new() -> Self {
        Verb(std::marker::PhantomData)
    }
}

impl<F> Default for Verb<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl Verb<form::BaseForm> {
    pub fn agrees_with<S: TakesBase>(&self, _: &S) {}
}
impl Verb<form::SForm> {
    pub fn agrees_with<S: TakesS>(&self, _: &S) {}
}
impl Verb<form::Am> {
    pub fn agrees_with<S: TakesAm>(&self, _: &S) {}
}
impl Verb<form::Is> {
    pub fn agrees_with<S: TakesIs>(&self, _: &S) {}
}
impl Verb<form::Are> {
    pub fn agrees_with<S: TakesAre>(&self, _: &S) {}
}
impl Verb<form::Was> {
    pub fn agrees_with<S: TakesWas>(&self, _: &S) {}
}
impl Verb<form::Were> {
    pub fn agrees_with<S: TakesWere>(&self, _: &S) {}
}
impl Verb<form::Unmarked> {
    pub fn agrees_with<S>(&self, _: &S) {}
}

/// Implements the `Takes*` traits a type is entitled to, given its person and
/// number. One line per subject keeps the tables impossible to get half-right.
#[macro_export]
#[doc(hidden)]
macro_rules! __agreement {
    ([$($g:tt)*] $ty:ty => first singular) => {
        $crate::__agreement!(@impl [$($g)*] $ty => TakesAm TakesWas TakesBase);
    };
    ([$($g:tt)*] $ty:ty => third singular) => {
        $crate::__agreement!(@impl [$($g)*] $ty => TakesIs TakesWas TakesS);
    };
    ([$($g:tt)*] $ty:ty => plural) => {
        $crate::__agreement!(@impl [$($g)*] $ty => TakesAre TakesWere TakesBase);
    };
    (@impl [$($g:tt)*] $ty:ty => $first:ident $($more:ident)*) => {
        impl<$($g)*> $crate::grammar::$first for $ty {}
        $crate::__agreement!(@impl [$($g)*] $ty => $($more)*);
    };
    (@impl [$($g:tt)*] $ty:ty =>) => {};
}

/// Chooses `Noun<Singular>` or `Noun<Plural>` from a `const bool`, so the
/// macro can type a noun by its spelling.
#[doc(hidden)]
pub struct NumberOf<const PLURAL: bool>;

#[doc(hidden)]
pub trait SelectNumber {
    type Number;
}
impl SelectNumber for NumberOf<false> {
    type Number = Singular;
}
impl SelectNumber for NumberOf<true> {
    type Number = Plural;
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot come before a singular noun",
    label = "`{Self}` needs a plural noun",
    note = "use a plural noun here, or a determiner like `the`, `a` or `this`"
)]
pub trait BeforeSingular {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot come before a plural noun",
    label = "`{Self}` counts exactly one thing",
    note = "use a singular noun here, or a determiner like `the`, `some` or `these`"
)]
pub trait BeforePlural {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot come before a vowel sound; write `An`",
    label = "this noun starts with a vowel sound",
    note = "the article follows the sound, not the letter: an apple, an hour, an MBA"
)]
pub trait BeforeVowelSound {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot come before a consonant sound; write `A`",
    label = "this noun starts with a consonant sound",
    note = "the article follows the sound, not the letter: a banana, a unicorn, a one-off"
)]
pub trait BeforeConsonantSound {}

/// Checks a determiner against the first sound of its noun.
#[doc(hidden)]
pub struct Sound<const VOWEL: bool>;

impl Sound<true> {
    pub fn check<D: BeforeVowelSound>(_: &D) {}
}
impl Sound<false> {
    pub fn check<D: BeforeConsonantSound>(_: &D) {}
}

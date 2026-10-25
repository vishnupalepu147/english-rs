//! Pronouns.
//!
//! The personal pronouns are unit structs: `I`, `You`, `He`, `She`, `It`,
//! `We`, `They`. Their object forms (`Me`, `Him`, `Her`, `Us`, `Them`) know
//! their person and number but do not implement [`Subject`], so
//! `english!(Me am happy)` is rejected for the right reason.
//!
//! A pronoun that stands for a particular noun is a [`Pronoun`], which borrows
//! its antecedent. In English (C edition) a pronoun is a raw pointer: nothing
//! stops "it" from referring to something that has left the conversation, or
//! to two things at once. Here it is a reference, so the borrow checker
//! rejects both.

use std::fmt;
use std::marker::PhantomData;

use crate::grammar::{
    BeforeConsonantSound, BeforePlural, BeforeSingular, BeforeVowelSound, Person, Subject,
};
use crate::noun::Determiner;

macro_rules! pronouns {
    ($($(#[$doc:meta])* $name:ident = $text:literal, $person:ident, $plural:literal, $class:tt $($word:ident)?;)+) => {$(
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str($text)
            }
        }

        crate::__agreement!([] $name => $class $($word)?);

        impl $name {
            pub const PERSON: Person = Person::$person;
            pub const PLURAL: bool = $plural;
        }
    )+};
}

pronouns! {
    /// First person singular. Always capitalised, for reasons lost to history.
    I = "I", First, false, first singular;
    /// Second person. Grammatically plural even when you are alone.
    You = "you", Second, true, plural;
    He = "he", Third, false, third singular;
    She = "she", Third, false, third singular;
    It = "it", Third, false, third singular;
    We = "we", First, true, plural;
    /// Third person plural, and the singular pronoun for anyone whose pronouns
    /// you have not been told. Agrees like a plural: "they are".
    They = "they", Third, true, plural;
    Me = "me", First, false, first singular;
    Him = "him", Third, false, third singular;
    Her = "her", Third, false, third singular;
    Us = "us", First, true, plural;
    Them = "them", Third, true, plural;
}

macro_rules! subjects {
    ($($name:ident)+) => {$(
        impl Subject for $name {
            const PERSON: Person = $name::PERSON;
            const PLURAL: bool = $name::PLURAL;
            fn render(&self) -> String {
                self.to_string()
            }
        }
    )+};
}

subjects!(I You He She It We They);

/// A pronoun that can stand in for a named thing: `It` for a singular noun,
/// `They` for a plural one, and `He`, `She` or `They` for a person.
pub trait Personal: Subject + fmt::Display + Default + Copy {}
impl Personal for He {}
impl Personal for She {}
impl Personal for It {}
impl Personal for They {}

/// A pronoun with an antecedent. It holds a shared borrow of the noun it
/// stands for, so it cannot outlive that noun, and the noun cannot be moved
/// away while the pronoun is still in use.
///
/// ```
/// use english::prelude::*;
///
/// let dog = Noun::new("dog").the();
/// let it = dog.pronoun();
/// assert_eq!(english!(it barks).to_string(), "It barks.");
/// assert_eq!(it.antecedent(), "the dog");
/// ```
///
/// A dangling pronoun does not compile:
///
/// ```compile_fail,E0505
/// use english::prelude::*;
///
/// let dog = Noun::new("dog").the();
/// let it = dog.pronoun();
/// drop(dog); // the dog leaves the story
/// english!(it barks); // error[E0505]: cannot move out of `dog` because it is borrowed
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Pronoun<'a, P> {
    antecedent: &'a str,
    pronoun: PhantomData<P>,
}

impl<'a, P: Personal> Pronoun<'a, P> {
    pub(crate) fn new(antecedent: &'a str) -> Self {
        Pronoun {
            antecedent,
            pronoun: PhantomData,
        }
    }

    /// The noun phrase this pronoun stands for.
    pub fn antecedent(&self) -> &'a str {
        self.antecedent
    }
}

impl<P: Personal> fmt::Display for Pronoun<'_, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        P::default().fmt(f)
    }
}

impl<P: Personal> Subject for Pronoun<'_, P> {
    const PERSON: Person = P::PERSON;
    const PLURAL: bool = P::PLURAL;
    fn render(&self) -> String {
        self.to_string()
    }
}

crate::__agreement!(['a] Pronoun<'a, He> => third singular);
crate::__agreement!(['a] Pronoun<'a, She> => third singular);
crate::__agreement!(['a] Pronoun<'a, It> => third singular);
crate::__agreement!(['a] Pronoun<'a, They> => plural);

macro_rules! demonstratives {
    ($($name:ident = $text:literal;)+) => {$(
        /// A demonstrative or expletive pronoun used as a subject.
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name;
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str($text)
            }
        }
    )+};
}

/// Demonstrative pronouns. These are distinct from the determiners in
/// [`crate::det`]: in "This is fine" `This` is the subject, while in "This dog
/// is fine" it is part of one.
pub mod demonstrative {
    use super::*;

    demonstratives! {
        This = "this";
        That = "that";
        These = "these";
        Those = "those";
        There = "there";
    }

    impl Subject for This {
        const PERSON: Person = Person::Third;
        const PLURAL: bool = false;
        fn render(&self) -> String {
            self.to_string()
        }
    }
    impl Subject for That {
        const PERSON: Person = Person::Third;
        const PLURAL: bool = false;
        fn render(&self) -> String {
            self.to_string()
        }
    }
    impl Subject for These {
        const PERSON: Person = Person::Third;
        const PLURAL: bool = true;
        fn render(&self) -> String {
            self.to_string()
        }
    }
    impl Subject for Those {
        const PERSON: Person = Person::Third;
        const PLURAL: bool = true;
        fn render(&self) -> String {
            self.to_string()
        }
    }
    /// "There" agrees with whatever follows the verb ("there is a dog",
    /// "there are dogs"), which this edition does not look at, so it takes
    /// every form.
    impl Subject for There {
        const PERSON: Person = Person::Third;
        const PLURAL: bool = false;
        fn render(&self) -> String {
            self.to_string()
        }
    }

    crate::__agreement!([] This => third singular);
    crate::__agreement!([] That => third singular);
    crate::__agreement!([] These => plural);
    crate::__agreement!([] Those => plural);
    crate::__agreement!(@impl [] There => TakesAm TakesIs TakesAre TakesWas TakesWere TakesBase TakesS);

    // The same words are determiners before a noun ("these mice"), and one
    // type plays both parts.
    impl Determiner for This {
        const TEXT: &'static str = "this";
    }
    impl Determiner for That {
        const TEXT: &'static str = "that";
    }
    impl Determiner for These {
        const TEXT: &'static str = "these";
    }
    impl Determiner for Those {
        const TEXT: &'static str = "those";
    }
    impl BeforeSingular for This {}
    impl BeforeSingular for That {}
    impl BeforePlural for These {}
    impl BeforePlural for Those {}
    impl BeforeVowelSound for This {}
    impl BeforeVowelSound for That {}
    impl BeforeVowelSound for These {}
    impl BeforeVowelSound for Those {}
    impl BeforeConsonantSound for This {}
    impl BeforeConsonantSound for That {}
    impl BeforeConsonantSound for These {}
    impl BeforeConsonantSound for Those {}
}

/// `Her` is also the possessive determiner: "her dog".
impl Determiner for Her {
    const TEXT: &'static str = "her";
}
impl BeforeSingular for Her {}
impl BeforePlural for Her {}
impl BeforeVowelSound for Her {}
impl BeforeConsonantSound for Her {}

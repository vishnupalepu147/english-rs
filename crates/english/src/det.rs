//! Determiners. Each one says which numbers of noun it may precede, and the
//! indefinite articles also say which first sound.

use crate::grammar::{BeforeConsonantSound, BeforePlural, BeforeSingular, BeforeVowelSound};
use crate::noun::Determiner;

pub use crate::pronoun::Her;
pub use crate::pronoun::demonstrative::{That, These, This, Those};

macro_rules! determiners {
    ($($name:ident = $text:literal: $($bound:ident)*;)+) => {$(
        #[derive(Clone, Copy, Debug)]
        pub struct $name;
        impl Determiner for $name {
            const TEXT: &'static str = $text;
        }
        $(impl $bound for $name {})*
    )+};
}

determiners! {
    The = "the": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    A = "a": BeforeSingular BeforeConsonantSound;
    An = "an": BeforeSingular BeforeVowelSound;
    My = "my": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    Your = "your": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    His = "his": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    Its = "its": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    Our = "our": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    Their = "their": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    Every = "every": BeforeSingular BeforeVowelSound BeforeConsonantSound;
    Each = "each": BeforeSingular BeforeVowelSound BeforeConsonantSound;
    Some = "some": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    No = "no": BeforeSingular BeforePlural BeforeVowelSound BeforeConsonantSound;
    Many = "many": BeforePlural BeforeVowelSound BeforeConsonantSound;
    Several = "several": BeforePlural BeforeVowelSound BeforeConsonantSound;
    Both = "both": BeforePlural BeforeVowelSound BeforeConsonantSound;
    Few = "few": BeforePlural BeforeVowelSound BeforeConsonantSound;
    All = "all": BeforePlural BeforeVowelSound BeforeConsonantSound;
}

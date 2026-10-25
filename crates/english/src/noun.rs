//! Nouns and people.

use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;

use crate::grammar::{
    BeforePlural, BeforeSingular, Person as GrammaticalPerson, Plural, Singular, Subject,
};
use crate::lexicon;
use crate::pronoun::{It, Personal, Pronoun, They};

/// A common noun. Its number is part of its type, and you only ever write
/// its dictionary form: the plural is computed, so "mouses" cannot happen.
///
/// ```
/// use english::prelude::*;
///
/// assert_eq!(Noun::new("mouse").plural().the().to_string(), "the mice");
/// assert_eq!(Noun::new("hour").a().to_string(), "an hour");
/// assert_eq!(Noun::new("unicorn").a().to_string(), "a unicorn");
/// ```
#[derive(Clone, Debug)]
pub struct Noun<N = Singular> {
    determiner: Option<Cow<'static, str>>,
    word: Cow<'static, str>,
    number: PhantomData<N>,
    /// The rendered phrase, kept so pronouns can borrow it.
    phrase: String,
}

impl<N> Noun<N> {
    fn build(determiner: Option<Cow<'static, str>>, word: Cow<'static, str>) -> Self {
        let phrase = match &determiner {
            Some(d) => format!("{d} {word}"),
            None => word.to_string(),
        };
        Noun {
            determiner,
            word,
            number: PhantomData,
            phrase,
        }
    }

    /// The noun itself, without its determiner.
    pub fn word(&self) -> &str {
        &self.word
    }

    /// `the dog`, `the dogs`.
    pub fn the(self) -> Self {
        Self::build(Some("the".into()), self.word)
    }
}

impl Noun<Singular> {
    /// A singular noun in its dictionary form.
    pub fn new(word: impl Into<Cow<'static, str>>) -> Self {
        Self::build(None, word.into())
    }

    /// `a dog`, `an hour`: the article is chosen from the sound of the word.
    pub fn a(self) -> Self {
        let article = lexicon::indefinite_article(&self.word);
        Self::build(Some(article.into()), self.word)
    }

    /// The plural, which is a different type. `mouse` becomes `mice`.
    pub fn plural(self) -> Noun<Plural> {
        Noun::build(self.determiner, lexicon::plural(&self.word).into())
    }

    /// `it`, borrowing this noun.
    pub fn pronoun(&self) -> Pronoun<'_, It> {
        Pronoun::new(&self.phrase)
    }
}

impl Noun<Plural> {
    /// `they`, borrowing this noun.
    pub fn pronoun(&self) -> Pronoun<'_, They> {
        Pronoun::new(&self.phrase)
    }
}

impl<N> fmt::Display for Noun<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.phrase)
    }
}

impl Subject for Noun<Singular> {
    const PERSON: GrammaticalPerson = GrammaticalPerson::Third;
    const PLURAL: bool = false;
    fn render(&self) -> String {
        self.phrase.clone()
    }
}

impl Subject for Noun<Plural> {
    const PERSON: GrammaticalPerson = GrammaticalPerson::Third;
    const PLURAL: bool = true;
    fn render(&self) -> String {
        self.phrase.clone()
    }
}

crate::__agreement!([] Noun<Singular> => third singular);
crate::__agreement!([] Noun<Plural> => plural);

/// A determiner such as `the`, `a` or `these`. See [`crate::det`].
pub trait Determiner {
    const TEXT: &'static str;
}

#[doc(hidden)]
impl Noun<Singular> {
    /// Used by `english!` for `The dog`, `A dog` and so on.
    pub fn with_determiner<D: Determiner + BeforeSingular>(_: &D, word: &'static str) -> Self {
        Self::build(Some(D::TEXT.into()), word.into())
    }
}

#[doc(hidden)]
impl Noun<Plural> {
    /// Used by `english!` for `The dogs`, `These dogs` and so on.
    pub fn with_determiner<D: Determiner + BeforePlural>(_: &D, word: &'static str) -> Self {
        Self::build(Some(D::TEXT.into()), word.into())
    }
}

/// A named person. Their pronouns are part of their type and default to
/// `They`, because a name does not tell you someone's pronouns.
///
/// ```
/// use english::prelude::*;
///
/// let ada = Person::new("Ada");
/// assert_eq!(english!(ada writes code. They 're good at it).to_string(),
///            "Ada writes code. They're good at it.");
///
/// let ferris = Person::new("Ferris").pronouns(He);
/// let he = ferris.pronoun();
/// assert_eq!(he.past("go").then("home").to_string(), "He went home.");
/// ```
#[derive(Clone, Debug)]
pub struct Person<P = They> {
    name: Cow<'static, str>,
    pronouns: PhantomData<P>,
}

impl Person<They> {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Person {
            name: name.into(),
            pronouns: PhantomData,
        }
    }
}

impl<P: Personal> Person<P> {
    /// Use the pronouns this person asked for.
    pub fn pronouns<Q: Personal>(self, _: Q) -> Person<Q> {
        Person {
            name: self.name,
            pronouns: PhantomData,
        }
    }

    /// Their pronoun, borrowing them.
    pub fn pronoun(&self) -> Pronoun<'_, P> {
        Pronoun::new(&self.name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<P> fmt::Display for Person<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

/// A name is third-person singular whatever the person's pronouns are:
/// "Ada is here; they are early."
impl<P> Subject for Person<P> {
    const PERSON: GrammaticalPerson = GrammaticalPerson::Third;
    const PLURAL: bool = false;
    fn render(&self) -> String {
        self.name.to_string()
    }
}

crate::__agreement!([P] Person<P> => third singular);

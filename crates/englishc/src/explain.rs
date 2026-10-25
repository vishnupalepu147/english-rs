//! `englishc --explain`.

pub fn explain(code: &str) -> Option<&'static str> {
    let code = code.trim().to_ascii_uppercase();
    let code = if code.starts_with('E') {
        code
    } else {
        format!("E{code:0>4}")
    };
    EXPLANATIONS
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, text)| text.trim())
}

const EXPLANATIONS: &[(&str, &str)] = &[
    (
        "E0001",
        r#"
A verb form does not agree with its subject.

Erroneous code example:

```english
He are late. They was here. She don't care.
```

In English (C edition) agreement is checked by the reader at run time, which
is why this compiles in most heads. English (Rust edition) checks it in the
type system: every subject implements the `Takes*` traits for the forms it
may take, and `He` does not implement `TakesAre`.

The forms are:

| subject                   | be (present) | be (past) | have | do   | other verbs |
|---------------------------|--------------|-----------|------|------|-------------|
| I                         | am           | was       | have | do   | run         |
| he, she, it, one noun     | is           | was       | has  | does | runs        |
| you, we, they, plurals    | are          | were      | have | do   | run         |

Corrected example:

```english
He is late. They were here. She doesn't care.
```

`you` is grammatically plural even when it means one person, and so is
singular `they`. The subjunctive ("if I were you") is accepted after `if`,
`wish`, `unless` and similar words.
"#,
    ),
    (
        "E0002",
        r#"
The wrong indefinite article was used.

Erroneous code example:

```english
a apple, an banana, a hour, an unicorn
```

`a` goes before a consonant sound and `an` before a vowel sound. The rule is
about sound, not spelling, so `hour` (silent h) takes `an`, `unicorn` (a "y"
sound) takes `a`, and initialisms are read letter by letter: an MBA, an FBI
agent, a URL. Acronyms read as words follow the word: a NATO summit.

Corrected example:

```english
an apple, a banana, an hour, a unicorn
```
"#,
    ),
    (
        "E0003",
        r#"
`of` was written where the auxiliary `have` belongs.

Erroneous code example:

```english
I could of won. You should of called.
```

`could've` and `could of` sound the same when spoken, but `of` is a
preposition and cannot follow a modal verb. The compiler reads the text, not
the sound.

Corrected example:

```english
I could have won. You should've called.
```
"#,
    ),
    (
        "E0004",
        r#"
An irregular verb was given the regular `-ed` ending.

Erroneous code example:

```english
I goed home and we runned.
```

About 180 verbs still use their Old English inflections for backward
compatibility, and `-ed` is not implemented for them. The past tense and the
past participle must be looked up:

```english
go → went → gone
run → ran → run
bring → brought → brought
```

Corrected example:

```english
I went home and we ran.
```

In English (Rust edition) you never write inflected forms: `They.past("go")`
renders as "They went", so this error cannot occur.
"#,
    ),
    (
        "E0005",
        r#"
A past tense was used where a past participle belongs, or the other way round.

Erroneous code example:

```english
She had went. They have ate. I seen it.
```

After `have`, `has` or `had` a verb takes its past participle (`gone`,
`eaten`, `seen`). Without an auxiliary it takes its simple past (`went`,
`ate`, `saw`). For regular verbs the two are spelled the same, which is why
this goes unnoticed until an irregular verb comes along.

Corrected example:

```english
She had gone. They have eaten. I saw it (or: I have seen it).
```
"#,
    ),
    (
        "E0006",
        r#"
An irregular noun was given the regular `-s` plural, or a plural was made
plural again.

Erroneous code example:

```english
The childs have teeths. Two mouses. Many criterions. The childrens.
```

Corrected example:

```english
The children have teeth. Two mice. Many criteria. The children.
```

In English (Rust edition) the plural is a different type, produced by
`Noun::plural`, which knows the irregular table.
"#,
    ),
    (
        "E0007",
        r#"
`its` and `it's` were swapped.

Erroneous code example:

```english
Its a trap. The dog wagged it's tail.
```

`it's` is a contraction of "it is" or "it has". `its` is the possessive, and
like `his` and `hers` it has no apostrophe.

Corrected example:

```english
It's a trap. The dog wagged its tail.
```
"#,
    ),
    (
        "E0008",
        r#"
`your` and `you're` were swapped.

Erroneous code example:

```english
Your welcome. Your a genius.
```

`you're` is a contraction of "you are"; `your` is the possessive.

Corrected example:

```english
You're welcome. You're a genius.
```
"#,
    ),
    (
        "E0009",
        r#"
`there`, `their` and `they're` were swapped.

Erroneous code example:

```english
Their is a problem. There going home. They lost there own keys.
```

`there` is a place, and also the dummy subject in "there is". `their` is the
possessive. `they're` is a contraction of "they are".

Corrected example:

```english
There is a problem. They're going home. They lost their own keys.
```
"#,
    ),
    (
        "E0010",
        r#"
`then` was used after a comparative, where `than` belongs.

Erroneous code example:

```english
Rust is faster then C. I would rather walk then drive.
```

`then` is about time ("first this, then that"); `than` introduces the second
half of a comparison.

Corrected example:

```english
Rust is faster than C. I would rather walk than drive.
```
"#,
    ),
    (
        "E0011",
        r#"
A word was not found in the dictionary, and a likely correction is known.

Erroneous code example:

```english
Irregardless, I dont think alot of it.
```

Corrected example:

```english
Regardless, I don't think a lot of it.
```

The dictionary only knows the misspellings it has a confident correction for;
everything else is assumed to be a word you know and it does not.
"#,
    ),
    (
        "E0012",
        r#"
The first-person pronoun was written in lower case.

Erroneous code example:

```english
i think i'm right.
```

`I` is always capitalized, wherever it appears in a sentence. It is the only
pronoun with this property, and no edition will change it.

Corrected example:

```english
I think I'm right.
```
"#,
    ),
    (
        "E0013",
        r#"
An object pronoun was used as the subject of a sentence.

Erroneous code example:

```english
Me and my brother went home. Him and her are friends.
```

`me`, `him`, `her`, `us` and `them` are the objective case. A subject needs
the subjective case: `I`, `he`, `she`, `we`, `they`. Joining two subjects
with `and` does not change this, and by convention `I` goes last.

Corrected example:

```english
My brother and I went home. He and she are friends.
```

In English (Rust edition) the object pronouns do not implement `Subject`, so
`english!(Me am happy)` fails with "`Me` cannot be the subject of a sentence".
"#,
    ),
];

//! The standard library of English: irregular verbs, irregular plurals and the
//! rules for everything else.
//!
//! Most of these functions are `const fn`, because the `english!` macro asks
//! them questions at compile time ("is `dogs` plural?", "does `hour` start
//! with a vowel sound?"). Comparisons are ASCII case-insensitive.
//!
//! Where a table cell holds several spellings they are separated by `|`, and
//! the first one is the preferred form.

/// `(base, past, past participle)` for every irregular verb English still ships.
pub const IRREGULAR_VERBS: &[(&str, &str, &str)] = &[
    ("arise", "arose", "arisen"),
    ("awake", "awoke", "awoken"),
    ("be", "was|were", "been"),
    ("bear", "bore", "borne|born"),
    ("beat", "beat", "beaten|beat"),
    ("become", "became", "become"),
    ("begin", "began", "begun"),
    ("behold", "beheld", "beheld"),
    ("bend", "bent", "bent"),
    ("bet", "bet", "bet"),
    ("bid", "bid", "bid"),
    ("bind", "bound", "bound"),
    ("bite", "bit", "bitten|bit"),
    ("bleed", "bled", "bled"),
    ("blow", "blew", "blown"),
    ("break", "broke", "broken"),
    ("breed", "bred", "bred"),
    ("bring", "brought", "brought"),
    ("broadcast", "broadcast", "broadcast"),
    ("build", "built", "built"),
    ("burn", "burned|burnt", "burned|burnt"),
    ("burst", "burst", "burst"),
    ("buy", "bought", "bought"),
    ("cast", "cast", "cast"),
    ("catch", "caught", "caught"),
    ("choose", "chose", "chosen"),
    ("cling", "clung", "clung"),
    ("come", "came", "come"),
    ("cost", "cost", "cost"),
    ("creep", "crept", "crept"),
    ("cut", "cut", "cut"),
    ("deal", "dealt", "dealt"),
    ("dig", "dug", "dug"),
    ("dive", "dived|dove", "dived"),
    ("do", "did", "done"),
    ("draw", "drew", "drawn"),
    ("dream", "dreamed|dreamt", "dreamed|dreamt"),
    ("drink", "drank", "drunk"),
    ("drive", "drove", "driven"),
    ("dwell", "dwelt|dwelled", "dwelt|dwelled"),
    ("eat", "ate", "eaten"),
    ("fall", "fell", "fallen"),
    ("feed", "fed", "fed"),
    ("feel", "felt", "felt"),
    ("fight", "fought", "fought"),
    ("find", "found", "found"),
    ("fit", "fit|fitted", "fit|fitted"),
    ("flee", "fled", "fled"),
    ("fling", "flung", "flung"),
    ("fly", "flew", "flown"),
    ("forbid", "forbade", "forbidden"),
    ("forecast", "forecast", "forecast"),
    ("foresee", "foresaw", "foreseen"),
    ("forget", "forgot", "forgotten|forgot"),
    ("forgive", "forgave", "forgiven"),
    ("forsake", "forsook", "forsaken"),
    ("freeze", "froze", "frozen"),
    ("get", "got", "got|gotten"),
    ("give", "gave", "given"),
    ("go", "went", "gone"),
    ("grind", "ground", "ground"),
    ("grow", "grew", "grown"),
    ("hang", "hung|hanged", "hung|hanged"),
    ("have", "had", "had"),
    ("hear", "heard", "heard"),
    ("hide", "hid", "hidden"),
    ("hit", "hit", "hit"),
    ("hold", "held", "held"),
    ("hurt", "hurt", "hurt"),
    ("keep", "kept", "kept"),
    ("kneel", "knelt|kneeled", "knelt|kneeled"),
    ("know", "knew", "known"),
    ("lay", "laid", "laid"),
    ("lead", "led", "led"),
    ("lean", "leaned|leant", "leaned|leant"),
    ("leap", "leaped|leapt", "leaped|leapt"),
    ("learn", "learned|learnt", "learned|learnt"),
    ("leave", "left", "left"),
    ("lend", "lent", "lent"),
    ("let", "let", "let"),
    ("lie", "lay|lied", "lain|lied"),
    ("light", "lit|lighted", "lit|lighted"),
    ("lose", "lost", "lost"),
    ("make", "made", "made"),
    ("mean", "meant", "meant"),
    ("meet", "met", "met"),
    ("mislead", "misled", "misled"),
    ("mistake", "mistook", "mistaken"),
    ("misunderstand", "misunderstood", "misunderstood"),
    ("overcome", "overcame", "overcome"),
    ("overtake", "overtook", "overtaken"),
    ("pay", "paid", "paid"),
    ("plead", "pleaded|pled", "pleaded|pled"),
    ("prove", "proved", "proved|proven"),
    ("put", "put", "put"),
    ("quit", "quit", "quit"),
    ("read", "read", "read"),
    ("redo", "redid", "redone"),
    ("rewrite", "rewrote", "rewritten"),
    ("ride", "rode", "ridden"),
    ("ring", "rang", "rung"),
    ("rise", "rose", "risen"),
    ("run", "ran", "run"),
    ("say", "said", "said"),
    ("see", "saw", "seen"),
    ("seek", "sought", "sought"),
    ("sell", "sold", "sold"),
    ("send", "sent", "sent"),
    ("set", "set", "set"),
    ("sew", "sewed", "sewn|sewed"),
    ("shake", "shook", "shaken"),
    ("shed", "shed", "shed"),
    ("shine", "shone|shined", "shone|shined"),
    ("shoot", "shot", "shot"),
    ("show", "showed", "shown|showed"),
    ("shrink", "shrank|shrunk", "shrunk"),
    ("shut", "shut", "shut"),
    ("sing", "sang", "sung"),
    ("sink", "sank", "sunk"),
    ("sit", "sat", "sat"),
    ("slay", "slew", "slain"),
    ("sleep", "slept", "slept"),
    ("slide", "slid", "slid"),
    ("sling", "slung", "slung"),
    ("slit", "slit", "slit"),
    ("smell", "smelled|smelt", "smelled|smelt"),
    ("sneak", "sneaked|snuck", "sneaked|snuck"),
    ("speak", "spoke", "spoken"),
    ("speed", "sped|speeded", "sped|speeded"),
    ("spell", "spelled|spelt", "spelled|spelt"),
    ("spend", "spent", "spent"),
    ("spill", "spilled|spilt", "spilled|spilt"),
    ("spin", "spun", "spun"),
    ("spit", "spat|spit", "spat|spit"),
    ("split", "split", "split"),
    ("spoil", "spoiled|spoilt", "spoiled|spoilt"),
    ("spread", "spread", "spread"),
    ("spring", "sprang", "sprung"),
    ("stand", "stood", "stood"),
    ("steal", "stole", "stolen"),
    ("stick", "stuck", "stuck"),
    ("sting", "stung", "stung"),
    ("stink", "stank|stunk", "stunk"),
    ("stride", "strode", "stridden"),
    ("strike", "struck", "struck|stricken"),
    ("string", "strung", "strung"),
    ("strive", "strove|strived", "striven|strived"),
    ("swear", "swore", "sworn"),
    ("sweep", "swept", "swept"),
    ("swell", "swelled", "swollen|swelled"),
    ("swim", "swam", "swum"),
    ("swing", "swung", "swung"),
    ("take", "took", "taken"),
    ("teach", "taught", "taught"),
    ("tear", "tore", "torn"),
    ("tell", "told", "told"),
    ("think", "thought", "thought"),
    ("throw", "threw", "thrown"),
    ("thrust", "thrust", "thrust"),
    ("tread", "trod", "trodden"),
    ("undergo", "underwent", "undergone"),
    ("understand", "understood", "understood"),
    ("undertake", "undertook", "undertaken"),
    ("undo", "undid", "undone"),
    ("uphold", "upheld", "upheld"),
    ("upset", "upset", "upset"),
    ("wake", "woke", "woken"),
    ("wear", "wore", "worn"),
    ("weave", "wove", "woven"),
    ("weep", "wept", "wept"),
    ("wet", "wet|wetted", "wet|wetted"),
    ("win", "won", "won"),
    ("wind", "wound", "wound"),
    ("withdraw", "withdrew", "withdrawn"),
    ("withhold", "withheld", "withheld"),
    ("withstand", "withstood", "withstood"),
    ("wring", "wrung", "wrung"),
    ("write", "wrote", "written"),
];

/// Common regular verbs. Together with the irregular table this is the set of
/// words `englishc` is confident are verbs.
pub const REGULAR_VERBS: &[&str] = &[
    "accept",
    "add",
    "admit",
    "agree",
    "allow",
    "answer",
    "appear",
    "argue",
    "arrive",
    "ask",
    "attack",
    "avoid",
    "bake",
    "believe",
    "belong",
    "borrow",
    "bother",
    "breathe",
    "call",
    "care",
    "carry",
    "change",
    "charge",
    "chase",
    "cheat",
    "check",
    "clean",
    "climb",
    "close",
    "code",
    "collect",
    "compile",
    "complain",
    "complete",
    "consider",
    "contain",
    "continue",
    "cook",
    "copy",
    "count",
    "cover",
    "crash",
    "create",
    "cross",
    "cry",
    "dance",
    "decide",
    "deliver",
    "depend",
    "deploy",
    "describe",
    "deserve",
    "destroy",
    "die",
    "disagree",
    "disappear",
    "discover",
    "dislike",
    "drop",
    "earn",
    "enjoy",
    "enter",
    "escape",
    "exist",
    "expect",
    "explain",
    "fail",
    "fill",
    "finish",
    "fix",
    "follow",
    "force",
    "guess",
    "hate",
    "happen",
    "help",
    "hope",
    "hurry",
    "ignore",
    "imagine",
    "improve",
    "include",
    "intend",
    "invite",
    "join",
    "joke",
    "jump",
    "kick",
    "kill",
    "kiss",
    "knock",
    "laugh",
    "like",
    "listen",
    "live",
    "look",
    "love",
    "manage",
    "marry",
    "matter",
    "mention",
    "mind",
    "miss",
    "move",
    "need",
    "notice",
    "obey",
    "offer",
    "open",
    "order",
    "own",
    "paint",
    "pass",
    "perform",
    "pick",
    "plan",
    "play",
    "please",
    "point",
    "prefer",
    "prepare",
    "pretend",
    "print",
    "promise",
    "protect",
    "pull",
    "push",
    "reach",
    "realize",
    "receive",
    "recommend",
    "refuse",
    "relax",
    "remain",
    "remember",
    "remove",
    "repeat",
    "reply",
    "report",
    "require",
    "rest",
    "return",
    "review",
    "rush",
    "save",
    "scream",
    "search",
    "seem",
    "serve",
    "share",
    "shout",
    "smile",
    "solve",
    "sound",
    "start",
    "stay",
    "stop",
    "study",
    "succeed",
    "suggest",
    "suppose",
    "talk",
    "taste",
    "test",
    "thank",
    "touch",
    "travel",
    "try",
    "turn",
    "type",
    "understand",
    "use",
    "visit",
    "vote",
    "wait",
    "walk",
    "want",
    "warn",
    "wash",
    "watch",
    "wish",
    "wonder",
    "work",
    "worry",
    "yell",
];

/// `(singular, plural)` for nouns that do not just take `-s`.
pub const IRREGULAR_PLURALS: &[(&str, &str)] = &[
    ("aircraft", "aircraft"),
    ("alumnus", "alumni"),
    ("analysis", "analyses"),
    ("appendix", "appendices|appendixes"),
    ("axis", "axes"),
    ("bacterium", "bacteria"),
    ("basis", "bases"),
    ("cactus", "cacti|cactuses"),
    ("calf", "calves"),
    ("child", "children"),
    ("crisis", "crises"),
    ("criterion", "criteria"),
    ("curriculum", "curricula|curriculums"),
    ("datum", "data"),
    ("deer", "deer"),
    ("diagnosis", "diagnoses"),
    ("echo", "echoes"),
    ("elf", "elves"),
    ("fish", "fish|fishes"),
    ("focus", "foci|focuses"),
    ("foot", "feet"),
    ("fungus", "fungi|funguses"),
    ("goose", "geese"),
    ("half", "halves"),
    ("hero", "heroes"),
    ("hoof", "hooves|hoofs"),
    ("hypothesis", "hypotheses"),
    ("index", "indices|indexes"),
    ("knife", "knives"),
    ("leaf", "leaves"),
    ("life", "lives"),
    ("loaf", "loaves"),
    ("louse", "lice"),
    ("man", "men"),
    ("matrix", "matrices|matrixes"),
    ("medium", "media|mediums"),
    ("moose", "moose"),
    ("mouse", "mice"),
    ("nucleus", "nuclei"),
    ("oasis", "oases"),
    ("octopus", "octopuses|octopi"),
    ("offspring", "offspring"),
    ("ox", "oxen"),
    ("parenthesis", "parentheses"),
    ("person", "people|persons"),
    ("phenomenon", "phenomena"),
    ("potato", "potatoes"),
    ("radius", "radii|radiuses"),
    ("salmon", "salmon"),
    ("scarf", "scarves|scarfs"),
    ("self", "selves"),
    ("series", "series"),
    ("sheep", "sheep"),
    ("shelf", "shelves"),
    ("species", "species"),
    ("stimulus", "stimuli"),
    ("stratum", "strata"),
    ("syllabus", "syllabi|syllabuses"),
    ("thesis", "theses"),
    ("thief", "thieves"),
    ("tomato", "tomatoes"),
    ("tooth", "teeth"),
    ("torpedo", "torpedoes"),
    ("vertex", "vertices|vertexes"),
    ("veto", "vetoes"),
    ("wife", "wives"),
    ("wolf", "wolves"),
    ("woman", "women"),
];

/// Acronyms that are read as words, so they take the article of their first
/// sound rather than of their first letter's name ("a NATO summit").
const SPOKEN_ACRONYMS: &[&str] = &[
    "nato", "nasa", "nascar", "unicef", "unesco", "scuba", "laser", "radar", "opec", "fifa",
    "nafta", "gif", "jpeg", "png", "posix", "ascii", "awol", "wasm", "sql", "faq", "scsi", "gui",
];

/// Words that start with a vowel letter but a consonant sound ("a unicorn").
const CONSONANT_SOUND_PREFIXES: &[&str] = &[
    "eu", "ewe", "one", "once", "ouija", "ubiq", "uganda", "ukr", "uku", "ufo", "unanim", "uni",
    "ura", "ure", "uri", "uro", "usa", "use", "usi", "usu", "usur", "uta", "ute", "uti", "uto",
    "uvu",
];

/// Exceptions to [`CONSONANT_SOUND_PREFIXES`]: `un-` words that are negations ("an
/// uninvited guest").
const VOWEL_SOUND_EXCEPTIONS: &[&str] = &["unin", "unim", "unident"];

/// Words that start with a silent `h` ("an hour").
const SILENT_H_PREFIXES: &[&str] = &["hour", "honest", "honor", "honour", "heir"];

// ---------------------------------------------------------------------------
// const string helpers

const fn lower(b: u8) -> u8 {
    b.to_ascii_lowercase()
}

/// ASCII case-insensitive equality.
pub const fn eq(a: &str, b: &str) -> bool {
    eq_bytes(a.as_bytes(), b.as_bytes())
}

const fn eq_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if lower(a[i]) != lower(b[i]) {
            return false;
        }
        i += 1;
    }
    true
}

const fn starts_with(word: &[u8], prefix: &str) -> bool {
    let p = prefix.as_bytes();
    if word.len() < p.len() {
        return false;
    }
    let mut i = 0;
    while i < p.len() {
        if lower(word[i]) != lower(p[i]) {
            return false;
        }
        i += 1;
    }
    true
}

const fn ends_with(word: &[u8], suffix: &str) -> bool {
    let s = suffix.as_bytes();
    if word.len() < s.len() {
        return false;
    }
    let offset = word.len() - s.len();
    let mut i = 0;
    while i < s.len() {
        if lower(word[offset + i]) != lower(s[i]) {
            return false;
        }
        i += 1;
    }
    true
}

const fn starts_with_any(word: &[u8], prefixes: &[&str]) -> bool {
    let mut i = 0;
    while i < prefixes.len() {
        if starts_with(word, prefixes[i]) {
            return true;
        }
        i += 1;
    }
    false
}

const fn contains(list: &[&str], word: &[u8]) -> bool {
    let mut i = 0;
    while i < list.len() {
        if eq_bytes(list[i].as_bytes(), word) {
            return true;
        }
        i += 1;
    }
    false
}

/// Does the `|`-separated `alternatives` contain `word`?
pub const fn alternatives_contain(alternatives: &str, word: &str) -> bool {
    let alts = alternatives.as_bytes();
    let w = word.as_bytes();
    let mut start = 0;
    while start <= alts.len() {
        let mut end = start;
        while end < alts.len() && alts[end] != b'|' {
            end += 1;
        }
        if end - start == w.len() {
            let mut i = 0;
            let mut same = true;
            while i < w.len() {
                if lower(alts[start + i]) != lower(w[i]) {
                    same = false;
                    break;
                }
                i += 1;
            }
            if same {
                return true;
            }
        }
        start = end + 1;
    }
    false
}

/// The preferred (first) spelling in a `|`-separated cell.
pub fn preferred(alternatives: &str) -> &str {
    alternatives.split('|').next().unwrap_or(alternatives)
}

const fn is_vowel(b: u8) -> bool {
    matches!(lower(b), b'a' | b'e' | b'i' | b'o' | b'u')
}

// ---------------------------------------------------------------------------
// articles

/// Does `word` start with a vowel *sound*, and so take `an`?
///
/// ```
/// use english::lexicon::wants_an;
/// assert!(wants_an("apple") && wants_an("hour") && wants_an("MBA") && wants_an("8"));
/// assert!(!wants_an("unicorn") && !wants_an("European") && !wants_an("NATO"));
/// ```
pub const fn wants_an(word: &str) -> bool {
    let w = word.as_bytes();
    if w.is_empty() {
        return false;
    }
    // Numbers are read aloud: an 8, an 11, an 18, an 80.
    if w[0].is_ascii_digit() {
        if w[0] == b'8' {
            return true;
        }
        let mut digits = 0;
        while digits < w.len() && w[digits].is_ascii_digit() {
            digits += 1;
        }
        return w[0] == b'1'
            && digits > 1
            && (w[1] == b'1' || w[1] == b'8')
            && (digits == 2 || digits == 5);
    }
    // Initialisms are read letter by letter: an MBA, an FBI agent, a URL, an
    // X-ray.
    if is_initialism(w) || w.len() == 1 || w[1] == b'-' {
        return matches!(
            lower(w[0]),
            b'a' | b'e' | b'f' | b'h' | b'i' | b'l' | b'm' | b'n' | b'o' | b'r' | b's' | b'x'
        );
    }
    if starts_with_any(w, SILENT_H_PREFIXES) {
        return true;
    }
    if starts_with_any(w, VOWEL_SOUND_EXCEPTIONS) {
        return true;
    }
    if starts_with_any(w, CONSONANT_SOUND_PREFIXES) {
        return false;
    }
    is_vowel(w[0])
}

/// All caps, at least two letters, and not a word in disguise like `NATO`.
const fn is_initialism(w: &[u8]) -> bool {
    if w.len() < 2 || contains(SPOKEN_ACRONYMS, w) {
        return false;
    }
    let mut i = 0;
    while i < w.len() {
        if !(w[i].is_ascii_uppercase() || w[i].is_ascii_digit()) {
            return false;
        }
        i += 1;
    }
    true
}

/// `"a"` or `"an"`.
pub const fn indefinite_article(word: &str) -> &'static str {
    if wants_an(word) { "an" } else { "a" }
}

// ---------------------------------------------------------------------------
// nouns

/// Is `word` a plural noun? Used by `english!` to type-check determiners.
///
/// ```
/// use english::lexicon::is_plural;
/// assert!(is_plural("dogs") && is_plural("mice") && is_plural("children") && is_plural("firemen"));
/// assert!(!is_plural("dog") && !is_plural("bus") && !is_plural("analysis") && !is_plural("omen"));
/// ```
pub const fn is_plural(word: &str) -> bool {
    let w = word.as_bytes();
    let mut i = 0;
    while i < IRREGULAR_PLURALS.len() {
        let (singular, plural) = IRREGULAR_PLURALS[i];
        if eq_bytes(singular.as_bytes(), w) {
            // Zero plurals (sheep, deer) read as singular unless told otherwise.
            return false;
        }
        if alternatives_contain(plural, word) {
            return true;
        }
        i += 1;
    }
    // Compounds of man and woman: firemen, councilwomen.
    if ends_with(w, "men") && !contains(NOT_MEN, w) {
        return true;
    }
    ends_with(w, "s")
        && !ends_with(w, "ss")
        && !ends_with(w, "us")
        && !ends_with(w, "is")
        && w.len() > 2
}

/// Singular words that happen to end in "men".
const NOT_MEN: &[&str] = &[
    "omen", "amen", "semen", "stamen", "abdomen", "specimen", "regimen", "acumen", "hymen",
    "yemen", "ramen", "dolmen", "lumen", "albumen", "rumen", "cyclamen", "germen",
];

/// The plural of a singular noun: `dog → dogs`, `box → boxes`, `city → cities`,
/// `mouse → mice`.
pub fn plural(singular: &str) -> String {
    for (s, p) in IRREGULAR_PLURALS {
        if eq(s, singular) {
            return match_case(singular, preferred(p));
        }
    }
    regular_plural(singular)
}

/// What `-s` would give, with no knowledge of irregular nouns.
pub fn regular_plural(singular: &str) -> String {
    let w = singular.as_bytes();
    let lower = singular.to_ascii_lowercase();
    if ["s", "x", "z", "ch", "sh"]
        .iter()
        .any(|s| lower.ends_with(s))
    {
        format!("{singular}es")
    } else if w.len() > 1 && lower.ends_with('y') && !is_vowel(w[w.len() - 2]) {
        format!("{}ies", &singular[..singular.len() - 1])
    } else {
        format!("{singular}s")
    }
}

// ---------------------------------------------------------------------------
// verbs

/// Find the irregular entry whose base is `base`.
pub fn irregular(base: &str) -> Option<&'static (&'static str, &'static str, &'static str)> {
    IRREGULAR_VERBS.iter().find(|(b, _, _)| eq(b, base))
}

/// Is `word` the base form of a verb we know?
pub fn is_verb_base(word: &str) -> bool {
    irregular(word).is_some() || REGULAR_VERBS.iter().any(|v| eq(v, word))
}

/// If `word` is the `-s` form of a verb we know, return its base.
pub fn base_of_third_person(word: &str) -> Option<&'static str> {
    IRREGULAR_VERBS
        .iter()
        .map(|(b, _, _)| *b)
        .chain(REGULAR_VERBS.iter().copied())
        .find(|base| *base != "be" && eq(&third_person(base), word))
}

/// The third-person singular present: `run → runs`, `go → goes`, `try → tries`,
/// `have → has`, `be → is`.
pub fn third_person(base: &str) -> String {
    let lower = base.to_ascii_lowercase();
    match lower.as_str() {
        "be" => return match_case(base, "is"),
        "have" => return match_case(base, "has"),
        _ => {}
    }
    let w = base.as_bytes();
    let sibilant = ["s", "x", "z", "ch", "sh"]
        .iter()
        .any(|s| lower.ends_with(s));
    let consonant_o = lower.ends_with('o') && w.len() > 1 && !is_vowel(w[w.len() - 2]);
    if sibilant || consonant_o {
        format!("{base}es")
    } else if lower.ends_with('y') && w.len() > 1 && !is_vowel(w[w.len() - 2]) {
        format!("{}ies", &base[..base.len() - 1])
    } else {
        format!("{base}s")
    }
}

/// The simple past: `walk → walked`, `go → went`.
pub fn past(base: &str) -> String {
    match irregular(base) {
        Some((_, past, _)) => match_case(base, preferred(past)),
        None => regular_past(base),
    }
}

/// The past participle: `walk → walked`, `go → gone`.
pub fn participle(base: &str) -> String {
    match irregular(base) {
        Some((_, _, participle)) => match_case(base, preferred(participle)),
        None => regular_past(base),
    }
}

/// What `-ed` would give, with no knowledge of irregular verbs. This is how
/// `englishc` predicts mistakes like `goed` and `runned`.
pub fn regular_past(base: &str) -> String {
    let lower = base.to_ascii_lowercase();
    let w = lower.as_bytes();
    let n = w.len();
    if lower.ends_with('e') {
        format!("{base}d")
    } else if n > 1 && lower.ends_with('y') && !is_vowel(w[n - 2]) {
        format!("{}ied", &base[..base.len() - 1])
    } else if doubles_final_consonant(w) {
        format!("{base}{}ed", &base[base.len() - 1..])
    } else {
        format!("{base}ed")
    }
}

/// One syllable ending consonant-vowel-consonant (`stop`, `run`), so the final
/// consonant doubles before `-ed`.
fn doubles_final_consonant(w: &[u8]) -> bool {
    let n = w.len();
    if n < 3 || matches!(w[n - 1], b'w' | b'x' | b'y') {
        return false;
    }
    let cvc = !is_vowel(w[n - 1]) && is_vowel(w[n - 2]) && !is_vowel(w[n - 3]);
    let vowel_groups = w
        .iter()
        .zip(std::iter::once(&b'_').chain(w.iter()))
        .filter(|&(&c, &prev)| is_vowel(c) && !is_vowel(prev))
        .count();
    cvc && vowel_groups == 1
}

/// The finite verb forms the `english!` macro can tell apart at compile time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FormClass {
    /// `run`, `have`, `do`: takes I, you, we, they and plurals.
    Base = 0,
    /// `runs`, `has`, `does`: takes he, she, it and singular nouns.
    ThirdSingular = 1,
    /// `am`
    Am = 2,
    /// `is`
    Is = 3,
    /// `are`
    Are = 4,
    /// `was`
    Was = 5,
    /// `were`
    Were = 6,
    /// Past tense and modals (`ran`, `walked`, `can`): agree with anything.
    Any = 7,
}

const MODALS: &[&str] = &[
    "can", "could", "will", "would", "shall", "should", "may", "might", "must", "did", "won",
    "ought",
];

/// Classify a finite verb by its spelling. `negated` means it was written with
/// `'t`, as in `don 't`, where the stem (`don`) is what you pass in.
pub const fn form_class(word: &str, negated: bool) -> FormClass {
    let w = word.as_bytes();
    if negated {
        return if eq_bytes(w, b"don") || eq_bytes(w, b"haven") {
            FormClass::Base
        } else if eq_bytes(w, b"doesn") || eq_bytes(w, b"hasn") {
            FormClass::ThirdSingular
        } else if eq_bytes(w, b"isn") {
            FormClass::Is
        } else if eq_bytes(w, b"aren") {
            FormClass::Are
        } else if eq_bytes(w, b"wasn") {
            FormClass::Was
        } else if eq_bytes(w, b"weren") {
            FormClass::Were
        } else {
            FormClass::Any
        };
    }
    if eq_bytes(w, b"am") {
        return FormClass::Am;
    }
    if eq_bytes(w, b"is") {
        return FormClass::Is;
    }
    if eq_bytes(w, b"are") {
        return FormClass::Are;
    }
    if eq_bytes(w, b"was") {
        return FormClass::Was;
    }
    if eq_bytes(w, b"were") {
        return FormClass::Were;
    }
    if eq_bytes(w, b"has") || eq_bytes(w, b"does") {
        return FormClass::ThirdSingular;
    }
    if contains(REGULAR_VERBS, w) {
        return FormClass::Base;
    }
    if contains(MODALS, w) || ends_with(w, "ed") {
        return FormClass::Any;
    }
    let mut i = 0;
    while i < IRREGULAR_VERBS.len() {
        let (base, past, _) = IRREGULAR_VERBS[i];
        // `put`, `cut`, `read`: base and past are spelled alike, so the
        // sentence might be past tense. Give it the benefit of the doubt.
        if alternatives_contain(past, word) {
            return FormClass::Any;
        }
        if eq_bytes(base.as_bytes(), w) {
            return FormClass::Base;
        }
        i += 1;
    }
    if ends_with(w, "s") && !ends_with(w, "ss") && !ends_with(w, "us") && w.len() > 2 {
        FormClass::ThirdSingular
    } else {
        FormClass::Base
    }
}

/// Copy the capitalisation of `like` onto `word`: `match_case("Go", "went")` is
/// `"Went"`.
pub fn match_case(like: &str, word: &str) -> String {
    let mut chars = like.chars();
    let first_upper = chars.next().is_some_and(char::is_uppercase);
    let all_upper =
        first_upper && like.chars().count() > 1 && like.chars().all(|c| !c.is_lowercase());
    if all_upper {
        word.to_uppercase()
    } else if first_upper {
        capitalize(word)
    } else {
        word.to_string()
    }
}

/// Upper-case the first letter.
pub fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn articles() {
        for w in [
            "apple",
            "hour",
            "honest",
            "MBA",
            "FBI",
            "X-ray",
            "8",
            "11",
            "18",
            "80",
            "uninvited",
            "umbrella",
            "egg",
        ] {
            assert!(wants_an(w), "{w} should take an");
        }
        for w in [
            "banana",
            "unicorn",
            "university",
            "European",
            "one",
            "URL",
            "NATO",
            "user",
            "7",
            "100",
            "uniform",
        ] {
            assert!(!wants_an(w), "{w} should take a");
        }
    }

    #[test]
    fn plurals() {
        assert_eq!(plural("dog"), "dogs");
        assert_eq!(plural("box"), "boxes");
        assert_eq!(plural("city"), "cities");
        assert_eq!(plural("day"), "days");
        assert_eq!(plural("Mouse"), "Mice");
        assert_eq!(plural("person"), "people");
        assert_eq!(plural("knife"), "knives");
        assert_eq!(regular_plural("child"), "childs");
    }

    #[test]
    fn verbs() {
        assert_eq!(third_person("run"), "runs");
        assert_eq!(third_person("go"), "goes");
        assert_eq!(third_person("watch"), "watches");
        assert_eq!(third_person("try"), "tries");
        assert_eq!(third_person("play"), "plays");
        assert_eq!(third_person("have"), "has");
        assert_eq!(past("walk"), "walked");
        assert_eq!(past("go"), "went");
        assert_eq!(participle("go"), "gone");
        assert_eq!(regular_past("go"), "goed");
        assert_eq!(regular_past("run"), "runned");
        assert_eq!(regular_past("stop"), "stopped");
        assert_eq!(regular_past("eat"), "eated");
        assert_eq!(regular_past("try"), "tried");
        assert_eq!(regular_past("bake"), "baked");
        assert_eq!(regular_past("begin"), "begined");
        assert_eq!(base_of_third_person("goes"), Some("go"));
        assert_eq!(base_of_third_person("watches"), Some("watch"));
    }

    #[test]
    fn form_classes() {
        assert_eq!(form_class("run", false), FormClass::Base);
        assert_eq!(form_class("runs", false), FormClass::ThirdSingular);
        assert_eq!(form_class("ran", false), FormClass::Any);
        assert_eq!(form_class("put", false), FormClass::Any);
        assert_eq!(form_class("kiss", false), FormClass::Base);
        assert_eq!(form_class("don", true), FormClass::Base);
        assert_eq!(form_class("doesn", true), FormClass::ThirdSingular);
        assert_eq!(form_class("is", false), FormClass::Is);
    }
}

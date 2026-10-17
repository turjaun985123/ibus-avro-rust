// Pattern data derived from jsAvroPhonetic by Rifat Nabi, Copyright (C) OmicronLab.
// Generated from ../../../avrolib.js; see LICENSE for the MPL-2.0 terms.
// Keep rule ordering and pattern ordering intact: both are semantically significant.

#[derive(Clone, Copy, Debug)]
pub(crate) enum Position { Prefix, Suffix }

#[derive(Clone, Copy, Debug)]
pub(crate) enum Scope { Punctuation, Vowel, Consonant, Exact }

#[derive(Clone, Copy, Debug)]
pub(crate) struct Condition {
    pub(crate) position: Position,
    pub(crate) scope: Scope,
    pub(crate) value: &'static str,
    pub(crate) negative: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rule {
    pub(crate) conditions: &'static [Condition],
    pub(crate) replace: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Pattern {
    pub(crate) find: &'static str,
    pub(crate) replace: &'static str,
    pub(crate) rules: &'static [Rule],
}

pub(crate) const VOWELS: &str = "aeiou";
pub(crate) const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";
pub(crate) const CASE_SENSITIVE: &str = "oiudgjnrstyz";

pub(crate) static PATTERNS: &[Pattern] = &[
    Pattern {
        find: r"bhl",
        replace: r"ভ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"psh",
        replace: r"পশ",
        rules: &[
        ],
    },
    Pattern {
        find: r"bdh",
        replace: r"ব্ধ",
        rules: &[
        ],
    },
    Pattern {
        find: r"bj",
        replace: r"ব্জ",
        rules: &[
        ],
    },
    Pattern {
        find: r"bd",
        replace: r"ব্দ",
        rules: &[
        ],
    },
    Pattern {
        find: r"bb",
        replace: r"ব্ব",
        rules: &[
        ],
    },
    Pattern {
        find: r"bl",
        replace: r"ব্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"bh",
        replace: r"ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"vl",
        replace: r"ভ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"b",
        replace: r"ব",
        rules: &[
        ],
    },
    Pattern {
        find: r"v",
        replace: r"ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"cNG",
        replace: r"চ্ঞ",
        rules: &[
        ],
    },
    Pattern {
        find: r"cch",
        replace: r"চ্ছ",
        rules: &[
        ],
    },
    Pattern {
        find: r"cc",
        replace: r"চ্চ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ch",
        replace: r"ছ",
        rules: &[
        ],
    },
    Pattern {
        find: r"c",
        replace: r"চ",
        rules: &[
        ],
    },
    Pattern {
        find: r"dhn",
        replace: r"ধ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"dhm",
        replace: r"ধ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"dgh",
        replace: r"দ্ঘ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ddh",
        replace: r"দ্ধ",
        rules: &[
        ],
    },
    Pattern {
        find: r"dbh",
        replace: r"দ্ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"dv",
        replace: r"দ্ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"dm",
        replace: r"দ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"DD",
        replace: r"ড্ড",
        rules: &[
        ],
    },
    Pattern {
        find: r"Dh",
        replace: r"ঢ",
        rules: &[
        ],
    },
    Pattern {
        find: r"dh",
        replace: r"ধ",
        rules: &[
        ],
    },
    Pattern {
        find: r"dg",
        replace: r"দ্গ",
        rules: &[
        ],
    },
    Pattern {
        find: r"dd",
        replace: r"দ্দ",
        rules: &[
        ],
    },
    Pattern {
        find: r"D",
        replace: r"ড",
        rules: &[
        ],
    },
    Pattern {
        find: r"d",
        replace: r"দ",
        rules: &[
        ],
    },
    Pattern {
        find: r"...",
        replace: r"...",
        rules: &[
        ],
    },
    Pattern {
        find: r".`",
        replace: r".",
        rules: &[
        ],
    },
    Pattern {
        find: r"..",
        replace: r"।।",
        rules: &[
        ],
    },
    Pattern {
        find: r".",
        replace: r"।",
        rules: &[
        ],
    },
    Pattern {
        find: r"ghn",
        replace: r"ঘ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ghn",
        replace: r"ঘ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"gdh",
        replace: r"গ্ধ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Gdh",
        replace: r"গ্ধ",
        rules: &[
        ],
    },
    Pattern {
        find: r"gN",
        replace: r"গ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"GN",
        replace: r"গ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"gn",
        replace: r"গ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"Gn",
        replace: r"গ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"gm",
        replace: r"গ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"Gm",
        replace: r"গ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"gl",
        replace: r"গ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"Gl",
        replace: r"গ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"gg",
        replace: r"জ্ঞ",
        rules: &[
        ],
    },
    Pattern {
        find: r"GG",
        replace: r"জ্ঞ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Gg",
        replace: r"জ্ঞ",
        rules: &[
        ],
    },
    Pattern {
        find: r"gG",
        replace: r"জ্ঞ",
        rules: &[
        ],
    },
    Pattern {
        find: r"gh",
        replace: r"ঘ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Gh",
        replace: r"ঘ",
        rules: &[
        ],
    },
    Pattern {
        find: r"g",
        replace: r"গ",
        rules: &[
        ],
    },
    Pattern {
        find: r"G",
        replace: r"গ",
        rules: &[
        ],
    },
    Pattern {
        find: r"hN",
        replace: r"হ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"hn",
        replace: r"হ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"hm",
        replace: r"হ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"hl",
        replace: r"হ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"h",
        replace: r"হ",
        rules: &[
        ],
    },
    Pattern {
        find: r"jjh",
        replace: r"জ্ঝ",
        rules: &[
        ],
    },
    Pattern {
        find: r"jNG",
        replace: r"জ্ঞ",
        rules: &[
        ],
    },
    Pattern {
        find: r"jh",
        replace: r"ঝ",
        rules: &[
        ],
    },
    Pattern {
        find: r"jj",
        replace: r"জ্জ",
        rules: &[
        ],
    },
    Pattern {
        find: r"j",
        replace: r"জ",
        rules: &[
        ],
    },
    Pattern {
        find: r"J",
        replace: r"জ",
        rules: &[
        ],
    },
    Pattern {
        find: r"kkhN",
        replace: r"ক্ষ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"kShN",
        replace: r"ক্ষ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"kkhm",
        replace: r"ক্ষ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"kShm",
        replace: r"ক্ষ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"kxN",
        replace: r"ক্ষ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"kxm",
        replace: r"ক্ষ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"kkh",
        replace: r"ক্ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"kSh",
        replace: r"ক্ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ksh",
        replace: r"কশ",
        rules: &[
        ],
    },
    Pattern {
        find: r"kx",
        replace: r"ক্ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"kk",
        replace: r"ক্ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"kT",
        replace: r"ক্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"kt",
        replace: r"ক্ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"kl",
        replace: r"ক্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"ks",
        replace: r"ক্স",
        rules: &[
        ],
    },
    Pattern {
        find: r"kh",
        replace: r"খ",
        rules: &[
        ],
    },
    Pattern {
        find: r"k",
        replace: r"ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"lbh",
        replace: r"ল্ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ldh",
        replace: r"ল্ধ",
        rules: &[
        ],
    },
    Pattern {
        find: r"lkh",
        replace: r"লখ",
        rules: &[
        ],
    },
    Pattern {
        find: r"lgh",
        replace: r"লঘ",
        rules: &[
        ],
    },
    Pattern {
        find: r"lph",
        replace: r"লফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"lk",
        replace: r"ল্ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"lg",
        replace: r"ল্গ",
        rules: &[
        ],
    },
    Pattern {
        find: r"lT",
        replace: r"ল্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"lD",
        replace: r"ল্ড",
        rules: &[
        ],
    },
    Pattern {
        find: r"lp",
        replace: r"ল্প",
        rules: &[
        ],
    },
    Pattern {
        find: r"lv",
        replace: r"ল্ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"lm",
        replace: r"ল্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"ll",
        replace: r"ল্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"lb",
        replace: r"ল্ব",
        rules: &[
        ],
    },
    Pattern {
        find: r"l",
        replace: r"ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"mth",
        replace: r"ম্থ",
        rules: &[
        ],
    },
    Pattern {
        find: r"mph",
        replace: r"ম্ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"mbh",
        replace: r"ম্ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"mpl",
        replace: r"মপ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"mn",
        replace: r"ম্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"mp",
        replace: r"ম্প",
        rules: &[
        ],
    },
    Pattern {
        find: r"mv",
        replace: r"ম্ভ",
        rules: &[
        ],
    },
    Pattern {
        find: r"mm",
        replace: r"ম্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"ml",
        replace: r"ম্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"mb",
        replace: r"ম্ব",
        rules: &[
        ],
    },
    Pattern {
        find: r"mf",
        replace: r"ম্ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"m",
        replace: r"ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"0",
        replace: r"০",
        rules: &[
        ],
    },
    Pattern {
        find: r"1",
        replace: r"১",
        rules: &[
        ],
    },
    Pattern {
        find: r"2",
        replace: r"২",
        rules: &[
        ],
    },
    Pattern {
        find: r"3",
        replace: r"৩",
        rules: &[
        ],
    },
    Pattern {
        find: r"4",
        replace: r"৪",
        rules: &[
        ],
    },
    Pattern {
        find: r"5",
        replace: r"৫",
        rules: &[
        ],
    },
    Pattern {
        find: r"6",
        replace: r"৬",
        rules: &[
        ],
    },
    Pattern {
        find: r"7",
        replace: r"৭",
        rules: &[
        ],
    },
    Pattern {
        find: r"8",
        replace: r"৮",
        rules: &[
        ],
    },
    Pattern {
        find: r"9",
        replace: r"৯",
        rules: &[
        ],
    },
    Pattern {
        find: r"NgkSh",
        replace: r"ঙ্ক্ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngkkh",
        replace: r"ঙ্ক্ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"NGch",
        replace: r"ঞ্ছ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Nggh",
        replace: r"ঙ্ঘ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngkh",
        replace: r"ঙ্খ",
        rules: &[
        ],
    },
    Pattern {
        find: r"NGjh",
        replace: r"ঞ্ঝ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngOU",
        replace: r"ঙ্গৌ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngOI",
        replace: r"ঙ্গৈ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngkx",
        replace: r"ঙ্ক্ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"NGc",
        replace: r"ঞ্চ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nch",
        replace: r"ঞ্ছ",
        rules: &[
        ],
    },
    Pattern {
        find: r"njh",
        replace: r"ঞ্ঝ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngh",
        replace: r"ঙ্ঘ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngk",
        replace: r"ঙ্ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngx",
        replace: r"ঙ্ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngg",
        replace: r"ঙ্গ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngm",
        replace: r"ঙ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"NGj",
        replace: r"ঞ্জ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ndh",
        replace: r"ন্ধ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nTh",
        replace: r"ন্ঠ",
        rules: &[
        ],
    },
    Pattern {
        find: r"NTh",
        replace: r"ণ্ঠ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nth",
        replace: r"ন্থ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nkh",
        replace: r"ঙ্খ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngo",
        replace: r"ঙ্গ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nga",
        replace: r"ঙ্গা",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngi",
        replace: r"ঙ্গি",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngI",
        replace: r"ঙ্গী",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngu",
        replace: r"ঙ্গু",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngU",
        replace: r"ঙ্গূ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nge",
        replace: r"ঙ্গে",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngO",
        replace: r"ঙ্গো",
        rules: &[
        ],
    },
    Pattern {
        find: r"NDh",
        replace: r"ণ্ঢ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nsh",
        replace: r"নশ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ngr",
        replace: r"ঙর",
        rules: &[
        ],
    },
    Pattern {
        find: r"NGr",
        replace: r"ঞর",
        rules: &[
        ],
    },
    Pattern {
        find: r"ngr",
        replace: r"ংর",
        rules: &[
        ],
    },
    Pattern {
        find: r"nj",
        replace: r"ঞ্জ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Ng",
        replace: r"ঙ",
        rules: &[
        ],
    },
    Pattern {
        find: r"NG",
        replace: r"ঞ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nk",
        replace: r"ঙ্ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"ng",
        replace: r"ং",
        rules: &[
        ],
    },
    Pattern {
        find: r"nn",
        replace: r"ন্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"NN",
        replace: r"ণ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Nn",
        replace: r"ণ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"nm",
        replace: r"ন্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"Nm",
        replace: r"ণ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"nd",
        replace: r"ন্দ",
        rules: &[
        ],
    },
    Pattern {
        find: r"nT",
        replace: r"ন্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"NT",
        replace: r"ণ্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"nD",
        replace: r"ন্ড",
        rules: &[
        ],
    },
    Pattern {
        find: r"ND",
        replace: r"ণ্ড",
        rules: &[
        ],
    },
    Pattern {
        find: r"nt",
        replace: r"ন্ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"ns",
        replace: r"ন্স",
        rules: &[
        ],
    },
    Pattern {
        find: r"nc",
        replace: r"ঞ্চ",
        rules: &[
        ],
    },
    Pattern {
        find: r"n",
        replace: r"ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"N",
        replace: r"ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"OI`",
        replace: r"ৈ",
        rules: &[
        ],
    },
    Pattern {
        find: r"OU`",
        replace: r"ৌ",
        rules: &[
        ],
    },
    Pattern {
        find: r"O`",
        replace: r"ো",
        rules: &[
        ],
    },
    Pattern {
        find: r"OI",
        replace: r"ৈ",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                ],
                replace: r"ঐ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                ],
                replace: r"ঐ",
            },
        ],
    },
    Pattern {
        find: r"OU",
        replace: r"ৌ",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                ],
                replace: r"ঔ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                ],
                replace: r"ঔ",
            },
        ],
    },
    Pattern {
        find: r"O",
        replace: r"ো",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                ],
                replace: r"ও",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                ],
                replace: r"ও",
            },
        ],
    },
    Pattern {
        find: r"phl",
        replace: r"ফ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"pT",
        replace: r"প্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"pt",
        replace: r"প্ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"pn",
        replace: r"প্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"pp",
        replace: r"প্প",
        rules: &[
        ],
    },
    Pattern {
        find: r"pl",
        replace: r"প্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"ps",
        replace: r"প্স",
        rules: &[
        ],
    },
    Pattern {
        find: r"ph",
        replace: r"ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"fl",
        replace: r"ফ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"f",
        replace: r"ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"p",
        replace: r"প",
        rules: &[
        ],
    },
    Pattern {
        find: r"rri`",
        replace: r"ৃ",
        rules: &[
        ],
    },
    Pattern {
        find: r"rri",
        replace: r"ৃ",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                ],
                replace: r"ঋ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                ],
                replace: r"ঋ",
            },
        ],
    },
    Pattern {
        find: r"rrZ",
        replace: r"রর‍্য",
        rules: &[
        ],
    },
    Pattern {
        find: r"rry",
        replace: r"রর‍্য",
        rules: &[
        ],
    },
    Pattern {
        find: r"rZ",
        replace: r"র‍্য",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: false },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"r", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"y", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"w", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"x", negative: true },
                ],
                replace: r"্র্য",
            },
        ],
    },
    Pattern {
        find: r"ry",
        replace: r"র‍্য",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: false },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"r", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"y", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"w", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"x", negative: true },
                ],
                replace: r"্র্য",
            },
        ],
    },
    Pattern {
        find: r"rr",
        replace: r"রর",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Vowel, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"r", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Punctuation, value: r"", negative: true },
                ],
                replace: r"র্",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: false },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"r", negative: true },
                ],
                replace: r"্রর",
            },
        ],
    },
    Pattern {
        find: r"Rg",
        replace: r"ড়্গ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Rh",
        replace: r"ঢ়",
        rules: &[
        ],
    },
    Pattern {
        find: r"R",
        replace: r"ড়",
        rules: &[
        ],
    },
    Pattern {
        find: r"r",
        replace: r"র",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: false },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"r", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"y", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"w", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"x", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"Z", negative: true },
                ],
                replace: r"্র",
            },
        ],
    },
    Pattern {
        find: r"shch",
        replace: r"শ্ছ",
        rules: &[
        ],
    },
    Pattern {
        find: r"ShTh",
        replace: r"ষ্ঠ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Shph",
        replace: r"ষ্ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Sch",
        replace: r"শ্ছ",
        rules: &[
        ],
    },
    Pattern {
        find: r"skl",
        replace: r"স্ক্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"skh",
        replace: r"স্খ",
        rules: &[
        ],
    },
    Pattern {
        find: r"sth",
        replace: r"স্থ",
        rules: &[
        ],
    },
    Pattern {
        find: r"sph",
        replace: r"স্ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"shc",
        replace: r"শ্চ",
        rules: &[
        ],
    },
    Pattern {
        find: r"sht",
        replace: r"শ্ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"shn",
        replace: r"শ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"shm",
        replace: r"শ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"shl",
        replace: r"শ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"Shk",
        replace: r"ষ্ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"ShT",
        replace: r"ষ্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"ShN",
        replace: r"ষ্ণ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Shp",
        replace: r"ষ্প",
        rules: &[
        ],
    },
    Pattern {
        find: r"Shf",
        replace: r"ষ্ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Shm",
        replace: r"ষ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"spl",
        replace: r"স্প্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"sk",
        replace: r"স্ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"Sc",
        replace: r"শ্চ",
        rules: &[
        ],
    },
    Pattern {
        find: r"sT",
        replace: r"স্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"st",
        replace: r"স্ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"sn",
        replace: r"স্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"sp",
        replace: r"স্প",
        rules: &[
        ],
    },
    Pattern {
        find: r"sf",
        replace: r"স্ফ",
        rules: &[
        ],
    },
    Pattern {
        find: r"sm",
        replace: r"স্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"sl",
        replace: r"স্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"sh",
        replace: r"শ",
        rules: &[
        ],
    },
    Pattern {
        find: r"Sc",
        replace: r"শ্চ",
        rules: &[
        ],
    },
    Pattern {
        find: r"St",
        replace: r"শ্ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"Sn",
        replace: r"শ্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"Sm",
        replace: r"শ্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"Sl",
        replace: r"শ্ল",
        rules: &[
        ],
    },
    Pattern {
        find: r"Sh",
        replace: r"ষ",
        rules: &[
        ],
    },
    Pattern {
        find: r"s",
        replace: r"স",
        rules: &[
        ],
    },
    Pattern {
        find: r"S",
        replace: r"শ",
        rules: &[
        ],
    },
    Pattern {
        find: r"oo`",
        replace: r"ু",
        rules: &[
        ],
    },
    Pattern {
        find: r"oo",
        replace: r"ু",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"উ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"উ",
            },
        ],
    },
    Pattern {
        find: r"o`",
        replace: r"",
        rules: &[
        ],
    },
    Pattern {
        find: r"oZ",
        replace: r"অ্য",
        rules: &[
        ],
    },
    Pattern {
        find: r"o",
        replace: r"",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Vowel, value: r"", negative: false },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"o", negative: true },
                ],
                replace: r"ও",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Vowel, value: r"", negative: false },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"o", negative: false },
                ],
                replace: r"অ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                ],
                replace: r"অ",
            },
        ],
    },
    Pattern {
        find: r"tth",
        replace: r"ত্থ",
        rules: &[
        ],
    },
    Pattern {
        find: r"t``",
        replace: r"ৎ",
        rules: &[
        ],
    },
    Pattern {
        find: r"TT",
        replace: r"ট্ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"Tm",
        replace: r"ট্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"Th",
        replace: r"ঠ",
        rules: &[
        ],
    },
    Pattern {
        find: r"tn",
        replace: r"ত্ন",
        rules: &[
        ],
    },
    Pattern {
        find: r"tm",
        replace: r"ত্ম",
        rules: &[
        ],
    },
    Pattern {
        find: r"th",
        replace: r"থ",
        rules: &[
        ],
    },
    Pattern {
        find: r"tt",
        replace: r"ত্ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"T",
        replace: r"ট",
        rules: &[
        ],
    },
    Pattern {
        find: r"t",
        replace: r"ত",
        rules: &[
        ],
    },
    Pattern {
        find: r"aZ",
        replace: r"অ্যা",
        rules: &[
        ],
    },
    Pattern {
        find: r"AZ",
        replace: r"অ্যা",
        rules: &[
        ],
    },
    Pattern {
        find: r"a`",
        replace: r"া",
        rules: &[
        ],
    },
    Pattern {
        find: r"A`",
        replace: r"া",
        rules: &[
        ],
    },
    Pattern {
        find: r"a",
        replace: r"া",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"আ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"a", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"য়া",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Exact, value: r"a", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"আ",
            },
        ],
    },
    Pattern {
        find: r"i`",
        replace: r"ি",
        rules: &[
        ],
    },
    Pattern {
        find: r"i",
        replace: r"ি",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ই",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ই",
            },
        ],
    },
    Pattern {
        find: r"I`",
        replace: r"ী",
        rules: &[
        ],
    },
    Pattern {
        find: r"I",
        replace: r"ী",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ঈ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ঈ",
            },
        ],
    },
    Pattern {
        find: r"u`",
        replace: r"ু",
        rules: &[
        ],
    },
    Pattern {
        find: r"u",
        replace: r"ু",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"উ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"উ",
            },
        ],
    },
    Pattern {
        find: r"U`",
        replace: r"ূ",
        rules: &[
        ],
    },
    Pattern {
        find: r"U",
        replace: r"ূ",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ঊ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ঊ",
            },
        ],
    },
    Pattern {
        find: r"ee`",
        replace: r"ী",
        rules: &[
        ],
    },
    Pattern {
        find: r"ee",
        replace: r"ী",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ঈ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"ঈ",
            },
        ],
    },
    Pattern {
        find: r"e`",
        replace: r"ে",
        rules: &[
        ],
    },
    Pattern {
        find: r"e",
        replace: r"ে",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"এ",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Exact, value: r"`", negative: true },
                ],
                replace: r"এ",
            },
        ],
    },
    Pattern {
        find: r"z",
        replace: r"য",
        rules: &[
        ],
    },
    Pattern {
        find: r"Z",
        replace: r"্য",
        rules: &[
        ],
    },
    Pattern {
        find: r"y",
        replace: r"্য",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: true },
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: true },
                ],
                replace: r"য়",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                ],
                replace: r"ইয়",
            },
        ],
    },
    Pattern {
        find: r"Y",
        replace: r"য়",
        rules: &[
        ],
    },
    Pattern {
        find: r"q",
        replace: r"ক",
        rules: &[
        ],
    },
    Pattern {
        find: r"w",
        replace: r"ও",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                    Condition { position: Position::Suffix, scope: Scope::Vowel, value: r"", negative: false },
                ],
                replace: r"ওয়",
            },
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Consonant, value: r"", negative: false },
                ],
                replace: r"্ব",
            },
        ],
    },
    Pattern {
        find: r"x",
        replace: r"ক্স",
        rules: &[
            Rule {
                conditions: &[
                    Condition { position: Position::Prefix, scope: Scope::Punctuation, value: r"", negative: false },
                ],
                replace: r"এক্স",
            },
        ],
    },
    Pattern {
        find: r":`",
        replace: r":",
        rules: &[
        ],
    },
    Pattern {
        find: r":",
        replace: r"ঃ",
        rules: &[
        ],
    },
    Pattern {
        find: r"^`",
        replace: r"^",
        rules: &[
        ],
    },
    Pattern {
        find: r"^",
        replace: r"ঁ",
        rules: &[
        ],
    },
    Pattern {
        find: r",,",
        replace: r"্‌",
        rules: &[
        ],
    },
    Pattern {
        find: r",",
        replace: r",",
        rules: &[
        ],
    },
    Pattern {
        find: r"$",
        replace: r"৳",
        rules: &[
        ],
    },
    Pattern {
        find: r"`",
        replace: r"",
        rules: &[
        ],
    },
];

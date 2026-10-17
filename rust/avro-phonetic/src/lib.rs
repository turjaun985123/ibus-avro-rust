// Derived from jsAvroPhonetic by Rifat Nabi; original code copyright OmicronLab.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

//! Avro phonetic transliteration and suggestion engine.

mod dictionary;
mod engine;
mod patterns;
mod regex_patterns;
mod suggestions;

pub use engine::{EngineSession, EngineSettings, EngineView, KeyEvent, KeyResult, LookupOrientation};
pub use suggestions::{CandidateList, Preferences, SuggestionEngine};

use patterns::{Condition, Pattern, Position, Scope, CASE_SENSITIVE, CONSONANTS, PATTERNS, VOWELS};

const OPTIONAL_SUFFIX: &str = "(্[যবম])?(্?)([ঃঁ]?)";

/// Transliterate Latin Avro phonetic input into Bengali text.
///
/// Pattern and conditional-rule ordering matches the existing JavaScript engine.
pub fn parse(input: &str) -> String {
    let fixed = fix_string(input);
    parse_patterns(&fixed, PATTERNS, "")
}

/// Build a Bengali regular expression used to find matching dictionary entries.
pub(crate) fn parse_dictionary_pattern(input: &str) -> String {
    let mut fixed = String::with_capacity(input.len());
    for character in input.chars() {
        let lowercase: String = character.to_lowercase().collect();
        if !regex_patterns::IGNORE.contains(lowercase.as_str()) {
            fixed.push_str(&lowercase);
        }
    }
    parse_patterns(&fixed, regex_patterns::PATTERNS, OPTIONAL_SUFFIX)
}

fn fix_string(input: &str) -> String {
    let mut fixed = String::with_capacity(input.len());
    for character in input.chars() {
        let lowercase: String = character.to_lowercase().collect();
        if CASE_SENSITIVE.contains(lowercase.as_str()) {
            fixed.push(character);
        } else {
            fixed.push_str(&lowercase);
        }
    }
    fixed
}

fn parse_patterns(input: &str, patterns: &[Pattern], replacement_suffix: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;

    while cursor < chars.len() {
        let mut matched = false;

        for pattern in patterns {
            let pattern_len = pattern.find.chars().count();
            let end = cursor + pattern_len;
            if end > chars.len() || !matches_pattern(&chars[cursor..end], pattern) {
                continue;
            }

            let replacement = pattern
                .rules
                .iter()
                .find(|rule| {
                    rule.conditions
                        .iter()
                        .all(|condition| condition_matches(condition, &chars, cursor, end))
                })
                .map_or(pattern.replace, |rule| rule.replace);

            output.push_str(replacement);
            output.push_str(replacement_suffix);
            cursor = end;
            matched = true;
            break;
        }

        if !matched {
            output.push(chars[cursor]);
            cursor += 1;
        }
    }

    output
}

fn matches_pattern(chars: &[char], pattern: &Pattern) -> bool {
    chars.iter().copied().eq(pattern.find.chars())
}

fn condition_matches(condition: &Condition, chars: &[char], start: usize, end: usize) -> bool {
    let matches = match condition.scope {
        Scope::Punctuation => match condition.position {
            Position::Prefix if start == 0 => true,
            Position::Suffix if end == chars.len() => true,
            Position::Prefix => chars
                .get(start - 1)
                .is_some_and(|character| is_punctuation(*character)),
            Position::Suffix => chars
                .get(end)
                .is_some_and(|character| is_punctuation(*character)),
        },
        Scope::Vowel => adjacent_char(condition.position, chars, start, end).is_some_and(is_vowel),
        Scope::Consonant => {
            adjacent_char(condition.position, chars, start, end).is_some_and(is_consonant)
        }
        Scope::Exact => {
            let value: Vec<char> = condition.value.chars().collect();
            let range = match condition.position {
                Position::Prefix => start.checked_sub(value.len()).map(|begin| begin..start),
                Position::Suffix => Some(end..end.saturating_add(value.len())),
            };
            range
                .and_then(|range| chars.get(range))
                .is_some_and(|candidate| candidate.iter().copied().eq(value))
        }
    };

    if condition.negative {
        !matches
    } else {
        matches
    }
}

fn adjacent_char(position: Position, chars: &[char], start: usize, end: usize) -> Option<char> {
    match position {
        Position::Prefix => start.checked_sub(1).and_then(|index| chars.get(index)).copied(),
        Position::Suffix => chars.get(end).copied(),
    }
}

fn is_vowel(character: char) -> bool {
    let lowercase: String = character.to_lowercase().collect();
    VOWELS.contains(lowercase.as_str())
}

fn is_consonant(character: char) -> bool {
    let lowercase: String = character.to_lowercase().collect();
    CONSONANTS.contains(lowercase.as_str())
}

fn is_punctuation(character: char) -> bool {
    !is_vowel(character) && !is_consonant(character)
}

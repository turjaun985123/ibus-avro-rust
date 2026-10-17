use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CACHE_LIMIT: usize = 256;
const KHANDATTA: char = '\u{09ce}';
const ANUSVARA: char = '\u{0982}';
const NGA: char = '\u{0999}';
const TA: char = '\u{09a4}';
const YA: char = '\u{09df}';

static AUTOCORRECT_DATA: std::sync::OnceLock<HashMap<String, String>> =
    std::sync::OnceLock::new();
static SUFFIX_DATA: std::sync::OnceLock<HashMap<String, String>> = std::sync::OnceLock::new();

fn autocorrect_data() -> &'static HashMap<String, String> {
    AUTOCORRECT_DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/autocorrect.json"))
            .expect("embedded Avro autocorrect data must be valid JSON")
    })
}

fn suffix_data() -> &'static HashMap<String, String> {
    SUFFIX_DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/suffixes.json"))
            .expect("embedded Avro suffix data must be valid JSON")
    })
}

#[derive(Clone, Debug)]
pub struct Preferences {
    pub dictionary_enabled: bool,
    pub candidate_limit: usize,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            dictionary_enabled: true,
            candidate_limit: 15,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateList {
    pub words: Vec<String>,
    pub previous_selection: usize,
}

#[derive(Default)]
struct WordParts {
    leading: String,
    middle: String,
    trailing: String,
}

#[derive(Clone)]
struct SuffixMemory {
    base_candidate: String,
    phonetic_key: String,
}

struct Autocorrect {
    corrected: Option<String>,
    exact: bool,
}

/// Builds Avro dictionary candidates, suffix forms, and remembered selections.
pub struct SuggestionEngine {
    preferences: Preferences,
    phonetic_cache: HashMap<String, Vec<String>>,
    regex_cache: HashMap<String, Regex>,
    candidate_selections: HashMap<String, String>,
    suffix_memory: HashMap<String, SuffixMemory>,
}

impl Default for SuggestionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SuggestionEngine {
    pub fn new() -> Self {
        Self {
            preferences: Preferences::default(),
            phonetic_cache: HashMap::new(),
            regex_cache: HashMap::new(),
            candidate_selections: load_candidate_selections(),
            suffix_memory: HashMap::new(),
        }
    }

    pub fn preferences(&self) -> &Preferences {
        &self.preferences
    }

    pub fn set_preferences(&mut self, preferences: Preferences) {
        self.preferences = Preferences {
            dictionary_enabled: preferences.dictionary_enabled,
            candidate_limit: preferences.candidate_limit.max(1),
        };
    }

    pub fn suggest(&mut self, input: &str) -> CandidateList {
        let parts = separate_padding(input);
        let leading = crate::parse(&parts.leading);
        let trailing = crate::parse(&parts.trailing);
        let phonetic = crate::parse(&parts.middle);

        if !self.preferences.dictionary_enabled {
            return CandidateList {
                words: vec![format!("{leading}{phonetic}{trailing}")],
                previous_selection: 0,
            };
        }

        let word_key = parts.middle.to_lowercase();
        let mut dictionary_words = self.dictionary_suggestions(&word_key);
        let autocorrect = self.autocorrect(input, &parts.middle);
        if let Some(corrected) = &autocorrect.corrected {
            if !autocorrect.exact {
                push_unique(&mut dictionary_words, corrected.clone());
            }
        }

        if !self.phonetic_cache.contains_key(&word_key) && !dictionary_words.is_empty() {
            trim_cache(&mut self.phonetic_cache);
            self.phonetic_cache
                .insert(word_key.clone(), dictionary_words.clone());
        }

        let suffix_words = self.add_suffixes(&word_key);
        let mut words = Vec::new();
        if let Some(corrected) = &autocorrect.corrected {
            push_unique(&mut words, corrected.clone());
        }
        let mut ranked = suffix_words;
        ranked.sort_by_key(|candidate| damerau_levenshtein(&phonetic, candidate));
        for candidate in ranked {
            push_unique(&mut words, candidate);
        }
        push_unique(&mut words, phonetic.clone());

        for word in &mut words {
            if !(autocorrect.exact && autocorrect.corrected.as_deref() == Some(word.as_str())) {
                *word = format!("{leading}{word}{trailing}");
            }
        }
        words.truncate(self.preferences.candidate_limit.max(1));
        let previous = self.previous_selection(&parts.middle, &words);
        CandidateList {
            words,
            previous_selection: previous,
        }
    }

    /// Remember the highlighted candidate for this input until the next commit.
    pub fn update_candidate_selection(&mut self, input: &str, candidate: &str) {
        if !self.preferences.dictionary_enabled {
            return;
        }
        let parts = separate_padding(input);
        self.candidate_selections
            .insert(parts.middle, candidate.to_owned());
    }

    /// Persist the selected candidate and any learned suffix base candidate.
    pub fn string_committed(&mut self, _input: &str, candidate: &str) {
        if !self.preferences.dictionary_enabled {
            return;
        }

        if let Some(memory) = self.suffix_memory.get(candidate) {
            self.candidate_selections
                .entry(memory.phonetic_key.clone())
                .or_insert_with(|| memory.base_candidate.clone());
        }
        save_candidate_selections(&self.candidate_selections);
    }

    fn dictionary_suggestions(&mut self, word_key: &str) -> Vec<String> {
        if let Some(cached) = self.phonetic_cache.get(word_key) {
            return cached.clone();
        }
        crate::dictionary::search(word_key, &mut self.regex_cache)
    }

    fn autocorrect(&self, whole_word: &str, middle: &str) -> Autocorrect {
        let dictionary = autocorrect_data();
        if let Some(corrected) = dictionary.get(whole_word) {
            if corrected == whole_word {
                return Autocorrect {
                    corrected: Some(corrected.clone()),
                    exact: true,
                };
            }
            return Autocorrect {
                corrected: Some(crate::parse(corrected)),
                exact: false,
            };
        }

        let normalized = fix_case(middle);
        Autocorrect {
            corrected: dictionary.get(&normalized).map(|value| crate::parse(value)),
            exact: false,
        }
    }

    fn add_suffixes(&mut self, word: &str) -> Vec<String> {
        let mut candidates = self.phonetic_cache.get(word).cloned().unwrap_or_default();
        self.suffix_memory.clear();
        let chars: Vec<char> = word.chars().collect();
        if chars.len() < 2 {
            return candidates;
        }

        for suffix_start in 1..chars.len() {
            let suffix_key: String = chars[suffix_start..].iter().collect();
            let Some(suffix) = suffix_data().get(&suffix_key) else {
                continue;
            };
            let phonetic_key: String = chars[..suffix_start].iter().collect();
            let Some(base_candidates) = self.phonetic_cache.get(&phonetic_key).cloned() else {
                continue;
            };
            let suffix_first = suffix.chars().next();

            for base_candidate in base_candidates {
                let Some(last) = base_candidate.chars().last() else {
                    continue;
                };
                let (combined, remember) = if is_bengali_vowel(last) && suffix_first.is_some_and(is_kar) {
                    (format!("{base_candidate}{}{suffix}", YA), true)
                } else if last == KHANDATTA {
                    let stem = remove_last_char(&base_candidate);
                    (format!("{stem}{}{suffix}", TA), true)
                } else if last == ANUSVARA {
                    let stem = remove_last_char(&base_candidate);
                    (format!("{stem}{}{suffix}", NGA), false)
                } else {
                    (format!("{base_candidate}{suffix}"), true)
                };
                push_unique(&mut candidates, combined.clone());
                if remember {
                    self.suffix_memory.entry(combined).or_insert(SuffixMemory {
                        base_candidate,
                        phonetic_key: phonetic_key.clone(),
                    });
                }
            }
        }
        candidates
    }

    fn previous_selection(&mut self, word: &str, candidates: &[String]) -> usize {
        let mut selected = self.candidate_selections.get(word).cloned();
        let chars: Vec<char> = word.chars().collect();

        if selected.is_none() && chars.len() >= 2 {
            for suffix_len in 1..chars.len() {
                let split = chars.len() - suffix_len;
                let suffix_key: String = chars[split..].iter().collect::<String>().to_lowercase();
                let Some(suffix) = suffix_data().get(&suffix_key) else {
                    continue;
                };
                let phonetic_key: String = chars[..split].iter().collect();
                let Some(base) = self.candidate_selections.get(&phonetic_key).cloned() else {
                    continue;
                };
                let Some(last) = base.chars().last() else {
                    continue;
                };
                let suffix_first = suffix.chars().next();
                let combined = if is_bengali_vowel(last) && suffix_first.is_some_and(is_kar) {
                    format!("{base}{}{suffix}", YA)
                } else if last == KHANDATTA {
                    format!("{}{TA}{suffix}", remove_last_char(&base))
                } else if last == ANUSVARA {
                    format!("{}{NGA}{suffix}", remove_last_char(&base))
                } else {
                    format!("{base}{suffix}")
                };
                self.candidate_selections
                    .insert(word.to_owned(), combined.clone());
                selected = Some(combined);
                break;
            }
        }

        selected
            .as_deref()
            .and_then(|selected| candidates.iter().position(|candidate| candidate == selected))
            .unwrap_or(0)
    }
}

fn fix_case(input: &str) -> String {
    let mut fixed = String::with_capacity(input.len());
    for character in input.chars() {
        let lowercase: String = character.to_lowercase().collect();
        if crate::patterns::CASE_SENSITIVE.contains(lowercase.as_str()) {
            fixed.push(character);
        } else {
            fixed.push_str(&lowercase);
        }
    }
    fixed
}

fn push_unique(words: &mut Vec<String>, candidate: String) {
    if !words.iter().any(|word| word == &candidate) {
        words.push(candidate);
    }
}

fn trim_cache<T>(cache: &mut HashMap<String, T>) {
    if cache.len() >= CACHE_LIMIT {
        cache.clear();
    }
}

fn damerau_levenshtein(a: &str, b: &str) -> usize {
    let left: Vec<char> = a.chars().collect();
    let right: Vec<char> = b.chars().collect();
    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }

    let columns = right.len() + 1;
    let mut distance = vec![0; (left.len() + 1) * columns];
    for i in 0..=left.len() {
        distance[i * columns] = i;
    }
    for j in 0..=right.len() {
        distance[j] = j;
    }

    for i in 1..=left.len() {
        for j in 1..=right.len() {
            let cost = usize::from(left[i - 1] != right[j - 1]);
            let deletion = distance[(i - 1) * columns + j] + 1;
            let insertion = distance[i * columns + j - 1] + 1;
            let substitution = distance[(i - 1) * columns + j - 1] + cost;
            let mut best = deletion.min(insertion).min(substitution);
            if i > 1 && j > 1 && left[i - 1] == right[j - 2] && left[i - 2] == right[j - 1] {
                best = best.min(distance[(i - 2) * columns + j - 2] + cost);
            }
            distance[i * columns + j] = best;
        }
    }
    distance[left.len() * columns + right.len()]
}

fn is_kar(character: char) -> bool {
    matches!(
        character,
        '\u{09be}'
            | '\u{09bf}'
            | '\u{09c0}'
            | '\u{09c1}'
            | '\u{09c2}'
            | '\u{09c3}'
            | '\u{09c7}'
            | '\u{09c8}'
            | '\u{09cb}'
            | '\u{09cc}'
            | '\u{09c4}'
    )
}

fn is_bengali_vowel(character: char) -> bool {
    matches!(
        character,
        '\u{0985}'
            | '\u{0986}'
            | '\u{0987}'
            | '\u{0988}'
            | '\u{0989}'
            | '\u{098a}'
            | '\u{098b}'
            | '\u{098f}'
            | '\u{0990}'
            | '\u{0993}'
            | '\u{0994}'
            | '\u{098c}'
            | '\u{09e1}'
            | '\u{09be}'
            | '\u{09bf}'
            | '\u{09c0}'
            | '\u{09c1}'
            | '\u{09c2}'
            | '\u{09c3}'
            | '\u{09c7}'
            | '\u{09c8}'
            | '\u{09cb}'
            | '\u{09cc}'
    )
}

fn remove_last_char(value: &str) -> &str {
    value
        .char_indices()
        .next_back()
        .map_or(value, |(index, _)| &value[..index])
}

fn separate_padding(input: &str) -> WordParts {
    let chars: Vec<char> = input.chars().collect();
    let mut leading_end = 0;
    while leading_end < chars.len() {
        if chars.get(leading_end) == Some(&',') && chars.get(leading_end + 1) == Some(&',') {
            break;
        }
        let Some(unit_len) = padding_unit_len(&chars, leading_end) else {
            break;
        };
        leading_end += unit_len;
    }

    let mut trailing_start = chars.len();
    while trailing_start > leading_end {
        if trailing_start >= 2
            && matches!(chars[trailing_start - 2], ':' | '.')
            && chars[trailing_start - 1] == '`'
        {
            trailing_start -= 2;
        } else if is_padding_character(chars[trailing_start - 1]) {
            trailing_start -= 1;
        } else {
            break;
        }
    }

    for position in trailing_start..chars.len().saturating_sub(1) {
        if chars[position] == ',' && chars[position + 1] == ',' {
            let mut end = position;
            while end + 1 < chars.len() && chars[end] == ',' && chars[end + 1] == ',' {
                end += 2;
            }
            trailing_start = end;
            break;
        }
    }
    trailing_start = trailing_start.max(leading_end);

    WordParts {
        leading: chars[..leading_end].iter().collect(),
        middle: chars[leading_end..trailing_start].iter().collect(),
        trailing: chars[trailing_start..].iter().collect(),
    }
}

fn padding_unit_len(chars: &[char], position: usize) -> Option<usize> {
    if position + 1 < chars.len()
        && matches!(chars[position], ':' | '.')
        && chars[position + 1] == '`'
    {
        Some(2)
    } else if is_padding_character(chars[position]) {
        Some(1)
    } else {
        None
    }
}

fn is_padding_character(character: char) -> bool {
    matches!(
        character,
        '-' | ']'
            | '~'
            | '!'
            | '@'
            | '#'
            | '%'
            | '&'
            | '*'
            | '(' 
            | ')'
            | '_'
            | '='
            | '+'
            | '['
            | '{'
            | '}'
            | '\''
            | '"'
            | ';'
            | '<'
            | '>'
            | '/'
            | '?'
            | '|'
            | '.'
            | ','
    )
}

fn candidate_selection_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".candidate-selections.json"))
}

fn load_candidate_selections() -> HashMap<String, String> {
    let Some(path) = candidate_selection_path() else {
        return HashMap::new();
    };
    fs::read(path)
        .ok()
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default()
}

fn save_candidate_selections(selections: &HashMap<String, String>) {
    let Some(path) = candidate_selection_path() else {
        return;
    };
    let Ok(serialized) = serde_json::to_vec(selections) else {
        return;
    };
    let temporary = path.with_extension("json.tmp");
    if fs::write(&temporary, serialized).is_ok() {
        let _ = fs::rename(temporary, path);
    }
}

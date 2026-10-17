use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

type WordTables = HashMap<String, Vec<String>>;

static WORD_TABLES: OnceLock<WordTables> = OnceLock::new();

fn word_tables() -> &'static WordTables {
    WORD_TABLES.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/dictionary.json"))
            .expect("embedded Avro word dictionary must be valid JSON")
    })
}

pub(crate) fn search(query: &str, regex_cache: &mut HashMap<String, Regex>) -> Vec<String> {
    let Some(bucket_names) = buckets(query) else {
        return Vec::new();
    };

    if !regex_cache.contains_key(query) {
        if regex_cache.len() >= 256 {
            regex_cache.clear();
        }
        let pattern = crate::parse_dictionary_pattern(query);
        let anchored = format!("^(?:{pattern})$");
        if let Ok(regex) = Regex::new(&anchored) {
            regex_cache.insert(query.to_owned(), regex);
        } else {
            return Vec::new();
        }
    }

    let Some(regex) = regex_cache.get(query) else {
        return Vec::new();
    };
    let tables = word_tables();
    let mut matches = Vec::new();

    for bucket in bucket_names {
        let table_name = format!("w_{bucket}");
        if let Some(words) = tables.get(&table_name) {
            matches.extend(words.iter().filter(|word| regex.is_match(word)).cloned());
        }
    }

    matches
}

fn buckets(query: &str) -> Option<&'static [&'static str]> {
    let first = query.chars().next()?.to_ascii_lowercase();
    Some(match first {
        'a' => &["a", "aa", "e", "oi", "o", "nya", "y"],
        'b' => &["b", "bh"],
        'c' => &["c", "ch", "k"],
        'd' => &["d", "dh", "dd", "ddh"],
        'e' => &["i", "ii", "e", "y"],
        'f' => &["ph"],
        'g' => &["g", "gh", "j"],
        'h' => &["h"],
        'i' => &["i", "ii", "y"],
        'j' => &["j", "jh", "z"],
        'k' => &["k", "kh"],
        'l' => &["l"],
        'm' => &["h", "m"],
        'n' => &["n", "nya", "nga", "nn"],
        'o' => &["a", "u", "uu", "oi", "o", "ou", "y"],
        'p' => &["p", "ph"],
        'q' => &["k"],
        'r' => &["rri", "h", "r", "rr", "rrh"],
        's' => &["s", "sh", "ss"],
        't' => &["t", "th", "tt", "tth", "khandatta"],
        'u' => &["u", "uu", "y"],
        'v' => &["bh"],
        'w' => &["o"],
        'x' => &["e", "k"],
        'y' => &["i", "y"],
        'z' => &["h", "j", "jh", "z"],
        _ => return None,
    })
}

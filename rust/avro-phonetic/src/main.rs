use avro_phonetic::{parse, Preferences, SuggestionEngine};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let suggestions = args.first().is_some_and(|argument| argument == "suggest");
    if suggestions {
        args.remove(0);
    }
    let dictionary_enabled = !args.iter().any(|argument| argument == "--no-dictionary");
    args.retain(|argument| argument != "--no-dictionary");

    let mut input = args.join(" ");
    if input.is_empty() {
        io::stdin().read_to_string(&mut input)?;
    }

    if suggestions {
        let mut engine = SuggestionEngine::new();
        engine.set_preferences(Preferences {
            dictionary_enabled,
            candidate_limit: 15,
        });
        let candidates = engine.suggest(&input);
        for (index, word) in candidates.words.iter().enumerate() {
            let marker = if index == candidates.previous_selection { ">" } else { " " };
            println!("{marker} {word}");
        }
    } else {
        print!("{}", parse(&input));
    }
    Ok(())
}

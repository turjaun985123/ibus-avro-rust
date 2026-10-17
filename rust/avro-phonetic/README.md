# Avro phonetic Rust core

This crate is the Rust migration of the Avro phonetic input engine. It ports the ordered transliteration patterns and contextual rules, the Bengali dictionary matcher, autocorrect mappings, suffix generation, Damerau-Levenshtein candidate ranking, remembered candidate choices, and the per-engine key/selection state machine. It also provides a command-line frontend: input on stdin (or as arguments) is transliterated; the `suggest` subcommand prints ranked dictionary candidates. `--no-dictionary` disables dictionary suggestions.

The GJS IBus process, D-Bus adapter, lookup-table UI, and desktop preferences are still being migrated. Legacy word data is embedded as generated UTF-8 JSON; source data remains the canonical version during migration. GitHub Actions owns all Rust builds and validation; local Cargo commands are prohibited by repository policy.

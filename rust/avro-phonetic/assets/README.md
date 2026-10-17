# Embedded legacy data

These compact UTF-8 JSON files are generated from the project's existing MPL-2.0 JavaScript assets:

- `dictionary.json` from `avrodict.js`
- `autocorrect.json` from `autocorrect.js`
- `suffixes.json` from `suffixdict.js`

Regenerate them with `python3 rust/avro-phonetic/tools/import_legacy_data.py`. The corresponding source files remain the canonical data during the migration.

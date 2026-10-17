#!/usr/bin/env python3
"""Regenerate Rust tables from the legacy Avro phonetic JavaScript source."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT / "avrolib.js"
OUTPUT = Path(__file__).resolve().parents[1] / "src" / "patterns.rs"
POSITION_MAP = {"prefix": "Prefix", "suffix": "Suffix"}
SCOPE_MAP = {
    "punctuation": "Punctuation",
    "vowel": "Vowel",
    "consonant": "Consonant",
    "exact": "Exact",
}


def rust_string(value: str) -> str:
    hashes = ""
    while '"' + hashes in value:
        hashes += "#"
    return f'r{hashes}"{value}"{hashes}'


def load_patterns(source_path: Path) -> tuple[list[dict], int]:
    source = source_path.read_text()
    start = source.index('"patterns"')
    start = source.index("[", start)
    patterns, end = json.JSONDecoder().raw_decode(source[start:])
    return patterns, start + end


def load_string(source_path: Path, key: str, start: int = 0) -> str:
    source = source_path.read_text()
    marker = f'"{key}"'
    position = source.index(marker, start) + len(marker)
    position = source.index(":", position) + 1
    value, _ = json.JSONDecoder().raw_decode(source[position:].lstrip())
    return value


def render_pattern_table(patterns: list[dict]) -> str:
    lines = ["pub(crate) static PATTERNS: &[Pattern] = &["]
    for pattern in patterns:
        lines.extend(
            [
                "    Pattern {",
                f"        find: {rust_string(pattern['find'])},",
                f"        replace: {rust_string(pattern['replace'])},",
                "        rules: &[",
            ]
        )
        for rule in pattern.get("rules", []):
            lines.extend(["            Rule {", "                conditions: &["])
            for match_rule in rule.get("matches", []):
                scope = match_rule["scope"]
                negative = scope.startswith("!") or match_rule.get("negative", False)
                if scope.startswith("!"):
                    scope = scope[1:]
                lines.append(
                    "                    Condition { position: Position::%s, scope: Scope::%s, value: %s, negative: %s },"
                    % (
                        POSITION_MAP[match_rule["type"]],
                        SCOPE_MAP[scope],
                        rust_string(match_rule.get("value", "")),
                        str(bool(negative)).lower(),
                    )
                )
            lines.extend(
                [
                    "                ],",
                    f"                replace: {rust_string(rule['replace'])},",
                    "            },",
                ]
            )
        lines.extend(["        ],", "    },"])
    lines.append("];" )
    return "\n".join(lines) + "\n"


def render_base(patterns: list[dict]) -> str:
    header = '''// Pattern data derived from jsAvroPhonetic by Rifat Nabi, Copyright (C) OmicronLab.
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

'''
    return header + render_pattern_table(patterns)


if __name__ == "__main__":
    patterns, _ = load_patterns(SOURCE)
    OUTPUT.write_text(render_base(patterns))

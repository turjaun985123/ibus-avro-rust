#!/usr/bin/env python3
"""Generate the regular-expression pattern table from avroregexlib.js."""

from pathlib import Path

from generate_patterns import ROOT, load_patterns, load_string, render_pattern_table

SOURCE = ROOT / "avroregexlib.js"
OUTPUT = Path(__file__).resolve().parents[1] / "src" / "regex_patterns.rs"

if __name__ == "__main__":
    patterns, end = load_patterns(SOURCE)
    ignored = load_string(SOURCE, "ignore", end)
    header = '''// Pattern data derived from jsAvroPhonetic by OmicronLab contributors.
// Generated from ../../../avroregexlib.js; see LICENSE for the MPL-2.0 terms.
use super::patterns::{Condition, Pattern, Position, Rule, Scope};

'''
    OUTPUT.write_text(header + f"pub(crate) const IGNORE: &str = {__import__('generate_patterns').rust_string(ignored)};\n\n" + render_pattern_table(patterns))

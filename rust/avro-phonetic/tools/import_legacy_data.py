#!/usr/bin/env python3
"""Convert legacy JavaScript data assignments to compact UTF-8 JSON."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
ASSETS = Path(__file__).resolve().parents[1] / "assets"


def read_assignment(filename: str):
    source = (ROOT / filename).read_text()
    start = source.index("=") + 1
    value, _ = json.JSONDecoder().raw_decode(source[start:].lstrip())
    return value


def write_json(filename: str, data) -> None:
    (ASSETS / filename).write_text(
        json.dumps(data, ensure_ascii=False, separators=(",", ":")) + "\n"
    )


if __name__ == "__main__":
    ASSETS.mkdir(parents=True, exist_ok=True)
    write_json("dictionary.json", read_assignment("avrodict.js"))
    write_json("autocorrect.json", read_assignment("autocorrect.js"))
    write_json("suffixes.json", read_assignment("suffixdict.js"))

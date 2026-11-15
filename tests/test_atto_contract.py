"""Fail-closed source projection checks for the AttoLang contract."""
import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "atto" / "happy_wakey_contract.atto"
SCHEMA = ROOT / "schemas" / "alarm-occurrence.schema.json"
RUST = ROOT / "src" / "lib.rs"

EXPECTED = {
    "AlarmOccurrenceState": ("state",),
    "AlarmTransitionEvent": ("event",),
    "TransitionDisposition": ("transition_response", "disposition"),
}


def snake_case(identifier: str) -> str:
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", identifier).lower()


def extract_atto_enums(text: str) -> dict[str, list[str]]:
    # Anchored declarations: a comment mentioning a variant is never an admission.
    pattern = re.compile(r"^define enum ([A-Za-z][A-Za-z0-9_]*) as\s*$"
                         r"(.*?)^end\s*$", re.M | re.S)
    enums = {}
    for match in pattern.finditer(text):
        name = match.group(1)
        if name in enums:
            raise ValueError(f"duplicate Atto enum: {name}")
        members = []
        for line in match.group(2).splitlines():
            value = line.strip()
            if not re.fullmatch(r"[A-Za-z][A-Za-z0-9_]*", value):
                raise ValueError(f"invalid enum member in {name}: {value}")
            if value in members:
                raise ValueError(f"duplicate enum member in {name}: {value}")
            members.append(value)
        if not members:
            raise ValueError(f"empty Atto enum: {name}")
        enums[name] = members
    return enums


def extract_rust_enums(text: str) -> dict[str, list[str]]:
    enums = {}
    for name in EXPECTED:
        pattern = rf"pub enum {name}\s*\{{([^{{}}]*)\}}"
        match = re.search(pattern, text)
        if match is None:
            raise ValueError(f"missing Rust enum: {name}")
        enums[name] = [part.strip() for part in match.group(1).split(",") if part.strip()]
    return enums


def wire_values(schema: dict, key: tuple[str, ...]) -> list[str]:
    node = schema["$defs"][key[0]]
    for part in key[1:]:
        node = node["properties"][part]
    return node["enum"]


def validate_projection(atto: str, rust: str, schema: dict) -> None:
    atto_enums = extract_atto_enums(atto)
    rust_enums = extract_rust_enums(rust)
    if set(atto_enums) != set(EXPECTED):
        raise ValueError("Atto enum set drifted")
    for name, key in EXPECTED.items():
        if atto_enums[name] != rust_enums[name]:
            raise ValueError(f"{name}: Rust declaration drift")
        if [snake_case(v) for v in atto_enums[name]] != wire_values(schema, key):
            raise ValueError(f"{name}: JSON wire declaration drift")


class AttoProjectionTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.atto = SOURCE.read_text(encoding="utf-8")
        cls.rust = RUST.read_text(encoding="utf-8")
        cls.schema = json.loads(SCHEMA.read_text(encoding="utf-8"))

    def test_current_projection_matches_authorities(self):
        validate_projection(self.atto, self.rust, self.schema)

    def test_missing_variant_fails_closed(self):
        with self.assertRaises(ValueError):
            validate_projection(self.atto.replace("    MarkMissed\n", ""), self.rust, self.schema)

    def test_duplicate_variant_fails_closed(self):
        with self.assertRaises(ValueError):
            validate_projection(self.atto.replace("    MarkMissed\n",
                                                  "    MarkMissed\n    MarkMissed\n"),
                                self.rust, self.schema)

    def test_schema_drift_fails_closed(self):
        schema = json.loads(json.dumps(self.schema))
        schema["$defs"]["state"]["enum"][0] = "not_scheduled"
        with self.assertRaises(ValueError):
            validate_projection(self.atto, self.rust, schema)


if __name__ == "__main__":
    unittest.main()

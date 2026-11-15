# AttoLang contract surface

This folder is a **source-level contract projection**, not a generated
network SDK or proof of AttoLang runtime parity. `happy_wakey_contract.atto`
exposes the nominal alarm state, event, and disposition enums, plus the
canonical alarm/occurrence schema identifiers. Its enum members deliberately
use AttoLang identifiers; **their JSON spellings are lower_snake_case**.

The executable authority remains the JSON Schema, TypeSpec and Rust files at
the repository root. In particular, schema constraints such as required
`transition_id`, `expected_generation`, `snooze_until` conditionality,
and `additionalProperties: false` must be checked at the trust boundary
before interpreting an Atto value. Do not derive a serializer from enum member
names without an explicit, tested mapping.

Run `cargo test --test atto_contract --locked` to verify
the Atto enum projection against JSON Schema and Rust. This is **host-side
source conformance only**; it does not parse, compile, execute, or serialize
`.atto` programs. Guest compiler support and cross-backend behavior require
separate evidence.

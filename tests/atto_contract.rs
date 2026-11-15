//! Rust tests for AttoLang's nominal enum projection and JSON wire mapping.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const ENUMS: [(&str, &[&str]); 3] = [
    ("AlarmOccurrenceState", &["$defs", "state", "enum"]),
    ("AlarmTransitionEvent", &["$defs", "event", "enum"]),
    (
        "TransitionDisposition",
        &[
            "$defs",
            "transition_response",
            "properties",
            "disposition",
            "enum",
        ],
    ),
];

fn fixtures() -> (String, String, Value) {
    let atto = include_str!("../atto/happy_wakey_contract.atto").into();
    let rust = include_str!("../src/lib.rs").into();
    let json =
        serde_json::from_str(include_str!("../schemas/alarm-occurrence.schema.json")).unwrap();
    (atto, rust, json)
}

fn identifier(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn atto_enums(source: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut result = BTreeMap::new();
    let mut lines = source.lines();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if !line.starts_with("define enum ") {
            continue;
        }
        let name = line
            .strip_prefix("define enum ")
            .and_then(|s| s.strip_suffix(" as"))
            .ok_or("malformed Atto header")?;
        if !identifier(name) || result.contains_key(name) {
            return Err("duplicate/invalid enum".into());
        }
        let mut members = Vec::new();
        let mut end = false;
        for line in lines.by_ref() {
            let item = line.trim();
            if item == "end" {
                end = true;
                break;
            }
            if item.is_empty() {
                continue;
            }
            if !identifier(item) || members.iter().any(|m| m == item) {
                return Err(format!("invalid/duplicate member: {item}"));
            }
            members.push(item.to_owned());
        }
        if !end || members.is_empty() {
            return Err(format!("unterminated/empty enum {name}"));
        }
        result.insert(name.into(), members);
    }
    Ok(result)
}

fn rust_members(source: &str, name: &str) -> Result<Vec<String>, String> {
    let header = format!("pub enum {name} {{");
    let mut matches = source.match_indices(&header);
    let (position, _) = matches.next().ok_or("missing Rust enum")?;
    if matches.next().is_some() {
        return Err("duplicate Rust enum".into());
    }
    let body = source[position + header.len()..]
        .split_once('}')
        .ok_or("unterminated Rust enum")?
        .0;
    let members: Vec<_> = body
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    if members.is_empty() || members.iter().any(|m| !identifier(m)) {
        return Err("invalid Rust enum variants".into());
    }
    Ok(members)
}

fn snake_case(name: &str) -> String {
    let mut out = String::new();
    let mut previous: Option<char> = None;
    for ch in name.chars() {
        if ch.is_ascii_uppercase() {
            if previous.is_some_and(|p| p.is_ascii_lowercase() || p.is_ascii_digit()) {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
        previous = Some(ch);
    }
    out
}

fn verify(atto: &str, rust: &str, schema: &Value) -> Result<(), String> {
    let source = atto_enums(atto)?;
    let expected: BTreeSet<_> = ENUMS.iter().map(|(name, _)| *name).collect();
    if source.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected {
        return Err("enum set drift".into());
    }
    for (name, keys) in ENUMS {
        let members = &source[name];
        if members != &rust_members(rust, name)? {
            return Err(format!("Rust variant drift: {name}"));
        }
        let mut wire = schema;
        for key in keys {
            wire = wire.get(*key).ok_or("schema path missing")?;
        }
        let names: Vec<_> = wire
            .as_array()
            .ok_or("missing wire variants")?
            .iter()
            .map(|v| v.as_str().ok_or("non-string variant"))
            .collect::<Result<_, _>>()?;
        if members.iter().map(|v| snake_case(v)).collect::<Vec<_>>() != names {
            return Err(format!("wire value drift: {name}"));
        }
    }
    Ok(())
}

#[test]
fn pinned_atto_rust_json_parity() {
    let (a, r, j) = fixtures();
    verify(&a, &r, &j).unwrap();
}

#[test]
fn rejects_missing_duplicate_and_bad_variant() {
    let (a, r, j) = fixtures();
    assert!(verify(&a.replace("    MarkMissed\n", ""), &r, &j).is_err());
    assert!(verify(
        &a.replace("    MarkMissed\n", "    MarkMissed\n    MarkMissed\n"),
        &r,
        &j
    )
    .is_err());
    assert!(verify(&a.replace("    MarkMissed\n", "    Mark-Missed\n"), &r, &j).is_err());
}

#[test]
fn rejects_schema_and_rust_drift() {
    let (a, r, mut j) = fixtures();
    j["$defs"]["state"]["enum"][0] = json!("wrong");
    assert!(verify(&a, &r, &j).is_err());
    let (_, _, original) = fixtures();
    assert!(verify(&a, &r.replace("    MarkMissed,", "    Other,"), &original).is_err());
}

#[test]
fn rejects_extra_duplicate_and_unterminated_enums() {
    let (a, r, j) = fixtures();
    assert!(verify(&format!("{a}\ndefine enum Other as\n    X\nend\n"), &r, &j).is_err());
    assert!(verify(
        &format!("{a}\ndefine enum AlarmOccurrenceState as\n    X\nend\n"),
        &r,
        &j
    )
    .is_err());
    assert!(verify(&a.replace("    Rejected\nend", "    Rejected"), &r, &j).is_err());
}

fn schema_id_literal<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let signature = format!("pub fnc {name}(): string {{");
    let mut matches = source.match_indices(&signature);
    let (index, _) = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    let remaining = &source[index + signature.len()..];
    let (body, _) = remaining.split_once('}')?;
    let value = body.trim().strip_prefix("return \"")?.strip_suffix("\";")?;
    if value.contains('"') || value.contains('\\') {
        return None;
    }
    Some(value)
}

#[test]
fn atto_schema_ids_match_authoritative_json_ids() {
    let (atto, _, occurrence) = fixtures();
    let alarm: Value = serde_json::from_str(include_str!("../schemas/alarm.schema.json")).unwrap();
    assert_eq!(
        schema_id_literal(&atto, "occurrence_schema_id"),
        occurrence["$id"].as_str()
    );
    assert_eq!(
        schema_id_literal(&atto, "alarm_schema_id"),
        alarm["$id"].as_str()
    );
}

#[test]
fn rejects_dynamic_duplicate_or_mutated_schema_ids() {
    let (atto, _, _) = fixtures();
    assert_eq!(
        schema_id_literal(
            &atto.replace(
                "return \"https://schemas.happy-wakey.dev/alarm.schema.json\";",
                "return external_url();"
            ),
            "alarm_schema_id"
        ),
        None
    );
    assert_eq!(
        schema_id_literal(
            &format!("{atto}\npub fnc alarm_schema_id(): string {{ return \"unsafe\"; }}"),
            "alarm_schema_id"
        ),
        None
    );
    let alarm: Value = serde_json::from_str(include_str!("../schemas/alarm.schema.json")).unwrap();
    assert_ne!(
        schema_id_literal(
            &atto.replace("alarm.schema.json", "alarm-v2.schema.json"),
            "alarm_schema_id"
        ),
        alarm["$id"].as_str()
    );
}

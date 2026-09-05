#!/usr/bin/env python3
from __future__ import annotations

import json
import re
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas"


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


schema_docs = [load(path) for path in sorted(SCHEMAS.glob("*.schema.json"))]
registry = Registry()
for document in schema_docs:
    Draft202012Validator.check_schema(document)
    registry = registry.with_resource(document["$id"], Resource.from_contents(document))

fixtures = {
    "app-snapshot.schema.json": ROOT / "examples/app-snapshot.json",
    "alarm.schema.json": ROOT / "examples/alarm.json",
    "alarm-occurrence.schema.json": ROOT / "examples/alarm-occurrence.json",
    "service-operation-request.schema.json": ROOT / "examples/service-operation-request.json",
    "service-operation-response.schema.json": ROOT / "examples/service-operation-response.json",
    "sync-envelope.schema.json": ROOT / "examples/sync-envelope.json",
    "morning-briefing.schema.json": ROOT / "examples/morning-briefing.json",
}
for schema_name, fixture_path in fixtures.items():
    schema = load(SCHEMAS / schema_name)
    Draft202012Validator(
        schema,
        registry=registry,
        format_checker=FormatChecker(),
    ).validate(load(fixture_path))

openapi = load(ROOT / "openapi/happy-wakey.openapi.json")
assert openapi["openapi"].startswith("3.1.")
operation_ids = []
for path_item in openapi["paths"].values():
    for method, operation in path_item.items():
        if method in {"get", "post", "put", "patch", "delete"}:
            operation_ids.append(operation["operationId"])
assert len(operation_ids) == len(set(operation_ids)), "duplicate operationId"
assert {"health", "listAlarms", "createAlarm", "transitionOccurrence", "pullChanges", "pushChanges"} == set(operation_ids)

briefing_schema = load(SCHEMAS / "morning-briefing.schema.json")
type_spec = (ROOT / "typespec/main.tsp").read_text(encoding="utf-8")
type_spec_models = set(re.findall(r"^model\s+([A-Za-z][A-Za-z0-9_]*)", type_spec, re.MULTILINE))
peer_models = {
    "OnboardingIntent",
    "AccountContext",
    "ConnectorConsent",
    "SourceItemCandidate",
    "UsefulnessDecision",
    "SafeDeepLink",
    "BriefingCard",
    "MorningBriefing",
    "EmbeddingDescriptor",
    "CorrelationFinding",
    "RealtimeEnvelope",
    "ChatSession",
}
assert peer_models <= set(briefing_schema["$defs"]), "JSON Schema authority is missing peer models"
assert peer_models <= type_spec_models, "TypeSpec authority is missing peer models"
assert briefing_schema["$defs"]["SafeDeepLink"]["properties"]["feedFallbackAllowed"]["const"] is False
assert briefing_schema["$defs"]["EmbeddingDescriptor"]["properties"]["dimensions"]["maximum"] == 4100

generated_files = (
    ROOT / "generated/json-schema/types.d.ts",
    ROOT / "generated/json-schema/validator.cjs",
    ROOT / "generated/typespec/types.ts",
    ROOT / "generated/typespec/validator.cjs",
    ROOT / "generated/typespec/protobuf/@typespec/protobuf/main.proto",
    ROOT / "generated/provenance.json",
)
assert all(path.is_file() and path.stat().st_size > 0 for path in generated_files)

sql = (ROOT / "sql/schema.sql").read_text(encoding="utf-8")
for required in (
    "happy_wakey_alarms",
    "happy_wakey_alarm_occurrences",
    "happy_wakey_transition_receipts",
    "happy_wakey_sync_changes",
    "happy_wakey_async_operations",
):
    assert required in sql

print(
    f"validated {len(schema_docs)} Draft 2020-12 schemas, {len(fixtures)} fixtures, "
    f"{len(operation_ids)} operations, {len(peer_models)} peer-source briefing models, "
    "generated validators, Protobuf, and declarative SQL"
)

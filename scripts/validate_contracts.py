#!/usr/bin/env python3
from __future__ import annotations

import json
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

sql = (ROOT / "sql/schema.sql").read_text(encoding="utf-8")
for required in (
    "happy_wakey_alarms",
    "happy_wakey_alarm_occurrences",
    "happy_wakey_transition_receipts",
    "happy_wakey_sync_changes",
    "happy_wakey_async_operations",
):
    assert required in sql

print(f"validated {len(schema_docs)} Draft 2020-12 schemas, {len(fixtures)} fixtures, {len(operation_ids)} operations, and declarative SQL")

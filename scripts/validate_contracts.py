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
    "provider-connection.schema.json": ROOT / "examples/provider-connection.json",
    "sleep-summary.schema.json": ROOT / "examples/sleep-summary.json",
    "biometric-summary.schema.json": ROOT / "examples/biometric-summary.json",
    "day-plan.schema.json": ROOT / "examples/day-plan.json",
    "habit.schema.json": ROOT / "examples/habit.json",
    "inbox-digest.schema.json": ROOT / "examples/inbox-digest.json",
    "message-digest.schema.json": ROOT / "examples/message-digest.json",
    "market-watch.schema.json": ROOT / "examples/market-watch.json",
    "environment-brief.schema.json": ROOT / "examples/environment-brief.json",
    "briefing.schema.json": ROOT / "examples/briefing.json",
    "module-layout.schema.json": ROOT / "examples/module-layout.json",
    "async-operation.schema.json": ROOT / "examples/async-operation.json",
}
for schema_name, fixture_path in fixtures.items():
    schema = load(SCHEMAS / schema_name)
    Draft202012Validator(
        schema,
        registry=registry,
        format_checker=FormatChecker(),
    ).validate(load(fixture_path))

# Every schema must be exercised by at least one conformance fixture. A new
# contract that nothing validates against is indistinguishable from a typo.
unexercised = {path.name for path in sorted(SCHEMAS.glob("*.schema.json"))} - set(fixtures) - {
    "common.schema.json",
    "error.schema.json",
}
assert not unexercised, f"schemas without a fixture: {sorted(unexercised)}"


def reject(schema_name: str, document, why: str) -> None:
    """A counter-example the contract must refuse. Fail-closed rules are only
    real if something checks that they actually close."""
    validator = Draft202012Validator(
        load(SCHEMAS / schema_name),
        registry=registry,
        format_checker=FormatChecker(),
    )
    assert not validator.is_valid(document), f"expected rejection: {why}"


briefing = load(ROOT / "examples/briefing.json")

# A section that is not ready may not smuggle items through.
not_ready_with_items = json.loads(json.dumps(briefing))
not_ready_with_items["sections"][0]["state"] = "not_connected"
reject("briefing.schema.json", not_ready_with_items, "not_connected section carrying items")

# A degraded or missing section must say why.
degraded_without_detail = json.loads(json.dumps(briefing))
degraded_without_detail["sections"][2]["state_detail"] = None
reject("briefing.schema.json", degraded_without_detail, "degraded section without state_detail")

# A ready section must name where its content came from.
ready_without_provenance = json.loads(json.dumps(briefing))
ready_without_provenance["sections"][0]["provenance"] = []
reject("briefing.schema.json", ready_without_provenance, "ready section without provenance")

inbox = load(ROOT / "examples/inbox-digest.json")

# Metadata-only authorization must never carry a body preview.
metadata_with_snippet = json.loads(json.dumps(inbox))
metadata_with_snippet["accounts"][0]["items"][0]["snippet"] = "Hi, following up on"
reject("inbox-digest.schema.json", metadata_with_snippet, "metadata_only account carrying a snippet")

# A counts-only account exposes a number and nothing else.
counts_with_items = json.loads(json.dumps(inbox))
counts_with_items["accounts"][1]["items"] = inbox["accounts"][0]["items"]
reject("inbox-digest.schema.json", counts_with_items, "counts_only account carrying items")

messages = load(ROOT / "examples/message-digest.json")

# A platform no third party may read cannot carry threads.
unavailable_with_threads = json.loads(json.dumps(messages))
unavailable_with_threads["platforms"][2]["threads"] = messages["platforms"][0]["threads"]
reject("message-digest.schema.json", unavailable_with_threads, "policy_unavailable platform carrying threads")

# Nor an unread count, which would imply a read that did not happen.
unavailable_with_count = json.loads(json.dumps(messages))
unavailable_with_count["platforms"][2]["unread_total"] = 3
reject("message-digest.schema.json", unavailable_with_count, "policy_unavailable platform reporting a count")

def validator_for(ref: str) -> Draft202012Validator:
    """Validate against one named $def, resolving its relative references
    against the owning document's base URI."""
    return Draft202012Validator(
        {"$schema": "https://json-schema.org/draft/2020-12/schema", "$ref": ref},
        registry=registry,
        format_checker=FormatChecker(),
    )


HABIT_ENTRY = "https://schemas.happy-wakey.dev/habit.schema.json#/$defs/habit_entry"
completed_entry = {
    "id": "00000000-0000-4000-8000-000000000001",
    "habit_id": "00000000-0000-4000-8000-000000000002",
    "day": "2026-09-05",
    "state": "completed",
    "completed_at": "2026-09-05T13:30:00Z",
    "block_id": None,
    "generation": 1,
    "updated_at": "2026-09-05T13:30:00Z",
}
validator_for(HABIT_ENTRY).validate(completed_entry)
assert not validator_for(HABIT_ENTRY).is_valid(
    {**completed_entry, "completed_at": None}
), "expected rejection: completed habit entry without completed_at"
assert not validator_for(HABIT_ENTRY).is_valid(
    {**completed_entry, "state": "skipped"}
), "expected rejection: skipped habit entry carrying completed_at"

# The response subject an accepted operation is told to listen on must be that
# operation's own inbox; anything else would hand a caller someone else's replies.
accepted = load(ROOT / "examples/async-operation.json")
accepted = {
    "schema": "happy-wakey.async-operation.accepted.v1",
    "operation_id": accepted["operation_id"],
    "response_subject": "happy-wakey.responses." + accepted["operation_id"],
}
Draft202012Validator(
    load(SCHEMAS / "async-operation.schema.json"),
    registry=registry,
    format_checker=FormatChecker(),
).validate(accepted)
reject(
    "async-operation.schema.json",
    {**accepted, "response_subject": "happy-wakey.responses.not-a-uuid"},
    "accepted operation whose response subject is not a uuid inbox",
)

# The JetStream wake-up is credential-free by construction: the API derives the
# verified owner from the outbox row, never from the signal.
reject(
    "async-operation.schema.json",
    {
        "schema": "happy-wakey.async-operation.signal.v1",
        "operation_id": accepted["operation_id"],
        "bearer": "redacted",
    },
    "signal carrying a credential",
)

openapi = load(ROOT / "openapi/happy-wakey.openapi.json")
assert openapi["openapi"].startswith("3.1.")
operation_ids = []
for path_item in openapi["paths"].values():
    for method, operation in path_item.items():
        if method in {"get", "post", "put", "patch", "delete"}:
            operation_ids.append(operation["operationId"])
assert len(operation_ids) == len(set(operation_ids)), "duplicate operationId"
assert {
    "health",
    "listAlarms",
    "createAlarm",
    "transitionOccurrence",
    "pullChanges",
    "pushChanges",
    "listProviderConnections",
    "connectProvider",
    "disconnectProvider",
    "getBriefing",
    "getDayPlan",
    "getSleepSummary",
    "getBiometricSummary",
    "getInboxDigest",
    "getMessageDigest",
    "getMarketWatch",
    "getEnvironmentBrief",
    "listHabits",
    "createHabit",
    "recordHabitEntry",
    "getModuleLayout",
    "putModuleLayout",
} == set(operation_ids)

sql = (ROOT / "sql/schema.sql").read_text(encoding="utf-8")
for required in (
    "happy_wakey_alarms",
    "happy_wakey_alarm_occurrences",
    "happy_wakey_transition_receipts",
    "happy_wakey_sync_changes",
    "happy_wakey_async_operations",
    "happy_wakey_provider_connections",
    "happy_wakey_sleep_summaries",
    "happy_wakey_biometric_summaries",
    "happy_wakey_day_plans",
    "happy_wakey_plan_blocks",
    "happy_wakey_habits",
    "happy_wakey_habit_entries",
    "happy_wakey_briefings",
    "happy_wakey_module_layouts",
    "happy_wakey_watchlist_symbols",
    "happy_wakey_vip_senders",
):
    assert required in sql, f"missing table in declarative SQL: {required}"

# No provider secret may be declared in the desired state. A connection carries
# `credential_ref`, a handle into the secret boundary, and nothing else. Only
# executable lines are scanned; prose about credentials is allowed in comments.
executable_sql = "\n".join(
    line.split("--", 1)[0] for line in sql.splitlines()
).lower()
for forbidden in ("access_token", "refresh_token", "client_secret", "api_key", "password", "bearer"):
    assert forbidden not in executable_sql, f"credential column leaked into declarative SQL: {forbidden}"

print(
    f"validated {len(schema_docs)} Draft 2020-12 schemas, {len(fixtures)} fixtures, "
    f"11 fail-closed counter-examples, {len(operation_ids)} operations, and declarative SQL"
)

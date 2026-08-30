# happy-wakey-interfaces

The static contract authority for every Happy Wakey app, service, SDK, sync
engine, and deployment. This repository contains types, schemas, protocol
descriptions, and formal specifications only. It deliberately contains no
database access, HTTP client, UI, scheduler, notification, or transition
implementation.

## Contract layers

| Authority | Purpose |
| --- | --- |
| `src/lib.rs` | Rust data types and trait signatures; no concrete bodies |
| `schemas/*.schema.json` | Draft 2020-12 validation at every trust boundary |
| `openapi/happy-wakey.openapi.json` | HTTP routes and their exact schemas |
| `sql/schema.sql` | Declarative PostgreSQL/CockroachDB desired state |
| `formal/alarm_occurrence.qnt` | Total alarm-occurrence transition relation |
| `examples/*.json` | Cross-language conformance fixtures |

The existing Rust/Qt and Flutter app machines remain the authorities for their
native UI/effect state. `AppSnapshot` mirrors that established contract so
services and clients cannot invent a second representation. The independent
`AlarmOccurrence` machine owns durable alarm execution. A process may be
suspended while an occurrence is still scheduled, so the two machines must
never be collapsed into one set of booleans.

The app snapshot exposes nine effect lanes, including `bluetooth`. That lane
governs scan, connect, disconnect, and preview-command effects in both desktop
implementations; it does not put device identifiers or credentials into sync
or API payloads.

## Safety rules

- Unknown enum values and undeclared fields fail schema validation.
- Every mutating request carries an idempotency `transition_id`.
- Persistent TLS uses `ServiceOperationRequest` and re-authenticates the bearer
  on every frame. The asynchronous lane registers an `AsyncOperationRequest`
  over authenticated HTTPS, persists only its verified owner and operation,
  and sends a credential-free `AsyncOperationSignal` through JetStream. Both
  lanes return the same `ServiceOperationResponse` and currently expose only
  the read-only `list_alarms` operation.
- A service operation bearer is transient authorization data: it must remain
  inside the bounded encrypted request and must never enter telemetry, the
  asynchronous outbox, JetStream, response streams, or dead-letter payloads.
- Every occurrence transition carries `expected_generation`; stale callers
  stutter and cannot overwrite a newer state.
- Invalid state/event pairs are rejected without mutation.
- Owners come from Shared Auth at the service boundary, never from a client
  supplied owner field.
- Timestamps are RFC 3339 strings and recurring alarm times name an IANA time
  zone; clients must not silently substitute their local zone.
- SQL is desired state for review through
  `declarative-migrations/declarative-postgres-migrate.rs`; routine CI never
  applies destructive changes.

## Validate

```sh
python3 scripts/validate_contracts.py
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings

# Optional formal checks, with the same pinned tool used by the apps:
npx --yes --package=@informalsystems/quint@0.32.0 quint typecheck formal/alarm_occurrence.qnt
npx --yes --package=@informalsystems/quint@0.32.0 quint run \
  formal/alarm_occurrence.qnt --main=alarm_occurrence \
  --max-samples=10000 --max-steps=24 --invariant=occurrence_safety
```

Formal verification proves the declared finite abstraction and checked bound,
not clocks, operating systems, notification providers, networks, or hardware.
Those failures return as controlled events and remain fenced by generation.

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

## Domains

Alarms and occurrences remain the founding contract. The morning dashboard adds
eleven more, each with a schema, a Rust type, a conformance fixture and, where
it is durable, a row in the declarative SQL:

| Contract | What it owns |
| --- | --- |
| `provider-connection` | Every upstream source, the capabilities actually granted, and the quota it must stay inside |
| `sleep-summary` | One night, normalized across HealthKit, Health Connect, Oura, WHOOP, Google Health, Garmin and Withings |
| `biometric-summary` | One day of physiology and its deviation from the owner's own baseline |
| `day-plan` | The day as one ordered ribbon of blocks, with capacity arithmetic and a predicted energy curve |
| `habit` | Recurring commitments as schedulable, defensible objects rather than checkboxes |
| `inbox-digest` | What is waiting in the mail, metadata-only by default |
| `message-digest` | What is waiting in direct messages, with the access level each platform actually permits |
| `market-watch` | Watchlist, holdings and market events, each quote carrying the timeliness its licence grants |
| `environment-brief` | Weather, daylight, air quality and the commute about to be made |
| `briefing` | The composed morning brief that draws on all of the above |
| `module-layout` | How the owner arranged the surface, and each module's refresh floor |

The existing Rust/Qt and Flutter app machines remain the authorities for their
native UI/effect state. `AppSnapshot` mirrors that established contract so
services and clients cannot invent a second representation. The independent
`AlarmOccurrence` machine owns durable alarm execution. A process may be
suspended while an occurrence is still scheduled, so the two machines must
never be collapsed into one set of booleans.

The app snapshot exposes twenty-one effect lanes. `bluetooth` governs scan,
connect, disconnect, and preview-command effects in both desktop
implementations; it does not put device identifiers or credentials into sync
or API payloads. The twelve dashboard lanes — `briefing`, `day_plan`, `tasks`,
`habits`, `focus`, `sleep`, `biometrics`, `inbox`, `messages`, `commute`,
`portfolio` and `provider_sync` — each fence one upstream domain, so a mail
provider that is rate limited leaves its own lane failed without stalling the
lanes beside it. `tests/schema_parity.rs` compares the lane list in `src/lib.rs`
against `schemas/app-snapshot.schema.json` directly, because nothing else forces
the two spellings to agree.

## Dashboard safety rules

The dashboard contracts add four invariants to the ones below, and the contract
validator checks each with a counter-example rather than trusting prose.

- **Absence is data.** A brief section, a mail account and a messaging platform
  each carry a state that means "this could not be read here", and a section in
  any state other than `ready` carries no items. A composed brief that silently
  drops what it could not assemble is indistinguishable, to the person reading
  it, from a brief that had nothing to say; that failure mode has already sunk
  at least one shipped product in this category.
- **Authorization travels with the payload.** `content_class` and `feed_class`
  record what the upstream consent and licence actually granted. A digest read
  under a header-only mail scope cannot carry a body preview, a counts-only
  account cannot carry items, and a delayed quote cannot be re-labelled live.
- **Policy is encoded, not documented.** `MessageAccessLevel` distinguishes a
  sanctioned user-level API from a rate-capped one, from a badge-only source,
  from a platform no third party may read at any tier. A platform at
  `policy_unavailable` cannot carry threads or even an unread count, so a client
  cannot promise a unified direct-message inbox that is not buildable.
- **No credential enters the contract.** A connection carries `credential_ref`,
  an opaque handle resolved inside the secret boundary. The validator asserts
  that no column named for a token, key, secret, password or bearer exists in
  the declarative SQL.

Physiological records are deliberately absent from the sync envelope's
`collection` enum. They are read per device from the platform health store under
their own consent and are not replicated through a general-purpose change log;
adding them needs its own review rather than a new enum value.

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

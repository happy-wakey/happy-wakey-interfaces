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
| `typespec/*.tsp` | Independent HTTP and Protobuf interface authority |
| `openapi/happy-wakey.openapi.json` | HTTP routes and their exact schemas |
| `sql/schema.sql` | Declarative PostgreSQL/CockroachDB desired state |
| `formal/alarm_occurrence.qnt` | Total alarm-occurrence transition relation |
| `examples/*.json` | Cross-language conformance fixtures |

TypeSpec and JSON Schema are peer sources of truth. Neither is generated from
the other. `npm run generate` compiles TypeSpec to OpenAPI and Protobuf, then
generates an executable validator and TypeScript types from the TypeSpec path;
it separately generates an executable validator and declarations from the
JSON Schema path. `generated/provenance.json` records the source hashes, and
CI rejects generated drift. The peer-coverage test requires both sources to
declare the public morning-briefing models while allowing each source to keep
its own validation semantics.

## Morning briefing boundary

The briefing contract describes a feedless heads-up display: useful messages,
VIP and bottleneck signals, calendar, travel, weather, markets, KPIs, history,
tasks, news, and optional audio. It intentionally carries summaries and opaque
source references rather than reusable connector credentials or raw private
message bodies.

- A social deep link must point to a specific item, carry a still-valid
  `useful` decision with a score of at least `0.8`, and set
  `feedFallbackAllowed` to `false`.
- Every briefing, chat session, embedding descriptor, regression finding, and
  realtime event is tenant scoped. The authenticated subject and tenant are
  server-derived Shared Auth claims, never caller-selected authorization.
- Embeddings are referenced by hash and opaque ID; vectors and original private
  content are not wire payloads. Dimensions are bounded at 4,100.
- Correlation findings explicitly forbid causal claims and record sample size,
  confidence bounds, p-value, and multiple-testing correction.
- Extended weather is an uncertainty-labelled outlook. A 15–21 day ensemble
  must never be presented as a deterministic forecast.
- WebSocket, persistent TLS/TCP, and NATS carry one sequenced realtime envelope
  with resumable delivery and explicit acknowledgement semantics.

## Delivery and composition are two layers, not two briefings

`morning-briefing` is the **delivery** contract: the cards a client renders,
each with its usefulness decision and its safe deep link. It is the only
briefing shape that goes on the wire to a client.

`briefing-composition` is the **input** layer beneath it: what each domain
contributed to a given morning, in what state, and from which provider. It
exists so a card can answer "why am I seeing this", and so a source that could
not be read is recorded as an explicit gap rather than quietly vanishing from
the output. A composition section in any state other than `ready` carries no
items, and every ready section names its provenance; the contract validator
checks both with counter-examples that must be rejected.

The two must not converge into one object and must not both be delivered. If a
field belongs to what the reader sees, it belongs to `morning-briefing`; if it
belongs to how the briefing was assembled, it belongs to `briefing-composition`.

## Composition domains

The domains a briefing is composed from, each with a schema, a Rust type, a
conformance fixture and, where it is durable, a row in the declarative SQL:

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
| `module-layout` | How the owner arranged the surface, and each module's refresh floor |

### Composition safety rules

These are checked by counter-example in `scripts/validate_contracts.py` rather
than trusted to prose.

- **Absence is data.** A composition section, a mail account and a messaging
  platform each carry a state meaning "this could not be read here", and a
  section that is not `ready` carries no items.
- **Authorization travels with the payload.** `content_class` and `feed_class`
  record what consent and licence actually granted, so a digest read under a
  header-only mail scope cannot carry a body preview, a counts-only account
  cannot carry items, and a delayed quote cannot be relabelled live.
- **Policy is encoded, not documented.** `MessageAccessLevel` separates a
  sanctioned user-level API from a rate-capped one, from a badge-only source,
  from a platform no third party may read at any tier. This is what keeps the
  connector list in `typespec/main.tsp` honest: a connector may be *named*
  there while its real access level is `policy_unavailable`.
- **No credential enters the contract.** A connection carries `credential_ref`,
  an opaque handle into the secret boundary, and the validator asserts that no
  column named for a token, key, secret, password or bearer exists in the
  declarative SQL.

Physiological records are deliberately absent from the sync envelope's
`collection` enum. They are read per device from the platform health store
under their own consent and are not replicated through a general-purpose change
log; adding them needs its own review, not a new enum value.

The existing Rust/Qt and Flutter app machines remain the authorities for their
native UI/effect state. `AppSnapshot` mirrors that established contract so
services and clients cannot invent a second representation. The independent
`AlarmOccurrence` machine owns durable alarm execution. A process may be
suspended while an occurrence is still scheduled, so the two machines must
never be collapsed into one set of booleans.

The app snapshot exposes twenty-one effect lanes. `bluetooth` governs scan,
connect, disconnect, and preview-command effects in both desktop
implementations; it does not put device identifiers or credentials into sync
or API payloads. The twelve composition lanes — `briefing`, `day_plan`,
`tasks`, `habits`, `focus`, `sleep`, `biometrics`, `inbox`, `messages`,
`commute`, `portfolio` and `provider_sync` — each fence one upstream domain, so
a mail provider that is rate limited leaves its own lane failed without
stalling the lanes beside it. `tests/schema_parity.rs` compares the lane list
in `src/lib.rs` against `schemas/app-snapshot.schema.json` directly, because
nothing else forces the two spellings to agree.

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

The declarative `sql/schema.sql` is the reviewed storage intersection of the
independent TypeSpec and JSON Schema authorities. `sql/peer-source-map.json`
makes every model-to-table mapping executable in validation, while
`sql/postgres-rls.sql` is the PostgreSQL/Supabase tenant-policy overlay. A
request transaction sets `app.tenant_id` and `app.subject_id` only after Shared
Auth verification; absent settings match no row.

Morning-intelligence storage contains hashes, encrypted content references,
bounded classifier evidence, expiring provider-supported deep links, briefings,
1–4100-dimensional vectors, non-causal correlations, and Ores Chat session
references. It contains neither provider credentials nor plaintext message
bodies.

## Validate

```sh
python3 scripts/validate_contracts.py
npm ci
npm run check
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

## Background

The competitive inventory, the 169-capability gap analysis, and the 2026
provider integration surface that the composition domains are drawn from live
in `happy-wakey/happy-wakey-docs` under `docs/product/parity-2026/`. The
integration surface is the reason `message-digest` encodes access levels: in
2026 most direct-message platforms cannot be read by a third party at all.

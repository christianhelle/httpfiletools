# Walt — Work History

## 2026-05-24 — PRD update for `httpgenerator-core` integration

**Context:** `httpgenerator` has been rewritten in Rust and now exposes
`httpgenerator-core`. PRD previously framed `httpgenerator` as a C# source
system whose behavior had to be re-implemented in this repo. Updated PRD
to mirror the existing `httprunner-core` consumption pattern.

**Changed:**

- `PRD.md` — broad but surgical edits across source systems, MVP scope,
  feature traceability, REP/GEN requirements, architecture (workspace
  layout, foundation crates, generator ownership, data flow, dependency
  strategy), OpenAPI parser decision (closed; owned upstream), determinism,
  tests/goldens with provenance, acceptance gates, migration, compatibility,
  milestones 1–2, risks, agent roles + handoffs, benchmarks, open
  questions, backlog seeds, summary.
- `.squad/decisions/inbox/walt-httpgenerator-core-integration.md` —
  proposed decision for Scribe to merge into `.squad/decisions.md`.

**Decisions applied (from approved plan):**

- `crates/generator/` stays a thin integration crate; `cli` does not depend
  on `httpgenerator-core` directly.
- Engine exposes `generate_http_files` → `GeneratorResult { files:
  Vec<HttpFile { filename, content }> }`; file writing stays in the
  integration crate.
- Exact-pin both `httpgenerator-core` and `httprunner-core` through MVP.
- Azure scope/tenant token acquisition is included in MVP as
  product-wrapper behavior.
- Telemetry: support key default-on, redacted feature/error envelopes
  default-on disabled by `--no-logging`, no endpoint enabled in MVP,
  pluggable sink retained.
- Generator fixtures: small smoke suite with provenance metadata, no
  vendoring of the upstream corpus.

**Notes:**

- No code or build changes; documentation-only update, so no builds run.
- Session-only model directive (Opus 4.7) honored; durable model
  preferences untouched.

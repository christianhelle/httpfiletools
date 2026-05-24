# Squad Decisions

## Active Decisions

1. **`PRD.md` is the current source of truth for product scope and implementation direction.**
2. **This repository will remain Rust-only for production code.** Existing C# behavior may inform parity, but new production code belongs in Rust.
3. **MVP is CLI-first.** Generator and runner functionality ship before any TUI, GUI, or extension work.
4. **Generator and runner must share one canonical `.http` parser/model.** Generated output is required to remain valid runner input.
5. **Telemetry posture for MVP (supersedes the prior "telemetry out of MVP" stance).** No telemetry *endpoint* is enabled in the MVP build, and no network egress for telemetry occurs by default. However, redacted feature/error telemetry *envelopes* are constructed default-on at the product-wrapper layer and discarded via a no-op sink unless an out-of-MVP sink is wired in; users can disable envelope construction entirely with `--no-logging`. The diagnostic *support key* is shown by default. The pluggable sink interface is retained so a real endpoint can be activated post-MVP without a redesign.
6. **The repository consumes `httpgenerator-core` (Rust) as the canonical generation engine.** `crates/generator/` is a thin integration crate over the engine; the `cli` crate does not depend on `httpgenerator-core` directly. Local code must not reimplement OpenAPI loading, normalization, operation naming, request rendering, or output-layout policy unless a documented gap requires a narrowly-scoped adapter inside `crates/generator/`. The engine returns generated output in memory (`GeneratorResult { files: Vec<HttpFile { filename, content }> }`); file-system writes, dry-run semantics, and parse-back validation with `httprunner-core` are product-layer responsibilities. Both `httpgenerator-core` and `httprunner-core` are exact-pinned (`=x.y.z`) through MVP, with deliberate snapshot-reviewed bumps. Generator fixtures are a small smoke suite with provenance metadata; engine-internal rendering regressions are filed upstream. Azure scope/tenant token acquisition for protected OpenAPI fetches is product-wrapper behavior owned by `crates/generator/` and is included in MVP.

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

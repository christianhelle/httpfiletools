# Squad Decisions

## Active Decisions

### 2026-05-25T23:05:30.184+02:00: Team setup
**By:** Christian Helle (via Copilot)
**What:** Created the `httpfiletools` Squad roster with Danny, Rusty, Linus, Livingston, Yen, Scribe, and Ralph. The project will be a Rust `.http` tooling suite that reuses `httprunner-core` and `httpgenerator-core`.
**Why:** User confirmed the proposed team during project setup.

### 2026-05-25T23:05:30.184+02:00: User directive
**By:** Christian Helle (via Copilot)
**What:** Build on autopilot/fleet and commit changes as often as possible in small logical groups without a co-author.
**Why:** User request — captured for team memory.

### 2026-05-25T23:05:30.184+02:00: Workspace shape
**By:** Christian Helle (via Copilot)
**What:** Use a Cargo workspace with `crates/cli` and `crates/core`. `crates/core` is an orchestration wrapper around `httprunner-core` and `httpgenerator-core`, not a reimplementation.
**Why:** This keeps the binary thin, creates a stable seam for tests, and leaves room for future tools without over-splitting too early.

### 2026-05-25T23:05:30.184+02:00: Minimum supported Rust version
**By:** Christian Helle (via Copilot)
**What:** Declare Rust 1.95 as the minimum supported Rust version for `httpfiletools`.
**Why:** `httpgenerator-core` requires Rust 1.95, making it the practical floor for the workspace.

### 2026-05-25T23:05:30.184+02:00: Publishing shape
**By:** Christian Helle (via Copilot)
**What:** Initially publish one user-facing binary crate/package named `httpfiletools` and keep `crates/core` internal with `publish = false`.
**Why:** Existing public APIs already live in `httprunner-core` and `httpgenerator-core`, so publishing a new core crate early would add unnecessary compatibility commitments.

### 2026-05-25T23:05:30.184+02:00: Error handling strategy
**By:** Christian Helle (via Copilot)
**What:** `crates/core` should define typed `httpfiletools` error variants around upstream failures, preserve source errors for diagnostics, and let `crates/cli` map them to stable exit codes/messages.
**Why:** This avoids leaking raw upstream error chains as the public CLI contract while retaining diagnostic detail.

### 2026-05-25T23:05:30.184+02:00: Upstream option exposure
**By:** Christian Helle (via Copilot)
**What:** Expose a stable subset of upstream runner/generator options initially, using 1:1 flag mapping for selected upstream-compatible behavior and typed `httpfiletools` wrapper errors.
**Why:** This preserves compatibility where chosen without committing to every upstream option before the merged CLI stabilizes.

### 2026-05-25T23:05:30.184+02:00: Generator CLI compatibility
**By:** Christian Helle (via Copilot)
**What:** `httpfiletools generate` should preserve `httpgenerator` CLI behavior where applicable, except for explicitly chosen output defaults and controls.
**Why:** This keeps generator UX aligned with upstream behavior while honoring new `httpfiletools` output decisions.

### 2026-05-25T23:05:30.184+02:00: Generator input compatibility
**By:** Christian Helle (via Copilot)
**What:** `httpfiletools generate` should support local OpenAPI files and HTTP(S) OpenAPI URLs, matching upstream `httpgenerator`, but should not support stdin initially.
**Why:** Upstream-compatible behavior removes the earlier basename-only constraint that drove the file-only input decision.

### 2026-05-25T23:05:30.184+02:00: OpenAPI version and validation behavior
**By:** Christian Helle (via Copilot)
**What:** `httpfiletools generate` should preserve upstream `httpgenerator` support for Swagger/OpenAPI 2.0, 3.0, and 3.1, including upstream validation defaults and `--skip-validation` behavior.
**Why:** User chose upstream validation compatibility for generator behavior.

### 2026-05-25T23:05:30.184+02:00: Generate output defaults and controls
**By:** Christian Helle (via Copilot)
**What:** `httpfiletools generate <openapi>` writes output by default, overwrites existing targets by default, uses deterministic `.http` naming for single-file output, and supports explicit output and stdout controls where compatible.
**Why:** This captures the chosen default file-output contract while preserving room for upstream-compatible output modes.

### 2026-05-25T23:05:30.184+02:00: Generator output conflict resolution
**By:** Christian Helle (via Copilot)
**What:** When upstream `httpgenerator` output behavior conflicts with earlier single-file `generate` output decisions, upstream compatibility wins. For `generate`, `--output` should mean output directory; basename `.http` defaults and `--stdout` apply only where compatible with the selected single-file output mode.
**Why:** User chose to preserve upstream `--output-type` behavior fully, which requires honoring upstream directory/multi-file output semantics.

### 2026-05-25T23:05:30.184+02:00: Initial CLI shape
**By:** Christian Helle (via Copilot)
**What:** Start with `httpfiletools generate <openapi>` and `httpfiletools run <http-file>`. Defer aliases until core behavior is stable.
**Why:** This keeps the public CLI surface simple and separates CLI parsing from core orchestration.

### 2026-05-25T23:05:30.184+02:00: Runner compatibility
**By:** Christian Helle (via Copilot)
**What:** `httpfiletools run` should preserve `httprunner` CLI flags, semantics, donation/support banner behavior, default output, verbose output, `--pretty-json`, and banner-disable behavior.
**Why:** Exact runner compatibility should preserve existing automation and user habits from `httprunner`.

### 2026-05-25T23:05:30.184+02:00: CLI compatibility edge decisions
**By:** Linus
**What:** Preserve upstream subcommand-specific flag meanings: `generate -v` means version because `httpgenerator -v` means version, while `run -v` means verbose because `httprunner -v` means verbose. Do not reserve a root-level `-v` shortcut. Treat `run --upgrade` as unresolved packaging behavior for the suite rather than implementing standalone `httprunner` self-update blindly.
**Why:** Keeping meanings scoped to subcommands maximizes compatibility while avoiding a misleading global flag contract or package-manager side effects.

### 2026-05-25T23:05:30.184+02:00: Golden compatibility test scope
**By:** Christian Helle (via Copilot)
**What:** Capture fixture-based golden tests for stdout, stderr, exit codes, generated `.http` content, and key error cases from `httprunner` and `httpgenerator`.
**Why:** Compatibility is a core requirement, and visible behavior should be locked before implementation changes can drift from upstream.

### 2026-05-25T23:05:30.184+02:00: HTTP integration test strategy
**By:** Christian Helle (via Copilot)
**What:** Use local deterministic test servers/fixtures by default for runner integration tests, and reserve live external HTTP calls for ignored/manual tests.
**Why:** This keeps default `cargo test` reliable while preserving a path for explicit real-network compatibility checks.

### 2026-05-25T23:05:30.184+02:00: Initial compatibility test plan and fixture inventory
**By:** Yen
**What:** Golden tests should cover CLI shape/help, generator outputs and errors, runner outputs and errors, and narrow compatibility guards. Fixtures should include representative OpenAPI 2.0/3.0/3.1 documents, invalid inputs, local HTTP runner cases, and committed golden stdout/stderr/content files with volatile data normalized.
**Why:** The suite needs deterministic default tests that assert orchestration behavior and visible compatibility without depending on external network calls or reimplementing upstream internals.

### 2026-05-25T23:05:30.184+02:00: Keep `httprunner-core` default features enabled
**By:** Rusty
**What:** Use `httprunner-core = "0.9.51"` with its default features enabled in the initial workspace foundation.
**Why:** Disabling default features caused upstream compile errors in `telemetry::tracking`; keeping defaults preserves compatibility with the published crate and keeps the workspace buildable.

## Governance

- All meaningful changes require team consensus.
- Document architectural decisions here.
- Keep history focused on work, decisions focused on direction.

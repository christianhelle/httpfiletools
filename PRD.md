# Product Requirements Document — httpfiletools

**Status:** Draft baseline  
**Repository:** `christianhelle/httpfiletools`  
**Date:** 2026-04-05  
**Audience:** maintainers, contributors, reviewers, and AI coding agents  
**Purpose:** define the product, architecture, scope, delivery plan, and agent operating model for a Rust-native suite of `.http` file tools

---

## 1. Executive summary

`httpfiletools` is a new Rust-native product that combines two existing tool lines into a single coherent workspace:

1. **HTTP File Runner** (`christianhelle/httprunner`) — parses and executes `.http` files, evaluates assertions, manages variables, and produces logs/reports. Published as the Rust crate `httprunner-core`.
2. **HTTP File Generator** (`christianhelle/httpgenerator`) — generates `.http` files from OpenAPI specifications, with options for output layout, headers, auth, and IDE-friendly test snippets. Rewritten in Rust and published as the `httpgenerator-core` crate.

Both source systems are now Rust-based foundation crates. The new product must preserve the practical value of both tools by consuming those foundations directly rather than reimplementing them, and remove the prior split-language and split-maintenance burden. The resulting repository should be suitable for both human development and **repeatable experimentation with multiple models and agent workflows**, which means the product contract, architecture boundaries, and acceptance criteria must be unusually explicit.

The first release of `httpfiletools` is **CLI-first**. The priority is a high-quality Rust workspace with:

- a shared `.http` parsing and data model inherited from `httprunner-core`,
- a robust **OpenAPI-to-`.http` generator** built directly on `httpgenerator-core`,
- a reliable **`.http` runner** built directly on `httprunner-core`,
- deterministic outputs and golden tests,
- cross-platform support for Windows, macOS, and Linux,
- documentation and team structure that allow different agents/models to work from the same source of truth.

---

## 2. Background and source systems

### 2.1 Source system A — HTTP File Runner

`httprunner` is already written in Rust, publishes the `httprunner-core` crate on crates.io, and provides a strong execution foundation. Its published feature set includes:

- parsing and executing `.http` files,
- multiple-file execution and recursive discovery,
- verbose output and pretty-printed JSON,
- report generation,
- request/response export,
- assertions,
- variables and request variables,
- environment files and environment selection,
- built-in dynamic functions,
- conditional execution via directives such as `@dependsOn` and `@if`,
- configurable delays and timeout directives,
- insecure HTTPS support,
- CLI, TUI, and GUI surfaces,
- an existing Rust workspace split across `core`, `cli`, `tui`, and `gui`.

The new repository should treat `httprunner` as the primary source of truth for **runtime behavior**, **Rust project patterns**, and the current `.http` execution dialect. For MVP, the published `httprunner-core` crate should be consumed directly wherever it already satisfies the required contract.

### 2.2 Source system B — HTTP File Generator

`httpgenerator` has been rewritten in Rust and publishes the `httpgenerator-core` crate. It owns the OpenAPI-to-`.http` generation engine, including:

- local-file and URL-based OpenAPI input,
- OpenAPI parsing, normalization, and operation naming,
- output layout selection equivalent to `OneRequestPerFile`, `OneFile`, and `OneFilePerTag`,
- literal and environment-variable authorization headers,
- custom header injection,
- content-type overrides,
- base URL overrides,
- skipping generated headers,
- per-request timeout metadata,
- IntelliJ/REST Client test block generation,
- in-memory rendering of generated files exposed via a `generate_http_files` entry point that returns a `GeneratorResult { files: Vec<HttpFile> }` with `HttpFile { filename, content }` so the consumer owns file-system writes.

For MVP, the published `httpgenerator-core` crate should be consumed directly wherever it already satisfies the required contract. This repository must not reimplement OpenAPI loading, normalization, operation naming, request rendering, or layout policy unless a specific gap is documented and an adapter is added behind the generator integration boundary.

Product-layer concerns that remain outside `httpgenerator-core` and stay owned by this repository include: file-system persistence, dry-run semantics, parse-back validation of generated content via `httprunner-core`, Azure scope/tenant token acquisition for protected OpenAPI fetches, `skip-validation` policy, unified CLI ergonomics, redaction policy, and telemetry/support-key wrapper behavior. The legacy C# implementation, the Visual Studio extension, and the historical telemetry pattern are not inherited by default.

### 2.3 Why a new unified product

Even with both engines now in Rust, splitting their distribution across two product surfaces creates friction:

1. **Inconsistent product UX** — users assemble generation and execution workflows by hand across two separately-released CLIs.
2. **Behavior drift risk** — generated files and runnable files can diverge unless both tools share one parser/dialect contract and one product-level integration story.
3. **Testing fragmentation** — parity and regression testing for end-to-end "generate then run" workflows currently spans repos and release cadences.
4. **Agent friction** — experiments with AI tooling are harder when product scope, contracts, and release surfaces are spread across multiple repositories.

Unifying the product surface on top of `httpgenerator-core` and `httprunner-core` keeps each engine independently maintainable while giving users one coherent CLI, one shared parser/dialect, and one fixture/test surface for parity work.

---

## 3. Product vision

Build the best Rust-native toolkit for working with `.http` files:

- **Generate** them from OpenAPI,
- **run** them locally or in CI,
- keep the format **human-readable and IDE-friendly**,
- make the toolchain **predictable, scriptable, and cross-platform**,
- and expose a repository structure that is easy for both humans and agents to reason about.

---

## 4. Problem statement

API developers and testers often need both of the following:

1. a quick way to turn an OpenAPI contract into editable `.http` files, and
2. a reliable way to execute those `.http` files with assertions, variable substitution, and reporting.

Today those capabilities exist in separate repositories with different implementation languages and different maintenance surfaces. This causes duplicated effort, inconsistent evolution, and a weak end-to-end story for workflows like:

- fetch OpenAPI spec,
- generate `.http` files,
- edit or enrich the generated requests,
- run them locally,
- run them in CI,
- compare results,
- iterate safely.

The new product must unify these workflows without losing the useful features already proven in the existing tools.

---

## 5. Goals

### 5.1 Primary goals

1. **Rust-only codebase** for all production functionality in this repository.
2. **Shared core model** so generated `.http` files are valid runner inputs by construction.
3. **CLI-first MVP** that supports both generation and execution.
4. **Feature parity where it matters most** with the existing generator and runner.
5. **Deterministic outputs** suitable for golden tests and code review.
6. **Strong Windows support** in both implementation and UX.
7. **Repository clarity** so multiple models/agents can work against a stable product contract.

### 5.2 Secondary goals

1. Preserve a migration path for users of `httprunner` and `httpgenerator`.
2. Keep the architecture open to future TUI/GUI surfaces.
3. Enable fixture-driven, benchmarkable development workflows for agent experiments.

---

## 6. Non-goals

The following are explicitly **not required for MVP** unless later promoted:

1. Porting the existing Visual Studio extension.
2. Shipping TUI or GUI parity on day one.
3. Building a general-purpose OpenAPI SDK/code generator (this product consumes `httpgenerator-core`; it does not reimplement an OpenAPI toolchain).
4. Supporting non-HTTP protocols.
5. Activating an outbound telemetry endpoint in MVP builds. The MVP ships the support-key display and redacted feature/error telemetry envelopes (default-on, disabled by `--no-logging`) with a pluggable sink wired to no remote destination; enabling an endpoint is a deliberate later decision.
6. Designing a remote SaaS control plane or cloud service.
7. Building a plugin marketplace before core generation and execution are stable.

---

## 7. Target users and jobs-to-be-done

| User | Primary need | What success looks like |
|------|--------------|-------------------------|
| API developer | Generate editable `.http` files from an OpenAPI contract | They can generate files in seconds, inspect them, tweak variables, and run them immediately |
| QA / test engineer | Execute `.http` suites repeatedly with assertions and reports | They can run many files in CI, get stable exit codes, and inspect readable reports |
| Platform / backend engineer | Validate endpoints against a contract quickly | They can generate and run contract-derived requests without writing custom scripts |
| Maintainer migrating from existing tools | Preserve known flags and workflows with less maintenance overhead | They can map old commands to the new suite without relearning everything |
| AI coding agent | Work from a precise contract with stable boundaries | It can implement one slice at a time without inventing architecture or scope |

### Jobs to be done

1. **When I have an OpenAPI document, I want to generate `.http` files that are readable and runnable, so I can explore and test the API quickly.**
2. **When I have `.http` files, I want to execute them with variables, assertions, and reporting, so I can automate API validation.**
3. **When I use the tool in CI, I want deterministic exit codes and artifacts, so builds are actionable rather than ambiguous.**
4. **When I ask an AI agent to work on the repo, I want the scope, interfaces, and review gates to be explicit, so experiments are comparable.**

---

## 8. Product principles

1. **Generated files must be runnable.**  
   The generator is not done when it writes text; it is done when the runner can parse and execute the output.

2. **One shared `.http` dialect.**  
   There must be one canonical parser/data model for the project, not separate dialect implementations that drift over time.

3. **CLI-first, human-readable, script-friendly.**  
   The core product must work well in terminals, scripts, and CI before any secondary surface is prioritized.

4. **Determinism over cleverness.**  
   File names, ordering, spacing, and rendered sections should be stable between runs for the same inputs.

5. **Privacy-first defaults.**  
   Secrets must be redacted in logs/reports, and telemetry must not be assumed.

6. **Cross-platform by default.**  
   Windows is a first-class environment, not a compatibility target after Linux/macOS.

7. **Architecture should support delegation.**  
   Crate boundaries and contracts should make it easy to assign work cleanly to humans and agents.

---

## 9. Scope by release

### 9.1 MVP scope

MVP includes:

- Rust workspace with thin integration crates over `httpgenerator-core` and `httprunner-core`,
- unified CLI surface,
- generator subcommand backed by `httpgenerator-core`,
- runner subcommand backed by `httprunner-core`,
- shared `.http` parser/data model via `httprunner-core`,
- deterministic file rendering inherited from `httpgenerator-core`, with parse-back validation through `httprunner-core` before files are written,
- product-layer dry-run, file-writing, and result-shaping semantics,
- Azure scope/tenant token acquisition for protected OpenAPI fetches as a product wrapper around `httpgenerator-core`,
- reports/logging/export and redaction policy on top of `httprunner-core`,
- support-key display by default; redacted feature/error telemetry envelopes enabled by default, disabled with `--no-logging`, with no telemetry endpoint enabled in the MVP build and a pluggable sink retained for later activation,
- fixture-driven tests using a small smoke suite with provenance metadata rather than vendoring the full upstream `httpgenerator-core` corpus,
- migration guidance from the source tools,
- explicit squad/team routing documentation.

Milestones 0 through 3 establish the implementation foundation, but the MVP is not considered **shippable** until Milestone 4 closes the unified CLI, docs, migration, and packaging loop.

### 9.2 Post-MVP scope

Post-MVP candidates:

- TUI parity,
- GUI parity or new GUI direction,
- compatibility wrapper binaries for legacy command names,
- richer config file support,
- JSON reports and machine-readable summaries beyond the first CLI release,
- plugin/extensibility surfaces,
- IDE integrations or extension ports.

### 9.3 Explicitly deferred until proven necessary

- Visual Studio extension rewrite,
- full WASM support for the generator,
- generalized auth provider plugin system,
- remote collaboration/cloud sync features.

---

## 10. Feature traceability from the source repos

| Capability | Source repo | Target status | Notes |
|-----------|-------------|---------------|-------|
| Parse `.http` files | `httprunner` | **Carry forward** | Canonical parser behavior should come from the Rust runner lineage |
| Execute `.http` files | `httprunner` | **Carry forward** | Core runtime behavior |
| Execute multiple files | `httprunner` | **Carry forward** | Must remain CLI-friendly |
| Recursive discovery | `httprunner` | **Carry forward** | Useful for CI and project-wide suites |
| Verbose output | `httprunner` | **Carry forward** | Preserve readable detail |
| Pretty-printed JSON output | `httprunner` | **Carry forward** | Keep for debugging |
| Log file generation | `httprunner` | **Carry forward** | Must redact secrets |
| Markdown/HTML reports | `httprunner` | **Carry forward** | MVP includes markdown and HTML |
| Export requests/responses | `httprunner` | **Carry forward** | Useful for debugging and audit trails |
| Delays | `httprunner` | **Carry forward** | Support global and per-request controls |
| Request timeouts | `httprunner` | **Carry forward** | Preserve `@timeout` and `@connection-timeout` semantics where practical |
| Assertions | `httprunner` | **Carry forward** | Must remain a first-class runtime concept |
| Variables and request variables | `httprunner` | **Carry forward** | Required for realistic suites |
| Environment files | `httprunner` | **Carry forward** | Preserve `http-client.env.json` loading and named environment selection |
| Built-in functions | `httprunner` | **Carry forward** | Prefer compatibility where reasonable |
| Conditional execution | `httprunner` | **Carry forward** | Preserve `@dependsOn`/`@if` semantics where possible |
| Insecure HTTPS mode | `httprunner` | **Carry forward** | Useful in dev/test environments |
| TUI | `httprunner` | **Defer** | Not part of MVP |
| GUI | `httprunner` | **Defer** | Not part of MVP |
| OpenAPI from file | `httpgenerator` | **Inherited from `httpgenerator-core`** | Engine-owned input loading |
| OpenAPI from URL | `httpgenerator` | **Inherited from `httpgenerator-core`** | Engine-owned input loading |
| Output directory | `httpgenerator` | **Wrapper/product-layer requirement** | `httpgenerator-core` returns in-memory `HttpFile { filename, content }`; this product owns file-system writes |
| Output types | `httpgenerator` | **Inherited from `httpgenerator-core`** | One request per file, one file, one file per tag |
| Skip validation | `httpgenerator` | **Wrapper/product-layer requirement** | Escape hatch surfaced via CLI; routed into `httpgenerator-core` options |
| Authorization header injection | `httpgenerator` | **Inherited from `httpgenerator-core`** | Direct header input supported |
| Authorization from environment variable | `httpgenerator` | **Inherited from `httpgenerator-core`** | Strongly preferred over embedded secrets |
| Base URL override | `httpgenerator` | **Inherited from `httpgenerator-core`** | Needed when specs omit or misuse servers |
| Content-Type override | `httpgenerator` | **Inherited from `httpgenerator-core`** | Preserved by the engine |
| Custom headers | `httpgenerator` | **Inherited from `httpgenerator-core`** | Useful for generated requests |
| Skip generated headers | `httpgenerator` | **Inherited from `httpgenerator-core`** | Preserved by the engine |
| Azure scope / tenant auth support | `httpgenerator` | **Wrapper/product-layer requirement** | Not owned by `httpgenerator-core`; this product wraps token acquisition before invoking the engine for protected OpenAPI fetches |
| Output timeout | `httpgenerator` | **Inherited from `httpgenerator-core`** | Per-request timeout metadata exposed by the engine |
| IntelliJ test generation | `httpgenerator` | **Inherited from `httpgenerator-core`** | Engine generates the test blocks; product surfaces a CLI toggle |
| Telemetry and support key | `httpgenerator` | **Wrapper/product-layer requirement (forward-compatible)** | Support key shown by default; redacted feature/error telemetry envelopes enabled by default and disabled by `--no-logging`; no telemetry endpoint enabled in MVP builds; pluggable sink retained |
| Visual Studio extension | `httpgenerator` | **Defer** | Out of MVP |

---

## 11. Functional requirements

### 11.1 Repository and packaging requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| REP-001 | All shipped product binaries and reusable production crates in this repository must be implemented in Rust; CI/workflow files, docs, and release scripts may use standard ecosystem formats and tooling. | Must |
| REP-002 | The project must be structured as a Rust workspace with clear crate boundaries. | Must |
| REP-003 | Shared behavior between generator and runner must live in reusable crates rather than binary-specific code. | Must |
| REP-004 | The repository must contain fixture inputs for OpenAPI specs and `.http` files. | Must |
| REP-005 | The repository must include golden-output fixtures for generation tests. | Must |
| REP-006 | The repository must include documentation sufficient for human and agent onboarding. | Must |
| REP-007 | Windows, macOS, and Linux must be supported in CI. | Must |
| REP-008 | Release packaging should target GitHub Releases and `cargo install`. | Should |
| REP-009 | The repository must consume `httprunner-core` as the canonical `.http` parser/execution foundation for MVP and must not reimplement equivalent runner-core behavior unless a specific gap is documented. | Must |
| REP-010 | The repository must consume `httpgenerator-core` as the canonical OpenAPI-to-`.http` generation foundation for MVP and must not reimplement OpenAPI loading, normalization, operation naming, request rendering, or output-layout policy unless a specific gap is documented and isolated behind the generator integration crate. | Must |
| REP-011 | `httpgenerator-core` and `httprunner-core` must be exact-pinned (`=x.y.z`) in this repository through MVP; upgrades to either core crate are deliberate, snapshot-reviewed events. | Must |

### 11.2 Generator requirements

These requirements describe the product-level generator contract. Items marked **Inherited** are satisfied by `httpgenerator-core` and must not be reimplemented in this repository; items marked **Wrapper** are owned by the `crates/generator/` integration crate and the CLI on top of it.

| ID | Requirement | Source | Priority |
|----|-------------|--------|----------|
| GEN-001 | The generator must accept an OpenAPI input from a local file path. | Inherited (`httpgenerator-core`) | Must |
| GEN-002 | The generator must accept an OpenAPI input from a URL. | Inherited (`httpgenerator-core`) | Must |
| GEN-003 | The generator must support JSON and YAML source documents. | Inherited (`httpgenerator-core`) | Must |
| GEN-004 | The generator must support OpenAPI 3.x and should preserve practical compatibility with Swagger/OpenAPI 2.0 inputs to the extent supported by `httpgenerator-core`; gaps must be documented rather than worked around. | Inherited (`httpgenerator-core`) | Must |
| GEN-005 | The generator must support `--output` for selecting the output location, and must own writing the in-memory `HttpFile` results returned by `httpgenerator-core` to disk. | Wrapper | Must |
| GEN-006 | The generator must support output layout modes equivalent to one-request-per-file, one-file, and one-file-per-tag. | Inherited (`httpgenerator-core`) | Must |
| GEN-007 | File names must be deterministic and stable for identical inputs. | Inherited (`httpgenerator-core`) | Must |
| GEN-008 | The generator must support `--base-url` as an override when server information is absent or unsuitable. | Inherited (`httpgenerator-core`) | Must |
| GEN-009 | The generator must support a default `Content-Type` override. | Inherited (`httpgenerator-core`) | Must |
| GEN-010 | The generator must support one or more custom headers added to generated requests. | Inherited (`httpgenerator-core`) | Must |
| GEN-011 | The generator must optionally omit generated header parameters. | Inherited (`httpgenerator-core`) | Must |
| GEN-012 | The generator must support an explicit authorization header value. | Inherited (`httpgenerator-core`) | Must |
| GEN-013 | The generator must support generating files that load the authorization header from an environment variable, including a user-selectable variable name. | Inherited (`httpgenerator-core`) | Must |
| GEN-014 | The generator must support Azure scope-based token acquisition and optional tenant selection for protected OpenAPI fetches. Token acquisition is product-wrapper behavior; the resulting bearer is passed into `httpgenerator-core` as an authorization header. | Wrapper | Should |
| GEN-015 | The generator must support skipping schema validation. The CLI flag is wrapper-owned; the behavior is routed into `httpgenerator-core` options. | Wrapper | Must |
| GEN-016 | The generator must render request bodies, path variables, query variables, and header variables in a readable, editable form. | Inherited (`httpgenerator-core`) | Must |
| GEN-017 | The generator must include summary and description comments when available. | Inherited (`httpgenerator-core`) | Must |
| GEN-018 | The generator must support optional IntelliJ/REST Client style test snippet output. | Inherited (`httpgenerator-core`) | Should |
| GEN-019 | The generator must support a preview or dry-run path that avoids partial writes when desired. Dry-run is product-wrapper behavior implemented by withholding the file-write step on the in-memory `GeneratorResult`. | Wrapper | Should |
| GEN-020 | The generator must fail clearly when remote specs cannot be fetched or parsed, surfacing the original `httpgenerator-core` diagnostic without flattening it. | Wrapper | Must |
| GEN-021 | The generator must not silently embed secrets into files when an environment-based pattern is requested. | Inherited (`httpgenerator-core`) | Must |
| GEN-022 | Generated `.http` output must be parseable by the same project parser used by the runner. The generator integration crate must parse-back every `HttpFile.content` returned by `httpgenerator-core` with `httprunner-core` before writing to disk and must fail the run on parse-back errors. | Wrapper | Must |

For MVP planning, **GEN-014**, **GEN-018**, and **GEN-019** are stretch items rather than release gates unless they are later promoted by an explicit product decision.

Any behavior that appears to require changing OpenAPI loading, normalization, naming, rendering, or layout policy must be raised as a documented gap against `httpgenerator-core`. The default response is to file the gap upstream or add an isolated adapter inside `crates/generator/`, not to rewrite engine behavior locally.

### 11.3 Runner requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| RUN-001 | The runner must execute a single `.http` file. | Must |
| RUN-002 | The runner must execute multiple `.http` files in one invocation. | Must |
| RUN-003 | The runner must support recursive discovery of `.http` files. | Must |
| RUN-004 | The runner must support variable substitution in URLs, headers, and bodies. | Must |
| RUN-005 | The runner must support request variables for chaining values across requests. | Must |
| RUN-006 | The runner must support the existing built-in function set where practical, or document any gaps explicitly. | Must |
| RUN-007 | The runner must support response assertions for status code, body content, and headers. | Must |
| RUN-008 | The runner must support conditional execution semantics compatible with the current runner directives. | Must |
| RUN-009 | The runner must support request delays, including global CLI delay and per-request delay directives. | Must |
| RUN-010 | The runner must support verbose output. | Must |
| RUN-011 | The runner must support pretty-printed JSON in verbose mode. | Must |
| RUN-012 | The runner must support insecure HTTPS for development scenarios. | Must |
| RUN-013 | The runner must support log-file output. | Must |
| RUN-014 | The runner must support summary reports in markdown and HTML. | Must |
| RUN-015 | The runner must support exporting request/response artifacts. | Must |
| RUN-016 | The runner must emit clear per-file and overall summary statistics. | Must |
| RUN-017 | The runner must redact secrets in logs and exported artifacts where feasible. | Must |
| RUN-018 | The runner must return stable exit codes for success, assertion failure, parse failure, and operational failure. | Must |
| RUN-019 | The runner should support fail-fast behavior as an optional mode. | Should |
| RUN-020 | The runner should expose a machine-readable summary format after MVP if not included in MVP. | Could |
| RUN-021 | The runner must support timeout directives compatible with the current runner dialect, including read-timeout and connection-timeout behavior. | Must |
| RUN-022 | The runner must execute `.http` files produced by the generator without manual normalization when the generated output stays within the documented MVP contract. | Must |
| RUN-023 | The runner must support environment-file loading compatible with `http-client.env.json` and named environment selection from the CLI. | Must |

### 11.4 Shared `.http` language requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| LANG-001 | There must be one canonical parser implementation used by both generator validation and runner execution. For MVP, that canonical parser should come from `httprunner-core` unless a documented gap requires a thin adapter. | Must |
| LANG-002 | The generator must only emit syntax that the canonical parser supports. | Must |
| LANG-003 | The parser and renderer must accept LF and CRLF inputs safely, and the repository must document a normalized line-ending policy for checked-in fixtures. | Must |
| LANG-004 | The parser must preserve enough structure to support readable error messages with file/line context. | Must |
| LANG-005 | The parser must distinguish syntax errors from execution errors. | Must |
| LANG-006 | The project must maintain compatibility fixtures for known `.http` dialect behaviors that users depend on. | Should |

### 11.5 Authentication and secret handling requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| AUTH-001 | Direct authorization-header input must be supported for generation and remote-fetch scenarios. | Must |
| AUTH-002 | Environment-based authorization header loading must be supported. | Must |
| AUTH-003 | Azure scope-based token acquisition should be supported for generation scenarios that fetch protected OpenAPI specs. | Should |
| AUTH-004 | Secrets must be redacted in human-readable logs/reports by default. | Must |
| AUTH-005 | Generated files should prefer environment-variable placeholders over hard-coded secrets when the user requests that pattern. | Must |
| AUTH-006 | If multiple authentication sources or patterns are requested together, the CLI must either enforce a documented precedence order or reject ambiguous combinations explicitly. | Must |

For MVP planning, **AUTH-003** is a stretch item rather than a ship gate unless it is later promoted by an explicit product decision.

### 11.6 Documentation requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| DOC-001 | The repository must include a root README with install and quick-start instructions. | Must |
| DOC-002 | The repository must include examples for generator and runner flows. | Must |
| DOC-003 | The repository must include a migration guide from `httprunner` and `httpgenerator`. | Should |
| DOC-004 | CLI help text must be complete enough to act as a primary reference for common flows. | Must |
| DOC-005 | The repository must include explicit squad/team routing docs to support model and agent experimentation. | Must |

---

## 12. Proposed CLI contract

### 12.1 Primary binary

The recommended MVP binary is:

```text
httpfiletools
```

with at least these subcommands:

```text
httpfiletools generate <spec> [options]
httpfiletools run <paths...> [options]
httpfiletools version
httpfiletools help
```

### 12.2 `generate` subcommand

Proposed baseline:

```text
httpfiletools generate <spec>
  --output <path>
  --output-type <one-request-per-file|one-file|one-file-per-tag>
  --base-url <url>
  --content-type <mime-type>
  --authorization-header <value>
  --load-authorization-header-from-environment
  --authorization-header-variable-name <name>
  --azure-scope <scope>
  --azure-tenant-id <tenant-id>
  --custom-header <name:value>
  --skip-headers
  --skip-validation
  --generate-intellij-tests
  --timeout <seconds>
  --dry-run
```

`--timeout` in the generator contract refers to generation/file-writing operations, not runner request-execution timeouts.

### 12.3 `run` subcommand

Proposed baseline:

```text
httpfiletools run <paths...>
  --discover
  --verbose
  --pretty-json
  --delay <milliseconds>
  --env <name>
  --insecure
  --log [file]
  --report [markdown|html]
  --export [directory]
  --fail-fast
  --no-banner
```

### 12.4 CLI design notes

1. **A unified binary is preferred for MVP.**  
   It fits the new repository identity and simplifies docs, packaging, and discovery.

2. **Legacy command-name compatibility is desirable but not required for MVP.**  
   If needed later, thin wrappers or aliases may map:
   - `httprunner` → `httpfiletools run`
   - `httpgenerator` → `httpfiletools generate`

3. **Human-readable output remains the default.**  
   Machine-readable outputs can be added later without degrading human UX.

4. **Exit codes must be documented and stable.**

5. **Artifact flags must have deterministic destination semantics.**  
   If optional-value forms such as `--log`, `--report`, or `--export` are retained, their default locations and naming rules must be documented and testable.

### 12.5 Exit-code baseline

Unless superseded by an explicit ADR or product decision, the CLI should treat the following exit-code contract as the MVP baseline:

| Code | Meaning | Notes |
|------|---------|-------|
| `0` | Success | All requested work completed without parse, assertion, or operational failure |
| `1` | Assertion failure | One or more requests executed but the documented assertions failed |
| `2` | Usage/input error | Invalid CLI arguments or other user-input problems that prevent a meaningful run |
| `3` | Parse failure | `.http` or related structured input could not be parsed according to the documented dialect |
| `4` | Operational/runtime failure | Network, TLS, auth, file-system, or other execution-time failure |

Unexpected internal failures should currently surface as operational/runtime failures until a separate internal-error code is explicitly introduced.

---

## 13. Proposed architecture

### 13.1 Workspace layout

Recommended initial layout:

```text
crates/
  generator/    # thin integration over httpgenerator-core: CLI option translation,
                # Azure token acquisition wrapper, parse-back validation via httprunner-core,
                # dry-run + file-writing, product result shaping, documented gap adapters only
  runner/       # thin integration over httprunner-core for reports, export, redaction, and result shaping
  cli/          # clap surface, output formatting, fs/network orchestration
fixtures/
  openapi/      # small smoke-suite input specs with provenance metadata
  http/         # runnable sample .http files
  golden/       # expected generated outputs (product-layer snapshots, not a mirror of upstream)
docs/           # optional supporting docs later
.squad/         # team routing, charters, decisions
```

`httprunner-core` and `httpgenerator-core` are required external dependencies and the canonical foundations for runner and generator behavior in MVP. Both are exact-pinned during MVP.

### 13.2 Foundation library and crate responsibilities

#### External foundation — `httprunner-core`

Owns:

- `.http` AST and supporting types,
- parser,
- parser diagnostics,
- request execution,
- shared variable model,
- request-variable handling,
- built-in functions, conditions, delays, timeout directives, and environment-file loading exposed by the published crate,
- baseline runtime result modeling and common errors provided by the library.

Does **not** own:

- OpenAPI parsing,
- product-specific report/export formatting for this repository,
- unified CLI ergonomics for this repository.

#### External foundation — `httpgenerator-core`

Owns:

- OpenAPI source loading (file/URL),
- OpenAPI parser/crate selection and normalization,
- operation naming,
- request template generation and rendering of `.http` text,
- output layout selection (one-request-per-file, one-file, one-file-per-tag),
- file-name determinism rules,
- header/auth/base-url/content-type/custom-header/skip-headers/per-request-timeout behavior,
- IntelliJ/REST Client test snippet generation,
- the in-memory `generate_http_files` entry point that returns `GeneratorResult { files: Vec<HttpFile> }` with `HttpFile { filename, content }`.

Does **not** own:

- file-system writes (the consumer persists the returned `HttpFile` set),
- dry-run policy,
- parse-back validation against `httprunner-core`,
- Azure scope/tenant token acquisition for protected OpenAPI fetches,
- unified CLI ergonomics, redaction policy, or telemetry behavior for this repository.

#### `generator`

Owns:

- thin integration over `httpgenerator-core`: translating CLI options into `httpgenerator-core` configuration,
- invoking `generate_http_files` and consuming `GeneratorResult`,
- parse-back validating each `HttpFile.content` with `httprunner-core` before writing,
- dry-run semantics and file-system writes for the in-memory engine result,
- Azure scope/tenant token acquisition wrapper before invoking the engine,
- product-shaped result types returned to the CLI,
- any explicitly documented gap adapters required beyond `httpgenerator-core`.

Does **not** own:

- OpenAPI parsing, normalization, operation naming, request rendering, or output-layout policy (those live in `httpgenerator-core`),
- canonical `.http` parsing or runner execution behavior already provided by `httprunner-core`,
- terminal formatting decisions.

#### `runner`

Owns:

- composition over `httprunner-core` for repo-specific execution orchestration,
- logs, reports, and export artifacts,
- redaction policy and result translation for the unified product,
- any explicitly documented gap adapters required beyond the published crate.

Does **not** own:

- replacement parser/runtime behavior already available in `httprunner-core`,
- OpenAPI ingestion,
- CLI argument parsing,
- generator-specific rendering policies.

#### `cli`

Owns:

- `clap` command definitions,
- argument validation and precedence,
- human-readable output formatting,
- file-system orchestration,
- process exit codes,
- user-facing error presentation.

The CLI crate should be the only layer that knows about terminal UX and command ergonomics.

### 13.3 Core data-flow contracts

#### Generation flow

```text
OpenAPI input (file/URL)
  -> CLI option translation (crates/generator)
  -> Azure scope/tenant token acquisition wrapper if requested
  -> httpgenerator-core::generate_http_files(config)
       -> load/fetch + validate/normalize
       -> map operations to internal generation model
       -> render canonical .http text
       -> return GeneratorResult { files: Vec<HttpFile { filename, content }> }
  -> parse-back validate each HttpFile.content with httprunner-core
  -> dry-run? print plan and stop, else write files to --output
  -> emit product result to CLI
```

#### Execution flow

```text
.http input
  -> resolve environment selection and runtime options
  -> parse and execute via httprunner-core
  -> collect summary + artifacts
  -> render console output + reports
```

### 13.4 Dependency strategy

Recommended baseline:

- `httprunner-core` as the required parser/runtime dependency for the canonical `.http` dialect, exact-pinned (`=x.y.z`) through MVP,
- `httpgenerator-core` as the required OpenAPI-to-`.http` generation dependency, exact-pinned (`=x.y.z`) through MVP,
- `clap` for CLI parsing,
- `reqwest` for HTTP work this repository owns directly (e.g., Azure token acquisition); OpenAPI fetching is delegated to `httpgenerator-core`,
- `serde`, `serde_json`, and `serde_yaml` for data handling this repository owns directly,
- `thiserror` in library crates,
- `anyhow` only at binary/application boundaries,
- `tokio` only where it meaningfully improves correctness or ergonomics,
- `insta` for snapshot/golden testing,
- `assert_cmd` and `predicates` for CLI tests,
- `wiremock` or `httpmock` for local HTTP test servers.

Bumping either core crate is a deliberate, snapshot-reviewed event: the bump PR must re-run the smoke-suite goldens, surface any wrapper-visible diffs, and is reviewed by Walt (generator) or Jesse (runner) as applicable.

### 13.5 OpenAPI parser selection

OpenAPI parser selection is owned by `httpgenerator-core` and is not a sprint-0 decision for this repository. This product inherits whatever OpenAPI crate `httpgenerator-core` chooses, and benefits from upstream improvements there.

If a concrete OpenAPI-parsing gap is observed (e.g., a spec that the engine rejects but a user reasonably expects to work), the response is:

1. document the gap in this repository,
2. file it upstream against `httpgenerator-core`,
3. and, only if unavoidable for MVP, add a narrowly-scoped adapter inside `crates/generator/` rather than reimplementing OpenAPI parsing locally.

### 13.6 Error-handling strategy

1. Library crates return typed errors with context.
2. The CLI translates those errors into stable user-facing messages and exit codes.
3. Parse errors must include file and line context where possible.
4. Network/auth failures must not be flattened into generic “generation failed” or “run failed” messages.
5. No broad “best effort” silent fallbacks.

### 13.7 Determinism requirements

`httpgenerator-core` owns the rendering and naming policy that produces stable output for identical inputs and options. The integration crate must not introduce nondeterminism on top of the engine output — wrapper steps (parse-back, dry-run, file write) must preserve byte-for-byte the `HttpFile.content` returned by `httpgenerator-core`. For identical inputs and options, the product must remain stable across runs with respect to:

- file names,
- file-name collision resolution,
- request ordering and documented tie-breakers when source order is ambiguous,
- server/example selection when the source spec offers multiple valid candidates,
- variable ordering where controllable,
- header ordering where controllable,
- rendered section layout,
- trailing newlines and the normalized line-ending policy for checked-in fixtures.

If a determinism issue is traced into the engine, it is filed upstream against `httpgenerator-core` rather than patched by post-processing engine output.

### 13.8 Cross-platform requirements

The architecture must account for:

- Windows path handling,
- UTF-8 and console output behavior,
- CRLF vs LF issues in generated files and fixtures,
- TLS behavior differences,
- shell-independent command behavior in docs and tests.

---

## 14. Quality strategy

### 14.1 Test pyramid

#### Unit tests

Use for:

- parser tokens and directive handling,
- operation naming,
- variable substitution,
- renderer helpers,
- error mapping.

#### Snapshot / golden tests

Use for:

- product-layer rendering of representative `httpgenerator-core` outputs (wrapper integration, not a mirror of the upstream engine corpus),
- stable CLI help output where appropriate,
- representative generated request files.

Generator goldens are a **small smoke suite** that pins wrapper behavior (CLI option translation, dry-run, file-write, parse-back integration) using `httpgenerator-core` as the engine. Each golden fixture carries provenance metadata recording the source spec, the exact `httpgenerator-core` version used, and the relevant CLI options. The full upstream `httpgenerator-core` corpus is not vendored; engine-internal rendering regressions are an upstream concern.

#### Integration tests

Use for:

- end-to-end generator workflows,
- generate-then-run workflows that exercise the shared parser boundary,
- runner execution against a mock server,
- report/export generation,
- multi-file discovery behavior,
- environment-variable auth flows,
- redaction behavior for logs, reports, and exported artifacts.

#### Compatibility tests

Use for:

- source fixtures imported or derived from `httprunner` / `httprunner-core`,
- a small smoke set derived from `httpgenerator-core` examples, with provenance metadata; the full upstream corpus is intentionally not vendored,
- generated-file round-tripping through the parser,
- representative generated-file execution through the runner without manual edits.

### 14.2 CI requirements

CI should eventually include:

1. formatting,
2. linting,
3. unit/integration tests,
4. cross-platform matrix builds,
5. release packaging checks,
6. fixture/golden verification.

### 14.3 Core acceptance gates

No milestone is complete unless:

1. the relevant crate tests pass,
2. the CLI examples in docs remain accurate,
3. generated fixtures are deterministic,
4. representative generated outputs round-trip through `httprunner-core` and execute without manual normalization where applicable,
5. each generator integration test parse-back validates engine output with `httprunner-core` before any file is considered "shipped" by the test,
6. a reviewer validates that crate boundaries were respected and that no engine behavior (OpenAPI parsing, normalization, naming, rendering, layout) was reimplemented locally without a documented gap.

### 14.4 Quality bar for generated files

A generated file is acceptable when it is:

- syntactically valid,
- readable without machine post-processing,
- clearly labeled,
- safe with respect to secret handling,
- executable by the runner without manual normalization.

---

## 15. Migration and compatibility strategy

### 15.1 Migration from `httpgenerator`

The new product should preserve the spirit of the existing CLI where practical:

- same major concepts,
- familiar output layout names,
- familiar auth and base-url options,
- same general expectation of generated file readability.

Generation behavior itself is inherited from `httpgenerator-core`, so users moving from the standalone `httpgenerator` CLI should generally see equivalent generated output for equivalent flags. Where the unified CLI differs (e.g., subcommand layout, Azure token handling, dry-run, `--no-logging`), the repo must document:

- what changed,
- why it changed,
- how to express the same intent in the new CLI.

### 15.2 Migration from `httprunner`

The new product should preserve:

- direct reuse of `httprunner-core` wherever it already satisfies the MVP contract,
- `.http` parsing semantics users rely on,
- runner directives and variable capabilities,
- reporting and logging value,
- overall feel of “run file(s), get readable results.”

TUI and GUI are not MVP requirements, but the core architecture should avoid blocking them forever.

### 15.3 Compatibility principle

Compatibility is strongest when the generator and runner share the same parser and data model. For MVP, that shared contract comes from `httprunner-core`, and generation behavior comes from `httpgenerator-core`. The project should prefer **shared contracts and upstream fixes** over compatibility shims or local reimplementations whenever possible.

---

## 16. Delivery plan and milestones

Milestones 0 through 3 establish the engine and shared contracts. The first **shippable MVP** is only considered complete once Milestone 4 exit criteria are met.

### Milestone 0 — Foundations

Deliverables:

- workspace skeleton,
- crate boundaries and external dependency boundaries,
- initial ADRs/decisions,
- fixture directory layout,
- squad roster and routing,
- PRD checked into repo.

Exit criteria:

- no unresolved ambiguity about crate ownership,
- no unresolved ambiguity about CLI-first MVP direction,
- initial testing strategy documented.

### Milestone 1 — Shared core

Deliverables:

- `httprunner-core` and `httpgenerator-core` dependency integration with exact version pins,
- documented coverage/gap analysis against the PRD runner and generator contracts,
- any thin adapter boundaries needed for generator validation or unified CLI integration,
- core fixture coverage and provenance metadata convention for generator goldens.

Exit criteria:

- baseline parser/execution fixtures pass against `httprunner-core`,
- the generator integration crate can invoke `httpgenerator-core::generate_http_files` and parse-back validate every returned `HttpFile.content` with `httprunner-core`,
- local crates can depend on the published foundations without reimplementing runner or generator engine internals.

### Milestone 2 — Generator MVP

Deliverables:

- CLI option translation into `httpgenerator-core` configuration,
- Azure scope/tenant token acquisition wrapper for protected OpenAPI fetches,
- output layout passthrough,
- header/auth/base-url passthrough,
- dry-run and file-writing semantics on top of the in-memory `GeneratorResult`,
- parse-back validation in the integration crate,
- smoke-suite snapshot tests with provenance metadata.

Exit criteria:

- sample OpenAPI fixtures generate stable `.http` outputs through `httpgenerator-core`,
- generated outputs parse successfully with `httprunner-core` before being written,
- no engine behavior (OpenAPI parsing, naming, rendering, layout) is reimplemented in `crates/generator/`.

### Milestone 3 — Runner MVP

Deliverables:

- runner integration pipeline on top of `httprunner-core`,
- variables, assertions, and environment selection,
- delays, conditions, and timeouts,
- logs/reports/export.

Exit criteria:

- representative suites execute successfully in integration tests,
- exit codes and reports are stable.

### Milestone 4 — Unified CLI, docs, and MVP ship gate

Deliverables:

- polished subcommand UX,
- docs and examples,
- migration notes,
- release packaging path.

Exit criteria:

- a new user can install, generate, and run using only repo docs,
- a reviewer can map source-tool behavior to the new suite.

### Milestone 5 — Post-MVP expansion

Candidates:

- TUI or GUI,
- wrapper binaries,
- richer config,
- extended reporting,
- IDE integrations.

---

## 17. Risks and mitigations

| Risk | Why it matters | Mitigation |
|------|----------------|------------|
| `httpgenerator-core` gap or regression | Generator parity may be blocked by upstream behavior | Exact-pin the engine, run smoke-suite goldens on every bump, file gaps upstream, and isolate any required adapter behind `crates/generator/` |
| Divergent `.http` dialects | Generator output may not run cleanly | Parse-back validate every `HttpFile.content` from `httpgenerator-core` with `httprunner-core` before writing |
| Local re-implementation drift | Wrapper crate accidentally reinvents engine behavior | Reviewer gate explicitly rejects OpenAPI parsing, normalization, naming, rendering, or layout code in `crates/generator/` without a documented gap |
| Scope creep from runner extras | TUI/GUI and advanced UX could delay MVP | Keep MVP CLI-only and defer secondary surfaces |
| Auth complexity | Azure/token flows can grow quickly | Keep Azure token acquisition narrowly scoped to the generator wrapper for protected OpenAPI fetches |
| Determinism drift | Golden tests become noisy and reviews expensive | Treat engine output as byte-for-byte authoritative; only the wrapper's own side-effects are under our determinism control |
| Core-crate bumps invisibly change output | Snapshot goldens silently update | Exact-pin both core crates; bump PRs must show diffs and are reviewed by Walt or Jesse as applicable |
| Windows regressions | Primary user workflows may break silently | Treat Windows as first-class in docs, tests, and path handling |
| Secret leakage in logs | Security/privacy issue | Redact by default; support-key shown by default; redacted telemetry envelopes default-on but disabled by `--no-logging`; no endpoint enabled in MVP |
| Agent drift | Different models may invent incompatible solutions | Use this PRD plus squad docs as the implementation contract |

---

## 18. AI/agent operating model

This repository is intentionally being set up for experiments with different models and agent tooling. That means product work must be partitionable into stable, reviewable work packets.

### 18.1 Team roles

| Agent | Role | Primary ownership |
|-------|------|-------------------|
| Gus | Tech Lead | workspace topology, architecture, dependency policy, cross-cutting decisions |
| Mike | Core / Parser Developer | `.http` AST, parser, diagnostics, shared model |
| Walt | Generator Developer | `httpgenerator-core` integration, CLI option translation, parse-back validation, dry-run/file-writing semantics, Azure token wrapper, generator goldens with provenance |
| Jesse | Runner Developer | request execution, assertions, conditions, reports, export |
| Saul | CLI / Integration Developer | command design, user-facing output, fs orchestration, exit codes |
| Hank | Tester / QA | fixtures, golden tests, integration tests, CI acceptance |
| Scribe | Session Logger | durable context, decision capture, work history |
| Ralph | Work Monitor | ongoing work visibility and continuity |

### 18.2 Handoff model

1. **Gus defines boundaries before feature code begins.**
2. **Mike establishes the canonical parser contract by integrating `httprunner-core` and documenting any adapter boundaries.**
3. **Walt integrates `httpgenerator-core` behind the `crates/generator/` integration seam: CLI option translation, parse-back validation, dry-run, file writing, Azure token wrapper, and documented gap adapters only.**
4. **Jesse builds the product runner surface on top of `httprunner-core` runtime behavior.**
5. **Saul integrates generator and runner into the CLI surface.**
6. **Hank validates each layer with fixtures and end-to-end checks and curates the generator smoke suite with provenance metadata.**
7. **Scribe records decisions that affect future work.**

### 18.3 Required work-packet format

Every non-trivial implementation task should state:

- objective,
- relevant PRD sections,
- files/crates in scope,
- constraints and non-goals,
- acceptance criteria,
- designated reviewer.

### 18.4 Reviewer gates

| Work type | Required reviewer |
|-----------|-------------------|
| Crate boundaries, dependency policy, architecture shifts | Gus |
| Parser contract changes | Mike |
| Generator behavior and output layout | Walt |
| Runner semantics and report behavior | Jesse |
| CLI UX, help text, exit code behavior | Saul |
| Tests, fixtures, CI, benchmark correctness | Hank |

### 18.5 Suggested benchmark tasks for model/agent experiments

| Benchmark | Goal | Expected artifact | Primary evaluator |
|-----------|------|------------------|-------------------|
| B1: Workspace bootstrap | Create initial Cargo workspace and crate skeleton | compilable scaffold | Gus |
| B2: Foundation baseline | Adopt `httprunner-core` and `httpgenerator-core` as canonical foundations with exact version pins; document any required adapters | passing fixture coverage + gap notes | Mike + Walt |
| B3: Generator integration smoke suite | Wire `httpgenerator-core::generate_http_files`, parse-back validate with `httprunner-core`, write files, and pin behavior with a small smoke suite carrying provenance metadata | snapshot/golden fixtures + provenance | Walt + Hank |
| B4: Runner end-to-end | Execute fixture suites with assertions and reports | integration test suite | Jesse + Hank |
| B5: CLI integration | Wire subcommands and user-facing help | usable binary UX | Saul |
| B6: Migration docs | Explain parity and differences from source tools | docs + examples | Saul + Scribe |

### 18.6 Evaluation rubric for comparing models/tools

Score each experiment on a 1–5 scale for:

1. **Correctness** — does the artifact satisfy the requirements?
2. **Completeness** — did it cover all relevant surfaces?
3. **Constraint adherence** — did it honor crate boundaries, scope, and review gates?
4. **Test quality** — were the right fixtures/tests added?
5. **Change clarity** — is the diff readable and well-structured?
6. **Rework burden** — how much cleanup was required after the first pass?
7. **Decision hygiene** — were trade-offs surfaced explicitly instead of buried in code?

---

## 19. Definition of done

The MVP is done when all of the following are true:

1. A user can generate `.http` files from a representative OpenAPI spec using the Rust CLI.
2. Those generated files can be executed by the Rust runner without manual normalization.
3. The runner supports assertions, variables, conditions, reports, and logging at the documented baseline.
4. The repository contains fixtures and tests that protect the shared parser and generated output.
5. The CLI is documented well enough for a new contributor to use without external tribal knowledge.
6. The squad docs and PRD are sufficient for a new agent/model to start work without inventing architecture from scratch.

---

## 20. Open questions and decisions to validate early

These questions should be resolved in the first implementation phase:

1. **Swagger 2 compatibility path** — confirm what `httpgenerator-core` supports today and decide whether any conversion step is needed at the wrapper layer.
2. **Unified binary only vs wrapper binaries** — when to add legacy-name compatibility.
3. **IntelliJ test block strategy** — confirm `httpgenerator-core`'s test snippet output meets the product bar; surface as a CLI toggle only.
4. **Post-MVP TUI/GUI direction** — reuse patterns from `httprunner` vs redesign later.
5. **Auth abstraction** — whether shared auth helpers belong in `crates/generator/`, `cli`, or a thin shared integration crate.
6. **Telemetry endpoint activation** — when, and under what privacy design, to wire the pluggable telemetry sink to a real destination beyond MVP.

OpenAPI parser selection is no longer an open question for this repository; it is owned by `httpgenerator-core`.

Unless superseded by explicit decisions, the defaults in this PRD should be treated as the implementation baseline.

---

## 21. Initial backlog seeds

These are suitable first issues/tasks:

1. Create Rust workspace skeleton with `generator`, `runner`, and `cli` crates and wire `httprunner-core` and `httpgenerator-core` with exact version pins.
2. Validate `httprunner-core` coverage against the PRD runner contract and record any explicit MVP gaps.
3. Validate `httpgenerator-core` coverage against the PRD generator contract and record any explicit MVP gaps.
4. Import parser and runner fixtures from `httprunner` / `httprunner-core`.
5. Build a small generator smoke suite from representative OpenAPI samples, with provenance metadata recording engine version and CLI options used.
6. Implement CLI option translation from `crates/generator/` into `httpgenerator-core` configuration.
7. Implement the Azure scope/tenant token acquisition wrapper used before invoking `httpgenerator-core` for protected OpenAPI fetches.
8. Implement dry-run and file-write semantics over `GeneratorResult` returned by `httpgenerator-core`.
9. Add parse-back validation for generated outputs via `httprunner-core` inside the generator integration crate.
10. Wire `httprunner-core` variables, assertions, conditions, timeouts, and environment selection into the unified runner surface.
11. Add markdown and HTML report generation plus export/redaction behavior on top of `httprunner-core`.
12. Design CLI subcommands, `--env`, `--no-logging`, and stable exit codes.
13. Wire support-key display and the no-endpoint pluggable telemetry sink defaults.
14. Write migration notes from `httpgenerator` and `httprunner`.
15. Establish CI matrix for Windows/macOS/Linux including a core-crate bump verification job.

---

## 22. Summary

`httpfiletools` should become the Rust-native home for both `.http` generation and execution. The repository must do more than merely hold code: it must provide a clear, stable contract for implementation, review, and AI-assisted experimentation while consuming `httprunner-core` for execution and `httpgenerator-core` for generation wherever those engines already satisfy the contract. The architecture, tests, CLI, and squad model should all reinforce the same goal: generated `.http` files that are readable, deterministic, and immediately runnable — produced by the upstream engines and integrated, validated, and shipped by this product.

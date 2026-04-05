# Product Requirements Document — httpfiletools

**Status:** Draft baseline  
**Repository:** `christianhelle/httpfiletools`  
**Date:** 2026-04-05  
**Audience:** maintainers, contributors, reviewers, and AI coding agents  
**Purpose:** define the product, architecture, scope, delivery plan, and agent operating model for a Rust-native suite of `.http` file tools

---

## 1. Executive summary

`httpfiletools` is a new Rust-native product that combines two existing tool lines into a single coherent workspace:

1. **HTTP File Runner** (`christianhelle/httprunner`) — parses and executes `.http` files, evaluates assertions, manages variables, and produces logs/reports.
2. **HTTP File Generator** (`christianhelle/httpgenerator`) — generates `.http` files from OpenAPI specifications, with options for output layout, headers, auth, and IDE-friendly test snippets.

The new product must preserve the practical value of both tools while removing the split-language maintenance burden. The resulting repository should be suitable for both human development and **repeatable experimentation with multiple models and agent workflows**, which means the product contract, architecture boundaries, and acceptance criteria must be unusually explicit.

The first release of `httpfiletools` is **CLI-first**. The priority is a high-quality Rust workspace with:

- a shared `.http` parsing and data model,
- a robust **OpenAPI-to-`.http` generator**,
- a reliable **`.http` runner**,
- deterministic outputs and golden tests,
- cross-platform support for Windows, macOS, and Linux,
- documentation and team structure that allow different agents/models to work from the same source of truth.

---

## 2. Background and source systems

### 2.1 Source system A — HTTP File Runner

`httprunner` is already written in Rust and provides a strong execution foundation. Its published feature set includes:

- parsing and executing `.http` files,
- multiple-file execution and recursive discovery,
- verbose output and pretty-printed JSON,
- report generation,
- request/response export,
- assertions,
- variables and request variables,
- built-in dynamic functions,
- conditional execution via directives such as `@dependsOn` and `@if`,
- configurable delays,
- insecure HTTPS support,
- CLI, TUI, and GUI surfaces,
- an existing Rust workspace split across `core`, `cli`, `tui`, and `gui`.

The new repository should treat `httprunner` as the primary source of truth for **runtime behavior**, **Rust project patterns**, and the current `.http` execution dialect.

### 2.2 Source system B — HTTP File Generator

`httpgenerator` is written in C# and currently provides:

- local-file and URL-based OpenAPI input,
- output directory selection,
- output layout selection (`OneRequestPerFile`, `OneFile`, `OneFilePerTag`),
- optional validation skipping,
- authorization headers,
- loading authorization headers from environment variables,
- custom header injection,
- content-type overrides,
- base URL overrides,
- Azure scope and tenant-based token acquisition,
- output timeouts,
- optional IntelliJ test block generation,
- optional omission of generated headers,
- a Visual Studio extension and CLI.

The new repository should treat `httpgenerator` as the primary source of truth for **generation behavior and CLI ergonomics**, while intentionally not inheriting its C# implementation or its telemetry pattern by default.

### 2.3 Why a new unified product

The current split creates four kinds of friction:

1. **Language fragmentation** — runner logic lives in Rust while generator logic lives in C#, which increases maintenance cost and blocks shared libraries.
2. **Behavior drift risk** — generated files and runnable files can diverge unless both tools share a single parser and dialect contract.
3. **Testing fragmentation** — parity and regression testing must currently span different stacks and tooling.
4. **Agent friction** — experiments with AI tooling are harder when architecture, contracts, and language ecosystems are inconsistent.

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
3. Building a general-purpose OpenAPI SDK/code generator.
4. Supporting non-HTTP protocols.
5. Recreating telemetry/support-key behavior from `httpgenerator`.
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

- Rust workspace with shared crates,
- unified CLI surface,
- generator subcommand,
- runner subcommand,
- shared `.http` parser/data model,
- deterministic file rendering,
- reports/logging/export,
- fixture-driven tests,
- migration guidance from the source tools,
- explicit squad/team routing documentation.

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
| Assertions | `httprunner` | **Carry forward** | Must remain a first-class runtime concept |
| Variables and request variables | `httprunner` | **Carry forward** | Required for realistic suites |
| Built-in functions | `httprunner` | **Carry forward** | Prefer compatibility where reasonable |
| Conditional execution | `httprunner` | **Carry forward** | Preserve `@dependsOn`/`@if` semantics where possible |
| Insecure HTTPS mode | `httprunner` | **Carry forward** | Useful in dev/test environments |
| TUI | `httprunner` | **Defer** | Not part of MVP |
| GUI | `httprunner` | **Defer** | Not part of MVP |
| OpenAPI from file | `httpgenerator` | **Carry forward** | Must support local specs |
| OpenAPI from URL | `httpgenerator` | **Carry forward** | Must support remote specs |
| Output directory | `httpgenerator` | **Carry forward** | Core generator requirement |
| Output types | `httpgenerator` | **Carry forward** | One request per file, one file, one file per tag |
| Skip validation | `httpgenerator` | **Carry forward** | Keep as an escape hatch |
| Authorization header injection | `httpgenerator` | **Carry forward** | Direct header input supported |
| Authorization from environment variable | `httpgenerator` | **Carry forward** | Strongly preferred over embedded secrets |
| Base URL override | `httpgenerator` | **Carry forward** | Needed when specs omit or misuse servers |
| Content-Type override | `httpgenerator` | **Carry forward** | Preserve current utility |
| Custom headers | `httpgenerator` | **Carry forward** | Useful for generated requests |
| Skip generated headers | `httpgenerator` | **Carry forward** | Preserve escape hatch |
| Azure scope / tenant auth support | `httpgenerator` | **Carry forward** | Important for Azure-hosted APIs |
| Output timeout | `httpgenerator` | **Carry forward** | Maintain operational control |
| IntelliJ test generation | `httpgenerator` | **Carry forward if low-friction** | Should be supported if it does not distort the shared parser contract |
| Telemetry and support key | `httpgenerator` | **Do not copy by default** | Revisit later only with explicit privacy design |
| Visual Studio extension | `httpgenerator` | **Defer** | Out of MVP |

---

## 11. Functional requirements

### 11.1 Repository and packaging requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| REP-001 | The repository must contain only Rust production code for the new toolchain. | Must |
| REP-002 | The project must be structured as a Rust workspace with clear crate boundaries. | Must |
| REP-003 | Shared behavior between generator and runner must live in reusable crates rather than binary-specific code. | Must |
| REP-004 | The repository must contain fixture inputs for OpenAPI specs and `.http` files. | Must |
| REP-005 | The repository must include golden-output fixtures for generation tests. | Must |
| REP-006 | The repository must include documentation sufficient for human and agent onboarding. | Must |
| REP-007 | Windows, macOS, and Linux must be supported in CI. | Must |
| REP-008 | Release packaging should target GitHub Releases and `cargo install`. | Should |

### 11.2 Generator requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| GEN-001 | The generator must accept an OpenAPI input from a local file path. | Must |
| GEN-002 | The generator must accept an OpenAPI input from a URL. | Must |
| GEN-003 | The generator must support JSON and YAML source documents. | Must |
| GEN-004 | The generator must support OpenAPI 3.x and should preserve practical compatibility with Swagger/OpenAPI 2.0 inputs used by the current tool. | Must |
| GEN-005 | The generator must support `--output` for selecting the output location. | Must |
| GEN-006 | The generator must support output layout modes equivalent to one-request-per-file, one-file, and one-file-per-tag. | Must |
| GEN-007 | File names must be deterministic and stable for identical inputs. | Must |
| GEN-008 | The generator must support `--base-url` as an override when server information is absent or unsuitable. | Must |
| GEN-009 | The generator must support a default `Content-Type` override. | Must |
| GEN-010 | The generator must support one or more custom headers added to generated requests. | Must |
| GEN-011 | The generator must optionally omit generated header parameters. | Must |
| GEN-012 | The generator must support an explicit authorization header value. | Must |
| GEN-013 | The generator must support generating files that load the authorization header from an environment variable. | Must |
| GEN-014 | The generator must support Azure scope-based token acquisition and optional tenant selection. | Should |
| GEN-015 | The generator must support skipping schema validation. | Must |
| GEN-016 | The generator must render request bodies, path variables, query variables, and header variables in a readable, editable form. | Must |
| GEN-017 | The generator must include summary and description comments when available. | Must |
| GEN-018 | The generator must support optional IntelliJ/REST Client style test snippet output. | Should |
| GEN-019 | The generator must support a preview or dry-run path that avoids partial writes when desired. | Should |
| GEN-020 | The generator must fail clearly when remote specs cannot be fetched or parsed. | Must |
| GEN-021 | The generator must not silently embed secrets into files when an environment-based pattern is requested. | Must |
| GEN-022 | Generated `.http` output must be parseable by the same project parser used by the runner. | Must |

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

### 11.4 Shared `.http` language requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| LANG-001 | There must be one canonical parser implementation used by both generator validation and runner execution. | Must |
| LANG-002 | The generator must only emit syntax that the canonical parser supports. | Must |
| LANG-003 | Line endings and file encoding must be handled in a cross-platform safe way. | Must |
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

### 12.3 `run` subcommand

Proposed baseline:

```text
httpfiletools run <paths...>
  --discover
  --verbose
  --pretty-json
  --delay <milliseconds>
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

---

## 13. Proposed architecture

### 13.1 Workspace layout

Recommended initial layout:

```text
crates/
  core/         # shared .http AST, parser, variables, common errors, redaction
  generator/    # OpenAPI loading, normalization, naming, rendering
  runner/       # request execution, assertions, reports, export
  cli/          # clap surface, output formatting, fs/network orchestration
fixtures/
  openapi/      # input specs for tests
  http/         # runnable sample .http files
  golden/       # expected generated outputs
docs/           # optional supporting docs later
.squad/         # team routing, charters, decisions
```

### 13.2 Crate responsibilities

#### `core`

Owns:

- `.http` AST and supporting types,
- parser,
- parser diagnostics,
- shared variable model,
- common errors,
- redaction utilities,
- text rendering primitives that are not generator-specific.

Does **not** own:

- HTTP execution,
- OpenAPI parsing,
- CLI formatting.

#### `generator`

Owns:

- OpenAPI source loading (file/URL),
- input validation and normalization,
- operation naming,
- request template generation,
- output layout selection,
- generator-specific settings and result types.

Does **not** own:

- canonical `.http` parsing,
- runtime request execution,
- terminal formatting decisions.

#### `runner`

Owns:

- HTTP request execution,
- assertion evaluation,
- variable capture/extraction,
- conditional execution,
- delays,
- logs, reports, and export artifacts,
- runtime result modeling.

Does **not** own:

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
OpenAPI input
  -> load/fetch
  -> validate/normalize
  -> map operations to internal generation model
  -> render canonical .http text
  -> write files
  -> optional parse-back validation using core parser
```

#### Execution flow

```text
.http input
  -> parse with canonical parser
  -> resolve variables and directives
  -> execute requests
  -> evaluate assertions
  -> collect summary + artifacts
  -> render console output + reports
```

### 13.4 Dependency strategy

Recommended baseline:

- `clap` for CLI parsing,
- `reqwest` for HTTP,
- `serde`, `serde_json`, and `serde_yaml` for data handling,
- `thiserror` in library crates,
- `anyhow` only at binary/application boundaries,
- `tokio` only where it meaningfully improves correctness or ergonomics,
- `insta` for snapshot/golden testing,
- `assert_cmd` and `predicates` for CLI tests,
- `wiremock` or `httpmock` for local HTTP test servers.

### 13.5 OpenAPI parser selection

This is an explicit sprint-0 decision point. The chosen Rust crate must be evaluated against:

1. OpenAPI 3.x support quality,
2. practical Swagger/OpenAPI 2.0 migration needs,
3. support for composed schemas and examples,
4. maintenance activity,
5. ergonomics for deterministic rendering,
6. compatibility with the project’s error-handling style.

The generator design must isolate the OpenAPI crate behind the generator layer so it can be replaced if necessary.

### 13.6 Error-handling strategy

1. Library crates return typed errors with context.
2. The CLI translates those errors into stable user-facing messages and exit codes.
3. Parse errors must include file and line context where possible.
4. Network/auth failures must not be flattened into generic “generation failed” or “run failed” messages.
5. No broad “best effort” silent fallbacks.

### 13.7 Determinism requirements

For identical inputs and options, generation should be stable across runs with respect to:

- file names,
- request ordering,
- variable ordering where controllable,
- header ordering where controllable,
- rendered section layout,
- trailing newlines and line endings policy.

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

- OpenAPI-to-`.http` rendering,
- stable CLI help output where appropriate,
- representative generated request files.

Golden tests are the key defense against subtle regressions in formatting and compatibility.

#### Integration tests

Use for:

- end-to-end generator workflows,
- runner execution against a mock server,
- report/export generation,
- multi-file discovery behavior,
- environment-variable auth flows.

#### Compatibility tests

Use for:

- source fixtures imported or derived from `httprunner`,
- source fixtures imported or derived from `httpgenerator`,
- generated-file round-tripping through the parser.

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
4. a reviewer validates that crate boundaries were respected.

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

Where behavior changes, the repo must document:

- what changed,
- why it changed,
- how to express the same intent in the new CLI.

### 15.2 Migration from `httprunner`

The new product should preserve:

- `.http` parsing semantics users rely on,
- runner directives and variable capabilities,
- reporting and logging value,
- overall feel of “run file(s), get readable results.”

TUI and GUI are not MVP requirements, but the core architecture should avoid blocking them forever.

### 15.3 Compatibility principle

Compatibility is strongest when the generator and runner share the same parser and data model. The project should prefer **shared contracts** over compatibility shims whenever possible.

---

## 16. Delivery plan and milestones

### Milestone 0 — Foundations

Deliverables:

- workspace skeleton,
- crate boundaries,
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

- canonical `.http` AST,
- parser extraction or clean-room parser equivalent,
- parser diagnostics,
- core fixture coverage.

Exit criteria:

- parser passes baseline fixtures,
- runner and generator crates can depend on the same core types.

### Milestone 2 — Generator MVP

Deliverables:

- OpenAPI loading,
- output layout modes,
- header/auth/base-url support,
- deterministic rendering,
- snapshot tests.

Exit criteria:

- sample OpenAPI fixtures generate stable `.http` outputs,
- generated outputs parse successfully with the canonical parser.

### Milestone 3 — Runner MVP

Deliverables:

- execution pipeline,
- variables and assertions,
- delays and conditions,
- logs/reports/export.

Exit criteria:

- representative suites execute successfully in integration tests,
- exit codes and reports are stable.

### Milestone 4 — Unified CLI and docs

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
| OpenAPI crate maturity in Rust | Generator parity may be blocked by crate limitations | Isolate parser choice behind generator abstractions and evaluate early |
| Divergent `.http` dialects | Generator output may not run cleanly | Use one canonical parser and add parse-back validation |
| Scope creep from runner extras | TUI/GUI and advanced UX could delay MVP | Keep MVP CLI-only and defer secondary surfaces |
| Auth complexity | Azure/token flows can grow quickly | Keep auth support explicit and narrow for MVP |
| Determinism drift | Golden tests become noisy and reviews expensive | Define rendering/stability rules early and enforce them with snapshots |
| Windows regressions | Primary user workflows may break silently | Treat Windows as first-class in docs, tests, and path handling |
| Secret leakage in logs | Security/privacy issue | Redact by default and avoid implicit telemetry |
| Agent drift | Different models may invent incompatible solutions | Use this PRD plus squad docs as the implementation contract |

---

## 18. AI/agent operating model

This repository is intentionally being set up for experiments with different models and agent tooling. That means product work must be partitionable into stable, reviewable work packets.

### 18.1 Team roles

| Agent | Role | Primary ownership |
|-------|------|-------------------|
| Gus | Tech Lead | workspace topology, architecture, dependency policy, cross-cutting decisions |
| Mike | Core / Parser Developer | `.http` AST, parser, diagnostics, shared model |
| Walt | Generator Developer | OpenAPI ingestion, generation, deterministic rendering |
| Jesse | Runner Developer | request execution, assertions, conditions, reports, export |
| Saul | CLI / Integration Developer | command design, user-facing output, fs orchestration, exit codes |
| Hank | Tester / QA | fixtures, golden tests, integration tests, CI acceptance |
| Scribe | Session Logger | durable context, decision capture, work history |
| Ralph | Work Monitor | ongoing work visibility and continuity |

### 18.2 Handoff model

1. **Gus defines boundaries before feature code begins.**
2. **Mike establishes the canonical parser contract.**
3. **Walt consumes the core contract for generation.**
4. **Jesse consumes the core contract for execution.**
5. **Saul integrates generator and runner into the CLI surface.**
6. **Hank validates each layer with fixtures and end-to-end checks.**
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
| B2: Parser baseline | Implement or extract canonical `.http` parser behavior | passing parser fixtures | Mike |
| B3: Generator golden path | Generate stable `.http` outputs from sample specs | snapshot/golden fixtures | Walt + Hank |
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

1. **OpenAPI crate choice** — which Rust crate gives the best balance of correctness and maintainability?
2. **Swagger 2 compatibility path** — native support vs conversion step.
3. **Unified binary only vs wrapper binaries** — when to add legacy-name compatibility.
4. **IntelliJ test block strategy** — generator-only templating vs broader test-snippet abstraction.
5. **Post-MVP TUI/GUI direction** — reuse patterns from `httprunner` vs redesign later.
6. **Auth abstraction** — whether shared auth helpers belong in `core` or a separate crate.

Unless superseded by explicit decisions, the defaults in this PRD should be treated as the implementation baseline.

---

## 21. Initial backlog seeds

These are suitable first issues/tasks:

1. Create Rust workspace skeleton with `core`, `generator`, `runner`, and `cli` crates.
2. Import or recreate parser fixtures from `httprunner`.
3. Define canonical `.http` AST and parser error model.
4. Evaluate Rust OpenAPI crates and record a decision.
5. Create generator snapshot fixtures from representative OpenAPI samples.
6. Implement deterministic operation naming and file naming rules.
7. Implement generator output layout modes.
8. Add parse-back validation for generated outputs.
9. Port runner assertion handling and variable substitution.
10. Port conditional execution and delay directives.
11. Add markdown and HTML report generation.
12. Design CLI subcommands and stable exit codes.
13. Write migration notes from `httpgenerator` and `httprunner`.
14. Establish CI matrix for Windows/macOS/Linux.

---

## 22. Summary

`httpfiletools` should become the Rust-native home for both `.http` generation and execution. The repository must do more than merely hold code: it must provide a clear, stable contract for implementation, review, and AI-assisted experimentation. The architecture, tests, CLI, and squad model should all reinforce the same goal: generated `.http` files that are readable, deterministic, and immediately runnable.

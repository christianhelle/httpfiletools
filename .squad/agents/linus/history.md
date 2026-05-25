# Project Context

- **Project:** httpfiletools
- **Primary user:** Christian Helle
- **Created:** 2026-05-25T22:07:44.449+02:00
- **Language:** Rust
- **Purpose:** Generate `.http` files from OpenAPI specifications and run `.http` files.
- **Upstream projects:** `christianhelle/httprunner`, `christianhelle/httpgenerator`
- **Core dependency rule:** Use `httprunner-core` and `httpgenerator-core`; do not re-implement core functionality.

## Core Context

Linus is responsible for CLI UX, subcommands, flags, packaging, and developer experience.

## Recent Updates

- Team initialized for `httpfiletools`.
- 2026-05-25T22:07:44.449+02:00: Inspected upstream `httprunner` and `httpgenerator` public READMEs, Rust CLI argument sources, and CLI tests. Prepared the initial compatibility handoff below for the next CLI implementation wave.
- 2026-05-25T23:05:30.184+02:00: Implemented the initial `httpfiletools generate` and `httpfiletools run` CLI surface in `crates/cli`, wired run execution through core wrapper APIs, and added minimal core wrappers for discovery, reports, and exports. `cargo fmt --all -- --check` and `cargo test --workspace --quiet` pass.

## Learnings

- The repository currently has Squad scaffolding but no Rust workspace files yet.
- Upstream `httpgenerator` uses `httpgenerator [URL or input file] [OPTIONS]`, prints help successfully when invoked with no args, and treats `-v`/`--version` as version flags.
- Upstream `httprunner` uses `httprunner [FILE...] [OPTIONS]`, prints help successfully when invoked with no args, and treats `-v`/`--verbose` as verbose while version is long-only `--version`.

## CLI compatibility handoff

### Overall command shape

- Root commands: `httpfiletools generate ...` and `httpfiletools run ...`.
- `httpfiletools generate` should behave like upstream `httpgenerator` with command identity/help examples rewritten to the `httpfiletools generate` subcommand.
- `httpfiletools run` should behave like upstream `httprunner` with command identity/help examples rewritten to the `httpfiletools run` subcommand.
- Both subcommands should print their subcommand help and exit 0 when invoked without required workflow input, matching the upstream no-arg user experience.

### `httpfiletools generate` mapping from `httpgenerator`

Compatibility target usage: `httpfiletools generate <URL-or-input-file> [OPTIONS]`.

| Upstream | Initial `httpfiletools` flag | Meaning / default |
| --- | --- | --- |
| positional `[URL or input file]` | positional `<OPENAPI>` | OpenAPI source path or URL. Upstream accepts local files and HTTP(S) URLs; see conflicts below. |
| `-h`, `--help` | `-h`, `--help` | Print generate help; exit 0. |
| `-v`, `--version` | `-v`, `--version` on `generate` | Print version for this binary/subcommand; do not make `-v` verbose under `generate`. |
| `-o`, `--output <OUTPUT>` | `-o`, `--output <OUTPUT>` | Output directory, default `./`. Preserve upstream directory semantics for multi-file output. |
| `--no-logging` | `--no-logging` | Do not log errors or collect telemetry; support key output becomes unavailable/hidden. |
| `--skip-validation` | `--skip-validation` | Skip OpenAPI validation before generation. Needed for upstream OpenAPI 3.1 validation gap. |
| `--authorization-header <HEADER>` | same | Literal Authorization header for generated requests. Takes precedence over Azure token acquisition. |
| `--load-authorization-header-from-environment` | same | Generate `.http` variable-based authorization instead of inlining the header. |
| `--authorization-header-variable-name <VARIABLE-NAME>` | same | Variable name for generated authorization reference; default `authorization`. |
| `--content-type <CONTENT-TYPE>` | same | Default generated Content-Type header variable; default `application/json`. |
| `--base-url <BASE-URL>` | same | Override/default base URL when the OpenAPI document lacks usable server URL. |
| `--output-type <OUTPUT-TYPE>` | same | Output layout; default `OneRequestPerFile`; accepted values are `OneRequestPerFile`, `OneFile`, `OneFilePerTag`, case-insensitive. |
| `--azure-scope <SCOPE>` | same | Azure Entra ID scope for token acquisition. |
| `--azure-tenant-id <TENANT-ID>` | same | Azure Entra ID tenant for token acquisition. If tenant is provided without scope, generation continues and prints a plain stderr warning. |
| `--timeout <SECONDS>` | same | Timeout for remote loading / file writing; default `120`. |
| `--generate-intellij-tests` | same | Append IntelliJ HTTP Client status-code tests. |
| `--custom-header <HEADER>` | same, repeatable | Add raw custom HTTP header to every generated request. |
| `--skip-headers` | same | Do not generate shared header variables/parameters in output files. |

Generate output/error behavior to preserve:

- No args to the subcommand: print help to stdout, empty stderr, exit 0.
- Successful generation prints plain redirected output including header, support key unless disabled, validation statistics when validation runs, written file paths, completion, and duration.
- Missing input during execution is an error: `missing OpenAPI input path or URL; run with --help for usage`.
- OpenAPI 3.1 validation failure should recommend retrying with `--skip-validation`.
- Missing files and validation/load/write errors should be plain text on stderr and exit non-zero.

### `httpfiletools run` mapping from `httprunner`

Compatibility target usage: `httpfiletools run [FILE...] [OPTIONS]` and `httpfiletools run --discover [OPTIONS]`.

| Upstream | Initial `httpfiletools` flag | Meaning / default |
| --- | --- | --- |
| positional `FILE...` | positional `FILE...` | Zero or more `.http` files. If none and no `--discover`, print run help and exit 0. Conflicts with `--discover`. |
| `-h`, `--help` | `-h`, `--help` | Print run help; exit 0. |
| `--version` | `--version` on `run` | Print version. Do not add `-v` for version under `run`. |
| `-v`, `--verbose` | `-v`, `--verbose` | Detailed HTTP request/response information. |
| `--log [FILENAME]` | same | Log output to file; if flag has no value, default log basename is `log`. |
| `--env <ENVIRONMENT>` | same | Load variables from `http-client.env.json` environment. |
| `--insecure` | same | Accept invalid HTTPS certificates and hostnames. |
| `--discover` | same | Recursively discover `.http` files from current directory; conflicts with positional files. |
| `--upgrade` | compatibility conflict | Upstream self-update flag; see conflicts below before implementing. |
| `--no-banner` | same | Suppress donation/support banner. |
| `--pretty-json` | same | Pretty-print JSON payloads in verbose/export output; most visible with `--verbose`. |
| `--report [FORMAT]` | same | Generate summary report; no value defaults to `markdown`; values: `markdown`, `html`. |
| `--export` | same | Export individual requests/responses to timestamped files. |
| `--export-json` | same | Export execution results as JSON. |
| `--include-secrets` | same | Disable default redaction in logs/reports/exports. |
| `--no-telemetry` | same | Disable anonymous telemetry and suppress support key output. |
| `--delay <MILLISECONDS>` | same | Delay between requests; default `0`. |

Run output/error behavior to preserve:

- No args to the subcommand: print help to stdout and exit 0.
- Successful execution should include upstream runner summaries and final `All discovered files processed successfully` line for compatible fixture flows.
- Failed request/assertion processing should exit 1 via upstream processor success status.
- `--discover` with no files should exit 0 after discovery behavior.
- Redact sensitive values in logs/reports/exports by default; only `--include-secrets` may preserve them.
- `--no-banner` suppresses donation banner; `--no-telemetry` suppresses support key.

### Known compatibility conflicts / decisions for implementation

1. `-v` is intentionally subcommand-specific: `generate -v` means version; `run -v` means verbose. Do not put a global `-v` on the root command.
2. `generate --output` must keep upstream `httpgenerator` directory semantics, especially with `--output-type OneRequestPerFile` and `OneFilePerTag`. Treat any file-output or `--stdout` convenience as an additional compatibility layer only when unambiguous, preferably limited to `--output-type OneFile`.
3. Upstream `httpgenerator` accepts remote OpenAPI URLs, while an earlier project inbox item says initial `generate` requires a file path. For CLI compatibility, the implementation wave should either accept URLs immediately or explicitly document the temporary incompatibility.
4. Upstream `httprunner --upgrade` is a self-update mechanism for the standalone runner. In `httpfiletools`, packaging is unified, so `run --upgrade` should be deferred or mapped to a root-level package update decision rather than blindly updating `httprunner`.
5. Help text is API: examples should use `httpfiletools generate` / `httpfiletools run`, but option names, defaults, repeatability, and exit-code behavior should match upstream contracts.

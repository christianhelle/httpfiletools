# httpfiletools

`httpfiletools` is a Rust tool suite for working with `.http` files. It combines the core ideas from `httprunner` and `httpgenerator`: generate `.http` request files from OpenAPI documents, then run those files from one CLI.

## Upstream compatibility

This project reuses the upstream engines directly:

- `httpgenerator-core` powers OpenAPI loading, normalization, and `.http` generation.
- `httprunner-core` powers `.http` parsing, execution, reports, and exports.

`httpfiletools` is an integration layer around those crates, not a reimplementation. Generated output and runner behavior should remain compatible with the upstream tools unless documented otherwise.

## Workspace shape

- `src/cli` contains the public `httpfiletools` binary.
- `src/core` contains the integration library used by the CLI.

## Commands

Generate `.http` files from a local OpenAPI file:

```powershell
httpfiletools generate .\openapi.yaml
```

Generate from an HTTP(S) OpenAPI URL:

```powershell
httpfiletools generate https://example.test/openapi.json --output .\requests
```

Run an `.http` file:

```powershell
httpfiletools run .\requests\Requests.http
```

Discover and run `.http` files:

```powershell
httpfiletools run --discover
```

## Generator behavior

`httpfiletools generate <openapi>` accepts local files and HTTP(S) URLs supported by `httpgenerator-core`. Standard input is not supported initially; pass a file path or URL.

Generated content is produced by `httpgenerator-core`, so filenames and request formatting are intended to match upstream output. `--output <dir>` is always treated as an output directory. The CLI creates the directory if needed and writes every generated file into it.

Selected generator flags:

- `--output <dir>` / `-o <dir>`: output directory, default `./`.
- `--output-type <OneRequestPerFile|OneFile|OneFilePerTag>`: upstream generation layout.
- `--base-url <url>`: override the generated base URL.
- `--content-type <value>`: generated request content type, default `application/json`.
- `--authorization-header <value>`: include an authorization header.
- `--load-authorization-header-from-environment`: load authorization from an environment variable.
- `--authorization-header-variable-name <name>`: variable name for authorization, default `authorization`.
- `--custom-header <header>`: add a custom header; may be repeated.
- `--skip-headers`: omit generated headers.
- `--skip-validation`: tolerate selected OpenAPI 3.1 validation issues.
- `--timeout <seconds>`: upstream document load timeout, default `120`.
- `--generate-intellij-tests`: generate IntelliJ-compatible tests where upstream supports it.
- `--no-logging`: suppress generator banner logging.

## Runner behavior

`httpfiletools run <http-file>` delegates parsing and execution to `httprunner-core`. Selected flags and outputs are kept compatible with `httprunner` where implemented.

Selected runner flags:

- `--verbose` / `-v`: enable verbose request execution.
- `--env <environment>`: select an environment.
- `--insecure`: allow insecure TLS.
- `--discover`: discover `.http` files instead of passing explicit files.
- `--no-banner`: run silently where upstream supports banner suppression.
- `--pretty-json`: pretty-print JSON in supported runner exports/output.
- `--log [filename]`: write runner logs using the upstream log behavior.
- `--report [markdown|html]`: generate a report; default format is `markdown`.
- `--export`: export per-request results.
- `--export-json`: export aggregate JSON results.
- `--delay <milliseconds>`: delay between requests.

`--include-secrets` and `--no-telemetry` are accepted for compatibility but do not currently change behavior.

## Development

Install Rust 1.95 or newer. Common validation commands:

```powershell
cargo fmt --all -- --check
cargo test --workspace --quiet
```

## Releasing to crates.io

Publish the workspace crates in dependency order:

```powershell
cargo publish -p httpfiletools-core
cargo publish -p httpfiletools
```

`httpfiletools` depends on `httpfiletools-core`, so the core crate must be available on crates.io before publishing the CLI crate.

## Compatibility tests

The default test suite is deterministic and should not require external services. It includes fixture-based generator compatibility checks and a local in-process HTTP server for runner behavior.

Some compatibility tests may be ignored for manual use, such as golden CLI stdout/stderr/exit-code checks while the CLI surface is still being locked down. Run ignored tests explicitly when updating compatibility fixtures:

```powershell
cargo test --workspace -- --ignored
```

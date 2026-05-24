# httpfiletools

`httpfiletools` is a unified Rust CLI for generating `.http` files from OpenAPI documents and executing `.http` suites with the canonical `httprunner-core` runtime.

## Status

This repository implements the PRD baseline with a workspace split into thin `generator`, `runner`, and `cli` crates.

## Install

```bash
cargo install --path crates/cli
```

## Quick start

Generate `.http` files from an OpenAPI fixture:

```bash
httpfiletools generate fixtures/openapi/petstore.yaml --output generated --output-type one-file
```

Run a suite recursively:

```bash
httpfiletools run fixtures/http --discover --verbose --report html --export artifacts
```

## Commands

- `httpfiletools generate <spec>`
- `httpfiletools run <paths...>`
- `httpfiletools version`
- `httpfiletools help`

## Repository layout

```text
crates/
  cli/
  generator/
  runner/
fixtures/
  openapi/
  http/
  golden/
```

## Notes

- `httpgenerator-core` is exact-pinned at `1.1.0`.
- `httprunner-core` is exact-pinned at `0.9.51`.
- Generated output is parse-back validated with `httprunner-core` before it is written.
- `--no-logging` suppresses the default support-key banner.

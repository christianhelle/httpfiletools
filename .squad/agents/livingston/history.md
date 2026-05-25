# Project Context

- **Project:** httpfiletools
- **Primary user:** Christian Helle
- **Created:** 2026-05-25T22:07:44.449+02:00
- **Language:** Rust
- **Purpose:** Generate `.http` files from OpenAPI specifications and run `.http` files.
- **Upstream projects:** `christianhelle/httprunner`, `christianhelle/httpgenerator`
- **Core dependency rule:** Use `httprunner-core` and `httpgenerator-core`; do not re-implement core functionality.

## Core Context

Livingston is responsible for OpenAPI behavior, `.http` syntax, and interoperability.

## Recent Updates

- Team initialized for `httpfiletools`.

## Learnings

- The repository currently has Squad scaffolding but no Rust workspace files yet.

## 2026-05-25T23:05:30.184+02:00 — Core API semantics advisory

- Inspected `crates/core` scaffold against `httpgenerator-core 1.1.0` and `httprunner-core 0.9.51` public APIs and local registry source.
- Generator wrapper should load with `httpgenerator_core::openapi::load_and_normalize_document` (or `_with_options` only if exposing OpenAPI 3.1 tolerance) and render with `httpgenerator_core::generate_http_files`; do not duplicate base URL, naming, headers, sample body, or output grouping logic.
- Runner file-level behavior should prefer `httprunner_core::processor::{ProcessorConfig, process_http_files_with_config/process_http_files_with_silent}` for full `.http` semantics; direct `parser::parse_http_file` plus `runner::execute_http_request` is only a low-level/single-request seam and bypasses dependency, condition, request-variable, function, delay, logging, and aggregate result orchestration.
- `parse_http_content(content, environment_name)` currently ignores its environment argument upstream, so in-memory content cannot load `http-client.env.json` through that API.
- Validation: `cargo test --workspace --quiet` passed with 0 tests.

## 2026-05-25T23:05:30.184+02:00 — Generate flag subset verification

- Verified upstream `httpgenerator 1.1.0` CLI flags against `httpgenerator-core 1.1.0` and current `crates/core::GenerateOptions`.
- Stable generate subset now: `<OPENAPI>`, help/version, `--output`, `--skip-validation`, auth header/env variable controls, content/base URL controls, output type, timeout, IntelliJ tests, repeatable custom headers, and skip headers.
- Deferred from stable subset: Azure acquisition flags, `--no-logging`, and non-upstream `--stdout` until explicit CLI-layer behavior is chosen.
- Handoff written to `.squad/decisions/inbox/livingston-generator-flags.md`.

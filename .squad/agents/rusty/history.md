# Project Context

- **Project:** httpfiletools
- **Primary user:** Christian Helle
- **Created:** 2026-05-25T22:07:44.449+02:00
- **Language:** Rust
- **Purpose:** Generate `.http` files from OpenAPI specifications and run `.http` files.
- **Upstream projects:** `christianhelle/httprunner`, `christianhelle/httpgenerator`
- **Core dependency rule:** Use `httprunner-core` and `httpgenerator-core`; do not re-implement core functionality.

## Core Context

Rusty is responsible for Rust core integration and implementation seams.

## Recent Updates

- Team initialized for `httpfiletools`.

## Learnings

- The repository currently has Squad scaffolding but no Rust workspace files yet.

## 2026-05-25T22:07:44.449+02:00 — Rust workspace foundation

- Created a Cargo workspace with `crates/core` as private `httpfiletools-core` and `crates/cli` as the user-facing `httpfiletools` binary.
- Inspected `httpgenerator-core 1.1.0`; generation should flow through `openapi::load_and_normalize_document` plus `generate_http_files` rather than local OpenAPI generation logic.
- Inspected `httprunner-core 0.9.51`; parsing and single-request execution are exposed through `parser::parse_http_file`, `parser::parse_http_content`, and `runner::execute_http_request`.
- `httprunner-core 0.9.51` currently needs default features enabled for this workspace build; disabling telemetry caused upstream compile errors in `telemetry::tracking`.
- Validation passed with `cargo test --workspace --quiet` on Rust 1.95.0.

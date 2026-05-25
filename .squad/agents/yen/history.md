# Project Context

- **Project:** httpfiletools
- **Primary user:** Christian Helle
- **Created:** 2026-05-25T22:07:44.449+02:00
- **Language:** Rust
- **Purpose:** Generate `.http` files from OpenAPI specifications and run `.http` files.
- **Upstream projects:** `christianhelle/httprunner`, `christianhelle/httpgenerator`
- **Core dependency rule:** Use `httprunner-core` and `httpgenerator-core`; do not re-implement core functionality.

## Core Context

Yen is responsible for test strategy, fixtures, integration tests, CLI behavior, and edge cases.

## Recent Updates

- Team initialized for `httpfiletools`.
- 2026-05-25T22:07:44.449+02:00: Prepared the initial compatibility test plan and fixture inventory for golden CLI tests, deterministic local HTTP integration tests, and selected upstream runner/generator compatibility checks. Captured the handoff in `.squad/decisions/inbox/yen-test-strategy.md`.

## Learnings

- The repository currently has Squad scaffolding but no Rust workspace files yet.
- Initial test scaffolding should avoid Rust source edits and start with fixture-driven integration tests once `crates/cli` and `crates/core` exist.

- 2026-05-25T23:05:30.184+02:00: Added default-passing compatibility test scaffolding for `add-compat-tests`: core golden generation fixture comparison, invalid OpenAPI error wrapper check, deterministic local HTTP server execution, and ignored CLI stdout/stderr/exit-code golden harness pending final CLI contract.

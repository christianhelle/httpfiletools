# Project Context

- **Project:** httpfiletools
- **Created:** 2026-05-25
- **Primary user:** Christian Helle
- **Language:** Rust
- **Purpose:** Generate `.http` files from OpenAPI specifications and run `.http` files.
- **Core dependency rule:** Use `httprunner-core` and `httpgenerator-core`; do not re-implement core functionality.

## Core Context

Scribe maintains decisions, orchestration logs, session logs, and cross-agent context.

## Recent Updates

📌 Team initialized on 2026-05-25

## Learnings

Initial setup complete.

The repository currently has Squad scaffolding but no Rust workspace files yet.

## 2026-05-25T23:05:30.184+02:00 — Setup and Wave 1 logging

- Merged setup, compatibility, workspace, and test-strategy decision inbox entries into `.squad/decisions.md`.
- Logged setup/Wave 1 orchestration state under `.squad/log/` and `.squad/orchestration-log/`.
- Rust workspace scaffold now exists from Rusty's Wave 1 commit `f6ef315`; earlier “no Rust workspace files yet” notes are historical only.

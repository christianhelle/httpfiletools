# Squad Team

> httpfiletools

## Coordinator

| Name | Role | Notes |
|------|------|-------|
| Squad | Coordinator | Routes work, enforces handoffs and reviewer gates. |

## Members

| Name | Role | Charter | Status |
|------|------|---------|--------|
| Danny | Lead / Architect | `.squad/agents/danny/charter.md` | Active |
| Rusty | Rust Core Dev | `.squad/agents/rusty/charter.md` | Active |
| Linus | CLI Dev | `.squad/agents/linus/charter.md` | Active |
| Livingston | OpenAPI & HTTP Domain Specialist | `.squad/agents/livingston/charter.md` | Active |
| Yen | Tester / QA | `.squad/agents/yen/charter.md` | Active |
| Scribe | Session Logger | `.squad/agents/scribe/charter.md` | Active |
| Ralph | Work Monitor | `.squad/agents/ralph/charter.md` | Active |

## Project Context

- **Project:** httpfiletools
- **Created:** 2026-05-25
- **Primary user:** Christian Helle
- **Language:** Rust
- **Purpose:** Suite of `.http` file tools for generating `.http` files from OpenAPI specifications and running `.http` files.
- **Upstream projects:** Merge concepts from `christianhelle/httprunner` and `christianhelle/httpgenerator`.
- **Core dependency rule:** Use the existing crates `httprunner-core` and `httpgenerator-core`; do not re-implement their core functionality.

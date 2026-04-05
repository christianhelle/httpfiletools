# Squad Team

> Rust-native `.http` generation and execution suite with a shared parser, generator, runner, and CLI.

## Coordinator

| Name | Role | Notes |
|------|------|-------|
| Squad | Coordinator | Routes work, enforces handoffs and reviewer gates. Does not generate domain artifacts directly. |

## Members

| Name | Role | Charter | Status |
|------|------|---------|--------|
| Gus | Tech Lead | `.squad/agents/gus/charter.md` | ✅ Active |
| Mike | Core / Parser Developer | `.squad/agents/mike/charter.md` | ✅ Active |
| Walt | Generator Developer | `.squad/agents/walt/charter.md` | ✅ Active |
| Jesse | Runner Developer | `.squad/agents/jesse/charter.md` | ✅ Active |
| Saul | CLI / Integration Developer | `.squad/agents/saul/charter.md` | ✅ Active |
| Hank | Tester / QA | `.squad/agents/hank/charter.md` | ✅ Active |
| Scribe | Session Logger | `.squad/agents/scribe/charter.md` | 📋 Silent |
| Ralph | Work Monitor | `.squad/agents/ralph/charter.md` | 🔄 Monitor |

## Coding Agent

<!-- copilot-auto-assign: false -->

| Name | Role | Charter | Status |
|------|------|---------|--------|
| @copilot | Coding Agent | — | 🤖 Coding Agent |

### Capabilities

**🟢 Good fit — auto-route when enabled:**

- Small, well-scoped features with clear acceptance criteria
- Parser, generator, runner, or CLI work that stays inside an approved crate boundary
- Documentation, examples, and migration notes
- Test fixture generation and straightforward CI wiring

**🟡 Needs review — route to @copilot but require squad review:**

- Medium features that touch multiple crates
- Refactors driven by approved architecture decisions
- Compatibility work for source-tool migration

**🔴 Not suitable — route to a named squad member instead:**

- Architecture or dependency-policy decisions
- Ambiguous scope or product trade-offs
- Security-sensitive auth design changes
- Changes that alter the canonical `.http` dialect contract

## Project Context

- **Owner:** Christian Helle
- **Stack:** Rust, Cargo, Clap, Reqwest, Markdown, fixture-driven testing, Squad orchestration
- **Description:** Unified Rust toolchain for generating `.http` files from OpenAPI and running `.http` files locally or in CI
- **Created:** 2026-04-05

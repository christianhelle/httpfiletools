# Squad Decisions

## Active Decisions

1. **`PRD.md` is the current source of truth for product scope and implementation direction.**
2. **This repository will remain Rust-only for production code.** Existing C# behavior may inform parity, but new production code belongs in Rust.
3. **MVP is CLI-first.** Generator and runner functionality ship before any TUI, GUI, or extension work.
4. **Generator and runner must share one canonical `.http` parser/model.** Generated output is required to remain valid runner input.
5. **Telemetry is out of MVP scope unless explicitly reintroduced by a later decision.**

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

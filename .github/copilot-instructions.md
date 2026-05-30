# Copilot instructions

## Git commit policy

For all agentic work, commit automatically without being asked:

- Commit changes in small, logical groups (e.g. dependency bumps, core changes,
  CLI changes, docs, and tooling/config each as their own commit).
- Commit as often as possible to produce a detailed progress history. Prefer many
  small commits over a few large ones.
- Do **not** add a `Co-authored-by` trailer or any other co-author attribution to
  commits.
- Write clear, conventional commit messages describing the change.

## Validation

Validate the workspace before committing functional changes:

- `cargo fmt --all -- --check`
- `cargo test --workspace`

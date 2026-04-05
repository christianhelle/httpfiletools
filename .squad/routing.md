# Work Routing

How to decide who handles what for `httpfiletools`.

## Routing Table

| Work Type | Route To | Examples |
|-----------|----------|----------|
| Architecture and crate boundaries | Gus | Workspace layout, dependency policy, ADRs, cross-cutting contracts |
| `.http` parser and shared model | Mike | AST types, parser behavior, diagnostics, variable model |
| OpenAPI ingestion and file generation | Walt | Spec loading, rendering, naming, output layout, generator parity |
| Runtime execution and reports | Jesse | HTTP execution, assertions, conditions, delays, exports, reporting |
| CLI UX and integration | Saul | Clap surface, help text, output formatting, exit codes, file orchestration |
| Testing and CI | Hank | Fixtures, snapshots, integration tests, release gates, benchmark quality |
| Code review | Hank | Review correctness, edge cases, fixture coverage, regression risk |
| Scope and priorities | Gus | Milestone planning, trade-offs, acceptance boundaries |
| Session logging | Scribe | Automatic — never needs routing |
| Continuity and work monitoring | Ralph | Watch backlog drift, surface missing follow-ups, maintain continuity |

## Issue Routing

| Label | Action | Who |
|-------|--------|-----|
| `squad` | Triage: analyze issue, assign `squad:{member}` label | Squad |
| `squad:{name}` | Pick up issue and complete the work | Named member |

### How Issue Assignment Works

1. When a GitHub issue gets the `squad` label, the **Coordinator** triages it — analyzing content, assigning the right `squad:{member}` label, and commenting with triage notes.
2. When a `squad:{member}` label is applied, that member picks up the issue in their next session.
3. Members can reassign by removing their label and adding another member's label.
4. The `squad` label is the "inbox" — untriaged issues waiting for Coordinator review.

## Rules

1. **Eager by default** — spawn all agents who could usefully start work, including anticipatory downstream work.
2. **Scribe always runs** after substantial work, always as `mode: "background"`. Never blocks.
3. **Quick facts → coordinator answers directly.** Don't spawn an agent for "what port does the server run on?"
4. **When two agents could handle it**, pick the one whose domain is the primary concern.
5. **"Team, ..." → fan-out.** Spawn all relevant agents in parallel as `mode: "background"`.
6. **Anticipate downstream work.** If a feature is being built, spawn the tester to write test cases from requirements simultaneously.
7. **Issue-labeled work** — when a `squad:{member}` label is applied to an issue, route to that member. The Coordinator handles all `squad` (base label) triage.
8. **`PRD.md` is the baseline product contract.** If requirements are unclear, read the PRD before inventing scope.
9. **The canonical `.http` dialect belongs to Mike's core contract.** Generator and runner changes must not fork the language silently.
10. **Every non-trivial change needs a reviewer gate.** Architecture goes to Gus, behavior and tests go to Hank, and domain-specific reviewers should be pulled in as needed.

# Keep httprunner-core default features enabled

Date: 2026-05-25T22:07:44.449+02:00
Owner: Rusty

## Decision

Use `httprunner-core = "0.9.51"` with its default features enabled in the initial workspace foundation.

## Rationale

I first tried `default-features = false` to keep the foundation lean, but `httprunner-core 0.9.51` failed to compile because `telemetry::tracking` references helper functions gated behind the telemetry feature. Keeping defaults preserves compatibility with the published crate and keeps the workspace buildable without reimplementing runner behavior.

## Implication

Future CLI telemetry policy should be handled at the application layer or after an upstream crate fix, not by disabling `httprunner-core` defaults in this workspace.

# End-to-End Provenance Demo v1

Issue: **#1504**

This demo is a deterministic, fixture-scoped walkthrough of the End-to-End
Provenance v1 contract. It shows one complete provenance bundle for a
playable-demo-v2 collect-and-exit change, one replay that reproduces the
expected verdict, and one replay that intentionally diverges so the diff path is
visible.

## Fixture Layout

- `examples/end-to-end-provenance-v1/demo/bundle.complete.fixture.json` records
  a complete eight-link provenance chain and replay inputs.
- `examples/end-to-end-provenance-v1/demo/bundle.diverged.fixture.json` keeps
  the same evidence chain but points at a mismatched expected verdict.
- `examples/end-to-end-provenance-v1/demo/refs/` contains the linked intent,
  generated artifact, validation, runtime, regression, review, rollback, and
  deterministic metadata evidence.
- `examples/end-to-end-provenance-v1/demo/runs/replay-pass/` is the tracked
  fixture run snapshot used by local replay.
- `examples/end-to-end-provenance-v1/demo/expected/` contains the reproduced and
  diverged expected verdict fixtures.

## Deterministic Replay

The Rust smoke test reads the fixture JSON, validates the complete bundle with
`ProvenanceBundleArtifact::evaluate_with_root`, and replays both bundles through
`replay_provenance_bundle`, which reuses `evaluate_run`. The replay workspace is
created under the system temp directory during the test and is not a tracked
artifact.

The demo does not require network access, a live browser, or command execution
from a dashboard. Evidence is read from tracked fixtures only.

## Boundaries

- Rust/local replay owns trusted verification decisions.
- Dashboard and Studio surfaces remain read-only inspection surfaces.
- The demo demonstrates audit and replay evidence only; it does not promote,
  auto-approve, merge, apply source changes, or implement #1505 dashboard UI or
  #1506 governance.
- Generated state remains untracked unless explicitly fixture-scoped.
- Existing provenance bundle and replay contracts remain backward-compatible.
- This is a fixture-scoped audit/replay demonstration only, with no broad
  readiness or engine-replacement claim.
- **#1 remains open.**
- **#23 remains open.**

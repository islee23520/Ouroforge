# Narrative Assist Demo v1

This deterministic demo shows the Narrative and Theme-Arc Authoring Assist v1
proposal path from candidate generation to a human-selected integration
provenance record. It is fixture-scoped under
`examples/narrative-assist-v1/demo/` and reuses the existing local Rust
contracts from #1864 and #1865.

The demo has no network dependency and no live browser dependency. Browser,
Studio, dashboard, or cockpit surfaces remain read-only if they display this
evidence. The selected candidate is still not trusted source; it is only ready
for the existing review/apply/trust-gradient path.

## Fixtures

- `examples/narrative-assist-v1/demo/manifest-v1.json` — stable demo manifest,
  fixed timestamps, selected candidate id, and read-only boundary flags.
- `examples/narrative-assist-v1/demo/candidate-set-v1.json` — deterministic
  candidate set regenerated from
  `examples/narrative-assist-v1/narrative-candidate-brief-v1.json`.
- `examples/narrative-assist-v1/demo/selection-v1.json` — human selection
  provenance for `theme-arc-harbor-repair-v1`.
- `examples/narrative-assist-v1/demo/integration-provenance-v1.json` — recorded
  integration evidence with `ready-for-review-apply` status.

## Local reproduction

From a fresh clone, run:

```bash
cargo test -p ouroforge-core --test narrative_assist_demo_contract --jobs 2
```

The smoke test regenerates candidates from the source brief at fixed time
`1786400000000`, records the human selection at fixed time `1786500001000`, and
compares those computed records with the fixture JSON files. It also validates
that the manifest declares no network, no live browser, proposal-only behavior,
review/apply requirement, and no trusted write authority.

## Boundaries

This demo is intentionally conservative: it does not assert fun, tone quality,
production readiness, shippability, market fit, or Godot replacement/parity. It
only demonstrates deterministic proposal records and provenance for a human
selection. Tone/soul/fun/quality remain human judgments; generated material is
proposal-only until a separate trusted review/apply path accepts it.

Generated runs, browser state, screenshots, builds, and other local artifacts
must remain untracked unless a future issue explicitly scopes them as fixtures.
#1 and #23 remain open governance anchors.

# Content-at-Scale Generation and Curation Demo v1

Content-at-Scale Generation and Curation Demo v1 (#1653) is a deterministic,
fixture-scoped demo of the Milestone 38 pipeline for one genre (grid-puzzle). It
composes the four implementation slices end to end and shows the curation gate
rejecting weak content while admitting a curve-verified set.

It reuses existing surfaces only; it adds no new engine, runtime, or writer, and
it reproduces deterministically with no network or live browser.

## The flow

For each demo scenario the smoke test
(`crates/ouroforge-core/tests/content_scale_demo_contract.rs`) runs:

1. **Generation** (#1649) — `generate_campaign` turns a campaign brief
   (`examples/content-scale-v1/demo/*-campaign.json`) into a set of grid-puzzle
   proposals through the front door.
2. **Solvability** (Milestone 28) — each generated proposal is run through the
   engine-room solver (`puzzle_solver::solve`); the demo counts how many are
   solvable rather than asserting it.
3. **Novelty** (#1650) — `compute_novelty` measures the set; a repetitive set is
   flagged low-novelty.
4. **Curve** (#1651) — `verify_curve` checks the declared whole-game difficulty
   curve (`*-curve.json`) for spikes/regressions.
5. **Curation** (#1652) — the composed facts (solvable count, balance, novelty,
   curve) are fed to the evaluator curation gate, which admits or rejects the
   campaign.

## Scenarios

| Scenario | Generation | Solvable | Novelty | Curve | Curation verdict |
| --- | --- | --- | --- | --- | --- |
| Admitted | 3 distinct levels | 3/3 | not low | verified | **Pass** |
| Repetitive | 3 identical levels | 3/3 | low-novelty | verified | **rejected — LowNovelty** |
| Curve spike | (curve input) | — | not low | spike + regression | **rejected — CurveViolation** |

The admitted campaign is generated, all three levels are solvable, the set is
novel, and the whole-game difficulty **curve** is verified, so **curation**
admits it. The repetitive campaign is curated out for low novelty; the
curve-spike campaign is curated out for a curve violation. Balance for the
grid-puzzle demo is represented by the declared difficulty evidence (`balanced`
is declared true in these fixtures); the distinguishing signals are novelty and
the curve.

## Contract and boundary

- Deterministic and fixture-scoped: a fixed `now_unix_ms`, fixtures under
  `examples/content-scale-v1/demo/`, no network or live browser.
- Asserts **gate states and behavior**, never subjective quality. No auto-merge,
  quality, fun, production-ready, shippable, or Godot-replacement claim.
- Proposal-only and read-only: generation emits proposals; the demo performs no
  trusted write, auto-apply, or promotion. Rust/local owns the logic;
  browser/Studio surfaces are unchanged and read-only. No new language; Elixir
  remains NO-GO per ADR #92.

## Generated-state policy

No generated runs/assets/content/release artifacts are committed. The only added
data files are the four tiny deterministic fixtures under
`examples/content-scale-v1/demo/`.

## #1 / #23 governance

#1 remains open as the roadmap/vision anchor and #23 as the repo-memory/design
context anchor. This demo does not close, narrow, or modify either; any such
change requires a separate explicit governance decision.

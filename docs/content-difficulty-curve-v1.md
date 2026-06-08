# Whole-Game Difficulty-Curve Verification v1

Whole-Game Difficulty-Curve Verification v1 (#1651) is the third implementation
slice of Content-at-Scale Generation and Curation v1 (`docs/content-scale-v1.md`,
#1648) under #1 Era G Milestone 38. It raises difficulty verification from
per-level to **whole-game**: it authors a difficulty curve across an ordered
campaign and verifies its shape against declared tolerances.

This slice reuses existing measurements; it is **not** a new engine.

## What this slice adds

`content_difficulty_curve.rs` reads a curve-input document
(`ouroforge.difficulty-curve-input.v1`) — an ordered list of stages, each
carrying the evidence for one campaign step — and produces a `CurveReport`:

- each stage's scalar **difficulty is derived from existing evidence**:
  - `m28-difficulty` — the Milestone 28 difficulty metric
    (`puzzle_difficulty_metric::DifficultyMetric`), using the solution-length
    signal;
  - `m32-balance` — the Milestone 32 balance report
    (`ouroforge.balance-report.v1`), using `avg_turns + (1 - win_rate) * 10` so
    that stages the synthetic players win less and take longer to clear measure
    as harder;
- the realized curve is checked between consecutive stages: an increase larger
  than the declared **spike** tolerance is flagged as a `spike`; a decrease
  larger than the declared **regression** tolerance is flagged as a
  `regression`. A curve with no findings is monotonic-enough and `passed`.

## Contract

- **Computed from existing measurements, not asserted.** Difficulty comes from
  the M28 metric and the M32 balance report; the curve report can be re-derived
  and audited. No simulation, solver, or balance engine is introduced.
- **Descriptive only.** "Spike," "regression," and "passed" are measurements
  against declared, evidence-backed tolerances — not a fun, quality, or balance
  guarantee.
- **Fail closed.** A malformed input, an unsupported source, or missing/malformed
  per-stage evidence (e.g. a balance report missing its win rate) is an error.

## Language and Studio boundary

Rust/local owns the verification logic. No JS/Studio changes — browser/Studio
surfaces remain read-only. No new language or runtime is introduced;
distributed/Elixir remains NO-GO per ADR #92
(`docs/distributed-elixir-design.md`).

## Generated-state policy

No generated runs/assets/content/release artifacts are committed. The only added
data files are the tiny deterministic fixtures under
`examples/generative-front-door/` (`content-difficulty-curve-pass-v1.json`,
`content-difficulty-curve-spike-v1.json`,
`content-difficulty-curve-missing-evidence-v1.json`) consumed by the contract
test `crates/ouroforge-core/tests/content_difficulty_curve_contract.rs`.

## Wording

This slice makes no auto-merge, quality, fun, production-ready, shippable, or
Godot-replacement claim.

## #1 / #23 governance

#1 remains open as the roadmap/vision anchor and #23 as the repo-memory/design
context anchor. This slice does not close, narrow, or modify either; any such
change requires a separate explicit governance decision.

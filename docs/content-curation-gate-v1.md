# Content Curation Gate v1

Content Curation Gate v1 (#1652) is the fourth implementation slice of
Content-at-Scale Generation and Curation v1 (`docs/content-scale-v1.md`, #1648)
under #1 Era G Milestone 38. It is the **campaign-level promotion guard**: the
campaign-level analogue of the per-level engine-room guard.

This slice **composes with** the existing evaluator gates; it is **not** a new
evaluator.

## What this slice adds

`content_curation_gate.rs` (in `ouroforge-evaluator`) consumes a curation check
(`ouroforge.content-curation-gate.v1`) — the declared summary of a campaign's
existing solver/balance/novelty/curve evidence — and emits a verdict:

- **admit (`Pass`)** only when every level is solvable, the campaign is balanced,
  it clears the novelty threshold, and the whole-game difficulty curve is
  verified;
- otherwise it rejects with the first failing reason: `Unsolvable`, `Imbalanced`,
  `LowNovelty`, or `CurveViolation`;
- malformed evidence (wrong schema, empty campaign, a **missing evidence
  dimension** — solver, balance, novelty, or curve — zero levels, or
  `levelsSolvable > levelsTotal`) is `MalformedEvidence` — fail closed. A
  campaign cannot be admitted without all four evidence dimensions declared, even
  when every pass flag is set.

The gate composes into the existing gate-categories object as one more declared
`contentCuration` category ANDed under the `declared-gate-and` aggregation
(`undeclaredGatePolicy: neutral`). An undeclared curation gate contributes no
category and stays neutral.

## Contract

- **Composes, does not replace.** The curation gate is one additional declared
  category in the existing aggregation — no parallel evaluator, no new aggregator.
- **Reuse.** Solvability, balance, novelty, and curve are produced by the
  existing surfaces (engine-room solver — Milestone 28; balance telemetry —
  Milestone 32; `content_novelty` — #1650; `content_difficulty_curve` — #1651).
  Because `ouroforge-core` depends on this evaluator crate, the gate consumes
  those producers' declared result evidence rather than importing them — exactly
  as the visual gate consumes a precomputed `compare` artifact.
- **No uncurated content promoted.** A campaign that fails any dimension is not
  admitted; the gate fails closed and performs no trusted write, auto-apply, or
  promotion.
- **Descriptive only.** "Balanced," "novel," and "curated" are measurements
  against declared, evidence-backed thresholds — never a fun, quality, or taste
  claim.

## Language and Studio boundary

Rust/local owns the gate logic. No JS/Studio changes — browser/Studio surfaces
remain read-only. No new language or runtime is introduced; distributed/Elixir
remains NO-GO per ADR #92 (`docs/distributed-elixir-design.md`).

## Generated-state policy

No generated runs/assets/content/release artifacts are committed. The only added
data files are the tiny deterministic fixtures under `examples/content-curation/`
consumed by the contract test
`crates/ouroforge-core/tests/content_curation_gate_contract.rs`.

## Wording

This slice makes no auto-merge, quality, fun, production-ready, shippable, or
Godot-replacement claim.

## #1 / #23 governance

#1 remains open as the roadmap/vision anchor and #23 as the repo-memory/design
context anchor. This slice does not close, narrow, or modify either; any such
change requires a separate explicit governance decision.

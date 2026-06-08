# Scenario Coverage v36 — Content-at-Scale Regression Suite

Issue: **#1654** (Era G Milestone 38). Part of Content-at-Scale Generation and
Curation v1 (#1648). Scenario Coverage v36 locks the behavior of campaign-scale
generation (#1649), the dedup/novelty metrics (#1650), the whole-game
difficulty-curve verifier (#1651), and the curation gate (#1652), and guards the
backward compatibility of single-level Milestone 30 generation (#1593).

Scenario Coverage v36 is an enumerated, fixture-scoped regression suite. It
asserts **states and shapes only** — no flaky or timing-based assertions — so a
breaking change to the content-at-scale pipeline fails CI. Coverage numbering
continues from v35 onward (v37/v38 are owned by other milestones).

## What is covered

The matrix
`examples/content-scale-v1/scenario-coverage-v36/matrix.fixture.json` enumerates
every case; the runner
`crates/ouroforge-core/tests/scenario_coverage_v36_content_scale.rs` executes
them against the real merged surfaces (generation, novelty, curve, and the
evaluator curation gate). Every fixture is reused from the slice it locks.

| Area | Case | Expected |
| --- | --- | --- |
| Backward compatibility (#1593) | `singleLevelM30Valid` | single-level intake remains valid (proposed / pending) |
| Backward compatibility | `singleLevelM30Invalid` | rejected fail-closed |
| Generation (#1649) | `campaignValid` | a set of proposals covering both genres |
| Generation | `campaignInvalid` | rejected fail-closed |
| Novelty (#1650) | `mixed` | not low-novelty, at least one duplicate detected |
| Novelty | `low` | flagged low-novelty |
| Curve (#1651) | `pass` | verified (passed) |
| Curve | `spike` | not passed (spike + regression) |
| Curve | `missingEvidence` | fail-closed (error) |
| Curation (#1652) | `admit` | `Pass` |
| Curation | `unsolvable` | `Unsolvable` |
| Curation | `imbalanced` | `Imbalanced` |
| Curation | `lowNovelty` | `LowNovelty` |
| Curation | `curveSpike` | `CurveViolation` |
| Curation | `malformed` | `MalformedEvidence` |
| Curation | `missingDimension` | `MalformedEvidence` |

## Contract

- **States and shapes only.** The suite asserts proposal/verdict states, gate
  outcomes, and report shapes — never subjective quality, timing, or ordering of
  non-deterministic data.
- **Backward compatibility.** The single-level Milestone 30 intake path is
  exercised so campaign-scale work cannot silently break it.
- **Reuse.** The runner uses the existing test harness and the real merged
  surfaces; it reuses the fixtures committed by #1649–#1653. It adds no engine,
  runtime, or writer.
- **Conservative wording.** No auto-merge, quality, fun, production-ready,
  shippable, or Godot-replacement claim.

## Generated-state policy

No generated runs/assets/content/release artifacts are committed. The only added
data file is the tiny deterministic matrix under
`examples/content-scale-v1/scenario-coverage-v36/`.

## #1 / #23 governance

#1 remains open as the roadmap/vision anchor and #23 as the repo-memory/design
context anchor. This suite does not close, narrow, or modify either; any such
change requires a separate explicit governance decision.

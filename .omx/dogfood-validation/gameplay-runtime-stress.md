# Dogfood B5 Gameplay / Runtime Stress Evidence

## Metadata

- Blocker: B5 — gameplay/runtime stress evidence is not durable on origin/main
- Report version: `dogfood-gameplay-runtime-stress-v1`
- Demo identity: `collect-and-exit-local-rc-candidate`
- Branch: `dogfood/b5-gameplay-runtime-stress-20260610005954`
- Base: `origin/main` after B4 acceptance and current merge commit `026efcee`
- Source basis: `examples/playable-demo-v2/collect-and-exit/`
- Evidence classification: `bounded-local-runtime-stress-evidence`
- Issue state evidence: #1 OPEN; #23 OPEN.

## Purpose

This handoff makes B5 durable by tracking gameplay/runtime stress evidence for the compact Collect-and-Exit dogfood demo. It records scenario pass/fail coverage, deterministic stress limits, command/output refs, bounded failure cases, and guardrails. It is evidence/coordination only: no product gameplay feature, runtime implementation, release flow, trusted-write path, hosted service, auto-port, foreign-runtime embedding, or Era Q full-3D work is added.

## Merged prerequisite evidence

| Blocker | PR | Origin-main artifact | Status for B5 |
| --- | --- | --- | --- |
| B1 claim coverage | #2334 MERGED | `.omx/dogfood-validation/claim-coverage-matrix.md` | Present; identifies runtime stress as a required dogfood evidence lane. |
| B2 compact demo spec | #2335 MERGED | `.omx/dogfood-validation/demo-game-spec.md` | Present; defines the Collect-and-Exit local/manual demo target and required success/failure/stress scenarios. |
| B3 pipeline dry-run | #2336 MERGED | `.omx/dogfood-validation/pipeline-dry-run.md` | Present; records failed-classified canonical pipeline dry-run evidence, not production readiness. |
| B4 export readiness | #2337 MERGED | `.omx/dogfood-validation/export-release-readiness.md` | Present; records local/manual release-candidate readiness boundaries and retained-artifact gaps. |

## Commands executed

All commands were run from a fresh B5 worktree based on `origin/main`. Generated evidence stayed in test temp directories or ignored build roots.

```bash
node --test \
  examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/gameplay-loop-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/behavior-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/hud-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/level-set-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs
cargo test -p ouroforge-core --test runtime_frame_budget_contract --jobs 2
cargo test -p ouroforge-core --test behavior_contracts behavior_runtime_contract --jobs 2
```

## Evidence summary

| Requirement | Verdict | Evidence / output ref | Notes |
| --- | --- | --- | --- |
| End-to-end runtime smoke | pass | `examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs` output: `collect-and-exit e2e evidence smoke passed` | Exercises the local JS runtime harness over the demo fixture. |
| Gameplay loop | pass | `examples/playable-demo-v2/collect-and-exit/gameplay-loop-smoke.test.cjs` output: `collect-and-exit gameplay loop smoke passed` | Covers key collection, door/exit progression, checkpoint/reset evidence, and bounded local loop behavior. |
| Hazard behavior / bounded failure | pass | `examples/playable-demo-v2/collect-and-exit/behavior-smoke.test.cjs` output: `hazard lose at frame 49, dormant pass wins` | Confirms deterministic expected-negative path and stable hazard failure frame. |
| HUD/runtime state | pass | `examples/playable-demo-v2/collect-and-exit/hud-smoke.test.cjs` output: `collect-and-exit HUD smoke passed` | Confirms visible objective/key/health model coverage. |
| Level-set stress | pass | `examples/playable-demo-v2/collect-and-exit/level-set-smoke.test.cjs` output: `4 levels winnable` | Confirms the bounded level variants remain locally winnable. |
| Scenario matrix | pass | `examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs` output: `12 scenarios validated` | Confirms required acceptance areas, evidence expectations, and forbidden-action guardrails. |
| QA/playtest lanes | pass-with-expected-negative-path | `examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs` output: `3 workers, 16 evidence refs` | Includes win path, designed hazard failure, and integration-surface lane evidence. |
| QA swarm stress | pass-with-classified-failures | `examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs` output: `1 pass / 2 classified fail` | Confirms deterministic local swarm classification without auto-fix/source mutation, including `hazard_contact_loss` and `objective_blocked` failure classes. |
| Runtime frame budget contract | pass | `cargo test -p ouroforge-core --test runtime_frame_budget_contract --jobs 2`: `3 passed` | Confirms within-budget and violation classification at the Rust boundary. |
| Behavior runtime contract | pass | `cargo test -p ouroforge-core --test behavior_contracts behavior_runtime_contract --jobs 2`: `22 passed; 3 filtered out` | Confirms deterministic behavior runtime, evidence bundles, replay keys, loader boundary, and guardrails. |

## Stress limits and bounded failure cases

- Worker/route bound: `qa-playtest-plan.json` uses 3 workers; `qa/qa-swarm-plan.json` keeps routes within its worker policy.
- Runtime frame budget: scene metadata records `updateMs=2`, `renderMs=6`, `evidenceMs=1`, `totalMs=9` against budget `8/16/4/20`; frame-budget contract validates pass and violation classification.
- Hazard failure: expected-negative hazard route classifies `hazard_contact_loss` / designed gameplay failure; behavior smoke reports stable lose frame 49 and a dormant pass path.
- Objective-blocked failure: QA swarm classifies a blocked objective route separately from hazard contact and budget exhaustion.
- Generated-state boundary: playtest and swarm reports are written only to temporary directories removed before exit; no `runs`, `target`, `dashboard-data`, `dist`, screenshots, or QA report outputs are committed by B5.

## Capability boundary

B5 supports a bounded local 2D runtime stress claim for the compact dogfood demo: deterministic input routes, simple AABB collision/trigger interactions, HUD state, checkpoint/reset evidence, level variants, frame-budget classification, and expected-negative QA classifications are covered by local smokes/contracts.

B5 does not prove broad commercial quality, long-session soak, memory/device-matrix performance, production save durability, browser-in-real-Chrome rendering, native/mobile/console export, release readiness, store readiness, secure distribution, full Godot parity, Godot replacement status, plugin runtime execution, or full-3D engine capability.

## Verification commands for this PR

```bash
node --test examples/dogfood-gameplay-runtime-stress-v1/gameplay-runtime-stress-smoke.test.cjs
node --test \
  examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/gameplay-loop-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/behavior-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/hud-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/level-set-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs \
  examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs
cargo test -p ouroforge-core --test runtime_frame_budget_contract --jobs 2
cargo test -p ouroforge-core --test behavior_contracts behavior_runtime_contract --jobs 2
git diff --check origin/main...HEAD
```

## Non-goals and guardrails

- #1 and #23 remain open.
- Era Q M102–M106 remain deferred/non-goal; no full-3D implementation is added.
- No hosted/cloud/multi-user scope, trusted browser/source writes, auto-port, live bridge, runtime embedding, release automation, signing, upload, publishing, credential flow, or Steam depot flow is added.
- No production-ready, store-ready, commercial release, native export, Godot replacement, full Godot parity, or shipped-game maturity claim is made.

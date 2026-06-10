# Dogfood Claim Coverage Matrix

Blocker: **B1 — No claim coverage matrix**
Linked issue claim source: [#1](https://github.com/shaun0927/Ouroforge/issues/1) — final goal and implementation roadmap.
Issue #1 state at capture: **OPEN**.
Issue #23 state at capture: **OPEN**.
Matrix version: `dogfood-claim-coverage-v1`
Updated: `2026-06-10T01:12:00Z`

## Purpose

This matrix is an evidence register for dogfood validation. It maps #1 and compact-demo claims to owner lanes, concrete evidence paths, a verdict, and a gap classification. It does not implement engine behavior and does not close #1 or #23.

Allowed verdicts: `verified`, `unverified`, `deferred`, `non-goal`.
Allowed gap classifications: `verified`, `unverified`, `deferred`, `non-goal`.

## Coverage matrix

| Claim ID | Claim text | #1 link | Owner lane | Evidence path | Verdict | Gap classification | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OF-001 | Ouroforge final goal is an evidence-native game engine loop: Seed → Build → Observe → Verify → Journal → Evolve. | https://github.com/shaun0927/Ouroforge/issues/1 | QA/evidence/evolve | docs/roadmap/vision.md; docs/roadmap/architecture.md; examples/evolve-loop-depth-v1/scenario-coverage-v20-evolve-depth.test.cjs | verified | verified | Existing docs and evolve-depth regression evidence cover the loop shape; this B1 PR only registers coverage. |
| OF-002 | Roadmap status and active Era boundaries are explicit and evidence-linked. | https://github.com/shaun0927/Ouroforge/issues/1 | Integration lead | docs/roadmap/progress.json; docs/roadmap/active/era-q.md; docs/roadmap/CHANGELOG.md | verified | verified | Progress JSON and active Era Q doc are the current local roadmap evidence. |
| OF-003 | Rust verification kernel owns artifact truth and trusted validation semantics. | https://github.com/shaun0927/Ouroforge/issues/1 | Kernel/evaluator | docs/roadmap/architecture.md; crates/ouroforge-core/src/lib.rs; crates/ouroforge-evaluator/src/lib.rs | verified | verified | Local source/docs identify the Rust kernel/evaluator boundary. |
| OF-004 | Studio executor is a control plane and must not own artifact meaning, trusted writes, or releases. | https://github.com/shaun0927/Ouroforge/issues/1 | Studio/control-plane | docs/roadmap/architecture.md; docs/distributed-elixir-design.md | verified | verified | Evidence is design/architecture-level; no new control-plane behavior is added here. |
| OF-005 | Local-first Phoenix LiveView Studio posture is documented; hosted/multi-user remains deferred. | https://github.com/shaun0927/Ouroforge/issues/1 | Studio UX validation | docs/roadmap/architecture.md; docs/distributed-elixir-design.md | verified | verified | Local-first posture is documented; hosted/multi-user is not claimed active. |
| OF-006 | Era Q M101 is DEFER and M102–M106 remain GO-gated, deferred, and unimplemented absent future explicit GO. | https://github.com/shaun0927/Ouroforge/issues/1 | Roadmap governance | docs/roadmap/active/era-q.md; docs/roadmap/progress.json | deferred | deferred | This protects against accidentally treating full-3D M102–M106 as active implementation work. |
| OF-007 | External-engine on-ramp scope is source-only/clean-room; no auto-port, live bridge, or runtime embedding is claimed. | https://github.com/shaun0927/Ouroforge/issues/1 | Migration/on-ramp | docs/roadmap/active/era-p.md; docs/roadmap/milestones/era-r.md | verified | verified | Existing roadmap docs keep source-only and clean-room boundaries explicit. |
| OF-008 | Compact dogfood demo release-candidate evidence is not yet complete until dependent lane reports exist. | https://github.com/shaun0927/Ouroforge/issues/1 | Dogfood integration | .omx/dogfood-validation/blocker-driven-pr-plan.md | unverified | unverified | B2–B5 now have bounded evidence; this row stays unverified to prevent overclaiming production/store readiness while B3 remains failed-classified and B4 retains an RC artifact gap. |
| OF-009 | Pipeline dry-run evidence for the compact demo must exist before claiming reproducible local pipeline readiness. | https://github.com/shaun0927/Ouroforge/issues/1 | Pipeline dry-run | .omx/dogfood-validation/pipeline-dry-run.md | verified | verified | B3 evidence exists and is classified failed; it is evidence completeness, not green runtime/readiness. |
| OF-010 | Runtime stress evidence for the compact demo must exist before claiming runtime stability/scenario stress readiness. | https://github.com/shaun0927/Ouroforge/issues/1 | Gameplay runtime stress | .omx/dogfood-validation/gameplay-runtime-stress.md | verified | verified | B5 bounded local runtime stress evidence exists; it does not claim broad runtime stability, production quality, or store readiness. |
| PROTECT-001 | #1 remains open; this PR must not close or mutate it. | https://github.com/shaun0927/Ouroforge/issues/1 | Integration lead | .omx/dogfood-validation/claim-coverage-matrix.md | verified | non-goal | Guardrail row: the PR body must not contain closing keywords for #1. |
| PROTECT-002 | #23 remains open; this PR must not close or mutate it. | https://github.com/shaun0927/Ouroforge/issues/23 | Integration lead | .omx/dogfood-validation/claim-coverage-matrix.md | verified | non-goal | Guardrail row: the PR body must not contain closing keywords for #23. |

## Gap register

- B2: `.omx/dogfood-validation/demo-game-spec.md` exists and defines the compact demo target scope.
- B3: `.omx/dogfood-validation/pipeline-dry-run.md` exists and records failed-classified local dry-run evidence; downstream runtime/scenario fixes remain separate work.
- B4: `.omx/dogfood-validation/export-release-readiness.md` exists and records local/manual evidence plus an explicit retained-RC-artifact gap.
- B5: `.omx/dogfood-validation/gameplay-runtime-stress.md` exists and records bounded local runtime stress evidence; broad runtime stability/scenario stress readiness remains unclaimed.
- Era Q M102–M106: deferred/non-goal for this PR; no full-3D implementation is added.

## Explicit non-goals

- Do not close #1 or #23.
- Do not implement Era Q full-3D M102–M106.
- Do not add hosted/cloud/multi-user scope.
- Do not add trusted browser/source writes.
- Do not add auto-port, live bridge, or foreign runtime embedding.
- Do not add release automation, signing, upload, Steam publishing, or production/store readiness claims.

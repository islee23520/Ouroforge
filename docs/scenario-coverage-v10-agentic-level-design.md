# Scenario Coverage v10: Agentic Level Design Regression Suite

Issue: #641 - Scenario Coverage v10: Agentic Level Design Regression Suite.

Scenario Coverage v10 records the regression matrix for Agentic Scene and Level
Designer v1. The #640 demo proves the chain composes; this matrix proves each
feature area has focused coverage or a visible gap.

Canonical fixture index:
`examples/agentic-level-design-regression-suite-v1/coverage-matrix.fixture.json`.

## Coverage Matrix

| Area | Valid anchor | Edge/malformed coverage | Test anchor |
| --- | --- | --- | --- |
| Intent validation | `examples/level-intent-v1/level-intent.valid.fixture.json` | partial, blocked, contradictory, missing objectives | `cargo test level_intent_v1` |
| Generation plan validation | `examples/scene-generation-plan-v1/scene-generation-plan.valid.fixture.json` | stale, blocked, missing intent, malformed evidence | `cargo test scene_generation_plan_v1` |
| Layout constraints | `examples/spatial-layout-constraint-solver-v1/spatial-layout.valid.fixture.json` | violated, unsupported, blocked, malformed evidence | `cargo test spatial_layout_solver_v1` |
| Tilemap/terrain draft | `examples/tilemap-terrain-generation-draft-v1/tilemap-terrain-draft.valid.fixture.json` | stale, unsupported, blocked, duplicate placement | `cargo test tilemap_terrain_generation_draft_v1` |
| Entity/objective/encounter placement | `examples/entity-objective-encounter-placement-draft-v1/entity-objective-placement.valid.fixture.json` | stale, unsupported, blocked, overlap | `cargo test entity_objective_placement_draft_v1` |
| Reachability/pathing | `examples/reachability-pathing-evidence-v1/reachability.valid.fixture.json` | unreachable, unsupported, stale, blocked, duplicate cell | `cargo test reachability_pathing_evidence_v1` |
| Objective proof | `examples/objective-completion-proof-v1/objective.complete.fixture.json` | unreachable, stale, blocked, result drift | `cargo test objective_completion_proof_v1` |
| Difficulty/pacing heuristic | `examples/difficulty-pacing-heuristic-evidence-v1/heuristics.within-target.fixture.json` | out-of-target, stale, blocked, warning drift | `cargo test difficulty_pacing_heuristic_v1` |
| Visual/semantic diff | `examples/level-visual-semantic-diff-v1/diff.compared.fixture.json` | unchanged, partial, stale, blocked, status drift | `cargo test level_visual_semantic_diff_v1` |
| Agent-generated level draft | `examples/agent-generated-level-draft-v1/level-draft.drafted.fixture.json` | partial, missing evidence, stale, blocked, duplicate operation | `cargo test agent_generated_level_draft_v1` |
| Review-gated level apply | `examples/review-gated-level-apply-v1/level-apply.ready.fixture.json` | missing review, rejected, stale, blocked, self-approval | `cargo test review_gated_level_apply_v1` |
| Studio/dashboard read model | `examples/agentic-level-design-demo-v1/demo-chain.fixture.json` | missing/malformed Studio read-model assertions | `node examples/agentic-level-design-regression-suite-v1/coverage-smoke.test.cjs` |

The coverage matrix intentionally names malformed, missing, stale, unsupported,
and blocked examples so a passing demo cannot hide feature-specific regressions.

## Generated State

The suite is fixture-scoped. Generated level drafts, previews, screenshots,
runs, dashboard exports, temp projects, and local tool state remain ignored
unless a future issue explicitly scopes a deterministic fixture.

The smoke audits these generated roots remain absent:

- `examples/agentic-level-design-regression-suite-v1/runs`;
- `examples/agentic-level-design-regression-suite-v1/dashboard-data`;
- `examples/agentic-level-design-regression-suite-v1/tmp`.

## Boundaries

Scenario Coverage v10 does not add:

- no autonomous full game generation;
- no browser trusted writes;
- command bridge or local server bridge;
- hidden command execution;
- auto-apply or auto-merge;
- self-approval or reviewer bypass;
- unrestricted source mutation;
- arbitrary script execution, dynamic code loading, plugin loader, or visual
  scripting;
- production editor, native export, hosted/cloud behavior, plugin runtime,
  marketplace, account system, production-ready claim, autonomous launch, or
  current Godot replacement.

Rust/local validation owns trusted persistence, draft/apply validation,
generated evidence writing, and CLI contracts. Browser/dashboard/Studio surfaces
remain read-only or draft-only for trusted state.

#1 and #23 remain open.

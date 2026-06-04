# Agentic Level Design Demo v1

Issue: #640 - Agentic Level Design Demo v1.

Agentic Level Design Demo v1 composes the Agentic Scene and Level Designer v1
contracts into one deterministic source fixture. It demonstrates the evidence
chain for a collect-and-exit level without adding autonomous generation,
browser-side authority, or production-editor behavior.

## Scope

The demo links existing fixture-scoped artifacts for:

- level intent and constraints;
- scene generation plan;
- spatial layout constraint validation;
- tilemap terrain draft;
- entity, objective, and encounter placement draft;
- reachability and pathing evidence;
- objective completion proof;
- difficulty and pacing heuristic evidence;
- visual and semantic diff evidence;
- agent-generated level draft;
- review-gated level apply metadata;
- Studio level design inspection.

The canonical fixture index is
`examples/agentic-level-design-demo-v1/demo-chain.fixture.json`.

## Flow

1. Human/product intent is represented by `level-intent-v1`.
2. The deterministic generation plan records proposed zones, required assets,
   scenario checks, and expected evidence without writing level state.
3. Layout, tilemap, and placement fixtures show proposed construction and
   validation evidence as source-like artifacts.
4. Reachability, objective proof, difficulty/pacing, and visual/semantic diff
   fixtures prove bounded review evidence separately from draft and apply state.
5. The agent-generated level draft bundles the proposal as untrusted review
   input.
6. Review-gated apply metadata records accepted non-self review, rollback
   metadata, rerun command text, and generated-state audit refs. It is readiness
   metadata only; it does not mutate files by itself.
7. Studio/cockpit may inspect the chain as escaped read-only data.

## Commands

Focused demo smoke:

```bash
node examples/agentic-level-design-demo-v1/demo-smoke.test.cjs
```

Supporting checks:

```bash
cargo test agent_generated_level_draft_v1
cargo test review_gated_level_apply_v1
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Full issue closure should also run the repository verification required by the
governing issue: `cargo fmt --check`, `cargo test`,
`cargo clippy --all-targets --all-features -- -D warnings`, dashboard and
cockpit Node checks, `git diff --check`, and live issue checks for #640, #1, and
#23.

## Generated State and Cleanup

The demo smoke reads tracked source fixtures only. It must not leave generated
state under the demo fixture directory. Generated runs, previews, screenshots,
dashboard exports, temp projects, and local tool state remain ignored unless a
later issue explicitly scopes a deterministic fixture.

The smoke audits these fixture-local generated roots remain absent:

- `examples/agentic-level-design-demo-v1/runs`;
- `examples/agentic-level-design-demo-v1/dashboard-data`;
- `examples/agentic-level-design-demo-v1/screenshots`;
- `examples/agentic-level-design-demo-v1/tmp`.

## Known Gaps

- The demo chains deterministic fixtures; it does not create arbitrary levels or
  games autonomously.
- Review-gated apply is represented by trusted metadata and rerun command text;
  this demo does not mutate the collect-and-exit project.
- Difficulty and pacing metrics are advisory only and do not prove subjective
  game quality.
- Browser/dashboard/Studio surfaces inspect exported data only.

## Boundary

This demo does not implement:

- autonomous full game generation;
- browser trusted writes;
- command bridge or local server bridge;
- hidden command execution;
- auto-apply or auto-merge;
- self-approval or reviewer bypass;
- unrestricted source mutation;
- arbitrary script execution, dynamic code loading, plugin loader, or visual
  scripting;
- production editor, full visual level editor, native export, hosted/cloud
  behavior, plugin runtime, marketplace, account system, production-ready claim,
  autonomous launch, or current Godot replacement.

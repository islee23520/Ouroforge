# Agentic Scene and Level Designer v1 Scope Contract

Issue: #627 - Agentic Scene and Level Designer v1 Scope and Contract. This
document is the canonical scope contract for #1 Milestone 11.

Agentic Scene and Level Designer v1 defines a bounded, evidence-gated path for
agents to propose, validate, compare, review, and apply level and scene designs
through local artifacts. It is not autonomous unrestricted game creation, a
production editor, a secure sandbox claim, native export, plugin runtime,
hosted/cloud product, autonomous launch system, or current Godot replacement.

## Purpose

The milestone coordinates follow-up issues for level/scene authoring while
preserving the same trust, evidence, review, and generated-state boundaries used
by existing scene, tilemap, visual draft, scenario, dashboard, and cockpit
contracts. This control document adds no level generator, solver, draft writer,
Studio UI, apply behavior, command bridge, source mutation, export, plugin,
hosted, or runtime feature by itself.

## Bounded Target

The v1 target is 2D-first evidence-gated level and scene authoring with:

- a level intent and design constraint model;
- a scene generation plan artifact;
- spatial layout and placement constraints;
- tilemap and terrain generation drafts;
- entity, objective, and encounter placement drafts;
- reachability and pathing evidence;
- objective completion and win/loss proof evidence;
- difficulty, pacing, and balance heuristic evidence;
- visual and semantic diff evidence;
- an agent-generated level draft bundle;
- review-gated level apply records with rollback metadata;
- Studio/dashboard/cockpit inspection surfaces that remain escaped and
  read-only or draft-only;
- a deterministic demo and regression suite;
- generated drafts, previews, screenshots, runs, dashboard exports, temp
  projects, and local tool state kept ignored unless a later issue explicitly
  scopes a tiny deterministic fixture.

3D compatibility is limited to notes and evidence shapes already authorized by
the 3D Capability Gate contracts. Level design v1 must not imply broad 3D
authoring, physics, import, rendering, export, or editor parity.

## Artifact Separation

Follow-up work must keep these concepts separate:

- intent and design constraints describe goals and boundaries;
- generation plans describe proposed construction steps;
- drafts describe proposed scene, tilemap, asset-reference, placement, or
  objective edits;
- evidence artifacts prove reachability, objective feasibility, pacing,
  visual/semantic diffs, compatibility, and generated-state hygiene;
- review decisions accept, reject, or block a draft without applying it by
  themselves;
- apply records are trusted Rust/local outcomes only when a follow-up issue
  explicitly scopes review-gated apply.

No browser or agent-produced draft is trusted persistence until Rust/local
validation, review gating, and generated-state checks accept it.

## Dependency Order

Follow-up issues should proceed in this order unless a later live issue audit
proves a safer dependency order:

1. #628 - Level Intent and Design Constraint Model v1.
2. #629 - Scene Generation Plan Artifact v1.
3. #630 - Spatial Layout and Placement Constraint Solver v1.
4. #631 - Tilemap and Terrain Generation Draft v1.
5. #632 - Entity, Objective, and Encounter Placement Draft v1.
6. #633 - Reachability and Pathing Evidence v1.
7. #634 - Objective Completion and Win Loss Proof v1.
8. #635 - Difficulty, Pacing, and Balance Heuristic Evidence v1.
9. #636 - Level Visual Diff and Semantic Comparison v1.
10. #637 - Agent-Generated Level Draft v1.
11. #638 - Review-Gated Level Apply v1.
12. #639 - Studio Level Design Inspection Surface v1.
13. #640 - Agentic Level Design Demo v1.
14. #641 - Scenario Coverage v10: Agentic Level Design Regression Suite.
15. #642 - Roadmap and #1 Governance Refresh after Agentic Scene and Level
    Designer v1.

Each issue should use the smallest safe PR unit with focused tests, manual QA
where the surface is user-visible, generated-state audit, and conservative
wording audit. Do not combine intent, planning, solver, draft, evidence,
review/apply, Studio, demo, and regression behavior when independent
verification would be clearer.

## Trusted Boundary

- Rust/local code owns trusted validation, persistence, draft/apply validation,
  generated evidence artifact writing, project/run binding, source-like fixture
  validation, and CLI behavior.
- Agents may propose intent, plans, drafts, evidence, review notes, and
  candidate patches, but their outputs are untrusted until the scoped Rust/local
  boundary validates them.
- Browser runtime, dashboard, and Studio/cockpit surfaces may render exported
  evidence and prepare draft-only copyable data when explicitly scoped. They
  must not write trusted files, execute commands, start local servers, bridge to
  shell tools, auto-apply, auto-merge, self-approve, mutate source, publish,
  export, or persist browser state as authority.
- Generated level drafts, previews, screenshots, runs, dashboard exports, temp
  projects, browser profiles, and local tool state stay untracked unless a
  follow-up issue explicitly scopes deterministic source-like fixtures.

## Compatibility Expectations

Level design work must be additive. It must preserve existing Seeds, scenes,
tilemaps, assets, project manifests, runs, scenarios, dashboard exports, Studio
read models, behavior contracts, source-like fixtures, and already-scoped 3D
capability contracts unless a PR includes an explicit migration note and focused
compatibility tests.

Existing scene, tilemap, asset, visual draft, scenario, evidence, dashboard, and
cockpit models should be extended before adding parallel systems. New schemas
should prefer additive versioned fields and explicit read-model branches over
ambiguous shape changes.

## Verification and Closure Gates

Every Agentic Scene and Level Designer follow-up issue should include:

- live issue checks for the current issue, #1, and #23;
- focused Rust tests or Node smokes for new schemas, read models, fixtures,
  dashboard/cockpit display, or browser-local behavior;
- compatibility checks for touched Seeds, scenes, tilemaps, assets, project
  manifests, runs, scenarios, dashboard exports, Studio read models, and
  behavior contracts;
- generated-state audit showing only ignored local/runtime outputs are present;
- conservative wording audit: no autonomous full game generation, production
  editor, current Godot replacement, production-ready, secure-sandbox, native
  export, plugin runtime, hosted/cloud, or autonomous launch claims;
- final latest-main gate before issue closure:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

Browser/dashboard/Studio issues should add read-only rendering, missing or
malformed state, and XSS escaping coverage. Trusted Rust/local issues should add
invalid, stale, missing-evidence, generated-state, and review-gate tests.

## Non-Goals for Milestone 11

Milestone 11 does not authorize:

- autonomous full game generation or unrestricted project mutation;
- production editor or full visual level editor claims;
- visual scripting implementation;
- arbitrary code/script generation or execution;
- browser trusted writes, browser command bridges, local server command bridges,
  hidden command execution, auto-apply, auto-merge, or self-approval;
- unrestricted source mutation apply;
- native export, platform packaging, signing, notarization, or release/publish
  automation;
- plugin runtime, marketplace, hosted/cloud/server/auth/account behavior, remote
  asset hosting, collaboration infrastructure, or production CI/CD automation;
- claims that heuristic difficulty or pacing metrics prove subjective game
  quality;
- current Godot replacement, production-ready engine, shipped-game maturity,
  secure-sandbox, or broad compatibility-stable claims.

## Governance Anchors

- #1 remains the roadmap/final-goal anchor and must stay open unless a separate
  explicit governance decision changes it.
- #23 remains the memory/governance anchor and must stay open unless a separate
  explicit governance decision changes it.
- This document may be revised only by an explicit follow-up governance issue or
  roadmap refresh. Implementation issues should cite it rather than weakening
  its boundaries locally.

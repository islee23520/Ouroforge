# Ouroforge Roadmap

## Current status

Ouroforge is a local, evidence-native MVP. It now supports a small project
workspace loop in addition to the original run-centered demo, with hardened
run evidence fidelity and a completed Agentic Review & Regression Promotion v1
loop around proposal rationale, review decisions, review-gated scene
application, rerun comparison, regression promotion, Journal v2, and Studio
review cockpit state:

```text
project manifest -> scaffold/scene/seed/scenario pack -> run -> evidence -> compare -> proposal/review/application -> regression promotion/matrix -> journal/Studio inspection
```

The trusted boundary remains Rust and the local filesystem. Browser examples
read exported JSON and show copyable commands; they do not execute commands,
write trusted files, accept mutations, or act as a production editor. Browser
and CDP observations are evidence inputs, not trusted authority.

## Completed evidence-native milestones

The current implementation has completed these documented milestone surfaces:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`)
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`,
  `docs/scenario-evaluator-v1-demo.md`)
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`)
- Studio v1 (`docs/studio-v1.md`, `docs/studio-v1-demo.md`)
- Engine Expansion v1 (`docs/engine-expansion-v1.md`,
  `docs/engine-expansion-v1-demo.md`)
- Authoring Loop v2 (`docs/authoring-loop-v2.md`,
  `docs/scene-edit-transactions.md`, `docs/run-comparison-v2.md`,
  `docs/scene-only-mutation-v2.md`, `docs/studio-v2-cockpit.md`)
- Project Workspace Loop v1 (`docs/project-workspace-loop-v1.md`,
  `docs/project-manifest-v1.md`, `docs/project-scaffold-v1.md`,
  `docs/scenario-pack-v1.md`, `docs/project-run-v1.md`,
  `docs/project-comparison-v1.md`, `docs/project-mutation-loop-v1.md`,
  `docs/studio-v3-project-workspace-cockpit.md`)
- Evidence Fidelity & Trust Boundary Hardening v1
  (`docs/evidence-fidelity-trust-boundary-v1.md`,
  `docs/runtime-probe-contract-v2.md`,
  `docs/input-replay-evidence-v2.md`,
  `docs/openchrome-cdp-evidence-fidelity-v2.md`,
  `docs/reproducible-run-command-context-v1.md`,
  `docs/studio-evidence-fidelity-surfaces.md`)
- Agentic Review & Regression Promotion v1
  (`docs/agentic-review-regression-promotion-v1.md`,
  `docs/mutation-proposal-quality-v2.md`,
  `docs/review-decision-ledger-v1.md`,
  `docs/project-mutation-loop-v1.md`,
  `docs/regression-run-matrix-v1.md`,
  `docs/evidence-backed-journal-v2.md`,
  `docs/studio-review-cockpit-v1.md`)

These milestones are still MVP contracts, not public compatibility promises.
Generated run evidence remains ignored local state unless an issue explicitly
scopes a tiny deterministic fixture as tracked source-like data.

## Near-term governance and public-readiness work

Agentic Review & Regression Promotion v1 issue #302 is the current roadmap/#1
governance refresh. Its purpose is to keep top-level docs and #1 aligned with
the completed review/regression milestone while preserving conservative public
wording and leaving #1/#23 open.

After #302, the next milestone candidates should stay inside the same
local-first, Rust-trusted, browser-read-only boundary. Suitable candidates are
agentic loop orchestration hardening, playable authoring expressiveness,
source-mutation design gates, asset pipeline work, visual authoring, or public
alpha readiness only when each is backed by fixed PR units, regression coverage,
generated-state audits, and explicit non-goals.

The public-readiness docs remain governance inputs, not automated launch paths:

- `docs/public-readiness-audit.md`
- `docs/public-demo-evidence.md`
- `docs/public-launch-checklist.md`

Maintainers should use those documents for a separate manual repository
visibility decision. Public launch remains a governance action, not an automated
code path.

## Product direction

- Keep the evidence-native loop inspectable, file-based, and local-first.
- Use Rust-owned validation for trusted persistence, project resolution, run
  binding, comparison artifacts, and scene-only mutation application.
- Keep browser surfaces static/read-only for trusted state: display exported
  data, preview runtime state, and show copyable CLI commands only.
- Expand runtime/scenario coverage only when backed by concrete issues and
  tests.
- Keep authoring actions connected to QA evidence, semantic comparison,
  journals, rollback metadata, explicit mutation review, and regression
  promotion/matrix context.
- Treat evidence fidelity and review governance as first-class contracts: every
  run-facing surface should distinguish Rust-trusted artifacts from browser/CDP
  observations, and should expose missing or malformed evidence/review state as
  warnings instead of inferred passes.

## Active anchors

- #1 remains the broad vision and implementation-roadmap anchor until a separate
  explicit governance decision replaces it.
- #23 remains open as the repo-memory/design context anchor.

## Non-goals

Ouroforge is not currently trying to be:

- a Godot replacement;
- a production-ready or compatibility-stable public engine API;
- a hosted/cloud engine;
- a native packaged editor or native export implementation;
- a general marketplace or plugin platform;
- a browser-side trusted file writer or command bridge;
- an autonomous public-launch automation system.

Any shift in those boundaries requires a design issue, explicit maintainer
approval, and evidence that the change belongs in the current roadmap.

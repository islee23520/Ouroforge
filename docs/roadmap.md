# Ouroforge Roadmap

## Current status

Ouroforge is a local, evidence-native MVP. It now supports a small project
workspace loop in addition to the original run-centered demo:

```text
project manifest -> scaffold/scene/seed/scenario pack -> run -> evidence -> compare -> journal/mutation review -> Studio inspection
```

The trusted boundary remains Rust and the local filesystem. Browser examples
read exported JSON and show copyable commands; they do not execute commands,
write trusted files, accept mutations, or act as a production editor.

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

These milestones are still MVP contracts, not public compatibility promises.
Generated run evidence remains ignored local state unless an issue explicitly
scopes a tiny deterministic fixture as tracked source-like data.

## Near-term governance and public-readiness work

Project Workspace Loop v1 issue #253 is the current roadmap/#1 governance
refresh. Its purpose is to keep top-level docs and #1 aligned with the completed
local-first evidence loop while preserving conservative public wording.

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
  journals, rollback metadata, and explicit mutation review.

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

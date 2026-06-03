# Ouroforge

Ouroforge is an evidence-native game engine experiment built around local Ouroboros-style loops: Seed → Run → Evidence → Evaluation → Journal → Mutation proposal.

> Status: pre-release private MVP moving toward public open-source readiness. Ouroforge is not a Godot replacement, does not promise compatibility, and should be treated as an inspectable local prototype. Current roadmap docs record completed Evidence Fidelity & Trust Boundary hardening, Agentic Review & Regression Promotion v1, Agentic Loop Orchestration v1, and the implemented Engine Expressiveness v2 playable-authoring surfaces while preserving a local-first, Rust-trusted, browser-read-only boundary.

## What works today

- Validate a Seed file.
- Create local run artifacts under `runs/`.
- Capture browser/runtime evidence through local Chrome DevTools Protocol.
- Execute a minimal Scenario DSL against the runtime.
- Produce deterministic evaluator verdicts and journals.
- Propose deterministic mutation records for failed runs.
- Inspect runs with a read-only evidence dashboard, including the exported regression run matrix for project-bound scenario history.
- Inspect/edit the minimal game-runtime scene through Rust-validated scene edit commands and a static authoring cockpit prototype.
- Inspect existing before/after run comparison artifacts, including project comparison context, in the read-only evidence dashboard.
- Validate a local project workspace manifest with `project validate`.
- Scaffold a tiny local project workspace with `project init --template minimal-2d`.
- Validate project scenario packs through `project validate`.
- Run project-declared Seeds with additive project metadata using `run <seed> --project <root-or-manifest> --scenario-pack <id>`.
- Apply scene-only mutations through Rust validation when authorized by project manifest context.
- Inspect Project Workspace Loop v1, Evidence Fidelity, Agentic Review/Regression, Agentic Loop Orchestration, and Engine Expressiveness v2 scene/runtime state in the static Studio cockpit without browser writes or command execution.

## Prerequisites

- Rust toolchain with Cargo.
- Node.js for static UI smoke tests.
- Python 3 for local static file serving in the MVP command.
- Chrome or Chromium available at a standard path, or set `OUROFORGE_CHROME=/path/to/chrome`.

No Playwright, database, cloud service, account system, or hosted runtime is required.

## Quickstart

```bash
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid
cargo run -p ouroforge-cli -- project init .omx/tmp/project-scaffold-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run .omx/tmp/project-scaffold-smoke/seeds/platformer.yaml --project .omx/tmp/project-scaffold-smoke --scenario-pack smoke --workers 1
rm -rf .omx/tmp/project-scaffold-smoke
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- seed validate seeds/runtime-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4
```

The run command prints a run directory such as `runs/run-...`. Project-bound runs add optional project context to `run.json`, ledger, journal, and dashboard export; legacy runs without `--project` remain compatible. Generated run artifacts are intentionally ignored by git.

Inspect a run:

```bash
cargo run -p ouroforge-cli -- evidence list runs/<run-id>
cargo run -p ouroforge-cli -- journal show runs/<run-id>
cargo run -p ouroforge-cli -- mutation list runs/<run-id>
cargo run -p ouroforge-cli -- compare runs/<run-id> runs/<run-id>
```

Export dashboard data and serve the static UI:

```bash
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Then open:

- Evidence dashboard: <http://127.0.0.1:8000/examples/evidence-dashboard/>
- Authoring cockpit: <http://127.0.0.1:8000/examples/authoring-cockpit/>
- Runtime demo: <http://127.0.0.1:8000/examples/game-runtime/>
- Playable Demo v2 collect-and-exit fixture: <http://127.0.0.1:8000/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json>

Public-readiness demo screenshots and fresh-clone smoke evidence are recorded in `docs/public-demo-evidence.md`. The Engine Expansion v1 playable template is `examples/game-runtime/scene.json` plus `seeds/platformer.yaml`; the final integration demo is `seeds/engine-expansion-v1-demo.yaml` and documented in `docs/engine-expansion-v1-demo.md`. Run either seed with `--workers 4`, export dashboard data, and compare generated runs when checking dashboard/compare compatibility. Runtime v1 playable demo evidence is recorded in `docs/runtime-v1-demo.md`. Scenario/Evaluator v1 integration demo evidence is recorded in `docs/scenario-evaluator-v1-demo.md`. Evolve Loop v1 integration demo evidence is recorded in `docs/evolve-loop-v1-demo.md`. Studio v1 composition evidence is recorded in `docs/studio-v1-demo.md`. Authoring Loop v2 evidence is documented in `docs/authoring-loop-v2.md`, `docs/scene-edit-transactions.md`, `docs/run-comparison-v2.md`, `docs/input-replay-evidence-v2.md`, `docs/scene-only-mutation-v2.md`, `docs/studio-v2-cockpit.md`, `docs/openchrome-cdp-evidence-fidelity-v2.md`, `docs/reproducible-run-command-context-v1.md`, and `docs/studio-evidence-fidelity-surfaces.md`. Project Workspace Loop v1 evidence is documented in `docs/project-workspace-loop-v1.md`, `docs/project-run-v1.md`, `docs/project-comparison-v1.md`, `docs/project-mutation-loop-v1.md`, and `docs/studio-v3-project-workspace-cockpit.md`. Agentic Review & Regression Promotion v1 is documented in `docs/agentic-review-regression-promotion-v1.md`, `docs/mutation-proposal-quality-v2.md`, `docs/review-decision-ledger-v1.md`, `docs/regression-run-matrix-v1.md`, `docs/evidence-backed-journal-v2.md`, and `docs/studio-review-cockpit-v1.md`. Agentic Loop Orchestration v1 is completed and documented in `docs/agentic-loop-orchestration-v1.md`, `docs/authoring-loop-plan-v1.md`, `docs/authoring-loop-dry-run-v1.md`, `docs/authoring-loop-execution-v1.md`, `docs/authoring-loop-recovery-v1.md`, `docs/authoring-loop-evidence-bundle-v1.md`, `docs/agent-handoff-contract-v1.md`, and the Studio Loop Cockpit notes in `examples/authoring-cockpit/README.md`. Engine Expressiveness v2 scope and implemented evidence are documented in `docs/engine-expressiveness-v2.md`, `docs/engine-expressiveness-v2-governance-handoff.md`, `docs/scene-component-model-v2.md`, `docs/collision-physics-v2.md`, `docs/gameplay-trigger-flags-v1.md`, `docs/scene-transitions-v1.md`, `docs/playable-demo-v2-collect-and-exit.md`, `docs/scenario-coverage-v3.md`, and `docs/studio-authoring-surface-v2-expressive-inspection.md`; animation/audio gameplay events and manifest-declared scene transitions are completed bounded evidence surfaces, not broad editor/runtime claims. Source Mutation Preview v1 scope and guardrails are documented in `docs/source-mutation-preview-v1.md`. Playable Demo v2 collect-and-exit evidence is documented in `docs/playable-demo-v2-collect-and-exit.md`. Scenario Coverage v3 evidence is documented in `docs/scenario-coverage-v3.md`.

## Verification

Recommended local verification before opening a PR:

```bash
cargo fmt --check
cargo test
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

For Engine Expansion v1 integration evidence, validate and run `seeds/engine-expansion-v1-demo.yaml` and `seeds/platformer.yaml` with `--workers 4`, export dashboard data, compare the latest two generated runs, and record the generated run ids plus comparison artifact. For Runtime v1 demo evidence, run `cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4`, export dashboard data, and record the generated run id. For Scenario/Evaluator v1 demo evidence, validate and run `seeds/scenario-evaluator-v1-demo.yaml`, inspect generated `scenario_input_replay` artifacts when replay provenance is in scope, compare two generated demo runs when before/after evidence is needed, export dashboard data, and record the generated run ids. The dashboard comparison panel is read-only: it displays existing comparison artifacts and evidence links without computing browser-side comparisons, mutating runs, accepting mutations, or generating AI summaries. For Evolve Loop v1 demo evidence, validate and run `seeds/evolve-v1-demo.yaml`, then run `cargo run -p ouroforge-cli -- evolve demo runs/<run-id>` and record the lifecycle summary. For Project Workspace Loop v1 evidence, scaffold a temporary `minimal-2d` project, validate the manifest, run the project seed with `--project` and `--scenario-pack`, export dashboard data, and remove generated dashboard output after verification. For Openchrome/CDP Evidence Fidelity v2 smoke evidence, run `seeds/runtime-v1-demo.yaml` with `--workers 2`, export dashboard data, confirm console/performance/CDP metadata in the generated read model, and remove generated dashboard data before commit. For Reproducible Run Command Context v1 evidence, run a legacy seed and a project-bound seed, export dashboard data, confirm read-only command context in journal/dashboard/cockpit, and remove generated dashboard/temp project output before commit. For Studio Evidence Fidelity surface evidence, export dashboard data, confirm the cockpit evidence fidelity panel renders transaction/probe/replay/CDP/command-context status, and remove generated dashboard output before commit. For Regression Run Matrix v1 evidence, create or reuse project-bound runs with `--scenario-pack`, export dashboard data, confirm dashboard/cockpit matrix panels render pass/fail/pending status read-only, and keep generated `runs/` plus `dashboard-data.json` untracked. For Evidence-Backed Journal v2 evidence, run/update a generated journal, confirm the `Authoring Governance Lifecycle` section shows proposal/review/application/compare/promotion ids or explicit missing/malformed states, and keep generated journals/runs untracked. For Studio Review Cockpit v1 evidence, export dashboard data, confirm proposal/review/application/promotion/matrix cards render as escaped read-only state in the cockpit, and keep generated dashboard output untracked. For Agentic Loop Orchestration v1 evidence, generate loop plan/dry-run/execution/recovery/bundle/handoff artifacts as issue scope requires, export dashboard data, confirm the Studio loop cockpit renders escaped read-only state, and keep generated dashboard output untracked. For Playable Demo v2 evidence, run `node examples/game-runtime/playable-demo-v2.test.cjs` and `node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs`; for Scenario Coverage v3 evidence, run `node examples/engine-expressiveness-v2-regression/evidence-smoke.test.cjs`; for Studio Authoring Surface v2 evidence, run the cockpit smoke checks and confirm expressive component, collision, trigger, HUD, transition, and event panels render exported data as escaped read-only state. From a Cargo-enabled environment also validate the project and Seed listed in `docs/playable-demo-v2-collect-and-exit.md`. For public-readiness smoke evidence, also run the MVP command with `--workers 4` and record the generated run id.

## Repository map

- `crates/ouroforge-core` — Seed, run artifacts, ledger/evidence APIs, CDP/browser smoke, Scenario DSL, evaluator, journal, mutation proposals, evolve v0, project manifest/scaffold/run/comparison/mutation models, dashboard read model, regression run matrix model, scene edit model.
- `crates/ouroforge-cli` — CLI entrypoints for Seed/run/evidence/journal/mutation/dashboard/scene/project commands.
- `seeds/` — MVP Seed examples.
- `examples/runtime-probe` — minimal runtime probe page.
- `examples/game-runtime` — minimal 2D runtime demo.
- `examples/playable-demo-v2/collect-and-exit` — source-only collect-and-exit demo fixture and evidence smoke.
- `examples/engine-expressiveness-v2-regression` — Scenario Coverage v3 source fixture, scenario pack, and evidence smoke.
- `examples/evidence-dashboard` — read-only static run inspection UI.
- `examples/authoring-cockpit` — static Studio v3 authoring cockpit prototype over exported run/project evidence and the minimal scene model.
- `docs/` — architecture, roadmap, evidence-fidelity/trust-boundary contracts, manifest/scaffold/scenario-pack/project-run/project-comparison/project-mutation contracts, Engine Expressiveness v2 evidence, source mutation preview scope, artifact write policy, public-readiness audit, and demo evidence notes.

## Generated local state

Do not commit generated or local runtime/tool state:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- `dashboard-data/`
- `.claude/`
- `.openchrome/`
- `.omc/`
- `.omx/`

See `docs/artifact-write-policy-v1.md` for the trusted artifact write categories and generated-output/source-like collision policy.

## Maturity boundaries

Ouroforge currently targets one local reproducible MVP demo. It does not provide:

- hosted/cloud execution;
- authentication or authorization;
- native app packaging or native export implementation;
- plugin systems;
- marketplace features;
- visual scripting;
- browser-side trusted file writes or command bridges;
- broad engine compatibility guarantees.

Public release requires the evidence gates in `docs/public-readiness-audit.md` and `docs/public-launch-checklist.md`, followed by a separate manual visibility decision.

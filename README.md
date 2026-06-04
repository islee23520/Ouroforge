# Ouroforge

Ouroforge is an evidence-native game engine experiment built around local Ouroboros-style loops:

> **Seed → Run → Evidence → Evaluation → Journal → Mutation proposal**

It is a **local-first, Rust-trusted, browser-read-only** prototype: the Rust CLI owns all state and mutations, while the browser surfaces (dashboard, authoring cockpit) only *read* exported evidence — they never write files or execute commands.

> **Status:** pre-release private MVP moving toward public open-source readiness. Ouroforge is **not** a Godot replacement, makes no compatibility promises, and should be treated as an inspectable local prototype. See [Maturity boundaries](#maturity-boundaries) for what it deliberately does not do.

## What works today

- Validate a Seed file and scaffold/validate a local project workspace (`project init` / `project validate`).
- Create local run artifacts under `runs/`, including project-bound runs with additive metadata.
- Capture browser/runtime evidence through local Chrome DevTools Protocol.
- Execute a minimal Scenario DSL against the runtime and produce deterministic evaluator verdicts + journals.
- Propose deterministic mutation records for failed runs.
- Apply **scene-only** mutations through Rust validation when authorized by project manifest context.
- Inspect/edit the minimal game-runtime scene through Rust-validated scene edit commands and a static authoring cockpit prototype.
- Inspect runs — evidence, comparisons, regression run matrix, review/promotion state, asset evidence, and visual-authoring draft/diff/review evidence — in a read-only dashboard and Studio cockpit, with no browser writes or command execution.

## Prerequisites

- Rust toolchain with Cargo.
- Node.js for static UI smoke tests.
- Python 3 for local static file serving in the MVP command.
- Chrome or Chromium at a standard path, or set `OUROFORGE_CHROME=/path/to/chrome`.

No Playwright, database, cloud service, account system, or hosted runtime is required.

## Quickstart

```bash
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid
cargo run -p ouroforge-cli -- project init .omx/tmp/project-scaffold-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run .omx/tmp/project-scaffold-smoke/seeds/platformer.yaml \
    --project .omx/tmp/project-scaffold-smoke --scenario-pack smoke --workers 1
rm -rf .omx/tmp/project-scaffold-smoke
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
```

The run command prints a run directory such as `runs/run-...`. Project-bound runs add optional project context to `run.json`, ledger, journal, and dashboard export; legacy runs without `--project` stay compatible. Generated run artifacts are intentionally git-ignored.

Inspect a run:

```bash
cargo run -p ouroforge-cli -- evidence list runs/<run-id>
cargo run -p ouroforge-cli -- journal show runs/<run-id>
cargo run -p ouroforge-cli -- mutation list runs/<run-id>
cargo run -p ouroforge-cli -- compare runs/<run-id> runs/<run-id>
```

Export dashboard data and serve the static read-only UIs:

```bash
cargo run -p ouroforge-cli -- dashboard export \
    --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Then open:

- Evidence dashboard: <http://127.0.0.1:8000/examples/evidence-dashboard/>
- Authoring cockpit: <http://127.0.0.1:8000/examples/authoring-cockpit/>
- Runtime demo: <http://127.0.0.1:8000/examples/game-runtime/>
- Playable Demo v2 (collect-and-exit): <http://127.0.0.1:8000/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json>

## Verification

Recommended local checks before opening a PR:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Each completed milestone has its own integration-evidence procedure (which seeds to run, which smokes to execute, what to record). Those per-milestone steps live in the matching `docs/*.md` file — see [Documentation](#documentation) — and contribution expectations are in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Documentation

Start here:

| Topic | Doc |
|-------|-----|
| Architecture | [`docs/architecture.md`](docs/architecture.md) |
| Roadmap | [`docs/roadmap.md`](docs/roadmap.md) |
| Evidence fidelity & trust boundary | [`docs/evidence-fidelity-trust-boundary-v1.md`](docs/evidence-fidelity-trust-boundary-v1.md) |
| Artifact write policy | [`docs/artifact-write-policy-v1.md`](docs/artifact-write-policy-v1.md) |
| Public-readiness gates | [`docs/public-readiness-audit.md`](docs/public-readiness-audit.md), [`docs/public-launch-checklist.md`](docs/public-launch-checklist.md) |

Completed milestones (each has detailed contracts and demo-evidence notes in `docs/`, grouped by theme):

- **Core loop** — Runtime v1, Scenario/Evaluator v1, Evolve Loop v1 (`docs/runtime-v1.md`, `docs/scenario-evaluator-v1.md`, `docs/evolve-loop-v1.md`).
- **Project workspace** — manifest, scaffold, scenario packs, project run/comparison/mutation (`docs/project-workspace-loop-v1.md` and the `docs/project-*-v1.md` set).
- **Agentic review & regression** — review decision ledger, regression run matrix, evidence-backed journal, Studio review cockpit (`docs/agentic-review-regression-promotion-v1.md`).
- **Agentic loop orchestration** — plan / dry-run / execution / recovery / evidence-bundle / agent-handoff (`docs/agentic-loop-orchestration-v1.md`).
- **Engine expressiveness v2** — scene component model, collision/physics, trigger flags, scene transitions (`docs/engine-expressiveness-v2.md`).
- **Asset pipeline v1** — asset manifest, preview evidence, reference integrity (`docs/asset-pipeline-v1.md`, `docs/asset-pipeline-v1-governance-handoff.md`).
- **Visual authoring v1** — safe local edit-draft cockpit, Rust preflight/transaction preview, review-gated apply (`docs/visual-authoring-v1.md`).
- **Source mutation (gated)** — design gate, threat model, file classes, inert patch preview, sandbox dry-run boundary; source-apply remains **blocked** by design (`docs/source-mutation-design-gate-v1.md`, `docs/source-mutation-sandbox-boundary-v1.md`).

The full set of 90+ contracts and demo-evidence notes lives under [`docs/`](docs/).

## Repository map

- `crates/ouroforge-core` — Seed, run artifacts, ledger/evidence APIs, CDP/browser smoke, Scenario DSL, evaluator, journal, mutation proposals, evolve, project manifest/scaffold/run/comparison/mutation models, dashboard read model, regression run matrix model, scene edit model.
- `crates/ouroforge-cli` — CLI entrypoints for seed/run/evidence/journal/mutation/dashboard/scene/project commands.
- `seeds/` — MVP Seed examples.
- `examples/game-runtime` — minimal 2D runtime demo.
- `examples/runtime-probe` — minimal runtime probe page.
- `examples/evidence-dashboard` — read-only static run inspection UI.
- `examples/authoring-cockpit` — static Studio authoring cockpit prototype over exported run/project evidence.
- `examples/playable-demo-v2/` — source-only asset-backed collect-and-exit demo fixture and evidence smokes.
- `examples/*-regression`, `examples/*-v1`, `examples/*-v2` — milestone source fixtures, scenario packs, and evidence smokes.
- `docs/` — architecture, roadmap, trust-boundary/evidence contracts, milestone evidence/governance notes, and public-readiness audits.

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

See [`docs/artifact-write-policy-v1.md`](docs/artifact-write-policy-v1.md) for trusted artifact write categories and the generated-output / source-like collision policy.

## Maturity boundaries

Ouroforge currently targets one local reproducible MVP demo. It does **not** provide:

- hosted/cloud execution;
- authentication or authorization;
- native app packaging or native export implementation;
- plugin systems;
- marketplace features;
- visual scripting;
- browser-side trusted file writes or command bridges;
- broad engine compatibility guarantees.

Public release requires the evidence gates in [`docs/public-readiness-audit.md`](docs/public-readiness-audit.md) and [`docs/public-launch-checklist.md`](docs/public-launch-checklist.md), followed by a separate manual visibility decision.

## Contributing & security

- Contribution workflow and review expectations: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Security policy and vulnerability reporting: [`SECURITY.md`](SECURITY.md)
- License: see [`LICENSE`](LICENSE)

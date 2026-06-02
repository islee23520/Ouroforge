# Ouroforge

Ouroforge is an evidence-native game engine experiment built around local Ouroboros-style loops: Seed → Run → Evidence → Evaluation → Journal → Mutation proposal.

> Status: pre-release private MVP moving toward public open-source readiness. Ouroforge is not a Godot replacement, does not promise compatibility, and should be treated as an inspectable local prototype.

## What works today

- Validate a Seed file.
- Create local run artifacts under `runs/`.
- Capture browser/runtime evidence through local Chrome DevTools Protocol.
- Execute a minimal Scenario DSL against the runtime.
- Produce deterministic evaluator verdicts and journals.
- Propose deterministic mutation records for failed runs.
- Inspect runs with a read-only evidence dashboard.
- Inspect/edit the minimal game-runtime scene through Rust-validated scene edit commands and a static authoring cockpit prototype.
- Inspect existing before/after run comparison artifacts in the read-only evidence dashboard.

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
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- seed validate seeds/runtime-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4
```

The run command prints a run directory such as `runs/run-...`. Generated run artifacts are intentionally ignored by git.

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

Public-readiness demo screenshots and fresh-clone smoke evidence are recorded in `docs/public-demo-evidence.md`. The Engine Expansion v1 playable template is `examples/game-runtime/scene.json` plus `seeds/platformer.yaml`; run it with `--workers 4`, export the dashboard data, and compare the generated run to itself when checking dashboard/compare compatibility. Runtime v1 playable demo evidence is recorded in `docs/runtime-v1-demo.md`. Scenario/Evaluator v1 integration demo evidence is recorded in `docs/scenario-evaluator-v1-demo.md`. Evolve Loop v1 integration demo evidence is recorded in `docs/evolve-loop-v1-demo.md`. Studio v1 composition evidence is recorded in `docs/studio-v1-demo.md`.

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

For Engine Expansion v1 template evidence, run `cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4`, export dashboard data, run `cargo run -p ouroforge-cli -- compare runs/<run-id> runs/<run-id>`, and record the generated run id plus comparison artifact. For Runtime v1 demo evidence, run `cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4`, export dashboard data, and record the generated run id. For Scenario/Evaluator v1 demo evidence, validate and run `seeds/scenario-evaluator-v1-demo.yaml`, compare two generated demo runs when before/after evidence is needed, export dashboard data, and record the generated run ids. The dashboard comparison panel is read-only: it displays existing comparison artifacts and evidence links without computing browser-side comparisons, mutating runs, accepting mutations, or generating AI summaries. For Evolve Loop v1 demo evidence, validate and run `seeds/evolve-v1-demo.yaml`, then run `cargo run -p ouroforge-cli -- evolve demo runs/<run-id>` and record the lifecycle summary. For public-readiness smoke evidence, also run the MVP command with `--workers 4` and record the generated run id.

## Repository map

- `crates/ouroforge-core` — Seed, run artifacts, ledger/evidence APIs, CDP/browser smoke, Scenario DSL, evaluator, journal, mutation proposals, evolve v0, dashboard read model, scene edit model.
- `crates/ouroforge-cli` — CLI entrypoints for Seed/run/evidence/journal/mutation/dashboard/scene commands.
- `seeds/` — MVP Seed examples.
- `examples/runtime-probe` — minimal runtime probe page.
- `examples/game-runtime` — minimal 2D runtime demo.
- `examples/evidence-dashboard` — read-only static run inspection UI.
- `examples/authoring-cockpit` — static authoring cockpit prototype over the minimal scene model.
- `docs/` — architecture, roadmap, public-readiness audit, and demo evidence notes.

## Generated local state

Do not commit generated or local runtime/tool state:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- `.openchrome/`
- `.omc/`
- `.omx/`

## Maturity boundaries

Ouroforge currently targets one local reproducible MVP demo. It does not provide:

- hosted/cloud execution;
- authentication or authorization;
- native app packaging;
- plugin systems;
- marketplace features;
- visual scripting;
- broad engine compatibility guarantees.

Public release requires the evidence gates in `docs/public-readiness-audit.md` and `docs/public-launch-checklist.md`, followed by a separate manual visibility decision.

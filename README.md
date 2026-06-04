# Ouroforge

**A game engine that closes its own loop — where a game observes how it ran, judges the evidence, and proposes its own next change.**

The name is *Ouroboros* (the serpent that feeds on its own tail) + *Forge*. The ambition is a creation cycle that turns back on itself:

> **Seed → Run → Evidence → Evaluation → Journal → Mutation → (back to Seed)**

You declare intent as a **Seed** — a goal with acceptance criteria. Ouroforge runs it, captures **evidence** from the real runtime, renders a deterministic **verdict**, **journals** what actually happened, and — when a run falls short — proposes the **mutation** that should come next. Then the loop turns again. The long arc is agentic, self-evolving game authoring: AI proposes the changes, evidence decides which ones become real.

## The conviction: evidence over assertion

A self-mutating engine is only safe if nothing it does is taken on faith. So Ouroforge is **evidence-native**: every step is an inspectable local artifact, never hidden orchestration state. One boundary makes the whole loop trustworthy —

- **The Rust core and the local filesystem are the only trusted authority.** They own all state and every mutation.
- **Agents, browsers, and CDP observations are *evidence inputs*, never authority.** They can inform a proposal; they can never apply one.
- **The browser surfaces only read.** The dashboard and authoring cockpit render exported JSON and copyable commands — they never write files, run commands, or accept mutations.

Intent becomes a playable artifact with provenance at every link of the chain — which is what lets an AI-in-the-loop forge stay honest.

> **Status:** pre-release private MVP, local-first, with public-readiness evidence and launch-governance handoffs recorded for a future manual visibility review. It runs one reproducible local demo today. Ouroforge is **not** a Godot replacement and makes no compatibility promises — treat it as an inspectable prototype. See [Maturity boundaries](#maturity-boundaries) for what it deliberately does not do, and the [Roadmap](docs/roadmap.md) for where the loop is heading.

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

Inspect a run, then read the loop's output:

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

- Evidence dashboard: <http://127.0.0.1:8000/examples/evidence-dashboard/>
- Authoring cockpit: <http://127.0.0.1:8000/examples/authoring-cockpit/>
- Runtime demo: <http://127.0.0.1:8000/examples/game-runtime/>

### Prerequisites

Rust + Cargo, Node.js (static UI smoke tests), Python 3 (local static serving), and Chrome/Chromium at a standard path (or set `OUROFORGE_CHROME=/path/to/chrome`). No Playwright, database, cloud service, account system, or hosted runtime required.

### Verification before a PR

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js && node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js && node examples/authoring-cockpit/cockpit.test.cjs
```

Per-milestone evidence steps live in the matching `docs/*.md`; contribution expectations are in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Documentation

| Topic | Doc |
|-------|-----|
| Architecture (the loop, in detail) | [`docs/architecture.md`](docs/architecture.md) |
| Roadmap & completed milestones | [`docs/roadmap.md`](docs/roadmap.md) |
| Evidence fidelity & trust boundary | [`docs/evidence-fidelity-trust-boundary-v1.md`](docs/evidence-fidelity-trust-boundary-v1.md), [`docs/public-alpha-security-trust-boundary-v1.md`](docs/public-alpha-security-trust-boundary-v1.md) |
| Artifact write policy | [`docs/artifact-write-policy-v1.md`](docs/artifact-write-policy-v1.md) |
| Public-readiness gates | [`docs/public-readiness-audit.md`](docs/public-readiness-audit.md), [`docs/public-launch-checklist.md`](docs/public-launch-checklist.md), [`docs/public-alpha-readiness-governance-handoff-v1.md`](docs/public-alpha-readiness-governance-handoff-v1.md) |
| Versioning, changelog, and release artifact policy | [`docs/release-versioning-policy-v1.md`](docs/release-versioning-policy-v1.md), [`docs/release-artifact-policy-v1.md`](docs/release-artifact-policy-v1.md) |
| Public wording guardrail | [`docs/public-wording-guardrail-v1.md`](docs/public-wording-guardrail-v1.md), [`docs/public-wording-audit-process-v1.md`](docs/public-wording-audit-process-v1.md), [`docs/public-wording-scan-pa1.8.1.md`](docs/public-wording-scan-pa1.8.1.md), [`docs/public-wording-scan-pa1.8.2.md`](docs/public-wording-scan-pa1.8.2.md) |

The full set of 90+ milestone contracts and demo-evidence notes lives under [`docs/`](docs/).

## Repository map

- `crates/ouroforge-core` — the trusted core: Seed, run artifacts, ledger/evidence APIs, CDP/browser smoke, Scenario DSL, evaluator, journal, mutation proposals, evolve, project + scene models, dashboard read model.
- `crates/ouroforge-cli` — CLI entrypoints for seed/run/evidence/journal/mutation/dashboard/scene/project commands.
- `seeds/` — MVP Seed examples.
- `examples/game-runtime`, `examples/runtime-probe` — the minimal 2D runtime and probe page.
- `examples/evidence-dashboard`, `examples/authoring-cockpit` — read-only static inspection UIs over exported evidence.
- `examples/*-v1`, `examples/*-v2`, `examples/*-regression` — milestone fixtures, scenario packs, and evidence smokes.
- `docs/` — architecture, roadmap, trust-boundary/evidence contracts, milestone notes, public-readiness audits, and Public Alpha Readiness handoff docs.

## Generated local state

Do not commit generated or local runtime/tool state: `runs/`, `target/`, `examples/evidence-dashboard/dashboard-data.json`, `dashboard-data/`, `.claude/`, `.openchrome/`, `.omc/`, `.omx/`. See [`docs/artifact-write-policy-v1.md`](docs/artifact-write-policy-v1.md) for the trusted-write categories and the generated-output / source-like collision policy.

## Maturity boundaries

Ouroforge currently targets one local reproducible MVP demo. It does **not** provide hosted/cloud execution, authentication/authorization, native packaging or export, plugin systems, marketplace features, visual scripting, browser-side trusted file writes or command bridges, or broad engine compatibility guarantees.

Public release requires the evidence gates in [`docs/public-readiness-audit.md`](docs/public-readiness-audit.md) and [`docs/public-launch-checklist.md`](docs/public-launch-checklist.md), followed by a separate manual visibility decision.

## Contributing & security

- Contribution workflow and review expectations: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Security policy and vulnerability reporting: [`SECURITY.md`](SECURITY.md)
- License: see [`LICENSE`](LICENSE)

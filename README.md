# Ouroforge

**Ouroforge is a local-first, evidence-native prototype for game-authoring loops.**
It turns a declared goal into a local run, captures evidence from the runtime,
records what happened, and proposes the next change without giving agents or
browser surfaces trusted write authority.

The name is *Ouroboros* (the serpent that feeds on its own tail) + *Forge*. The
loop is intentionally inspectable:

> **Seed → Run → Evidence → Evaluation → Journal → Mutation → (back to Seed)**

> **Status:** pre-release private MVP with public-readiness and public-alpha
> launch-governance evidence recorded for a future manual visibility review. It
> runs one reproducible local demo today. Ouroforge is **not** a Godot
> replacement and makes no compatibility promises — treat it as an inspectable
> prototype, not a public launch or support commitment.

## What is Ouroforge?

A **Seed** describes intent and acceptance criteria. Ouroforge runs that intent
locally, captures runtime evidence, renders a deterministic verdict, journals the
result, and records mutation proposals when the run falls short. The long-term
ambition is agentic game authoring where AI can suggest changes, but evidence and
review decide which changes become real.

The current MVP is useful as a reproducible local demo and contract suite for the
loop. It is not a hosted service, production editor, release pipeline, or broad
engine replacement.

## What works today

Current checked-in behavior includes:

- Seed validation and local run execution for `seeds/platformer.yaml`.
- Local project validation and minimal 2D project scaffolding.
- Generated run evidence under `runs/` with ledger, journal, evaluator, mutation,
  comparison, and dashboard read models.
- A minimal browser runtime/probe path driven through local Chrome/Chromium.
- Read-only static dashboard and authoring cockpit surfaces over exported JSON.
- Fixture-backed contracts for scene, asset, tilemap, source-preview, sandbox,
  review, and public-readiness documentation boundaries.
- Source Mutation Preview v1 is complete as inert preview/review/sandbox
  evidence only; source patch apply to the trusted maintainer worktree remains
  unimplemented and forbidden until a separate later governance gate authorizes
  it.
- 3D Capability Gate v1 is complete as bounded local 3D evidence: scene graph,
  camera/projection, mesh/material refs, render smoke, collision/trigger,
  animation, probe/evaluator compatibility, deterministic demo/regression
  fixtures, normalized dashboard read models, and escaped read-only Studio
  inspection. It is not production 3D readiness, broad 3D compatibility, native
  export, plugin runtime, hosted/cloud behavior, or a Godot replacement claim.

Generated run, dashboard, screenshot, sandbox, and local tool artifacts are local
state and stay untracked unless a future issue explicitly scopes a deterministic
fixture.

## Quickstart

### Prerequisites

Install Rust + Cargo, Node.js, Python 3, and Chrome/Chromium at a standard path
(or set `OUROFORGE_CHROME=/path/to/chrome`). No Playwright, database, cloud
service, account system, or hosted runtime is required.

### Run the local checks and demo

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

The run command prints a run directory such as `runs/run-...`. Project-bound runs
add optional project context to `run.json`, ledger, journal, and dashboard export;
legacy runs without `--project` stay compatible. Generated run artifacts are
intentionally git-ignored.

### Inspect evidence from a run

```bash
cargo run -p ouroforge-cli -- evidence list runs/<run-id>
cargo run -p ouroforge-cli -- journal show runs/<run-id>
cargo run -p ouroforge-cli -- mutation list runs/<run-id>
cargo run -p ouroforge-cli -- compare runs/<run-id> runs/<run-id>
```

### Open the read-only demo surfaces

```bash
cargo run -p ouroforge-cli -- dashboard export \
    --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

- Evidence dashboard: <http://127.0.0.1:8000/examples/evidence-dashboard/>
- Authoring cockpit: <http://127.0.0.1:8000/examples/authoring-cockpit/>
- Runtime demo: <http://127.0.0.1:8000/examples/game-runtime/>

The current quickstart command audit is recorded in
[`docs/fresh-clone-onboarding-command-audit-v1.md`](docs/fresh-clone-onboarding-command-audit-v1.md).
For an isolated fresh-clone-style smoke, run
[`scripts/fresh-clone-smoke.sh`](scripts/fresh-clone-smoke.sh) as documented in
[`docs/fresh-clone-smoke-v1.md`](docs/fresh-clone-smoke-v1.md). Troubleshooting
and cleanup guidance lives in
[`docs/fresh-clone-troubleshooting-cleanup-v1.md`](docs/fresh-clone-troubleshooting-cleanup-v1.md).
These notes clarify expected generated state and cleanup boundaries without
changing repository visibility, release status, or trusted-write authority.

## Core loop

Ouroforge's loop is built around evidence over assertion:

1. **Seed** — declare intent and acceptance criteria.
2. **Run** — execute a local runtime/demo path and collect generated artifacts.
3. **Evidence** — capture bounded runtime, browser, project, scenario, and probe
   outputs as inspectable files.
4. **Evaluation** — produce a deterministic verdict from the evidence.
5. **Journal** — summarize what actually happened with evidence references.
6. **Mutation proposal** — record proposed next changes as reviewable data, not
   trusted source writes.
7. **Repeat** — a later reviewed change can become the next seed/run cycle.

The Rust core and local filesystem own trusted state. Agents, browser workers,
and Chrome DevTools Protocol observations are evidence inputs only.

## Demos and examples

- `seeds/platformer.yaml` — the MVP seed used by the local demo.
- `examples/game-runtime/` and `examples/runtime-probe/` — minimal local runtime
  and probe pages.
- `examples/evidence-dashboard/` — read-only evidence dashboard over exported
  dashboard JSON.
- `examples/authoring-cockpit/` — read-only authoring cockpit over generated
  evidence and proposal data.
- `examples/*-v1`, `examples/*-v2`, and `examples/*-regression` — milestone
  fixtures, scenario packs, and evidence smokes.

## Safety model

Ouroforge's current safety boundary is conservative:

- **Trusted authority:** Rust CLI/core code and the local filesystem.
- **Evidence only:** agents, browser workers, and CDP observations can inform
  proposals but cannot apply them.
- **Read-only browser surfaces:** dashboard and cockpit pages render exported
  JSON and copyable commands; they do not write files, run commands, or accept
  source mutations.
- **No command bridge:** browser/UI surfaces do not invoke local commands or
  local server command bridges.
- **No source apply authority:** source-preview, sandbox, stale-target, rollback,
  and review artifacts are evidence/governance boundaries unless a later explicit
  issue authorizes trusted apply.
- **Generated-state isolation:** `runs/`, `target/`, dashboard exports, `.omx/`,
  `.omc/`, `.openchrome/`, `.claude/`, and sandbox outputs remain local ignored
  state.

Security and trust-boundary references:

- [`SECURITY.md`](SECURITY.md)
- [`docs/evidence-fidelity-trust-boundary-v1.md`](docs/evidence-fidelity-trust-boundary-v1.md)
- [`docs/public-alpha-security-trust-boundary-v1.md`](docs/public-alpha-security-trust-boundary-v1.md)
- [`docs/public-alpha-disclosure-and-sandbox-limitations-v1.md`](docs/public-alpha-disclosure-and-sandbox-limitations-v1.md)
- [`docs/artifact-write-policy-v1.md`](docs/artifact-write-policy-v1.md)

## Non-goals and maturity boundaries

Ouroforge does **not** currently provide:

- hosted/cloud execution, accounts, authentication, authorization, or
  multi-tenant behavior;
- production readiness, support/security SLA, compatibility stability, or secure
  sandboxing for arbitrary untrusted content;
- native export, packaging, signing, publishing, deployment, or release
  automation;
- plugin runtime, marketplace, visual scripting, or third-party code-loading
  ecosystem;
- browser trusted writes, local command bridges, auto-apply, auto-merge, or
  reviewer bypass;
- source patch apply to the trusted maintainer worktree.

Public release still requires fresh evidence gates in
[`docs/public-readiness-audit.md`](docs/public-readiness-audit.md),
[`docs/public-launch-checklist.md`](docs/public-launch-checklist.md), and the
manual visibility-decision process. The launch-governance and communication-pack
docs are preparation artifacts, not a visibility toggle or publication event.

## Roadmap

The roadmap and per-milestone completion records — with each milestone's
evidence chain and non-goals — live in [`docs/roadmap.md`](docs/roadmap.md).
Cross-cutting boundaries are in
[Non-goals and maturity boundaries](#non-goals-and-maturity-boundaries); they are
not repeated per milestone here. Earlier completed milestones — including
Safe Source Mutation Apply, the GDD-to-Playable Prototype v1 prototype lane,
the Plugin / Extension System v1 lane, the Full Studio Editor lane, the
Godot-Plus Demo lane, and the Autonomous QA / Playtest Swarm v1 lane — keep
their full evidence chain and per-issue records in
[`docs/roadmap.md`](docs/roadmap.md); only the current Era's snapshot is
summarised below.

**Current state.** Era H (Milestones 42–46) is recorded complete on merged
evidence in [`docs/roadmap.md`](docs/roadmap.md): Multi-Agent Production Pipeline
v1 (M42), Autonomous Producer and Whole-Game Orchestration v1 (M43), Scaled Trust
Gradient / Release Provenance / Compliance v1 (M44), the Shipping and LiveOps
Layer-3 Re-evaluation Design Gate (M45), and the Era H closing autonomy
assessment (M46). The descriptive autonomy posture is in
[`docs/era-h-autonomy-assessment.md`](docs/era-h-autonomy-assessment.md): agents
and local Rust contracts can carry proposal, evidence, orchestration, QA,
provenance, and release-candidate preparation work, but vision, taste/fun, legal
compliance acceptance, and release go/no-go remain human decisions.

**Earlier foundations remain recorded.** Era E established bounded local trust and
Layer-3 DEFER in [`docs/layer3-reevaluation-v1.md`](docs/layer3-reevaluation-v1.md);
Era F/G added genre/function evidence and specialized production gates. The #1 and
#23 anchors are deliberately kept open as ongoing north-star tracks. The full
per-era completion history and evidence chains live in
[`docs/roadmap.md`](docs/roadmap.md) and the matching `docs/*.md` contracts.

**Next.** Era I/J work continues only through bounded, issue-scoped genre and
human-judgment milestones. Shipping/native-store export, hosted/cloud,
real-player telemetry, live balancing, update/patch pipelines, and distributed
Layer-3 behavior remain **DEFER** absent a separate #1508 Layer-3 GO; Rust-first /
local-first is preserved absent that GO.

## Contributor guide

- Contribution workflow and review expectations: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Security policy and vulnerability reporting: [`SECURITY.md`](SECURITY.md)
- License: [`LICENSE`](LICENSE)

Before opening a PR, run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js && node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js && node examples/authoring-cockpit/cockpit.test.cjs
```

Per-milestone evidence steps live in the matching `docs/*.md` files. Keep
generated/local runtime state untracked.

## Documentation map

Use [`docs/README.md`](docs/README.md) as the expanded documentation index. The
README keeps only the most common starting points so public-alpha readers do not
have to scan every milestone contract first.

| Reader question | Start here |
| --- | --- |
| How does the loop work in detail? | [`docs/architecture.md`](docs/architecture.md) |
| What is complete and what is next? | [`docs/roadmap.md`](docs/roadmap.md) |
| What is the trust boundary? | [`docs/README.md#safetytrust-boundaries`](docs/README.md#safetytrust-boundaries) |
| Where are milestone references grouped? | [`docs/README.md`](docs/README.md) |
| What wording is forbidden or risky? | [`docs/public-wording-guardrail-v1.md`](docs/public-wording-guardrail-v1.md), [`docs/public-wording-audit-process-v1.md`](docs/public-wording-audit-process-v1.md) |
| Where is the final docs IA audit? | [`docs/docs-link-wording-audit-pa1.5.3.md`](docs/docs-link-wording-audit-pa1.5.3.md) |

## Repository map

- `crates/ouroforge-core` — trusted core models and evidence APIs for seeds,
  runs, ledgers, browser smoke, scenarios, evaluator, journal, mutation
  proposals, project/scene contracts, source-preview boundaries, and dashboard
  read models.
- `crates/ouroforge-cli` — CLI entrypoints for seed, run, evidence, journal,
  mutation, dashboard, scene, project, source-preview, and related commands.
- `seeds/` — MVP seed examples.
- `examples/` — runtime demos, read-only UIs, fixtures, scenario packs, and
  regression examples.
- `docs/` — architecture, roadmap, trust-boundary/evidence contracts, milestone
  notes, public-readiness audits, and governance handoff docs.

## Generated local state

Do not commit generated or local runtime/tool state: `runs/`, `target/`,
`examples/evidence-dashboard/dashboard-data.json`, `dashboard-data/`, `sandbox/`,
`.claude/`, `.openchrome/`, `.omc/`, `.omx/`. See
[`docs/artifact-write-policy-v1.md`](docs/artifact-write-policy-v1.md) for the
trusted-write categories and generated-output/source-like collision policy.

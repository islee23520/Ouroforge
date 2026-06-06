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

The roadmap and completed milestone references live in
[`docs/roadmap.md`](docs/roadmap.md). Public-alpha readiness, launch governance,
release artifact policy, communication-pack, and post-launch roadmap responses
are documented as separate references rather than launch approval. Public Alpha
Launch Governance v1 is recorded there as complete as a governance/documentation
milestone only: it does not publish an announcement, change repository
visibility, release packages, or approve public-launch execution. Source
Mutation Preview v1 is also recorded there as complete but preview-only; it does
not authorize trusted source apply, branch merge/rebase automation, browser
command bridges, dependency/CI mutation, native export, plugin runtime, public
launch automation, or Godot replacement claims. Production 2D Engine Core v1 is
recorded as complete in `docs/roadmap.md` and
`docs/production-2d-engine-core-v1.md` as a bounded local 2D vertical-slice
evidence milestone; it is not a production-ready, shipped-game, broad
compatibility, secure-sandbox, native-export, plugin-runtime, hosted/cloud,
source-apply, public-launch, or Godot replacement claim. Multi-Agent Production
Pipeline v1 is now recorded as complete in `docs/roadmap.md`,
`docs/multi-agent-production-pipeline-v1.md`, and
`docs/multi-agent-production-pipeline-governance-handoff.md` as a local
evidence-gated collaboration/accountability milestone; it is not hidden-agent
orchestration, cloud execution, auto-apply/auto-merge authority, release
automation, public-launch approval, or production readiness. 3D Capability Gate
v1 is recorded as complete in `docs/roadmap.md`,
`docs/3d-capability-gate-v1.md`, and
`docs/studio-3d-inspection-surface-v1.md` as a bounded local 3D evidence
milestone; it is not production 3D readiness, broad 3D compatibility,
advanced-import/rendering parity, native-export, plugin-runtime, hosted/cloud,
source-apply, public-launch, or Godot replacement scope. Agentic Scene and Level
Designer v1 is recorded as complete in `docs/roadmap.md`,
`docs/agentic-scene-level-designer-v1.md`, and
`docs/agentic-scene-level-designer-governance-handoff.md` as a bounded local
level/scene-authoring evidence milestone; it is not autonomous full game
generation, a production editor, visual scripting, browser trusted writes,
native export, plugin runtime, hosted/cloud behavior, source-apply authority,
public-launch approval, or Godot replacement scope. Gameplay Scripting /
Logic System v1 is now recorded as complete in `docs/roadmap.md`,
`docs/gameplay-scripting-logic-system-v1.md`, `examples/gameplay-logic-demo-v1/`,
and `examples/gameplay-logic-regression-v9/` as a structured data-only behavior
and evidence milestone; it is not arbitrary script execution, a production-stable
scripting API, plugin runtime, browser trusted write path, command bridge,
unrestricted source apply, public-launch approval, or Godot replacement scope.
GDD-to-Playable Prototype v1 is recorded as complete in `docs/roadmap.md`,
`docs/gdd-to-playable-prototype-v1.md`,
`docs/gdd-to-prototype-governance-handoff.md`, the GDD prototype fixture/docs
chain, and `examples/gdd-to-prototype-demo-v1/` as a bounded evidence-gated
prototype path. Autonomous QA / Playtest Swarm v1 is now recorded as complete in
`docs/roadmap.md`, `docs/autonomous-qa-playtest-swarm-v1.md`,
`docs/autonomous-qa-playtest-swarm-governance-handoff.md`, the QA swarm fixture/docs
chain, and `examples/qa-swarm-regression-suite-v13/` as bounded local QA/playtest
evidence and backlog inputs. Safe Source Mutation Apply v1 is recorded as
complete in `docs/roadmap.md`, `docs/safe-source-mutation-apply-v1.md`, and
`docs/safe-source-apply-governance-handoff.md` as review-gated trusted apply for
explicitly allowed low-risk source-like file classes only: validated previews,
sandbox evidence, independent review, stale-target checks, rollback metadata,
allowlisted verification, post-apply evidence, audit ledger entries, and
emergency hold controls remain mandatory. Plugin / Extension System v1 is
recorded as complete in `docs/roadmap.md`, `docs/plugin-extension-system-v1.md`,
`docs/plugin-threat-model-v1.md`, `docs/plugin-extension-system-demo-v1.md`, and
`docs/scenario-coverage-v16-plugin-extension.md` as a bounded declarative,
allowlisted, evidence-backed extension foundation: declarative manifests, local
registry/discovery, an allowlisted extension point catalog, capability/
compatibility validation, descriptor evidence, read-only dashboard/Studio
inspection, a fixture plugin pack, a fail-closed threat-model gate, CLI
inspection, and regression coverage, with plugins declaring metadata only and
never executing code. These milestones do not authorize
autonomous unrestricted game creation, uncontrolled asset generation, hidden
workers, remote/cloud swarm orchestration, unrestricted source/script mutation,
forbidden file-class mutation, dependency/CI/build-script mutation, browser
trusted writes, command bridges, auto-fix, auto-apply, auto-merge, native export,
plugin runtime, hosted/cloud behavior, production readiness, public launch,
secure-sandbox guarantees, autonomous source repair, or a current Godot
replacement claim. With Safe Source Mutation Apply v1, Full Studio Editor v1,
Plugin / Extension System v1, Godot-Plus Demo Game v1, Game Complexity Ladder v1
/ Era E Milestone 24, and End-to-End Provenance Bundle and Audit Surface v1 /
Era E Milestone 25 now recorded as complete, the next recommended governance
milestone is Era E Milestone 26: Era E Refresh and Layer-3 Re-evaluation
Trigger. Engine growth remains demand-driven and rung-justified; Layer-3
distributed orchestration / Elixir per ADR #92 remains deferred for
re-evaluation at Milestone 26.

Loop Coverage Metric v1 / Era E Milestone 20 is recorded in the roadmap as a
descriptive authorship/verification fraction only; at that completion point, the
recommended follow-on was Era E Milestone 21: Second Game Class and Loop
Generalization.

Second Game Class and Loop Generalization v1 / Era E Milestone 21 is now recorded
in the roadmap as complete: the loop generalized to a second game class — the
Signal Gate Platformer — measured by the loop coverage metric, demonstrating
loop-produced generalization rather than broad genre support, production
readiness, or a Godot replacement. At that completion point the recommended
follow-on was Era E Milestone 22: Trust Gradient Design Gate.

Trust Gradient v1 / Era E Milestone 22 is now recorded in the roadmap as complete
after a GO design-gate decision: bounded, reversible, audited, default-off
auto-apply for a narrow scope only (low-risk scene-only-data, high-confidence,
all-gates-pass, in-budget proposals with a one-command rollback handle). It is
not auto-merge, self-approval, source-affecting auto-apply, or a quality/Godot
replacement claim, and autonomy stays opt-in behind a kill switch. At that
completion point the recommended follow-on was Era E Milestone 23:
Multi-Iteration Evolve Campaigns.

Multi-Iteration Evolve Campaigns v1 / Era E Milestone 23 is now recorded in the
roadmap as complete: a campaign is a bounded, audited, multi-iteration sequence
of evolve proposals that stops on a declared acceptance condition or an exhausted
budget, with convergence reported descriptively and a journal narrative over the
iterations. Convergence is descriptive, not a quality or Godot replacement claim;
campaigns respect the Trust Gradient, add no unsupervised unbounded looping, and
grant no auto-merge or reviewer bypass. At that completion point the recommended
follow-on was Era E Milestone 24: Game Complexity Ladder.

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

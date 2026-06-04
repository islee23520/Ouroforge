# Gameplay Scripting / Logic System v1 Scope Contract

Issue: #611 — Gameplay Scripting / Logic System v1 Scope and Contract. This
document is the canonical scope contract for #1 Milestone 10.

Gameplay Scripting / Logic System v1 defines a bounded path for Ouroforge to
represent, validate, review, run, and inspect structured gameplay behavior. It
is safe gameplay logic authoring through data-first artifacts, not arbitrary
executable scripting, a production-stable scripting API, secure sandbox claim,
native export platform, plugin runtime, hosted/cloud product, autonomous launch
system, unrestricted source-apply path, or current Godot replacement claim.

## Purpose

The milestone coordinates follow-up issues for behavior authoring while
preserving the trust, review, evidence, and generated-state boundaries already
used by scene, runtime, scenario, dashboard, Studio/cockpit, 2D, and 3D
contracts. This control document adds no behavior runtime, script runtime,
sandbox execution, Studio behavior UI, plugin loader, command bridge, source
mutation, hosted service, export, or production scripting behavior by itself.

The first implementation path is structured, deterministic gameplay behavior.
Script module and sandbox topics are design gates only until separate governance
issues explicitly authorize any executable runtime.

## Bounded behavior target

The v1 target is structured local gameplay behavior with:

- a source-like behavior model that can reference existing scenes, entities,
  triggers, state, objectives, and bounded 2D/3D runtime evidence;
- deterministic event and signal definitions with explicit producers,
  consumers, payload shapes, ordering, and replay/evidence expectations;
- state-machine and ability/action models for local gameplay logic without
  arbitrary code execution;
- behavior runtime integration that interprets validated data models through
  Rust/local and browser-runtime contracts scoped by follow-up issues;
- behavior-specific scenario assertions and evaluator coverage;
- agent-generated behavior drafts that remain untrusted proposals until Rust
  validation and review gates accept them;
- review-gated behavior apply records with rollback metadata and evidence links
  when a later issue explicitly scopes trusted apply;
- behavior evidence and journal integration for run/result inspection;
- Studio/dashboard/cockpit behavior inspection surfaces that remain escaped,
  read-only or draft-only, and non-authoritative for trusted persistence;
- generated drafts, transactions, run outputs, dashboard exports, screenshots,
  temp projects, browser profiles, and local tool state kept ignored unless a
  later issue explicitly scopes deterministic source-like fixtures.

The target deliberately excludes arbitrary JS/Rust/Python/Lua/WASM execution,
`eval`, dynamic import, command bridges, local server bridges, plugin loading,
marketplaces, remote code loading, hosted services, production-stable scripting
API promises, and broad engine compatibility claims.

## Artifact separation

Follow-up work must keep these concepts separate:

- behavior models describe declarative local gameplay intent and bindings;
- events and signals describe deterministic observations and dispatch contracts;
- state machines describe permitted states, transitions, guards, and effects;
- abilities/actions describe bounded commands that the runtime may interpret;
- script module interfaces describe a future expansion boundary only, not an
  executable runtime;
- sandbox/trust-boundary documents describe threat models and authorization
  requirements only, not a secure-sandbox guarantee;
- drafts are untrusted candidate behavior changes;
- review decisions accept, reject, or block drafts without applying them by
  themselves;
- apply records are trusted Rust/local outcomes only when a follow-up issue
  explicitly scopes review-gated behavior apply;
- evidence and journals report what happened and do not create write authority.

No browser, Studio surface, dashboard, generated draft, agent output, or script
module artifact is trusted persistence until the scoped Rust/local boundary
validates it and the applicable review gate accepts it.

## Dependency order

Follow-up issues should proceed in this order unless a later live issue audit
proves a safer dependency order:

1. #612 — Gameplay Behavior Model v1.
2. #613 — Event and Signal System v1.
3. #614 — State Machine and Ability Action Model v1.
4. #615 — Script Module Interface Design Gate v1.
5. #616 — Safe Script Sandbox and Trust Boundary v1.
6. #617 — Behavior Runtime Integration v1.
7. #618 — Behavior Test and Scenario Assertion v1.
8. #619 — Agent-Generated Behavior Draft v1.
9. #620 — Review-Gated Behavior Apply v1.
10. #621 — Behavior Evidence and Journal Integration v1.
11. #622 — Studio Behavior Inspection Surface v1.
12. #623 — Gameplay Logic Demo v1.
13. #624 — Scenario Coverage v9: Gameplay Logic Regression Suite.
14. #625 — Roadmap and #1 Governance Refresh after Gameplay Scripting / Logic
    System v1.

Each issue should use the smallest safe PR unit with focused tests, generated-
state audit, no-arbitrary-script audit, compatibility audit, and conservative
wording audit. Do not combine behavior model, events, state machines, runtime,
drafts, apply, evidence, Studio, demo, and regression behavior when independent
verification would be clearer.

## Trusted boundary

- Rust/local code owns trusted validation, persistence, behavior draft/apply
  validation, generated evidence artifact writing, project/run binding,
  source-like fixture validation, and CLI behavior.
- Browser runtime code may interpret already-validated local behavior data only
  when a follow-up issue scopes the runtime behavior. It does not gain trusted
  filesystem persistence, shell command execution, dynamic code import, source
  mutation authority, or local server bridge authority.
- Dashboard, Studio, and cockpit surfaces display exported evidence and may
  prepare draft-only copyable data when explicitly scoped. They are not trusted
  writers and must not contain hidden command bridges, local command execution,
  `eval`, dynamic import, plugin loading, auto-apply, auto-merge, self-approval,
  unrestricted source mutation, publish, export, or credentialed/network/install
  behavior.
- Generated behavior drafts, transactions, review/apply records, runs,
  dashboard data, screenshots, browser profiles, temp projects, and local tool
  state stay untracked unless a follow-up issue explicitly scopes a tiny
  deterministic source-like fixture.

## Compatibility expectations

Gameplay logic work must be additive. It must preserve existing Seeds, scenes,
project manifests, runs, scenarios, dashboard exports, Studio read models,
2D/3D fixtures, behavior contracts, source-like fixtures, and existing runtime
probe/read-model shapes unless a PR includes an explicit migration note and
focused compatibility tests.

Existing scene, component, trigger, runtime, scenario, evidence, journal,
dashboard, and cockpit models should be extended before adding parallel systems.
New schemas should prefer additive versioned fields and explicit read-model
branches over ambiguous shape changes.

## Script expansion boundaries

Script module work is design-gate only in #615. It may define interface shapes,
capabilities, permissions, import/export boundaries, fixture metadata, and
future authorization questions, but it must not execute scripts, load plugins,
introduce dynamic imports, fetch remote code, or treat generated script text as
trusted behavior.

Sandbox/trust-boundary work is design-gate only in #616. It may define threat
models, blocked operations, evidence requirements, review gates, kill switches,
and later authorization prerequisites, but it must not claim secure sandboxing or
add an executable sandbox runtime.

Any executable script runtime requires a separate explicit governance issue with
its own threat model, test matrix, generated-state policy, rollback story,
review-gate story, and conservative public wording.

## Verification and closure gates

Every Gameplay Scripting / Logic System follow-up issue should include:

- live issue checks for the current issue, #1, and #23;
- focused Rust tests or Node smokes for new schemas, read models, fixtures,
  runtime behavior, dashboard/cockpit display, browser-local behavior, or docs
  assertions touched by the issue;
- compatibility checks for touched Seeds, scenes, project manifests, runs,
  scenarios, dashboard exports, Studio read models, 2D/3D fixtures, source-like
  fixtures, and runtime probe/read-model contracts;
- generated-state audit showing only ignored local/runtime outputs are present;
- no-arbitrary-script audit: no `eval`, dynamic import, plugin loader, command
  bridge, local server bridge, hidden command execution, browser trusted write,
  auto-apply, auto-merge, self-approval, or unrestricted source mutation;
- conservative wording audit: no production-stable scripting API, current Godot
  replacement, production-ready, secure-sandbox, native export, plugin runtime,
  hosted/cloud, or autonomous launch claims;
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
malformed state, XSS escaping, and no-command/no-write coverage. Trusted
Rust/local issues should add invalid, stale, missing-evidence, generated-state,
no-arbitrary-script, and review-gate tests.

## Non-goals for Milestone 10

Milestone 10 does not authorize:

- arbitrary JS/Rust/Python/Lua/WASM script execution;
- `eval`, dynamic import, remote code loading, plugin loading, extension loader,
  marketplace, or dynamic code installation;
- browser trusted writes, browser command bridges, local server command bridges,
  hidden command execution, credentialed/network/install command behavior,
  auto-apply, auto-merge, self-approval, or reviewer bypass;
- unrestricted source mutation apply or source-changing behavior without scoped
  review and sandbox gates;
- production-stable scripting API, broad compatibility-stable engine API,
  secure-sandbox guarantee, production-ready engine, shipped-game maturity, or
  current Godot replacement claim;
- native export, platform packaging, signing, notarization, or release/publish
  automation;
- hosted/cloud/server/auth/account behavior, remote asset/code hosting,
  collaboration infrastructure, or production CI/CD automation.

## Governance anchors

- #1 remains the roadmap/final-goal anchor and must stay open unless a separate
  explicit governance decision changes it.
- #23 remains the memory/governance anchor and must stay open unless a separate
  explicit governance decision changes it.
- This document may be revised only by an explicit follow-up governance issue or
  roadmap refresh. Implementation issues should cite it rather than weakening
  its boundaries locally.

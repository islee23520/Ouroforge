# Build and Release Candidate Agent Lane Design Gate v1

Status: **design gate only**. This document defines a future local-first
build/release-candidate agent lane for the Multi-Agent Production Pipeline v1.
It does not implement build, export, package, publish, signing, deployment,
release automation, CI/CD automation, native export, hosted demo, or visibility
change behavior, and it does not execute commands.

## Purpose

The build/release-candidate agent lane is an accountability lane for collecting
local readiness evidence and blockers before a human maintainer decides whether
a future separately authorized release-candidate activity may proceed. It is not
a release system and it is not a production launch gate.

The lane may produce recommendation artifacts such as:

- a release-candidate readiness checklist;
- a build prerequisite inventory;
- smoke-test and regression evidence references;
- generated-state hygiene notes;
- unresolved blocker summaries;
- explicit governance handoff records for human review.

All artifacts are advisory evidence. They do not publish, package, sign, upload,
merge, approve, or alter public visibility.

## Allowed design responsibilities

A future build/release-candidate agent may be allowed to inspect already-trusted
local evidence and prepare bounded advisory artifacts:

1. Collect build prerequisites
   - list required local toolchain versions as observed evidence;
   - link project manifest, scenario pack, asset manifest, and generated run
     refs;
   - identify missing prerequisites without installing them.
2. Prepare a release-candidate checklist
   - summarize required checks and known blockers;
   - require exact commit, PR, issue, and generated evidence refs;
   - distinguish docs-only/local evidence from publishable artifacts.
3. Gather smoke evidence references
   - link existing cargo, node, dashboard, cockpit, scenario, QA, and regression
     outputs;
   - mark stale/missing/malformed refs instead of repairing them.
4. Identify blockers
   - report failed checks, missing reviews, unresolved ownership conflicts,
     generated-state leaks, wording drift, and release-policy gaps;
   - preserve blocked status until a human or later scoped issue resolves it.
5. Produce recommendation artifacts
   - recommend `hold`, `needs-review`, or `ready-for-human-release-review`;
   - include rationale, alternatives rejected, confidence, scope risk, evidence
     refs, and explicit non-authority language.

## Forbidden actions

This design gate forbids the lane from performing or enabling:

- publishing packages, binaries, hosted demos, marketplace artifacts, or public
  launch messages;
- release automation, production CI/CD automation, deployment automation, or
  workflow mutation;
- signing, notarization, registry upload, package upload, installer creation, or
  public visibility changes;
- native export, platform packaging, desktop/mobile/web distribution, or
  Godot-replacement packaging claims unless a later explicit governance issue
  authorizes the exact scope;
- credentialed commands, registry-token use, network/install commands,
  dependency mutation, CI/workflow/build-script mutation, dynamic code loading,
  plugin loading, or arbitrary script execution;
- browser trusted writes, browser command bridges, local server command bridges,
  hidden command execution, hidden background agents, unbounded spawning,
  remote worker pools, hosted/cloud/server orchestration, or account systems;
- auto-apply, auto-merge, self-approval, reviewer bypass, hidden promotion, or
  unrestricted source mutation;
- production-ready, shipped-game, commercial-readiness, secure-sandbox,
  compatibility-stability, or current Godot replacement claims.

## Required evidence

A recommendation artifact must link evidence instead of asserting readiness from
memory. Required evidence includes:

- issue and PR refs for the scoped work;
- exact commit or branch head under review;
- `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets
  --all-features -- -D warnings` results where Rust contracts are involved;
- dashboard and authoring cockpit Node syntax/smoke checks where display or
  demo evidence is involved;
- QA/playtest and performance/regression lane refs when present;
- production evidence bundle and decision ledger refs when present;
- generated-state audit showing that runs, dashboard exports, temp projects,
  local tool state, and generated bundles remain ignored unless explicitly
  fixture-scoped;
- conservative public wording audit covering no production readiness, no
  autonomous arbitrary game completion, no hidden agents, no release automation,
  and no Godot replacement claim;
- protected governance check that issues #1 and #23 remain open unless a
  separate explicit governance decision authorizes changing them.

Missing, malformed, stale, or conflicting evidence must produce a hold/blocker
recommendation. The lane must not repair evidence, install tools, mutate trusted
state, or invent missing verification.

## Recommendation states

| State | Meaning | Required next action |
| --- | --- | --- |
| `hold` | Required evidence is missing, stale, malformed, failed, or unsafe. | Keep work blocked and record blockers. |
| `needs-review` | Evidence is present but requires human governance/release review. | Human maintainer reviews scope and policy. |
| `ready-for-human-release-review` | Local evidence appears complete for a human decision. | Human maintainer may open a separate release-governance issue. |

No state authorizes publishing, signing, upload, CI/CD mutation, native export,
public visibility changes, or release automation.

## Governance handoff

Before any future release-candidate action can exist, a separate issue must
explicitly authorize that action and define:

- artifact class and audience;
- exact manual or trusted local authority;
- required credentials and how they remain outside agent access;
- verification, rollback, and unpublish/hold criteria;
- public wording boundaries;
- #1/#23 governance status.

Until such an issue exists, this lane remains advisory and read-only. It may
link `docs/release-artifact-policy-v1.md` and
`docs/release-versioning-policy-v1.md`, but it does not supersede them.

## Generated-state and display boundary

Generated build/release-candidate recommendation artifacts must live under local
generated roots such as `runs/multi-agent-pipeline` unless a future fixture is
explicitly scoped. Dashboard, cockpit, and Studio surfaces may display escaped
read-only summaries only. They must not execute commands, spawn agents, write
trusted browser state, bridge to local commands, repair stale evidence,
auto-apply, auto-merge, self-approve, release, publish, sign, upload, deploy, or
change visibility.

## Definition of done for this design gate

This issue is complete when this document exists with conservative wording and a
focused audit verifies that it remains design-only, references required evidence
and governance handoff, forbids release/publish/signing/export/CI/CD/credential
boundaries, preserves generated-state policy, and keeps issues #1 and #23 open.

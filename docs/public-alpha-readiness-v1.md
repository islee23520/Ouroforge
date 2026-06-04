# Public Alpha Readiness v1 Scope and Contract

Status: **scope contract** for issue #367 PA1.1.1.

Public Alpha Readiness v1 follows Source Mutation Preview v1 and prepares
Ouroforge for a possible future public/open-source alpha review. It is a
preparation milestone only: it tightens onboarding, demo reproducibility,
public-facing documentation, evidence references, security/trust-boundary policy,
contribution templates, wording guardrails, versioning policy, readiness gates,
and roadmap governance. It does not change repository visibility, publish
artifacts, automate releases, approve launch messaging, or implement product
behavior.

## Why this milestone follows Source Mutation Preview v1

Source Mutation Preview v1 established a conservative boundary for source-like
change proposals: preview, review, sandbox, stale-target, and evidence artifacts
may exist, but trusted source apply and browser command authority remain blocked.
That makes the next safe step governance and public-readiness hardening rather
than more write authority.

Public Alpha Readiness v1 therefore focuses on whether a fresh reader can:

1. understand the local evidence-native loop;
2. clone the repository and run bounded local checks;
3. inspect generated evidence without tracking local state;
4. find the security/trust-boundary and contribution policies;
5. distinguish public-readiness preparation from actual public launch.

## Completed baseline entering the milestone

This milestone starts from these completed local-first evidence surfaces:

| Baseline area | Representative references | Boundary |
| --- | --- | --- |
| Visual Authoring v1 | `docs/visual-authoring-v1.md`, `docs/visual-edit-draft-model-v1.md`, `docs/visual-authoring-v1-governance-handoff.md` | Safe local edit-draft cockpit, review-gated evidence, no browser trusted writes. |
| Asset Pipeline v1 | `docs/asset-pipeline-v1.md`, `docs/asset-manifest-v1.md`, `docs/asset-pipeline-v1-governance-handoff.md` | Local asset manifests and evidence, no remote marketplace or plugin runtime. |
| Source Mutation Preview v1 | `docs/source-mutation-preview-v1.md`, `docs/source-mutation-threat-model-v1.md`, `docs/patch-preview-artifact-v1.md` | Inert preview/review/sandbox evidence only; no trusted source apply. |
| Agentic Loop Orchestration v1 | `docs/agentic-loop-orchestration-v1.md`, `docs/authoring-loop-plan-v1.md`, `docs/agent-handoff-contract-v1.md` | Data-only plans and CLI-owned execution boundaries. |
| Review and Regression | `docs/review-decision-ledger-v1.md`, `docs/regression-run-matrix-v1.md`, `docs/studio-review-cockpit-v1.md` | Evidence-linked review decisions, no auto-merge or reviewer bypass. |
| Engine Expressiveness v2 | `docs/engine-expressiveness-v2.md`, `docs/playable-demo-v2-collect-and-exit.md`, `docs/scenario-coverage-v4-asset-pipeline.md` | Bounded local demo/fixtures, not broad engine compatibility or Godot replacement. |

These baselines are MVP contracts and evidence references, not production
readiness, compatibility stability, secure-sandbox, native-export, plugin-runtime,
or hosted-service claims.

## Dependency order

Public Alpha Readiness v1 work should proceed in this dependency order. Later
items may cite earlier evidence, but they must remain bounded to preparation,
docs, scripts, templates, checks, and governance unless a later issue explicitly
authorizes more.

| Order | Area | Primary issue | Expected artifact boundary |
| --- | --- | --- | --- |
| 1 | Milestone scope contract | #367 | This document: scope, baseline, dependency order, boundaries, verification, closure gates. |
| 2 | Fresh clone onboarding | #368 | README/docs audit, fresh-clone smoke, troubleshooting, cleanup, generated-state policy. |
| 3 | Canonical demo script | #369 | Non-destructive local demo command sequence and smoke evidence. |
| 4 | Public demo evidence refresh | #370-#371 | Demo screenshots/evidence references and generated-state policy; no tracked local outputs unless fixture-scoped. |
| 5 | Security/trust boundary policy | #372 | Responsible disclosure, browser read-only boundary, no source apply, no secure-sandbox guarantee. |
| 6 | Contribution and issue templates | #373-#374 | Conservative contributor guidance, issue templates, public intake boundaries. |
| 7 | README/docs IA v2 | #375 | Navigation and information architecture for public-alpha readers. |
| 8 | Readiness gate and final report | #376 | Checklist/report format and execution evidence for prepared/blocked/deferred outcome. |
| 9 | Wording guardrails | #377 | Forbidden claim scan, replacement guidance, and public wording audit process. |
| 10 | Release/versioning policy | release/versioning docs | Version labels and artifact policy without publication automation. |
| 11 | Roadmap and #1 refresh | #387 or later governance refresh | Preserve #1 as open roadmap anchor unless a separate explicit governance decision exists. |

If implementation findings require moving scope across these areas, record the
blocker and proposed boundary change in the affected issue before opening or
merging changed work.

## Public-readiness versus public-launch boundary

Public readiness means repository-local evidence and policies are prepared for a
future manual maintainer review. It does not mean public launch.

This milestone does not authorize or automate:

- repository visibility changes or GitHub settings mutation;
- package, binary, crates.io, npm, signing, upload, deployment, or release
  publication;
- launch announcements, go-live automation, or public communication publication;
- production editor behavior, hosted/cloud/server/auth behavior, or support SLA;
- native export, plugin runtime, marketplace, third-party code loading, or Godot
  replacement claims;
- source patch apply, browser trusted writes, command bridges, auto-apply,
  auto-merge, hidden command execution, or reviewer bypass;
- secure-sandbox, compatibility-stable, production-ready, or security-guarantee
  claims.

A future readiness report may say the repository is
`prepared-for-manual-review`; that is still only a signal for a separate human
visibility decision outside the PR that records it.

## Verification policy

Every PR in this milestone should record live issue state and local checks
appropriate to its scope. The minimum broad command set is:

```bash
gh issue view <current-issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short --ignored
```

Issue-specific checks may add `cargo audit`, Node dashboard/cockpit checks,
wording scans, link checks, generated-state audits, or smoke scripts when the
changed files touch those surfaces.

## Closure gates

A Public Alpha Readiness v1 issue is ready to close only when:

1. all fixed PR units for that issue are merged in their required order;
2. latest `main` has been pulled;
3. issue-level verification passes on latest `main`;
4. guardrails, drift-prevention, over-engineering, generated-state policy, and
   #1/#23 state are checked;
5. a final issue comment records merged PRs, verification evidence, known gaps,
   and closure rationale;
6. #1 and #23 remain open unless a separate explicit governance decision exists.

## Over-engineering and drift checks

All answers must remain **No** for this scope issue and its readiness follow-ups:

- Did this work implement product behavior rather than readiness artifacts?
- Did this work change repository visibility or GitHub settings?
- Did this work publish, release, upload, sign, announce, or automate launch?
- Did this work add browser trusted writes, command bridges, source apply,
  auto-merge, auto-apply, hidden command execution, hosted services, or account
  systems?
- Did this work claim production readiness, compatibility stability, secure
  sandboxing, Godot replacement status, native export, plugin runtime, or support
  SLA?
- Did this work track generated runs, dashboards, screenshots, temp projects, or
  local tool state outside explicit fixtures?
- Did this work close, replace, or weaken #1 or #23 governance anchors?

## Relationship to #1 and #23

- #1 remains the open evidence-native implementation roadmap anchor.
- #23 remains the open memory/governance anchor.
- Public Alpha Readiness v1 may reference those issues, but it must not close,
  replace, relabel, or modify them unless a separate explicit governance decision
  authorizes that action.

## Definition of done for #367

- This canonical scope contract exists.
- The completed baseline is summarized.
- Dependency order is explicit.
- Public-readiness versus public-launch boundary is explicit.
- Verification policy and closure gates are defined.
- No product implementation, release automation, publishing, or visibility change
  was added.
- #1 and #23 remain open.

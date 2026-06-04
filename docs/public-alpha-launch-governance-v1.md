# Public Alpha Launch Governance v1

Status: **scope contract only** for issue #378.

Public Alpha Launch Governance v1 follows Public Alpha Readiness v1. Readiness
preparation gathered evidence that a future public review could be considered;
this milestone defines the governance needed to decide whether, when, and how a
future public-alpha transition may happen. It does not launch Ouroforge, change
repository visibility, publish packages, announce a release, or implement product
behavior.

## Manual launch boundary

All launch actions remain external manual maintainer decisions. In particular,
this milestone does not authorize or automate:

- repository visibility changes or GitHub settings mutation;
- launch announcements or public communication publication;
- release, package, binary, crates.io, npm, signing, or upload actions;
- production CI/CD, deployment, hosted/cloud/server/auth behavior, or support
  operations;
- browser trusted writes, command bridges, local server bridges, source apply,
  auto-merge, auto-apply, or hidden command execution;
- production-ready, compatibility-stable, secure-sandbox, Godot replacement,
  native export, plugin runtime, marketplace, or support-SLA claims.

A future `go` record means maintainers may consider a separate manual GitHub
settings review. It is not executable authority.

## Why governance follows readiness

Public Alpha Readiness v1 prepared artifacts such as license/security policy,
README and roadmap hardening, public issue templates, demo evidence, smoke/audit
output, and launch-checklist reconciliation. Those artifacts reduce ambiguity,
but they do not answer operational questions such as:

- who records a go/no-go decision;
- how blockers are held or rolled back;
- how public issues and PRs are triaged without overpromising support;
- how security reports are handled before broad visibility;
- how demo stability is monitored without hosted/service guarantees;
- how public communication stays conservative;
- how post-launch roadmap changes remain tied to #1 governance.

This scope issue coordinates those governance follow-ups while keeping launch
execution outside the repository change set.

## Dependency order

Follow-up work should proceed in this dependency order. Later items may cite
previous docs, but they must remain bounded to governance/docs/templates,
playbooks, or checklists unless a later issue explicitly authorizes more.

| Order | Area | Primary issue | Expected artifact boundary |
| --- | --- | --- | --- |
| 1 | Milestone scope contract | #378 | This document: scope, dependency order, launch boundary, verification, closure gates. |
| 2 | Visibility decision record | #379 | Go/no-go/hold template, review fields, examples, and manual visibility boundary. |
| 3 | Hold and rollback criteria | #380 | [`public-alpha-launch-hold-criteria-v1.md`](public-alpha-launch-hold-criteria-v1.md) and [`public-alpha-launch-response-rollback-v1.md`](public-alpha-launch-response-rollback-v1.md): blockers, manual response/rollback options, deferral, and evidence needed before reconsideration. |
| 4 | Public issue intake | #381 | Issue triage policy/snippets/templates for public-alpha requests without support guarantees. |
| 5 | Public PR intake | #382 | PR review/intake policy, maintainer expectations, and forbidden-scope handling. |
| 6 | Security response | Security playbook docs | Vulnerability-reporting flow, escalation boundaries, and no public security guarantee overclaim. |
| 7 | Demo stability monitoring | Demo monitoring docs | Manual/CI-adjacent evidence policy, generated-state cleanup, Chrome caveats, and refresh cadence. |
| 8 | Public alpha communication pack | #385 | Conservative announcement/README/FAQ wording drafts; no actual publication. |
| 9 | Post-launch roadmap triage | Roadmap triage docs | Triage rules for public feedback, rejected claims, and follow-up issue boundaries. |
| 10 | Roadmap and #1 refresh | #387 | Governance refresh that preserves #1 as the open roadmap anchor unless separately authorized. |

If implementation findings require moving scope across these areas, record the
blocker and proposed boundary change in the relevant issue before opening or
merging changed work.

## Verification policy

Every PR under this milestone should at minimum verify:

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

Issue-specific documentation checks should also confirm:

- the artifact remains governance/docs/templates/playbooks/checklists only;
- repository visibility and GitHub settings remain unchanged;
- no launch, release, package publication, or public communication publication is
  automated;
- no generated local state is tracked unless explicitly fixture-scoped;
- public wording stays conservative;
- #1 and #23 remain open.

## Closure gates

A Public Alpha Launch Governance v1 issue is ready to close only when:

1. all fixed PR units for that issue are merged in their required order;
2. latest `main` has been pulled;
3. issue-level verification passes on latest `main`;
4. guardrails, drift-prevention, over-engineering, generated-state policy,
   launch-governance boundary, and #1/#23 state are checked;
5. a final issue comment records merged PRs, verification evidence, known gaps,
   and closure rationale;
6. #1 and #23 remain open unless a separate explicit governance decision exists.

## Over-engineering and drift checks

All answers must remain **No** for this scope issue and its governance follow-ups:

- Did this work implement product behavior rather than governance artifacts?
- Did this work change repository visibility or GitHub settings?
- Did this work publish, release, upload, sign, announce, or automate launch?
- Did this work add browser trusted writes, command bridges, source apply,
  auto-merge, auto-apply, hidden command execution, hosted services, or account
  systems?
- Did this work claim production readiness, compatibility stability, secure
  sandboxing, Godot replacement status, native export, plugin runtime, or support
  SLA?
- Did this work track generated runs, dashboards, screenshots, launch reports,
  temp projects, or local tool state outside explicit fixtures?
- Did this work close, replace, or weaken #1 or #23 governance anchors?

## Relationship to #1 and #23

- #1 remains the open evidence-native implementation roadmap anchor.
- #23 remains the open memory/governance anchor.
- Public Alpha Launch Governance v1 may reference those issues, but it must not
  close, replace, relabel, or modify them unless a separate explicit governance
  decision authorizes that action.

## Definition of done for #378

- This canonical scope contract exists.
- Dependency order is explicit.
- Launch-governance boundary is explicit.
- Verification policy and closure gates are defined.
- No product implementation, release automation, publishing, or visibility change
  was added.
- #1 and #23 remain open.

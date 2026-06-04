# Post-launch roadmap triage v1

Status: **governance-only taxonomy**. This document routes post-launch
feedback into roadmap buckets without changing repository visibility, publishing
a release, implementing product features, or weakening the design-gate-first
policy for risky scopes.

Use this taxonomy when an issue, PR, discussion, demo finding, or maintainer
note asks for work after public-alpha readiness. It is an intake aid, not an
automatic acceptance path. Every accepted request still needs a narrowly scoped
issue, explicit non-goals, verification evidence, and generated-state audit.

## Triage principles

1. **Evidence first:** requests must cite a run artifact, journal, scenario,
   screenshot, doc gap, user report, or explicit maintainer decision.
2. **Smallest safe bucket:** route work to the most conservative roadmap bucket
   that can answer the request; prefer docs, fixtures, and evidence over new
   authority or automation.
3. **Design-gate-first:** risky surfaces require a design gate before any
   implementation PR. This includes source mutation apply, native export,
   plugins, distributed QA, hosted/cloud behavior, trusted browser writes,
   command bridges, and security-sensitive execution.
4. **No maturity drift:** do not accept wording that implies production
   readiness, compatibility stability, secure sandboxing, support SLAs, native
   export, plugin runtime, source apply, or Godot replacement status.
5. **Generated state stays local:** `runs/`, `target/`, `.openchrome/`, `.omc/`,
   `.omx/`, `.claude/`, dashboard exports, screenshots, and local tool output
   remain untracked unless a separate issue scopes a tiny deterministic fixture.
6. **Protected anchors remain open:** #1 is the broad roadmap anchor and #23 is
   the repository memory/design-context anchor until a separate explicit
   governance decision says otherwise.

## Roadmap buckets

| Bucket | Route these requests here | Acceptance criteria | Reject or move when |
| --- | --- | --- | --- |
| Source Mutation Apply Design Gate | Requests to apply, merge, write, or commit source-like patches from proposals or previews. | A design issue defines file classes, sandbox/worktree boundary, threat model, rollback/audit evidence, stale-target protection, review gates, and trusted Rust/local authority. | The request asks for direct apply, browser writes, command execution, auto-merge, arbitrary patch application, or implementation before a design gate. |
| Native Export Design Gate | Requests for packaging, desktop builds, native launchers, export presets, or distributable game/editor bundles. | A design issue defines target platforms, reproducibility evidence, artifact provenance, signing/non-signing policy, generated output policy, and explicit non-promises. | The request implies shipping binaries, app-store readiness, support SLAs, compatibility guarantees, or release automation. |
| Plugin Design Gate | Requests for extensions, marketplaces, third-party code loading, scripting APIs, or user-provided runtime modules. | A design issue defines capability boundaries, trust model, manifest/schema evidence, isolation limits, review policy, and no-marketplace/no-secure-sandbox wording. | The request assumes a plugin runtime, arbitrary untrusted execution, marketplace, browser command bridge, or security guarantee. |
| Distributed QA Design Gate | Requests for remote workers, hosted test runs, multi-machine orchestration, queues, or shared dashboards. | A design issue defines local-vs-remote authority, data retention, auth/security non-goals, reproducibility evidence, failure reporting, and offline fallback. | The request requires cloud hosting, accounts, external production services, secrets, CI mutation, or operational guarantees. |
| Visual Authoring v2 | Requests to improve local edit-draft authoring after Visual Authoring v1. | The work remains draft/preview/review-gated, uses Rust-owned trusted validation, and keeps browser surfaces read-only or draft-only. | The request needs trusted browser writes, visual scripting, source apply, project mutation outside scoped Rust commands, or production-editor claims. |
| Asset Pipeline v2 | Requests for richer local asset metadata, references, atlas/tilemap workflows, or preview/read-model evidence. | The work stays local-first, source-like asset metadata is validated by Rust, and runtime/dashboard surfaces expose evidence without remote hosting or marketplace scope. | The request requires browser uploads as trusted state, remote asset hosting, third-party marketplaces, native packaging, or broad compatibility promises. |
| Engine Expressiveness v3 | Requests for new local runtime/gameplay expressiveness, scenario coverage, or demo mechanics. | The request names bounded components, deterministic scenario evidence, fixtures, and dashboard/cockpit read models. | The request claims production engine parity, broad Godot compatibility, plugin runtime, native export, or unbounded feature breadth. |
| Public Docs v2 | Requests for README, roadmap, public-readiness, issue template, demo evidence, or communication improvements. | The change is documentation/template/checklist only, preserves conservative wording, and does not automate launch, release, repository visibility, or publication. | The request asks to publish, announce, change visibility, promise support, weaken maturity boundaries, or close #1/#23. |

## Intake decision criteria

Use these outcomes for each request:

- **Accept into an existing bucket** when the request has evidence, is narrowly
  scoped, fits one bucket, states explicit non-goals, and has local verification.
- **Request clarification** when evidence, affected files, user value, non-goals,
  or verification commands are missing.
- **Move to a design gate** when the request touches authority, persistence,
  source mutation, execution, external services, security, native packaging, or
  third-party code loading.
- **Reject for current roadmap** when the request depends on production-readiness
  claims, compatibility promises, hosted/cloud guarantees, support SLAs, launch
  automation, or untrusted code execution.
- **Defer as generated/local state** when the only artifact is an unreviewed run,
  screenshot, dashboard export, local tool output, or machine-local path.

## Label and template mapping

Recommended issue labels remain descriptive, not automatic approval:

- `governance`: design gates, roadmap decisions, launch/manual-decision policy.
- `documentation`: roadmap, README, templates, public-readiness evidence, docs
  gaps, and response snippets.
- `enhancement`: bounded implementation work after the matching design gate or
  milestone issue exists.
- `public-readiness`: public-readiness remediation, launch checklist evidence,
  conservative communication, and manual visibility-decision support.
- `security`: vulnerability-reporting policy, threat models, trust-boundary
  reviews, and security-sensitive design-gate blockers.

Feature requests should use `.github/ISSUE_TEMPLATE/feature_request.yml` and
name the roadmap bucket, evidence, explicit non-goals, and verification commands.
Public readiness work should use `.github/ISSUE_TEMPLATE/public_readiness.yml`
and state that the task does not publish, announce, or change visibility.

## Closure audit for triage-only PRs

Before closing a governance-only triage issue, verify:

```bash
gh issue view <issue> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
grep -RInE "Godot replacement|production-ready|compatibility-stable|secure sandbox|native export ready|plugin runtime ready|source apply ready|support SLA" README.md docs .github || true
git ls-files runs target .openchrome .omc .omx .claude examples/evidence-dashboard/dashboard-data.json
```

The grep command is an audit prompt, not a blanket failure: any matches must be
reviewed for conservative negation or explicit non-goal wording. The `git
ls-files` command should print no generated local-state paths unless a separate
fixture-scoped issue authorized them.

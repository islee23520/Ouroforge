# Public issue intake triage v1

Status: **governance-only intake taxonomy**. This document defines how public
alpha issues should be categorized, labeled, and described before any maintainer
accepts, converts, closes, or escalates them. It does not configure GitHub
labels, mutate repository settings, change repository visibility, publish a
release, automate launch actions, or implement product behavior.

Use this playbook for public GitHub issues during the public-alpha governance
period. It complements `docs/post-launch-roadmap-triage-v1.md`,
`docs/public-launch-checklist.md`, and `docs/security-response-playbook-v1.md`.
When documents overlap, use the most conservative route and keep #1 and #23 open
unless a separate explicit governance decision says otherwise.

## Intake principles

1. **Classify before committing:** every issue needs a public category, evidence
   basis, requested outcome, and explicit non-goals before implementation work is
   accepted.
2. **Labeling is descriptive:** labels communicate routing and risk; they do not
   approve work, create support coverage, or promise response times.
3. **Prefer documentation and evidence:** public-alpha intake should preserve the
   evidence-native loop with docs, templates, checklists, fixtures, and local
   verification before new behavior.
4. **Security and forbidden scope get conservative handling:** move sensitive
   details out of public issues and reject or redirect requests that require
   unsupported authority.
5. **No maturity drift:** do not imply production readiness, compatibility
   stability, secure sandboxing, native export, plugin runtime, source apply,
   Godot replacement status, or support SLAs.
6. **Generated state stays local:** `runs/`, `target/`, `.openchrome/`, `.omc/`,
   `.omx/`, `.claude/`, dashboard exports, screenshots, and local tool output
   remain untracked unless a separate fixture-scoped issue explicitly authorizes
   them.

## Public intake categories

| Category | Use when the issue reports or asks for | Default labels | Required triage fields | First routing decision |
| --- | --- | --- | --- | --- |
| Bug | A reproducible local MVP failure, regression, panic, wrong verdict, broken CLI/demo path, or documented evidence mismatch. | `bug`, `needs-triage` | Reproduction steps, expected behavior, actual behavior, commit/branch, environment, evidence artifact, generated-state status. | Confirm reproducibility or ask for missing evidence before accepting a fix. |
| Docs | Missing, stale, confusing, or overclaiming documentation, README, roadmap, checklist, template, or communication wording. | `documentation`, `needs-triage` | Affected document, proposed correction, conservative wording check, verification/audit command, explicit non-goals. | Accept as docs-only when wording stays conservative and no launch/release action is implied. |
| Demo/onboarding | Public demo, walkthrough, smoke path, authoring cockpit, dashboard, or setup friction that affects a local first-run experience. | `public-readiness`, `documentation`, `needs-triage` | Demo path, command or walkthrough step, evidence link, expected user outcome, generated artifacts, manual decision boundary. | Route to public-readiness remediation; do not publish, host, or change visibility. |
| Security | Suspected vulnerability, trusted boundary bypass, secret/local-path leak, dependency risk, unsafe generated artifact, or public security wording issue. | `security`, `needs-triage` | Public-safe summary, affected boundary, sensitivity of details, affected commit, private coordination status, sanitized evidence status. | Follow `docs/security-response-playbook-v1.md`; keep exploit details, secrets, and private artifacts out of public issues. |
| Feature request | A bounded product, CLI, runtime, scenario, dashboard, authoring, engine, asset, or workflow enhancement. | `enhancement`, `needs-triage` | User problem, smallest proposed scope, roadmap bucket, evidence, explicit non-goals, local verification. | Route through `docs/post-launch-roadmap-triage-v1.md`; design-gate buckets do not authorize implementation. |
| Forbidden scope request | A request for unsupported authority or claims: repository visibility change, launch/release automation, package publication, hosted/cloud/auth, marketplace/plugin runtime, native export shipping, browser trusted writes, command bridge, source apply, auto-merge/apply, production support, or Godot-replacement positioning. | `governance`, `needs-triage` | Requested forbidden capability, policy conflict, safer alternative, public wording risk, affected guardrail. | Reject, redirect to a governance/design-gate discussion, or close when no safe alpha path exists. |
| Roadmap/governance | A decision about roadmap order, public-alpha readiness, go/no-go, hold/rollback, issue/PR intake, label policy, or protected anchors #1/#23. | `governance`, `public-readiness`, `needs-triage` | Decision needed, affected anchors/issues, evidence, manual decision owner, non-goals, verification/audit path. | Keep as governance-only unless a separate implementation issue is created. |
| Support question | A question about setup, local commands, examples, evidence artifacts, expected MVP limits, or how to report an issue. | `question`, `needs-triage` | User goal, command/output context, environment, docs already checked, privacy/generated-state status. | Answer or redirect to docs without creating support SLAs or ongoing support commitments. |

## Recommended label set

These labels are recommended repository vocabulary only. This PR does not create,
rename, delete, or configure labels in GitHub.

| Label | Meaning | Notes |
| --- | --- | --- |
| `needs-triage` | Initial public issue has not yet been classified, de-duplicated, or evidence-checked. | Remove only after category, evidence, and next action are recorded. |
| `bug` | Reproducible local MVP defect or regression. | Requires reproduction and evidence before implementation acceptance. |
| `documentation` | Docs, README, roadmap, checklist, template, or wording-only change. | Must preserve conservative public-alpha wording. |
| `public-readiness` | Manual public-alpha readiness, demo evidence, launch checklist, or communication preparation. | Does not authorize visibility changes or publication. |
| `security` | Suspected security report or security-sensitive boundary/wording issue. | Use public-safe summaries and private coordination when needed. |
| `enhancement` | Bounded feature or behavior request. | Must map to a roadmap bucket or design gate. |
| `governance` | Roadmap, go/no-go, intake policy, protected anchors, or guardrail decision. | Governance-only unless another issue scopes implementation. |
| `question` | Setup, usage, evidence interpretation, or expectation-setting question. | Answering does not create support coverage or response guarantees. |
| `needs-info` | Reporter must add required fields before routing can continue. | Use instead of accepting ambiguous work. |
| `blocked` | Work is blocked by missing evidence, guardrail conflict, dependency order, or maintainer decision. | Record the blocker and safe next step. |
| `duplicate` | Issue duplicates an existing public issue. | Link the canonical issue before closing. |
| `wontfix` | Request is intentionally not planned for the current alpha scope. | Use with a conservative rationale and safer alternatives when possible. |
| `generated-state` | Issue depends on generated/local artifacts that should not be committed as source. | Ask for a source-like summary or fixture-scoped follow-up if needed. |

## Minimum triage fields

Every public issue should record these fields in the issue body or first
maintainer triage comment:

- **Category:** one of bug, docs, demo/onboarding, security, feature request,
  forbidden scope request, roadmap/governance, or support question.
- **Evidence:** command output, run id, journal/verdict excerpt, screenshot,
  affected doc section, user report, or explicit maintainer decision.
- **Affected surface:** CLI, runtime, scenario, dashboard, authoring cockpit,
  docs/template, public-readiness checklist, security boundary, roadmap, or
  external request.
- **Requested outcome:** fix, clarify, document, route to design gate, answer,
  close, convert to PR, or defer.
- **Explicit non-goals:** what the issue must not change or promise.
- **Generated-state status:** whether any evidence is local/generated and must
  remain untracked.
- **Verification path:** the smallest local commands, docs audit, wording scan,
  or artifact check that can prove the requested outcome.
- **Protected-anchor check:** whether the issue affects #1 or #23; they remain
  open unless a separate explicit governance decision says otherwise.

Security reports should use only public-safe versions of these fields and move
sensitive reproduction details, secrets, local paths, private screenshots, and
exploit detail to the private coordination path in
`docs/security-response-playbook-v1.md`.

## Template alignment

Public intake should start with the closest existing template:

- Use `.github/ISSUE_TEMPLATE/bug_report.yml` for reproducible defects and local
  regression evidence.
- Use `.github/ISSUE_TEMPLATE/feature_request.yml` for bounded enhancement ideas
  routed through the roadmap bucket model.
- Use `.github/ISSUE_TEMPLATE/public_readiness.yml` for public-alpha governance,
  demo/onboarding, communication, checklist, and template/documentation work.

If a report does not fit a template, maintainers should add the minimum triage
fields above in a comment before accepting work. This policy intentionally does
not add GitHub automation, issue forms that hide existing templates, labeler
rules, issue actions, or repository settings.

## Docs/template audit for PLG1.4.1

Before merging taxonomy-only changes, verify that the changed docs/templates:

- define all eight intake categories;
- list labels as recommendations, not configured automation;
- include required triage fields;
- preserve no-launch, no-visibility-change, no-publication, no-support-SLA, and
  generated-state boundaries;
- keep security handling aligned with `docs/security-response-playbook-v1.md`;
- keep #1 and #23 open.

Suggested audit commands:

```bash
gh issue view 381 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
grep -RInE "production-ready|compatibility-stable|secure sandbox|native export ready|plugin runtime ready|source apply ready|Godot replacement|support SLA" docs .github README.md || true
git ls-files runs target .openchrome .omc .omx .claude examples/evidence-dashboard/dashboard-data.json
```

The wording scan is an audit prompt: conservative negations and explicit
non-goals are acceptable, but any positive maturity or support promise must be
removed before merge. The generated-state command should print no tracked local
state unless a separate fixture-scoped issue authorized it.

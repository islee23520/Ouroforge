# Public Alpha Launch Response and Rollback Criteria v1

Status: **manual response and rollback playbook** for issue #380 / PLG1.3.2.

This document defines how maintainers should respond if a future manual public
alpha visibility decision creates issues. It does not change repository
visibility, publish announcements, publish packages, mutate GitHub settings,
execute rollback, lock issues, create advisories, or automate communication.
All actions below are manual maintainer decisions that require fresh evidence at
the time of use.

## Scope boundary

Use this playbook after one of these events:

- a manual public-visibility decision record has been created;
- public issue/PR intake reveals a launch-governance blocker;
- security, demo stability, generated-state, wording, or source-apply ambiguity
  appears after a public-alpha review;
- maintainers need a conservative response path before reconsidering visibility,
  communication, or roadmap posture.

The playbook is governance-only. It must not be connected to GitHub settings,
release workflows, browser controls, source mutation apply, auto-merge,
auto-apply, command execution, package publication, hosted/cloud/server/auth
flows, or public support guarantees.

## Response severity levels

| Severity | Use when | Manual response target | Escalation owner |
| --- | --- | --- | --- |
| `response-critical` | Secret/private data exposure, high-severity vulnerability, misleading security/support guarantee, source-apply/command-execution ambiguity, or generated private artifact reaches public view. | Stop public communication, preserve evidence privately, select visibility/notice/security path, and require maintainer/security owner signoff. | Maintainer plus security owner. |
| `response-major` | Fresh-clone/demo/readiness gate fails publicly, public docs overclaim maturity, issue/PR intake causes unsafe contributor behavior, or generated-state policy is confusing. | Add or update a public notice if appropriate, file blocker issue, and hold further public-alpha promotion until fixed. | Maintainer plus docs/demo owner. |
| `response-minor` | Typo, stale link, unclear wording, or low-risk doc mismatch that does not change trust boundaries or launch authority. | File a scoped follow-up and correct through normal PR review. | Docs owner. |
| `response-defer` | Evidence is incomplete, timing is unsuitable, or the report cannot be validated safely. | Record deferral reason and keep launch posture unchanged until evidence is fresh. | Maintainer. |

Severity labels are triage aids only; they do not execute repository actions.

## Manual response criteria

| Area | Trigger | Manual response options | Evidence required before action |
| --- | --- | --- | --- |
| Visibility posture | Public visibility appears to expose sensitive data, unsafe claims, or untriaged security risk. | Keep visibility unchanged if still private; if already public, maintainers may manually consider temporary private visibility, issue locking, or a README notice outside this PR. | `gh issue view 1`, `gh issue view 23`, offending path/issue/PR, visibility decision record, and security owner note when applicable. |
| README or docs notice | Readers need a conservative boundary statement while a blocker is remediated. | Add a manual docs PR with a dated notice that states the blocker, non-goals, and expected remediation path without promising support or launch timing. | Blocker issue, wording scan, docs diff, generated-state audit, and reviewer signoff. |
| Issue intake pressure | New public issues contain secrets, private paths, exploit detail, or requests for forbidden scope. | Triage with existing templates/snippets, minimize or lock only through manual maintainer action, and redirect security details to `SECURITY.md`. | Issue links, redaction/minimization decision, security-report path, and public response text. |
| PR intake pressure | Public PRs attempt dependency changes, source apply, release automation, visibility changes, browser command bridges, or broad product claims without design approval. | Request changes or close with a conservative response; require a design issue before implementation. | PR link, changed-file audit, dependency/security rationale, and guardrail checklist. |
| Security advisory path | A validated vulnerability or secret exposure requires private coordination. | Follow `SECURITY.md` and security playbook; create an advisory or private coordination channel only by manual maintainer decision. | Report id, affected versions/paths, impact, remediation owner, and disclosure timeline decision. |
| Demo regression | Public demo instructions fail after visibility review. | Add a blocker issue, update demo evidence or troubleshooting docs, and avoid claims that the demo is stable until checks pass again. | Failing command/output, checkout SHA, generated-state cleanup, and rerun evidence. |
| Wording overclaim | Public docs or templates imply production readiness, compatibility stability, secure sandboxing, Godot replacement status, native export readiness, plugin runtime readiness, source apply readiness, support SLA, or launch approval. | Correct wording through PR; if already public and material, add a notice or issue comment clarifying the boundary. | File/line, replacement wording, wording scan, reviewer signoff. |
| #1/#23 governance drift | #1 or #23 is closed, replaced, or modified without explicit governance decision. | Restore or reopen if appropriate, or record a separate governance decision before further launch work. | `gh issue view 1`, `gh issue view 23`, audit of the change, and decision link. |

## Rollback option matrix

Rollback means choosing a conservative manual posture. It does **not** mean this
repo contains executable rollback automation.

| Option | When appropriate | Manual actions | Not authorized by this document |
| --- | --- | --- | --- |
| `visibility-review-hold` | The repository has not been made public, or maintainers have not completed a visibility decision. | Keep visibility unchanged; record blockers; rerun hold criteria after fixes. | Any automated settings mutation or public announcement. |
| `communication-retract-or-amend` | A draft or public-facing statement overclaims maturity, support, security, compatibility, or launch status. | Update/retract the communication manually; add correction wording; link the blocker. | Automated posting, deletion, or external account operations. |
| `readme-notice` | A short in-repo notice helps prevent reader confusion while a blocker is fixed. | Open a docs PR with dated notice and removal criteria. | Support SLA, security guarantee, or promise of fix timing. |
| `issue-lock-or-minimize` | A public issue exposes secrets, exploit detail, harassment, or unsafe private data. | Maintainers may manually lock/minimize/redact according to GitHub policy and `SECURITY.md`. | Bot-driven moderation or hidden deletion by this codebase. |
| `security-advisory-path` | A validated security issue needs coordinated disclosure. | Follow `SECURITY.md` and maintainer security workflow. | Automatic advisory creation, publication, or CVE claims. |
| `blocker-fix-and-rerun` | A scoped docs/test/template problem can be remediated normally. | File/assign blocker, merge fix PR, rerun verification, update final evidence. | Treating a failed gate as launch-approved. |
| `roadmap-defer` | The issue reveals that public-alpha timing or scope is not ready. | Record deferral and next candidate milestone without closing #1/#23. | Replacing the roadmap anchor or accepting broad future scope automatically. |

## Response record template

```markdown
# Public Alpha Response / Rollback Record

- Date:
- Checkout SHA or decision record:
- Reviewer:
- Severity: <response-critical | response-major | response-minor | response-defer>
- Selected option: <visibility-review-hold | communication-retract-or-amend | readme-notice | issue-lock-or-minimize | security-advisory-path | blocker-fix-and-rerun | roadmap-defer>
- Repository visibility changed by this record: no
- GitHub settings mutated by this record: no
- Release/package/publication automation added: no
- Product behavior added: no
- Public launch approved: no
- #1 open: <yes/no with issue-view evidence>
- #23 open: <yes/no with issue-view evidence>

## Trigger

Link the issue, PR, report, docs path, command output, or decision record that
triggered response consideration. Do not include secrets, private paths, exploit
details, or private screenshots in public comments.

## Evidence

| Evidence | Result | Notes |
| --- | --- | --- |
| `gh issue view 1 --repo shaun0927/Ouroforge` |  |  |
| `gh issue view 23 --repo shaun0927/Ouroforge` |  |  |
| Wording/generated-state/security/demo check |  |  |

## Manual action and removal criteria

State the manual action chosen, who owns it, what evidence removes the hold, and
which verification commands must pass before public-alpha review resumes.
```

## Reconsideration gate

Before maintainers resume a public-alpha visibility review after any response or
rollback option, they should:

1. pull latest `main` or use a clean fresh clone;
2. verify #1 and #23 remain open;
3. rerun the relevant hold criteria from
   [`public-alpha-launch-hold-criteria-v1.md`](public-alpha-launch-hold-criteria-v1.md);
4. rerun broad verification from
   [`public-alpha-readiness-gate-v1.md`](public-alpha-readiness-gate-v1.md) when
   code, templates, demo evidence, or security posture changed;
5. record whether the blocker is fixed, deferred, or still held;
6. restate that actual repository visibility changes remain separate manual
   maintainer actions outside repository automation.

## Non-goals

This playbook does not automate repository visibility changes, GitHub settings,
issue locking, security advisories, announcements, releases, package publishing,
rollback execution, source patch apply, merge/rebase behavior, browser command
execution, hosted operations, support commitments, or production/security
claims.

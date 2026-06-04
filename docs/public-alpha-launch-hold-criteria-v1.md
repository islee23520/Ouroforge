# Public Alpha Launch Hold Criteria v1

Status: **manual launch-governance hold checklist** for issue #380 / PLG1.3.1.

This document defines conditions that must hold a future public-alpha visibility
review. It does not launch Ouroforge, change repository visibility, publish an
announcement, publish packages, mutate GitHub settings, or automate rollback.
All launch and rollback actions remain manual maintainer decisions outside this
PR and outside issue #380.

## Scope boundary

Use this checklist before a dated visibility decision record or public-alpha
communication draft is treated as ready for maintainer review. A hold outcome
means: keep visibility unchanged, keep the launch decision pending or deferred,
record the blocker evidence, and rerun the relevant verification after a scoped
fix merges.

This checklist is governance-only. It must not be wired into GitHub settings,
branch protection, release workflows, browser command surfaces, source apply,
auto-merge, auto-apply, package publication, or hosted/cloud/server/auth flows.

## Hold outcome vocabulary

| Outcome | Meaning | Allowed next action |
| --- | --- | --- |
| `hold-critical` | A blocker creates security, trust-boundary, generated-state, or launch-authority risk. | Stop public-alpha review, file or link the blocker, and require maintainer acknowledgement before another review. |
| `hold-remediate` | Required evidence is missing, stale, failed, or ambiguous but does not expose immediate sensitive data. | Keep visibility unchanged and remediate through a scoped issue/PR. |
| `defer` | Review timing, environment, or ownership is not ready. | Record why the decision is deferred and schedule a later evidence refresh. |
| `clear-for-manual-review` | No hold criteria apply and required evidence is fresh. | Maintainers may separately consider a manual visibility decision record. |

`clear-for-manual-review` is not a launch approval. It only means this hold
checklist did not find a blocker for the separate manual decision.

## Required hold criteria

| Area | Hold when | Severity | Evidence to record | Minimum owner/check cadence |
| --- | --- | --- | --- | --- |
| Fresh-clone smoke | README quickstart, canonical demo smoke, dashboard smoke, or cockpit smoke fails on a clean latest-main checkout without an environment caveat. | `hold-remediate` | Failing command, output excerpt, checkout SHA, OS/tool versions, and whether retry/fresh clone reproduced it. | Maintainer running visibility review; rerun after the fix PR merges. |
| High-severity audit finding | `cargo audit`, dependency review, security review, or manual trust-boundary audit reports a high-severity vulnerability or exploitable public-intake path. | `hold-critical` | Advisory/finding id, affected package or path, impact summary, and selected remediation or deferral decision. | Security owner or maintainer; re-check before any renewed visibility decision. |
| Generated secret or private artifact | A tracked file contains secrets, tokens, private paths, private issue links, internal screenshots, generated run output, launch reports, or local tool state outside explicit fixtures. | `hold-critical` | Path, commit/SHA range, redaction/removal plan, generated-state audit command, and whether disclosure follow-up is required. | Maintainer plus security owner; re-check after cleanup and before public communication. |
| Forbidden public wording | Public-facing docs/templates claim production readiness, compatibility stability, Godot replacement status, secure sandboxing, native export readiness, plugin runtime readiness, source apply readiness, support SLA, public visibility changed, launch approval, or autonomous launch/release automation. | `hold-remediate` or `hold-critical` when security/support guarantees are implied. | File/line, offending wording, replacement wording, and wording scan command. | Documentation owner; scan every launch-governance PR and final review. |
| Missing or stale security policy | `SECURITY.md`, security-report template, or vulnerability-reporting path is missing, stale, contradictory, or invites public exploit details. | `hold-critical` | Missing/stale section, reporter-safe replacement path, and public issue template audit. | Security owner; review before visibility decision and after template changes. |
| Source apply ambiguity | README, docs, templates, browser surfaces, or PR/issue intake imply trusted source patch apply, merge, rollback execution, browser command bridge, or hidden command execution is currently available. | `hold-critical` | File/line or UI surface, ambiguity summary, and no-apply/no-command-bridge replacement wording. | Maintainer plus source-mutation reviewer; scan every related PR. |
| Browser command bridge ambiguity | Dashboard, cockpit, templates, or docs imply browser trusted writes, local server command execution, upload, install, merge, or release controls. | `hold-critical` | Surface/path, screenshot-free text evidence, and corrected read-only wording or test evidence. | Browser/docs owner; verify with Node smoke tests and wording scan. |
| Demo breakage | MVP demo evidence, screenshots, smoke fixtures, or public demo instructions are stale enough that a reader cannot reproduce the documented local demo path. | `hold-remediate` | Broken step, expected vs actual output, whether generated artifacts were cleaned, and updated demo evidence plan. | Demo owner; rerun after demo-doc or fixture refresh. |
| Issue/PR intake gap | Public issue or PR templates omit generated-state, forbidden-scope, dependency-approval, security-reporting, or no-launch/no-publication boundaries. | `hold-remediate` | Template path, missing field, expected intake path, and focused template audit. | Maintainer reviewing public intake; audit before final communication pack. |
| #1/#23 governance drift | #1 or #23 is closed, replaced, relabeled, or modified without separate explicit governance authorization. | `hold-critical` | `gh issue view 1`, `gh issue view 23`, change summary, and restoration/governance decision link. | Maintainer; check before every closure and visibility decision. |

## Hold check procedure

1. Pull latest `main` or use a clean fresh clone for the intended visibility
   review date.
2. Confirm #1 and #23 are open:

   ```bash
   gh issue view 1 --repo shaun0927/Ouroforge
   gh issue view 23 --repo shaun0927/Ouroforge
   ```

3. Run the broad public-alpha verification set from
   [`public-alpha-readiness-gate-v1.md`](public-alpha-readiness-gate-v1.md) or
   record why the review is deferred.
4. Run a focused wording scan over changed public-facing docs/templates. Treat
   conservative negations and non-goal statements as allowed, but record any
   positive claim or ambiguous launch/source-apply/browser-command wording as a
   hold.
5. Run a generated-state audit:

   ```bash
   git status --short --ignored
   git ls-files runs target .openchrome .omc .omx .claude \
     examples/evidence-dashboard/dashboard-data.json
   ```

6. Record one outcome from the vocabulary above, the evidence commands, and the
   required remediation owner before continuing to any visibility decision record
   or communication pack.

## Hold record template

```markdown
# Public Alpha Launch Hold Record

- Date:
- Checkout SHA:
- Reviewer:
- Outcome: <hold-critical | hold-remediate | defer | clear-for-manual-review>
- #1 open: <yes/no with `gh issue view 1` evidence>
- #23 open: <yes/no with `gh issue view 23` evidence>
- Repository visibility changed: no
- Release/package/publication automation added: no
- Product behavior added: no
- Public launch approved: no

## Findings

| Area | Severity | Evidence | Required next action | Owner |
| --- | --- | --- | --- | --- |
|  |  |  |  |  |

## Closure or deferral rationale

State why the review is held, deferred, or clear for a separate manual maintainer
decision. If clear, restate that the actual visibility change remains outside
this record and outside repository automation.
```

## Non-goals

This hold checklist does not implement product features, source patch apply,
rollback execution, release automation, publication workflows, repository setting
mutation, browser command execution, hosted operations, support guarantees, or a
public launch announcement.

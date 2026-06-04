# Public Visibility Decision Record v1

Status: **template only** for a future manual maintainer decision.

This document defines the required decision-record shape for deciding whether the
repository is ready to become public. It does not change repository visibility,
create a launch announcement, publish a release, mutate GitHub settings, or add
any automation that can perform those actions.

## Record location and review boundary

Decision records should be written as dated Markdown files under
`docs/public-visibility-decisions/` when a maintainer is ready to evaluate a
specific visibility window. The directory may remain absent until an actual
manual review is scheduled.

Review rules:

- A decision record is evidence for maintainer review only.
- The final repository visibility toggle is a manual GitHub settings action
  outside this issue and outside any generated decision record.
- A `go` decision means maintainers may consider the manual settings change; it
  is not an automatic launch, release, package publication, or support promise.
- A `no-go` or `hold` decision keeps the repository private until blockers are
  resolved and a new record is reviewed.
- The record must be reviewed against `docs/public-launch-checklist.md`,
  `SECURITY.md`, current README/docs wording, generated-state policy, and the
  live state of #1 and #23.

## Maintainer review process

Use this process when a dated decision record is scheduled. The process is
review-only and does not automate repository visibility, release, package, or
announcement actions.

1. **Prepare evidence on latest `main`.** Pull latest `main`, verify #1 and #23
   are open, and run the current public-readiness commands from
   `docs/public-launch-checklist.md`.
2. **Draft a dated record.** Copy the template into
   `docs/public-visibility-decisions/public-visibility-YYYY-MM-DD.md` only when
   maintainers are evaluating a real visibility window. Use `pending` instead of
   guessing missing evidence. Use pending instead of guessing.
3. **Run wording and generated-state audits.** Confirm no generated local state
   is tracked and no production-ready, compatibility-stable, secure sandbox,
   Godot replacement, native export, plugin runtime, source apply, support SLA,
   launch automation, or release automation claim was added.
4. **Review blockers.** Any `fail`, `partial`, or `pending` critical field must
   map to a blocker, owner, and required evidence command before a `go` decision
   can be recorded.
5. **Record maintainer approval.** A named maintainer records `approved`,
   `not approved`, or `pending` in the decision record. Approval is evidence for
   a human settings review, not an executable permission.
6. **Keep the manual boundary explicit.** If the decision is `go`, maintainers
   may separately decide whether to perform the GitHub settings visibility
   change manually. The record itself does not perform or schedule that change.
7. **Post-review follow-up.** If visibility is changed manually later, record
   the external action in a separate governance note or issue comment; do not
   retrofit the decision record into launch automation.

See [`docs/public-visibility-decision-examples-v1.md`](public-visibility-decision-examples-v1.md)
for non-executable example records.

## Required fields

Every decision record must include all fields below.

| Field | Required content |
| --- | --- |
| `record_id` | Stable identifier, recommended format `public-visibility-YYYY-MM-DD`. |
| `recorded_at` | UTC timestamp for the review record. |
| `reviewer` | Maintainer or reviewer accountable for the evidence summary. |
| `readiness_gate_result` | `pass`, `fail`, or `partial`, with command/evidence links. |
| `known_risks` | Remaining risks, caveats, environment limits, and public-alpha constraints. |
| `non_goals` | Explicitly restate forbidden launch/release/product claims. |
| `security_contact_status` | Status of `SECURITY.md` and vulnerability-reporting path. |
| `docs_status` | README, roadmap, public demo, issue template, and wording-scan status. |
| `generated_state_status` | Confirmation that generated runs, dashboards, screenshots, temp projects, and local tool state remain untracked unless fixture-scoped. |
| `maintainer_approval` | Named maintainer approval state: `approved`, `not approved`, or `pending`. |
| `go_no_go_decision` | `go`, `no-go`, or `hold`; must include rationale. |
| `blockers` | Blocking issues or `none`; include owner and next evidence command when known. |
| `manual_visibility_boundary` | Required statement that any visibility toggle is manual and external to this record. |
| `issue_governance` | Live state of #1 and #23, which must remain open unless separately authorized. |

## Decision record template

Copy this section into a dated record when a review is scheduled. Leave unknown
items as `pending`; do not infer evidence.

```markdown
# Public Visibility Decision — YYYY-MM-DD

- record_id: public-visibility-YYYY-MM-DD
- recorded_at: YYYY-MM-DDTHH:MM:SSZ
- reviewer: <maintainer/reviewer>
- readiness_gate_result: <pass|fail|partial>
- go_no_go_decision: <go|no-go|hold>

## Manual visibility boundary

Repository visibility remains unchanged by this record. Any visibility toggle is
a manual maintainer action in GitHub settings, external to this document, issue,
and pull request. This record does not publish a release, package, announcement,
or support guarantee.

## Readiness gate evidence

- Commands run:
  - `gh issue view <governance-issue> --repo shaun0927/Ouroforge`
  - `gh issue view 1 --repo shaun0927/Ouroforge`
  - `gh issue view 23 --repo shaun0927/Ouroforge`
  - `<fresh clone / README / demo / audit commands as applicable>`
- Result: <pass|fail|partial>
- Evidence refs: <links or artifact paths>

## Known risks

- <risk/caveat>

## Non-goals and forbidden claims

This decision does not authorize production-ready, compatibility-stable, secure sandbox,
Godot replacement, native export, plugin runtime, source apply,
marketplace, hosted/cloud/auth, support SLA, launch automation, release
automation, package publication, browser trusted writes, command bridge,
auto-merge, auto-apply, or hidden command execution claims.

## Security contact status

- `SECURITY.md` status: <current/pending/blocker>
- Vulnerability-reporting path: <summary>
- Blockers: <none|list>

## Docs status

- README status: <current/pending/blocker>
- Roadmap status: <current/pending/blocker>
- Public demo evidence status: <current/pending/blocker>
- Issue/PR intake status: <current/pending/blocker>
- Conservative wording scan: <pass|fail|partial>

## Generated-state status

Generated demo, run, dashboard, screenshot, launch report, temp project, and
local tool artifacts remain untracked unless explicitly fixture-scoped.

- `git status --short --ignored`: <summary>
- Tracked generated artifact exceptions: <none|list>

## Maintainer approval

- Approval state: <approved|not approved|pending>
- Approver: <name or pending>
- Approval evidence: <link or note>

## Blockers

| Blocker | Owner | Required evidence to clear |
| --- | --- | --- |
| <none|blocker> | <owner> | <command/link> |

## Issue governance

- #1 state: <OPEN required unless separately authorized>
- #23 state: <OPEN required unless separately authorized>
```

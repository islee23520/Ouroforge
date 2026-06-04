# Public Visibility Decision Examples v1

Status: **non-executable examples only**.

These examples illustrate how to fill out
`docs/public-visibility-decision-record-v1.md`. They are not real launch
decisions, do not change repository visibility, do not approve publication, and
do not create release, package, announcement, support, or GitHub settings
automation. These examples are not actual go/no-go decisions.

## Example A — no-go / blocker present

```markdown
# Public Visibility Decision — 2026-06-04 example no-go

- record_id: public-visibility-2026-06-04-example-no-go
- recorded_at: 2026-06-04T00:00:00Z
- reviewer: example-maintainer
- readiness_gate_result: fail
- go_no_go_decision: no-go

## Manual visibility boundary

Repository visibility remains unchanged by this record. Any visibility toggle is
a manual maintainer action in GitHub settings, external to this document, issue,
and pull request. This record does not publish a release, package, announcement,
or support guarantee.

## Readiness gate evidence

- Commands run:
  - `gh issue view 1 --repo shaun0927/Ouroforge`
  - `gh issue view 23 --repo shaun0927/Ouroforge`
  - `cargo audit`
  - `git status --short --ignored`
- Result: fail
- Evidence refs: local review notes show a missing security-contact update.

## Known risks

- SECURITY.md contact path is stale.
- README wording still needs a conservative public-alpha refresh.

## Non-goals and forbidden claims

This decision does not authorize production-ready, compatibility-stable, secure sandbox,
Godot replacement, native export, plugin runtime, source apply,
marketplace, hosted/cloud/auth, support SLA, launch automation, release
automation, package publication, browser trusted writes, command bridge,
auto-merge, auto-apply, or hidden command execution claims.

## Security contact status

- `SECURITY.md` status: blocker
- Vulnerability-reporting path: stale contact alias must be replaced.
- Blockers: SECURITY-contact-refresh

## Docs status

- README status: pending
- Roadmap status: current
- Public demo evidence status: pending
- Issue/PR intake status: current
- Conservative wording scan: partial

## Generated-state status

Generated demo, run, dashboard, screenshot, launch report, temp project, and
local tool artifacts remain untracked unless explicitly fixture-scoped.

- `git status --short --ignored`: only ignored local roots observed
- Tracked generated artifact exceptions: none

## Maintainer approval

- Approval state: not approved
- Approver: example-maintainer
- Approval evidence: no-go review note

## Blockers

| Blocker | Owner | Required evidence to clear |
| --- | --- | --- |
| SECURITY-contact-refresh | maintainer | Updated `SECURITY.md` and rerun wording/security audit |

## Issue governance

- #1 state: OPEN
- #23 state: OPEN
```

## Example B — go / manual action still external

```markdown
# Public Visibility Decision — 2026-06-04 example go

- record_id: public-visibility-2026-06-04-example-go
- recorded_at: 2026-06-04T00:00:00Z
- reviewer: example-maintainer
- readiness_gate_result: pass
- go_no_go_decision: go

## Manual visibility boundary

Repository visibility remains unchanged by this record. Any visibility toggle is
a manual maintainer action in GitHub settings, external to this document, issue,
and pull request. This record does not publish a release, package, announcement,
or support guarantee.

## Readiness gate evidence

- Commands run:
  - `gh issue view 1 --repo shaun0927/Ouroforge`
  - `gh issue view 23 --repo shaun0927/Ouroforge`
  - `cargo fmt --check`
  - `cargo test`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo audit`
  - `node examples/evidence-dashboard/dashboard.test.cjs`
  - `node examples/authoring-cockpit/cockpit.test.cjs`
  - `git status --short --ignored`
- Result: pass
- Evidence refs: dated maintainer log or issue comment.

## Known risks

- Public alpha remains pre-release and locally operated.
- No support SLA, compatibility promise, or production readiness is claimed.

## Non-goals and forbidden claims

This decision does not authorize production-ready, compatibility-stable, secure sandbox,
Godot replacement, native export, plugin runtime, source apply,
marketplace, hosted/cloud/auth, support SLA, launch automation, release
automation, package publication, browser trusted writes, command bridge,
auto-merge, auto-apply, or hidden command execution claims.

## Security contact status

- `SECURITY.md` status: current
- Vulnerability-reporting path: maintainer-reviewed contact instructions present
- Blockers: none

## Docs status

- README status: current
- Roadmap status: current
- Public demo evidence status: current
- Issue/PR intake status: current
- Conservative wording scan: pass

## Generated-state status

Generated demo, run, dashboard, screenshot, launch report, temp project, and
local tool artifacts remain untracked unless explicitly fixture-scoped.

- `git status --short --ignored`: only ignored local roots observed
- Tracked generated artifact exceptions: none

## Maintainer approval

- Approval state: approved
- Approver: example-maintainer
- Approval evidence: maintainer review note

## Blockers

| Blocker | Owner | Required evidence to clear |
| --- | --- | --- |
| none | n/a | n/a |

## Issue governance

- #1 state: OPEN
- #23 state: OPEN
```

## Example review notes

- A `go` example is still not a launch action. It only says the evidence record
  supports a separate manual maintainer settings review.
- A `no-go` example must preserve blockers rather than weakening wording or
  bypassing verification.
- Neither example should be copied into `docs/public-visibility-decisions/` as a
  real decision without replacing all example values with live evidence.

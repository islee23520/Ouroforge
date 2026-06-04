# Public PR Intake Policy v1

Status: **public-alpha pull-request checklist and forbidden-scope gate** for issue #382 / PLG1.5.1.

This policy defines how maintainers should triage public-alpha pull requests
before review. It is governance documentation only. It does not change branch
protection, repository visibility, GitHub settings, merge rules, release
workflows, package publication, source apply, browser command execution, or any
product behavior.

## Intake boundary

A public-alpha PR may be reviewed only when it names a linked issue or accepted
maintenance scope, lists the exact intended files, and stays within conservative
public-alpha boundaries. If scope is unclear, maintainers should hold review and
ask for the smallest issue-backed slice instead of inferring intent.

This policy is a checklist, not automation. Maintainers still make manual review
and merge decisions through normal GitHub review.

## Initial PR intake checklist

Use this checklist before a public-alpha PR is treated as review-ready:

| Check | Required evidence | Pass condition | Hold / reject when |
| --- | --- | --- | --- |
| Linked scope | PR body names a linked issue, PR unit, roadmap bucket, or maintenance reason. | Scope is concrete and matches changed files. | No issue/scope, broad roadmap claim, or work crosses issue boundaries without explanation. |
| Drift lock | PR body lists current issue/slice, expected files, authorized behavior, explicit non-goals, generated artifacts, and #1/#23 state. | The PR body makes review boundaries auditable. | Missing or contradictory drift lock, or #1/#23 state is omitted for governance/public-alpha work. |
| Generated-state audit | PR body and/or reviewer evidence includes `git status --short --ignored` when generated roots may be present. | Generated roots remain ignored/untracked unless fixture-scoped. | Tracked `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, dashboard exports, screenshots, launch reports, or local tool state appear without explicit fixture authorization. |
| Guardrail audit | PR states whether it changes launch, release, visibility, source apply, browser command, dependency, security, or public wording boundaries. | It remains docs/templates/checklists or a separately authorized bounded implementation. | It adds forbidden behavior, weakens boundaries, or omits guardrail impact. |
| Verification evidence | PR lists focused checks and broad checks appropriate to the files changed. | Required commands are run or a narrow docs-only rationale is recorded. | No verification is supplied for changed behavior/docs/templates. |
| Wording audit | Public-facing text is scanned for forbidden overclaims. | Hits are conservative negations, examples, or non-goals. | Positive claims imply production readiness, compatibility stability, secure sandboxing, Godot replacement status, native export readiness, plugin runtime readiness, source apply readiness, support SLA, public visibility changed, or launch approval. |
| Security/public data audit | PR avoids secrets, tokens, exploit details, private screenshots, private paths, and sensitive generated artifacts. | Public text is sanitized and security details are routed through `SECURITY.md`. | The PR exposes sensitive material or asks reviewers to discuss exploit details publicly. |

## Forbidden-scope gates

A public-alpha PR must be held or rejected unless a separate explicit issue
authorizes the bounded work and required verification:

| Gate | Hold/reject trigger | Required redirection |
| --- | --- | --- |
| Launch / visibility | Repository visibility changes, GitHub settings mutation, launch announcement, release publication, package publication, signing, upload, or public go-live automation. | Redirect to manual launch-governance decision records; no PR may automate the action. |
| Source apply / merge authority | Source patch apply, trusted worktree mutation, source merge/rebase automation, rollback execution, auto-merge, auto-apply, reviewer bypass, or hidden command execution. | Redirect to a source-mutation design/implementation issue with explicit trust-boundary evidence. |
| Browser command authority | Browser trusted writes, local server command bridge, uploads, installs, command execution, or hidden background actions from dashboard/cockpit surfaces. | Keep browser surfaces read-only; require a separate design gate for any future authority. |
| Dependency / supply chain | Dependency manifests, lockfiles, CI/workflows, build scripts, install scripts, package managers, network/bootstrap commands, or registry credentials are changed without explicit issue approval. | Hold for the dependency/review-readiness policy and a dedicated approval issue. |
| Native export / plugin runtime / marketplace | Native export readiness, plugin runtime, extension marketplace, third-party code loading, desktop/mobile installers, app-store readiness, or packaged release claims. | Redirect to the relevant future design gate; do not accept as public-alpha intake by default. |
| Hosted/cloud/auth/support | Hosted services, cloud runtime, multi-user auth, accounts, public support process, support SLA, or security guarantee. | Reject or redirect to a later governance issue; current public alpha is local-first and no-SLA. |
| Public wording overclaim | Wording claims production-ready, compatibility-stable, secure-sandbox, Godot replacement, native-export-ready, plugin-runtime-ready, source-apply-ready, support-SLA, launch-approved, or public-visibility-changed status. | Request conservative wording aligned with `docs/public-wording-guardrail-v1.md`. |
| Generated/private artifact | Generated run output, screenshots, dashboard exports, local tool state, private bytes, private paths, secrets, or sensitive security details are committed without fixture-scoped authorization. | Request removal/redaction and rerun generated-state/security audit. |
| #1/#23 governance drift | PR closes, replaces, weakens, or relabels #1 or #23 without a separate explicit governance decision. | Hold until maintainers record the separate governance decision or restore the anchor state. |

## Intake disposition vocabulary

Use one of these outcomes in review comments when scope is unclear:

| Outcome | Meaning | Next action |
| --- | --- | --- |
| `intake-ready` | Scope, drift lock, guardrails, wording, generated-state, and verification evidence are sufficient for normal review. | Proceed to review/merge-readiness checks. |
| `intake-hold` | Evidence is missing or a bounded design/approval issue is needed. | Request the missing evidence or redirect to the prerequisite issue. |
| `intake-reject` | The PR conflicts with non-goals or exposes unsafe public data. | Close or request a substantially different PR; preserve public safety details. |
| `intake-defer` | Maintainers cannot validate the PR safely or the timing/scope is unsuitable. | Record the deferral reason and keep launch/roadmap state unchanged. |

## Reviewer response template

```markdown
Public-alpha PR intake: <intake-ready | intake-hold | intake-reject | intake-defer>

- Linked scope:
- Drift lock complete:
- Generated-state audit:
- Guardrail audit:
- Wording audit:
- Security/public-data audit:
- Required verification:
- #1 open:
- #23 open:

Decision rationale:
```

## Relationship to the PR template

`.github/PULL_REQUEST_TEMPLATE.md` is the contributor-facing prompt for this
policy. Maintainers should use this document when a PR leaves checklist fields
blank, changes files outside the stated scope, or attempts forbidden work.

## Non-goals

This policy does not implement merge automation, branch protection, release
workflows, repository visibility changes, issue locking, security advisories,
dependency review process details, Lore commit-message policy, or final
review/merge readiness criteria. Those latter policy details are reserved for
#382 / PLG1.5.2.

## Summary

- Linked issue:
- PR unit or scoped slice:
- Roadmap bucket or design gate:

## Drift lock

- [ ] Current issue number and PR unit/slice are named above.
- [ ] Exact files expected to change are listed below:
  -
- [ ] Authorized behavior is limited to:
  -
- [ ] Explicit non-goals remain out of scope:
  - no repository visibility change, launch automation, release publication, or package publication unless a separate maintainer decision explicitly authorizes it;
  - no source patch apply, native export, plugin runtime, distributed QA runtime, hosted/cloud/auth behavior, browser trusted writes, command bridge, local server bridge, auto-merge, auto-apply, or hidden command execution unless the linked design gate and implementation issue authorize a bounded slice;
  - no production-ready, compatibility-stable, secure sandbox, Godot replacement, native export ready, plugin runtime ready, source apply ready, or support SLA claims.
- [ ] Generated artifacts remain untracked unless a separate fixture-scoped issue authorizes them: `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, dashboard exports, screenshots, launch reports, and local tool output.
- [ ] This PR explains why it remains inside the named roadmap bucket or design gate from `docs/post-launch-roadmap-triage-v1.md` or the linked issue.
- [ ] #1 and #23 remain open unless a separate explicit governance decision says otherwise.

## Public-alpha contributor guardrails

- [ ] This PR does not publish, release, announce, toggle visibility, mutate GitHub settings, or automate launch/rollback/merge behavior.
- [ ] This PR does not introduce dependency changes unless the linked issue explicitly approves them.
- [ ] This PR does not commit generated demo/run/dashboard/screenshot/local-tool state unless a fixture-scoped issue explicitly authorizes it.
- [ ] Public-facing wording has been checked for forbidden overclaims or the PR is not public-facing.
- [ ] Security-sensitive details, secrets, private paths, and private screenshots are not included.

## Verification

- [ ] Focused checks:
- [ ] Broad checks, when applicable:
- [ ] Wording/generated-state audit, when applicable:
- [ ] `git status --short --ignored` reviewed for generated/local artifacts when applicable.

## Notes

Use `CONTRIBUTING.md`, `docs/public-pr-intake-policy-v1.md`, `docs/public-wording-guardrail-v1.md`, and `docs/post-launch-roadmap-response-snippets-v1.md` for conservative issue and PR responses when scope is unclear or drifting.

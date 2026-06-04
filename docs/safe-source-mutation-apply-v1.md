# Safe Source Mutation Apply v1

Safe Source Mutation Apply v1 defines the bounded contract for any future trusted
source patch application in Ouroforge. It is a governance and implementation
scope document: source patches may reach the trusted worktree only through a
validated preview, sandbox dry-run evidence, an accepted independent review,
stale-target checks, rollback metadata, allowlisted verification, and
post-apply comparison evidence.

This milestone does not authorize unrestricted source mutation, auto-apply,
auto-merge, reviewer bypass, dependency mutation, CI/workflow mutation,
build-script mutation, browser trusted writes, command bridges, secure-sandbox
claims, autonomous source repair, current Godot replacement claims, or production
editor/engine maturity claims.

#1 remains the roadmap/vision anchor and #23 remains the repo memory/design
anchor. This contract preserves both issues as open anchors.

## Bounded target

Safe Source Mutation Apply v1 is limited to explicitly allowed source-like file
classes. It must keep these artifacts separated instead of collapsing them into a
single opaque apply result:

1. patch preview artifact;
2. file-class validation report;
3. diff integrity report;
4. sandbox dry-run plan and report;
5. review decision;
6. source apply transaction;
7. trusted worktree context evidence;
8. stale target/hash guard;
9. sandbox-to-trusted promotion readiness;
10. rollback snapshot and recovery guidance;
11. allowlisted verification report;
12. post-apply rerun/comparison evidence;
13. dependency/CI/build-script blocker evidence;
14. audit ledger entry;
15. source apply evidence bundle;
16. Studio/dashboard read-only inspection surface; and
17. emergency hold/kill-switch state.

Generated previews, sandbox outputs, rollback snapshots, verification logs,
runs, dashboard data, temp worktrees, and local tool state remain untracked
unless a later issue explicitly introduces deterministic fixture-scoped source
files.

## Trusted boundary

Trusted persistence is owned by Rust/local validation and never by the browser,
Studio, dashboard HTML, generated previews, or advisory text. A trusted source
apply readiness check must fail closed unless all required evidence is present,
fresh, and mutually consistent:

- preview id, diff integrity id, file-class report id, sandbox report id, review
  decision id, and transaction id all match;
- every target path is source-like, inside the trusted worktree, and outside
  generated/local/hidden roots;
- every before hash matches the current trusted target before any write;
- sandbox dry-run evidence proves the same diff/preview/transaction and records
  only allowlisted verification commands;
- the review decision is accepted, independent, fresh, and exactly covers the
  transaction and target set;
- rollback metadata exists before apply readiness;
- dependency manifests, lockfiles, CI/workflow files, build scripts, install
  scripts, credential/auth/network/cloud code, release/export/publish files,
  generated roots, and hidden tool roots are blocked unless separate explicit
  governance authorizes them; and
- post-apply verification plus rerun/comparison evidence is recorded before any
  success claim.

## Dependency order for follow-up issues

The follow-up issues are intentionally ordered so later trusted-write behavior is
not implemented before its controls exist:

| Order | Issue | Purpose |
| --- | --- | --- |
| 1 | #700 | Threat model and fail-closed apply preconditions. |
| 2 | #701 | Trusted worktree boundary and eligible local state. |
| 3 | #702 | Source apply transaction model. |
| 4 | #703 | Stale target/hash guard. |
| 5 | #704 | Independent review decision enforcement. |
| 6 | #705 | Sandbox-to-trusted promotion readiness. |
| 7 | #706 | Rollback snapshot and recovery metadata. |
| 8 | #707 | Allowlisted verification runner. |
| 9 | #708 | Post-apply rerun and comparison evidence. |
| 10 | #709 | Dependency, CI, and build-script blockers. |
| 11 | #710 | Append-only audit ledger. |
| 12 | #711 | Evidence bundle. |
| 13 | #712 | Studio source apply review surface, read-only only. |
| 14 | #713 | Safe source apply demo after preceding gates. |
| 15 | #714 | Regression coverage matrix. |
| 16 | #715 | Emergency hold and kill-switch. |
| 17 | #716 | Roadmap/#1 governance refresh after evidence exists. |

Later issues may split work further, but they must not weaken these boundaries or
claim production maturity before the full evidence chain exists.

## Verification and closure gates

A Safe Source Mutation Apply v1 issue cannot close until its own scope has:

- targeted tests or documentation checks for the artifact/control it introduces;
- broad repository checks when Rust contracts, dashboard readers, Studio views,
  command allowlists, or public wording change;
- evidence that #1 and #23 remain open;
- evidence that generated/local state stayed untracked except ignored roots;
- an issue comment listing merged PRs, verification output, known gaps, and
  non-goals; and
- no browser apply/write/command bridge, unrestricted source mutation,
  dependency/CI/build mutation, or production-ready claim.

For the milestone as a whole, #716 is the final governance refresh and must not
mark Safe Source Mutation Apply v1 complete until implementation evidence from
#700 through #715 exists or unresolved items are explicitly deferred with open
follow-up anchors.

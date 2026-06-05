# Safe Source Mutation Apply v1 governance handoff

Safe Source Mutation Apply v1 is complete after #716 as a bounded,
review-gated trusted source apply chain for explicitly allowed low-risk
source-like file classes only. It does not authorize unrestricted source
mutation, autonomous source repair, browser trusted writes, command bridges,
dependency/CI/build-script mutation, auto-apply, auto-merge, self-approval,
reviewer bypass, secure-sandbox guarantees, production-ready mutation claims, or
current Godot replacement positioning.

#1 remains open as the broad roadmap/final-goal anchor. #23 remains open as the
repo-memory/design context anchor.

## Completed evidence chain

- #699 — Safe Source Mutation Apply v1 scope and contract.
- #700 — source apply threat model refresh.
- #701 — trusted worktree apply boundary.
- #702 — source patch apply transaction model.
- #703 — stale target and hash guard.
- #704 — independent source apply review decision enforcement.
- #705 — sandbox-to-trusted promotion readiness.
- #706 — rollback snapshot and recovery metadata.
- #707 — allowlisted verification runner.
- #708 — post-apply rerun and comparison evidence.
- #709 — dependency, CI, and build-script mutation blockers.
- #710 — append-only source apply audit ledger.
- #711 — source apply evidence bundle.
- #712 — read-only Studio source apply review surface.
- #713 — deterministic Safe Source Apply demo.
- #714 — Scenario Coverage v14 source apply regression suite.
- #715 — emergency hold and kill-switch.
- #716 — roadmap, documentation, and #1 governance refresh.

## What is complete

Safe Source Mutation Apply v1 now records the full evidence sequence required
before a trusted worktree write can be considered:

1. validated patch preview and file-class report;
2. diff integrity evidence;
3. sandbox dry-run evidence over the same preview/transaction;
4. accepted independent review decision, with self-approval rejected;
5. clean trusted-worktree context and stale-target/hash checks;
6. rollback snapshot and recovery metadata before apply readiness;
7. allowlisted verification commands only;
8. post-apply rerun/comparison evidence before success claims;
9. dependency, lockfile, CI/workflow, build-script, credential, network, cloud,
   release/export/publish, generated-state, and hidden-root blockers;
10. append-only audit ledger and evidence bundle links;
11. read-only Studio/dashboard inspection; and
12. emergency hold/kill-switch visibility.

## Audits and boundaries

- **Generated-state audit:** generated previews, sandbox outputs, rollback
  snapshots, verification logs, run outputs, dashboard exports, temp worktrees,
  and local tool state remain ignored unless a later issue explicitly scopes a
  deterministic fixture.
- **No forbidden file-class audit:** dependency manifests, lockfiles,
  package manifests/locks, CI/workflows, build scripts, shell/install scripts,
  credential/auth/network/cloud code, release/export/publish files,
  generated/local state, hidden tool roots, opaque binaries, symlinks, traversal
  targets, and unknown classes remain blocked without a new governance decision.
- **No auto-apply audit:** the v1 path requires an accepted independent review,
  exact evidence linkage, rollback metadata, allowlisted verification, and
  post-apply evidence. It does not add autonomous apply, autonomous repair,
  reviewer bypass, self-approval, auto-merge, branch merge/rebase automation, or
  browser-originated commands.
- **Rollback audit:** rollback metadata and recovery guidance are preconditions
  for readiness and evidence, not a production recovery guarantee.
- **Browser/Studio audit:** Studio and dashboard surfaces remain escaped
  read-only inspection. They cannot apply patches, write trusted files, run
  commands, merge branches, or mutate local state.

## Known gaps

The completed milestone is intentionally narrow. Future work still needs new
scoped issues before any expansion to broader file classes, executable script
repair, dependency or CI mutation, build/release/export mutation, marketplace or
plugin runtime behavior, native/mobile/desktop export, hosted/cloud operation,
secure sandbox claims, production-grade recovery guarantees, public launch, or
current Godot replacement positioning.

## Next recommendation

Continue with the remaining accepted Era C issue sequence in small evidence-first
units: Full Studio Editor surfaces first, then Plugin / Extension System and
Godot-Plus Demo work only where the open issue sequence already scopes them. Do
not infer broader source mutation authority from this completion.

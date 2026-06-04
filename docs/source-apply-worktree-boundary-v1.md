# Source Apply Worktree Boundary v1

Source Apply Worktree Boundary v1 defines where future trusted source apply may
be considered and which local repository states must fail closed before any
trusted write. It is the SA15.3.1 policy surface for #701. It does not implement trusted source apply, patch application, rollback, verification command
execution, browser writes, command bridges, auto-merge, auto-apply, dependency
mutation, CI/workflow mutation, build-script mutation, autonomous source repair,
or production-ready mutation claims.

This policy is an input to later validation work. The authoritative fixture shape
is `examples/source-apply-worktree-boundary-v1/worktree-context-policy.sample.json`.

## Worktree zones

| Zone | Boundary | Write eligibility |
| --- | --- | --- |
| Trusted main worktree | The Git worktree containing tracked source files and the current branch/head. | Eligible only after later #702-#715 controls prove exact transaction, freshness, review, rollback, verification, and evidence. This document grants no write authority. |
| Sandbox worktree | Generated copy under a sandbox/run root, such as `runs/**/sandbox/**/worktree` or `sandbox/**/worktree`. | Patch text may be applied only as sandbox evidence. It must not mutate trusted files and must not imply secure-sandbox guarantees. |
| Temp/local tool state | `.omx/`, `.omc/`, `.claude/`, `.openchrome/`, `target/`, `runs/`, `dashboard-data/`, caches, and other ignored/generated roots. | Not eligible for trusted source apply unless a later issue explicitly scopes a deterministic fixture. |
| Browser/Studio/dashboard | Static or browser-local inspection surfaces. | Display-only/draft-only. Cannot apply patches, execute commands, merge branches, or write trusted files. |

## Allowed target roots

Future apply context validation may consider only source-like targets that the
source file-class validator reports as `allowed` or separately review-held. The
policy starts from these categories:

- deterministic seed and scene-like fixtures, for example `seeds/*.yaml` and
  explicitly tracked example scene data;
- tracked scenario packs and deterministic regression fixture data;
- review-held documentation/governance files;
- review-held Rust trust-boundary code, tests, evidence readers, and Studio or
  dashboard display code when a separate issue explicitly allows the review
  level.

A target is still blocked when it is outside the trusted worktree, under a
generated/local/hidden root, stale, dirty, ambiguous, symlink-traversed,
untracked-colliding, dependency/CI/build-script related, or missing required
apply evidence.

## Generated and local-state exclusions

The following are excluded from trusted source apply by default:

- `.git/`, `.omx/`, `.omc/`, `.claude/`, `.openchrome/`;
- `runs/`, `target/`, `dashboard-data/`, `sandbox/`, temp/cache roots;
- generated preview, sandbox, report, verification, rollback, dashboard, and run
  outputs;
- hidden path components except separately classified governance roots such as
  `.github/**`, which remain blocked by the dependency/CI/build-script blocker;
- absolute paths, rooted paths, traversal paths, and path-prefix aliases that do
  not normalize inside the trusted worktree.

## Dirty worktree policy

Future apply context validation must fail closed when any target file or target
parent state is dirty or ambiguous. Blocked states include:

- modified, deleted, renamed, or type-changed tracked target files;
- untracked files that collide with a target path or would become an apply target;
- staged changes touching target files;
- unmerged/conflicted paths;
- symlinks, hard-link aliases, or traversal where detectable for a target path;
- missing target files unless the transaction explicitly authorizes creation and
  the target has no collision; and
- inability to inspect Git status or canonical target paths.

Non-target dirty files are context evidence. A later implementation may decide
whether they block all applies, but target dirtiness and generated-root collisions
must block before trusted writes.

## Concurrent apply and branch/head policy

Trusted apply readiness must record context evidence that includes:

- repository root and canonical worktree root;
- current branch or detached-head state;
- current head commit;
- lock/attempt identity for the apply readiness check;
- target path list and target status summary;
- generated/local ignored root summary; and
- blocked reasons.

Concurrent apply hazards are blocked when a lock is active, an attempt id is
reused, the branch/head differs from the transaction expectation, or the context
cannot prove that the target state inspected is the target state being written.

## Context evidence shape

A future Rust validator should emit a generated evidence artifact shaped like
`source-apply-worktree-context-v1` with at least:

- `schemaVersion`;
- `status` (`passed` or `blocked`);
- `repositoryRoot` and `worktreeRoot`;
- `branch` and `headCommit`;
- `policyId`;
- `targets[]` with path, class decision, git status, root zone, and blocked
  reasons;
- `generatedRootSummary[]`;
- `lockStatus`;
- `blockedReasons[]`; and
- guardrails stating that no trusted write, command execution, browser write,
  auto-apply, or merge occurred.

## Closure constraints

A #701 PR unit may close only when the combined work has policy docs/fixtures,
context validation for safe/unsafe states, context evidence/read-model
compatibility, focused tests for blocked cases, full required verification, and
final evidence proving #1 and #23 remain open. This SA15.3.1 policy PR alone is
not sufficient to close #701.

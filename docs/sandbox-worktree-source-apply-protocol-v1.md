# Sandbox Worktree Source Apply Protocol v1

This protocol is the #2378/M126 sandbox worktree layer for Safe Source Mutation
Apply v1. It does **not** create a second source-apply authority path. A sandbox
apply is valid only when it references the existing Safe Source Apply preview,
review decision, transaction, rollback snapshot, and audit ledger evidence.

## Boundary

- The trusted/main worktree is never mutated by this protocol.
- Mutation happens only in an isolated temporary git worktree.
- The audit ledger records before/after `git status --short` snapshots for the
  trusted/main worktree because repository automation may independently mutate
  the main checkout.
- A dedicated `CARGO_TARGET_DIR` is required so sandbox verification cannot reuse
  or pollute the shared target directory.
- Generated evidence stays in ignored/generated roots such as `target/...` or
  `.ouroforge/generated/...`.

## Allowed file classes

The data model enumerates allowed sandbox target file classes:

- `rust-source`
- `rust-test`
- `spec-document`
- `fixture`
- `example-data`

Dependency manifests, lockfiles, CI/workflow files, build/install scripts,
credential/auth/network/cloud code, hidden tool roots, generated roots, release
publish/export files, and browser/Studio trusted-write surfaces remain blocked
unless a later explicit governance issue authorizes them.

## Required evidence fields

A protocol artifact records:

- Safe Source Apply version: `safe-source-mutation-apply-v1`.
- Preview, transaction, independent review, rollback, and audit ledger refs.
- Sandbox worktree path, trusted worktree path, base revision, evidence root, and
  dedicated `CARGO_TARGET_DIR`.
- Trusted/main worktree `git status --short` snapshots before and after.
- Per-target path, file class, before hash, expected after hash, and observed
  sandbox after hash.
- Apply state, cleanup state, and guardrails.

## Validity rules

The deterministic evaluation is `valid` only if all checks pass:

1. sandbox and trusted worktree paths differ;
2. generated evidence root is ignored/generated;
3. `CARGO_TARGET_DIR` is dedicated and not the shared `target` directory;
4. sandbox apply state is `applied`;
5. cleanup is not incomplete;
6. trusted/main before/after status snapshots match;
7. every target has an allowed file class; and
8. every observed sandbox after hash matches the expected after hash.

Any failure returns `blocked` with explicit reasons. The artifact carries no
ability to run scripts, perform browser trusted writes, self-approve, auto-apply,
auto-merge, or mutate the trusted/main worktree.

## Closure classification

#2378 is product-observed only when sandbox apply smoke evidence, blocked unsafe
path tests, git-status before/after audit snapshots, cleanup metadata, and
rollback metadata validation are linked from the PR/issue. Otherwise it must be
reported as a gap rather than represented as green product evidence.

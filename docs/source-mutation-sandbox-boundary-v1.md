# Source Mutation Sandbox Boundary v1

Source Mutation Sandbox Boundary v1 is a Source Mutation Design Gate v1 control
artifact. It defines the isolation expectations required before evaluating any
future source patch preview. It does not implement worktree automation, source
mutation apply, arbitrary patch apply, command runners, schedulers, browser
writes, command bridges, or merge automation.

The sandbox boundary exists because source patch evaluation can otherwise leak
state across the trusted repository, generated run artifacts, local credentials,
network access, dependency installation, and browser/CDP observations. A future
implementation milestone must keep evaluation isolated, reviewable, reversible,
and auditable before any source mutation apply path is considered.

## Sandbox boundary principles

- Evaluation occurs in a separate local git worktree or equivalent isolated copy,
  never in the primary maintainer worktree.
- The evaluation branch/ref is explicit and tied to the patch preview base ref.
- The worktree starts clean: no unstaged source changes, no untracked source-like
  files, and ignored local/generated roots documented before evaluation.
- Source mutation apply remains blocked under this design gate; a sandbox may be
  designed for future dry-run evaluation only.
- Browser and Studio surfaces remain read-only; they may display sandbox evidence
  but must not create worktrees, write files, or run commands.

## Worktree and branch requirements

A future sandbox evaluation design should require:

| Requirement | Purpose |
| --- | --- |
| Dedicated worktree path | Prevent evaluation from mutating the primary checkout. |
| Dedicated branch/ref | Keep preview evaluation separate from `main` and reviewer branches. |
| Recorded base commit | Detect stale preview and stale rollback context. |
| Clean preflight status | Prove no unrelated local changes contaminate evaluation. |
| Canonical repository root | Prevent path traversal or symlink/hard-link ambiguity. |
| Ignored-root inventory | Identify generated/local roots before evaluation begins. |
| Target file hash preflight | Confirm preview target hashes match the isolated worktree. |
| Cleanup plan | State how the temporary worktree and generated outputs are removed or retained for evidence. |

## Isolation invariants

A future sandbox boundary must reject or hold evaluation when:

- the worktree path is inside an ignored generated root such as `runs/` or
  `target/`;
- the target path escapes the repository root;
- target files are symlink/hard-link ambiguous;
- the base commit or target hashes differ from the patch preview;
- dependency manifests, CI workflows, secrets, build scripts, plugin loaders,
  hosted/server/auth code, native export, or public-launch automation are touched
  without separate governance approval;
- credentials or network access are required to evaluate the preview; or
- cleanup expectations are missing.

## Required sandbox preflight evidence

Before any future evaluation, the sandbox design should capture:

1. source repository path and isolated worktree path;
2. branch/ref and base commit;
3. patch preview id and patch artifact hash;
4. clean status output for source-like files;
5. ignored/generated root inventory;
6. target file hashes and file classes;
7. allowed command list for this evaluation; and
8. explicit no-credential/no-network/no-install-script policy acknowledgement.

The evidence is review context only. This document does not create the worktree,
run commands, or apply patches.

## No-implementation boundary

This sandbox boundary does not authorize:

- source mutation application;
- arbitrary patch apply;
- worktree automation or cleanup automation;
- command runners, schedulers, daemons, or hidden shell execution;
- browser-side trusted writes or command bridges;
- dependency installation or dependency mutation;
- credentialed commands, implicit network access, CI/workflow mutation, native
  export, plugin runtime, hosted/cloud/server/auth, distributed QA/Elixir,
  production editor, public launch automation, or Godot replacement claims.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. This document does not close, replace, or narrow either issue.

## Allowed command policy

A future sandbox evaluator may only run commands that are declared before
execution, require no credentials, require no network, avoid dependency
installation, and operate inside the isolated worktree or generated evidence
roots. The policy is allowlist-first: an omitted command is disallowed until a
reviewer records why it is necessary, bounded, and safe for the specific patch
preview.

### Allowlisted command classes

| Command class | Boundary |
| --- | --- |
| Repository inspection | Read-only commands such as `git status --short`, `git diff --check`, `git rev-parse`, and file hash checks against the isolated worktree. |
| Formatting checks | Check-only format commands such as `cargo fmt --check` that do not rewrite source files. |
| Deterministic tests | Project test commands already required by the governing issue, such as `cargo test`, when they do not require network, secrets, or install scripts. |
| Static analysis | Check-only linters such as `cargo clippy --all-targets --all-features -- -D warnings` when they use already-present dependencies and local caches. |
| Evidence packaging | Commands that copy or serialize bounded logs, status, hashes, and generated reports into approved generated evidence roots. |

Allowed commands must be recorded with command text, working directory, expected
outputs, timeout/failure expectations, and whether they may write generated
artifacts. Any command that can mutate source files must be converted to a
check-only mode or rejected.

### Disallowed command classes

The sandbox policy must reject:

- arbitrary shell snippets, command injection, unreviewed scripts, schedulers,
  daemons, or command runners;
- dependency installation, dependency upgrades, package-manager mutation,
  postinstall scripts, build-script mutation, or toolchain bootstrap;
- network access, remote service calls, hosted/cloud/server/auth flows,
  credential prompts, secret reads, SSH agents, token material, or browser
  session reuse that carries credentials;
- CI/workflow mutation, release/publish/export commands, native export, plugin
  runtime activation, public-launch automation, or Godot replacement claims;
- source mutation apply, arbitrary patch apply, auto-merge, auto-accept,
  auto-apply, or writes to the primary maintainer worktree; and
- browser-side trusted file writes, browser command bridges, or CDP flows that
  execute repository commands.

### No-credential, no-network, no-install-script acknowledgement

Every future evaluation record should include an explicit acknowledgement that
its allowed commands:

1. do not need credentials, tokens, SSH agents, browser login state, or local
   secret files;
2. do not require network access or remote services;
3. do not run dependency installation, postinstall hooks, or toolchain bootstrap;
4. do not mutate source files except inside a separately authorized dry-run
   sandbox artifact; and
5. do not create browser, Studio, CI, or plugin write paths.

If any acknowledgement cannot be made, the evaluation is held for separate
review instead of broadening the allowlist implicitly.

# Source Mutation Sandbox Boundary v1

Source Mutation Sandbox Boundary v1 is a Source Mutation Design Gate v1 control
artifact. It defines the isolation expectations required when evaluating a
source patch preview. The current implementation supports only bounded
sandbox-only preview application and allowlisted required-test execution. It
does not implement trusted source mutation apply, arbitrary patch apply, broad
worktree automation, schedulers, browser writes, command bridges, or merge
automation.

The sandbox boundary exists because source patch evaluation can otherwise leak
state across the trusted repository, generated run artifacts, local credentials,
network access, dependency installation, and browser/CDP observations. Every
implementation milestone must keep evaluation isolated, reviewable, reversible,
and auditable before any source mutation apply path is considered.

## Sandbox boundary principles

- Evaluation occurs in a separate local git worktree or equivalent isolated copy,
  never in the primary maintainer worktree.
- The evaluation branch/ref is explicit and tied to the patch preview base ref.
- The worktree starts clean: no unstaged source changes, no untracked source-like
  files, and ignored local/generated roots documented before evaluation.
- Source mutation apply remains blocked under this design gate; the sandbox is a
  dry-run evaluation location only.
- Browser and Studio surfaces remain read-only; they may display sandbox evidence
  but must not create worktrees, write files, or run commands.

## Worktree and branch requirements

A sandbox evaluation design must require:

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

A sandbox boundary must reject or hold evaluation when:

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

Before evaluation, the sandbox design should capture:

1. source repository path and isolated worktree path;
2. branch/ref and base commit;
3. patch preview id and patch artifact hash;
4. clean status output for source-like files;
5. ignored/generated root inventory;
6. target file hashes and file classes;
7. allowed command list for this evaluation; and
8. explicit no-credential/no-network/no-install-script policy acknowledgement.

The evidence is review context only. A bounded Rust-owned evaluator may create
sandbox-local generated files, copy/apply preview diffs inside the sandbox
worktree, and run allowlisted required-test `argv` vectors there; that evidence
must not be treated as trusted-worktree apply authority.

## No-implementation boundary

This sandbox boundary does not authorize:

- source mutation application to the trusted worktree;
- arbitrary patch apply outside the validated preview/sandbox path;
- broad worktree automation or cleanup automation beyond declared sandbox roots;
- shell command runners, schedulers, daemons, or hidden shell execution;
- browser-side trusted writes or command bridges;
- dependency installation or dependency mutation;
- credentialed commands, implicit network access, CI/workflow mutation, native
  export, plugin runtime, hosted/cloud/server/auth, distributed QA/Elixir,
  production editor, public launch automation, or Godot replacement claims.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. This document does not close, replace, or narrow either issue.

## Allowed command policy

A sandbox evaluator may only run commands that are declared before execution,
require no credentials, require no network, avoid dependency installation, and
operate inside the isolated worktree or generated evidence roots. The policy is allowlist-first: an omitted command is disallowed until a
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

Allowed commands must be recorded with command text, normalized `argv`, matched
allowlist policy id, working directory, expected outputs, timeout/failure
expectations, and whether they may write generated artifacts. Any command that can mutate source files must be converted to a
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

## Failure and cleanup policy

A sandbox evaluation must fail closed. Any failed preflight, rejected
command, stale hash, missing evidence artifact, unexpected source-like write,
credential/network/install-script requirement, or cleanup ambiguity stops the
evaluation and records a reviewer-facing failure instead of retrying with a
broader boundary.

### Failure handling expectations

| Failure class | Required response |
| --- | --- |
| Dirty or ambiguous worktree | Stop before evaluation; record status output and conflicting paths. |
| Stale base commit or target hash | Stop before evaluation; record expected and observed refs/hashes. |
| Disallowed command request | Stop before execution; record the rejected command and policy reason. |
| Network, credential, or install-script requirement | Stop before execution; record the blocked requirement without reading secrets or contacting services. |
| Test/check failure | Preserve bounded logs and generated evidence; do not auto-apply, auto-revert source, or broaden the command allowlist. |
| Unexpected source-like write | Stop and quarantine evidence; record touched paths and require reviewer decision before cleanup or retry. |
| Cleanup failure | Preserve enough state for diagnosis; do not silently delete evidence or continue with a contaminated worktree. |

Failures must be terminal for that evaluation attempt unless a reviewer creates a
new explicit evaluation record with updated scope. Retrying in place, editing the
primary checkout, or switching to a more permissive shell path is prohibited.

### Cleanup expectations

Before evaluation begins, the design must state which generated artifacts are
retained for evidence and which temporary directories may be removed. Cleanup is
limited to the isolated worktree and generated evidence roots declared in the
preflight record. It must not remove or rewrite primary-repository source files,
ignored local configuration, credentials, dependency caches, browser profiles, or
unrelated generated runs.

A cleanup record should capture:

1. evaluation id and patch preview id;
2. isolated worktree path and branch/ref;
3. generated evidence roots retained;
4. temporary paths removed, if any;
5. paths intentionally retained for reviewer inspection;
6. cleanup command text or manual action description; and
7. cleanup result, including any remaining contaminated state.

If cleanup is unsafe or incomplete, the final state is `failed_cleanup_required`
or equivalent reviewer-visible status, not `passed`.

### Evidence capture expectations

A sandbox evaluation should capture a bounded evidence bundle containing:

- preflight repository/worktree/ref/hash/status evidence;
- allowed command policy acknowledgement and command list;
- command outputs, exit statuses, and bounded logs for check-only evaluation;
- generated artifact inventory before and after evaluation;
- failure classification and blocked reason when evaluation does not pass;
- cleanup plan and cleanup result; and
- explicit confirmation that source mutation apply, arbitrary patch apply,
  browser trusted writes, hidden command execution, credentials, network access,
  install scripts, CI/workflow mutation, and dependency mutation remained blocked.

The evidence bundle is review material only. It does not authorize merge,
auto-accept, source mutation application, or closure of #1 or #23.

## Implemented generated-state audit for SMP1.6

The SMP1.6 sandbox evaluator writes only generated/local artifacts under the
declared sandbox root, for example:

- `sandbox/<evaluation-id>/worktree/...` for the isolated preview copy;
- `sandbox/<evaluation-id>/evidence/report.json` for sandbox apply evidence;
- `sandbox/<evaluation-id>/evidence/test-execution-report.json` for allowlisted
  required-test execution summaries; and
- `sandbox/<evaluation-id>/cleanup.json` or equivalent cleanup metadata when the
  governing issue scopes cleanup evidence.

These paths are generated evidence, not source fixtures, unless a later issue
explicitly scopes a tiny deterministic checked-in fixture. They must stay
untracked with `runs/`, `target/`, dashboard exports, `.omx/`, `.openchrome/`,
`.omc/`, and `.claude/`. PR and issue closure evidence must include `git status
--short`, `git diff --check`, and a statement that no generated sandbox, preview,
report, dashboard, run, or local tool artifact was committed.

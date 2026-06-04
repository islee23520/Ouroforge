# Source Patch Sandbox Dry-Run Evaluator v1

Source Patch Sandbox Dry-Run Evaluator v1 is the bounded local evidence path for
Source Mutation Preview v1 issue #360. It exists to answer whether an already
validated source patch preview can be applied and checked inside an isolated
local copy without mutating the trusted main worktree.

It is not a trusted source-apply implementation, a branch merge tool, a secure
sandbox guarantee, a browser command bridge, a dependency installer, or a
production editor/runtime feature.

## Implemented boundary

The evaluator is split into explicit stages:

1. `SourcePatchSandboxEvaluatorPlan` validates the sandbox id, run-relative
   `sandbox/<id>/...` layout, cleanup policy, preview refs, and required-test
   allowlist metadata.
2. `apply_source_patch_preview_in_sandbox` copies preview targets from the
   trusted repo into `sandbox/<id>/worktree`, applies unified diff hunks there
   with Rust code, writes `sandbox/<id>/evidence/report.json`, and hashes trusted
   targets before and after to prove they were not written.
3. `run_source_patch_sandbox_allowlisted_tests` executes only declared
   `requiredTests` argv vectors that match the source patch test command
   allowlist and forbidden-command classifier. Commands run with `current_dir`
   set to `sandbox/<id>/worktree` and evidence is written to
   `sandbox/<id>/evidence/test-execution-report.json`.

All stages fail closed. Stale target hashes, invalid layout paths, missing
worktrees, command text/argv drift, forbidden command classes, non-allowlisted
argv, and failing test exits are recorded as blockers instead of escalating to a
trusted write or retrying destructively.

## Guardrails

The evaluator must preserve these invariants:

- Source patch apply to the trusted main worktree remains unimplemented and
  explicitly forbidden.
- Patch text may be applied only to sandbox worktree copies.
- Test commands may execute only as normalized argv arrays matched by the
  repository-local allowlist; no shell parsing or shell metacharacter
  composition is allowed.
- Network, install/bootstrap, credential/cloud-auth, dependency mutation,
  destructive filesystem, Git apply/merge/rebase/push, browser bridge, and local
  server command bridge commands are rejected before execution.
- Browser, dashboard, and Studio surfaces may display exported sandbox evidence
  read-only only; they do not invoke the evaluator.
- The evaluator makes no secure-sandbox, production isolation, public launch,
  hosted/cloud, native export, or broad editor/runtime claims.

## Generated-state audit

Sandbox evaluator outputs are generated local state. They must remain untracked
unless a future issue explicitly scopes a tiny deterministic fixture.

Generated paths include:

- `sandbox/<id>/worktree/**` — copied and patched preview targets;
- `sandbox/<id>/evidence/report.json` — sandbox apply report;
- `sandbox/<id>/evidence/test-execution-report.json` — allowlisted test evidence;
- `sandbox/<id>/cleanup.json` — cleanup/failure policy metadata;
- `.omx/context/issue-360-*.log` — local verification logs;
- `runs/**`, `target/**`, dashboard exports, and local tool state.

The repository ignores top-level `/sandbox/` in addition to existing generated
roots (`/runs/`, `/target/`, `.omx/`, `.omc/`, `.openchrome/`, `.claude/`, and
dashboard data exports). PRs must still run `git status --short --ignored` and
confirm only expected ignored generated roots appear.

## Cleanup and failure policy

A successful local run may remove sandbox worktree copies after evidence is
captured. A failed run should preserve evidence long enough for review. Cleanup
metadata must distinguish success cleanup from failure preservation and must not
remove trusted source files.

Cleanup remains a local generated-state concern. It does not authorize Git
clean/reset, recursive destructive commands outside the sandbox root, or hidden
browser/UI cleanup actions.

## Verification checklist

Every evaluator change should include focused tests for the exact changed slice
and broader checks when the issue requires them:

- live `gh issue view` checks for the current issue plus #1 and #23;
- sandbox layout/cleanup validation tests;
- sandbox-only apply isolation and stale-target rejection tests;
- allowlisted execution and forbidden-command rejection tests;
- `cargo fmt --check`;
- targeted Rust tests for source patch preview/sandbox/allowlist behavior;
- broader `cargo test` and `cargo clippy --all-targets --all-features -- -D warnings` when scoped;
- Node dashboard/cockpit checks when read-only display surfaces are in scope;
- `git diff --check`;
- `git status --short --ignored` generated-state audit.

## Known non-goals and gaps

- No trusted project worktree apply.
- No branch merge/rebase/push automation.
- No dependency installation, lockfile mutation, CI/workflow mutation, or build
  script mutation.
- No browser/dashboard/Studio command execution.
- No production security sandbox guarantee.
- No public release or package publishing decision.

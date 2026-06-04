# Source Patch Preview Coverage Matrix v1

Status: **Scenario Coverage v6 / SMP1.11.3 coverage matrix** for issue #365.

This matrix records the regression coverage added across SMP1.11.1,
SMP1.11.2, and SMP1.11.3 for source patch preview safety and evidence behavior.
It is documentation and test coverage evidence only. It does not implement source
patch apply, branch merge/rebase automation, browser trusted writes, command
bridges, dependency/CI mutation, arbitrary shell execution, public launch, or a
production security sandbox guarantee.

## Coverage map

| Risk / behavior | Coverage | Evidence |
| --- | --- | --- |
| Forbidden file classes | Preview target class regression tests reject generated, dependency, CI/workflow, build-script, hidden/local state, and unsafe source-like targets before preview/sandbox. | `crates/ouroforge-core/tests/patch_preview_artifact.rs`; `crates/ouroforge-core/src/lib.rs` |
| Unsafe paths and path traversal | Preview validation and patch diff integrity tests reject escaping paths, generated roots, duplicate targets, unsafe diff targets, and stat drift before any preview artifact is treated as ready. | `crates/ouroforge-core/tests/patch_preview_artifact.rs`; `crates/ouroforge-core/tests/patch_diff_integrity.rs` |
| Malformed diffs | Malformed parser warnings become preview blockers, so malformed diff text cannot pass preview validation before sandbox/review. | `crates/ouroforge-core/tests/patch_preview_artifact.rs`; `crates/ouroforge-core/tests/patch_diff_integrity.rs` |
| Missing evidence | Preview validation rejects missing source preview, file-class, diff-integrity, required-test, and linked evidence fields. | `crates/ouroforge-core/tests/patch_preview_artifact.rs`; `crates/ouroforge-core/tests/source_patch_evidence_bundle.rs` |
| Forbidden commands | Required test command metadata is normalized as argv and rejected for network/install/credential/destructive/Git apply/merge/browser-bridge classes before sandbox execution. | `crates/ouroforge-core/tests/source_patch_command_allowlist.rs`; `crates/ouroforge-core/tests/test_command_allowlist.rs`; `crates/ouroforge-core/tests/patch_preview_artifact.rs` |
| Sandbox dry-run pass/fail | Sandbox evaluator tests cover generated sandbox layout, sandbox-only patch application, allowlisted test pass, failed smoke capture, stale-target rejection, and no trusted-worktree writes. | `crates/ouroforge-core/tests/source_patch_sandbox_evaluator.rs` |
| Review and bundle linkage | Evidence bundle tests preserve preview, sandbox, test, review, stale-target, and forbidden-action refs without granting apply authority. | `crates/ouroforge-core/tests/source_patch_evidence_bundle.rs`; `crates/ouroforge-core/tests/source_patch_review_decision.rs` |
| Dashboard display | Dashboard Node tests render source patch evidence bundles, stale-target guards, and apply-transaction readiness as escaped read-only evidence; no apply/merge/command controls are rendered. | `examples/evidence-dashboard/dashboard.test.cjs`; `examples/evidence-dashboard/dashboard.js` |
| Studio/cockpit display | Authoring cockpit Node tests render source patch evidence bundles, stale-target guards, and apply-transaction readiness as escaped read-only Studio evidence; no apply/merge/command controls are rendered. | `examples/authoring-cockpit/cockpit.test.cjs`; `examples/authoring-cockpit/cockpit.js` |
| Generated-state hygiene | Generated preview, sandbox, report, dashboard, run, and local tool artifacts remain ignored/untracked unless fixture-scoped. | `git status --short --ignored`; `.gitignore`; `docs/source-patch-sandbox-dry-run-evaluator-v1.md` |

## Read-only surface assertions

Dashboard and Studio/cockpit surfaces may display:

- source patch preview ids and refs;
- file-class, diff-integrity, sandbox, test, review, stale-target, and bundle
  evidence refs;
- blocked reasons and forbidden-action notices;
- copyable/inert command text when already exported as evidence.

They must not render or create:

- apply patch buttons or trusted write controls;
- merge, rebase, push, auto-merge, auto-apply, or auto-accept controls;
- browser command bridges, local server command bridges, hidden command
  execution, or GitHub calls;
- dependency install, network, credential, CI/workflow, build-script, or arbitrary
  shell command authority.

## Verification commands

SMP1.11.3 and issue closure should record:

```bash
gh issue view 365 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

## Known gaps and non-goals

- No source patch apply to the trusted main worktree.
- No branch merge/rebase/push automation.
- No browser trusted writes, uploads, command bridges, local server bridges, or
  command execution.
- No dependency installation, network fetch, credentialed command, CI/workflow
  mutation, or build-script mutation.
- No production security sandbox guarantee.
- No public release or repository visibility decision.

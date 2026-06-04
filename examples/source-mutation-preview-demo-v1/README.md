# Source Mutation Preview Demo v1

This directory contains the SMP1.10 demo preview fixture for #364.

- `patch-preview-demo.sample.json` is a source patch preview artifact that targets an already-classified deterministic runtime demo config path.
- `sandbox-dry-run-plan.sample.json` records the inert sandbox plan, generated output roots, cleanup policy, and allowlisted smoke command.
- `sandbox-dry-run-evidence-summary.sample.json` records stable evidence ids and summaries while leaving generated sandbox reports untracked.
- The preview diff is embedded as evidence data only. It is not applied to the trusted worktree.
- `demo-behavior-copy.md` is a separate before-state note for this demo directory; the preview target itself is not modified by this PR.

## Preview-only workflow

1. Validate the preview artifact and file/diff metadata with Rust tests.
2. Validate the inert sandbox dry-run plan and execute a temp-dir smoke that applies the preview only inside a generated sandbox worktree.
3. Record evidence ids/summaries for generated sandbox reports without committing generated output.
4. In a later PR unit, document dashboard/Studio display and closure evidence.

## Guardrails

- No source patch apply to the trusted main worktree.
- No merge, auto-merge, auto-apply, auto-accept, branch automation, browser trusted write, command bridge, local server bridge, or hidden command execution.
- No dependency, CI/workflow, build-script, credentialed, network, install, or arbitrary shell command scope.
- Generated preview, sandbox, report, dashboard, and run artifacts remain untracked unless a later issue explicitly scopes a deterministic fixture.
- #1 and #23 remain open.

## Focused validation

```bash
cargo test -p ouroforge-core source_mutation_preview_demo_fixture_validates_without_apply_authority -- --nocapture
cargo test -p ouroforge-core source_mutation_preview_demo_sandbox_dry_run_keeps_generated_output_untracked -- --nocapture
```

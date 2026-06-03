# Authoring Loop Recovery v1

Authoring Loop Recovery v1 makes interrupted or failed authoring loops inspectable before any retry.

Recovery is explicit and local:

- `cargo run -p ouroforge-cli -- loop status <plan>` renders a read-only status summary.
- `cargo run -p ouroforge-cli -- loop resume <plan> --step <step-id>` renders a resume preflight summary.
- Resume preflight validates missing prerequisites, completed/running state, non-retryable recovery metadata, rollback references, and recovery evidence before any future execution.
- In this version, resume preflight does **not** execute the step. It reports readiness, blockers, required manual action, and safe command context.

## Recovery metadata

A loop step may include optional `recovery` metadata:

- failure reason/category/detail;
- retryability (`retryable`, `non-retryable`, or `manual-action-required`);
- required manual action;
- rollback references;
- evidence references;
- safe resume command context.

Existing plans without recovery metadata remain valid.

## Browser boundary

Dashboard and cockpit surfaces may render `loop_recovery`, `loopRecovery`, `loop_status`, or `loopStatus` data as escaped read-only evidence. They must not add resume buttons, execute commands, repair artifacts, apply mutations, promote regressions, or write trusted state.

## Non-goals

Authoring Loop Recovery v1 does not add hidden retries, automatic failure repair, remote coordination, hosted orchestration, scheduler/daemon behavior, source-code mutation, or public-launch automation.

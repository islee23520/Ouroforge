# Source Patch Test Command Allowlist v1

Source Patch Test Command Allowlist v1 is an inert policy artifact for Source
Mutation Preview. It records repository-local verification commands that may be
considered by a future sandbox dry-run evaluator. The artifact itself does not
execute commands.

## Schema

Fixture: `examples/source-patch-command-allowlist-v1/allowed-commands.sample.json`

Top-level fields:

| Field | Required | Meaning |
| --- | --- | --- |
| `schemaVersion` | yes | Must be `source-patch-test-command-allowlist-v1`. |
| `policyId` | yes | Stable identifier for the repository-local command policy. |
| `commands` | yes | Allowlisted command entries expressed as command text plus `argv`. |
| `guardrails` | yes | Must state that the policy is inert and does not execute commands. |

Each command records:

- `id`
- `command`
- `argv`
- `category`
- `matchPolicy` (`exact` or `prefix`)
- `workingDirectory` (`.` for the repository root)
- generated-artifact write declaration
- timeout
- rationale

## Matching policy

The Rust model normalizes command text from `argv` by joining trimmed arguments
with single spaces. The stored `command` must match that normalized value.

- `exact` entries must match one known command exactly, such as
  `cargo fmt --check`, strict clippy, or known Node syntax/smoke checks.
- `prefix` entries are reserved for focused Cargo test surfaces such as
  `cargo test -p ouroforge-core --test ...` and
  `cargo test -p ouroforge-cli --test ...`.

## Guardrails

- This policy is review metadata until a later issue explicitly scopes sandbox
  execution.
- Source patch apply to the trusted worktree remains unimplemented and
  forbidden.
- Browser/dashboard/Studio surfaces may display command text read-only only.
- Network, install, credential, dependency mutation, destructive filesystem, and
  arbitrary shell command rejection is handled by the follow-up forbidden-command
  policy unit.

## Forbidden command rejection

SMP1.5.2 adds a fail-closed classifier that rejects forbidden command classes
before allowlist prefix matching and before any future sandbox execution. The
classifier blocks:

- shell execution and shell metacharacter composition;
- network and remote-service commands;
- install/bootstrap/package acquisition commands;
- dependency mutation commands;
- credential, token, or cloud-auth commands;
- destructive filesystem/worktree reset commands; and
- source patch apply, merge, rebase, push, or auto-apply commands.

A rejected command records a reason and the boundary that no command was run.


## Patch preview requiredTests integration

SMP1.5.3 wires patch preview `requiredTests` metadata to this policy. When a
preview required test declares `allowlistPolicyId`, validation requires `argv`,
checks that `command` matches normalized `argv`, rejects forbidden command
classes first, and then matches the inert default allowlist. This is still a
pre-execution validation hook; it does not create a sandbox or run the command.

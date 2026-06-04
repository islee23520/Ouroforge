# Source Patch Test Command Allowlist v1

Source Patch Test Command Allowlist v1 is a policy artifact for Source Mutation
Preview. It records repository-local verification commands as normalized argv
vectors. The artifact itself is data, but the sandbox dry-run evaluator may
execute matching argv vectors inside `sandbox/<id>/worktree` after sandbox plan
validation and forbidden-command rejection.

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

- This policy is data; execution is allowed only through the sandbox dry-run
  evaluator after sandbox layout validation and forbidden-command rejection.
- Source patch apply to the trusted worktree remains unimplemented and
  forbidden.
- Browser/dashboard/Studio surfaces may display command text read-only only.
- Network, install, credential, dependency mutation, destructive filesystem, and
  arbitrary shell command rejection is handled by the follow-up forbidden-command
  policy unit.

## Forbidden command rejection

SMP1.5.2 adds a fail-closed classifier that rejects forbidden command classes
before allowlist prefix matching and before sandbox execution. The
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
preview required test includes `argv`, validation checks that `command` matches
normalized `argv`, rejects forbidden command classes first, and then matches the
inert default allowlist. `allowlistPolicyId` identifies the current preview
policy (`source-patch-preview-safe-local-checks-v1`) and requires `argv` when it
is present; it does not grant broader authority than an `argv`-only metadata
record. Preview validation itself remains pre-execution metadata; sandbox
execution requires a separate validated sandbox plan and worktree.

Prefix matches are intentionally review metadata only. They are useful for
showing that a command is shaped like a known local check, but they are not an
execution contract by themselves. The sandbox evaluator adds explicit policy-id
checks, forbidden-command rejection, worktree isolation, and audit evidence
before a matched command can be run.

## Sandbox required-test execution integration

SMP1.6.3 uses this policy as an execution gate inside the sandbox evaluator. A
required test may run only when all of the following are true:

1. `argv` is present and non-empty;
2. `command` equals the normalized `argv` text;
3. `allowlistPolicyId` is `source-patch-preview-safe-local-checks-v1`;
4. the forbidden-command classifier does not detect shell, network, install,
   credential, dependency mutation, destructive filesystem, source apply, merge,
   rebase, push, or auto-apply behavior; and
5. the normalized command matches the default source-patch test command
   allowlist.

Execution uses the sandbox worktree as `current_dir` and writes bounded stdout,
stderr, exit-code, command, `argv`, matched policy id, blocked reasons, and
guardrails to `sandbox/<id>/evidence/test-execution-report.json`. Failed or
blocked commands are evidence for reviewers; they do not authorize broadening the
allowlist, retrying through a shell, applying patches to the trusted worktree, or
triggering browser/Studio command execution.

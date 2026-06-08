# Dogfooding Campaign Harness and Friction Logging v1

Era L Milestone 68 adds the campaign harness for the existing Era I
engine-builder deckbuilder. The harness is a Rust/kernel entrypoint plus CLI
surface that creates or resumes the real-title run, records deterministic stage
progress in the existing `ledger.jsonl`, and maps every friction point to the
existing evidence pipeline. It does not add a verification engine, database,
telemetry schema, browser authority, or Elixir control-plane mutation path.

## Goal

Drive the dogfood title through the existing executor/openchrome evidence path
and log every friction point as evidence. The default CLI target is:

```bash
cargo run -p ouroforge-cli -- dogfood harness \
  --seed-path seeds/dogfood-deckbuilder.yaml \
  --runs-root runs \
  --workers 2
```

## Final Implementation Scope

- Rust harness API: `run_dogfood_campaign_harness`.
- CLI entrypoint: `ouroforge-cli dogfood harness`.
- Uses `create_run`, `bind_run_command_context`, and existing ledger append/read
  APIs rather than a new store.
- Emits append-only campaign events for detect, explain, trace, attribute,
  propose, re-verify, and apply-or-queue.
- Emits friction events for `stall`, `retry`, `manual-intervention`,
  `budget-halt`, and `gate-fail`, with stage attribution and evidence refs.
- Supports `--resume-run-dir` by reading the existing ledger and idempotently
  skipping already-recorded events.

## Success Criteria

- The harness creates or resumes the real-title run.
- Every campaign stage is represented in the existing ledger.
- Every friction observation carries a stage and existing artifact refs.
- Resume is deterministic and does not duplicate stage/friction events.
- The path remains autonomous; no human input is required by the harness.
- HIGH-RISK/source-affecting fixes are not auto-applied.

## Verification Method

```bash
set -euo pipefail
cargo build --workspace --jobs 2
cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2
! git ls-files | grep -qiE "new_(db|store|telemetry)_schema" || true
```

## Guardrails

- The autonomous path records progress without asking a human.
- Source-affecting fixes still route through source-apply, the four gates,
  design-integrity, openchrome re-run, and trust-gradient.
- HIGH-RISK/source-affecting changes stay queued for thin human go/no-go and are
  never auto-applied by this harness.
- The existing evidence pipeline remains authoritative: verdicts, `journal.md`,
  `ledger.jsonl`, loop-coverage attribution, evolve, source-apply, and
  trust-gradient.
- Rust remains the trusted data plane; the Elixir executor remains unchanged.

## Explicit Non-Goals

- New verification engine, data plane, telemetry store, or campaign database.
- Replacing openchrome, verdicts, journal, ledger, evaluator gates, source-apply,
  or trust-gradient.
- Mandatory human input in the autonomous path.
- Automated fun/taste or release go/no-go.
- Layer 3 distributed/hosted/live-ops work.

## Implementation Approach

The harness creates a normal run (or resumes one), binds the existing run command
context, and appends idempotent ledger events using stable idempotency keys. The
report returned by the API/CLI is derived from the run ledger; it is not a new
persistent artifact. Friction input is supplied as JSON observations and must
cite run/repo-relative artifact refs.

## PR Decomposition

This PR implements the campaign harness and friction logging together because
both are one append-only ledger path and are validated by the same resumability
contract tests.

## Over-Engineering Checklist

- [x] No new database, telemetry schema, or verifier.
- [x] No speculative diagnosis engine.
- [x] No direct source mutation.
- [x] No human UI surface.
- [x] State is existing run artifacts only.

## Drift-Prevention Checklist

- [x] Autonomous-first: the harness runs without human input.
- [x] Engine fixes still route through source-apply/gates/trust-gradient.
- [x] HIGH-RISK/source-affecting changes are never auto-applied.
- [x] Rust data plane and Elixir control plane boundaries remain unchanged.
- [x] Fun/taste and release go/no-go remain human.
- [x] #1 and #23 remain open.

## Language Boundary

The harness is Rust (`ouroforge-core`) with a CLI wrapper. Browser evidence still
comes from openchrome/JS probes, and the Elixir executor remains unchanged.

## Critical Risk Review

- **Duplicate ledger events on resume:** stable idempotency keys skip existing
  campaign events.
- **Friction without attribution:** validation rejects friction without a known
  campaign stage or evidence refs.
- **New data plane drift:** validation rejects new-store-looking refs and tests
  assert no `new_(db|store|telemetry)_schema` tracked file appears.
- **Self-application risk:** the harness only logs; source changes remain gated
  by source-apply and trust-gradient.

## Definition of Done

The harness is invokable, creates/resumes a run, records all campaign stages and
friction in `ledger.jsonl`, remains deterministic, introduces no new store or
verification engine, and passes the required build/test gate.

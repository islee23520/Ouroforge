# Review Decision Ledger v1

Review Decision Ledger v1 records explicit local review decisions for mutation
proposals. The ledger is written only by the Rust CLI under a run's generated
mutation artifacts:

```text
<run-dir>/mutation/review-decisions.json
```

The record is append-only audit evidence. It does not apply, merge, promote, or
rerun a mutation.

## Decision record fields

Each decision records:

- `id` — deterministic append-order id such as `review-decision-1`;
- `patch_draft_id` and `proposal_id` — the reviewed draft/proposal linkage;
- `state` / `decision_status` — `accepted`, `rejected`, `deferred`, or legacy
  `pending_review`;
- `reviewer_type` and `reviewer` — trusted local metadata (`human`, `agent`, or
  `system`);
- `reason` and `evidence_refs` — required for accepted/rejected/deferred
  decisions;
- optional expected hash fields;
- `guardrail_checklist` — must confirm record-only behavior, no auto-apply,
  browser read-only boundaries, and checked evidence refs.

## CLI usage

```bash
cargo run -p ouroforge-cli -- mutation review <run-dir> \
  --proposal <proposal-id> \
  --decision accepted \
  --reason "manual evidence review accepted" \
  --evidence mutation/patch-drafts.json
```

Allowed decisions are `accepted`, `rejected`, and `deferred`. Legacy flags
`--accept`, `--reject`, and `--defer` remain supported.

## Read-only surfaces

The journal, dashboard, and authoring cockpit may display decision ids, proposal
links, reviewer metadata, reasons, evidence refs, and guardrail checklist state.
Those surfaces remain read-only. Browser surfaces must not write review
decisions, execute commands, apply patches, or merge changes.

## Non-goals

- no mutation application;
- no auto-accept, auto-apply, auto-merge, or auto-promotion;
- no browser command bridge;
- no remote reviewer, auth system, or hosted audit service;
- no source-code mutation.

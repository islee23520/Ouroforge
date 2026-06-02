# Mutation Proposal Quality v2

Mutation Proposal Quality v2 makes mutation proposals evidence-linked review
records. A proposal may explain why a change is suggested, which evidence
supports it, what effect is expected, and which bounded mutation type is allowed.
It remains a record: it does not apply, accept, promote, rerun, merge, or execute
anything by itself.

## Proposal rationale fields

When a v2 rationale is present on `mutation/proposals.json`, it uses these
fields:

- `schema_version`: currently `1`.
- `failure_classification`: bounded deterministic failure category such as
  `scenario_assertion_failure`, `runtime_probe_failure`, `visual_mismatch`,
  `performance_regression`, `console_error`, `missing_evidence`, or `unknown`.
- `evidence_artifact_ids`: evidence artifact ids supporting the proposal. A v2
  rationale must include the proposal's primary `evidence_id`.
- `scenario_result_refs`: optional generated scenario result paths linked to the
  failure.
- `verdict_refs`: verdict artifacts, currently `verdict.json` for evolve
  proposals.
- `expected_effect`: deterministic review-facing expectation for the proposed
  change.
- `confidence`: bounded value (`low`, `medium`, or `high`).
- `reasoning_summary`: deterministic explanation derived from existing verdict,
  evidence, and journal context.
- `allowed_mutation_type`: bounded value (`scene_only`, `project_scene_only`, or
  `data_only`).

Existing v1 proposals without `rationale` remain readable. Malformed v2
rationale records fail validation or are displayed as missing/unavailable by
browser surfaces.

## Evolve emission

Failure-driven `evolve` proposals now attach rationale before the journal is
updated. The rationale is deterministic and derived from:

- the first failure in `verdict.json`;
- evidence refs from the verdict/failure;
- the evidence index;
- existing journal text when present;
- the selected proposal evidence id.

The journal `Next Mutation` section displays rationale summary, expected effect,
evidence ids, allowed mutation type, and confidence as text only.

## Dashboard and cockpit surfaces

The evidence dashboard and authoring cockpit render proposal rationale read-only
from exported dashboard data. They escape all proposal/rationale text and show
missing rationale as an explicit empty state.

These browser surfaces must not:

- write trusted files;
- execute commands;
- accept/reject proposals;
- apply mutations;
- promote regression scenarios;
- rerun QA;
- merge or rebase branches;
- start a command bridge.

## Verification

Focused checks for Proposal Quality v2:

```bash
cargo test mutation_proposal
cargo test evolve
cargo test journal
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Broad issue closure should also run `cargo fmt --check`, `cargo test`, and
`cargo clippy --all-targets --all-features -- -D warnings` on latest `main`.

## Non-goals

Proposal Quality v2 does not authorize:

- review decision ledger behavior;
- accepted mutation application changes;
- regression scenario promotion;
- source-code mutation;
- auto-apply, auto-accept, auto-promote, auto-rerun, or auto-merge;
- browser trusted writes or local command bridges;
- LLM/network proposal generation;
- production editor, plugin runtime, native export, hosted/cloud/server/auth, or
  public-launch behavior.

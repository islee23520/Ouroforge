# Multi-Agent Production Pipeline Demo v1

Status: **deterministic, fixture-scoped demo**. This walkthrough composes the
already-merged Multi-Agent Production Pipeline contracts under #1 Era H
Milestone 42 into one end-to-end story:

- the role-agent model and per-artifact ownership (#1675, `production_roles`);
- handoff artifacts and deterministic conflict resolution (#1676,
  `production_handoff`);
- reviewer/critic promotion gates over the Milestone 22 trust gradient (#1678,
  `production_review_gates`).

It adds **no new engine, runtime, writer, or orchestration system**. It is a set
of deterministic fixtures plus a smoke test that loads them through the existing
contracts and asserts behavior and gate states — never subjective quality. It
runs with **no network and no live browser**, and reproduces identically from a
fresh clone.

## Scenario

Three role agents collaborate on one project (a puzzle vertical slice):

1. The **designer** proposes a `design-brief`; the **level-designer** proposes a
   `scene-draft`; the **asset-import-planner** proposes an `asset-proposal`. Each
   role owns exactly one artifact class.
2. The roles hand work off to one another: designer → level-designer →
   reviewer, and asset-import-planner → qa-agent. Every handoff is a clean,
   accepted record (proposal-only; nothing is auto-applied).
3. The level-designer's `scene-draft` is submitted for promotion. While the
   review is **unreviewed**, the reviewer/critic gate **blocks** it. Only after
   an independent reviewer **and** critic both approve does the gate become
   `promote-allowed` — and even then the proposal merely routes through the
   existing review/apply/trust-gradient path; it is never auto-applied.

## Fixtures

All fixtures live under `examples/production-pipeline-v1/demo/` and validate
through the existing contracts:

| Fixture | Contract | Demonstrates |
| --- | --- | --- |
| `roles.fixture.json` | `production-roles-v1` | ownership per role; an unauthorized write by a non-owner and a direct trusted write are both rejected fail-closed |
| `handoffs.fixture.json` | `production-handoff-v1` | three clean, accepted role handoffs across the collaboration chain |
| `review-gate.before.fixture.json` | `production-review-gates-v1` | the `scene-draft` promotion is **blocked** while the reviewer is `pending` |
| `review-gate.after.fixture.json` | `production-review-gates-v1` | the **same gate** becomes `promote-allowed` only after reviewer **and** critic approve |

The before/after gate fixtures share the gate id `demo-gate-scene-draft` to show
the deterministic progression from blocked to promote-allowed.

## Reproduce

From a fresh clone, with no network and no live browser:

```bash
cargo test -p ouroforge-core --test production_pipeline_demo_contract
```

The smoke test `crates/ouroforge-core/tests/production_pipeline_demo_contract.rs`
loads every fixture through its contract and asserts:

- the role model records single-owner ownership and rejects the unauthorized
  write and the direct trusted write (fail-closed);
- the handoff ledger records three clean, accepted handoffs with no unresolved
  conflicts;
- the **before** gate is `blocked` (blocked-until-reviewed);
- the **after** gate, for the same gate id, is `promote-allowed`;
- no surface auto-applies, auto-merges, or self-approves.

## Boundary

This demo is inert local evidence. Role agents, generation, and any
browser/Studio surface emit proposals only, through the existing
review/apply/trust-gradient path. There are no direct trusted writes, no
auto-apply, no auto-merge, no self-approval, and no reviewer bypass. Browser and
Studio surfaces stay read-only. Generated assets/content require
license/provenance and the function-specific QA gate before promotion, and a
human retains the release go/no-go. This is not a production-ready engine, a
Godot replacement, or an autonomous shipping pipeline. Issues #1 and #23 remain
open.

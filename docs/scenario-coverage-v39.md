# Scenario Coverage v39 — Multi-Agent Production Pipeline Regression Suite

Status: **regression suite**. This locks Multi-Agent Production Pipeline v1
behavior under #1 Era H Milestone 42 so a breaking change fails CI. It reuses
the existing test/coverage harness; it is **not** a new engine, runtime, or
writer. Every assertion is over **state and shape only** — no flaky or
timing-based checks, no network, and no live browser. Scenario Coverage
numbering continues from the Era F sequence (v33+).

## What it locks

The suite composes the already-merged contracts:

- the role-agent model and per-artifact ownership (#1675, `production_roles`);
- handoff artifacts and deterministic conflict resolution (#1676,
  `production_handoff`);
- reviewer/critic promotion gates over the Milestone 22 trust gradient (#1678,
  `production_review_gates`);
- the deterministic demo (#1679);

plus a **backward-compatibility golden**: the existing **single-agent**
evolve campaign and safe source-apply flows remain valid after the multi-agent
additions.

## Enumerated scenarios

`examples/production-pipeline-v1/scenario-coverage-v39/matrix.fixture.json`
enumerates the locked behaviors. Each scenario records an `id`, `system`,
`kind`, and `expect`. The systems covered are `roles`, `handoff`, `gates`,
`demo`, and `backcompat`.

| System | Locked behavior |
| --- | --- |
| `roles` | single-owner ownership; non-owner proposal rejected; direct trusted write rejected; duplicate ownership fails closed |
| `handoff` | clean handoff accepted; concurrent edits conflict and block deterministically; stale ref is needs-fix; declared/computed mismatch fails closed |
| `gates` | blocked-until-reviewed; critic veto blocks; medium/high risk requires critic approval; promote-allowed never auto-applies; self-approval rejected |
| `demo` | the demo gate progresses from blocked to promote-allowed only after review |
| `backcompat` | the existing single-agent evolve campaign and safe source-apply remain valid |

## Fixtures and runner

- `examples/production-pipeline-v1/scenario-coverage-v39/matrix.fixture.json` —
  the enumerated scenario matrix.
- `roles.fixture.json`, `handoff.fixture.json`, `gates.fixture.json` —
  regression fixtures covering ownership, handoff/conflict, and reviewer/critic
  gate states/shapes.
- `backcompat.single-agent.golden.json` — the backward-compatibility golden; it
  references existing single-agent `evolve-campaign-v1` and
  `safe-source-apply-demo-v1` fixtures and the runner validates each through its
  existing contract.
- `crates/ouroforge-core/tests/scenario_coverage_v39_multi_agent_production_pipeline.rs`
  — the coverage runner.

## Reproduce

```bash
cargo test -p ouroforge-core --test scenario_coverage_v39_multi_agent_production_pipeline
```

## Boundary

The suite is inert local evidence. It asserts states and shapes, never
subjective quality. Role agents, generation, and any browser/Studio surface emit
proposals only, through the existing review/apply/trust-gradient path; there are
no direct trusted writes, no auto-apply, no auto-merge, no self-approval, and no
reviewer bypass. Browser and Studio surfaces stay read-only, and a human retains
the release go/no-go. This is not a production-ready engine, a Godot
replacement, or an autonomous shipping pipeline. Issues #1 and #23 remain open.

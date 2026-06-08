# Scenario Coverage v51: Curation Cockpit Regression Suite

Issue: #1855
Anchor: #1 Era J Milestone 57 (Candidate Generation and Curation Cockpit v1)

Scenario Coverage v51 locks the curation cockpit contract with deterministic
state/shape checks only. It covers N-variant candidate generation, human
selection provenance replay, read-only curation projections, a negative
trusted-write drift fixture, the fixture-scoped demo, and a Milestone 30
single-proposal backward-compatibility golden.

The suite is local and fixture-scoped. It does not run a live browser, use the
network, assert timing, mutate trusted sources, auto-apply fixes, auto-merge,
self-approve, or claim automated fun/quality/release/production/Godot parity.
Browser/Studio surfaces remain read-only. Generated runs/artifacts remain
untracked unless fixture-scoped. Issues #1 and #23 remain open.

## Matrix

`examples/curation-cockpit-v1/scenario-coverage-v51/matrix.fixture.json`
enumerates these rows:

| Row | Surface | Expected state/shape |
| --- | --- | --- |
| `V51.generation.n_variant` | #1852 candidate generation | four candidates, each proposed/pending/unverified and proposal-only. |
| `V51.curation.selection` | #1853 selection provenance | selected candidate replays by candidate-set id, proposal id, and payload digest. |
| `V51.curation.read_model` | #1853 read model | only `inspect-candidates` and `record-selection-provenance` actions are exposed. |
| `V51.curation.readonly_block` | #1853 negative fixture | trusted write/apply authority is rejected fail-closed. |
| `V51.demo.smoke` | #1854 demo | deterministic three-candidate demo with recorded selection. |
| `V51.m30.backcompat` | Milestone 30 generative front door | single grid-puzzle proposal remains valid, proposal-only, and provenance-linked. |

## Reproducibility

Run:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v51_curation_cockpit
```

The runner recomputes generation/curation state from fixtures and checks the
Milestone 30 golden path. It intentionally avoids flaky assertions and subjective
fun, quality, release, or market judgments.

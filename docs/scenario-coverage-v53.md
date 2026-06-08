# Scenario Coverage v53: Narrative Assist Regression Suite

Issue: #1867  
Anchor: #1 Era J Milestone 59 (Narrative and Theme-Arc Authoring Assist v1)

Scenario Coverage v53 locks the Narrative Assist v1 candidate generation,
human-selected integration, and deterministic demo contracts with state/shape
checks only. It also carries a backward-compatibility golden for the existing
Milestone 39 narrative system so narrative assist remains additive rather than a
replacement narrative engine.

The suite is local and fixture-scoped. It does not run a live browser, use the
network, assert timing, mutate trusted sources, auto-apply fixes, auto-merge,
self-approve, or claim automated fun, tone quality, production readiness,
market demand, or Godot parity. Browser/Studio surfaces remain read-only.
Generated runs/artifacts remain untracked unless fixture-scoped. Issues #1 and
#23 remain open.

## Matrix

`examples/narrative-assist-v1/scenario-coverage-v53/matrix.fixture.json`
enumerates these rows:

| Row | Surface | Expected state/shape |
| --- | --- | --- |
| `V53.candidate_generation.valid` | #1864 candidate generation | valid brief regenerates a proposal-only candidate set. |
| `V53.candidate_generation.rejects_malformed` | #1864 fail-closed generation | malformed candidate count drift is rejected. |
| `V53.candidate_generation.proposal_only_boundary` | #1864 proposal-only guard | `proposalOnly=false` drift is rejected. |
| `V53.integration.human_selection` | #1865 integration provenance | human-selected candidate records ready-for-review-apply evidence without trusted write authority. |
| `V53.integration.readonly_trusted_write_blocked` | #1865 read-only guard | trusted-write/apply selection drift is rejected. |
| `V53.demo.smoke` | #1866 deterministic demo | committed demo fixtures match records recomputed from local inputs. |
| `V53.m39_narrative.backcompat` | Milestone 39 narrative system | existing narrative definition still validates and advances to the pinned read model. |

## Reproducibility

Run:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v53_narrative_assist
```

The runner reads committed fixtures, recomputes candidate and integration state,
checks malformed and trusted-write drift guards, validates the demo fixture
round trip, and confirms the Milestone 39 narrative backcompat golden. It avoids
flaky timing assertions and subjective fun, tone, quality, release, or market
judgments.

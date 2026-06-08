# Scenario Coverage v54: Release Readiness Regression Suite (#1873)

Scenario Coverage v54 locks the Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 contracts with state-and-shape-only fixtures. It is a regression suite, not a new engine or release surface.

## Coverage matrix

The coverage matrix lives at `examples/release-readiness-v1/scenario-coverage-v54/coverage-matrix.json` and enumerates:

| Row | Area | Expected state |
| --- | --- | --- |
| `v54-copilot-recommend` | Balance co-pilot | Recommendation surfaced from planted dominant-build evidence |
| `v54-copilot-approve` | Balance co-pilot | Human approved with tweak |
| `v54-copilot-reverify` | Balance co-pilot | Reverified improved with no auto-apply |
| `v54-readiness-ready` | Release readiness | Complete bundle is mechanically ready but still requires human go/no-go |
| `v54-readiness-missing-gate` | Release readiness | Missing/blocked gate blocks the bundle |
| `v54-go-no-go-record` | Release readiness | Human go/no-go record is read-only and grants no authority |
| `v54-m25-provenance-backcompat` | Backward compatibility | Milestone 25 provenance bundle remains valid |
| `v54-m44-release-provenance-backcompat` | Backward compatibility | Milestone 44 release provenance bundle remains valid |

## Runner

```bash
cargo test -p ouroforge-core --test scenario_coverage_v54_release_readiness --jobs 2
```

The runner validates committed fixtures only. It does not use a live browser, network service, timing assertion, generated run output, signing workflow, Steam upload, release button, or market-demand signal.

## Boundaries

- Rust/local owns validation and evidence composition.
- Browser/Studio surfaces remain read-only inspection surfaces.
- Assertions cover states and shapes only; there is no automated fun score.
- Balance recommendations are human-approved and re-verified; they are never auto-applied.
- The release-readiness bundle requires a separate human go/no-go record and grants no release authority, auto-merge authority, self-approval, or trusted write.
- Generated runs/artifacts remain untracked unless fixture-scoped.
- #1 and #23 remain open.

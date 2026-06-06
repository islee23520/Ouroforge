# Godot-Plus Demo Performance / Stability Budget v1

Issue: **#794**

Bounded budgets for the collect-and-exit vertical slice (not production profiler claims).

| Budget | Source | Limit / expectation |
| --- | --- | --- |
| Frame budget | `scenes/collect-and-exit.scene.json` `runtimeDebug.frameBudget` | Within scene defaults |
| Load-time | Fixture validation + e2e smoke startup | Deterministic fresh-clone smokes pass |
| Console/crash-free | Node smokes + runtime probe tests | No uncaught failures in fixture smokes |
| QA swarm stability | `qa-swarm-smoke.test.cjs` | Plan enumerates bounded workers |
| Export verification | `scaffold-audit.test.cjs` + export profile | Local web target only |

Known gaps: no shipped-game SLA, no native export perf, no hosted load testing.

**#1 and #23 remain open.**

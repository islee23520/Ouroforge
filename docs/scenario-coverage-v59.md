# Scenario Coverage v59 — M67 Read-Only Operator Cockpit

Scenario Coverage v59 records the local executor cockpit regression suite for Milestone 67.
It covers the merged M67 issue range #2002, #2003, #2004, #2005, #2006, #2008, and #2009 before the final roadmap refresh in #2011.

## Boundary contract

- Elixir/OTP = local executor control plane only.
- Rust kernel = data plane and source of truth.
- The cockpit touches Rust only through the frozen `ouroforge` CLI surface.
- The cockpit is read-only operator UX: no direct artifact, ledger, evidence, trust-gradient, apply, release, merge, or deploy writes.
- Human judgment remains mandatory for intent, taste, legal, release, and ambiguous go/no-go states.
- Scope is local single-machine only: no hosted dashboard, distributed scheduler, remote workers, or remote telemetry mirror.
- #1 and #23 remain open governance anchors.

## V59 scenario matrix

| Scenario | Capability | Evidence |
| --- | --- | --- |
| `V59.contract.boundary` | M67-1 read-only cockpit contract | `OuroforgeExecutor.OperatorCockpit.Contract` and focused ExUnit coverage |
| `V59.status.campaign` | M67-2 campaign status model | `CampaignStatus.fixtures/0` covers normal, waiting, retrying, budget-limited, backpressured, and blocked states |
| `V59.dag.frontier` | M67-3 task DAG/progress model | `TaskDAG.fixtures/0` covers dependencies, wait gates, runnable frontier, retries, skipped work, and blockers |
| `V59.runbook.copy_only` | M67-4 blocked reason/runbook surface | `Runbook.fixtures/0` renders copy-only suggestions with zero executable actions |
| `V59.telemetry.local` | M67-5 telemetry/utilization panel | `TelemetryPanel.fixtures/0` covers queue depth, concurrency, budget, retries, backpressure, and stop gates without remote telemetry |
| `V59.parity.manual_executor` | M67-6 golden parity/manual fallback panel | `ParityPanel.fixture(:matching)` confirms byte-identical manual/executor `ouroforge` CLI transcript parity |
| `V59.demo.composed` | M67-7 minimal cockpit demo | `Demo.run/1` composes all panels into one read-only local snapshot |
| `V59.boundary.no_trusted_writes` | Drift-prevention guardrail | Regression tests assert no executable actions, no trusted-write authority, no remote telemetry, and human review for ambiguous states |

## Manual parity expectation

Artifact-affecting paths must compare executor-driven output with the equivalent manual `ouroforge` CLI transcript. The M67 demo fixture uses the same `DemoCampaign` manual and executor paths and requires byte-identical transcript bytes before reporting parity.

## Deferred scope

V59 intentionally does not add a hosted dashboard, browser-executed commands, remote workers, distributed scheduling, remote telemetry, or release/apply/deploy authority. Future milestones may improve presentation, but must preserve the two-plane contract unless a new governance issue explicitly changes it.

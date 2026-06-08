# Scenario Coverage v58: Executor Autonomy Regression Suite

Issue: #1950  
Anchor: Era K Milestone 66 (Production Orchestration Executor, Studio Layer)

Scenario Coverage v58 locks the end-to-end executor-driven autonomy envelope for
Era K by composing the M63 scheduler/CLI parity, M64 supervision/budget/recovery,
and M65 concurrency/backpressure/telemetry suites. It is an Elixir `mix test`
regression suite plus the existing Rust workspace regression gate. Elixir/OTP = control plane only; Rust kernel = data plane.

The executor reaches the kernel only via the frozen `ouroforge` CLI surface
recorded in `docs/distributed-elixir-design.md`. The executor never writes
artifacts, ledgers, evidence, release state, or trust-gradient records directly,
never owns artifact truth, and never self-certifies trusted writes. Trusted
writes remain routed through review/apply/trust-gradient. The manual Rust-CLI
loop remains a tested, first-class local fallback. Local single-machine only;
distributed, hosted, and live-ops orchestration remain Layer-3 DEFER. #1 and #23 remain open.

## Matrix

| Row | Surface | Regression caught |
| --- | --- | --- |
| `V58.parity.manual_executor` | `OuroforgeExecutor.DemoCampaign` | Executor-driven CLI transcript remains byte-identical to the manual Rust CLI path. |
| `V58.supervision.recovery` | `OuroforgeExecutor.SupervisedDemo` / `LedgerRecovery` | Crash recovery resumes from Rust-owned ledger evidence without duplicate trusted writes. |
| `V58.budget.halt` | `OuroforgeExecutor.BudgetGate` | Budget/stop gates prevent new assignment and CLI drive before autonomy can continue. |
| `V58.concurrency.telemetry` | `BoundedPipeline` / `ProgressSurface` | Bounded load preserves caps, improves utilization, emits read-only telemetry, and keeps verdict bytes unchanged. |
| `V58.boundary.autonomy` | composed autonomy regression | Concept-to-release-candidate orchestration remains a local control-plane envelope, not a new data plane or release authority. |

## Reproducibility

Run:

```bash
cargo test --workspace --jobs 2
(cd studio/executor && mix test)
```

The v58 suite is test-only. It records cross-cutting autonomy coverage without
changing Rust kernel schemas, artifact semantics, evaluator decisions, or
trusted-write acceptance.

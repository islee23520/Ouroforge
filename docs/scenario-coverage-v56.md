# Scenario Coverage v56: Supervised Executor Regression Suite

Issue: #1944  
Anchor: Era K Milestone 64 (Production Orchestration Executor, Studio Layer)

Scenario Coverage v56 locks the Milestone 64 supervision, budget, retry, and
recovery behavior for the Studio executor control plane. It is an Elixir `mix
test` regression suite plus the existing Rust workspace regression gate. Elixir/OTP = control plane only; Rust kernel = data plane.

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
| `V56.supervision.crash_isolation` | `OuroforgeExecutor.WorkerSupervisor` / `Worker` | A crashing worker can restart while sibling work completes under the same local OTP supervisor. |
| `V56.supervision.restart_limit` | supervisor restart intensity | Repeated crashes stop the local supervisor instead of spinning or emitting any trusted-write result. |
| `V56.budget.stop_halts` | `OuroforgeExecutor.BudgetGate` | Budget or stop-condition halts block assignment and CLI drive before kernel calls are attempted. |
| `V56.retry.backoff` | `OuroforgeExecutor.RetryPolicy` | Retry attempts and backoff remain deterministic and halt at the configured limit. |
| `V56.recovery.resume_idempotency` | `OuroforgeExecutor.LedgerRecovery` | Resume derives completed trusted writes only from Rust-owned ledger evidence and is stable when repeated. |

## Reproducibility

Run:

```bash
cargo test --workspace --jobs 2
(cd studio/executor && mix test)
```

The v56 suite is test-only. It records regression coverage for supervised
executor behavior without changing Rust kernel schemas, artifact semantics, or
trusted-write acceptance.

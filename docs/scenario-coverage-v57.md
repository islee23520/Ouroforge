# Scenario Coverage v57: Concurrency and Telemetry Regression Suite

Issue: #1949  
Anchor: Era K Milestone 65 (Production Orchestration Executor, Studio Layer)

Scenario Coverage v57 locks the Milestone 65 bounded concurrency,
backpressure, idle-worker work-stealing, and read-only telemetry behavior for the
Studio executor control plane. It is an Elixir `mix test` regression suite plus
the existing Rust workspace regression gate. Elixir/OTP = control plane only;
Rust kernel = data plane.

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
| `V57.concurrency.bounds` | `OuroforgeExecutor.BoundedPipeline` | Worker caps and command-family caps prevent over-assignment under load. |
| `V57.backpressure.pending` | bounded admission model | Ready work beyond capacity remains pending in deterministic order instead of growing an unbounded queue. |
| `V57.work_stealing.utilization` | adaptive scheduler simulation | Idle workers pull newly-ready work, improving utilization/throughput over a fixed-pool baseline without changing verdict bytes. |
| `V57.telemetry.read_only` | `OuroforgeExecutor.ProgressSurface` / `:telemetry` | Progress events expose kernel refs and local counters only, with no trusted-write or artifact-truth authority. |
| `V57.demo.boundary` | `OuroforgeExecutor.ConcurrencyTelemetryDemo` | The demo remains a local control-plane composition over bounded scheduling plus read-only telemetry. |

## Reproducibility

Run:

```bash
cargo test --workspace --jobs 2
(cd studio/executor && mix test)
```

The v57 suite is test-only. It records regression coverage for executor
concurrency and telemetry without changing Rust kernel schemas, artifact
semantics, or trusted-write acceptance.

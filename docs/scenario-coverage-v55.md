# Scenario Coverage v55: Executor Skeleton Regression Suite

Issue: #1938  
Anchor: Era K Milestone 63 (Production Orchestration Executor, Studio Layer)

Scenario Coverage v55 locks the Milestone 63 executor skeleton after the M62
GO gate. It is an Elixir `mix test` regression suite for the Studio executor
control plane plus the existing Rust workspace regression gate. Elixir/OTP = control plane only; Rust kernel = data plane.

The executor reaches the kernel only via the frozen `ouroforge` CLI surface
recorded in `docs/distributed-elixir-design.md`. The executor never writes
artifacts, ledgers, evidence, or release state directly, never owns artifact
truth, and never self-certifies trusted writes. Trusted writes remain routed
through review/apply/trust-gradient. The manual Rust-CLI loop remains a tested,
first-class local fallback. Local single-machine only; distributed, hosted, and
live-ops orchestration remain Layer-3 DEFER. #1 and #23 remain open.

## Matrix

| Row | Surface | Regression caught |
| --- | --- | --- |
| `V55.plan.dag` | `OuroforgeExecutor.ProductionPlan` | Invalid producer-plan DAGs fail closed on missing dependencies and cycles. |
| `V55.scheduler.determinism` | deterministic ready/assignment logic | Ready sets and worker assignment remain stable; assigned work is not reassigned before completion. |
| `V55.cli.frozen_surface` | `OuroforgeExecutor.CLI` | Executor-driven kernel calls stay inside the M62 frozen CLI surface. |
| `V55.trusted_write.routing` | trusted-write adapter guard | Direct artifact/ledger writes and self-certification remain blocked. |
| `V55.demo.golden_parity` | `OuroforgeExecutor.DemoCampaign` | Executor transcript stays byte-identical to the manual CLI transcript and drift fails the test. |

## Reproducibility

Run:

```bash
cargo test --workspace --jobs 2
(cd studio/executor && mix test)
```

The optional full demo parity smoke remains available with:

```bash
(cd studio/executor && mix test --only demo)
```

The v55 suite uses deterministic fixture-shaped checks for the normal `mix test`
path so it can run as a fast regression gate without mutating kernel artifacts.

# Executor Autonomy Assessment v1

Issue: #1951  
Era: K — Production Orchestration Executor (Studio Layer)  
Date: 2026-06-08

## Verdict

Era K is complete on merged evidence through Scenario Coverage v58 (#1950 / PR
#1984), with this governance refresh recording the outcome. The executor moves
Ouroforge's autonomy from an externally operated/ad-hoc loop to a first-class
local Studio control plane, while preserving the two-plane invariant:

- **Elixir/OTP = executor control plane only**: schedule, supervise, budget,
  retry, apply backpressure, and emit read-only telemetry.
- **Rust kernel = data plane**: artifact semantics, schemas, verdicts, ledgers,
  evidence, review/apply/trust-gradient acceptance, and release truth.
- The executor reaches the kernel only through the frozen `ouroforge` CLI surface.
- The executor never writes artifacts/ledgers/evidence directly, never owns
  artifact truth, never self-certifies, and never releases.
- The manual Rust-CLI loop remains a tested local-first fallback.
- Distributed/multi-machine, hosted/cloud, servers/databases, and live-ops remain
  Layer-3 DEFER under ADR #92 / Milestone 45 / #1508.

## Evidence chain

| Milestone | Scope | Merged evidence |
| --- | --- | --- |
| M62 | Design gate, two-plane contract, frozen CLI surface, GO recommendation | #1933 / PR #1957 |
| M63 | Executor skeleton, scheduler, CLI drive, golden-parity demo, Scenario Coverage v55 | #1934–#1938 / PR #1958, #1959, #1960, #1962, #1967 |
| M64 | Supervision, budgets, retry/recovery, supervised demo, Scenario Coverage v56 | #1939–#1944 / PR #1968, #1970, #1972, #1974, #1976, #1978 |
| M65 | Bounded concurrency/backpressure, read-only telemetry, load demo, Scenario Coverage v57 | #1945–#1949 / PR #1979, #1980, #1981, #1982, #1983 |
| M66 | End-to-end autonomy regression coverage v58 and governance refresh | #1950 / PR #1984, #1951 / this refresh |

## Autonomy measurement

This is a descriptive control-plane measurement over the Era K bounded local
campaign shape. It is not a release-readiness, quality, taste, legal, market, or
hosted-operations claim.

| Step family | Manual/ad-hoc path | Executor-driven path | Human retained? |
| --- | --- | --- | --- |
| Intent framing / taste / legal / release go-no-go | Human decides | Human decides | Yes |
| Plan DAG consumption and ready-set ordering | Human/operator or scripts | Executor scheduler | No |
| Worker assignment under load | Human/operator or scripts | Bounded pipeline | No |
| CLI invocation | Human/operator runs commands | Executor CLI adapter runs frozen `ouroforge` commands | No, after approved inputs |
| Golden parity check | Human/operator compares outputs | Regression/demo asserts byte parity | Human reviews failures |
| Crash isolation/restart | Human/operator restarts | OTP supervision + bounded restarts | Human reviews blocked states |
| Budget and stop gates | Human/operator checks | Executor checks before assign/CLI drive | Human approves mandatory gates |
| Retry/backoff | Human/operator reruns | Executor policy determines retry/halt | Human reviews exhaustion |
| Resume after crash | Human/operator inspects ledger | Executor reconstructs from Rust ledger/evidence | Human reviews ambiguous states |
| Backpressure/concurrency caps | Human/operator throttles | Bounded local admission control | Human tunes config |
| Progress visibility | Human/operator reads scattered logs | Read-only progress/`:telemetry` surface | Human interprets state |
| Trusted write/release acceptance | Human/review/apply/trust-gradient | Still human/review/apply/trust-gradient | Yes |

Operational-control-plane actions automated by Era K: **9 of 12 step families**
(75%) in the bounded local concept-to-release-candidate envelope. The retained
human families are the intended ones: intent/taste/legal/release judgment,
mandatory human gates, and review of blocked or ambiguous states. Trusted writes
remain outside executor authority.

## Boundaries and non-claims

Era K does **not** claim:

- distributed or multi-machine orchestration;
- hosted/cloud execution, servers/databases, live ops, or public launch
  automation;
- production readiness, quality/fun/legal certification, or market acceptance;
- direct Studio/browser trusted writes;
- executor-owned artifact truth;
- auto-merge, self-approval, reviewer bypass, or release authority.

## Roadmap consequence

Era K may be marked complete as a local Studio control-plane layer. Future work
should treat it as an optional orchestration convenience over the Rust kernel,
not a second data plane. Any expansion to distributed/hosted/live-ops requires a
new design gate and explicit human approval. #1 remains the broad roadmap anchor
and #23 remains the repo-memory/design context anchor; both remain open.

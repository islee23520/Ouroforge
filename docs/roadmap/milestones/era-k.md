### Era K: Production Orchestration Executor (Studio Layer)

Added 2026-06-08. Eras A–J build the verified loop, the genre-vertical game, the safety/provenance envelope, and the human creative loop, and they model multi-agent production as auditable artifacts and gates (Milestones 42–43). But the component that actually *drives* the producer→role-agent loop end-to-end — the orchestration **executor** — has stayed external and ad-hoc (the autonomous pipeline and operator-run agent sessions). Era K makes that executor a first-class, supervised runtime: the "studio" layer that sits *above* the Rust verification kernel, runs and supervises the role/producer agents, enforces budgets and gates, and routes every trusted write through the existing review/apply/trust-gradient path. This is what moves the north-star **autonomy** term from *modeled* to *operational* — the fraction of the concept→release pipeline that runs without human action.

**Guiding principle for Era K.** Two planes, strictly separated. The **Rust kernel owns the data plane** — seed/evidence/verdict/mutation semantics, the gates, determinism, and artifact truth — unchanged. The **executor owns only the control plane** — scheduling, worker supervision, budgets, retries, backpressure, telemetry — and never owns or defines any artifact's meaning. The executor *proposes and produces*; humans give intent and approve gates; trusted writes flow only through review/apply/trust-gradient. The executor must be optional: a fresh checkout must still run the full loop manually via the Rust CLI without it (local-first fallback).

**Language decided; build timing gated.** The executor's control-plane language is Elixir/OTP (decided 2026-06-08); the Rust kernel is unchanged. Milestone 62 reopens ADR #92 (`docs/distributed-elixir-design.md`), scoped to a *local single-machine* executor, and decides only the build **GO/DEFER timing** by evidence: build proceeds once the manual/ad-hoc orchestration shows measurable pain (operator-authored task assignment, manual restart/budget/conflict handling, hand-rolled supervision reinventing OTP). Distributed/multi-machine, hosted/cloud, and live-ops remain Layer-3 and DEFER per ADR #92 and Milestone 45/#1508 — Era K is single-machine and local-first.

**Boundaries (all prior boundaries unchanged; Era K additions in bold):**
- **The executor is control plane only; all artifact creation, validation, and gating stays in the Rust kernel and is reached only through the `ouroforge` CLI. The executor never defines or writes artifact semantics.**
- **Trusted writes and releases go only through review/apply/trust-gradient and the human go/no-go; the executor never self-certifies, auto-merges, or releases.**
- **The executor is optional; the manual Rust-CLI loop remains a tested, first-class fallback.**
- **Multi-machine/distributed, hosted/cloud, and live-ops stay Layer-3 (ADR #92 / Milestone 45) — Era K is local single-machine.**
- Rust-first kernel; conservative public wording; no "ships games autonomously" claim.

#### Milestone 62: Orchestration Executor Scope, ADR #92 Revisit, and Build-Timing Gate (Design-Gate-First)
Goal: with the control-plane language decided (Elixir/OTP), decide on evidence *whether and when* to build the executor, and fix the two-plane contract — without building it prematurely.
Target deliverables: a design-gate ADR (reopening #92, scoped to a local single-machine executor) that records the Elixir/OTP control-plane decision, defines the two-plane contract (Rust data plane vs Elixir/OTP control plane) and the exact `ouroforge` CLI surface the executor may call; measurable build triggers derived from current manual orchestration (operator-authored assignment frequency, manual restart/budget/conflict-resolution incidents, supervision-logic complexity) with a GO/DEFER timing decision; a preserved, tested local-first manual fallback contract.
Success criteria: the ADR records the Elixir/OTP control-plane decision and a documented GO/DEFER timing with evidence; the two-plane boundary and the "executor never owns artifact truth / never self-certifies" invariants are specified; DEFER remains the default absent trigger evidence; #1 and #23 remain open.

#### Milestone 63: Executor Skeleton and Rust-CLI Drive Contract
Goal: a minimal executor that drives a multi-step production campaign end-to-end through the kernel, replacing one ad-hoc operator loop.
Target deliverables: an executor that consumes the Milestone 43 production plan and its dependency graph, assigns ready tasks to a worker pool, and performs every engine action only via the `ouroforge` CLI (run/evaluate/review/apply); trusted writes routed exclusively through review/apply/trust-gradient; a loop-produced demo driving a small bounded campaign with a complete audit trail; a Scenario Coverage v55 regression suite.
Success criteria: a small campaign runs end-to-end via the executor producing the same artifacts/verdicts as the manual path (golden parity); no trusted write bypasses the gates; the manual fallback still works; deterministic and replayable.

#### Milestone 64: Supervised Workers, Budgets, Retry/Backoff, and Crash Recovery
Goal: make the executor robust for game-scale, long-horizon runs.
Target deliverables: worker lifecycle supervision (spawn, crash isolation, restart strategy) for the role/producer agent processes; runtime enforcement of the Milestone 43 budgets and stop conditions; retry/backoff on agent/task failure; resume-from-ledger recovery after an executor crash; a demo and a Scenario Coverage v56 regression suite.
Success criteria: a worker crash is isolated and recovered without corrupting artifacts; budgets/stop conditions halt a campaign safely; an interrupted campaign resumes from the ledger with no duplicate or lost trusted writes; reversibility preserved.

#### Milestone 65: Concurrency Control, Backpressure, and Live Telemetry Surface
Goal: scale and observe many concurrent production/QA tasks safely.
Target deliverables: bounded concurrency and backpressure across queued tasks; a read-only live telemetry/progress surface derived from kernel artifacts (no trusted writes, no artifact-truth ownership); adaptive utilization (idle workers pull ready work); a demo and a Scenario Coverage v57 regression suite.
Success criteria: concurrency stays within configured bounds under load; the telemetry surface reflects kernel artifacts and performs no trusted writes; utilization improves over a fixed-pool baseline without changing verdict bytes.

#### Milestone 66: Executor-Gated Autonomy Assessment, Roadmap and #1 Governance Refresh
Goal: record Era K completion and measure the operational autonomy uplift.
Target deliverables: an autonomy assessment comparing executor-driven vs manual campaigns (the fraction of concept→release run without human action, with humans retained on intent, taste, legal, and release); reaffirmation of the two-plane invariant and the ADR #92 / Layer-3 boundaries; a #1 completion comment; a Scenario Coverage v58 regression suite.
Success criteria: the roadmap and #1 reflect actual Era K completion with evidence; the human-judgment boundary and the kernel-owns-truth invariant are reaffirmed; #1 and #23 remain open.

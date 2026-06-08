# Distributed / Elixir Design Review

Issue: #92 — Distributed / Elixir Design Review

Status after DE1.1: **sufficiency audit draft only**. This document contains the
Rust/local evidence audit and open architecture questions. The final ADR-style
GO/NO-GO decision is intentionally deferred to DE1.2.

## 1. Current local-first architecture summary

Ouroforge currently treats local artifacts as the source of truth:

```text
Seed -> Run -> Ledger + Evidence -> Scenario Results -> Verdict -> Journal -> Mutation artifacts
```

Rust owns the authoritative artifact contracts and CLI commands. Browser and UI
code are local/static boundaries for capture, runtime probes, and read-only
inspection. Generated run state stays under `runs/` and is not committed.

Current merged product tracks through #91 provide:

- Runtime v1 local browser-game capabilities;
- Scenario/Evaluator v1 deterministic evidence capture and comparison;
- Evolve Loop v1 proposal/classification/draft/sandbox/rerun/review artifacts;
- Studio v1 read-only dashboard/cockpit surfaces over exported dashboard data.

Open GitHub issues at the start of this audit were #1, #23, and #92 only. #1 is
the roadmap source of truth and #23 is bot-owned memory; #92 is the only active
design implementation issue.

## 2. Current Rust worker-pool capability summary

The current worker pool is a local Rust/CDP browser-smoke pool:

- `BrowserSmokePoolConfig` rejects zero workers and expands a base
  `BrowserSmokeConfig` into one worker config per worker.
- `WorkerId` maps each worker to an isolated evidence directory such as
  `evidence/workers/worker-1`.
- Multi-worker runs create independent CDP page targets and spawn Rust threads to
  run browser smoke capture.
- Worker outcomes include `worker_id`, `ok`, optional `screenshot_path`, and
  optional error text.
- Pool results summarize `workers`, `succeeded`, `failed`, and ordered outcomes.
- Worker setup failures append ledger events with `browser.worker.failed` and the
  failing phase.
- `cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4` starts a
  local static runtime server, launches local headless Chrome with a local CDP
  port, runs the browser smoke pool, then executes scenarios, evaluator, journal,
  and evolve/no-op update as local Rust-owned artifact steps.

Observed verification across Runtime/Scenario/Evolve/Studio tracks repeatedly
passed with `--workers 4`. The most recent #91 post-merge verification produced:

- Run ID: `run-1780383623789-6914`
- Workers: 4/4 succeeded
- Scenarios: 1/1 passed
- Verdict: `passed`

## 3. What the current Rust pool is sufficient for

The current Rust/local pool is sufficient for the current MVP and v1 tracks:

- reproducible local Seed -> Run -> Evidence -> Verdict flows;
- local browser smoke parallelism within one machine;
- per-worker evidence isolation under one run directory;
- deterministic Scenario/Evaluator verification;
- local comparison/evolve artifacts;
- read-only Studio inspection of exported run data;
- local PR verification without a server, database, cloud service, or account
  system.

No merged evidence currently proves that the local Rust pool is insufficient for
Ouroforge's present private MVP or the completed v1 milestones.

## 4. Concrete limitations and evidence gaps

The current Rust pool has limits that may matter for a later distributed QA
milestone, but they are not yet proven blockers:

1. **Single-machine execution** — Workers are local threads and local Chrome/CDP
   targets. There is no remote worker registration, queue, or supervision model.
2. **Process supervision is minimal** — The MVP command starts local Python and
   Chrome child processes and tears them down, but it does not model long-running
   worker lifecycles, restarts, backoff, heartbeats, or remote crash recovery.
3. **Coordination is command-scoped** — Runs are one CLI invocation. There is no
   durable coordinator for multi-hour or multi-agent iteration beyond local
   artifacts.
4. **Telemetry fanout is artifact-only** — Evidence is persisted as files. There
   is no live telemetry fanout, subscription model, or distributed observer.
5. **Remote QA clusters are absent** — There is no protocol for assigning Seeds
   to remote nodes, collecting results, or reconciling remote artifacts into a
   canonical local run.
6. **Live ops is not a current product surface** — The project has no hosted
   runtime, accounts, cloud environment, or production game service that would
   require live supervision.

These are possible future gaps, not current insufficiency evidence. Any Elixir
adoption must map to one or more observed unmet needs above.

## 5. Candidate BEAM supervision use cases to evaluate in DE1.2

Potential BEAM use cases are credible only if a later milestone needs them:

- **Distributed worker orchestration**: assign local Rust QA commands to multiple
  remote nodes while collecting artifact manifests.
- **Long-running agent coordination**: supervise multi-step evaluate/evolve runs
  that outlive one CLI process and need retries/heartbeats.
- **Telemetry fanout**: broadcast run progress/events to observers without making
  telemetry canonical state.
- **Supervised remote QA clusters**: restart failed remote browser workers,
  isolate failing nodes, and preserve artifact handoff contracts.
- **Live ops**: only if a future issue introduces an actual hosted/local-live
  service boundary; no such boundary exists today.

DE1.1 does not recommend implementing these. It only records where BEAM might be
relevant if evidence proves the Rust-only local model insufficient.

## 6. Alternatives that remain Rust/local-first

Before introducing Elixir, these alternatives should be considered:

1. **Continue Rust-only local orchestration** — Current default. Best preserves
   local-first reproducibility and avoids new runtime complexity.
2. **Improve Rust process pool** — Add bounded child-process supervision,
   heartbeats, retries, or worker leases inside Rust if failures remain local.
3. **Shell-level workers** — Use documented shell/tmux/CI fanout for occasional
   parallel verification without changing Ouroforge's product architecture.
4. **External queue/service** — Rejected for now unless a future issue proves a
   server/cloud boundary is required; it would add stronger drift risk than
   Elixir.
5. **Future Elixir supervision** — Reserve for a concrete distributed
   orchestration milestone where BEAM supervision is solving observed remote or
   long-running coordination failures.

## 7. Rust artifact contract preservation requirements

Any future orchestration layer must treat Rust artifacts as canonical:

- Seed
- Run
- Ledger
- Evidence
- Journal
- Verdict
- mutation artifacts
- evaluator logic
- CLI/local file integrity

An orchestration layer may invoke Rust commands or move artifact bundles, but it
must not own artifact semantics, rewrite evaluator logic, or weaken local file
integrity. Canonical state remains JSON/Markdown/file artifacts produced and
validated by Rust.

## 8. Local-first compatibility requirements

Local-first operation must remain possible even if future orchestration is added:

- `cargo run -p ouroforge-cli -- run ... --workers N` must remain a valid local
  path for MVP verification.
- Local runs must not require a server, database, Elixir node, cloud account, or
  networked coordinator.
- Remote/distributed orchestration, if ever adopted, must be optional and
  artifact-compatible with local runs.
- Generated artifacts must remain inspectable from a checkout without hidden
  service state.

## 9. Operational risk analysis

Primary risks of premature Elixir adoption:

- introducing distributed architecture before a concrete distributed QA need;
- creating server/database/cloud assumptions contrary to the local-first MVP;
- splitting artifact ownership between Rust and another runtime;
- encouraging production/live-ops claims that the project does not support;
- increasing verification burden with another language/runtime;
- confusing command-scoped reproducibility with long-running service state.

Countermeasure: DE1.2 must produce an explicit GO/NO-GO ADR and may create
follow-up implementation issues only if the decision is GO.

## 10. DE1.1 audit conclusion

DE1.1 conclusion: **not yet insufficient**.

The current Rust local worker pool is sufficient for the completed MVP/v1 tracks.
The audit identifies plausible future distributed orchestration needs, but no
current evidence shows Rust local orchestration is already blocking Ouroforge.
The final recommendation remains pending until DE1.2 evaluates the boundary and
writes the explicit ADR.

## 11. Open questions for DE1.2

- Which concrete Milestone 7 scenario requires multiple machines rather than
  local workers?
- What failure mode requires BEAM restart/supervision instead of Rust child
  process supervision?
- What event stream needs live fanout rather than file-based ledger/evidence?
- How would remote artifacts be reconciled into a canonical local run directory?
- What local-only fallback command remains when the distributed layer is absent?

---

# ADR: Elixir/BEAM adoption decision

Status: **PARTIAL GO (language direction) — BUILD TIMING GATED.** Superseded in part by the 2026-06-08 update below; the original 2026-06-02 NO-GO analysis is retained as context.

Decision date: 2026-06-02 (original); 2026-06-08 (update)

## 2026-06-08 Update: Studio Executor Control Plane — language GO, build timing gated

Context that changed since 2026-06-02: the roadmap now defines a **Studio executor** (Era K, Milestones 62–66) — the production-orchestration runtime that drives the producer/role agents end-to-end. This is a **single-machine, local-first control plane**, distinct from the *distributed/multi-machine* orchestration that the original ADR rejected.

Decision:

- **Language direction: GO for Elixir/OTP as the Studio executor *control plane* only.** Elixir/OTP may own worker supervision (spawn/crash-isolation/restart), task scheduling over the production plan and its dependency graph, runtime enforcement of budgets/stop-conditions, retry/backoff, backpressure, concurrency control, and live telemetry fanout derived from kernel artifacts.
- **Build timing: still GATED on evidence (DEFER by default).** Building the executor proceeds only when the manual/ad-hoc orchestration shows measurable pain — operator-authored task assignment, manual restart/budget/conflict handling, hand-rolled supervision reinventing OTP. Milestone 62 is the design gate that records this GO/DEFER timing with evidence and the two-plane CLI contract.
- **The Rust kernel is unchanged and remains the data plane.** Elixir must never own seed/run/ledger/evidence/verdict/mutation schemas or validation, the evaluator gates, deterministic runtime/simulation, trusted-write acceptance, releases, or local file artifact contracts. The executor reaches the kernel only via the `ouroforge` CLI; the kernel validates everything; trusted writes flow only through review/apply/trust-gradient; the executor never self-certifies.
- **Still NO-GO / DEFER (unchanged):** distributed/multi-machine orchestration, hosted/cloud, servers/databases, and live-ops remain Layer-3 and deferred per Milestone 45 / #1508. Era K is local single-machine; the manual Rust-CLI loop remains a tested first-class fallback.

The remainder of this document (the 2026-06-02 NO-GO analysis and its "if adopted later, what it may own / must not own" boundaries) stands as the still-binding constraint set for everything *except* the narrowly-scoped Studio executor control plane authorized above.

## ADR question (2026-06-02, original)

Should Ouroforge introduce Elixir/BEAM now for distributed orchestration?

## Decision

Do **not** introduce Elixir/BEAM implementation in the current roadmap state.

The evidence shows the current Rust local worker pool is sufficient for the
completed local MVP and v1 tracks. The project has plausible future distributed
coordination needs, but those needs are not yet concrete enough to justify adding
a second runtime, supervision tree, server assumptions, or remote execution
architecture.

Elixir remains a reserved future option only after a later milestone produces
specific evidence that Rust local/process orchestration is insufficient.

## Why Rust local orchestration is not currently insufficient

Current merged evidence demonstrates local runs with four browser workers,
scenario execution, evaluator verdicts, journals, mutation artifacts, comparison,
and static Studio inspection. The current problem is not failed local
orchestration; it is preserving reproducible local artifact contracts while the
engine matures.

No current issue requires:

- multi-machine worker assignment;
- remote browser worker restart policies;
- long-running coordinator heartbeats;
- live telemetry subscribers;
- hosted live ops;
- distributed artifact reconciliation;
- server/database/cloud deployment.

Without one of those concrete needs, adding Elixir would be an architectural
pre-optimization.

## Rejected BEAM supervision use cases for now

These use cases are rejected for current implementation, not permanently:

| Use case | Current decision | Reason |
| --- | --- | --- |
| Distributed worker orchestration | Reject now | No remote QA node protocol or multi-machine failure evidence exists. |
| Long-running agent coordination | Reject now | Current workflows are command-scoped and artifact-backed; no multi-hour coordinator failure mode is proven. |
| Live ops | Reject now | Ouroforge has no hosted runtime, user accounts, production service, or live game operation surface. |
| Telemetry fanout | Reject now | Ledger/evidence files are sufficient for current inspection; no live subscriber requirement exists. |
| Supervised remote QA clusters | Reject now | Local `--workers N` evidence passes; no remote cluster or node failure evidence exists. |

## If Elixir is adopted later, what it may own

If a future issue reopens this decision with concrete evidence, Elixir may own
only orchestration boundaries such as:

- supervising remote QA worker processes that invoke Rust CLI commands;
- queueing artifact-producing jobs for remote/local workers;
- heartbeats, restart policy, and backoff for long-running coordinators;
- telemetry fanout of run progress events derived from Rust-owned artifacts;
- aggregating remote worker status before handing artifacts back to Rust-owned
  validation/import paths.

Elixir may coordinate work, but it must not define the semantic meaning of any
Ouroforge artifact.

## What Elixir must not own

Elixir must not own or replace:

- Seed schema or validation;
- run directory layout;
- ledger event schema;
- evidence artifact schema or file integrity rules;
- journal rendering semantics;
- verdict/evaluator logic;
- mutation proposal/classification/patch/review artifact semantics;
- CLI/local run contract;
- browser rendering, physics, frame loop, deterministic simulation, scene schema,
  runtime probes, or comparison semantics;
- source-code patch acceptance, commit, merge, or release decisions.

Rust remains the harness kernel and canonical artifact owner.

## Rust artifact contract preservation plan

A future orchestration layer may call Rust commands and may move complete artifact
bundles, but artifact validity must be checked by Rust. Any remote result must be
importable into the same file-based contracts used by local runs:

1. Rust creates or validates the Seed and Run identity.
2. Workers produce evidence into isolated artifact bundles.
3. Rust validates paths and schema before accepting evidence into a run.
4. Rust writes ledger, verdict, journal, comparison, and mutation artifacts.
5. Static UIs read exported dashboard data; they do not depend on Elixir state.

Elixir state must be treated as operational state, not product truth.

## Local-first compatibility plan

The local path remains mandatory:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
```

Future distributed orchestration, if any, must be optional. A fresh checkout must
remain able to validate Seeds, run local browser workers, produce evidence,
evaluate verdicts, render journals, inspect dashboards, and execute scene edit
validation without Elixir, OTP, Phoenix, a database, cloud credentials, or a
network coordinator.

## Alternatives selected for now

Selected current path:

1. Continue Rust-only local orchestration.
2. If local failures appear, improve Rust child-process/process-pool supervision
   first.
3. Use shell/CI/tmux fanout for occasional human-operated parallel verification.
4. Revisit Elixir only after concrete distributed QA evidence exists.

External queues, hosted services, databases, and cloud orchestration remain
rejected unless a future issue explicitly changes the product boundary.

## Revisit criteria

Reopen this decision only if at least one of these evidence-backed triggers
appears:

- local `--workers N` is insufficient because required QA must run across
  multiple machines;
- repeated long-running orchestration failures require supervised restart/backoff
  beyond reasonable Rust process management;
- a future milestone requires live progress fanout to multiple observers and file
  artifacts alone are demonstrably insufficient;
- remote QA nodes exist and need assignment, heartbeat, quarantine, and artifact
  handoff;
- local-first fallback remains defined and tested even with distributed
  orchestration.

A revisit issue must include concrete failure logs, run IDs, or operational
requirements. Abstract scale concerns are not enough.

## Follow-up implementation issues

None.

Because this ADR is **NO-GO for implementation now**, #92 creates no Elixir,
OTP, Phoenix, distributed worker, server, database, cloud, or remote execution
implementation issues.

## Final recommendation

Final recommendation: **NO-GO now; reserve Elixir for a future evidence-backed
distributed orchestration milestone.**

This preserves the current local-first MVP, keeps Rust artifact contracts
authoritative, avoids premature distributed architecture, and leaves a clear path
to revisit BEAM only when concrete remote/supervision needs exist.

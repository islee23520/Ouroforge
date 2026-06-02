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

# Ouroforge Architecture

> Extracted from #1 body during roadmap restructuring.

### Two-Plane Architecture (Kernel vs Studio Executor)

A persistent invariant across all Eras, made explicit here because later Eras add orchestration:

- **Rust verification kernel = data plane.** Owns artifact truth: seed/run/ledger/evidence/verdict/mutation schemas, the evaluator gates, the deterministic runtime, and all trusted-write validation. This never moves and is never self-certifying.
- **Studio executor = control plane (Era K).** May schedule, supervise, retry, budget, and observe the producer/role agents, but **never owns or defines any artifact's meaning** and **never performs a trusted write or release directly** — those flow only through review/apply/trust-gradient and the human go/no-go.
- **Posture shift (Era M, 2026-06-09):** Studio surfaces move from *read-only* to *read + gated-write* — humans may actively intervene at defined points, but every intervention is a validated, recorded proposal/constraint/directive through the existing gates, never a raw bypass. The interactive Studio is **local-first Phoenix LiveView** (Elixir control + presentation plane); hosted/multi-user is Layer-3 DEFER.

Eras A–J build the kernel, the genre-vertical game, the safety/provenance envelope, the agent-coordination *scaffolding* (Milestones 42–43, modeled as auditable artifacts and gates), and the human creative loop. The component that actually *drives* the multi-agent loop end-to-end — the orchestration **executor** — is currently external and ad-hoc; **Era K** makes it a first-class, supervised runtime. The executor's control-plane language is **Elixir/OTP — decided 2026-06-08** (control plane only; the kernel stays Rust). Milestone 62 decides only the build **GO/DEFER timing** by evidence (see ADR #92 and the Language Boundary Charter).


## Language Boundary Charter

Ouroforge should use each language where it is strongest, but the first private MVP must remain local-first and Rust-first.

### Rust owns initial core work

Rust is the default language for:

- harness kernel
- Seed / Run / Ledger / Evidence / Journal / Verdict models
- CLI
- evaluator
- mutation proposal storage
- evolve loop v0
- deterministic runtime core when native/runtime code begins
- file artifact integrity
- local orchestration and testable core logic

### TypeScript / JavaScript owns browser and UI surfaces

TypeScript or JavaScript is appropriate for:

- browser runtime probe scripts
- browser demo pages
- dashboard UI
- Godot-like authoring UI
- Chrome/CDP glue only if the implementing issue proves it is simpler and does not weaken Rust-owned artifact contracts

### Elixir owns the Studio executor control plane (Era K)

Decision (2026-06-08): Elixir/OTP is the chosen language for the **Studio executor control plane** — the production-orchestration runtime that schedules, supervises, budgets, retries, and observes the producer/role agents (Era K). This fixes the *direction*; the *build timing* stays evidence-gated at Milestone 62 (ADR #92 revisit). The Rust kernel is unchanged.

Elixir/OTP (control plane only) may own:

- producer/role-agent worker supervision (spawn, crash isolation, restart)
- task scheduling over the production plan and its dependency graph
- runtime enforcement of budgets and stop conditions
- retry/backoff, backpressure, and concurrency control
- live telemetry/progress fanout derived from kernel artifacts

Elixir must **never** own — these stay in the Rust kernel, reached only via the `ouroforge` CLI:

- rendering, physics, frame loop, deterministic simulation core
- seed/run/ledger/evidence/verdict/mutation schemas and validation
- the evaluator gates, trusted-write acceptance, and releases
- local file artifact contracts and the harness kernel

Still deferred (Layer-3, ADR #92 / Milestone 45): distributed/multi-machine orchestration, hosted/cloud, and live-ops. Era K is **local single-machine and local-first**; the manual Rust-CLI loop remains a tested fallback.

### Python is non-core unless explicitly justified

Python may be used only for one-off tooling, research scripts, or temporary migration helpers. It must not own core runtime contracts, harness semantics, or evaluator logic unless a later issue explicitly changes this charter.

### Language drift rule

If an issue does not explicitly authorize a language/runtime, default to Rust for core/harness work and TypeScript/JavaScript for browser/UI work. Any new language introduction requires an issue update explaining why the current authorized languages are insufficient.

---


## Suggested Repository Structure

```text
ouroforge/
  README.md
  docs/
    vision.md
    architecture.md
    evidence-native.md
    ouroboros-integration.md
    roadmap.md
  crates/
    ouroforge-core/
    ouroforge-ledger/
    ouroforge-evidence/
    ouroforge-harness/
    ouroforge-evaluator/
    ouroforge-runtime/
    ouroforge-cli/
  apps/
    studio/
    dashboard/
  examples/
    pong/
    platformer/
    topdown-shooter/
  seeds/
  journals/
  evidence/
```

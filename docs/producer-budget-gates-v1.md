# Producer Budgets, Stop Conditions and Human Approval Gates v1

Issue: #1685. Status: additive Rust/local contract.

`producer-budget-gates-v1` builds on Whole-Game Orchestration State v1 and
reuses the evolve-campaign/fuzz budget and stop-condition shape at game scale. It
adds hard iteration/cost budgets, a no-progress window, declared stop conditions,
and mandatory human approval gates. It is not a new control engine, worker
runtime, hosted orchestrator, browser command bridge, writer, or release system.

The policy evaluates to one of four read-only states:

- `continue` when the producer is inside budget, human gates are approved, and
  the no-progress window is not reached;
- `halted-budget-exhausted` with an evidence-linked diagnosis;
- `blocked-human-gate` while a mandatory human gate is pending;
- `stopped-no-progress` when the reused trailing no-progress window is reached.

Human approval gates are mandatory for vision, legal, and release decisions.
They block progress; they do not grant direct trusted writes, auto-apply,
auto-merge, self-approval, reviewer bypass, or shipping authority. Any trusted
write remains routed through the existing review/apply/trust-gradient path and
human release go/no-go.

Browser, dashboard, and Studio surfaces may display the read model only. Rust and local evidence own validation. Generated producer budget state, runs, assets,
content, coverage, and local artifacts remain untracked unless explicitly
fixture-scoped. Public wording stays conservative: no production-ready claim,
Godot replacement/parity claim, quality/fun guarantee, or autonomous shipping
claim is introduced.

Issues #1 and #23 remain open.

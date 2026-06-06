# Multi-Iteration Evolve Campaigns v1 scope and contract

Multi-Iteration Evolve Campaigns v1 upgrades Ouroforge's evolve path from a
single proposal-then-rerun step into a bounded, ordered sequence of evolve
iterations that converge toward a Seed's acceptance criteria with a full audit
trail. It remains local-first, artifact-first, sandboxed, and manual-review
gated except where the Milestone 22 trust gradient already permits bounded
auto-apply.

This document is the canonical control artifact for issues #1486 through #1492
under #1 Era E Milestone 23 (#23). It adds no product behavior by itself; each
follow-up GitHub issue remains the implementation contract for its own PRs. The
campaign metric defined here is descriptive: it reports how an evolve sequence
progressed, not a quality, correctness, or production-readiness guarantee.

Related control documents:

- `docs/architecture.md` for the Seed -> Run -> Ledger + Evidence -> Scenario
  Results -> Verdict -> Journal -> Mutation Proposal loop;
- `docs/evolve-loop-v1.md` and `docs/evolve-loop-depth-v1.md` for the single
  evolve iteration (proposal -> apply-or-propose -> rerun) this campaign
  sequences;
- `docs/adversarial-input-fuzzing-plan-v1.md` for the bounded stop-condition and
  budget pattern this campaign reuses;
- `docs/source-mutation-rollback-audit-v1.md` and the Milestone 22 trust
  gradient for the bounded auto-apply / manual-review boundary;
- `docs/scenario-evaluator-v1.md` for the four-gate verdict and before/after
  comparison boundaries each iteration consumes.

## 1. Purpose and relation to #1 and #23

Issue #1 defines Ouroforge as an evidence-native game engine built around local,
inspectable Ouroboros loops. Issue #23 anchors Era E Milestone 23. Multi-Iteration
Evolve Campaigns v1 supports that final goal by chaining individually bounded
evolve iterations so an operator can observe convergence toward a Seed's
acceptance criteria as an auditable sequence rather than a series of disconnected
single steps.

A campaign must answer only bounded questions:

- Which Seed acceptance criteria is the campaign converging toward?
- For each iteration, what hypothesis was tested, what four-gate verdict delta
  resulted, and what mutation was proposed next?
- Did the sequence reach acceptance, exhaust its iteration/cost budget, or stop
  on no-progress?
- When it did not converge, what evidence-linked diagnosis explains the stop?
- Which iterations were manual-review and which fell within the Milestone 22
  bounded auto-apply budget?

A campaign must not answer unbounded questions such as whether a change should
auto-merge, leave local review outside the bounded budget, or whether a semantic
quality improvement exists without artifact evidence. A campaign never loops
unbounded: every campaign has explicit stop conditions and budgets.

#1 and #23 remain open governance anchors. This contract does not close or modify
them; #1492 handles the roadmap/governance refresh as a separate explicit step.

## 2. Reused mechanisms (no duplicate engines)

The campaign is a thin orchestration layer over mechanisms that already exist in
`crates/ouroforge-core`. It introduces no new evolve, stop-condition, verdict, or
journal engine.

- **Evolve iteration**: reuses the evolve proposal/apply/rerun path
  (`EvolveSummary`, `evolve_run`, and the rerun-orchestration references such as
  `EvolveRerunOrchestration` / `EvolveRunReference`). A campaign iteration is one
  pass of this existing path.
- **Stop conditions**: reuse the adversarial-input fuzzing stop-condition and
  budget shape (`FuzzStopCondition` with `conditionId` + `description`,
  `FuzzBudget`, and the `Planned` / `Blocked` / `Exhausted` status pattern). The
  campaign declares its stop conditions in the same descriptive style; it does
  not invent a parallel termination model.
- **Four-gate verdict delta**: reuse the existing run comparison gate delta
  (`fourGate` / `fourGateDeltas` / `RunGateDelta`). Per-iteration convergence is
  measured from these deltas, not a new scoring function.
- **Journal**: reuse Journal v2 and its four-gate delta rendering. The campaign
  narrative (#1489) extends the existing summarizer with per-iteration entries
  and a final summary; it does not fork the journal engine.
- **Trust gradient**: reuse the Milestone 22 bounded auto-apply / rollback /
  high-risk-blocker boundary. The campaign does not widen any auto-apply
  authority.

## 3. Campaign model

A campaign is an ordered, finite sequence of evolve iterations directed at one
Seed's acceptance criteria.

- **Campaign**: `{ campaignId, seedRef, acceptanceTarget, budget, stopConditions,
  iterations[], outcome }`. All identifiers are path-safe; all references are
  artifact-relative.
- **Iteration**: an ordered entry `{ index, hypothesis, mutationRef,
  fourGateVerdict, verdictDelta, decision }` where:
  - `hypothesis` states, in bounded text, what the iteration expected to improve;
  - `mutationRef` links to the proposal/patch-draft artifact produced by the
    reused evolve path;
  - `fourGateVerdict` is the iteration's four-gate verdict for the rerun;
  - `verdictDelta` is the four-gate delta versus the previous iteration (or the
    baseline for the first iteration);
  - `decision` records whether the mutation was manual-review or fell within the
    Milestone 22 bounded auto-apply budget.
- **Baseline**: iteration index 0 establishes the starting four-gate verdict; a
  campaign with no baseline iteration is invalid.

The campaign performs no mutation logic of its own: each iteration delegates the
proposal -> apply-or-propose -> rerun work to the existing evolve path and only
records the resulting references and deltas.

## 4. Stop conditions

A campaign terminates on exactly one declared stop condition, reusing the fuzz
stop-condition descriptive shape:

- **acceptance-reached**: an iteration's four-gate verdict satisfies the Seed's
  `acceptanceTarget`. The campaign records the passing iteration and stops.
- **budget-exhausted**: the iteration count or cost budget is reached before
  acceptance. The campaign stops with a not-converged outcome and a diagnosis.
- **no-progress**: a bounded window of iterations shows no qualifying four-gate
  delta improvement. The campaign stops with a not-converged outcome and a
  diagnosis.

Every campaign must declare at least the acceptance-reached condition and an
explicit budget; a campaign without a stop condition, with a malformed budget, or
with zero iterations is invalid and is rejected before execution. Stale or
unresolvable refs (Seed, mutation, evidence) are rejected, never silently
skipped.

## 5. Convergence tracking and budgets

Convergence is a descriptive running state computed from per-iteration four-gate
verdict deltas:

- each iteration records its four-gate verdict delta versus the prior iteration;
- the running convergence state is **converged** once an iteration reaches the
  acceptance target, otherwise **not-converged**;
- the campaign emits one outcome artifact: **converged** (naming the passing
  iteration) or **not-converged** (with the diagnosis and the last recorded
  deltas).

Budgets are hard limits. Iteration and cost budgets are enforced on every step;
on exhaustion the campaign terminates with a not-converged outcome rather than
continuing. Convergence is never a quality or correctness guarantee — it states
only that the recorded four-gate verdict reached (or did not reach) the declared
acceptance target within budget.

## 6. Safe non-convergence

Non-convergence is a first-class, safe outcome, never an unbounded loop. When a
campaign stops on budget-exhausted or no-progress it must produce an
evidence-linked diagnosis: which gate(s) failed to converge, the last recorded
four-gate deltas, and references to the justifying evidence artifacts. The
operator can inspect the diagnosis and decide whether to extend the budget,
revise the hypothesis, or stop. The system itself does not retry beyond the
declared budget.

## 7. Trust-gradient relationship

Campaign iterations honor the Milestone 22 trust gradient unchanged:

- an iteration's mutation is **manual-review** by default;
- an iteration may **auto-apply** only when the mutation falls within the
  Milestone 22 bounded auto-apply risk budget (rollback-backed); outside that
  budget the iteration stays manual-review;
- the campaign never widens auto-apply authority, never auto-merges, and never
  performs unsupervised unbounded looping.

Each iteration's `decision` field records which path was taken so the audit trail
shows exactly where bounded auto-apply was used.

## 8. Ownership, compatibility, and generated state

- **Ownership**: Rust/local owns campaign orchestration, budgets, convergence
  tracking, and serialization. The browser surface is read-only; it may render
  campaign artifacts but makes no trusted decisions.
- **Backward compatibility**: all artifacts are additive. Existing evolve,
  verdict, and journal contracts remain valid; a single-shot evolve (a one-step
  campaign or the pre-campaign path) continues to work unchanged.
- **Generated state**: campaign runs and their artifacts are untracked unless
  they are explicitly fixture-scoped for a demo or regression suite.
- **Public wording**: conservative. No auto-merge, quality, production-readiness,
  or Godot-replacement claim is made anywhere in this contract or its follow-ups.

## 9. Follow-up issue sequence and closure gates

```text
#1486 scope -> #1487 campaign model + stop conditions
       -> #1488 convergence tracking + budget -> #1489 journal narrative
       -> { #1490 demo, #1491 coverage v24 } -> #1492 governance refresh
```

1. **#1486 Scope and Contract** (this issue): defines the campaign model, stop
   conditions, convergence tracking, budgets, non-convergence handling, and the
   trust-gradient relationship. Closes when this doc lands.
2. **#1487 Campaign Model and Stop Conditions v1**: implements the bounded
   iteration sequence and acceptance/budget/no-progress termination, reusing the
   fuzz stop-condition pattern. Closes when `evolve_campaign_contract.rs` and the
   termination fixtures pass.
3. **#1488 Convergence Tracking and Budget v1**: implements per-iteration delta
   tracking, converged/not-converged outcome, and hard budget enforcement.
   Closes when `evolve_campaign_convergence_contract.rs` and the
   converged/not-converged/no-progress fixtures pass.
4. **#1489 Journal Narrative v1**: extends Journal v2 with per-iteration entries
   and a final summary. Closes when `evolve_campaign_journal_contract.rs` and the
   narrative/not-converged fixtures pass.
5. **#1490 Demo v1**: deterministic converging and non-converging demo against
   playable-demo-v2. Closes when the demo fixtures, doc, and smoke test pass.
   Depends on the Milestone 22 Rollback-Backed Auto-Apply path for the
   auto-apply portion of the audit trail.
6. **#1491 Scenario Coverage v24**: regression suite locking termination,
   convergence, and narrative behaviors plus a single-shot-evolve
   backward-compat golden. Closes when the v24 runner and `docs/scenario-coverage-v24.md`
   land.
7. **#1492 Governance Refresh**: roadmap and #1 refresh, handled as a separate
   explicit governance decision. #1 and #23 remain open anchors throughout.

This contract adds no executable behavior. It only defines the boundaries the
follow-up issues implement.

### Era E: Loop Generalization and Trustworthy Autonomy

Added 2026-06-06. Eras A–D (and the 2026-06-05 Roadmap Alignment Addendum: Evaluator Depth 4.1, Evolve Loop Depth 5.1, Foundation Hardening A.H — all completed) bring the evidence loop to the point where it can produce, evaluate (four gates), and evolve a **single** small game class (Signal Gate / `examples/playable-demo-v2/collect-and-exit`). That proves the loop *works*; it does not yet prove the loop *generalizes to real games* or that the harness is *fully auditable and trustworthily autonomous*. Era E closes that distance.

Guiding principle for Era E: the durable advantage is the **evidence-backed loop as the primary development primitive**, not engine feature breadth. Engine capability grows only as a downstream consequence of a loop-produced game class demanding it (Milestone 24), never as a goal in itself. The north-star metric is **loop coverage × game complexity × trust**: what fraction of a game's trusted changes are produced and verified through the loop (rather than manual edits), at increasing game complexity, with reversible and audited autonomy.

Boundaries unchanged from Eras A–D: Rust-first and local-first; Studio surfaces read-only; conservative public wording; and Layer-3 scope (real native export, plugin runtime, hosted/cloud, distributed orchestration / Elixir per the `docs/distributed-elixir-design.md` ADR #92 NO-GO) remains deferred until evidence demands it — re-evaluated only at Milestone 26.

#### Milestone 20: Loop Coverage Metric and Authoring-Fraction Instrumentation

Goal: make "the loop builds the game" a measured, auditable fact rather than a narrative claim.

Target deliverables:

- a first-class loop-coverage evidence artifact per project/run that computes the fraction of trusted changes/artifacts produced and verified through the evidence loop (Seed → Build → Observe → Verify → Journal → Evolve) versus manual edits;
- attribution of each trusted artifact to its provenance class (loop-produced, loop-verified, or manual);
- a coverage verdict surfaced read-only in the dashboard/Studio and recorded with each run;
- regression detection when loop coverage drops.

Success criteria:

- Every demo run reports a loop-coverage verdict.
- A drop in loop coverage is detectable as a regression.
- The metric is computed by Rust/local validation, not asserted by prose.

#### Milestone 21: Second Game Class and Loop Generalization

Goal: prove the same loop produces a *different* game genre end-to-end, de-risking the central thesis more than any single feature.

Target deliverables:

- a second playable demo of a distinct game class (for example a one-screen platformer) generated, evaluated, and evolved through the same Seed → … → Evolve loop, alongside the existing collect-and-exit class;
- evidence that the loop required no bespoke per-game escape hatches;
- a comparable evidence shape (verdicts, journal, coverage) across both game classes.

Success criteria:

- Two distinct game classes are produced by the same loop with comparable evidence and loop coverage (Milestone 20).
- Generalization gaps surface as explicit, evidence-linked findings rather than silent manual patches.

#### Milestone 22: Trust Gradient Design Gate, then Bounded Rollback-Backed Auto-Apply

Goal: introduce a *graduated* autonomy step without weakening the safety posture. Today every trusted write is review-gated and "no auto-apply" is a hard, pervasive boundary; this milestone decides whether and how a narrow, reversible exception is ever allowed.

This milestone is **design-gate-first**, following the project's idiom for risky capabilities (compare ADR #92, Native Export Design Gate #168, the source-mutation design gate). No auto-apply behavior is implemented until the gate returns GO.

Target deliverables:

- a design-gate ADR (`docs/trust-gradient-design.md`) producing an explicit GO/NO-GO on bounded auto-apply, defining the risk-tier model, what may ever auto-apply (low-risk data/scene mutations only), and what must never;
- if GO: a risk-tier classifier; a rollback-backed bounded auto-apply path constrained by an explicit risk budget, applied only when a mutation is low-risk, high-confidence, and passes all four gates on rerun; an append-only audit log of every auto-applied change; a one-command rollback and an emergency kill switch;
- high-risk and source-affecting changes remain manual-review-gated regardless of the gate outcome.

Success criteria:

- A clear GO/NO-GO decision exists with documented criteria; the default remains "no auto-apply" unless GO.
- If GO: a low-risk mutation can auto-apply, is fully audit-logged, and is reversible with one command; risk-tier gating is enforced; high-risk never auto-applies; the kill switch halts autonomy.

#### Milestone 23: Multi-Iteration Evolve Campaigns

Goal: move evolve from a single proposal-then-rerun into bounded multi-iteration campaigns that converge toward an acceptance target with a full audit trail.

Target deliverables:

- an evolve campaign model that runs multiple bounded iterations toward a Seed's acceptance criteria, with convergence tracking, iteration/cost budgets, and explicit stop conditions (reusing the existing fuzzing stop-condition pattern);
- a campaign journal narrative linking each iteration's hypothesis, verdict delta, and next mutation;
- safe non-convergence handling: the campaign stops at budget with a diagnosis rather than looping unbounded.

Success criteria:

- A failing game converges to passing acceptance over N bounded iterations with a complete audit trail.
- Non-convergence stops safely at the declared budget with an evidence-linked diagnosis.
- Campaign autonomy honors the Milestone 22 trust gradient (manual-review unless within the auto-apply risk budget).

#### Milestone 24: Game Complexity Ladder (Demand-Driven, Capability-Gated)

Goal: make "increasing game complexity" explicit and gated, so engine capability grows only to satisfy the next loop-produced game class — never as feature-chasing.

Target deliverables:

- a documented complexity ladder of game classes (for example collect-and-exit → one-screen platformer → top-down → multi-scene/objective), each defined as a capability gate;
- a rule that each rung requires a loop-produced, evidence-backed demo before the next rung is claimed, and that any new engine capability (renderer/physics/audio/animation depth) must be justified by a specific rung's gate;
- coverage and verdict evidence per rung.

Success criteria:

- Each climbed rung has a loop-produced demo with passing four-gate evidence and a loop-coverage verdict.
- New engine capability lands only with an explicit rung justification; the roadmap does not pre-authorize broad engine breadth.

#### Milestone 25: End-to-End Provenance Bundle and Audit Surface

Goal: complete the "auditable" half of the thesis by unifying the existing provenance pieces (scene/transaction provenance, rollback metadata, evidence links, review/regression promotion) into one end-to-end chain a human can review and replay.

Target deliverables:

- a per-change provenance bundle spanning the full chain — intent/design brief → generated or edited artifact → trusted validation → runtime observation → evaluator verdict → regression comparison → journal/review decision → promotion/rollback;
- a read-only audit surface presenting one bundle per trusted change for human sign-off;
- replayability: any merged game change can be reconstructed and re-verified from its evidence.

Success criteria:

- Any merged trusted game change can be replayed from its provenance bundle.
- A reviewer can audit the full intent-to-promotion chain for a change in a single read-only surface.
- The bundle reuses existing provenance/evidence contracts (no parallel re-implementation).

#### Milestone 26: Era E Roadmap and #1 Refresh, and Layer-3 Re-evaluation Trigger

Goal: record Era E completion and decide, on evidence, whether Layer-3 scope is now warranted.

Target deliverables:

- a roadmap and #1 governance refresh recording Milestones 20–25 as completed only after merged evidence exists;
- an evidence-based re-evaluation of Layer-3: whether real native export, plugin runtime, hosted/cloud, and distributed orchestration / Elixir (ADR #92) are now demanded by the loop's generalization needs, with an explicit GO/DEFER decision and criteria;
- preservation of #1 and #23 as open anchors.

Success criteria:

- The roadmap and #1 reflect actual Era E completion with evidence.
- A documented GO/DEFER decision on Layer-3 exists; deferral remains the default unless evidence demands otherwise.

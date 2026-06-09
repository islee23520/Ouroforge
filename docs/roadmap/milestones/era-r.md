### Era R: Interrogated Semantic Re-Derivation (Legacy Logic → Verified Deterministic Behavior)

Added 2026-06-09. Era R is what turns the import on-ramps (O/P/Q) from "skeleton importers" into *working ports*. Logic porting is **not translation but re-derivation**: `interrogate → surface tacit knowledge → decompose into small behavioral units → capture an oracle (acceptance criteria) → re-express deterministically → differentially verify → iterate`. The agent drives it, the evidence loop gates it, and the human resolves only intent/feel. This reframes porting from an undecidable "prove semantic equivalence automatically" problem into a tractable, bounded, **per-unit verified re-derivation** — and is uniquely aligned with Ouroforge's seed/scenario/evidence oracle. It is also **legally cleaner** (clean-room re-implementation from observed behavior + interrogated intent, not copied decompiled code).

**Guiding principle for Era R.** Re-derivation, not translation; a unit is "ported" only when it passes a captured oracle; differential verification is at the *observable-outcome* level (the source is non-deterministic); feel/intent/fun is a human (Ring-2) escalation; everything runs through the existing gates (review/apply, evaluator, source-apply, evidence). Bounded by oracle quality and effort, with coverage made visible (no silent "fully ported").

**Boundaries (Era R additions in bold):**
- **Output is a fresh deterministic re-implementation from observed behavior + interrogated intent (clean-room) — never copied/translated decompiled source.**
- **A unit is only "ported" when it passes captured acceptance evidence; no oracle ⇒ flagged, not claimed.**
- **Differential verification is outcome-level, not bit-exact (source physics/timing is non-deterministic); feel/intent/fun is human-judged (Ring 2).**
- **Source-project + consented only; no shipped-build ripping; semantic-port coverage is tracked and honestly reported.**
- Rust = data plane (re-expression target + validation); Elixir/Phoenix = interrogation/UX surface; reuse Era L/M and the generation path; no new data plane.

#### Milestone 107: Re-Derivation Methodology ADR and "Re-Derivation ≠ Translation" Contract (Design-Gate-First)
Goal: fix the unit model, the oracle requirement, outcome-level differential verification, the human-feel escalation boundary, and the clean-room/source-only legal contract.
Target deliverables: a methodology ADR (behavioral-unit definition, "no oracle ⇒ not ported", clean-room re-derivation procedure, O/P/Q hand-off contract, differential-verification semantics).
Success criteria: the methodology, oracle rule, verification semantics, and legal boundary are specified; #1/#23 remain open.

#### Milestone 108: Legacy Logic Ingestion and Behavioral-Unit Extraction (Read-Only)
Goal: analyze source logic to know what behaviors exist and their engine coupling — not to translate.
Target deliverables: source-C# parsing (+ IL2CPP signature recovery as a degraded fallback), a behavior/call graph, engine-API touchpoint identification, behavioral-unit boundaries; a Scenario Coverage v90 regression suite.
Success criteria: behavioral units and their couplings are catalogued read-only; no translation occurs; deterministic analysis.

#### Milestone 109: Tacit-Knowledge Interrogation and Oracle Capture
Goal: surface each unit's implicit intent/invariants/edge-cases/timing assumptions via questions, and convert answers + observed behavior into Ouroforge-native acceptance criteria.
Target deliverables: a per-unit interrogation loop (Ouroboros/deep-interview-style) capturing answers as provenance; oracle synthesis into seeds/scenarios/golden evidence; a Scenario Coverage v91 regression suite.
Success criteria: units acquire captured oracles; intent is recorded as provenance; units without an oracle are explicitly flagged.

#### Milestone 110: Deterministic Re-Expression Engine
Goal: re-author each unit as deterministic Ouroforge behavior through the gated path.
Target deliverables: an agent-driven re-expression (behavior_runtime) via source-apply + the generation path, with non-deterministic dependencies re-derived as intent; a Scenario Coverage v92 regression suite.
Success criteria: units are re-expressed deterministically and applied only through the gates; reversible and provenance-tracked.

#### Milestone 111: Differential Verification (Behavioral A/B)
Goal: verify each re-derived unit against its captured oracle and the source's observable behavior at the outcome level.
Target deliverables: outcome-level A/B verification (non-determinism-tolerant), mismatch flagging, rollback on regression; a Scenario Coverage v93 regression suite.
Success criteria: a unit passes only when it matches its oracle/observable behavior; mismatches are flagged with evidence; no bit-exact claim for non-deterministic source.

#### Milestone 112: Semantic-Port Coverage and Convergence Tracking
Goal: make port progress and residual cost visible, and iterate to convergence.
Target deliverables: a semantic-port coverage metric (ported/verified vs pending vs human-feel-escalated) extending loop-coverage attribution; a residual backlog; a Scenario Coverage v94 regression suite.
Success criteria: coverage honestly reports what is and isn't re-derived; convergence loop terminates on pass or human escalation; no silent "fully ported".

#### Milestone 113: Re-Derivation UX and Human Intent/Feel Escalation
Goal: surface interrogation/decomposition/verification in the Studio and route intent/feel decisions to humans.
Target deliverables: a Phoenix LiveView re-derivation surface (unit progress, question/answer panels, A/B results) with an Era-M intervention escalation queue for intent/feel; a Scenario Coverage v95 regression suite.
Success criteria: the surface is read + gated-write; feel/intent decisions are human and recorded as provenance; the autonomous loop proceeds around them.

#### Milestone 114: Era R Roadmap and #1 Governance Refresh, and Port-Tractability Assessment
Goal: record Era R completion and assess how much of logic porting is now agent-driven vs human-feel residual.
Target deliverables: a port-tractability assessment (auto/assisted re-derivation fraction vs human-feel residual); reaffirmation of re-derivation≠translation, oracle-gated, outcome-level, source-only boundaries; a #1 completion comment.
Success criteria: the roadmap reflects actual Era R completion with evidence; boundaries reaffirmed; #1 and #23 remain open.

---

> **Document note (2026-06-08 restructure):** the three sections below — Definitions of Done, the Updated Non-Goal Boundary, and the 2026-06-05 Alignment Addendum — were relocated to the end so the Era A–K roadmap reads contiguously. Content is unchanged; the Addendum (Milestones 4.1 / 5.1 / A.H) refines the Era A/B foundations and is also listed in the Roadmap Index.

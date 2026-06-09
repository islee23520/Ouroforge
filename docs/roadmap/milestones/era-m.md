### Era M: Active Human Intervention (Agent-First, Human-Steerable)

Added 2026-06-09. Eras A–L make the engine produce, verify, ship, and autonomously self-improve a real title, with humans acting as gatekeepers (approve/reject, fun verdict, curation, go/no-go) over **read-only** surfaces. Era M refines the posture: **agent-first remains the default, but at defined points a human may actively intervene** — edit a proposal, hand-author an artifact, steer a live campaign, add a constraint, correct a diagnosis, or take over a stage. The one boundary change is that Studio surfaces move from **read-only** to **read + gated-write**: every human intervention is a *validated, recorded proposal/constraint/directive* routed through the existing gates (review/apply, scene/source-apply, evaluator, evidence/provenance), never a raw bypass of the gates, determinism, or audit.

**Guiding principle for Era M.** The autonomous-first default (Era L) is preserved: intervention is **opt-in at defined points and never required**; the loop still runs to completion without a human. Two planes hold: **Rust = data plane** (artifact truth, validation, determinism); **Elixir/OTP + Phoenix LiveView = control + interactive presentation plane** (local-first). Elixir captures and routes human intent but never owns or writes artifact semantics — Rust validates and records every intervention exactly as it does an agent change (gated, reversible, provenance-tracked).

**Boundaries (all prior boundaries unchanged; Era M additions in bold):**
- **Studio surfaces move from read-only to read + gated-write; every human intervention is a validated, recorded proposal/constraint/directive — never a raw write that bypasses the gates, determinism, or audit.**
- **The agent-first/autonomous default is preserved: intervention is opt-in and never required; the loop completes without a human.**
- **Rust owns the data plane (truth/validation); Elixir/OTP + Phoenix LiveView own the control + interactive presentation plane (local single-user). Elixir never owns artifact semantics.**
- **Hosted/multi-user collaborative Studio and real-time collaboration are Layer-3 and DEFER; a fresh checkout still runs the full loop via the CLI without the Studio.**
- Fun/taste verdict and release go/no-go remain human (Ring 2); Rust-first kernel; conservative wording.

#### Milestone 74: Active-Intervention Scope, Intervention-as-Evidence Contract, and Studio Stack Decision (Design-Gate-First)
Goal: fix the intervention points, the invariant that every intervention is validated/recorded (never a raw bypass), the read-only→read+gated-write posture shift, and the Studio surface stack.
Target deliverables: a design-gate ADR cataloguing the intervention points (amend, author, steer, constrain, correct, takeover) and the gated path each reuses; the "intervention-as-evidence" invariant; the posture-shift decision (read-only → read + gated-write); the **Phoenix LiveView (local single-user) decision** for the interactive Studio surface, with the hosted/collab DEFER boundary; the preserved local-first CLI fallback contract.
Success criteria: the ADR records the posture shift, the intervention-as-evidence invariant, and the Phoenix LiveView (local) decision; DEFER for hosted/collab; the autonomous-first default and CLI fallback are reaffirmed; #1 and #23 remain open.

#### Milestone 75: Proposal Amendment and Re-Verify (Amend-Before-Approve)
Goal: let a human edit an agent's proposed change and re-run the full gates before apply.
Target deliverables: capture of human edits to a proposal (diff/config); re-verification of the amended proposal through the four gates + design-integrity; before/after evidence; routing through review/apply; a demo and Scenario Coverage v66.
Success criteria: an amended proposal is re-verified and only applied on a green gate set; the edit is recorded as provenance; no amend bypasses the gates.

#### Milestone 76: Human-Authored Artifact Intake
Goal: let a human hand-author an artifact (card/scene/tuning/asset) and submit it through the same validation/gate/provenance path as agent output.
Target deliverables: a human-authoring intake for first-class artifacts; validation + scene/source-apply with `author=human` provenance; a demo and Scenario Coverage v67.
Success criteria: a human-authored artifact passes the same gates as an agent's and is recorded with human provenance; invalid input is rejected by the same validators; no bypass.

#### Milestone 77: Live Campaign Steering Directives
Goal: let a human inject mid-flight directives the executor honors, recorded as evidence.
Target deliverables: steering directives (re-prioritize, pin/exclude an approach, add a constraint, pause/resume) consumed by the Era K executor; directive validation + ledger recording; a demo and Scenario Coverage v68.
Success criteria: a directive measurably changes the live campaign and is recorded as evidence; directives are validated, not raw control; the loop continues autonomously between directives.

#### Milestone 78: Human Constraints as First-Class Gates
Goal: turn human-authored project constraints into evaluator-enforced gates the loop honors.
Target deliverables: a constraint model ("never mechanic X", "pixel-art only", budget caps) compiled into evaluator gates; violation blocking with evidence; a demo and Scenario Coverage v69.
Success criteria: a violating output is blocked by the constraint gate with evidence; constraints are recorded as evidence; constraints compose with the existing four gates.

#### Milestone 79: Diagnosis Correction and Intervention Feedback Loop
Goal: let a human correct a wrong auto-diagnosis/attribution (Era L) and have the correction improve future runs.
Target deliverables: capture of a human diagnosis/attribution override; provenance recording of the correction; a heuristic/prior update that re-attributes (not opaque ML); a demo and Scenario Coverage v70.
Success criteria: a corrected diagnosis is recorded and improves subsequent attribution; the correction is auditable; no automated fun/taste inference is introduced.

#### Milestone 80: Stage Takeover and Handback
Goal: let a human take over a specific stage manually, do the work, and hand back to the agent — with the manual work captured as evidence and re-verified.
Target deliverables: stage lock/unlock; manual-work capture as evidence + provenance; gate re-verification on handback; executor state consistency across the swap; a demo and Scenario Coverage v71.
Success criteria: a human can take over and hand back a stage with no artifact corruption; manual work is gated and recorded; the executor resumes consistently.

#### Milestone 81: Era M Roadmap and #1 Governance Refresh, and Human-Steerability Assessment
Goal: record Era M completion and assess the new posture.
Target deliverables: a human-steerability assessment (intervention coverage; that every intervention routed through the gates with no raw bypass); reaffirmation of the agent-first default, the two-plane boundary, and the read+gated-write posture; a #1 completion comment.
Success criteria: the roadmap and #1 reflect actual Era M completion with evidence; the no-bypass and agent-first-default invariants are reaffirmed; #1 and #23 remain open.

**Era M completion update (2026-06-09).** Era M is recorded as complete on merged evidence for M74-M80 plus the M81 governance refresh: M74 posture/Phoenix/two-plane design gate (#2052 / PR #2101), M75 proposal amendment (#2053-#2056 / PRs #2146, #2149, #2151, #2152; Scenario Coverage v66), M76 human-authored artifact intake (#2057-#2060 / PRs #2155, #2158, #2160, #2161; Scenario Coverage v67), M77 live campaign steering directives (#2064 / PR #2153; Scenario Coverage v68), M78 human constraints as first-class gates (#2065-#2068 / PRs #2162-#2165; Scenario Coverage v69), M79 diagnosis correction and intervention feedback (#2069-#2072 / PRs #2166, #2242, #2243, #2244; Scenario Coverage v70), M80 stage takeover and handback (#2076 / PR #2159; Scenario Coverage v71), and M81 governance (#2077). Human-steerability coverage is `6 / 6 = 100%` for the covered classes: proposal amendment, human-authored artifact intake, live campaign steering, human constraints, diagnosis correction/attribution feedback, and stage takeover/handback. The agent-first default, no-raw-bypass invariant, two-plane boundary, local-first CLI fallback, read + gated-write Studio posture, Layer-3 DEFER for hosted/multi-user Studio, and human-owned fun/taste plus release go/no-go are reaffirmed. #1 and #23 remain open.

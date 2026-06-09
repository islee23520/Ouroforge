### Era L: Autonomous Self-Validation and Improvement Loop (Real-Title Dogfooding)

Added 2026-06-08. Eras A–K make the engine able to *produce and verify* a real title and to *drive* that production with a supervised executor. Era L closes the last loop: the engine **validates and improves itself** by building a real title end to end and treating every friction point as evidence — **autonomously**. The cycle `detect → explain → trace → attribute → propose → re-verify → apply` reuses the existing evidence pipeline (openchrome run, scenario verdicts, the four gates + design-integrity gate, `journal.md`, `ledger.jsonl`, loop-coverage attribution, evolve, source-apply, trust-gradient). No new verification engine, no new data plane.

**Guiding principle for Era L (autonomy-first).** Self-debugging, self-auditing, log-tracing, self-verification, and self-improvement are the **default autonomous loop** — humans are not in the per-milestone path. The engine detects failure (verdict), explains it (journal), traces it (ledger), attributes it to a milestone (coverage attribution), proposes an engine fix (evolve/source-apply), re-verifies it through the *independent* gates (openchrome re-run), and applies it via the trust gradient. Humans remain only at the two irreducible Ring-2 touchpoints — the **fun/taste verdict** and the **high-risk irreversible go/no-go** (public release; engine self-kernel rewrite) — and even those are *thin* gates that broaden toward auto as verification strength and reversibility scale (per Milestone 44's trust gradient). Read-only human surfaces are **optional observability/override**; the autonomous loop never waits on them.

**Boundaries (all prior boundaries unchanged; Era L additions in bold):**
- **Self-improvement runs autonomously through the *existing* gates; trusted/source writes flow through source-apply + trust-gradient; only the high-risk irreversible tail keeps a thin human go/no-go, broadening toward auto per Milestone 44.**
- **Human diagnosis/feedback surfaces are optional observability/override only — the autonomous loop must run to completion without them.**
- **This loop improves the *engine* (the harness/pipeline), distinct from evolve, which improves the *game*; engine source changes are high-risk and use the source-apply gated path.**
- **Verification reuses openchrome + the evidence-native loop; no new verification engine and no new data plane are introduced.**
- The fun/taste verdict and the release go/no-go remain human (Ring 2). Layer-3 (distributed/multi-machine, hosted/cloud, live-ops) stays DEFER. Rust kernel = data plane; Elixir/OTP executor = control plane.

#### Milestone 68: Real-Title Dogfooding Campaign
Goal: validate the whole stack by autonomously producing one real, complete engine-builder deckbuilder end to end, capturing every friction point as evidence.
Target deliverables: a dogfooding campaign harness that drives the real title through the executor and openchrome QA; structured friction logging (where a stage stalled, retried, or required intervention) recorded into the ledger; the real title carried to a verified release candidate; a demo and a Scenario Coverage v60 regression suite.
Success criteria: a real title reaches a verified release candidate through the live pipeline; all friction is captured as evidence; the integrated chain is shown to compose (not just per-milestone green).

#### Milestone 69: Autonomous Cross-Milestone Self-Audit and Bottleneck Attribution
Goal: let the engine re-check each milestone against its own success criteria on the real build and attribute failures/slowdowns to specific milestones — without a human.
Target deliverables: bottleneck attribution extending loop-coverage attribution (a failed/slow step maps to a milestone/gate); a milestone-acceptance audit (a meta-evaluation that checks each milestone's success criteria against the real run's evidence); regression/trend tracking; a demo and a Scenario Coverage v61 regression suite.
Success criteria: a real-build failure is automatically attributed to the responsible milestone with evidence; each milestone is re-verified against its own criteria on the real title; the audit is descriptive and evidence-linked, not a human report.

#### Milestone 70: Autonomous Self-Diagnosis and Fix-Proposal Generation
Goal: turn an attributed failure into a root-cause hypothesis and a concrete engine fix proposal, autonomously.
Target deliverables: a root-cause diagnosis generator reading verdict/journal/ledger/attribution; a fix-proposal generator that emits source-apply proposals (engine changes) bounded by the existing high-risk classification; a demo and a Scenario Coverage v62 regression suite.
Success criteria: a planted engine defect yields an evidence-linked root cause and a concrete, scoped fix proposal; proposals are produced through the existing source-apply proposal path and never self-applied here; deterministic and replayable.

#### Milestone 71: Autonomous Self-Improvement Loop
Goal: close the loop — re-verify proposed engine fixes through the independent gates and apply them via the trust gradient, with only the high-risk tail held for a thin human go/no-go.
Target deliverables: a re-verify-then-apply loop (proposal → four gates + design-integrity + openchrome re-run on the real title → before/after verdict); trust-gradient routing that auto-applies reversible/low-risk fixes and queues high-risk/source-affecting changes for a thin human go/no-go; game-scale rollback/kill-switch reuse; a demo and a Scenario Coverage v63 regression suite.
Success criteria: a reversible low-risk engine fix is re-verified and auto-applied with no human; a high-risk fix is verified, made reversible, and queued for a one-click human go/no-go; every applied fix improves the attributed milestone's evidence and is rollback-able.

#### Milestone 72: Optional Human Oversight and Taste-Feedback Channel
Goal: provide a human-intuitive, optional observability/override surface and a fast taste-feedback capture — without ever blocking the autonomous loop.
Target deliverables: a read-only stage-health/blocker view (campaign/DAG/diagnosis/attribution) for spot-checking; a non-blocking override/escape-hatch for a stuck loop; fast taste/fun-feedback capture anchored to evidence and recorded as provenance (reusing the Milestone 57/58 human-in-loop path); a demo and a Scenario Coverage v64 regression suite.
Success criteria: the surface is read-only and performs no trusted writes; the autonomous loop completes whether or not a human looks; taste feedback is captured as provenance and never auto-applied; the override is auditable.

#### Milestone 73: Era L Roadmap and #1 Governance Refresh, and Autonomous-Loop Maturity Assessment
Goal: record Era L completion and assess how much of detect→fix→re-verify→apply now runs without human action.
Target deliverables: an autonomous-loop maturity assessment (fraction of self-validation/self-improvement cycles completed without human action, with humans retained on taste and high-risk go/no-go); reaffirmation of the autonomy-first, two-plane, and human-irreducible boundaries; a #1 completion comment; a Scenario Coverage v65 regression suite.
Success criteria: the roadmap and #1 reflect actual Era L completion with evidence; the autonomous-first default and the irreducible human touchpoints are reaffirmed; #1 and #23 remain open.

**Completion status (2026-06-09):** ✅ Era L is complete on merged evidence. M68-M72 are recorded through Scenario Coverage v60-v64 and the M73 Coverage v65/governance chain (#2046-#2047 / PR #2144-#2145), with the M70 fix-proposal evidence issue #2048 included in the diagnosis chain. The autonomous eligible low-risk self-improvement cycle completes with zero human action; high-risk/source-affecting changes are verified and queued for thin human go/no-go rather than auto-applied. No new verification engine, persistent store, telemetry schema, or data plane was introduced. Rust remains the data plane, the Elixir executor remains control plane, fun/taste and release go/no-go stay human Ring 2, and #1/#23 remain open.

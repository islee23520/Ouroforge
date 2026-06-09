# Ouroforge Roadmap

## Current status

## Era R semantic re-derivation governance

Era R starts with `docs/semantic-re-derivation-methodology-adr-v1.md` as its M107 design gate. The contract fixes the one-way on-ramp, re-derivation-not-translation boundary, behavioral-unit model, no-oracle-not-ported rule, outcome-level differential verification, deterministic 2D and 2.5D/3D state-hash requirements, source-project/open-text clean-room legal boundary, O/P/Q hand-off, and the Rust data-plane / Elixir-Phoenix control-plane split. #1 and #23 remain open.

M108 is scoped by `docs/legacy-logic-ingestion-behavioral-unit-contract-v1.md`, which defines the source-only ingestion subset, exact Rust-owned reports/IR/candidate-unit outputs, conservative fidelity grades, and the hand-off states for interrogation, oracle capture, re-expression, verification, or reject/defer.
The first M108 Rust data-plane implementation lives in `crates/ouroforge-core::legacy_logic_ingestion` and performs read-only C# / degraded IL2CPP-signature analysis into IR nodes, engine touchpoints, behavioral-unit candidates, fidelity reports, and re-derivation tasks.
Scenario Coverage v90 (`docs/scenario-coverage-v90-legacy-logic-ingestion.md`) locks the M108 regression ledger for false-clean lossy imports, no auto-port without oracle, deterministic digest/state-hash drift, and decompiled-source rejection.
M109 is scoped by `docs/tacit-knowledge-interrogation-oracle-capture-contract-v1.md`, which defines source-independent tacit intent records, deterministic oracle-capture specs, conservative Green/Yellow/Red oracle readiness grades, and the hand-off from interrogation to capture, re-expression, verification, reject, or defer.
The first M109 Rust data-plane implementation lives in `crates/ouroforge-core::tacit_oracle_capture` and converts per-unit questions, provenance answers, and observed deterministic traces into source-independent intent records, oracle specs, fidelity reports, and follow-up re-derivation tasks.
The M109 demo (`docs/tacit-oracle-capture-demo-v1.md`) records a fixture-backed captured oracle plus an explicitly oracle-less unit, preserving deterministic state-hash evidence and no-port-claim fidelity reporting.

Ouroforge is a local, evidence-native MVP. It now supports a small project
workspace loop in addition to the original run-centered demo, with hardened
run evidence fidelity, a completed Agentic Review & Regression Promotion v1
loop around proposal rationale, review decisions, review-gated scene
application, rerun comparison, regression promotion, Journal v2, and Studio
review cockpit state, a completed Agentic Loop Orchestration v1 control
layer for data-only plans, dry-run sequencing, CLI-only step execution,
recovery preflight, evidence bundles, agent handoffs, and Studio loop cockpit
inspection, a completed Engine Expressiveness v2 playable-authoring
surface for richer scene components, deterministic collision/triggers/HUD
state, collect-and-exit demo evidence, regression coverage, and read-only
Studio inspection, a completed Asset Pipeline v1 content-authoring foundation
for local asset manifests, atlas/tilemap metadata, reference integrity, runtime
loading evidence, preview/read-model evidence, Studio asset inspection, demo asset
refresh, and asset regression coverage, and a completed Visual Authoring v1 safe
local edit-draft cockpit for source-like drafts, Rust validation/preflight,
transaction previews, visual diff previews, review-gated apply evidence, demo
smoke ids, Scenario Coverage v5, read-only Studio/dashboard wording, and a
completed Production 2D Engine Core v1 milestone for bounded local 2D engine
evidence: render queue/camera/layers/tilemap integration, collision/physics,
input actions/replay, save-load/runtime-state digests, animation/VFX/audio
evidence, frame-budget profiling, a vertical-slice demo, Scenario Coverage v7,
and read-only Studio inspection, and a completed Multi-Agent Production Pipeline
v1 milestone for local evidence-gated collaboration/accountability artifacts:
role models, task boards, ownership, work packages, handoffs, shared state,
review/critic gates, QA queues, performance/regression lanes, decision ledgers,
production evidence bundles, read-only Studio/dashboard/cockpit inspection, a
deterministic demo, and Scenario Coverage v12, and a completed Autonomous Producer
and Whole-Game Orchestration v1 milestone for proposal-only game-scale production
planning and orchestration evidence: design-intent decomposition, whole-game
orchestration state, budget/stop-condition/human approval gates, a deterministic
autonomous producer demo, Scenario Coverage v40, and this conservative governance
refresh, and a completed 3D Capability Gate
v1 milestone for bounded local 3D evidence: scene graph/transform validation,
camera/projection contracts, mesh/material local refs, renderer smoke evidence,
collision/trigger evidence, animation/probe/evaluator compatibility, deterministic
3D demo/regression fixtures, normalized dashboard read models, read-only Studio
3D inspection, and a conservative governance refresh, and a completed Agentic
Scene and Level Designer v1 milestone for evidence-gated level/scene authoring:
intent and constraint models, generation plans, layout constraints, tilemap and
terrain drafts, entity/objective/encounter placement drafts, reachability and
objective proof evidence, difficulty/pacing heuristic evidence, visual/semantic
diffs, review-gated level apply records, read-only Studio inspection, a
deterministic demo, Scenario Coverage v10, and a conservative governance
refresh, and a completed Gameplay Scripting / Logic System v1 milestone for
structured data-only gameplay logic: behavior models, deterministic
event/signal contracts, state-machine and ability/action contracts, script and
sandbox design gates, runtime integration, scenario assertions, behavior drafts,
review-gated apply records, evidence/journal integration, escaped read-only
Studio/dashboard inspection, a deterministic demo, Scenario Coverage v9, and a
conservative governance refresh, a completed GDD-to-Playable Prototype v1
milestone for bounded evidence-gated prototype generation from a GDD: design
brief validation, requirement extraction, mechanics/core-loop mapping,
feasibility, scaffold/scene/behavior/asset/scenario plans, prototype task graph,
review-gated draft/apply records, run/evidence/journal bundles, read-only Studio
planning inspection, deterministic demo evidence, Scenario Coverage v11, and a
conservative governance refresh, a completed Evaluator Depth v1 milestone
that adds bounded visual and semantic evaluator gates, additive four-category
verdict/Journals, read-only inspection, deterministic demo evidence, Scenario
Coverage v19, and conservative governance, a completed Evolve Loop Depth v1 milestone
that consumes the four-gate evaluator foundation so failed mechanical, runtime,
visual, and semantic evidence can produce evidence-linked, failure-classified,
bounded data/scene/scenario proposals, compare before/after reruns, surface the
loop read-only in Studio, prove the lifecycle with deterministic demo fixtures,
and lock Scenario Coverage v20, and a completed Safe Source Mutation Apply v1
milestone for review-gated trusted source apply over explicitly allowed low-risk
source-like file classes: threat model, trusted-worktree boundary, source apply
transaction model, stale target/hash guard, independent review enforcement,
sandbox-to-trusted promotion readiness, rollback snapshot/recovery metadata,
allowlisted verification runner, post-apply rerun/comparison evidence,
dependency/CI/build-script blockers, append-only audit ledger, evidence bundle,
read-only Studio/dashboard inspection, deterministic demo, Scenario Coverage
v14, emergency hold/kill-switch, and conservative governance refresh, and a
completed Build / Export / Packaging v1 local export foundation: export target
matrix, export profile, dry-run export plan, deterministic staging, local web
bundle assembly, asset manifest/path
rewriting, runtime probe preservation, checksums/provenance, export verification
evidence, fixture-scoped Scenario Coverage v15, read-only inspection, and
explicit release/publish blockers, and a completed Foundation Hardening v1
architecture hygiene milestone that extracted `ouroforge-ledger`,
`ouroforge-evidence`, and `ouroforge-evaluator` behind the existing
`ouroforge-core` public facade while preserving golden verdict parity, and a
completed Full Studio Editor v1 milestone for a bounded local Studio authoring
foundation: integrated overview, scene/entity/component inspection, visual scene
canvas, draft-only authoring, Safe Source Apply handoff preview, asset,
evidence, scenario, export, and plugin inspection panels, workspace persistence,
command palette, accessibility/performance/diagnostics coverage, integrated
demo, Scenario Coverage v17, and conservative governance refresh, and a
completed Godot-Plus Demo Game v1 milestone for the Signal Gate local vertical
slice: a small playable 2D vertical slice, Studio inspect/draft workflow,
agentic iteration evidence, Safe Source Apply chain evidence, QA swarm evidence
and QA/playtest swarm evidence, local web export/package verification, plugin
descriptors used as metadata, Godot-plus comparison matrix,
documentation/reproducibility notes, performance budget evidence, regression
suite, and conservative governance refresh, and a completed Plugin / Extension
System v1 declarative extension foundation: a declarative plugin manifest
schema, local registry/discovery, an allowlisted extension point catalog,
capability/permission and version-compatibility validation, descriptor evidence
integration, a read-only Studio plugin browser and read-only
dashboard/scenario/asset descriptors, a fixture plugin pack, a security/threat-
model gate that fails closed on executable/credentialed/network capabilities,
load-order/conflict detection, CLI inspection, Scenario Coverage v16, a
deterministic demo, and conservative governance refresh, and a completed
Loop Coverage Metric v1 / Era E Milestone 20 governance chain: scope and
contract, provenance attribution, Rust/local computation/verdict/regression,
read-only Studio and dashboard inspection, fixture-scoped demo evidence,
Scenario Coverage v21, and this roadmap/#1 handoff, and a completed Second Game
Class and Loop Generalization v1 / Era E Milestone 21 governance chain: scope and
contract, second game seed and GDD, loop-produced second game implementation,
loop generalization gap evidence, a deterministic demo, Scenario Coverage v22,
and this roadmap/#1 handoff, and a completed Trust Gradient v1 / Era E
Milestone 22 governance chain: a GO design gate for bounded, reversible, audited,
default-off auto-apply (narrow scope only), a risk-tier classifier,
rollback-backed bounded auto-apply, an audit log and kill switch, read-only
Studio inspection, a deterministic demo, Scenario Coverage v23, and this
roadmap/#1 handoff, and a completed Multi-Iteration Evolve Campaigns v1 / Era E
Milestone 23 governance chain: scope and contract, campaign model and stop
conditions, convergence tracking and budget, journal narrative, a deterministic
demo, Scenario Coverage v24, and this roadmap/#1 handoff, and a completed Game
Complexity Ladder v1 / Era E Milestone 24 governance chain: scope and contract,
ladder model and capability gates, engine-growth demand justification, a
fixture-scoped rung demo, Scenario Coverage v25, and this docs-only roadmap/#1
handoff, and a completed End-to-End Provenance Bundle and Audit Surface v1 /
Era E Milestone 25 governance chain: parent scope, additive bundle model,
local replayability, read-only audit surface, deterministic fixture demo,
Scenario Coverage v26, and this conservative roadmap/#1 refresh:

```text
project manifest -> scaffold/scene/seed/scenario pack -> run -> evidence -> compare -> proposal/review/application -> regression promotion/matrix -> loop plan/dry-run/step/handoff -> expressive scene/demo regression -> asset manifest/loading/preview/regression -> visual edit draft/preflight/diff/review evidence -> Production 2D runtime/replay/profiling/regression -> multi-agent role/task/handoff/review/QA/regression/evidence bundle -> journal/Studio inspection -> bounded 3D capability evidence/read-only inspection -> evidence-gated agentic scene/level design -> structured gameplay logic -> bounded GDD-to-prototype planning/draft/apply/evidence -> four-gate evaluator depth/evidence -> evidence-linked bounded evolve depth proposals/rerun deltas -> review-gated Safe Source Mutation Apply evidence -> local web export/package evidence -> Foundation Hardening v1 crate seams with golden parity -> Full Studio Editor v1 local authoring cockpit evidence/governance -> Signal Gate Godot-Plus Demo Game v1 evidence/governance -> declarative Plugin / Extension System v1 descriptor/evidence foundation -> Loop Coverage Metric v1 attribution/computation/read-only inspection/demo/v21 regression/governance -> Second Game Class and Loop Generalization v1 signal-gate seed/GDD/loop-produced build/generalization evidence/demo/v22 regression/governance -> Trust Gradient v1 design-gate GO/risk-tier classifier/rollback-backed bounded auto-apply/audit log + kill switch/read-only inspection/demo/v23 regression/governance -> Multi-Iteration Evolve Campaigns v1 campaign model + stop conditions/convergence tracking + budget/journal narrative/demo/v24 regression/governance -> Game Complexity Ladder v1 rung gates/engine-growth demand/demo/v25 regression/governance -> End-to-End Provenance v1 additive bundle/replay/read-only audit/demo/v26 regression/governance
```

The trusted boundary remains Rust and the local filesystem. Browser examples
read exported JSON and show copyable commands; they do not execute commands,
write trusted files, accept mutations, or act as a production editor. Browser
and CDP observations are evidence inputs, not trusted authority.

Evaluator Depth v1 (#1279/#1283-#1288) is now complete as the Milestone 4.1
four-gate evaluator foundation. The merged evidence chain added bounded visual
gate fixtures, declared semantic invariant fixtures, additive four-category
`verdict.json` and Journal summaries, read-only dashboard/Studio inspection, a
deterministic demo, and Scenario Coverage v19. This completion is mechanical and
declared-invariant evaluation only: visual judgment is bounded screenshot diff
evidence rather than aesthetic quality, semantic judgment is over declared
invariants rather than general AI correctness, and no auto-fix/apply/merge,
trusted mutation, command bridge, release-readiness claim, or engine-replacement
claim is introduced.

Evolve Loop Depth v1 (#1290/#1292-#1298) is now complete as the Milestone 5.1
roadmap-alignment milestone. The merged evidence chain added per-gate,
evidence-linked mutation proposal rationale; failure-classification-driven
proposal selection; bounded data/scene/scenario mutation type mapping;
four-gate before/after rerun deltas and Evolve Journal v2; read-only Studio
inspection; deterministic demo fixtures; Scenario Coverage v20; and this
governance refresh. This completion consumes the corrected Evaluator Depth v1
four-gate foundation and #215 scene-only apply path, but it does not authorize
arbitrary source patching, auto-accept, auto-apply, auto-merge, self-approval,
reviewer bypass, command bridges, production-readiness claims, or current Godot
replacement positioning. Confidence remains an evidence-derived bounded signal,
not a correctness or quality guarantee.

Safe Source Mutation Apply v1 (#699-#716) is now complete as a bounded
review-gated trusted apply path for explicitly allowed low-risk source-like file
classes. The merged evidence chain added the #699 scope contract, #700 threat
model refresh, #701 trusted worktree boundary, #702 source apply transaction
model, #703 stale target/hash guard, #704 independent review enforcement, #705
sandbox-to-trusted promotion readiness, #706 rollback snapshot/recovery
metadata, #707 allowlisted verification runner, #708 post-apply
rerun/comparison evidence, #709 dependency/CI/build-script blockers, #710 audit
ledger, #711 evidence bundle, #712 read-only Studio inspection, #713 demo, #714
Scenario Coverage v14, #715 emergency hold/kill-switch, and this #716 governance
refresh. This completion authorizes only the bounded chain proved by those
artifacts: no unrestricted source mutation, forbidden file-class changes,
dependency/CI/build-script mutation, browser trusted writes, command bridges,
auto-fix, auto-apply, auto-merge, self-approval, reviewer bypass, autonomous
source repair, secure-sandbox guarantee, production-ready mutation claim, or
current Godot replacement positioning is introduced.

Full Studio Editor v1 (#757-#776) is now complete as a local-first,
evidence-backed Studio authoring cockpit foundation under #1. The merged
evidence chain added the #757 scope contract, project overview, scene tree,
entity/component inspector, visual scene canvas, draft-only authoring model,
Safe Source Apply handoff preview, asset browser/metadata inspection,
scenario/evidence and evidence-timeline panels, export/package inspection,
plugin/extension descriptor inspection, workspace persistence, command palette,
accessibility/performance/diagnostics coverage, fixture-scoped integrated demo,
Scenario Coverage v17, and this governance refresh. This completion authorizes
only local read-only inspection plus draft-only browser state and Rust/local
trusted validation/handoff evidence. It does not authorize direct Studio trusted
writes, browser command bridges, arbitrary shell execution, dependency install,
CI/workflow mutation, credentialed operations, network install/update,
publish/deploy/sign/upload, executable plugin runtime, marketplace behavior,
autonomous apply, auto-merge, self-approval, reviewer bypass, native desktop
editor, advanced visual scripting, full asset import pipeline, timeline or
animation editor parity, tilemap/terrain editor parity, production collaboration,
production-ready editor claims, secure-sandbox guarantees, full Godot parity,
Godot replacement positioning, or the separate Godot-plus demonstration game.
Generated workspace/editor state, drafts, previews, panel data, demo outputs,
validation reports, evidence artifacts, temp servers, screenshots, dashboard
exports, and local tool state remain ignored unless a future issue explicitly
scopes deterministic fixture data. #1 remains open. #23 remains open.

Build / Export / Packaging v1 (#720-#736) is now complete as a local-only export
foundation. The merged evidence chain added the export target matrix, export
profile schema, dry-run export plan, deterministic generated-state staging,
local web bundle assembly, asset manifest/path rewriting, runtime probe
preservation, checksum/provenance primitives, package metadata, verification
coverage, read-only inspection contracts, release/publish blockers, and Scenario
Coverage v15. This completion means a bounded local web package can be assembled
and inspected as evidence; it does not mean public release, deployment, signing,
store publishing, desktop/mobile/native export, production distribution,
commercial readiness, secure distribution, multi-platform parity, or Godot
replacement status. Generated package outputs, checksums, verification logs,
screenshots, temp servers, dashboard exports, and local tool state remain
ignored unless a future issue explicitly scopes a source-like fixture.

Foundation Hardening v1 (#1301-#1306) is now complete as Milestone A.H from #1's
Roadmap Alignment Addendum. The merged chain established the golden parity
baseline (#1302), extracted `ouroforge-ledger` (#1303), extracted
`ouroforge-evidence` (#1304), extracted `ouroforge-evaluator` (#1305), and
recorded this governance refresh (#1306). The realized dependency direction is
`ouroforge-ledger <- ouroforge-evidence <- ouroforge-evaluator <-
ouroforge-core <- ouroforge-cli`; `ouroforge-core` preserves public API
compatibility through re-exports and thin facades. `crates/ouroforge-core/src/lib.rs`
was reduced from approximately 89k lines at milestone start to 89,047 lines
after #1305, with the new extracted crates measuring 96 (`ouroforge-ledger`),
130 (`ouroforge-evidence`), and 2,960 (`ouroforge-evaluator`) lines at the
#1306 audit. The `refactor_parity_golden` gate remained byte-identical across
the milestone, and full workspace tests/clippy stayed green. This was mechanical
hygiene only: no feature, behavior, serialization, release, production-readiness,
plugin/runtime, hosted/cloud, native-export, source-apply, or Godot-replacement
claim changed. Mutation, evolve, runtime, behavior, and seed remain in
`ouroforge-core` as explicit A.H2 candidates, not accepted work.

Godot-Plus Demo Game v1 (#778-#797) is now complete as a bounded local workflow
demonstration under #1. The merged evidence chain proved Signal Gate as a small
playable demo game and 2D vertical slice with deterministic gameplay evidence,
read-only or draft-only Studio inspect/draft workflow, agentic iteration
evidence, Safe Source Apply review-gated source mutation chain evidence, QA
swarm evidence and QA/playtest swarm evidence, local web export/package
verification, plugin descriptors used as metadata, a scoped Godot-plus
comparison matrix, documentation/reproducibility notes, performance budget
evidence, regression suite coverage, and this governance refresh. The
remaining gaps stay explicit: full Godot parity, mature editor tooling,
native/mobile export, large game production, real marketplace, executable plugin
ecosystem, production collaboration, commercial release readiness, production
engine maturity, secure sandbox guarantees, hosted/cloud operation, and broad
support promises. This completion does not authorize direct Studio trusted
writes, browser command bridges, arbitrary shell execution, dependency install,
network install/update, credentialed operation, unreviewed source mutation,
auto-apply, auto-merge, self-approval, reviewer bypass, public deployment,
native/mobile/console/store publishing, production-readiness claims, or current
Godot replacement positioning. Generated demo outputs, exports, QA runs,
evidence bundles, screenshots, videos, temp servers, package bundles, and local
tool state remain ignored unless a later issue explicitly scopes deterministic
source-like fixtures. #1 remains open. #23 remains open.

Plugin / Extension System v1 (#738-#754) is now complete as a bounded
declarative, allowlisted, evidence-backed extension foundation. The merged
evidence chain added the #738 scope and contract, #739 plugin manifest schema,
#740 local registry and discovery, #741 extension point catalog, #742
capability declaration and permission model, #743 version compatibility, #744
descriptor evidence integration, #745 read-only Studio plugin browser, #746
read-only dashboard panel descriptor, #747 scenario template descriptor, #748
asset metadata descriptor, #749 fixture plugin pack, #750 security/threat-model
gate, #751 load-order and conflict detection, #752 CLI inspection, #753 Scenario
Coverage v16 regression suite, and #754 deterministic demo. Plugins declare
extension points and metadata only within the explicitly allowed v1 catalog and
never execute code. This completion authorizes only that bounded declarative
chain: no executable plugins, arbitrary JavaScript, native/dynamic library
loading, runtime script execution, shell command execution, dependency
installation, network plugin install/update, marketplace, credential access,
source mutation, export/publish/deploy mutation, CI/workflow mutation, secure
plugin sandbox guarantee, Godot-equivalent extension parity, production-ready
plugin ecosystem, or current Godot replacement claim is introduced.

Loop Coverage Metric v1 / Era E Milestone 20 (#1458/#1460-#1465) is now
complete as a conservative descriptive metric milestone. The merged evidence
chain added the #1458 scope and contract in PR #1537, #1460 provenance
attribution model in PR #1540, and #1461 computation/verdict/regression, #1462
read-only Studio and dashboard surfaces, #1463 fixture-scoped demo, and #1464
Scenario Coverage v21 in PR #1548. #1465 records only this roadmap/#1
governance refresh. Completion is valid only after that merged evidence chain:
loop coverage describes what fraction of trusted artifacts were produced by or
verified through the loop. It is not a quality guarantee, fun guarantee, accessibility guarantee,
production guarantee, release guarantee, commercial-readiness guarantee, or Godot
replacement guarantee. It grants no
source mutation authority, trusted browser writes, command bridge, auto-fix,
auto-apply, auto-merge, self-approval, or reviewer bypass. The full
intent-to-promotion provenance bundle remains Milestone 25 scope and must not be
backfilled into Milestone 20. The recommended next milestone from this point is
Era E Milestone 21: Second Game Class and Loop Generalization, using the loop
coverage metric to quantify generalization. Layer-3 distributed orchestration /
Elixir per ADR #92, native export, plugin runtime, and hosted/cloud scope remain
deferred and unchanged, to be re-evaluated only at Milestone 26. #1 and #23
remain open governance anchors.

Second Game Class and Loop Generalization v1 / Era E Milestone 21 (#1467-#1472)
is now complete as a conservative loop-generalization evidence milestone after
the required implementation evidence merged. The #1467 scope and contract, #1468
second game seed and GDD, #1469 loop-produced second game implementation, #1470
loop generalization gap evidence, #1471 deterministic demo, and #1472 Scenario
Coverage v22 all merged in PR #1547 under `examples/signal-gate-platformer/`
(seed, GDD, scene, scenario pack, manifest, fixture-scoped evidence) with Rust
and Node regression coverage. #1473 records only this roadmap/#1 governance
refresh and adds no runtime behavior. Completion is valid only after that merged
evidence chain: the loop generalized to a second game class — the Signal Gate
Platformer — measured by the loop coverage metric, demonstrating loop-produced
generalization rather than broad genre support, production readiness, fun or
commercial-readiness, or a Godot replacement. Rust/local retains ownership,
browser/Studio/dashboard surfaces remain read-only, contracts stay
backward-compatible, and generated artifacts remain ignored unless fixture-scoped
and reviewed; no source mutation authority, command bridge, auto-apply,
auto-merge, self-approval, or reviewer bypass is introduced. The recommended next
milestone from this point is Era E Milestone 22: Trust Gradient Design Gate.
Layer-3 distributed orchestration / Elixir per ADR #92, native export, plugin
runtime, and hosted/cloud scope remain deferred and unchanged, to be re-evaluated
only at Milestone 26. #1 and #23 remain open governance anchors.

Trust Gradient v1 / Era E Milestone 22 (#1476-#1483) is now complete as a
conservative bounded-autonomy milestone after a **GO** design-gate decision and
the required implementation evidence merged. #1476 recorded the design gate in PR
#1549: **GO for bounded, reversible, audited, default-off auto-apply, narrow
scope only** (`docs/trust-gradient-design.md`). The implementation chain then
merged #1477 risk-tier classifier in PR #1552, #1478 rollback-backed bounded
auto-apply in PR #1553, #1479 auto-apply audit log and kill switch in PR #1555,
#1480 read-only Studio Trust Gradient inspection surface in PR #1558, #1481
deterministic demo in PR #1559, and #1483 Scenario Coverage v23 in PR #1561
(Rust-owned regression guard driving the production classify/decide/validate
paths). #1484 records only this roadmap/#1 governance refresh and adds no runtime
behavior. Completion is valid only after that merged evidence chain: autonomy is
**bounded** (only low-risk, scene-only-data, high-confidence, all-gates-pass,
in-budget proposals), **reversible** (every auto-apply carries a one-command
rollback handle), **audited** (append-only log), and **default-off** (autonomy
opt-in, kill switch halts it). It is not auto-merge, not self-approval, not
source-affecting auto-apply, and not a quality, fun, production, or commercial
guarantee; it grants no trusted browser writes, command bridge, or reviewer
bypass. Rust/local retains ownership; browser/Studio/dashboard surfaces remain
read-only; contracts stay backward-compatible; generated artifacts remain ignored
unless fixture-scoped. The recommended next milestone from this point is Era E
Milestone 23: Multi-Iteration Evolve Campaigns. Layer-3 distributed orchestration
/ Elixir per ADR #92, native export, plugin runtime, and hosted/cloud scope
remain deferred and unchanged, to be re-evaluated only at Milestone 26. #1 and #23
remain open governance anchors.

Multi-Iteration Evolve Campaigns v1 / Era E Milestone 23 (#1486-#1491) is now
complete as a conservative bounded-campaign milestone after the required
implementation evidence merged. #1486 framed the scope and contract in PR #1551;
the campaign model and stop conditions (#1487), convergence tracking and budget
(#1488), journal narrative (#1489), and Scenario Coverage v24 (#1491) merged as a
cumulative stack in PR #1560 (`crates/ouroforge-core/src/evolve_campaign.rs` with
contract, convergence, and journal tests plus
`crates/ouroforge-core/tests/scenario_coverage_v24_evolve_campaign.rs`); the
deterministic demo (#1490) merged in PR #1563
(`examples/evolve-campaign-v1/demo/` and
`crates/ouroforge-core/tests/evolve_campaign_demo_contract.rs`). #1492 records
only this roadmap/#1 governance refresh and adds no runtime behavior. Completion
is valid only after that merged evidence chain: a campaign is a bounded,
audited, multi-iteration sequence of evolve proposals that stops on a declared
acceptance condition or an exhausted budget, with convergence reported
descriptively (converged / not-converged with an evidence-linked diagnosis) and a
journal narrative over the iterations. Convergence is descriptive, not a quality,
fun, production, or commercial guarantee; campaigns respect the Trust Gradient
(iterations remain manual-review unless within the M22 bounded auto-apply budget),
introduce no unsupervised unbounded looping, and grant no auto-merge,
self-approval, source-affecting auto-apply, trusted browser writes, command
bridge, or reviewer bypass. Rust/local retains ownership; browser/Studio/dashboard
surfaces remain read-only; contracts stay backward-compatible; generated artifacts
remain ignored unless fixture-scoped. The recommended next milestone from this
point is Era E Milestone 24: Game Complexity Ladder. Layer-3 distributed
orchestration / Elixir per ADR #92, native export, plugin runtime, and
hosted/cloud scope remain deferred and unchanged, to be re-evaluated only at
Milestone 26. #1 and #23 remain open governance anchors.

Game Complexity Ladder v1 / Era E Milestone 24 (#1493-#1498) is now complete as
a conservative governance milestone after the required implementation evidence
merged. #1493 supplied the scope and contract in PR #1522; #1494 added the
ladder model and capability gates in PR #1526; #1495 added the engine-growth
demand justification gate in PR #1527; #1496 added the fixture-scoped rung demo
in PR #1529; and #1497 added Scenario Coverage v25 in PR #1530. #1498 records
only this roadmap/#1 refresh and does not add runtime behavior. Completion is
valid only after the #1494-#1497 merged evidence chain: engine growth remains
demand-driven and rung-justified, the roadmap pre-authorizes no broad engine
breadth, generated artifacts remain ignored unless fixture-scoped and reviewed,
browser/Studio/dashboard surfaces remain read-only, no auto-promotion or
reviewer bypass is introduced, and no production readiness, broad compatibility,
shipped-game completeness, or Godot replacement claim is made. Layer-3
distributed orchestration / Elixir remains deferred under ADR #92 and should be
re-evaluated only at Milestone 26. The follow-on Era E Milestone 25 governance
chain is now recorded below. #1 and #23 remain open governance anchors.

End-to-End Provenance Bundle and Audit Surface v1 / Era E Milestone 25 is now
complete as a conservative, additive provenance/audit milestone after the
required evidence chain merged. Parent scope #1524 framed the milestone; #1531
added the bundle model; #1533 added local replayability; #1538 added the
read-only audit surface and human sign-off display; #1541 added the deterministic
fixture-scoped demo; #1542 added Scenario Coverage v26; and repair #1545 is
recognized only as supporting evidence for the final chain. Completion is valid
only for composition by reference over existing provenance, evidence, review,
rollback, and regression-promotion artifacts. It does not introduce a parallel
provenance engine, auto-promotion, auto-approval, auto-merge, reviewer bypass,
trusted browser writes, command bridges, production-readiness claims, quality
guarantees, or Godot replacement positioning. Rust/local tooling retains trusted
ownership; dashboard, Studio, and browser surfaces remain read-only; existing
contracts stay backward-compatible; and generated provenance, replay, dashboard,
browser, temp, and local tool artifacts remain untracked unless explicitly
fixture-scoped. Layer-3 distributed orchestration / Elixir remains deferred
under ADR #92 and should be re-evaluated at Milestone 26. The recommended next
milestone is Era E Milestone 26: Era E Refresh and Layer-3 Re-evaluation
Trigger. #1 and #23 remain open governance anchors.

Era E (Milestones 20-25) is now complete as an evidence-native milestone arc
after each milestone's implementation evidence merged: Milestone 20 Loop Coverage
Metric v1 (#1458/#1460-#1465), Milestone 21 Second Game Class and Loop
Generalization v1 (#1467-#1472), Milestone 22 Trust Gradient v1 (#1476-#1483),
Milestone 23 Multi-Iteration Evolve Campaigns v1 (#1486-#1491), Milestone 24 Game
Complexity Ladder v1 (#1493-#1498), and Milestone 25 End-to-End Provenance Bundle
and Audit Surface v1 (#1524/#1531-#1542). #1507 records only this consolidated
Era E roadmap/#1 governance refresh and adds no runtime behavior.

North-star assessment (#1's loop coverage x game complexity x trust) is
descriptive and evidence-cited, not a maturity claim:

- Loop coverage x game classes: the loop-coverage metric (Milestone 20)
  quantifies the fraction of trusted artifacts produced by or verified through the
  loop. It is evidenced across two game classes — the collect-and-exit baseline
  and the Signal Gate Platformer second class (Milestone 21,
  `examples/signal-gate-platformer/`) — demonstrating loop-produced
  generalization measured by that metric.
- Game complexity: the Game Complexity Ladder v1 (Milestone 24) records exactly
  one rung satisfied (`game-complexity-ladder-v1.collect-and-exit`); engine growth
  stays demand-driven and rung-justified, with no later rung, breadth, or parity
  claimed.
- Trust: the Trust Gradient (Milestone 22) is GO for bounded, reversible,
  audited, default-off auto-apply (narrow scope only); evolve campaigns
  (Milestone 23) remain bounded and audited and respect that gradient; end-to-end
  provenance (Milestone 25) composes a read-only bundle/replay/audit surface by
  reference over existing artifacts.

Known gaps and boundaries (unchanged): the metrics are descriptive, not quality,
fun, production, or commercial guarantees; only one complexity rung and two game
classes are evidenced; autonomy stays bounded, reversible, audited, and
default-off with no auto-merge, self-approval, source-affecting auto-apply, or
reviewer bypass; browser/Studio/dashboard surfaces remain read-only and
Rust/local retains ownership; contracts stay backward-compatible; generated
artifacts remain ignored unless fixture-scoped. No production readiness, broad
genre/engine breadth, or Godot replacement is claimed. The recommended next
milestone is Era E Milestone 26: Era E Refresh and Layer-3 Re-evaluation; the
go/defer decision on Layer-3 distributed orchestration / Elixir (deferred under
ADR #92) is made in the paired Layer-3 Re-evaluation Design Gate (#1508). #1 and
#23 remain open governance anchors.

## Completed evidence-native milestones

The current implementation has completed these documented milestone surfaces:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`)
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`,
  `docs/scenario-evaluator-v1-demo.md`)
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`)
- Evolve Loop Depth v1 / evidence-linked proposals and four-gate rerun deltas
  (`docs/evolve-loop-depth-v1.md`,
  `docs/evolve-loop-depth-v1-demo.md`,
  `docs/scenario-coverage-v20.md`,
  `examples/evolve-loop-depth-v1/`)
- Studio v1 (`docs/studio-v1.md`, `docs/studio-v1-demo.md`)
- Engine Expansion v1 (`docs/engine-expansion-v1.md`,
  `docs/engine-expansion-v1-demo.md`)
- Authoring Loop v2 (`docs/authoring-loop-v2.md`,
  `docs/scene-edit-transactions.md`, `docs/run-comparison-v2.md`,
  `docs/scene-only-mutation-v2.md`, `docs/studio-v2-cockpit.md`)
- Project Workspace Loop v1 (`docs/project-workspace-loop-v1.md`,
  `docs/project-manifest-v1.md`, `docs/project-scaffold-v1.md`,
  `docs/scenario-pack-v1.md`, `docs/project-run-v1.md`,
  `docs/project-comparison-v1.md`, `docs/project-mutation-loop-v1.md`,
  `docs/studio-v3-project-workspace-cockpit.md`)
- Evidence Fidelity & Trust Boundary Hardening v1
  (`docs/evidence-fidelity-trust-boundary-v1.md`,
  `docs/runtime-probe-contract-v2.md`,
  `docs/input-replay-evidence-v2.md`,
  `docs/openchrome-cdp-evidence-fidelity-v2.md`,
  `docs/reproducible-run-command-context-v1.md`,
  `docs/studio-evidence-fidelity-surfaces.md`)
- Agentic Review & Regression Promotion v1
  (`docs/agentic-review-regression-promotion-v1.md`,
  `docs/mutation-proposal-quality-v2.md`,
  `docs/review-decision-ledger-v1.md`,
  `docs/project-mutation-loop-v1.md`,
  `docs/regression-run-matrix-v1.md`,
  `docs/evidence-backed-journal-v2.md`,
  `docs/studio-review-cockpit-v1.md`)
- Agentic Loop Orchestration v1
  (`docs/agentic-loop-orchestration-v1.md`,
  `docs/authoring-loop-plan-v1.md`,
  `docs/authoring-loop-dry-run-v1.md`,
  `docs/authoring-loop-execution-v1.md`,
  `docs/authoring-loop-recovery-v1.md`,
  `docs/authoring-loop-evidence-bundle-v1.md`,
  `docs/agent-handoff-contract-v1.md`,
  `examples/authoring-cockpit/README.md`)
- Engine Expressiveness v2 / Playable Game Authoring v1 implemented subset
  (`docs/engine-expressiveness-v2.md`,
  `docs/engine-expressiveness-v2-governance-handoff.md`,
  `docs/scene-component-model-v2.md`,
  `docs/collision-physics-v2.md`,
  `docs/gameplay-trigger-flags-v1.md`,
  `docs/scene-transitions-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/scenario-coverage-v3.md`,
  `docs/studio-authoring-surface-v2-expressive-inspection.md`)
- Asset Pipeline v1 / Content Authoring Foundation
  (`docs/asset-pipeline-v1.md`,
  `docs/asset-manifest-v1.md`,
  `docs/sprite-atlas-manifest-v1.md`,
  `docs/tileset-tilemap-authoring-v2.md`,
  `docs/asset-reference-integrity-v1.md`,
  `docs/runtime-asset-loading-evidence-v1.md`,
  `docs/asset-preview-evidence-v1.md`,
  `docs/studio-asset-inspector-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/scenario-coverage-v4-asset-pipeline.md`,
  `docs/asset-pipeline-v1-governance-handoff.md`)
- Visual Authoring v1 / Safe Local Edit Cockpit
  (`docs/visual-authoring-v1.md`,
  `docs/visual-edit-draft-model-v1.md`,
  `docs/edit-draft-transaction-cli-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/visual-authoring-v1-governance-handoff.md`)
- Production 2D Engine Core v1 / bounded local 2D vertical-slice core
  (`docs/production-2d-engine-core-v1.md`,
  `docs/runtime-state-save-v1.md`,
  `docs/runtime-frame-budget-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/scenario-coverage-v3.md`,
  `docs/production-2d-studio-inspection-v1.md`)
- Multi-Agent Production Pipeline v1 / evidence-gated local collaboration
  (`docs/multi-agent-production-pipeline-v1.md`,
  `docs/agent-role-model-v1.md`,
  `docs/file-artifact-ownership-conflict-policy-v1.md`,
  `docs/agent-work-package-v1.md`,
  `docs/agent-handoff-v2.md`,
  `docs/agent-shared-state-snapshot-v1.md`,
  `docs/review-critic-gate-v1.md`,
  `docs/qa-agent-work-queue-v1.md`,
  `docs/performance-regression-lane-v1.md`,
  `docs/production-evidence-bundle-v1.md`,
  `docs/studio-multi-agent-pipeline-inspection-v1.md`,
  `docs/multi-agent-prototype-production-demo-v1.md`,
  `docs/multi-agent-pipeline-coverage-matrix-v1.md`,
  `docs/multi-agent-production-pipeline-governance-handoff.md`)
- 3D Capability Gate v1 / bounded local 3D capability evidence
  (`docs/3d-capability-gate-v1.md`,
  `docs/3d-scene-graph-v1.md`,
  `docs/3d-camera-projection-v1.md`,
  `docs/3d-mesh-material-refs-v1.md`,
  `docs/3d-render-smoke-v1.md`,
  `docs/3d-collision-physics-v1.md`,
  `docs/3d-animation-playback-v1.md`,
  `docs/3d-runtime-probe-contract-v1.md`,
  `examples/3d-demo-scene-v1/README.md`,
  `examples/3d-capability-regression-v8/README.md`,
  `docs/studio-3d-inspection-surface-v1.md`)
- Agentic Scene and Level Designer v1 / evidence-gated level authoring
  (`docs/agentic-scene-level-designer-v1.md`,
  `docs/level-intent-v1.md`,
  `docs/scene-generation-plan-v1.md`,
  `docs/spatial-layout-constraint-solver-v1.md`,
  `docs/tilemap-terrain-generation-draft-v1.md`,
  `docs/entity-objective-encounter-placement-draft-v1.md`,
  `docs/reachability-pathing-evidence-v1.md`,
  `docs/objective-completion-proof-v1.md`,
  `docs/difficulty-pacing-heuristic-evidence-v1.md`,
  `docs/level-visual-semantic-diff-v1.md`,
  `docs/agent-generated-level-draft-v1.md`,
  `docs/review-gated-level-apply-v1.md`,
  `docs/studio-level-design-inspection-surface-v1.md`,
  `docs/agentic-level-design-demo-v1.md`,
  `docs/scenario-coverage-v10-agentic-level-design.md`,
  `docs/agentic-scene-level-designer-governance-handoff.md`)
- Gameplay Scripting / Logic System v1 / structured data-only gameplay behavior
  (`docs/gameplay-scripting-logic-system-v1.md`,
  `docs/gameplay-behavior-model-v1.md`,
  `docs/gameplay-event-signal-system-v1.md`,
  `docs/gameplay-state-machine-v1.md`,
  `docs/gameplay-ability-action-v1.md`,
  `docs/script-module-interface-design-gate-v1.md`,
  `docs/safe-script-sandbox-trust-boundary-v1.md`,
  `docs/gameplay-state-ability-evidence-compatibility-v1.md`,
  `docs/behavior-draft-v1.md`,
  `docs/behavior-apply-transaction-v1.md`,
  `docs/studio-behavior-inspection-surface-v1.md`,
  `examples/gameplay-logic-demo-v1/README.md`,
  `examples/gameplay-logic-regression-v9/README.md`)
- Full Studio Editor v1 / local read-only and draft-only authoring cockpit
  (`docs/full-studio-editor-v1.md`,
  `docs/full-studio-editor-integrated-demo-v1.md`,
  `docs/scenario-coverage-v17-full-studio-editor.md`,
  `examples/full-studio-editor-demo-v1/`,
  `examples/full-studio-editor-regression-v17/`,
  `examples/authoring-cockpit/cockpit.test.cjs`)
- Build / Export / Packaging v1 / local export package foundation
  (`docs/build-export-packaging-v1.md`,
  `docs/export-target-matrix-v1.md`,
  `docs/export-staging-policy-v1.md`,
  `docs/scenario-coverage-v15-build-export-packaging.md`,
  `examples/build-export-packaging-regression-v15/coverage-matrix.fixture.json`,
  `examples/export-profile-v1/export-profile.valid.fixture.json`,
  `examples/export-bundle-v1/export-profile.fixture.json`,
  `examples/export-asset-manifest-v1/asset-manifest.valid.fixture.json`,
  `examples/export-probe-v1/`)
- Evolve Loop Depth v1 / evidence-linked bounded mutation loop
  (`docs/evolve-loop-depth-v1.md`,
  `docs/evolve-loop-depth-v1-demo.md`,
  `docs/scenario-coverage-v20.md`,
  `examples/evolve-loop-depth-v1/demo/`,
  `examples/evolve-loop-depth-v1/scenario-coverage-v20/`,
  `examples/evolve-loop-depth-v1/scenario-coverage-v20-evolve-depth.test.cjs`)
- Foundation Hardening v1 / mechanical crate decomposition
  (`docs/foundation-hardening-v1.md`,
  `docs/refactor-parity-golden-baseline-v1.md`,
  `crates/ouroforge-ledger/`,
  `crates/ouroforge-evidence/`,
  `crates/ouroforge-evaluator/`)
- Godot-Plus Demo Game v1 / Signal Gate local workflow demonstration
  (`docs/godot-plus-demo-game-v1.md`,
  `docs/godot-plus-demo-design-pillars-v1.md`,
  `docs/autonomous-qa-playtest-swarm-v1.md`,
  `docs/safe-source-mutation-apply-v1.md`,
  `docs/scenario-coverage-v15-build-export-packaging.md`,
  `docs/scenario-coverage-v16-plugin-extension.md`)
- Plugin / Extension System v1 / declarative local extension foundation
  (`docs/plugin-extension-system-v1.md`,
  `docs/plugin-system-design.md`,
  `docs/plugin-threat-model-v1.md`,
  `docs/studio-plugin-extension-panel-v1.md`,
  `docs/plugin-extension-system-demo-v1.md`,
  `docs/scenario-coverage-v16-plugin-extension.md`)
- Loop Coverage Metric v1 / Era E Milestone 20
  (`docs/loop-coverage-metric-v1.md`,
  `docs/loop-coverage-metric-v1-demo.md`,
  `docs/loop-coverage-metric-v1-governance-handoff.md`,
  `docs/scenario-coverage-v21.md`,
  `crates/ouroforge-core/src/loop_coverage_metric.rs`,
  `crates/ouroforge-core/tests/loop_coverage_metric_contract.rs`,
  `examples/loop-coverage-v1/`)
- Game Complexity Ladder v1 / Era E Milestone 24
  (`docs/game-complexity-ladder-v1.md`,
  `docs/game-complexity-ladder-v1-demo.md`,
  `docs/scenario-coverage-v25.md`,
  `docs/game-complexity-ladder-v1-governance-handoff.md`,
  `crates/ouroforge-core/tests/complexity_ladder_contract.rs`,
  `crates/ouroforge-core/tests/engine_growth_justification_contract.rs`,
  `examples/game-complexity-ladder-v1/`,
  `examples/engine-growth-justification-v1/`)
- End-to-End Provenance Bundle and Audit Surface v1 / Era E Milestone 25
  (`docs/end-to-end-provenance-v1.md`,
  `docs/provenance-bundle-v1.md`,
  `docs/provenance-replay-v1.md`,
  `docs/end-to-end-provenance-v1-demo.md`,
  `docs/scenario-coverage-v26.md`,
  `examples/end-to-end-provenance-v1/`,
  `examples/evidence-dashboard/dashboard.test.cjs`,
  `examples/authoring-cockpit/cockpit.test.cjs`,
  `crates/ouroforge-core/tests/provenance_bundle_contract.rs`,
  `crates/ouroforge-core/tests/provenance_replay_contract.rs`,
  `crates/ouroforge-core/tests/end_to_end_provenance_demo_contract.rs`,
  `crates/ouroforge-core/tests/scenario_coverage_v26_provenance.rs`)

These milestones are still MVP contracts, not public compatibility promises.
Generated run evidence remains ignored local state unless an issue explicitly
scopes a tiny deterministic fixture as tracked source-like data.

Engine Expressiveness v2 completion covers the implemented local playable demo,
component, collision, trigger, HUD, animation/audio event, manifest-declared
transition, regression, and Studio-inspection surfaces. These are bounded local
evidence contracts, not broad editor/runtime compatibility claims.

Asset Pipeline v1 completion covers local source-like asset manifests, sprite
atlas metadata, tileset/tilemap authoring metadata, scene/reference integrity,
runtime loading evidence, preview/read-model evidence, Studio asset inspection,
asset-backed playable-demo refresh, Scenario Coverage v4, and the post-milestone
governance handoff. It remains local-first and Rust-trusted; it does not add
remote asset hosting, browser uploads/writes, marketplace/plugin behavior,
native export, production editor claims, source mutation apply, or public launch
automation.

Visual Authoring v1 completion covers local source-like visual edit draft
fixtures, Rust-owned draft validation/preflight, transaction preview, visual diff
summary evidence, review-gated apply lifecycle evidence, ignored generated demo
smoke ids, Scenario Coverage v5, and read-only Studio/dashboard compatibility
wording. It remains a bounded Safe Local Edit Cockpit milestone: it does not add
browser trusted writes, command bridges, production editor behavior, visual
scripting, source mutation apply, plugin runtime, native export, hosted/cloud
services, public launch automation, or Godot replacement claims.

Production 2D Engine Core v1 completion covers the #581-#593 evidence chain:
scope/contract, renderer architecture, camera/layer/viewport behavior,
sprite/atlas/tilemap rendering, 2D physics/collision, input actions/replay,
runtime state save/load and deterministic replay digests, animation/VFX, audio
runtime evidence, frame-budget profiling, a bounded vertical-slice demo,
Scenario Coverage v7, and read-only Studio inspection. It remains local-first
and Rust-trusted for validation, persistence, generated evidence, source-like
fixture validation, save/load artifacts, and CLI behavior. Browser/dashboard/
Studio surfaces remain read-only or draft-only for trusted state. This completion
does not add 3D implementation, native export, plugin runtime, hosted/cloud
behavior, unrestricted source apply, command bridges, public launch automation,
production-ready status, broad compatibility-stable API promises, shipped-game
claims, secure-sandbox guarantees, or Godot replacement positioning.


Multi-Agent Production Pipeline v1 completion covers the #664-#679 evidence
chain: scope/contract, role model, task board, ownership, work packages,
handoffs, shared state snapshots, review/critic gates, QA queue,
performance/regression lane, build/release design gate, decision ledger,
production evidence bundle, read-only Studio/dashboard/cockpit inspection, demo,
and Scenario Coverage v12. It remains a local-first accountability framework: it
does not add hidden/background agents, unbounded spawning, hosted/cloud
orchestration, browser command bridges, trusted browser writes, auto-apply,
auto-merge, self-approval, reviewer bypass, dependency/CI/workflow mutation,
release automation, production readiness, public-launch approval, or current
Godot replacement positioning.

3D Capability Gate v1 completion covers the #596-#608 evidence chain: scope
contract, scene graph and transform hierarchy, camera/projection, mesh/material
local references, render smoke evidence, collision/trigger evidence, bounded
animation playback, runtime probe contract, scenario/evaluator compatibility, a
small deterministic 3D demo scene, Scenario Coverage v8, normalized dashboard
read models, read-only Studio 3D inspection, and this roadmap/#1 governance
refresh. It remains a bounded local capability gate. It does not add a full 3D
editor, production 3D renderer, broad 3D compatibility promise, broad
GLTF/import pipeline, advanced lighting/PBR/material graph, skeletal authoring,
native export, plugin runtime, hosted/cloud/server/auth behavior, browser
trusted writes, command bridges, unrestricted source apply, public launch
automation, production-ready status, secure-sandbox guarantees, shipped-game
maturity, or current Godot replacement positioning. Generated 3D runs,
screenshots, previews, dashboard exports, temp projects, and local tool outputs
remain ignored generated state unless a later issue explicitly scopes a tiny
deterministic source-like fixture.

Agentic Scene and Level Designer v1 completion covers the #627-#642 evidence
chain: scope/contract, level intent, scene generation plan, spatial layout
constraints, tilemap/terrain draft, entity/objective/encounter placement draft,
reachability/pathing evidence, objective completion proof, difficulty/pacing
heuristic evidence, level visual/semantic diff, agent-generated level draft,
review-gated level apply record, read-only Studio inspection, deterministic demo,
Scenario Coverage v10, and this roadmap/#1 governance refresh. It remains a
bounded local level-authoring milestone: it does not add autonomous unrestricted
game creation, a production editor, a full visual level editor, visual scripting,
browser trusted writes, command bridges, auto-apply, auto-merge, self-approval,
unrestricted source mutation apply, native export, plugin runtime, hosted/cloud
behavior, production-ready status, secure-sandbox guarantees, shipped-game
maturity, or current Godot replacement positioning. Generated level drafts,
previews, screenshots, runs, dashboard exports, temp projects, and local tool
outputs remain ignored generated state unless a later issue explicitly scopes a
deterministic source-like fixture.

After #625, Gameplay Scripting / Logic System v1 (#611-#625) is complete as a
structured data-only gameplay behavior/evidence milestone. It covers behavior
models, deterministic events/signals, state machines, ability/action contracts,
script and sandbox design gates, runtime integration, behavior scenario
assertions, agent-generated behavior drafts, review-gated behavior apply,
evidence/journal integration, Studio behavior inspection, demo evidence,
Scenario Coverage v9, and this roadmap/#1 governance refresh. This completion
stays bounded to local behavior/evidence contracts and is not arbitrary
third-party code loading, plugin runtime, browser command bridge, hosted
execution, production scripting, source-apply authority, public-launch approval,
or Godot replacement scope.


GDD-to-Playable Prototype v1 (#644-#661) is complete after the merged evidence
chain: #644 defined the scope contract, #645 added the design-brief schema, #646
added requirement extraction, #647 added mechanics/core-loop mapping, #648 added
the feasibility gate, #649 added scaffold planning, #650 added scene/level
planning, #651 added behavior planning, #652 added asset placeholder/reference
planning, #653 added scenario/acceptance planning, #654 added the prototype task
graph, #655 added the draft bundle, #656 added review-gated apply, #657 added
run/evidence/journal bundles, #658 added read-only Studio planning inspection,
#659 added the deterministic demo, #660 added Scenario Coverage v11, and #661
records this roadmap/#1 governance refresh. This completion enables a bounded,
evidence-gated GDD-to-prototype path only. It does not authorize autonomous
unrestricted game creation, arbitrary source/script mutation, browser trusted
writes, command bridges, auto-apply, auto-merge, uncontrolled asset generation,
generated proprietary assets, native export, plugin runtime, hosted/cloud
behavior, public launch, production-ready or shipped-game claims, or current
Godot replacement positioning. GDD-derived outputs remain untrusted until
Rust/local validation and review-gated apply; browser/dashboard/Studio surfaces
remain read-only or draft-only for trusted state; generated prototype outputs and
local tool state remain ignored unless explicitly fixture-scoped.

Evaluator Depth v1 (#1279/#1283-#1288) is complete after the merged evidence
chain: #1300 defined the scope contract, #1308 added Visual Evaluator Gate v1,
#1311 added Semantic Evaluator Gate v1, #1318 added four-category verdict and
Journal summaries, #1320 added read-only evaluator-depth inspection surfaces,
#1323 added the deterministic demo, and #1325 added Scenario Coverage v19 plus
the legacy two-gate golden. The resulting verdict changes are additive: legacy
two-gate runs remain byte-compatible when visual/semantic gates are undeclared,
while declared gates expose `mechanical`, `runtime`, `visual`, and `semantic`
categories with evidence-linked reasons. Rust/local validation retains trusted
persistence, gate logic, and verdict serialization; browser/dashboard/Studio
surfaces remain read-only evidence viewers. Known gaps remain deliberately
outside this milestone: no aesthetic visual quality judgment, no general
semantic correctness guarantee, no source mutation authority, no command bridge,
no native export, no plugin runtime, no hosted/cloud behavior, no release
readiness, and no engine-replacement claim.

Evolve Loop Depth v1 (#1290/#1292-#1298) is complete after the merged
evidence chain: #1290 defined the scope contract, #1292 added evidence-linked
mutation proposal rationale, #1293 added failure-classification-driven bounded
selection, #1294 added four-gate before/after rerun comparison plus Journal v2
summaries, #1295 added read-only Studio/evidence dashboard inspection, #1296
added the deterministic fixture-scoped demo, #1297 added Scenario Coverage v20
with legacy evolve v0 compatibility, and #1298 records this governance refresh.
The milestone completes #1 Milestone 5.1 under the disambiguated name **Evolve
Loop Depth v1** rather than the closed #215 scene-only apply milestone it
consumes. Proposals remain manual-review inputs; confidence is an evidence-
derived bounded signal, not a correctness or quality guarantee; mutation types
remain bounded to data, scene, and scenario artifacts; Rust/local validation owns
trusted proposal/comparison persistence and serialization; browser/dashboard/
Studio surfaces remain read-only evidence displays. This completion does not add
arbitrary source patching, auto-accept, auto-apply, auto-merge, self-approval,
reviewer bypass, browser trusted writes, command bridges, native export, plugin
runtime, hosted/cloud behavior, release readiness, production readiness, or a
current Godot replacement claim. Generated runs, proposals, comparisons,
screenshots, and local tool outputs remain ignored unless explicitly fixture-
scoped.

Source Mutation Design Gate v1 is complete as a design/control milestone. Its
outcome keeps source mutation apply blocked: the gate produced threat model,
file-class, preview-artifact, review-gate, rollback/audit, sandbox/worktree, and
read-only Studio review designs, but it did not implement source patch
application, arbitrary patch apply, browser command bridges, or source-mutation
readiness.

Source Mutation Preview v1 is complete as an inert preview/evidence milestone
after #365/#366. Its implementation slices added file-class validation, bounded
diff integrity checks, preview artifacts, stale-target guards, allowlisted
sandbox dry-run evidence, review-decision/evidence bundles, read-only
dashboard/Studio display, and generated-state cleanup/audit coverage. This is
not source apply authorization: source patch application to the trusted
maintainer worktree, merge/rebase automation, dependency/CI mutation, arbitrary
shell/network/install commands, browser command bridges, public launch
automation, native export, plugin runtime, and Godot replacement claims remain
out of scope unless a separate later governance issue authorizes them.

Public Alpha Launch Governance v1 is complete as a governance/documentation
track. It produced manual visibility decision records, hold/rollback criteria,
public issue and PR intake policies, security-response routing, demo-stability
monitoring, a conservative communication pack, and post-launch roadmap triage
references. It did not launch Ouroforge, publish an announcement, change
repository visibility or GitHub settings, release packages, add product
behavior, create support/security SLAs, implement source apply, or replace the
#1/#23 governance anchors.

## Near-term governance and public-readiness work

### Evaluator Depth v1 governance refresh

Evaluator Depth v1 is recorded as **complete for Milestone 4.1**, closing the
four-gate evaluator drift identified in #1's Roadmap Alignment Addendum. The
completion evidence is the merged implementation chain #1300, #1308, #1311,
#1318, #1320, #1323, and #1325, with required local verification and CI evidence
recorded on the linked issues. #1 and #23 remain open governance anchors.

Milestone 5.1 is now complete as **Evolve Loop Depth v1** (#1290/#1292-#1298).
The recommended next technical sequence is to continue **Era C** (#660-#797) on
the corrected four-gate, evidence-linked evolve foundation, with Foundation
Hardening (A.H/A.H2 candidates) treated only as a parallel hygiene track when a
separate scope issue defines no-behavior-change parity gates. Layer-3 expansion
remains unchanged and deferred: distributed orchestration / Elixir per ADR #92,
native export, plugin runtime, and hosted/cloud scope still require separate
scoped issue sequences and governance approval.

### Evolve Loop Depth v1 governance refresh

Evolve Loop Depth v1 is recorded as **complete for Milestone 5.1**, closing the
Roadmap Alignment Addendum item that remained after #215. The completion
evidence is the merged issue/PR chain #1290, #1292, #1293, #1294, #1295, #1296,
#1297, and #1298, with final demo and Scenario Coverage v20 verification recorded
on #1296/#1297 and this governance issue. #1 and #23 remain open governance
anchors.

The recommended next technical sequence is to continue **Era C (#660-#797)** on
the corrected four-gate, evidence-linked evolve foundation. In practical order,
this means QA/playtest swarm and Full Studio Editor/Godot-Plus demo work can
consume bounded proposal confidence, data/scene/scenario mutation type mapping,
and four-gate rerun deltas without weakening review gates. Foundation Hardening
(A.H) remains a parallel hygiene track only; future A.H2 work still needs a
separate scope issue before any extraction. Layer-3 scope remains unchanged and
deferred: distributed orchestration / Elixir per ADR #92, native export, plugin
runtime, and hosted/cloud still require separate design gates and governance
approval.

### Safe Source Mutation Apply v1 governance refresh

Safe Source Mutation Apply v1 is recorded as **complete as bounded review-gated
trusted source apply**, closing the Milestone 15 governance chain without closing
#1 or #23. The completion evidence is the merged issue sequence #699-#716: scope
and threat model, trusted-worktree boundary, transaction/stale-target/review
controls, sandbox promotion readiness, rollback, allowlisted verification,
post-apply rerun/comparison, dependency/CI/build-script blockers, audit ledger,
evidence bundle, read-only Studio inspection, deterministic demo, Scenario
Coverage v14, emergency hold/kill-switch, and this roadmap refresh.

The remaining source-mutation gaps stay explicit and require separate scoped
governance issues before implementation: expanding eligible file classes,
dependency or lockfile mutation, CI/workflow or build-script mutation,
credential/network/cloud/release/export mutation, browser trusted writes,
command bridges, autonomous source repair, executable script repair, production
secure-sandbox claims, and production-ready mutation guarantees. #1 and #23
remain open governance anchors.

### Plugin / Extension System v1 governance refresh

Plugin / Extension System v1 is recorded as **complete as a bounded,
declarative, allowlisted, evidence-backed extension foundation**, closing the
Plugin / Extension System milestone governance chain without closing #1 or #23.
The completion evidence is the merged issue sequence #738-#754: scope and
contract, plugin manifest schema, local registry and discovery, extension point
catalog, capability/permission model, version compatibility, descriptor
evidence integration, read-only Studio plugin browser, read-only dashboard
panel/scenario template/asset metadata descriptors, fixture plugin pack,
security/threat-model gate, load-order and conflict detection, CLI inspection,
Scenario Coverage v16, and the deterministic demo. Plugins declare extension
points and metadata only within the explicitly allowed v1 catalog; Rust/local
validation owns discovery, manifest validation, registry persistence, capability
and compatibility checks, and evidence writing; browser/dashboard/Studio
surfaces remain read-only rendering of inert descriptors.

The remaining plugin gaps stay explicit and require separate scoped governance
issues before implementation: executable plugins, scripting plugin runtime,
arbitrary JavaScript, native/dynamic extensions, marketplace, network
install/update, dependency installation, third-party package trust model, full
editor extensibility, command execution, source/export/publish/deploy mutation,
CI/workflow mutation, secure plugin sandbox guarantees, Godot-equivalent
extension parity, and a production-ready plugin ecosystem. #1 and #23 remain open
governance anchors.

### Loop Coverage Metric v1 governance refresh

Loop Coverage Metric v1 is recorded as **complete for Era E Milestone 20** as a
conservative descriptive metric milestone under #1. The completion evidence
chain is #1458 scope and contract in PR #1537, #1460 provenance attribution
model in PR #1540, and #1461 computation/verdict/regression, #1462 read-only
Studio and dashboard inspection, #1463 fixture-scoped demo, and #1464 Scenario
Coverage v21 in PR #1548. #1465 records only this roadmap/#1 refresh. #1 and
#23 remain open governance anchors.

The remaining boundaries stay explicit: loop coverage is an authorship and
verification fraction for trusted artifacts, not a quality guarantee, fun
guarantee, accessibility guarantee, production guarantee, release guarantee,
commercial-readiness guarantee, or Godot replacement guarantee. It
does not authorize source mutation, trusted browser writes, command bridges,
auto-fix, auto-apply, auto-merge, self-approval, or reviewer bypass. The full
intent-to-promotion provenance bundle remains Era E Milestone 25 scope. The
recommended next milestone from Milestone 20 is **Era E Milestone 21: Second
Game Class and Loop Generalization**, using loop coverage to quantify
generalization across a second bounded game class. Layer-3 distributed
orchestration / Elixir per ADR #92, native export, plugin runtime, and
hosted/cloud scope remain deferred and unchanged, to be re-evaluated only at
Milestone 26.

### Game Complexity Ladder v1 governance refresh

Game Complexity Ladder v1 is recorded as **complete for Era E Milestone 24** as
a conservative governance/docs milestone under #1. The completion evidence chain
is #1493 as scope and contract in PR #1522, #1494 ladder model and capability
gates in PR #1526, #1495 engine-growth demand justification in PR #1527, #1496
fixture-scoped rung demo in PR #1529, #1497 Scenario Coverage v25 in PR #1530,
and #1498 as this roadmap/#1 refresh. #1 and #23 remain open governance anchors.

The remaining boundaries stay explicit: engine growth is demand-driven and
rung-justified, no broad engine breadth is pre-authorized, generated artifacts
remain ignored unless fixture-scoped and reviewed, Rust/local code and the local
filesystem own trusted state, browser/Studio/dashboard surfaces remain read-only,
and there is no production readiness, shipped-game completeness, broad
compatibility, Godot replacement positioning, auto-promotion, auto-apply,
auto-merge, self-approval, reviewer bypass, or Layer-3 implementation. Layer-3
distributed orchestration / Elixir remains deferred under ADR #92 and should be
re-evaluated only at Milestone 26.

Era E Milestone 25 is now recorded as complete below. The recommended next
milestone is **Era E Milestone 26: Era E Refresh and Layer-3 Re-evaluation
Trigger**.

### End-to-End Provenance Bundle and Audit Surface v1 governance refresh

End-to-End Provenance Bundle and Audit Surface v1 is recorded as **complete for
Era E Milestone 25** as a conservative additive provenance/audit milestone under
#1. The completion evidence chain is parent scope #1524, #1531 bundle model,
#1533 local replayability, #1538 read-only audit surface and human sign-off
display, #1541 deterministic fixture-scoped demo, #1542 Scenario Coverage v26,
and #1545 as supporting repair evidence only. #1 and #23 remain open governance
anchors.

The remaining boundaries stay explicit: bundles compose existing provenance,
evidence, review, rollback, and regression-promotion artifacts by reference and
do not create a parallel provenance engine. Audit surfaces are read-only, human
sign-off remains a human decision, and no auto-promotion, auto-approval,
auto-merge, self-approval, reviewer bypass, trusted browser writes, command
bridges, production-readiness claim, quality guarantee, or Godot replacement
positioning is introduced. Rust/local tooling and the local filesystem retain
trusted ownership; browser/Studio/dashboard surfaces remain read-only; existing
contracts remain backward-compatible; generated provenance, replay, dashboard,
browser, temp, and local tool artifacts remain ignored unless explicitly
fixture-scoped. Layer-3 distributed orchestration / Elixir remains deferred
under ADR #92 and should be re-evaluated at Milestone 26.

The recommended next milestone is **Era E Milestone 26: Era E Refresh and
Layer-3 Re-evaluation Trigger**.

### Evidence-Native Marketplace v1 governance refresh

Evidence-Native Marketplace v1 is recorded as **complete for Era F Milestone 33**
as a conservative, additive, local-only milestone under #1. It makes accumulated
evidence compound into verifiable assets: a local verifiable-asset registry over
the free OSS core where each asset binds its acceptance suite, a deterministic
replay proof, and a Milestone 25 provenance lineage, and is re-verified locally
on consume. The completion evidence chain is parent scope/design gate #1612
(`docs/evidence-marketplace-v1.md`, PR #1623), the Local Verifiable-Asset
Registry v1 #1613 (PR #1629), the Evidence-Native Marketplace Demo v1 #1615
(`docs/evidence-marketplace-v1-demo.md`, PR #1632), and Scenario Coverage v33
#1616 (`docs/scenario-coverage-v33.md`, PR #1633). #1 and #23 remain open
governance anchors.

The boundaries stay explicit and reaffirmed. The registry is **local only**: it
publishes and consumes by reference and re-verifies locally; it performs no
trusted write, executes nothing, installs nothing, and adds no hosted, paid, or
network capability. Any hosted or paid marketplace — networked distribution, a
transaction layer, or take-rate collection — stays **DEFER until a #1508 Layer-3
GO** (Layer-3 distributed orchestration / Elixir remains NO-GO under ADR #92).
The OSS core (engine, verification loop, registry, replay proofs, provenance
bundles) is free; the transaction layer is paid and Layer-3-gated, recorded as a
policy line and not a code path. The registry reuses the Milestone 25 provenance
bundle and adds no parallel provenance engine. Adoption of a consumed asset flows
through the existing review/apply/trust-gradient path, never a direct trusted
write; generation stays proposal-only; browser/Studio surfaces remain read-only;
existing contracts remain backward-compatible; and generated registry, replay,
and evidence artifacts remain ignored unless explicitly fixture-scoped. No
production-readiness claim, quality guarantee, fun or shippability claim, or
Godot replacement/parity positioning is introduced.

The recommended next milestone is not expanded by this completion; any later Era
F rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Grid-Puzzle Game Class v1 governance refresh

Grid-Puzzle Game Class v1 (Era F Milestone 27 under #1) is now complete as the
Era F beachhead-genre milestone after the required implementation evidence
merged. The realized capability is a deterministic, probe-observable grid-puzzle
(block-pushing / Sokoban) game class that enters the existing loop and whose
acceptance — solvability and intended-solution replay — is **machine-checked by
the loop**, not asserted. This is the genre front door: a PuzzleScript-compatible
document is validated and loaded into the existing runtime, and the deterministic
verification loop decides it.

The merged evidence chain is the scope/design gate #1573
(`docs/grid-puzzle-game-class-v1.md`, PR #1621) — selecting the grid puzzle on
evidence and defining the game-class and DSL-ingest contracts without authorizing
behavior; the Grid-Puzzle Game Class and Runtime v1 #1574
(`examples/game-runtime/grid-puzzle.js`,
`crates/ouroforge-core/tests/grid_puzzle_game_class_contract.rs`, PR #1631) — a
deterministic, fixed-step, fully probe-exposed grid game class over the existing
runtime, fail-closed on malformed specs; the PuzzleScript-Compatible DSL Ingest v1
#1575 (`crates/ouroforge-core/src/grid_puzzle_dsl_ingest.rs`, PR #1753) — a
Rust/local **validate-then-load** front door that lowers a PuzzleScript-compatible
document into the `ouroforge.grid-puzzle.v1` game-class spec and rejects malformed
or out-of-subset input with a structured diagnostic, never a silent partial
import; the Grid-Puzzle Game Class Demo v1 #1576
(`docs/grid-puzzle-game-class-v1-demo.md`,
`examples/grid-puzzle-game-class-v1/demo/`, PR #1748) — a deterministic,
fixture-scoped demo that runs a grid-puzzle end to end with passing four-gate and
loop-coverage evidence recorded as a Milestone 24 ladder rung; and Scenario
Coverage v27 #1577 (`docs/scenario-coverage-v27.md`,
`crates/ouroforge-core/tests/scenario_coverage_v27_grid_puzzle.rs`, PR #1763) — an
enumerated grid-puzzle game-class regression matrix locking the run, gate, and
fail-closed behaviors.

The **remaining gaps** are demand-driven and tracked as their own rungs, not
backfilled here: the trusted bounded solver and over-solution detection are
Milestone 28 (#1579-#1585, recorded below, with the Design-Integrity Gate #1583
since merged); natural-language generation into this game class is Milestone 30; and
any engine breadth beyond the grid-puzzle rung's gate requires a separate scope
issue citing the specific gate it satisfies. The grid-puzzle class is the
beachhead, not a general engine claim.

The boundaries stay explicit and reaffirmed. Validation, the DSL ingest lowering,
the solver, and gate evaluation are owned by **Rust/local** and operate **over the
existing** `ouroforge.grid-puzzle.v1` state model and runtime — no new engine,
runtime, writer, or parser engine. Generation and DSL ingest are **proposal-only**
through the existing validate-then-load / review / apply / trust-gradient path and
are never a direct trusted write or auto-apply; browser/Studio surfaces remain
**read-only**. Genre and engine growth stay demand-driven under the Milestone 24
ladder. The wording is conservative: the demo proves the class runs and is
observable and carries no quality, fun, production-readiness, or Godot
replacement/parity claim. Existing runtime/probe/evaluator four-gate aggregation
contracts remain backward-compatible; generated runs/genre/evidence remain ignored
unless explicitly fixture-scoped; Layer-3 distributed orchestration / Elixir
remains NO-GO under ADR #92 and deferred under #1508. #1 and #23 remain open
governance anchors and are not modified, closed, or narrowed by this refresh.

### Generative Front Door v1 governance refresh

Generative Front Door v1 is recorded as **complete for Era F Milestone 30**
as a conservative, additive, local-only milestone under #1. It adds the
generation front door over the verification engine room: a non-developer can
describe a grid-puzzle in a plain brief, and the deterministic verification loop
decides whether the generated proposal has design integrity before it can be
promoted. Generation is the front door and the deterministic verification loop
is the engine room — layers, not alternatives. The completion evidence chain is
the parent scope/design gate #1592 (`docs/generative-front-door-v1.md`, PR
#1620); the Brief/NL Intake and Proposal Model v1 #1593
(`crates/ouroforge-core/src/generative_intake.rs`, PR #1699) — a plain brief
becomes a grid-puzzle *proposal* over the existing `MutationProposal` model with
generation provenance, proposal-only and never a trusted write; the Engine-Room
Promotion Guard v1 #1594 (`crates/ouroforge-core/src/generative_promotion_guard.rs`,
PR #1755) — a proposal is promotable only when it passes the engine room (the
deterministic solver #1580 and over-solution detector #1581 produce the facts,
the Milestone 28 design-integrity gate #1583 turns them into a verdict, and that
verdict is ANDed into the existing four-gate `declared-gate-and` aggregation),
otherwise it is blocked with a replayable evidence-linked reason; the
Non-Developer Accessibility Path v1 #1595
(`crates/ouroforge-core/src/generative_accessibility.rs`, PR #1758) — a
read-only view that surfaces the proposal, provenance, verdicts, and solver
trace, routing a verified proposal unchanged into the existing
review/apply/trust-gradient path and reporting an unverified brief without
promoting it; the Generative Front Door Demo v1 #1596
(`docs/generative-front-door-v1-demo.md`, PR #1771); and Scenario Coverage v30
#1597 (`docs/scenario-coverage-v30.md`, PR #1764). #1 and #23 remain open
governance anchors.

The boundaries stay explicit and reaffirmed. Generation **never performs a
trusted write**: it emits proposals only, and every promotion flows through the
existing review/apply/trust-gradient path (`source_apply_*`, `trust_gradient_*`),
never a direct trusted write, auto-apply, auto-merge, self-approval, or reviewer
bypass. Promotion stays **gated by the engine room**: no proposal that has not
passed the four gates plus solver and over-solution can be promoted, and the
guard adds no parallel aggregator. Browser and Studio surfaces remain
**read-only**. Rust/local owns the trusted validation, the solver/detector/gate
logic, and the proposal/provenance writing; the additions are backward-compatible
and preserve the existing runtime, probe, evaluator four-gate aggregation,
intake, solver, dashboard, cockpit, and review/apply/trust-gradient contracts.
"Verified" means only that a proposal passed the engine room — **not** that a
generated game is good, fun, shippable, or production-ready, and **not** a Godot
replacement or parity claim. Generated runs, genre, evidence, and registry
artifacts remain ignored unless explicitly fixture-scoped. Any hosted, paid, or
distributed capability stays **DEFER until a #1508 Layer-3 GO** (Layer-3 /
Elixir remains NO-GO under ADR #92).

The recommended next milestone is not expanded by this completion; any later Era
F rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Puzzle Solver and Over-Solution Detection v1 governance refresh

Puzzle Solver and Over-Solution Detection v1 (Era F Milestone 28 under #1) has
realized its moat capability on merged evidence: for an authored grid-puzzle
level, the deterministic verification loop can prove the level has **exactly its
intended solution**. Solvability is table stakes; the load-bearing deliverable
is over-solution detection, which surfaces any unintended shortcut as a
**replayable counterexample trace** ("watch the bypass") rather than a "trust
me". This is structurally possible only because the grid-puzzle runtime
(Milestone 27, #1574) is deterministic and fully observable.

The merged evidence chain is the scope/contract #1579
(`docs/puzzle-solver-oversolution-v1.md`, PR #1622); the Deterministic
Grid-Puzzle Solver v1 #1580 (`crates/ouroforge-core/src/puzzle_solver.rs`,
PR #1677) — bounded breadth-first search returning solvable with a replayable
witness, unsolvable only after full exploration, or bounded-search exhaustion
reported explicitly (never a false negative); the Designer Intent Capture and
Over-Solution Detector v1 #1581 (`puzzle_oversolution.rs`, PR #1708) — validated
intent capture and an exhaustive search for every distinct shorter unintended
solution, each a replayable trace, with no false positive on single-solution
levels and fail-closed on missing intent; the Difficulty Metric Artifact v1
#1582 (`puzzle_difficulty_metric.rs`, PR #1715) — solution length, branching
factor, dead-end density, and mechanic-introduction order computed from
solver/detector evidence (fail-closed on stale evidence), descriptive
measurement only; the Solver and Over-Solution Detection Demo v1 #1584
(`docs/puzzle-solver-oversolution-v1-demo.md`, PR #1722) — a deterministic
fixture-scoped demo where a dirty level's over-solution is caught with a trace
and the gate fails it while a clean level passes; and Scenario Coverage v28 #1585
(`docs/scenario-coverage-v28.md`, PR #1724) — an enumerated solver / detector /
difficulty / design-integrity-gate regression matrix plus a four-gate
`declared-gate-and` backward-compatibility golden.

The realized design-integrity gate verdict — *intent satisfied AND no unintended
over-solution* — composes via the existing evaluator `declared-gate-and`
aggregation and is demonstrated end to end (#1584) and regression-covered
(#1585). The Design-Integrity Gate v1 #1583
(`crates/ouroforge-evaluator/src/design_integrity_gate.rs`, PR #1751) — which
formalizes that gate as a declared gate inside the evaluator — has **since
merged**, so Milestone 28 is now recorded as **complete on merged evidence**. No
milestone is marked complete ahead of merged evidence.

The boundaries stay explicit and reaffirmed. The solver, detector, difficulty
metric, and gate semantics are owned by **Rust/local** and operate **over the
existing** `ouroforge.grid-puzzle.v1` state model and runtime — no new engine,
runtime, writer, or parallel evaluator. Detection, measurement, and gating only:
there is **no auto-fix** of detected over-solutions. The metrics are descriptive
and carry no difficulty, quality, fun, production-readiness, or Godot
replacement/parity claim. Generation stays proposal-only through the existing
review/apply/trust-gradient path; browser/Studio surfaces remain read-only;
existing runtime/probe/evaluator four-gate aggregation contracts remain
backward-compatible; generated runs/evidence remain ignored unless explicitly
fixture-scoped; Layer-3 distributed orchestration / Elixir remains NO-GO under
ADR #92 and deferred under #1508. #1 and #23 remain open governance anchors.

With the Design-Integrity Gate v1 #1583 merged, Milestone 28 is closed; no later
Era F rung is expanded by this refresh, and each requires its own scope issue with
explicit non-goals, regression coverage, generated-state audits, and the same
Layer-3 / hosted-paid boundaries.

### Foundation Hardening v1 governance refresh

Foundation Hardening v1 is recorded as **complete for Milestone A.H**, closing
the structural-drift item in #1's Roadmap Alignment Addendum. The completion
evidence is the merged issue/PR chain #1301, #1302, #1303, #1304, #1305, and
#1306, with zero-diff golden parity evidence and final full required gates
recorded on #1306. #1 and #23 remain open governance anchors.

The recommended next hardening direction is a later **A.H2** candidate focused
on one of the still-large `ouroforge-core` clusters: mutation/evolve, runtime,
behavior, or seed. This recommendation is not approval to start that work; an
A.H2 scope issue must define the exact crate seam, acyclic dependency direction,
re-export strategy, golden/parity gates, and no-behavior-change guardrails before
any extraction PR. Layer-3 scope remains unchanged and deferred: distributed
orchestration / Elixir per ADR #92, native export, plugin runtime, and
hosted/cloud still require separate design gates and governance approval.

### Full Studio Editor v1 governance refresh

Full Studio Editor v1 (#757-#776) is recorded as **complete as a bounded
local-first Studio foundation**, not as full Godot editor parity or a production
editor. The completed capability includes integrated Studio overview and
project context, scene tree, entity/component inspection, visual scene canvas,
draft-only authoring, Safe Source Apply handoff previews, asset browser,
scenario/playtest evidence, evidence timeline, export/package inspection,
plugin/extension descriptor inspection, workspace persistence, command palette,
accessibility/performance/diagnostics coverage, fixture-scoped integrated demo,
and Scenario Coverage v17 regression coverage. #1 and #23 remain open
governance anchors.

Remaining capability is deliberately separate from this completion: full Godot
parity, native desktop editor behavior, advanced visual scripting, a full asset
import pipeline, executable editor plugins, timeline/animation editor,
tilemap/terrain editor parity, production collaboration features, hosted/cloud
workflows, marketplace behavior, native export/release readiness, and the
Godot-plus demonstration game remain future scoped work. The browser Studio
continues to inspect trusted project state and create draft operations only; it
does not directly write trusted source files, execute shell commands, publish,
deploy, sign, upload, install or execute plugins, auto-apply, auto-merge,
self-approve, bypass review, or mutate CI/workflows. Trusted source mutation
still requires Safe Source Mutation Apply review gates: validated preview,
sandbox evidence, accepted independent review, stale-target checks, rollback
metadata, allowlisted verification, post-apply comparison, audit ledger, and
emergency hold checks.

### Godot-Plus Demo Game v1 governance refresh

Godot-Plus Demo Game v1 is recorded as **complete as a bounded local workflow
demonstration**, closing the Era C (#660-#797) governance chain without closing
#1 or #23. The completion evidence is the merged issue sequence #778-#797:
Signal Gate design and implementation, scenario/regression evidence,
QA swarm evidence, QA/playtest swarm evidence, agentic iteration and Safe
Source Apply chain evidence, read-only or draft-only Studio walkthrough
evidence, local web export/package verification, plugin descriptors used as
metadata, scoped comparison matrix, performance budget evidence,
documentation/reproducibility, and this roadmap refresh.

The recommended next technical sequence is not expanded by this completion.
Future work still needs separate scoped issues for any mature editor tooling,
native/mobile export, executable plugin ecosystem, real marketplace,
large-project production workflow, production collaboration, commercial release
readiness, production engine maturity, distributed orchestration / Elixir per
ADR #92, hosted/cloud behavior, or broader compatibility claim. #1 and #23
remain open governance anchors.

### Public Alpha Readiness handoff

The scope contract for this preparation milestone is [`docs/public-alpha-readiness-v1.md`](public-alpha-readiness-v1.md).

Public Alpha Readiness v1 is recorded as **prepared for manual public-visibility review, not launched**. The handoff lives in [`docs/public-alpha-readiness-governance-handoff-v1.md`](public-alpha-readiness-governance-handoff-v1.md). It summarizes readiness evidence, remaining manual launch boundaries, and conservative next milestone candidates without changing repository visibility, publishing packages, or approving public-launch messaging.

Actual visibility changes remain separate manual maintainer action after the launch checklist and decision-record process are rerun on the intended visibility date.


This roadmap/#1 governance refresh records the completed Visual Authoring v1 /
Safe Local Edit Cockpit milestone while preserving conservative public wording
and leaving #1/#23 open. Visual Authoring v1 (#343-#354) is now complete as a
local-first draft/preview/review evidence milestone; it is not a public-readiness
or production-editor claim.

After #387, Public Alpha Readiness (#367-#377) and Public Alpha Launch
Governance (#378-#387) are recorded as governance/readiness preparation tracks,
not launch execution. They produced readiness evidence, manual visibility
decision inputs, hold/rollback criteria, issue/PR intake policy, security and
demo-stability boundaries, conservative communication material, and this
roadmap/#1 refresh. They did not change repository visibility, publish packages,
announce launch, add support commitments, or implement product behavior.

The current governance outcome is **manual hold / ready for separate maintainer
decision**. Maintainers can either rerun the visibility checklist and decision
record on a chosen date, or continue technical roadmap work while keeping launch
actions manual. Production 2D Engine Core v1 (#581-#593) is now recorded as
complete, with this #594 roadmap/#1 refresh preserving the evidence chain and
conservative wording.
The 3D Capability Gate v1 sequence (#596-#608) and Agentic Scene and Level
Designer v1 sequence (#627-#642) are now recorded as complete after
implementation, demo, regression, Studio inspection, and governance evidence.
Gameplay Scripting / Logic System v1 (#611-#625) is now recorded as complete,
framed as bounded local behavior/evidence contracts rather than arbitrary
scripting, plugin loading, hosted execution, browser command bridges, source
apply, public launch, production readiness, or Godot replacement scope.


Other possible later governance topics remain Native Export Design Gate, Plugin
Design Gate, Source Mutation Apply Design Gate, and Visual Authoring v2. None is
authorized as implementation scope unless a separate fixed issue sequence opens
it with explicit non-goals, regression coverage, generated-state audits, and
manual launch boundaries.

The public-readiness and post-launch triage docs remain governance inputs, not
automated launch or roadmap-acceptance paths:

- `docs/public-readiness-audit.md`
- `docs/public-demo-evidence.md`
- `docs/public-launch-checklist.md`
- `docs/public-alpha-launch-governance-v1.md`
- `docs/public-alpha-communication-pack-v1.md`
- `docs/post-launch-roadmap-triage-v1.md`

Maintainers should use those documents for separate manual repository visibility
and post-launch roadmap decisions. Public launch and future milestone acceptance
remain governance actions, not automated code paths.

### Public Alpha Launch Governance outcome

Public Alpha Launch Governance v1 (#378-#387) is recorded as **governance
complete, launch not executed**. The completed artifacts define how maintainers
may make, hold, communicate, audit, and roll back a future public-alpha decision,
but the repository remains under the same manual visibility, release,
publication, and support boundaries described above.

Recommended next milestone candidates after this governance refresh are:

1. **Autonomous QA / Playtest Swarm v1 (#682-#698) is complete as bounded
   evidence-gated QA/playtest infrastructure**. The completed scope includes
   scenario candidates, adversarial fuzz plans, worker/budget policy, runtime
   invariants, route attempts, visual/performance/error evidence,
   flake/rerun policy, failure classification, backlog, run matrix, evidence
   bundle, Studio inspection, demo, Scenario Coverage v13, generated-state
   audits, and conservative wording. It remains evidence/backlog-only and must
   not become hidden workers, remote/cloud swarm orchestration, autonomous
   repair, browser trusted writes, command bridges, auto-fix, auto-apply,
   auto-merge, self-approval, production QA certification, public-launch
   approval, current Godot replacement, or production-ready claims.
2. **Safe Source Mutation Apply v1 (#699-#716) is complete as bounded
   review-gated trusted source-like apply**. Continue only through fixed scoped
   issue sequences for Full Studio Editor, Plugin / Extension System, and
   Godot-Plus Demo; preserve Rust/local trusted persistence, independent review,
   rollback, stale-target, sandbox, allowlisted verification, post-apply evidence,
   audit ledger, and emergency hold gates.
3. **Safe Executable Script Implementation Gate** — defer until maintainers open
   a separate governance issue with threat model, test matrix, rollback story,
   review-gate story, generated-state policy, and public wording; #611-#625 do
   not authorize arbitrary executable scripts.
4. **Bounded Production 3D expansion** — defer unless maintainers open a new
   evidence-backed scope contract after the 3D gate. The completed gate is not
   proof of broad 3D engine parity, advanced rendering, import pipelines, native
   export, production readiness, or Godot replacement status.
5. **Manual public visibility review** — only if maintainers separately choose
   it, using the launch checklist, visibility decision records, communication
   pack, hold/rollback criteria, issue/PR intake policy, security-response
   routing, and fresh verification on the intended decision date.
6. **Deferral/hold** — if any launch-governance blocker or generated-state,
   wording, demo-stability, security-reporting, or #1/#23 anchor concern is
   found during a future manual review.

None of these candidates is automatically accepted by this roadmap refresh.
Technical work still requires its own scoped issue/PR sequence; visibility,
announcement, release, package publication, support, and security guarantees
remain separate maintainer actions.

### Asset Generation and Asset-QA v1 governance refresh

Asset Generation and Asset-QA v1 is recorded as **complete for Era G Milestone
36** under #1, on merged evidence, as a conservative, additive, local-only
milestone. It gives the loop a **verified visual-asset function** (sprites,
tilesets, UI art) without asset slop or license risk: a generated asset is a
**proposal carrying license/provenance**, routed through the existing
review/apply/trust-gradient path, and is promoted only after a function-specific
**asset-QA gate** passes. The function is a composition of surfaces that already
exist — it adds no new engine, runtime, or writer.

The merged evidence chain is the scope/design gate #1634
(`docs/asset-pipeline-design.md`, PR #1703); the Asset Generation Proposal Model
v1 #1635 (`crates/ouroforge-core/src/asset_generation_proposal.rs`, PR #1709) —
generation emits a `MutationProposal` carrying an `AssetLicenseProvenance`
record, proposal-only (proposed / pending / unverified), failing closed on a
missing/unrecognized license, a missing required attribution, an off-list source,
or a malformed/oversize descriptor; the Asset-QA Gate v1 #1636
(`crates/ouroforge-evaluator/src/asset_qa_gate.rs`, PR #1714) — style-consistency,
format/resolution validity, visual-regression vs baseline, and license/provenance
completeness, composing additively with the existing four gates under
`declared-gate-and` (`undeclaredGatePolicy: neutral`), reusing the visual gate and
failing closed (a missing/non-comparable baseline is explicit `insufficient-data`,
never a silent pass); the Asset Import and Atlas Path v1 #1637
(`crates/ouroforge-core/src/asset_import.rs`, PR #1720) — validate-then-load
reusing the existing `ProjectAssetManifest` loader and atlas-integrity validation,
where every generated asset must have passed asset-QA before import; the Asset
Generation and QA Demo v1 #1638 (`docs/asset-pipeline-v1-demo.md`, PR #1727) — a
deterministic, fixture-scoped walkthrough where an asset is generated, blocked by
asset-QA when style-inconsistent, and promotable only when it passes, with
promotion routed through the gate; and Scenario Coverage v34 #1639
(`docs/scenario-coverage-v34.md`, PR #1730) — an enumerated, state/shape-only
regression suite over the proposal, asset-QA, and import behaviors plus a
four-gate backward-compatibility guarantee. #1 and #23 remain open governance
anchors.

The boundaries stay explicit and reaffirmed. **License/provenance and the
asset-QA gate are mandatory before any asset promotion; the gate fails closed.**
No unlicensed, uncredited, or unverified-style generated asset is ever promoted.
Generation stays proposal-only through the existing review/apply/trust-gradient
path, never a direct trusted write, auto-apply, self-approval, or reviewer bypass;
browser/Studio surfaces remain read-only. **Taste stays human**: the gate asserts
license/format/style-baseline/regression conformance, never that an asset "looks
good", is on-brand-by-taste, or is fun — art/audio/UX/narrative direction remain
human decisions. The function reuses the existing proposal model, evaluator four
gates, `compare`, provenance bundle, and Asset Pipeline v1 manifest/loader/atlas;
it adds no parallel engine. Existing contracts remain backward-compatible, and
generated assets/runs/artifacts remain ignored unless explicitly fixture-scoped.
A hosted/paid asset store stays **DEFER until a #1508 Layer-3 GO** (Layer-3
distributed orchestration / Elixir remains NO-GO under ADR #92). No
production-readiness, quality, fun, or Godot replacement/parity claim is
introduced.

The recommended next milestone is not expanded by this completion; any later Era
G rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Audio Generation and Audio-QA v1 governance refresh

Audio Generation and Audio-QA v1 is recorded as **complete for Era G Milestone
37** under #1, on merged evidence, as a conservative, additive, local-only
milestone. It extends the verified-asset pattern to **audio** (SFX, music,
adaptive audio): a generated audio asset is a **proposal carrying
license/provenance**, routed through the existing review/apply/trust-gradient
path, and is promotable only after a function-specific **audio-QA gate** passes.
The function is a composition of surfaces that already exist — it adds no new
audio engine, runtime, or writer.

The merged evidence chain is the scope/contract gate #1641
(`docs/audio-pipeline-v1.md`, PR #1705); the Audio Generation Proposal Model v1
#1642 (`crates/ouroforge-core/src/audio_generation.rs`, PR #1713) — generation
emits a `MutationProposal` carrying mandatory license/provenance, proposal-only
(proposed / pending / unverified), failing closed on a missing license, a blank
credit, or a malformed audio descriptor, and routing through the existing
trust-gradient as manual-only; the Audio-QA Check v1 #1643
(`crates/ouroforge-core/src/audio_qa.rs`, PR #1719) — format/loudness validity,
license/provenance completeness, and regression vs baseline, composing additively
under `declared-gate-and` (`undeclaredGatePolicy: neutral`) and failing closed
while preserving the `stale > fail > pass` precedence at the aggregation boundary;
the Adaptive-Audio Runtime Hooks v1 #1644
(`crates/ouroforge-core/src/audio_hooks.rs` plus the deterministic runtime mirror
`examples/game-runtime/audio-hooks.js`, PR #1729) — adaptive-audio hooks that emit
`BehaviorIntent` audio intents from a bounded world-state signal snapshot,
reusing the existing runtime audio-intent surface, deterministic and
snapshot/restore-stable; the Audio Generation and QA Demo v1 #1645
(`docs/audio-pipeline-v1-demo.md`, PR #1734) — a deterministic, fixture-scoped
walkthrough where audio is generated proposal-only, blocked when
unlicensed/invalid, and promotable only when verified, with adaptive hooks firing
deterministically; and Scenario Coverage v35 #1646
(`docs/scenario-coverage-v35.md`, PR #1737) — an enumerated, state/shape-only
regression suite over the proposal, audio-QA, and hook behaviors plus a
backward-compatibility guarantee that the existing runtime audio-intent emission
remains valid. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. **License/provenance and the
audio-QA gate are mandatory before any audio promotion; the gate fails closed.**
No unlicensed, uncredited, or unverified-style generated audio is ever promoted.
Generation stays proposal-only through the existing review/apply/trust-gradient
path, never a direct trusted write, auto-apply, self-approval, or reviewer bypass;
browser/Studio surfaces remain read-only and the runtime/probe stays read-only
with respect to trusted state. **Taste stays human**: the gate asserts
format/loudness/license/regression conformance, never that audio "sounds good" or
is fun — sound direction remains a human decision. The function reuses the
existing proposal model, trust-gradient, evaluator aggregation, `compare`,
provenance bundle, asset-manifest audio classification, and the runtime
audio-intent surface; it adds no parallel engine. Existing contracts remain
backward-compatible, and generated audio/runs/artifacts remain ignored unless
explicitly fixture-scoped. A hosted/paid audio store stays **DEFER until a #1508
Layer-3 GO** (Layer-3 distributed orchestration / Elixir remains NO-GO under ADR
#92). No production-readiness, quality, fun, or Godot replacement/parity claim is
introduced.

The recommended next milestone is not expanded by this completion; any later Era
G rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Long-Form Game Systems v1 governance refresh

Long-Form Game Systems v1 is recorded as **complete for Era G Milestone 39**
under #1, on merged evidence, as a conservative, additive, local-only milestone.
It adds the systems a longer-form game needs — meta-progression/unlocks,
economy/currency, save/profile and run-history at scale, UI/UX flow with
onboarding and accessibility, and an optional narrative/dialogue/event system —
each as a **Milestone 24 ladder rung** proven by a loop-produced, evidence-backed
demo. Each system is a trusted Rust/local **data system on the existing runtime**;
it adds no new engine, runtime, writer, or save service.

The merged evidence chain is the scope/contract gate #1656
(`docs/long-form-systems-v1.md`, PR #1707) — defines each system as a gated rung
and the Rust-trusted-state vs JS-runtime-UI boundary; Meta-Progression and
Unlocks v1 #1657 (`crates/ouroforge-core/src/meta_progression.rs`, PR #1711) —
deterministic cross-run progression counters and threshold-gated, monotonic
unlocks, with restore validation that rejects an unlock not justified by its
counters; Economy and Currency v1 #1658
(`crates/ouroforge-core/src/economy_system.rs`, PR #1716) — an integrity-checked
currency ledger with checked earn/spend and a fail-closed non-negative invariant;
Save/Profile and Run-History at Scale v1 #1659
(`crates/ouroforge-core/src/save_profile_scale.rs`, PR #1725) — a multi-profile
save store with a chained per-profile history digest (reusing the in-tree
SHA-256 hasher) and a `save-profile-v0` migration path; UI/UX Flow, Onboarding
and Accessibility v1 #1660 (`examples/game-runtime/uiux-flow.js` plus the trusted
contract `crates/ouroforge-core/src/uiux_flow.rs`, PR #1733) — a deterministic,
probe-observable in-game flow with reachable screens and declared accessibility
options, wired into the existing runtime and exposed read-only; the optional
Narrative/Dialogue/Event System v1 #1661
(`crates/ouroforge-core/src/narrative_system.rs`, PR #1736) — a data-driven
dialogue graph and flag-conditioned events with deterministic fixpoint
evaluation and fail-closed restore validation; the Long-Form Game Systems Demo v1
#1662 (`docs/long-form-systems-v1-demo.md`, PR #1739) — a deterministic,
fixture-scoped slice composing the systems and recording each as a **satisfied**
Milestone 24 rung with passing four-gate and loop-coverage evidence via the
complexity-ladder contract; and Scenario Coverage v37 #1663
(`docs/scenario-coverage-v37.md`, PR #1740) — an enumerated, state/shape-only
regression suite over all five systems plus a backward-compatibility guarantee
that an existing single-run save/restore remains valid. #1 and #23 remain open
governance anchors.

The boundaries stay explicit and reaffirmed. Growth stays **demand-driven**:
each system is a Milestone 24 rung claimed only on loop-produced, four-gate +
loop-coverage evidence, never breadth for its own sake. **UX and narrative tone
remain human decisions** — the contracts assert structure (determinism,
reachability, accessibility presence, deterministic dialogue/event firing), never
that a UI "looks good" or a story is good or fun. Trusted state is Rust/local and
changes only through validated, fail-closed logic; the in-game UI is the
deterministic JS runtime and is read-only with respect to trusted state, exposed
through the existing `window.__OUROFORGE__` probe; generation stays proposal-only
through the existing review/apply/trust-gradient path; generated assets/content
require license/provenance and the function-specific QA gate before promotion.
Existing contracts remain backward-compatible (the v0→v1 save migration preserves
prior saves), and generated runs/profiles/saves/artifacts remain ignored unless
explicitly fixture-scoped. Shipping/hosted/cloud/live-ops stays **DEFER until a
#1508 Layer-3 GO** (Layer-3 distributed orchestration / Elixir remains NO-GO
under ADR #92). No production-readiness, quality, fun, or Godot replacement/parity
claim is introduced.

The recommended next milestone is not expanded by this completion; any later Era
G rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.


### Card-Roguelite Substrate v1 governance refresh (Era I Milestone 47)

Card-Roguelite Substrate v1 is recorded as **complete for Era I Milestone 47**
under #1, on merged evidence, as a conservative, additive, Rust/local milestone.
It generalizes the existing deck-roguelike class into a validated
card-roguelite substrate, preserves the pre-substrate deck-roguelike golden,
adds an engine-builder deckbuilder variant strictly as a substrate config,
proves a deterministic fixture-scoped demo, and locks the surface with Scenario
Coverage v42. The milestone is a mechanical/balance substrate and regression
coverage milestone only; it does not claim that a game is fun, shippable,
production-ready, or a Godot replacement.

The merged evidence chain is the scope/design gate #1791 (PR #1892) — records
the substrate-as-config rule, Rust/local trust boundary, proposal-only generation
path, and human Era J fun/release gates; Card-Roguelite Substrate Core Model v1
#1792 (PR #1897) — adds the validated cards/modifiers/run/shop/meta substrate,
deterministic seeded resolution, digest/probe state, and read-only inspection
shape; Deck-Roguelike-as-Substrate-Config parity #1793 (PR #1898) — migrates the
existing deck-roguelike fixture to the substrate while preserving golden-byte
parity and fail-closed malformed behavior; Engine-Builder Deckbuilder Config v1
#1794 (PR #1903) — adds the first deckbuilder variant as a distinct config over
the same substrate, not a parallel engine; Card-Roguelite Substrate Demo v1 #1795
(PR #1909) — runs the deck-roguelike parity config and engine-builder config
deterministically from fixture-scoped docs/manifest evidence; and Scenario
Coverage v42 #1796 (PR #1915) — enumerates state/shape-only regressions for
determinism, config parity, engine-builder config, demo shape, pre-substrate
backward compatibility, generated-state wording, and governance. #1 and #23
remain open governance anchors.

The boundaries stay explicit and reaffirmed. A deckbuilder variant is a
configuration over the Card-Roguelite Substrate, not a new engine or browser/
Studio writer. Trusted validation, persistence, substrate scoring/balance,
export/provenance logic, evidence writing, run/project binding, review/apply/
trust-gradient path, and CLI behavior remain Rust/local. JavaScript/browser/
Studio surfaces remain deterministic or read-only inspection surfaces for trusted
state and do not gain trusted writes, command bridges, autonomous apply,
auto-merge, self-approval, reviewer bypass, or hidden mutation authority.
Generation remains proposal-only through the existing review/apply/trust-gradient
path. The fun/feel verdict, Steam account/signing/release, content survey,
market demand, hosted/cloud/mobile Layer-3 capability, and release go/no-go stay
human or deferred as previously scoped. Existing runtime, probe, evaluator,
evolve/campaign, compare, provenance-bundle, asset, dashboard, cockpit, and CLI
contracts remain backward-compatible; generated runs/assets/builds remain ignored
unless explicitly fixture-scoped. Scenario Coverage numbering continues through
v42.

The recommended next milestone is not expanded by this completion; any later Era
I/J work requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Era H closing governance and final autonomy assessment (Era H Milestone 46)

Era H Milestones 42-45 are recorded as **complete for Era H** under #1 only after
merged evidence for each milestone. This closing refresh is documentation and
governance only: it reviews #1, README, the roadmap, and the Era H milestone docs
for drift; records the completed evidence chain; and publishes the final autonomy
assessment in `docs/era-h-autonomy-assessment.md`. It adds no new capability, no
runtime behavior, and no shipping/liveops implementation.

The merged evidence chain is Multi-Agent Production Pipeline v1 / Milestone 42
#1674-#1681 (PRs #1704/#1790/#1876/#1877/#1878/#1879/#1880; Scenario Coverage
v39) — role agents, handoff/conflict resolution, reviewer/critic gates, demo,
regression coverage, and governance; Autonomous Producer and Whole-Game
Orchestration v1 / Milestone 43 #1682-#1688 (PRs #1701/#1884/#1885/#1888/#1890/
#1891/#1902; Scenario Coverage v40) — design-intent decomposition, orchestration
state, budgets, stop conditions, human approval gates, demo, coverage, and
governance; Scaled Trust Gradient, Release Provenance and Compliance v1 /
Milestone 44 #1689-#1696 (PRs #1702/#1906/#1910/#1956/#1964/#1973/#1987;
Scenario Coverage v41) — broadened-but-bounded low-risk rollback-backed
auto-apply, per-release provenance, compliance gate, demo, coverage, and
governance; and Shipping and LiveOps Layer-3 Re-evaluation Design Gate v1 /
Milestone 45 #1697 (PR #1997) — DEFER for native/store export, real-player
telemetry, live balancing, and update/patch pipelines absent a separate #1508
Layer-3 GO. #1 and #23 remain open governance anchors.

The final autonomy assessment is descriptive and evidence-scoped. For the two
tracked genre lines (the collect-and-exit / grid-puzzle line and the Signal Gate
/ deck-roguelike-to-deckbuilder line), the local evidence stack can automate or
agent-coordinate deterministic proposal creation, run/evidence capture,
mechanical evaluation, regression comparison, role handoff, producer planning,
source/release-candidate provenance, and blocked compliance/readiness reporting.
It does **not** automate vision, target audience judgment, taste, art/audio/UX/
narrative direction, fun/quality verdicts, legal/compliance acceptance,
market/distribution decisions, or release go/no-go. Concept-to-release autonomy
therefore stops at a **local web release candidate with synthetic and
fixture-scoped evidence**; the release decision remains human.

The permanent boundaries stay explicit. Generation, role-agent, and producer
outputs remain proposal-only through the existing review/apply/trust-gradient
path. High-risk and source-affecting changes never auto-apply. Browser, Studio,
dashboard, and cockpit surfaces remain read-only for trusted state. No
auto-merge, self-approval, reviewer bypass, hidden trusted writes, automated
quality/fun/taste claim, production-ready claim, Godot replacement/parity claim,
or autonomous-shipping claim is introduced. Shipping/native-store export,
hosted/cloud, real-player telemetry, live balancing, update/patch pipelines, and
live-ops remain **DEFER absent a separate #1508 Layer-3 GO**; distributed/Elixir
remains NO-GO for Layer-3 under ADR #92. Generated runs/assets/content/release
artifacts remain untracked unless explicitly fixture-scoped.

The recommended next work is not expanded by Era H closure. Later Era I/J work
must remain issue-scoped, evidence-backed, and human-gated, with the same
Layer-3, generated-state, and conservative-wording audits.

### Scaled Trust Gradient, Release Provenance and Compliance v1 governance refresh (Era H Milestone 44)

Scaled Trust Gradient, Release Provenance and Compliance v1 is recorded as **complete for
Era H Milestone 44** under #1, on merged evidence, as a conservative, additive,
local-only release-trust milestone. It scales the Milestone 22 trust gradient only
inside bounded, rollback-backed low-risk data/scene changes, adds a per-release
provenance bundle, adds a compliance reviewer gate, demonstrates the release-trust
path with fixture-scoped evidence, and locks Scenario Coverage v41. The milestone
records release evidence and blockers; it does not create release authority, a
browser/Studio writer, or an autonomous shipping path.

The merged evidence chain is the scope/contract gate #1689
(`docs/release-trust-provenance-v1.md`, PR #1702) — records the scaled trust,
release provenance, and compliance contracts while keeping high-risk and
source-affecting work outside auto-apply; Broadened Bounded Auto-Apply and
Game-Scale Rollback v1 #1690 (`crates/ouroforge-core/src/release_auto_apply.rs`,
PR #1906) — extends only the low-risk rollback-backed data/scene tier and keeps
high-risk, source-affecting, CI/workflow, dependency, credentialed, and release
mutations manual-review; Per-Release Provenance Bundle v1 #1691
(`crates/ouroforge-core/src/release_provenance_bundle.rs`, PR #1910) — records
linked release evidence, artifact/license/provenance refs, compliance refs, and a
human go/no-go placeholder without publishing; Compliance Reviewer Gate v1 #1693
(`crates/ouroforge-core/src/release_compliance_gate.rs`, PR #1956) — blocks
release readiness until license, policy, age-rating, artifact provenance, and the
human go/no-go checks pass; Scaled Trust, Release Provenance and Compliance Demo
v1 #1694 (`docs/release-trust-provenance-v1-demo.md`, PR #1964) — proves the
local fixture-scoped path while preserving blocked compliance and human-go/no-go
evidence; and Scenario Coverage v41 #1695 (`docs/scenario-coverage-v41.md`, PR
#1973) — locks release-trust regressions for low-risk auto-apply boundaries,
source/high-risk blocking, provenance bundle shape, compliance blocking, demo
evidence, and governance. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. High-risk and source-affecting
changes **never auto-apply**; they remain manual-review proposals through the
existing review/apply/trust-gradient path. Release requires compliance plus a
human go/no-go, and missing license, policy, age-rating, provenance, artifact, or
human approval evidence blocks the release-trust verdict. Rust/local owns trusted
validation, persistence, provenance/compliance logic, evidence writing,
run/project binding, source-apply, and CLI behavior. Browser, Studio, dashboard,
and cockpit surfaces remain deterministic/read-only inspection surfaces for
trusted state and gain no trusted writes, command bridges, autonomous apply,
auto-merge, self-approval, reviewer bypass, or hidden mutation authority.
Generated runs/assets/content/release artifacts remain ignored unless explicitly
fixture-scoped. Shipping, hosted/cloud, real-player telemetry, and live-ops stay
Layer-3 gated (DEFER per #1508), distributed orchestration / Elixir remains
NO-GO under ADR #92, and no production-ready, quality/fun, Godot
replacement/parity, or autonomous-shipping claim is introduced. Scenario Coverage
numbering continues from the Era H sequence through v41.

The recommended next milestone is not expanded by this completion; any later Era
I/J rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Autonomous Producer and Whole-Game Orchestration v1 governance refresh (Era H Milestone 43)

Autonomous Producer and Whole-Game Orchestration v1 is recorded as **complete for
Era H Milestone 43** under #1, on merged evidence, as a conservative, additive,
local-only milestone. It extends the Era H accountability framework from
role-agent collaboration into proposal-only whole-game production orchestration:
a human design intent can be decomposed into a deterministic production plan,
tracked through game-scale orchestration state, constrained by explicit budgets,
stop conditions, and mandatory human approval gates, demonstrated in a
fixture-scoped autonomous producer slice, and locked by Scenario Coverage v40.
The producer is a coordinator of evidence and proposals, not a release authority
or trusted writer.

The merged evidence chain is the scope/contract gate #1682
(`docs/autonomous-producer-v1.md`, PR #1701) — records the autonomous producer
GO/DEFER decision, whole-game orchestration contract, human-gate boundaries, and
proposal-only release-candidate path; Design-Intent Decomposition and Production
Plan v1 #1683 (`crates/ouroforge-core/src/producer_plan.rs`, PR #1884) — turns a
human design intent into a deterministic, Rust/local production task graph while
reusing the existing GDD/design-brief and Milestone 30 generation-planning
surfaces; Whole-Game Orchestration State v1 #1684
(`crates/ouroforge-core/src/producer_orchestration.rs`, PR #1885) — tracks
producer state over existing evolve-campaign and Milestone 42 handoff/review
evidence instead of adding a parallel orchestrator; Budgets, Stop Conditions and
Human Approval Gates v1 #1685 (`crates/ouroforge-core/src/producer_budget_gates.rs`,
PR #1888) — fails closed on budget exhaustion, mandatory human-gate blocks, and
no-progress stops using the existing campaign-budget and stop-condition shapes;
Autonomous Producer Demo v1 #1686 (`docs/autonomous-producer-v1-demo.md`, PR
#1890) — proves a deterministic intent-to-release-candidate audit trail with a
pending human release gate and safe budget halt; and Scenario Coverage v40 #1687
(`docs/scenario-coverage-v40.md`, PR #1891) — enumerates state/shape-only
regression coverage across planning, orchestration, gates, demo evidence, and
single-artifact campaign backward compatibility. #1 and #23 remain open
governance anchors.

The boundaries stay explicit and reaffirmed. Producer, generation, and role-agent
outputs are **proposals only** through the existing review/apply/trust-gradient
path — never a direct trusted write, release, auto-apply, auto-merge,
self-approval, reviewer bypass, or hidden trusted mutation. High-risk and
source-affecting changes remain blocked until explicit review and human approval;
the release go/no-go is human. Budgets, stop conditions, and approval gates are
mandatory and bounded; no unbounded autonomy is introduced. Generated
assets/audio/content require license/provenance and the function-specific QA gate
before promotion. Trusted logic is **Rust/local**; the deterministic JS runtime,
`window.__OUROFORGE__` probe, dashboard, cockpit, and Studio surfaces remain
**read-only** with respect to trusted state. Existing runtime, probe, evaluator,
evolve/campaign, compare, provenance-bundle, asset-manifest, QA-swarm, dashboard,
cockpit, source-apply, and CLI contracts remain backward-compatible, and
generated runs/assets/content/artifacts remain ignored unless explicitly
fixture-scoped. Shipping/hosted/cloud/live-ops stays **DEFER until a #1508
Layer-3 GO** (distributed orchestration / Elixir remains NO-GO under ADR #92). No
production-readiness, quality, fun, Godot replacement/parity, or autonomous
shipping claim is introduced. Scenario Coverage numbering continues from the Era
F/G/H sequence through v40.

The recommended next milestone is not expanded by this completion; any later Era
H/I rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Multi-Agent Production Pipeline v1 governance refresh (Era H Milestone 42)

Multi-Agent Production Pipeline v1 is recorded as **complete for Era H Milestone
42** under #1, on merged evidence, as a conservative, additive, local-only
milestone. It realizes the Milestone 13 role model as **evidence-gated,
proposal-only collaboration**: role-specialized agents propose artifacts, hand
work off to one another, and a reviewer/critic gate blocks promotion until an
independent review passes. Every contract **reuses an existing surface** — the
Milestone 13 agent role set, the evidence/journal refs, and the Milestone 22
trust gradient and review/apply path; it adds **no new orchestration engine,
runtime, writer, scheduler, or worker pool**, and **no role agent ever performs a
direct trusted write**.

The merged evidence chain is the scope/design gate #1674
(`docs/production-pipeline-design.md`, PR #1704) — records the GO/DEFER decision,
the role-agent model, artifact ownership, handoff/conflict-resolution, and
reviewer/critic promotion-gate contracts, with DEFER as the default outside the
bounded scope; Role Agent Model and Artifact Ownership v1 #1675
(`crates/ouroforge-core/src/production_roles.rs`, PR #1790) — per-artifact
ownership where each class has a single owning role, an unauthorized non-owner
write and a direct trusted write are rejected fail-closed, and ownership and
outcomes are recorded as observability evidence; Handoff Artifacts and Conflict
Resolution v1 #1676 (`crates/ouroforge-core/src/production_handoff.rs`, PR #1876)
— role-to-role handoffs with deterministic conflict resolution where concurrent
edits to one base are blocked and preserved (never auto-merged) and stale-ref
handoffs are needs-fix, failing closed on any declared/computed mismatch;
Reviewer/Critic Promotion Gates v1 #1678
(`crates/ouroforge-core/src/production_review_gates.rs`, PR #1877) — gates that
block promotion until an independent reviewer approves and the critic does not
veto, with higher trust-gradient risk requiring stronger review and
`promote-allowed` never auto-applying, across distinct implementer/reviewer/critic
actors (no self-approval); the Multi-Agent Production Pipeline Demo v1 #1679
(`docs/production-pipeline-v1-demo.md`, PR #1878) — a deterministic,
fixture-scoped slice where three role agents collaborate with handoffs and the
same gate progresses from blocked to promote-allowed only after review; and
Scenario Coverage v39 #1680 (`docs/scenario-coverage-v39.md`, PR #1879) — an
enumerated, state/shape-only regression suite over the role/handoff/gate
behaviors plus a backward-compatibility golden proving the existing single-agent
evolve/apply flows remain valid. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Role agents, generation, and the
producer emit **proposals only** through the existing review/apply/trust-gradient
path — never a direct trusted write, auto-apply, auto-merge, self-approval, or
reviewer bypass; high-risk and source-affecting changes are never auto-applied.
Handoff conflicts are surfaced and preserved, never silently merged, and
promotion requires an independent review/critic gate over the trust gradient with
the human release go/no-go preserved. Generated assets/audio/content require
license/provenance and the function-specific QA gate before promotion. Trusted
logic is **Rust/local**; the runtime, `window.__OUROFORGE__` probe, and
dashboard/cockpit/Studio surfaces remain **read-only**. The suite asserts states
and shapes only, with no flaky/timing-based assertions, no network, and no live
browser. Existing contracts remain backward-compatible, and generated
runs/assets/content/artifacts remain ignored unless explicitly fixture-scoped.
Shipping/hosted/cloud/live-ops stays **DEFER until a #1508 Layer-3 GO**
(distributed orchestration / Elixir remains NO-GO under ADR #92). No
production-readiness, quality, fun, Godot replacement/parity, or autonomous
shipping claim is introduced — this is an accountability and review-governance
framework, not an autonomous studio. Scenario Coverage numbering continues from
the Era F sequence (v39).

### Production-Scale QA Matrix v1 governance refresh

Production-Scale QA Matrix v1 is recorded as **complete for Era G Milestone 40**
under #1, on merged evidence, as a conservative, additive, local-only milestone.
It scales QA from per-artifact checks to whole-game production QA — a regression
matrix across content variants, seeds, and supported targets; visual-regression
at scale; performance/soak testing; crash/flaky/accessibility/asset-UX checks;
and a single consolidated production-QA verdict per build. Every capability
**reuses an existing runner** (the QA / playtest swarm, the visual gate, the
frame-budget surface, and the evaluator aggregation); it adds **no new test
engine, profiler, or evaluator**.

The merged evidence chain is the scope/contract gate #1665
(`docs/production-qa-matrix-v1.md`, PR #1700) — defines the matrix, visual,
soak, crash/accessibility, and verdict contracts and the reuse statement;
Regression Matrix (Content x Seed x Target) v1 #1666
(`crates/ouroforge-core/src/production_qa_matrix.rs`, PR #1710) — a regression
matrix over existing runners that aggregates per-coordinate verdicts and detects
cross-variant regressions with replayable evidence; Visual-Regression at Scale v1
#1667 (`crates/ouroforge-core/src/visual_regression_scale.rs`, PR #1712) — scales
the existing visual gate across screens/content variants, reusing the evaluator
`VisualComparisonOutcome`, surfacing detected diffs and missing baselines
explicitly; Performance and Soak Testing v1 #1668
(`crates/ouroforge-core/src/performance_soak.rs`, PR #1718) — soak/performance
runs over the existing frame-budget surface with a deterministic
`pass`/`regressed`/`unstable` verdict on integer milli-unit samples (no live
timing); Crash/Accessibility QA and Consolidated Production-QA Verdict v1 #1669
(`crates/ouroforge-core/src/production_qa_verdict.rs`, PR #1723) — composes the
per-check results into one descriptive verdict via the evaluator
`declared-gate-and` aggregation, failing closed when any declared check fails;
the Production-Scale QA Matrix Demo v1 #1670
(`docs/production-qa-matrix-v1-demo.md`, PR #1742) — a deterministic,
fixture-scoped slice where the matrix catches a planted cross-variant regression
and the consolidated verdict fails closed; and Scenario Coverage v38 #1671
(`docs/scenario-coverage-v38.md`, PR #1743) — an enumerated, state/shape-only
regression suite over the matrix/visual/performance/verdict behaviors plus a
backward-compatibility golden proving the existing per-artifact gates remain
valid. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. **QA is descriptive, never a
quality/fun guarantee**: the consolidated verdict aggregates bounded evidence
signals and is not proof of fun, accessibility compliance, market readiness,
production safety, current Godot replacement/parity, or shipped-game readiness;
"looks good / sounds good / is fun" remains a human decision and the human
release go/no-go is preserved. QA outputs are **evidence inputs only** — never a
trusted mutation, auto-fix, auto-apply, auto-merge, self-approval, or reviewer
bypass; generation/role-agent/producer output stays proposal-only through the
existing review/apply/trust-gradient path; generated assets/content require
license/provenance and the function-specific QA gate before promotion. Trusted
logic is Rust/local; browser/Studio surfaces remain read-only. QA is **synthetic
only** (no real-player Layer-3 data) with no flaky/timing-based assertions.
Existing contracts remain backward-compatible, and generated runs/assets/content/
artifacts remain ignored unless explicitly fixture-scoped. Shipping/hosted/cloud/
live-ops stays **DEFER until a #1508 Layer-3 GO** (Layer-3 distributed
orchestration / Elixir remains NO-GO under ADR #92). No production-readiness,
quality, fun, or Godot replacement/parity claim is introduced.

The recommended next milestone is not expanded by this completion; any later Era
G rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Content-at-Scale Generation and Curation v1 governance refresh

Content-at-Scale Generation and Curation v1 is recorded as **complete for Era G
Milestone 38** under #1, on merged evidence, as a conservative, additive,
local-only milestone. It scales generation from single levels to **campaign
scale** — many levels and a large card/relic pool across the grid-puzzle and
deck-roguelike genres — while making **curation mandatory** so scale does not
become slop: only solvable, balanced, sufficiently-novel content with a verified
whole-game difficulty curve is admitted. Every capability **reuses an existing
surface** (the Milestone 30 generative front door, the Milestone 28 solver and
difficulty metrics, the Milestone 32 balance telemetry, and the evaluator
`declared-gate-and` aggregation); it adds **no new generator, engine, runtime, or
writer**.

The merged evidence chain is the scope/contract gate #1648
(`docs/content-scale-v1.md`, PR #1706) — defines the campaign-scale generation,
dedup/novelty, whole-game curve, curation-gate, and asset/provenance contracts
and the reuse statement; Campaign-Scale Generation v1 #1649
(`crates/ouroforge-core/src/content_scale_generation.rs` plus the deck-roguelike
genre added to `generative_intake.rs`, PR #1779) — turns a campaign brief into a
*set* of proposal-only artifacts across both genres, reusing the existing
`MutationProposal` model with no per-game escape hatch; Deduplication and Novelty
Metrics v1 #1650 (`crates/ouroforge-core/src/content_novelty.rs`, PR #1781) —
descriptive dedup/novelty metrics computed over the existing generated artifacts
(a content digest, not a similarity engine), read/measure-only and never
destructive; Whole-Game Difficulty-Curve Verification v1 #1651
(`crates/ouroforge-core/src/content_difficulty_curve.rs`, PR #1784) — verifies an
ordered campaign's difficulty curve against declared tolerances, deriving each
stage's difficulty from the existing Milestone 28 metric and Milestone 32 balance
report and flagging spikes/regressions; Content Curation Gate v1 #1652
(`crates/ouroforge-evaluator/src/content_curation_gate.rs`, PR #1785) — the
campaign-level promotion guard, composing one declared `contentCuration` category
into the existing `declared-gate-and` aggregation and admitting a campaign only
when all four evidence dimensions are declared and solvable/balanced/novel/curve
pass; the Content-at-Scale Generation and Curation Demo v1 #1653
(`docs/content-scale-v1-demo.md`, PR #1786) — a deterministic, fixture-scoped
slice where generation feeds curation, low-novelty and curve-spike campaigns are
curated out, and a curve-verified set is admitted; and Scenario Coverage v36
#1654 (`docs/scenario-coverage-v36.md`, PR #1787) — an enumerated,
state/shape-only regression suite over generation/novelty/curve/curation plus a
backward-compatibility golden proving single-level Milestone 30 generation
remains valid. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. **Curation is mandatory** before any
campaign promotion, and **non-slop at scale is a process guarantee, not a quality
or fun claim**: "balanced", "novel", and "curated" are measurements against
declared, evidence-backed thresholds, not proof that a campaign is good, fun,
shippable, or production-ready; **content tone, taste, and art/audio/UX/narrative
direction stay human decisions** and the human release go/no-go is preserved.
Generation/role-agent/producer output stays **proposal-only** through the existing
review/apply/trust-gradient path — never a trusted write, auto-apply, auto-merge,
self-approval, or reviewer bypass — and generated assets/content require
license/provenance and the function-specific QA gate before promotion. Trusted
logic is Rust/local; browser/Studio surfaces remain read-only. Coverage is
state/shape-only with no flaky/timing-based assertions, existing contracts remain
backward-compatible, and generated runs/assets/content/artifacts remain ignored
unless explicitly fixture-scoped. Shipping/hosted/cloud/live-ops stays **DEFER
until a #1508 Layer-3 GO** (Layer-3 distributed orchestration / Elixir remains
NO-GO under ADR #92). No production-readiness, quality, fun, or Godot
replacement/parity claim is introduced.

The recommended next milestone is not expanded by this completion; any later Era
G rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Design Regression Harness v1 governance refresh

Design Regression Harness v1 (Era F Milestone 29 under #1) is recorded as
**complete on merged evidence**: design regression as **CI for game design**. On a
content or rule edit, the harness re-runs the Milestone 28 solver, over-solution
detector, and difficulty suite across the affected grid-puzzle levels, diffs the
recomputed status against the recorded baseline, and classifies each level
`unchanged`, `improved`, or `newly-broken` — every regression carrying a
**replayable trace** (the shortest over-solution counterexample, "watch the
bypass", or the previously-intended solution that no longer wins, "watch it
break"). It is an **orchestration** of existing surfaces, not a new comparison
engine.

The merged evidence chain is the scope/contract #1587
(`docs/design-regression-harness-v1.md`, PR #1628); the harness Model and Diff v1
#1588 (`crates/ouroforge-core/src/design_regression_harness.rs`, PR #1754) — the
`design-regression-harness-v1` artifact that re-runs the existing #1580 solver,
#1581 over-solution detector, and #1582 difficulty suite (no re-implementation),
diffs against the recorded baseline, classifies the outcome with a replayable
trace per regression, surfaces the verdict read-only in the dashboard, and fails
closed (stale baseline, exhausted budget, or malformed input → `inconclusive`,
never a false clean/improved/regression); the Demo v1 #1589
(`docs/design-regression-harness-v1-demo.md`, PR #1761) — a deterministic,
fixture-scoped demo where a shared-rule edit opens a new over-solution
**elsewhere** and is flagged with a replayable trace while a clean edit passes,
no network or live browser; and Scenario Coverage v29 #1590
(`docs/scenario-coverage-v29.md`, PR #1762) — an enumerated newly-broken /
improved / unchanged classification and trace-linkage regression matrix plus a
single-run solver/detector backward-compatibility golden proving the Milestone 28
surfaces remain valid outside the harness. All four are merged; Milestone 29 is
therefore recorded as complete. No milestone is marked complete ahead of merged
evidence.

The boundaries stay explicit and reaffirmed. The harness logic is owned by
**Rust/local** and **reuses** the existing solver, over-solution detector,
difficulty suite, and the `compare` / evolve-campaign verdict shape — no new
engine, runtime, writer, or parallel comparison engine. **Detection only**: there
is no auto-fix, auto-apply, auto-merge, self-approval, or reviewer bypass, and a
regression verdict blocks promotion rather than changing content; any trusted
write stays on the existing review/apply/trust-gradient path. The harness asserts
behavior and gate state and carries no difficulty, quality, fun,
production-readiness, or Godot replacement/parity claim. Generation stays
proposal-only; browser/Studio surfaces remain read-only; existing
runtime/probe/evaluator four-gate aggregation contracts remain
backward-compatible; generated runs/evidence remain ignored unless explicitly
fixture-scoped; Layer-3 distributed orchestration / Elixir remains NO-GO under ADR
#92 and deferred under #1508. #1 and #23 remain open governance anchors.

### Deck-Roguelike Game Class v1 governance refresh

Deck-Roguelike Game Class v1 is recorded as **complete for Era F Milestone 31**
under #1, on merged evidence, as a conservative, additive, local-only milestone.
It adds a deterministic, probe-exposed deck-roguelike game class (cards/relics/
runs, an energy budget per turn, and a scripted enemy) to the existing
game-runtime. It is a **demand-driven, capability-axis rung** on the Game
Complexity Ladder (`docs/game-complexity-ladder-v1.md`): it adds the **seeded
stochastic state** axis rather than new spatial breadth, and sits above the
structural rungs (collect-and-exit through multi-scene objective game). Every
capability **reuses an existing surface** — the runtime, the
`window.__OUROFORGE__` probe, the replay-state digest, and the seeded stochastic
determinism layer; it adds **no new engine, runtime, or writer**, and authorizes
**no renderer, physics, audio, or 3D breadth**.

The merged evidence chain is the scope/contract gate #1599
(`docs/deck-roguelike-game-class-v1.md`, PR #1624) — defines the deck-roguelike
game-class and seeded determinism contracts and the reuse statement; Seeded
Stochastic Determinism v1 #1600 (`crates/ouroforge-core/src/seeded_rng.rs` and
the runtime `mulberry32` stream, PR #1630) — makes all randomness derive from an
explicit seed, captured by snapshot/restore and folded into the replay-state
digest; Deck-Roguelike Game Class and Runtime v1 #1601
(`examples/game-runtime/deck-roguelike.js` with the runtime load/advance/digest/
probe wiring, PR #1752) — the deterministic, probe-exposed game class whose
shuffles reuse the seeded stream, so an identical seed and action sequence
reproduce a digest-stable run and malformed decks fail closed; the Deck-Roguelike
Game Class Demo v1 #1602 (`docs/deck-roguelike-game-class-v1-demo.md` and
`examples/deck-roguelike-game-class-v1/demo/`, PR #1757) — a deterministic,
fixture-scoped demo with passing four-gate and Milestone 20 loop-coverage
evidence and a recorded Milestone 24 ladder rung; and Scenario Coverage v31 #1603
(`docs/scenario-coverage-v31.md`, PR #1760) — an enumerated, state/shape-only
regression suite over seeded determinism (same/different seed), snapshot-across-
draw, and run reproducibility, plus a backward-compatibility golden proving the
prior non-stochastic classes keep byte-identical replay-state digests. #1 and
#23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. **All randomness is seeded and
replay-stable**: no unseeded randomness, wall-clock seeding, or ambient entropy;
a seeded run is reproducible from its declared inputs. The class is a **bounded
genre rung**, not a shipped game: no production-readiness, quality, fun,
shippable, or current Godot replacement/parity claim is introduced, and the
human release go/no-go is preserved. Generation/role-agent/producer output stays
**proposal-only** through the existing review/apply/trust-gradient path — never a
direct trusted write, auto-apply, auto-merge, self-approval, or reviewer bypass.
Trusted logic is Rust/local; browser/Studio surfaces remain read-only. Existing
contracts remain backward-compatible (the deck-roguelike digest key is additive),
and generated runs/artifacts remain ignored unless explicitly fixture-scoped.
Shipping/hosted/cloud/live-ops stays **DEFER until a #1508 Layer-3 GO** (Layer-3
distributed orchestration / Elixir remains NO-GO under ADR #92).

The recommended next milestone is not expanded by this completion; any later Era
F rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

Synthetic Player Balance v1 is recorded as **complete for Era F Milestone 32**
under #1, on merged evidence, as a conservative, additive, local-only milestone.
It adds **pre-launch synthetic balance telemetry** over the deck-roguelike game
class (Milestone 31): human-like persona agents play seeded runs, descriptive
telemetry aggregates them into a balance report, and a read-only cockpit flags
degenerate combos and dead items with replayable seeds and re-runs a proposed
nerf on the identical seed distribution to diff its win-rate impact. It is the
verification **engine room** for the generative front door — layers, not
alternatives. Every capability **reuses an existing surface** — the deck-roguelike
probe and seeded determinism, the evidence model, the `compare` contract, and the
read-only dashboard/cockpit surfacing; it adds **no new engine, runtime, writer,
or solver**.

The merged evidence chain is the scope/contract gate #1605
(`docs/synthetic-player-balance-v1.md`, PR #1625) — defines the persona,
telemetry, cockpit, and `compare`-based re-run contracts and the reuse statement;
Synthetic Player Persona Agents v1 #1606 (`examples/game-runtime/synthetic-player.js`
with the trusted Rust mirror `crates/ouroforge-core/tests/synthetic_player_agents_contract.rs`,
PR #1759, with the turn-budget boundary fix PR #1770) — human-like (skill/style)
personas that drive the existing probe deterministically over an integer-only
seeded decision stream, with a bounded run budget, not win-maximizers; Balance
Telemetry Aggregation v1 #1607 (`examples/game-runtime/balance-telemetry.js` and
`crates/ouroforge-core/tests/balance_telemetry_contract.rs`, PR #1777) — a
deterministic, descriptive balance report that flags a degenerate combo and a
dead item with replayable seeds; Balance Cockpit Read-Only Surface and Re-Run
Diff v1 #1608 (`examples/game-runtime/balance-cockpit.js` and
`crates/ouroforge-core/tests/balance_cockpit_rerun_contract.rs`, PR #1778) — a
read-only, HTML-escaped surfacing with per-flag counterexamples and a proposed
nerf re-run that diffs the win-rate, reusing the `compare` digest-equality signal
and never mutating the trusted spec; the Synthetic Player Balance Demo v1 #1609
(`docs/synthetic-player-balance-v1-demo.md` and
`examples/synthetic-player-balance-v1/demo/`, PR #1780) — a deterministic,
fixture-scoped demo (no network/live browser) that flags the `smite`+`hex`
degenerate combo with a replayable seed and re-runs a `smite` nerf to diff the
win-rate (5/5 → 3/5); and Scenario Coverage v32 #1610
(`crates/ouroforge-core/tests/scenario_coverage_v32_synthetic_player_balance.rs`,
PR #1765, repaired by PR #1769) — an enumerated, state/shape-only regression suite
over persona determinism, telemetry flags, and the cockpit re-run diff. #1 and
#23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. The **metrics are descriptive**, not
a balance or quality guarantee; the **cockpit is read-only and human-in-the-loop**
with **no auto-applied nerf or buff**. Personas are **human-like, not
win-maximizing or superhuman**; runs are **seeded and replay-stable** with **no
live or network telemetry**. The milestone is a **bounded analysis slice**, not a
shipped game: no production-readiness, quality, fun, shippable, or current Godot
replacement/parity claim is introduced, and the human release go/no-go is
preserved. Generation/role-agent/producer output stays **proposal-only** through
the existing review/apply/trust-gradient path — never a direct trusted write,
auto-apply, auto-merge, self-approval, or reviewer bypass. Trusted logic is
Rust/local; browser/Studio surfaces remain read-only. Existing contracts remain
backward-compatible (the persona run record's per-card tally and the report/diff
artifacts are additive), and generated runs/artifacts remain ignored unless
explicitly fixture-scoped. Shipping/hosted/cloud/live-ops stays **DEFER until a
#1508 Layer-3 GO** (Layer-3 distributed orchestration / Elixir remains NO-GO under
ADR #92).

The recommended next milestone is not expanded by this completion; any later Era
F rung requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same Layer-3 / hosted-paid boundaries.

### Era F (Milestones 27–34) governance refresh

Era F (**Accessible Authoring and Genre Verticalization**, Milestones 27–34
under #1) is recorded as **complete on merged evidence** as a conservative,
additive, local-first sequence. This is the Era F closing governance refresh
(#1 Milestone 35). It records what each milestone actually realized, assesses the
sequence against #1's extended north-star, and reaffirms every Era F boundary. It
marks **no** milestone complete ahead of merged evidence, adds no executable
behavior, and does not close or narrow #1 or #23. The guiding principle is
unchanged: **generation is the front door** (access and adoption) and the
**deterministic verification/balancing loop is the engine room** that makes
generated output non-slop — layers, not alternatives. Every milestone below is
recorded in its own per-milestone governance refresh above; this section
consolidates them and does not supersede them.

The merged per-milestone evidence chain is:

- **Milestone 27 — Grid-Puzzle Game Class v1** (#1573–#1577): the beachhead
  machine-checkable genre — a deterministic, probe-observable grid-puzzle class
  whose solvability and intended-solution replay are decided by the loop, with a
  validate-then-load PuzzleScript-compatible DSL ingest (PRs #1621/#1631/#1753/
  #1748/#1763; Scenario Coverage v27).
- **Milestone 28 — Puzzle Solver and Over-Solution Detection v1** (#1579–#1585):
  the moat capability — a bounded deterministic solver and an exhaustive
  over-solution detector that surface any unintended shortcut as a replayable
  counterexample, a descriptive difficulty-metric artifact, and the
  Design-Integrity Gate v1 #1583
  (`crates/ouroforge-evaluator/src/design_integrity_gate.rs`, PR #1751) ANDed into
  the evaluator `declared-gate-and` aggregation (PRs #1622/#1677/#1708/#1715/
  #1722/#1724/#1751; Scenario Coverage v28).
- **Milestone 29 — Design Regression Harness v1** (#1587–#1590): CI for game
  design — an orchestration that re-runs the M28 solver/detector/difficulty suite
  on each edit, diffs against the recorded baseline, classifies `unchanged` /
  `improved` / `newly-broken` with a replayable trace, and fails closed to
  `inconclusive` (PRs #1628/#1754/#1761/#1762; Scenario Coverage v29).
- **Milestone 30 — Generative Front Door v1** (#1592–#1597): the accessibility
  front door — a plain brief becomes a grid-puzzle *proposal* over the existing
  `MutationProposal` model, promotable only when it passes the engine room
  (solver + over-solution + design-integrity gate ANDed into the four-gate
  aggregation), routed unchanged into the existing review/apply/trust-gradient
  path and never a trusted write (PRs #1620/#1699/#1755/#1758/#1771/#1764;
  Scenario Coverage v30).
- **Milestone 31 — Deck-Roguelike Game Class v1** (#1599–#1603): the next genre
  rung — a seeded stochastic determinism layer (`seeded_rng.rs` and the runtime
  `mulberry32` stream) and a deterministic, probe-exposed deck-roguelike class
  whose shuffles are seed-reproducible and folded into the replay-state digest
  (PRs #1624/#1630/#1752/#1757/#1760; Scenario Coverage v31).
- **Milestone 32 — Synthetic Player Balance v1** (#1605–#1610): pre-launch
  balance telemetry — human-like persona agents (not win-maximizers) play seeded
  runs, descriptive telemetry flags degenerate combos and dead items with
  replayable seeds, and a read-only cockpit re-runs a proposed nerf on the
  identical seed distribution via `compare` to diff its win-rate (PRs #1625/#1759/
  #1770/#1777/#1778/#1780/#1765/#1769; Scenario Coverage v32).
- **Milestone 33 — Evidence-Native Marketplace v1** (#1612–#1616): evidence
  compounding into verifiable assets — a local verifiable-asset registry where
  each asset binds its acceptance suite, a deterministic replay proof, and an M25
  provenance lineage (the Asset Replay-Proof and Provenance Binding v1 #1614),
  re-verified locally on consume, with the paid/hosted transaction layer recorded
  as a Layer-3-gated policy line, not a code path (PRs #1623/#1629/#1756/#1632/
  #1633; Scenario Coverage v33).
- **Milestone 34 — OSS Trust Charter and Paid-Cloud Boundary Design Gate v1**
  (#1618): a design-gate ADR (`docs/oss-trust-charter.md`, PR #1626) adopting the
  MIT/Apache no-relicense / no-runtime-fee / no-install-fee / no-revenue-share
  charter and recording a per-surface paid-cloud GO/DEFER — DEFER by default on
  every surface, each tied to a #1508 Layer-3 hosted/cloud GO, and **never** a
  creative primitive. No cloud, hosted, or paid capability is implemented.

**North-star assessment (descriptive, not a maturity claim).** Against #1's
extended north-star `loop coverage × game complexity × trust × accessibility`:

- **Loop coverage × game complexity:** the loop now produces and machine-verifies
  two new genre verticals beyond the Era A–E structural ladder — grid puzzle
  (spatial, M27) and deck roguelike (seeded stochastic, M31) — each with a
  loop-produced, fixture-scoped demo carrying passing four-gate and Milestone 20
  loop-coverage evidence recorded as a Milestone 24 ladder rung (#1576, #1602).
  Coverage is measured per genre by those demos; it is **not** a claim of broad
  engine coverage or production parity.
- **Trust:** generation is verified rather than asserted — a generated proposal is
  promotable only after passing the engine room (M28 solver + over-solution + the
  design-integrity gate, ANDed into the existing four-gate `declared-gate-and`
  aggregation), and design regression (M29) re-proves the whole game on each edit.
  The trust posture is locked by the M34 charter (third-rails affirmed,
  paid-cloud DEFER per surface). Generation **never performs a trusted write**;
  every promotion flows through the existing review/apply/trust-gradient path.
- **Accessibility:** a non-developer can describe a grid-puzzle in a plain brief
  and receive a *verified-solvable proposal* (M30) without weakening the safety
  model, and pre-launch balance is surfaced as interpretable, human-in-the-loop
  evidence with replayable counterexamples (M32). "Verified" means only that a
  proposal passed the engine room — **not** that a generated game is good, fun,
  shippable, or production-ready.

**Remaining gaps (recorded, not backfilled here).** Engine breadth (renderer, 3D,
audio, physics depth) beyond each rung's gate stays demand-driven under the
Milestone 24 ladder and requires a separate scope issue citing the gate it
satisfies. The arcade rung and campaign-scale generation are later work (Era G+),
not Era F. Any hosted, cloud, paid, or marketplace-transaction capability stays
**DEFER until a #1508 Layer-3 GO**; distributed orchestration / Elixir remains
NO-GO under ADR #92 (`docs/distributed-elixir-design.md`).

**Boundaries reaffirmed across all Era F milestones.** Generation, role-agent,
and producer output is **proposal-only** through the existing review/apply/
trust-gradient path — never a direct trusted write, auto-apply, auto-merge,
self-approval, or reviewer bypass. Non-slop is a **process guarantee** (the engine
room), not a quality claim. Metrics are **descriptive**. Trusted logic is
**Rust/local**; the runtime, `window.__OUROFORGE__` probe, and dashboard/cockpit
surfaces are **read-only**; there is no browser command bridge, shell execution,
dependency install, CI/workflow mutation, credentialed operation, or publish/
deploy/sign/upload. Genre and engine growth stay **demand-driven**.
Cloud/hosted/marketplace monetization stays **Layer-3-gated** (DEFER per #1508).
Existing runtime/probe/evaluator four-gate aggregation, evolve/campaign,
`compare`, provenance-bundle, dashboard, cockpit, and CLI contracts remain
backward-compatible; generated runs/genre/evidence/registry artifacts remain
ignored unless explicitly fixture-scoped. No production-readiness, quality, fun,
shippability, or Godot replacement/parity claim is introduced. **#1 and #23
remain open governance anchors** and are not modified, closed, or narrowed by this
refresh; Scenario Coverage numbering continues from v26 (Era E) onward through the
Era F suite (v27–v33).

### Era G (Milestones 36–40) governance refresh

Era G (**Specialized Production Functions**, Milestones 36–40 under #1) is
recorded as **complete on merged evidence** as a conservative, additive,
local-first sequence. This is the Era G closing governance refresh (#1 Milestone
41). It records what each milestone actually realized, assesses the sequence
against #1's extended north-star, and reaffirms every Era G boundary. It marks
**no** milestone complete ahead of merged evidence, adds no executable behavior,
and does not close or narrow #1 or #23. The guiding principle is unchanged from
Era F and extended to each studio function: **each missing studio function
becomes a specialized, verified capability, never an unverified generator** —
generation (assets, audio, content, systems) stays proposal-only through the
existing review/apply/trust-gradient path, and every generated artifact must pass
a function-specific verification gate (asset-QA, audio-QA, content-curation,
systems contract, production-QA) before promotion. Every milestone below is
recorded in its own per-milestone governance refresh above; this section
consolidates them and does not supersede them.

The merged per-milestone evidence chain is:

- **Milestone 36 — Asset Generation and Asset-QA v1** (#1634–#1639): the loop's
  verified visual-asset function (sprites, tilesets, UI art) — a generated asset
  is a proposal carrying mandatory license/provenance, routed through the existing
  review/apply/trust-gradient path and promotable only after a function-specific
  asset-QA gate (style consistency, format/resolution validity, visual-regression
  vs baseline, license/provenance completeness) passes ANDed into the evaluator
  `declared-gate-and` aggregation, with validate-then-load import reusing the
  existing asset manifest/atlas surfaces (PRs #1703/#1709/#1714/#1720/#1727/#1730;
  Scenario Coverage v34).
- **Milestone 37 — Audio Generation and Audio-QA v1** (#1641–#1646): the same
  verified-asset pattern extended to audio (SFX, music, adaptive audio) — a
  generated audio asset is a proposal carrying mandatory license/provenance,
  promotable only after an audio-QA gate (format/loudness validity,
  license/provenance completeness, regression vs baseline) passes under
  `declared-gate-and`, with deterministic adaptive-audio runtime hooks reusing the
  existing runtime audio-intent surface (PRs #1705/#1713/#1719/#1729/#1734/#1737;
  Scenario Coverage v35).
- **Milestone 38 — Content-at-Scale Generation and Curation v1** (#1648–#1654):
  generation scaled from single levels to campaign scale across the grid-puzzle
  and deck-roguelike genres with **mandatory curation** — campaign-scale
  proposal-only generation, descriptive dedup/novelty metrics, whole-game
  difficulty-curve verification, and a content-curation gate ANDed into the
  four-gate aggregation that admits a campaign only when it is
  solvable/balanced/novel/curve-verified, reusing the M30 front door, the M28
  solver/difficulty metric, and the M32 balance telemetry (PRs #1706/#1779/#1781/
  #1784/#1785/#1786/#1787; Scenario Coverage v36).
- **Milestone 39 — Long-Form Game Systems v1** (#1656–#1663): the trusted
  Rust/local data systems a longer-form game needs — meta-progression/unlocks,
  economy/currency, save/profile and run-history at scale (with a v0→v1
  migration), UI/UX flow with onboarding and accessibility (deterministic,
  probe-observable, read-only), and an optional narrative/dialogue/event system —
  each a Milestone 24 ladder rung proven by a loop-produced, fixture-scoped demo
  carrying passing four-gate and loop-coverage evidence, on the existing runtime
  (PRs #1707/#1711/#1716/#1725/#1733/#1736/#1739/#1740; Scenario Coverage v37).
- **Milestone 40 — Production-Scale QA Matrix v1** (#1665–#1671): QA scaled from
  per-artifact checks to whole-game production QA — a regression matrix across
  content variants, seeds, and supported targets; visual-regression at scale;
  synthetic performance/soak testing; crash/accessibility checks; and a single
  consolidated production-QA verdict composed via `declared-gate-and` that fails
  closed when any declared check fails — every capability reusing an existing
  runner (the QA/playtest swarm, the visual gate, the frame-budget surface, the
  evaluator aggregation), with no new test engine, profiler, or evaluator (PRs
  #1700/#1710/#1712/#1718/#1723/#1742/#1743; Scenario Coverage v38).

**North-star assessment (descriptive, not a maturity claim).** Against #1's
extended north-star `loop coverage × game complexity × trust × accessibility ×
production coverage`:

- **Production coverage × loop coverage:** the loop now produces and
  machine-verifies five previously-uncovered specialized studio functions — visual
  assets (M36), audio (M37), content at campaign scale (M38), long-form game
  systems (M39), and whole-game production QA (M40) — each as a specialized
  capability with its own verification gate, extending the Era A–F mechanics/
  balance/verification coverage toward the functions a *larger, longer* game needs.
  Coverage is measured per function by that function's gate and its fixture-scoped
  demo; it is **not** a claim of full studio coverage, production parity, or broad
  engine breadth.
- **Game complexity:** the long-form systems (M39) are recorded as Milestone 24
  ladder rungs satisfied only on loop-produced, four-gate + loop-coverage
  evidence, and content-at-scale (M38) verifies whole-game difficulty curves at
  campaign scale; engine and content growth stay demand-driven and rung-justified,
  with no later rung, breadth, or parity claimed.
- **Trust:** every generated artifact is verified rather than asserted — assets,
  audio, and campaign content are promotable only after passing their
  function-specific gate (asset-QA, audio-QA, content-curation) ANDed into the
  existing four-gate `declared-gate-and` aggregation, and production-scale QA (M40)
  composes bounded evidence signals into one descriptive verdict that fails closed.
  Generation **never performs a trusted write**; every promotion flows through the
  existing review/apply/trust-gradient path, and license/provenance is mandatory
  before any asset/audio/content promotion.
- **Accessibility:** long-form systems surface deterministic, probe-observable
  UI/UX flow with declared accessibility options and onboarding (M39), and
  production-scale QA includes accessibility checks (M40) — surfaced as
  interpretable, structural evidence. "Verified" means only that an artifact
  passed its gate — **not** that a generated asset, sound, campaign, or game is
  good, on-brand, fun, shippable, or production-ready.

**Remaining gaps (recorded, not backfilled here).** Coverage is per function and
per demo, not whole-studio or production parity: only the five functions above are
evidenced, each at fixture scope, and art/audio/UX/narrative taste and the human
release go/no-go are deliberately retained outside every gate. Engine breadth
(renderer, 3D, physics depth) beyond each rung's gate stays demand-driven under
the Milestone 24 ladder and requires a separate scope issue citing the gate it
satisfies. Autonomous orchestration of these functions into a whole-game pipeline
is later work (Era H), not Era G. Any hosted, cloud, paid, real-player-telemetry,
or live-ops capability stays **DEFER until a #1508 Layer-3 GO**; distributed
orchestration / Elixir remains NO-GO under ADR #92
(`docs/distributed-elixir-design.md`).

**Boundaries reaffirmed across all Era G milestones.** Generation, role-agent,
and producer output is **proposal-only** through the existing review/apply/
trust-gradient path — never a direct trusted write, auto-apply, auto-merge,
self-approval, or reviewer bypass. **License/provenance and the function-specific
QA gate are mandatory before any asset/audio/content promotion; the gates fail
closed** — no unlicensed, uncredited, or unverified-style generated artifact is
ever promoted. Non-slop at scale is a **process guarantee** (each function's
verification gate), not a quality, fun, or production claim; metrics are
**descriptive**, and "looks good / sounds good / is fun" and art/audio/UX/
narrative direction remain human decisions with the human release go/no-go
preserved. Trusted logic is **Rust/local**; the runtime, `window.__OUROFORGE__`
probe, and dashboard/cockpit/Studio surfaces are **read-only**; there is no
browser command bridge, shell execution, dependency install, CI/workflow
mutation, credentialed operation, or publish/deploy/sign/upload. Genre, engine,
and function growth stay **demand-driven** (Milestone 24 ladder). Shipping,
hosted/cloud, real-player telemetry, and live-ops stay **Layer-3-gated** (DEFER
per Milestone 26 / #1508). Existing runtime/probe/evaluator four-gate
aggregation, evolve/campaign, `compare`, provenance-bundle, asset-manifest,
QA-swarm, dashboard, cockpit, and CLI contracts remain backward-compatible;
generated runs/assets/audio/content/profiles/artifacts remain ignored unless
explicitly fixture-scoped. No production-readiness, quality, fun, shippability, or
Godot replacement/parity claim is introduced. **#1 and #23 remain open governance
anchors** and are not modified, closed, or narrowed by this refresh; Scenario
Coverage numbering continues from v33 (Era F) onward through the Era G suite
(v34–v38).

**Era H complete — Milestones 42-46 (descriptive, not a maturity claim).** Era H
built the autonomous-orchestration accountability layer that Era G deliberately
deferred, and kept it as **evidence-gated, proposal-only collaboration**, not an
autonomous studio:

- **Era H Milestone 42 — Multi-Agent Production Pipeline v1** (#1674–#1681):
  realizes the Milestone 13 role model as role-specialized agents that propose
  artifacts, own a single artifact class each, hand work off with deterministic
  conflict resolution, and clear an independent reviewer/critic promotion gate
  over the Milestone 22 trust gradient before anything routes through the
  existing review/apply path — no role agent ever performs a direct trusted
  write, `promote-allowed` never auto-applies, and the human release go/no-go is
  preserved. Every contract reuses an existing surface (the M13 role set, the
  evidence/journal refs, the trust gradient and review/apply path); no new
  orchestration engine, runtime, writer, or scheduler is added (PRs
  #1704/#1790/#1876/#1877/#1878/#1879; Scenario Coverage v39). It is an
  accountability and review-governance framework only; autonomous producer
  orchestration, scaled trust/provenance, shipping, hosted/cloud, real-player
  telemetry, and live-ops stay **DEFER until a #1508 Layer-3 GO**, and
  distributed orchestration / Elixir remains NO-GO under ADR #92. **#1 and #23
  remain open governance anchors** and are not modified, closed, or narrowed by
  this milestone.

- **Era H Milestone 43 — Autonomous Producer and Whole-Game Orchestration v1**
  (#1682–#1688): extends the Era H accountability framework into bounded
  whole-game producer orchestration. The merged chain records the scope contract,
  deterministic design-intent decomposition, whole-game orchestration state,
  budget/stop-condition/human approval gates, deterministic autonomous producer
  demo, Scenario Coverage v40, and this governance refresh (PRs
  #1701/#1884/#1885/#1888/#1890/#1891/#1902; Scenario Coverage v40). The producer never
  directly writes trusted files or releases; it emits proposal/evidence artifacts
  through the existing review/apply/trust-gradient path, with mandatory budgets,
  stop conditions, and human approval gates. Browser/Studio surfaces remain
  read-only, generated state remains untracked unless fixture-scoped, shipping and
  live-ops remain Layer-3 gated, and no production-ready, quality/fun, Godot
  replacement/parity, or autonomous-shipping claim is introduced. **#1 and #23
  remain open governance anchors** and are not modified, closed, or narrowed by
  this milestone.


- **Era H Milestone 44 — Scaled Trust Gradient, Release Provenance and Compliance v1**
  (#1689–#1696): extends the Era H accountability framework into per-release
  trust, provenance, and compliance evidence. The merged chain records the scope
  contract, broadened-but-bounded low-risk rollback-backed auto-apply,
  per-release provenance bundle, compliance reviewer gate, fixture-scoped
  release-trust demo, Scenario Coverage v41, and this governance refresh (PRs
  #1702/#1906/#1910/#1956/#1964/#1973/#1987; Scenario Coverage v41). High-risk and
  source-affecting changes never auto-apply; release requires compliance plus a
  human go/no-go. Rust/local owns trusted provenance/compliance/review/apply
  logic; browser/Studio/dashboard/cockpit surfaces remain read-only and gain no
  trusted writes, auto-apply, auto-merge, self-approval, reviewer bypass, or
  command bridge. Generated release artifacts remain untracked unless
  fixture-scoped, shipping/live-ops stay Layer-3 gated, and no production-ready,
  quality/fun, Godot replacement/parity, or autonomous-shipping claim is
  introduced. **#1 and #23 remain open governance anchors** and are not modified,
  closed, or narrowed by this milestone.

- **Era H Milestone 45 — Shipping and LiveOps Layer-3 Re-evaluation Design Gate v1**
  (#1697): decides the shipping/liveops Layer-3 question after Era F-H evidence.
  The merged ADR records DEFER for native/store export, real-player telemetry,
  live balancing, and update/patch pipelines absent a separate #1508 Layer-3 GO
  (PR #1997). No implementation is added; Rust-first/local-first remains;
  autonomy ends at a local web release candidate with synthetic and
  fixture-scoped evidence; human release governance remains mandatory;
  distributed/Elixir remains NO-GO for Layer-3 under ADR #92. **#1 and #23 remain
  open governance anchors** and are not modified, closed, or narrowed by this
  milestone.

- **Era H Milestone 46 — Era H closing governance and final autonomy assessment**
  (#1698): records M42-M45 completion only on merged evidence, updates README and
  roadmap drift, and publishes the final autonomy assessment. The assessment is
  descriptive: local agents and Rust contracts can coordinate proposal, evidence,
  QA, provenance, compliance-blocking, and release-candidate preparation across
  the two tracked genre lines, but vision, taste/fun, legal/compliance
  acceptance, market/distribution choices, and release go/no-go remain human. No
  new capability, shipping/liveops implementation, production-ready claim,
  quality/fun claim, Godot replacement/parity claim, auto-merge, self-approval,
  reviewer bypass, or hidden trusted write is introduced. **#1 and #23 remain
  open governance anchors** and are not modified, closed, or narrowed by this
  milestone.

- **Era I Milestone 47 — Card-Roguelite Substrate v1** (#1791–#1797):
  generalizes the existing deck-roguelike class into a validated Rust/local
  card-roguelite substrate, preserves deck-roguelike golden parity, adds the
  engine-builder deckbuilder as a config over the same substrate, proves a
  fixture-scoped deterministic demo, and locks Scenario Coverage v42 (PRs
  #1892/#1897/#1898/#1903/#1909/#1915). The substrate is mechanical/balance
  evidence only: browser/Studio surfaces remain read-only, generation remains
  proposal-only, generated state remains untracked unless fixture-scoped, the
  fun/feel and release verdicts stay human, and no production-ready, quality/fun,
  shippability, Godot replacement/parity, or autonomous-shipping claim is
  introduced. **#1 and #23 remain open governance anchors** and are not modified,
  closed, or narrowed by this milestone.

- **Era I Milestone 48 — Multiplicative Scoring-Engine and Modifier Composition v1** (#1798–#1804):
  records a bounded scoring-engine chain over the existing card-roguelite
  substrate, including the scope contract, readable modifier/effect model,
  deterministic multiplicative resolution, composition/degen surfacing, a
  fixture-scoped scoring-engine demo, Scenario Coverage v43, and this governance
  refresh (PRs #1925/#1929/#1953/#1963/#1971/#1988). The milestone verifies
  mechanical scoring state only: modifiers remain individually readable,
  resolution is seed-stable and traceable, degenerate composition is surfaced as
  descriptive evidence, and the demo remains fixture-scoped. It is not a new
  parallel engine, not an automated fun/quality/release verdict, and not a
  production-ready or Godot replacement/parity claim. Trusted validation stays
  Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only;
  generation remains proposal-only through the existing review/apply/trust-gradient
  path; generated runs/artifacts stay untracked unless explicitly fixture-scoped.
  **#1 and #23 remain open governance anchors** and are not closed, narrowed, or
  modified by this milestone.

- **Era I Milestone 49 — Escalating Run Structure and Shop Economy v1** (#1805–#1810):
  records a bounded run/shop chain over the existing card-roguelite substrate,
  including the scope contract, escalating quota/ante run report, deterministic
  shop buy/sell/reroll/remove economy, fixture-scoped run-shop demo, Scenario
  Coverage v44, and this governance refresh (PRs #1998/#2050/#2103/#2106/#2108;
  Scenario Coverage v44). The milestone verifies mechanical run/shop state only:
  runs are bounded and seed-reproducible, quota curves are state/shape locked,
  shop actions expose levers over probability, and the demo remains
  fixture-scoped. It is not a new parallel engine, not an automated fun/quality/
  release verdict, and not a production-ready or Godot replacement/parity claim.
  Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio
  surfaces remain read-only; generation remains proposal-only through the
  existing review/apply/trust-gradient path; generated runs/artifacts stay
  untracked unless explicitly fixture-scoped. **#1 and #23 remain open
  governance anchors** and are not closed, narrowed, or modified by this
  milestone.

### Multiplicative Scoring-Engine and Modifier Composition v1 governance refresh (Era I Milestone 48)

Multiplicative Scoring-Engine and Modifier Composition v1 is recorded as
**complete for Era I Milestone 48** under #1, on merged evidence, as a
conservative, additive, Rust/local mechanical scoring milestone. It extends the
existing card-roguelite substrate with readable modifiers, deterministic
multiplicative resolution, composition/degen surfacing, a fixture-scoped demo,
and Scenario Coverage v43. It does not claim that a game is fun, balanced for
release, shippable, production-ready, or a Godot replacement.

The merged evidence chain is the scope/design gate #1798 (PR #1925) — records
the scoring-engine boundaries, substrate reuse, proposal-only generation path,
and human Era J fun/release gates; Modifier and Effect Model v1 #1799 (PR #1929)
— adds readable modifier/effect fixtures and validation without hidden score
authority; Deterministic Multiplicative Resolution Engine v1 #1800 (PR #1953) —
locks seed-stable ordering, overflow blocking, and traceable resolution state;
Combinatorial Composition Model v1 #1801 (PR #1963) — surfaces degenerate
composition descriptively without asserting fun or release quality;
Scoring-Engine Demo v1 #1802 (PR #1971) — proves a local fixture-scoped demo over
the scoring engine and substrate; and Scenario Coverage v43 #1803 (PR #1988) —
locks state/shape-only regression coverage for the modifier model, resolution
order, composition demo replay, substrate backcompat, generated-state wording,
and governance. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. A scoring-engine variant is
additive Rust/local state over the Card-Roguelite Substrate, not a new parallel
engine or browser/Studio writer. Trusted validation, persistence, substrate
scoring/balance, export/provenance logic, evidence writing, run/project binding,
review/apply/trust-gradient path, and CLI behavior remain Rust/local.
JavaScript/browser/Studio surfaces remain deterministic or read-only inspection
surfaces for trusted state and do not gain trusted writes, command bridges,
autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden mutation
authority. Generation remains proposal-only through the existing
review/apply/trust-gradient path. The fun/feel verdict, Steam account/signing/
release, content survey, market demand, hosted/cloud/mobile Layer-3 capability,
and release go/no-go stay human or deferred as previously scoped. Existing
runtime, probe, evaluator, evolve/campaign, compare, provenance-bundle, asset,
dashboard, cockpit, and CLI contracts remain backward-compatible; generated
runs/assets/builds remain ignored unless explicitly fixture-scoped. Scenario
Coverage numbering continues through v43.

The recommended next milestone is not expanded by this completion; any later Era
I/J work requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same human fun/release, read-only UI,
and Layer-3 boundaries.

### Escalating Run Structure and Shop Economy v1 governance refresh (Era I Milestone 49)

Escalating Run Structure and Shop Economy v1 is recorded as **complete for Era I
Milestone 49** under #1, on merged evidence, as a conservative, additive,
Rust/local mechanical run/shop milestone. It extends the existing
card-roguelite substrate with bounded escalating ante/quota run reports,
deterministic shop economy transactions, a fixture-scoped run-shop demo, and
Scenario Coverage v44. It does not claim that a game is fun, balanced for
release, shippable, production-ready, or a Godot replacement.

The merged evidence chain is the scope/design gate #1805 (PR #1998) — records
the run/shop boundaries, substrate reuse, proposal-only generation path, and
human Era J fun/release gates; Escalating Quota and Ante Run v1 #1806 (PR #2050)
— adds bounded run reports, non-decreasing quota validation, terminal win/loss
states, deterministic digests, and read-only inspection shape; Shop Economy v1
#1807 (PR #2103) — adds deterministic buy/sell/reroll/remove reports so shop
choices are probability levers rather than hidden trusted writes; Run and Shop
Demo v1 #1808 (PR #2106) — proves a local fixture-scoped demo over the run/shop
surface without network, live browser, or browser trusted writes; and Scenario
Coverage v44 #1809 (PR #2108) — locks state/shape-only regression coverage for
run escalation/win/loss, shop buy/sell/reroll/remove, substrate run/economy
backward compatibility, generated-state wording, and governance. #1 and #23
remain open governance anchors.

The boundaries stay explicit and reaffirmed. Run/shop behavior is mechanical
state evidence over the Card-Roguelite Substrate, not a new parallel engine or
browser/Studio writer. Trusted validation, persistence, substrate scoring,
balance/export/provenance logic, evidence writing, run/project binding,
review/apply/trust-gradient path, and CLI behavior remain Rust/local.
JavaScript/browser/Studio surfaces remain deterministic or read-only inspection
surfaces for trusted state and do not gain trusted writes, command bridges,
autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden mutation
authority. Generation remains proposal-only through the existing
review/apply/trust-gradient path. The fun/feel verdict, Steam account/signing/
release, content survey, market demand, hosted/cloud/mobile Layer-3 capability,
and release go/no-go stay human or deferred as previously scoped. Existing
runtime, probe, evaluator, evolve/campaign, compare, provenance-bundle, asset,
dashboard, cockpit, and CLI contracts remain backward-compatible; generated
runs/assets/builds remain ignored unless explicitly fixture-scoped. Scenario
Coverage numbering continues through v44.

The recommended next milestone is not expanded by this completion; any later Era
I/J work requires a separate scope issue with explicit non-goals, regression
coverage, generated-state audits, and the same human fun/release, read-only UI,
and Layer-3 boundaries.

**Era I Milestone 50 — Engine-Builder Balance Verification v1 (descriptive, not a fun claim).** Era I records a bounded mechanical balance verification chain for an engine-builder/card-roguelite variant over the existing substrate, not a new engine and not an automated fun verdict:

- **Era I Milestone 50 — Engine-Builder Balance Verification v1** (#1811–#1817): records the scope contract, combo/degenerate detector, dominant-build analyzer, fairness/daily-seed verifier, deterministic engine-builder balance demo, Scenario Coverage v45, and this governance refresh (PRs #1900/#1904/#1907/#1912/#1913). The milestone verifies descriptive mechanical surfaces: degenerate combo signatures, dominant-build concentration, and daily-seed fairness attribution. It does not assert that the game is good, fun, shippable, production-ready, or a Godot replacement/parity target. Balance verdicts are descriptive and evidence-linked; the human fun/release gate remains Era J. Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only; generation remains proposal-only through the existing review/apply/trust-gradient path. Generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

### Engine-Builder Balance Verification v1 governance refresh (Era I Milestone 50)

Engine-Builder Balance Verification v1 is recorded as **complete for Era I
Milestone 50** under #1, on merged evidence, as a conservative, additive,
local-only mechanical balance milestone. It treats a deckbuilder / engine-builder
variant as configuration over the existing card-roguelite substrate and verifies
only descriptive balance signals: degenerate combo risk, dominant-build
concentration, and skilled-player fairness attribution across deterministic daily
seeds. It is not a new parallel engine and not an automated quality, fun, or
release-readiness judgment.

The merged evidence chain is Combo / Degenerate Detector v1 #1812 (PR #1900) —
adds Rust/local detection for repeated low-cost/high-payoff loops and degenerate
combo signatures; Dominant-Build Analyzer v1 #1813 (PR #1904) — adds descriptive
build-concentration and payoff-skew analysis; Fairness and Daily-Seed Verifier v1
#1814 (PR #1907) — checks deterministic daily seeds for attribution-oriented
fairness evidence; Engine-Builder Balance Demo v1 #1815 (PR #1912) — documents and
proves a deterministic fixture-scoped demo over the composed balance surfaces;
and Scenario Coverage v45 #1816 (PR #1913) — locks state/shape-only regression
coverage across the detector, analyzer, verifier, demo fixture, and governance
boundaries. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Balance verdicts are **descriptive**
mechanical evidence, never a fun guarantee, popularity forecast, release gate, or
production-readiness claim. Fairness means a skilled player can attribute losses
to decisions rather than opaque luck; it does not guarantee that the experience is
enjoyable. Generation and producer output remain proposal-only through the
existing review/apply/trust-gradient path — never a direct trusted write,
auto-apply, auto-merge, self-approval, reviewer bypass, or hidden trusted
mutation. Trusted validation stays **Rust/local**; the deterministic JS runtime,
`window.__OUROFORGE__` probe, dashboard, cockpit, and Studio surfaces remain
**read-only** with respect to trusted state. Existing substrate, runtime, probe,
evaluator, evolve/campaign, compare, provenance-bundle, asset-manifest,
dashboard, cockpit, source-apply, and CLI contracts remain backward-compatible,
and generated runs/artifacts remain ignored unless explicitly fixture-scoped.
Shipping, hosted/cloud/mobile, live-ops, Steam account/signing/release, and market
demand remain human/Ring-3 or Layer-3-gated work outside this milestone; distributed
orchestration / Elixir remains NO-GO under ADR #92. No production-ready, quality,
fun, shippable, Godot replacement/parity, or autonomous-shipping claim is
introduced. Scenario Coverage numbering continues through v45.

The recommended next milestone is not expanded by this completion; later Era I/J
work requires separate scope issues with explicit non-goals, regression coverage,
generated-state audits, and the same human fun/release and Layer-3 boundaries.

**Era I Milestone 51 — Game-Feel and Juice Toolkit v1 (mechanical feedback; human feel gate).** Era I records a bounded game-feel and juice toolkit chain over the existing runtime and card-roguelite/deckbuilder substrate, not a new engine and not an automated fun verdict:

- **Era I Milestone 51 — Game-Feel and Juice Toolkit v1** (#1818–#1824): records the scope contract, runtime juice primitives, score-cascade payoff feedback, sub-100ms responsiveness verifier, deterministic Game-Feel and Juice demo, Scenario Coverage v46, and this governance refresh (PRs #1893/#1920/#1923/#1926/#1927/#1930). The milestone verifies mechanical feedback surfaces only: deterministic tween/shake/hit-stop/SFX intent shapes, score-cascade feedback ordering, and fixed-step input-to-feedback responsiveness pass/fail evidence. It does not assert that the interaction is good, fun, shippable, production-ready, or a Godot replacement/parity target. Feel and release verdicts remain human Era J/Ring-3 gates. Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only; generation remains proposal-only through the existing review/apply/trust-gradient path. Generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

### Game-Feel and Juice Toolkit v1 governance refresh (Era I Milestone 51)

Game-Feel and Juice Toolkit v1 is recorded as **complete for Era I Milestone 51** under #1, on merged evidence, as a conservative, additive, local-only mechanical feedback milestone. It treats juice as deterministic feedback evidence layered over existing runtime/substrate state. It is not a new runtime, renderer, engine, mixer, browser command bridge, automated quality score, automated fun score, or release-readiness judgment.

The merged evidence chain is Game-Feel and Juice Toolkit v1 Scope and Contract #1818 (PR #1893) — defines the Milestone 51 contracts, non-goals, reuse surfaces, and dependency order; Juice Primitives v1 #1819 (PR #1920) — adds deterministic runtime easing/tween, shake, hit-stop, and SFX feedback intents with read-only probe visibility; Score-Cascade Payoff Feedback v1 #1820 (PR #1923) — adds Rust/local ordered payoff feedback over authoritative substrate scoring without browser score authority; Sub-100ms Responsiveness Verification v1 #1821 (PR #1926) — adds deterministic fixed-step input-to-feedback responsiveness pass/fail evidence; Game-Feel and Juice Demo v1 #1822 (PR #1927) — documents and tests a fixture-scoped deterministic demo composing cascade feedback and responsiveness verdicts; and Scenario Coverage v46 #1823 (PR #1930) — locks state/shape-only regression coverage plus an existing runtime feedback backward-compatibility golden. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Juice is verified mechanically: primitive declarations, feedback ordering, and responsiveness budgets are evidence, not taste. The feel/fun verdict remains human-owned in Era J; responsiveness budget evidence is not a claim that a game is enjoyable, polished, production-ready, shippable, or marketable. Generation and producer output remain proposal-only through the existing review/apply/trust-gradient path — never a direct trusted write, auto-apply, auto-merge, self-approval, reviewer bypass, or hidden trusted mutation. Trusted validation stays **Rust/local**; the deterministic JS runtime, `window.__OUROFORGE__` probe, dashboard, cockpit, and Studio surfaces remain **read-only** with respect to trusted state. Existing substrate, runtime, probe, evaluator, evolve/campaign, compare, provenance-bundle, asset-manifest, dashboard, cockpit, source-apply, and CLI contracts remain backward-compatible, and generated runs/artifacts remain ignored unless explicitly fixture-scoped. Shipping, hosted/cloud/mobile, live-ops, Steam account/signing/release, and market demand remain human/Ring-3 or Layer-3-gated work outside this milestone; distributed orchestration / Elixir remains NO-GO under ADR #92. No production-ready, quality, fun, shippable, Godot replacement/parity, or autonomous-shipping claim is introduced. Scenario Coverage numbering continues through v46.

The recommended next milestone is not expanded by this completion; later Era I/J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same human feel/fun, release, and Layer-3 boundaries.

**Era I Milestone 52 — Deckbuilder UI Kit v1 (in-game JS UI; read-only/draft-only).** Era I records a bounded deckbuilder UI chain over the existing card-roguelite/deck-roguelike substrate and deterministic JS runtime, not a new engine, not a new UI framework, and not browser/Studio trusted-write authority:

- **Era I Milestone 52 — Deckbuilder UI Kit v1** (#1825–#1831): records the scope contract, card/hand/pipeline UI, shop/run-map UI, score-cascade display, deterministic Deckbuilder UI demo, Scenario Coverage v47, and this governance refresh (PRs #1895/#1894/#1961/#1966/#1969/#1977). The milestone verifies mechanical UI/probe state shapes only: draft-only hand and pipeline interactions, read-only shop and route planning state, score-cascade display presentation over Rust/local authority, deterministic demo expectations, and existing runtime UI/probe backward compatibility. It does not assert that the interface is good, fun, shippable, production-ready, or a Godot replacement/parity target. Feel/fun and release verdicts remain human Era J/Ring-3 gates. Trusted validation, scoring, persistence, and review/apply/trust-gradient writes stay Rust/local; in-game UI remains deterministic JavaScript; browser, dashboard, cockpit, and Studio surfaces remain read-only or draft-only. Generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

### Deckbuilder UI Kit v1 governance refresh (Era I Milestone 52)

Deckbuilder UI Kit v1 is recorded as **complete for Era I Milestone 52** under #1, on merged evidence, as a conservative, additive, local-only deckbuilder UI milestone. It treats deckbuilder UI as an in-game deterministic JavaScript presentation and draft/proposal surface over existing substrate state. It is not a new runtime, renderer, engine, UI framework, browser command bridge, automated quality score, automated fun score, or release-readiness judgment.

The merged evidence chain is Deckbuilder UI Kit v1 Scope and Contract #1825 (PR #1895) — defines the Milestone 52 contracts, non-goals, reuse surfaces, and dependency order; Card/Hand/Pipeline UI v1 #1826 (PR #1894) — adds deterministic hand/pipeline render state and draft-only probe interactions over the existing deck-roguelike runtime; Shop and Run-Map UI v1 #1827 (PR #1961) — adds read-only/draft-only shop offer and route-planning UI state that fails closed on stale offers and impossible paths; Number-Cascade and Score Display v1 #1828 (PR #1966) — adds score-cascade display presentation without browser score authority; Deckbuilder UI Demo v1 #1829 (PR #1969) — documents and tests a fixture-scoped deterministic demo composing the full UI/probe shape; and Scenario Coverage v47 #1830 (PR #1977) — locks state/shape-only regression coverage plus an existing runtime UI/probe backward-compatibility golden. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Deckbuilder UI is verified mechanically: probe shapes, draft-only interaction state, route/shop availability, and score display ordering are evidence, not taste. The feel/fun verdict remains human-owned in Era J; UI state evidence is not a claim that a game is enjoyable, polished, production-ready, shippable, or marketable. Generation and producer output remain proposal-only through the existing review/apply/trust-gradient path — never a direct trusted write, auto-apply, auto-merge, self-approval, reviewer bypass, or hidden trusted mutation. Trusted validation stays **Rust/local**; the deterministic JS runtime, in-game UI, `window.__OUROFORGE__` probe, dashboard, cockpit, and Studio surfaces remain **read-only** or **draft-only** with respect to trusted state. Existing substrate, runtime, probe, evaluator, evolve/campaign, compare, provenance-bundle, asset-manifest, dashboard, cockpit, source-apply, and CLI contracts remain backward-compatible, and generated runs/artifacts remain ignored unless explicitly fixture-scoped. Shipping, hosted/cloud/mobile, live-ops, Steam account/signing/release, and market demand remain human/Ring-3 or Layer-3-gated work outside this milestone; distributed orchestration / Elixir remains NO-GO under ADR #92. No production-ready, quality, fun, shippable, Godot replacement/parity, or autonomous-shipping claim is introduced. Scenario Coverage numbering continues through v47.

The recommended next milestone is not expanded by this completion; later Era I/J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same human feel/fun, release, read-only/draft-only UI, and Layer-3 boundaries.

**Era J Milestone 59 — Narrative and Theme-Arc Authoring Assist v1 (proposal-only; tone/soul human).** Era J records a bounded narrative/theme authoring assist chain over the existing Milestone 39 narrative system, Milestone 30 generative front door, and review/apply/trust-gradient path, not a new narrative engine, not browser/Studio trusted-write authority, and not an automated tone/soul/fun/quality verdict:

- **Era J Milestone 59 — Narrative and Theme-Arc Authoring Assist v1** (#1863–#1868): records the scope/design gate, narrative/theme candidate generation, human-curated narrative integration, deterministic narrative assist demo, Scenario Coverage v53, and this governance refresh (PRs #1908/#1965/#1975/#1989/#1990). Candidate generation emits bounded proposal-only narrative/theme-wit/flavor candidates with proposed/pending/unverified generative proposals. Human selection is recorded as provenance and replayed by candidate-set id, proposal id, and payload digest; selected material remains ready for review/apply, not trusted source. The deterministic demo and v53 suite are fixture-scoped state/shape checks, including Milestone 39 narrative-system backward compatibility. This milestone does not authorize direct trusted writes from generation or browser/Studio surfaces, auto-apply, auto-merge, self-approval, reviewer bypass, hidden trusted mutation, a parallel narrative engine, automated tone/soul/fun/quality/release scoring, production readiness, market-demand scoring, release buttons, or Godot replacement/parity claims. Tone, soul, and fun verdicts remain permanently human-owned. Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only; generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

**Era I Milestone 53 — Localization Pipeline v1 (mechanical localization; proposal-only generation).** Era I records a bounded localization chain over existing generation, deckbuilder UI strings, Rust/local validation, and fixture-scoped demo/coverage evidence, not a new engine, not a new UI framework, not a remote translation service, and not browser/Studio trusted-write authority:

- **Era I Milestone 53 — Localization Pipeline v1** (#1832–#1836): records the scope contract, string externalization and multi-language generation validation, deterministic Localization Demo v1, Scenario Coverage v48, and this governance refresh (PRs #1896/#1992/#1993/#1995). The milestone verifies mechanical localization surfaces only: stable string ids, source refs, proposal-only generated locale catalogs, completeness validation, placeholder integrity, rejected incomplete/mismatched locales, default-locale backward compatibility, and deterministic demo expectations. It does not automate creative tone, cultural suitability, translation quality judgment, fun, release readiness, market demand, production readiness, or Godot replacement/parity. Trusted validation, persistence, evidence writing, and review/apply/trust-gradient writes stay Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only or draft-only. Generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

### Localization Pipeline v1 governance refresh (Era I Milestone 53)

Localization Pipeline v1 is recorded as **complete for Era I Milestone 53** under #1, on merged evidence, as a conservative, additive, local-only mechanical localization milestone. It treats localization as validated string catalogs and proposal-only locale generation over existing runtime/UI and generation surfaces. It is not a new runtime, renderer, engine, UI framework, remote translation service, browser command bridge, automated quality score, automated fun score, cultural/tone authority, or release-readiness judgment.

The merged evidence chain is Localization Pipeline v1 Scope and Contract #1832 (PR #1896) — defines the Milestone 53 contracts, non-goals, reuse surfaces, and dependency order; String Externalization and Multi-Language Generation v1 #1833 (PR #1992) — adds Rust/local string catalogs, generated locale proposal validation, completeness checks, and placeholder integrity rejection fixtures; Localization Demo v1 #1834 (PR #1993) — documents and tests a fixture-scoped deterministic localized-title demo with an incomplete locale rejection; and Scenario Coverage v48 #1835 (PR #1995) — locks state/shape-only regression coverage plus a default-locale backward-compatibility golden. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Localization is verified mechanically: stable ids, complete locale entries, placeholder preservation, default-locale fallback shape, and validation diagnostics are evidence, not translation quality or cultural fit. The feel/fun verdict remains human-owned in Era J; localization evidence is not a claim that a game is enjoyable, polished, production-ready, shippable, marketable, or ready for Steam release. Generation and producer output remain proposal-only through the existing review/apply/trust-gradient path — never a direct trusted write, auto-apply, auto-merge, self-approval, reviewer bypass, hidden trusted mutation, or browser/Studio trusted catalog edit. Trusted validation stays **Rust/local**; the deterministic JS runtime, in-game UI, `window.__OUROFORGE__` probe, dashboard, cockpit, and Studio surfaces remain **read-only** or **draft-only** with respect to trusted state. Existing substrate, runtime, probe, evaluator, evolve/campaign, compare, provenance-bundle, asset-manifest, dashboard, cockpit, source-apply, and CLI contracts remain backward-compatible, and generated runs/artifacts remain ignored unless explicitly fixture-scoped. Shipping, hosted/cloud/mobile, live-ops, Steam account/signing/release, and market demand remain human/Ring-3 or Layer-3-gated work outside this milestone; distributed orchestration / Elixir remains NO-GO under ADR #92. No production-ready, quality, fun, shippable, release-authority, market-demand, Godot replacement/parity, or autonomous-shipping claim is introduced. Scenario Coverage numbering continues through v48.

The recommended next milestone is not expanded by this completion; later Era I/J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same human feel/fun, release, read-only/draft-only UI, proposal-only generation, and Layer-3 boundaries.

**Era I Milestone 54 — Steam Desktop Export and Steamworks v1 (local desktop export; human Ring-3 release).** Era I records a bounded Steam desktop export chain over the existing web runtime and deckbuilder substrate, not Layer-3 cloud/mobile, not a new engine, and not release automation:

- **Era I Milestone 54 — Steam Desktop Export and Steamworks v1** (#1837–#1843): records the design gate, Steam web-to-desktop wrapper/build pipeline, mockable Steamworks integration, Steam store asset proposal generation, deterministic Steam desktop export demo, Scenario Coverage v49, and this governance refresh (PRs #2000/#2001/#2007/#2012/#2016/#2051). The milestone verifies local desktop export state shapes only: Electron wrapping over the existing web runtime, SteamPipe/depot descriptor shape, mockable `steamworks.js` feature wiring, graceful no-Steam fallback, trusted-local-evidence leaderboard payloads, proposal-only store assets at Steam specs, deterministic fixture-scoped demo composition, and standalone web-build backward compatibility. It does not perform Steam account work, code signing, content survey, upload, store submission, release-button action, market-demand validation, production readiness, fun/quality judgment, or Godot replacement/parity claims. Trusted validation, persistence, evidence writing, package descriptor derivation, and review/apply/trust-gradient writes stay Rust/local; browser, dashboard, cockpit, Studio, Electron, JavaScript, and Steamworks surfaces remain read-only for trusted state. Generated desktop artifacts, depot builds, screenshots, trailer frames, and store assets remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

### Steam Desktop Export and Steamworks v1 governance refresh (Era I Milestone 54)

Steam Desktop Export and Steamworks v1 is recorded as **complete for Era I Milestone 54** under #1, on merged evidence, as a conservative, additive, local-only desktop export milestone. It treats Steam export as a deterministic package descriptor and fixture-scoped proposal/evidence chain over the existing web runtime. It is not a new runtime, renderer, engine, Steam SDK implementation, remote service, hosted/mobile/cloud capability, browser command bridge, automated quality score, automated fun score, market-demand signal, or release-readiness judgment.

The completion evidence is the merged chain #1837, #1838, #1839, #1840, #1841, #1842, and #1843. The bounded evidence includes `docs/steam-desktop-export-v1.md`, `docs/steam-desktop-export-v1-demo.md`, `docs/scenario-coverage-v49.md`, Steam export build/depot fixtures, Steamworks wiring/fallback/leaderboard fixtures, Steam store asset proposal fixtures, the deterministic demo fixture set, and the v49 regression suite. The verified state shapes remain local and deterministic; they require no network, live browser, real Steam connection, signing, upload, or release.

Boundaries reaffirmed: Steam desktop export is local desktop export only, not Layer-3 cloud/mobile. Steam account creation, partner portal work, code signing, content survey, store submission, release-button action, and market demand remain human/Ring-3 and out of engine scope. Browser, Studio, dashboard, cockpit, Electron, JavaScript, and Steamworks surfaces remain read-only for trusted state; Rust/local owns validation, provenance, descriptor derivation, persistence, and trusted-write gates. Generated runs/artifacts/builds remain untracked unless fixture-scoped. The human Era J fun/feel/release judgment remains outside this milestone.

The recommended next milestone is not expanded by this completion; later Era I/J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same human fun/feel, release, Steam Ring-3, read-only/draft-only UI, proposal-only generation, and Layer-3 boundaries.

**Era I Milestone 55 — Post-Launch Patch, Re-Verify, and Save-Migration Loop v1 (local patch loop; save compatibility).** Era I records a bounded post-launch patch chain over the existing Steam desktop export, source-apply/trust-gradient, and save/restore contracts, not release automation, not a new persistence system, and not browser/Studio trusted-write authority:

- **Era I Milestone 55 — Post-Launch Patch, Re-Verify, and Save-Migration Loop v1** (#1844–#1849): records the scope contract, patch re-verify/re-package gate, save migration/version-compatibility, deterministic post-launch patch demo, Scenario Coverage v50, and this governance refresh (PRs #2105/#2107/#2109/#2111/#2112). The milestone verifies local patch-loop state shapes only: a patch must pass the full declared gate set before re-package evidence is derived, old saves migrate forward through existing save/restore and replay-digest validation, incompatible saves fail closed with explicit evidence, the deterministic demo composes re-package plus save migration without network/live browser, and v50 locks patch/migration/backward-compatibility regressions. It does not perform Steam upload, signing, release-button action, market-demand validation, production readiness, fun/quality judgment, browser/Studio trusted writes, a new persistence mechanism, or Godot replacement/parity claims. Trusted validation, persistence, evidence writing, re-package descriptor derivation, save migration, and review/apply/trust-gradient writes stay Rust/local; browser, dashboard, cockpit, Studio, Electron, JavaScript, and Steamworks surfaces remain read-only for trusted state. Generated patch runs, package descriptors, migrated saves, builds, and evidence remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

### Post-Launch Patch, Re-Verify, and Save-Migration Loop v1 governance refresh (Era I Milestone 55)

Post-Launch Patch, Re-Verify, and Save-Migration Loop v1 is recorded as **complete for Era I Milestone 55** under #1, on merged evidence, as a conservative, additive, local-only patch and save-compatibility milestone. It treats patch handling as a deterministic Rust/local re-verify gate before local re-package evidence, and save compatibility as forward-only migration over the existing save/restore and replay-digest model. It is not a new runtime, renderer, engine, persistence mechanism, remote service, hosted/mobile/cloud capability, browser command bridge, automated quality score, automated fun score, market-demand signal, release-readiness judgment, or release automation.

The completion evidence is the merged chain #1844, #1845, #1846, #1847, and #1848. The bounded evidence includes `docs/post-launch-patch-v1.md`, `docs/post-launch-patch-v1-demo.md`, `docs/scenario-coverage-v50.md`, patch re-verify pass/fail fixtures, save migration forward/incompatible fixtures, the deterministic post-launch patch demo fixture, and the v50 regression suite with a non-patched build/save backward-compatibility golden. The verified state shapes remain local and deterministic; they require no network, live browser, real Steam connection, signing, upload, release, or subjective fun/quality assertion.

Boundaries reaffirmed: a patch re-verifies through the full gate set before re-package evidence is derived, and saves migrate forward with verified compatibility. Browser, Studio, dashboard, cockpit, Electron, JavaScript, and Steamworks surfaces remain read-only for trusted state; Rust/local owns validation, provenance, descriptor derivation, persistence, save migration, and trusted-write gates. Generated runs/artifacts/builds/migrated saves remain untracked unless fixture-scoped. The human Era J fun/feel/release judgment remains outside this milestone. #1 and #23 remain open governance anchors.

The recommended next milestone is not expanded by this completion; later Era I/J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same human fun/feel, release, Steam Ring-3, read-only/draft-only UI, proposal-only generation, and Layer-3 boundaries.

### Era I closing governance refresh (Milestones 47-55 / Milestone 56)

Era I is recorded as **complete for Milestones 47-55** under #1, on merged evidence, as a bounded genre-verticalization and local desktop-artifact milestone. Across the completed chain, Ouroforge produced an engine-builder deckbuilder variant as configuration over the card-roguelite substrate, not a parallel engine. The realized capability is descriptive and evidence-native: deterministic substrate state, readable scoring modifiers, bounded run/shop progression, mechanical balance verification, deterministic juice/feedback evidence, in-game deckbuilder UI/probe state, proposal-only localization, local Steam desktop export/package descriptor evidence, and the post-launch patch/save-migration loop.

Merged evidence by milestone:

- **M47 Card-Roguelite Substrate v1** (#1791-#1797): substrate-as-config, deck-roguelike golden parity, engine-builder config, deterministic demo, and Scenario Coverage v42.
- **M48 Scoring Engine v1** (#1798-#1804): readable modifiers, deterministic multiplicative resolution, degenerate-composition surfacing, demo, and Scenario Coverage v43.
- **M49 Run/Shop v1** (#1805-#1810): bounded ante/quota runs, deterministic shop economy levers, run-shop demo, Scenario Coverage v44, and governance refresh (#2110).
- **M50 Balance Verification v1** (#1811-#1817): combo/dominance/fairness evidence, deterministic balance demo, and Scenario Coverage v45.
- **M51 Game-Feel/Juice Toolkit v1** (#1818-#1824): deterministic feedback intents, score-cascade feedback, responsiveness evidence, demo, and Scenario Coverage v46.
- **M52 Deckbuilder UI Kit v1** (#1825-#1831): card/hand/pipeline UI, shop/run-map UI, score display, demo, and Scenario Coverage v47.
- **M53 Localization Pipeline v1** (#1832-#1836): stable string ids, proposal-only generated locale validation, localization demo, and Scenario Coverage v48.
- **M54 Steam Desktop Export and Steamworks v1** (#1837-#1843): local Electron wrapper/build descriptor, mockable Steamworks wiring/fallback/leaderboard, proposal-only store assets, deterministic demo, Scenario Coverage v49, and governance refresh.
- **M55 Post-Launch Patch, Re-Verify, and Save-Migration Loop v1** (#1844-#1849): patch re-verify/re-package gate, save migration/version compatibility, deterministic patch demo, Scenario Coverage v50, and governance refresh.

Shippability assessment (descriptive, not release authority): Era I reached a **verified Steam-shippable desktop artifact shape** for the loop-produced deckbuilder line. The evidence demonstrates that the deckbuilder variant can be represented as substrate configuration, validated mechanically, surfaced in deterministic UI/probe state, localized through proposal-only catalogs, wrapped into a local desktop package descriptor, wired to mockable Steamworks/local evidence surfaces, and guarded by patch re-verify plus save migration coverage. This is not a public release, store submission, code-signing result, content-survey completion, market-demand signal, production-ready claim, fun/quality verdict, or Godot replacement/parity claim. Human/Ring-3 Steam steps and the human Era J fun/feel/release verdict remain outside Era I.

Boundaries reaffirmed: the deckbuilder remains substrate-as-config; deterministic/seed-stable Rust/local validation owns trusted state, persistence, scoring/balance/export/provenance, save migration, evidence writing, run/project binding, review/apply/trust-gradient, and CLI behavior. JavaScript/browser/Studio/dashboard/cockpit/Electron/Steamworks surfaces remain deterministic read-only or proposal/draft-only surfaces for trusted state. Steam desktop export remains local and not Layer-3; hosted/cloud/mobile, real-player telemetry, live balancing, market demand, Steam account/signing/release, and public release authority remain human/Ring-3 or DEFER. Generated runs/artifacts/builds/migrated saves remain untracked unless fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by Era I completion.

The recommended next work is Era J human creative/release judgment and later scoped milestones only; no new capability is introduced by this governance refresh.

**Era J Milestone 57 — Candidate Generation and Curation Cockpit v1 (creation as curation; proposal-only).** Era J records a bounded creation-as-curation chain for deckbuilder candidate proposals over the existing card-roguelite/deck-roguelike substrate, not a new engine, not browser/Studio trusted-write authority, and not an automated fun or release verdict:

- **Era J Milestone 59 — Narrative and Theme-Arc Authoring Assist v1 (proposal-only; tone/soul human).** Era J records a bounded narrative/theme authoring assist chain over the existing Milestone 39 narrative system, Milestone 30 generative front door, and review/apply/trust-gradient path, not a new narrative engine, not browser/Studio trusted-write authority, and not an automated tone/soul/fun/quality verdict:

- **Era J Milestone 59 — Narrative and Theme-Arc Authoring Assist v1** (#1863–#1868): records the scope/design gate, narrative/theme candidate generation, human-curated narrative integration, deterministic narrative assist demo, Scenario Coverage v53, and this governance refresh (PRs #1908/#1965/#1975/#1989/#1990). Candidate generation emits bounded proposal-only narrative/theme-wit/flavor candidates with proposed/pending/unverified generative proposals. Human selection is recorded as provenance and replayed by candidate-set id, proposal id, and payload digest; selected material remains ready for review/apply, not trusted source. The deterministic demo and v53 suite are fixture-scoped state/shape checks, including Milestone 39 narrative-system backward compatibility. This milestone does not authorize direct trusted writes from generation or browser/Studio surfaces, auto-apply, auto-merge, self-approval, reviewer bypass, hidden trusted mutation, a parallel narrative engine, automated tone/soul/fun/quality/release scoring, production readiness, market-demand scoring, release buttons, or Godot replacement/parity claims. Tone, soul, and fun verdicts remain permanently human-owned. Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only; generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

**Era J Milestone 57 — Candidate Generation and Curation Cockpit v1** (#1851–#1856): records the scope/design gate, N-variant candidate generation, read-only curation surface, deterministic curation cockpit demo, Scenario Coverage v51, and this governance refresh (PRs #1901/#1914/#1917/#1918/#1921). Candidate generation reuses the Milestone 30 generative front door and emits proposal-only candidates with proposed/pending/unverified status. Human selection is recorded as provenance and replayed by candidate-set id, proposal id, and payload digest; the cockpit/dashboard read model exposes only inspection and provenance-recording actions. The deterministic demo and v51 suite are fixture-scoped state/shape checks, including Milestone 30 single-proposal backward compatibility. This milestone does not authorize direct trusted writes from generation or browser/Studio surfaces, auto-apply, auto-merge, self-approval, reviewer bypass, hidden trusted mutation, a parallel engine, automated fun/quality/release scoring, production readiness, or Godot replacement/parity claims. Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only; generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

**Era J Milestone 58 — Human Playtest Harness and Fun-Feel Gate v1 (human verdict, not automated fun).** Era J records a bounded human-owned playtest and fun/feel gate chain for deckbuilder candidates over the existing card-roguelite substrate, not a new engine, not browser/Studio trusted-write authority, and not an automated fun or release verdict:

- **Era J Milestone 58 — Human Playtest Harness and Fun-Feel Gate v1** (#1857–#1862): records the scope/design gate, structured human-playtest capture, human fun-feel release-readiness precondition, deterministic playtest/fun-feel demo, Scenario Coverage v52, and this governance refresh (PRs #1905/#1924/#1928/#1931/#1952). Structured capture stores session identity, first-session signals, feedback, and evidence refs as evidence only. The fun-feel gate blocks release-readiness without a fresh recorded human verdict and reports `approved-by-human` only when a human reviewer approves the scoped candidate and evidence. The deterministic demo and v52 suite are fixture-scoped state/shape checks, including no-auto-score drift coverage and existing evaluator aggregation backward compatibility. This milestone does not authorize direct trusted writes from generation or browser/Studio surfaces, auto-apply, auto-merge, self-approval, reviewer bypass, hidden trusted mutation, a parallel engine, automated fun/quality/release scoring, production readiness, market-demand scoring, release buttons, or Godot replacement/parity claims. The fun verdict remains permanently human-in-the-loop: a title cannot be release-ready without a human fun/feel verdict. Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only; generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

**Era J Milestone 60 — Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 (human-approved balance; required human go/no-go).** Era J records a bounded local balance recommendation and release-readiness evidence chain over the existing card-roguelite/deckbuilder substrate, not a release authority surface, not browser/Studio trusted-write authority, and not an automated fun, quality, market-demand, or production-readiness verdict:

- **Era J Milestone 60 — Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1** (#1869–#1874): records the scope/design gate, human-approved balance tuning recommendations, release-readiness bundle and human go/no-go surface, deterministic release-readiness demo, Scenario Coverage v54, and this governance refresh (PRs #1911/#2115/#2116/#2117/#2118). Balance recommendations are proposal-only, require human approval, and must be re-verified before any separate review/apply path can consider them. The release-readiness bundle can report mechanical readiness or blocked gates, but a separate recorded human go/no-go is required and grants no release authority, auto-merge authority, self-approval, reviewer bypass, or trusted write. The deterministic demo and v54 suite are fixture-scoped state/shape checks, including Milestone 25 and Milestone 44 provenance backward-compatibility goldens. This milestone does not authorize direct trusted writes from generation or browser/Studio surfaces, auto-apply, auto-merge, release buttons, Steam account/signing/release actions, market-demand scoring, automated fun/quality/release scoring, production readiness, or Godot replacement/parity claims. Trusted validation stays Rust/local; browser, dashboard, cockpit, and Studio surfaces remain read-only; generated runs/artifacts remain untracked unless explicitly fixture-scoped. **#1 and #23 remain open governance anchors** and are not closed, narrowed, or modified by this milestone.

### Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 governance refresh (Era J Milestone 60)

Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 is recorded as **complete for Era J Milestone 60** under #1, on merged evidence, as a conservative, additive, local-only balance and release-readiness milestone. It makes balance recommendations human-approved proposals and release readiness a human go/no-go record, not an automated release decision.

The merged evidence chain is Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 scope/design gate #1869 (PR #1911) — defines the Milestone 60 boundaries, non-goals, reuse surfaces, and dependency order; Balance Tuning Co-Pilot v1 #1870 (PR #2115) — surfaces deterministic balance recommendations as proposal-only evidence requiring human approval and re-verification; Release-Readiness Bundle and Go/No-Go Surface v1 #1871 (PR #2116) — composes mechanical readiness gates and records a human go/no-go without release authority; Release Readiness Demo v1 #1872 (PR #2117) — documents and tests a deterministic fixture-scoped ready/blocked/go-no-go demo; and Scenario Coverage v54 #1873 (PR #2118) — locks state/shape-only regression coverage plus Milestone 25 and Milestone 44 provenance backward compatibility. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Balance tuning remains **proposal-only and human-approved**: Ouroforge can recommend bounded mechanical adjustments and verify post-change evidence, but it cannot auto-apply, self-approve, bypass review, or write trusted source/project state. Release readiness remains **human go/no-go required**: Ouroforge can compose local evidence and report missing gates, but it cannot grant release authority, click a release button, sign/upload/submit to Steam, infer market demand, or decide that a title is fun, good, shippable, production-ready, or commercially ready. The deckbuilder variant remains configuration over the card-roguelite/deck-roguelike substrate, not a parallel engine. Trusted validation stays **Rust/local**; browser/dashboard/cockpit/Studio surfaces remain **read-only** with respect to trusted state. Generated runs/assets/builds/artifacts remain ignored unless explicitly fixture-scoped. Hosted/cloud/mobile capabilities, live-ops, Steam account/signing/release actions, content survey, market demand, and Layer-3 concerns remain outside this milestone. No production-ready, quality, fun, shippable, automated-release, Godot replacement/parity, or autonomous-shipping claim is introduced. Scenario Coverage numbering continues through v54.

The recommended next milestone is not expanded by this completion; later Era J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same proposal-only, read-only, human fun/release, human go/no-go, and Layer-3 boundaries.

### Narrative and Theme-Arc Authoring Assist v1 governance refresh (Era J Milestone 59)

Narrative and Theme-Arc Authoring Assist v1 is recorded as **complete for Era J Milestone 59** under #1, on merged evidence, as a conservative, additive, local-only narrative assist milestone. It treats narrative/theme assistance as human curation over proposal-only candidate sets: generation can assemble candidate material, but it cannot decide tone/soul, apply text, promote source changes, merge, self-approve, or write trusted source/project state.

The merged evidence chain is Narrative and Theme-Arc Authoring Assist v1 scope/design gate #1863 (PR #1908) — defines the Milestone 59 boundaries, non-goals, reuse surfaces, and dependency order; Narrative/Theme-Arc Candidate Generation v1 #1864 (PR #1965) — adds deterministic Rust/local candidate generation that wraps existing Milestone 30 generative proposals and extends Milestone 39 narrative surfaces; Human-Curated Narrative Integration v1 #1865 (PR #1975) — records and replays human-selected narrative candidate provenance without trusted write authority; Narrative Assist Demo v1 #1866 (PR #1989) — documents and tests a deterministic fixture-scoped candidate generation plus selected-integration demo; and Scenario Coverage v53 #1867 (PR #1990) — locks state/shape-only regression coverage plus the Milestone 39 narrative-system backward-compatibility golden. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Narrative generation remains **proposal-only** through the existing review/apply/trust-gradient path; human selection is provenance, not approval or apply authority. Tone/soul/fun/quality verdicts remain human-owned: Ouroforge may suggest candidate material and validate schema/provenance/stale refs, but it cannot claim that a candidate is funny, emotionally right, narratively good, fun, production-ready, shippable, or marketable. The deckbuilder variant remains configuration over the card-roguelite/deck-roguelike substrate, not a parallel engine. Trusted validation stays **Rust/local**; browser/dashboard/cockpit/Studio surfaces remain **read-only** with respect to trusted state. Generated runs/assets/builds/artifacts remain ignored unless explicitly fixture-scoped. Steam shipping remains a local desktop export concern; Steam account/signing/release, market demand, hosted/cloud/mobile capabilities, live-ops, and Layer-3 concerns remain outside this milestone. No production-ready, quality, fun, shippable, automated-release, Godot replacement/parity, or autonomous-shipping claim is introduced. Scenario Coverage numbering continues through v53.

The recommended next milestone is not expanded by this completion; later Era J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same proposal-only, read-only, human fun/release, human tone/soul, and Layer-3 boundaries.

### Human Playtest Harness and Fun-Feel Gate v1 governance refresh (Era J Milestone 58)

Human Playtest Harness and Fun-Feel Gate v1 is recorded as **complete for Era J Milestone 58** under #1, on merged evidence, as a conservative, additive, local-only human playtest milestone. It makes the fun/feel verdict permanently human-owned: Ouroforge can capture playtest evidence and enforce that a human verdict exists, but it cannot compute, infer, or replace the human fun verdict with an automated metric.

The merged evidence chain is Human Playtest Harness and Fun-Feel Gate v1 scope/design gate #1857 (PR #1905) — defines the Milestone 58 boundaries; Structured Human-Playtest Capture v1 #1858 (PR #1924) — records bounded human session signals and feedback as evidence only; Fun-Feel Evaluation Gate v1 #1859 (PR #1928) — blocks release-readiness without a fresh human verdict and records human sign-off without scoring fun; Playtest and Fun-Feel Gate Demo v1 #1860 (PR #1931) — documents and tests a deterministic fixture-scoped capture/block/unblock demo; and Scenario Coverage v52 #1861 (PR #1952) — locks state/shape-only regression coverage, no-auto-score drift protection, and evaluator aggregation backward compatibility. #1 and #23 remain open governance anchors.

The completion is descriptive and bounded. It does not say the title is fun, good, shippable, production-ready, market-validated, or a Godot replacement/parity target. It does not add a release button, public shipping authority, Steam account/signing/release flow, hosted/cloud/mobile Layer-3 capability, browser/Studio trusted writes, auto-merge, auto-apply, self-approval, or reviewer bypass. Steam shipping remains a local desktop export concern; Steam account/signing/release and market demand remain human/Ring-3 and out of engine scope.

The recommended next milestone is not expanded by this completion; later Era J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same proposal-only, read-only, human fun/release, and Layer-3 boundaries.

### Candidate Generation and Curation Cockpit v1 governance refresh (Era J Milestone 57)

Candidate Generation and Curation Cockpit v1 is recorded as **complete for Era J Milestone 57** under #1, on merged evidence, as a conservative, additive, local-only curation milestone. It treats creation as human curation over proposal-only candidate sets: generation can assemble candidate proposals, but it cannot apply, promote, merge, self-approve, or write trusted source/project state; auto-apply, auto-merge, self-approval, reviewer bypass, and hidden trusted mutation remain out of scope.

The merged evidence chain is Candidate Generation and Curation Cockpit v1 scope/design gate #1851 (PR #1901) — defines the Milestone 57 boundaries; N-Variant Candidate Generation v1 #1852 (PR #1914) — adds deterministic Rust/local candidate generation that wraps existing Milestone 30 generative proposals; Read-Only Curation Surface v1 #1853 (PR #1917) — records and replays human selection provenance without trusted write authority; Curation Cockpit Demo v1 #1854 (PR #1918) — documents and tests a deterministic fixture-scoped generation plus selection demo; and Scenario Coverage v51 #1855 (PR #1921) — locks state/shape-only regression coverage plus the Milestone 30 single-proposal golden. #1 and #23 remain open governance anchors.

The boundaries stay explicit and reaffirmed. Candidate generation remains **proposal-only** through the existing review/apply/trust-gradient path; human selection is provenance, not approval or apply authority. The deckbuilder variant remains configuration over the card-roguelite/deck-roguelike substrate, not a parallel engine. The human fun/feel verdict, release decision, Steam account/signing/release actions, market demand, hosted/cloud/mobile capabilities, live-ops, and Layer-3 concerns remain outside this milestone. Trusted validation stays **Rust/local**; browser/dashboard/cockpit/Studio surfaces remain **read-only** with respect to trusted state. Generated runs/assets/builds/artifacts remain ignored unless explicitly fixture-scoped. No production-ready, quality, fun, shippable, automated-release, Godot replacement/parity, or autonomous-shipping claim is introduced. Scenario Coverage numbering continues through v51.

The recommended next milestone is not expanded by this completion; later Era J work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same proposal-only, read-only, human fun/release, and Layer-3 boundaries.

### Era J closing governance and creative-leverage assessment (Milestone 61)

Era J Milestones 57-60 are recorded as **complete on merged evidence** under #1, with #1 and #23 preserved as open governance anchors. The closing assessment is documented in [`docs/era-j-creative-leverage-assessment.md`](era-j-creative-leverage-assessment.md). The evidence chain is Candidate Generation and Curation Cockpit v1 (#1851-#1856; PRs #1901/#1914/#1917/#1918/#1921/#1922), Human Playtest Harness and Fun-Feel Gate v1 (#1857-#1862; PRs #1905/#1924/#1928/#1931/#1952/#1954), Narrative and Theme-Arc Authoring Assist v1 (#1863-#1868; PRs #1908/#1965/#1975/#1989/#1990/#1991), and Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 (#1869-#1874; PRs #1911/#2115/#2116/#2117/#2118/#2119).

Creative-leverage assessment (descriptive, not a maturity claim): Era J increases output per human decision in the proposal/evidence layer. A human can inspect N candidate variants with provenance, capture structured playtest/fun-feel evidence, compare narrative/theme proposals, review balance recommendations with re-verification, and inspect release-readiness bundles plus go/no-go records. That leverage is bounded: the human decision remains the authority, and the system only preserves reproducible local evidence around it.

Permanent human core reaffirmed: fun/feel verdict, taste, tone/soul, creative direction, candidate curation, balance approval, release go/no-go, Steam account/signing/content-survey/release actions, wishlists, UA, discoverability, and market demand remain human/Ring-3 decisions. Rust/local owns trusted validation, evidence composition, provenance, persistence, review/apply/trust-gradient checks, and CLI behavior; browser/dashboard/cockpit/Studio surfaces remain read-only or draft/proposal-only for trusted state. Generated runs/assets/builds/artifacts remain untracked unless fixture-scoped. Hosted/cloud/mobile Layer-3 capability remains DEFER per #1508, and distributed/Elixir remains NO-GO under ADR #92. No automated creativity, automated fun/quality/release verdict, autonomous apply, auto-merge, self-approval, reviewer bypass, production-ready claim, market-demand claim, Godot replacement/parity claim, or autonomous-shipping authority is introduced.

The recommended next work is not expanded by this completion; later work requires separate scope issues with explicit non-goals, regression coverage, generated-state audits, and the same proposal-only, read-only, permanent-human-core, Ring-3 market, and Layer-3 boundaries.

## Product direction

- Keep the evidence-native loop inspectable, file-based, and local-first.
- Use Rust-owned validation for trusted persistence, project resolution, run
  binding, comparison artifacts, and scene-only mutation application.
- Keep browser surfaces static/read-only for trusted state: display exported
  data, preview runtime state, and show copyable CLI commands only; Studio
  source patch review surfaces remain inert evidence displays and never apply,
  merge, write files, or execute commands.
- Expand runtime/scenario coverage only when backed by concrete issues and
  tests; keep each expressive surface tied to its bounded evidence contract and
  do not infer broader production-engine/editor capabilities from completed
  animation/audio or transition slices.
- Keep authoring actions connected to QA evidence, semantic comparison,
  journals, rollback metadata, explicit mutation review, and regression
  promotion/matrix context.
- Treat evidence fidelity and review governance as first-class contracts: every
  run-facing surface should distinguish Rust-trusted artifacts from browser/CDP
  observations, and should expose missing or malformed evidence/review state as
  warnings instead of inferred passes.
- Treat Safe Source Mutation Apply v1 as complete but narrow. Its artifacts
  authorize only review-gated trusted apply for explicitly allowed low-risk
  source-like file classes after validated preview, sandbox dry-run evidence,
  independent accepted review, stale-target checks, rollback metadata,
  allowlisted verification, post-apply evidence, audit ledger, and emergency
  hold checks. They do not authorize unrestricted source mutation, forbidden
  file-class expansion, dependency/CI/build-script mutation, browser trusted
  writes, command bridges, autonomous source repair, auto-apply, auto-merge,
  production-ready mutation claims, secure-sandbox guarantees, native export,
  plugin runtime, hosted/cloud behavior, or current Godot replacement positioning.
- Treat Source Mutation Preview v1 as complete but preview-only. Its artifacts
  are evidence inputs for review/governance, not authority to apply patches,
  merge branches, execute browser-originated commands, mutate dependencies/CI,
  publish, package native exports, enable plugins, or launch publicly.
- Treat Public Alpha Launch Governance v1 as complete but non-executing. Its
  artifacts are decision/checklist/response inputs, not authority to toggle
  visibility, publish announcements, release packages, accept public roadmap
  scope automatically, promise support/security response times, or close #1/#23.
- Treat Production 2D Engine Core v1 as complete but bounded. Its artifacts are
  local evidence contracts for a small 2D vertical-slice class, not authority to
  claim production readiness, shipped-game maturity, secure sandboxing, broad
  compatibility stability, native export, plugin runtime, hosted/cloud behavior,
  source apply, public launch, or Godot replacement status.
- Treat Agentic Scene and Level Designer v1 as complete but bounded. Its
  artifacts are local evidence contracts for reviewable level/scene authoring,
  not authority to claim autonomous full game generation, production editor/full
  visual editor maturity, visual scripting, browser trusted writes, command
  bridges, unrestricted source apply, native export, plugin runtime, hosted/cloud
  behavior, public launch, production readiness, or Godot replacement status.
- Treat Gameplay Scripting / Logic System v1 as complete but bounded. Its
  artifacts are structured data-only local behavior/evidence contracts, not
  arbitrary script execution, a production-stable scripting API, secure sandbox
  guarantee, plugin runtime, command bridge, browser trusted write path,
  unrestricted source apply, native export, hosted/cloud behavior, public-launch
  automation, production readiness, or Godot replacement status.
- Treat Evaluator Depth v1 as complete but bounded. Its artifacts are
  mechanical visual diff and declared-invariant semantic evaluator contracts,
  not aesthetic scoring, general semantic correctness guarantees, source
  mutation authority, command bridges, browser trusted writes, native export,
  plugin runtime, hosted/cloud behavior, public-launch automation, release
  readiness, or engine-replacement status. Use Milestone 5.1 to consume these
  verdicts for evidence-linked mutation proposals rather than weakening the
  evaluator boundary.
- Treat Evolve Loop Depth v1 as complete but bounded. Its artifacts authorize
  evidence-linked, manual-review proposal inputs, bounded confidence signals,
  data/scene/scenario mutation type mapping, and four-gate rerun delta evidence;
  they do not authorize arbitrary source patching, auto-accept, auto-apply,
  auto-merge, self-approval, reviewer bypass, command bridges, browser trusted
  writes, native export, plugin runtime, hosted/cloud behavior, production
  readiness, or current Godot replacement positioning.
- Treat Full Studio Editor v1 as complete but bounded. Its artifacts authorize
  local project/scene/entity/asset/scenario/evidence/export/plugin inspection,
  draft-only browser authoring, Safe Source Apply handoff previews, workspace
  layout persistence, command-palette navigation/copy/draft actions, diagnostic
  surfaces, an integrated fixture demo, and Scenario Coverage v17. They do not
  authorize direct Studio trusted writes, command bridges, arbitrary shell
  execution, dependency install, CI/workflow mutation, credentialed operations,
  network install/update, publish/deploy/sign/upload, executable plugin runtime,
  marketplace behavior, autonomous apply, auto-merge, self-approval, reviewer
  bypass, native desktop editor, advanced visual scripting, full asset import
  pipeline, production collaboration, secure sandbox guarantees, full Godot
  parity, Godot replacement positioning, or production-ready editor claims.
- Treat Autonomous QA / Playtest Swarm v1 as complete but bounded. Its artifacts
  authorize deterministic local QA/playtest evidence, fixture-scoped fuzzing,
  explicit worker/rerun/time/output budgets, failure classification, backlog
  inputs, run matrices, evidence bundles, and read-only Studio/dashboard
  inspection; they do not authorize hidden workers, remote/cloud swarm
  orchestration, unbounded spawning, browser command bridges, browser trusted
  writes, auto-fix, auto-apply, auto-merge, self-approval, production QA
  certification, quality/fun/market/production safety guarantees, current Godot
  replacement positioning, or production-ready claims.
- Treat Build / Export / Packaging v1 as complete but local-only. Its artifacts
  authorize fixture-scoped local web package evidence, deterministic staging,
  profile/plan validation, asset manifests, runtime probe preservation,
  checksums/provenance, verification evidence, and read-only inspection; they do
  not authorize public release, deployment, signing, upload, app-store/Steam/itch
  publishing, desktop/mobile/native export, credentialed release flow,
  production-ready export claims, commercial distribution, secure distribution,
  multi-platform parity, or Godot replacement status.
- Treat Foundation Hardening v1 as complete but mechanical. Its artifacts
  authorize the realized `ouroforge-ledger`, `ouroforge-evidence`, and
  `ouroforge-evaluator` crate seams, re-export facades, and golden-parity audit;
  they do not authorize behavior changes, bug fixes, new user capability, source
  apply, native export, plugin runtime, hosted/cloud behavior, release
  readiness, production claims, or Godot replacement status.
- Treat Godot-Plus Demo Game v1 as complete but scoped. Its artifacts authorize
  claims about the Signal Gate local evidence-native workflow only: a small
  playable 2D vertical slice, Studio inspect/draft workflow, agentic iteration
  evidence, review-gated Safe Source Apply chain evidence, QA swarm evidence,
  QA/playtest swarm evidence, local web export/package verification, plugin
  descriptors used as metadata, scoped comparison evidence, performance budget
  evidence, documentation, and regression coverage. They do not authorize full
  Godot parity, mature editor tooling claims, native/mobile export, large game
  production, real marketplace, executable plugin ecosystem, production
  collaboration, commercial release readiness, production engine maturity,
  public deployment, direct Studio trusted writes, command bridges, unreviewed
  source mutation, auto-apply, auto-merge, self-approval, reviewer bypass,
  secure sandbox guarantees, or current Godot replacement positioning.
- Treat Plugin / Extension System v1 as complete but declarative-only. Its
  artifacts authorize claims about a bounded, allowlisted, evidence-backed
  extension foundation only: a declarative plugin manifest schema, local
  registry/discovery, an allowlisted extension point catalog, capability/
  permission and version-compatibility validation, descriptor evidence, a
  read-only Studio plugin browser and read-only dashboard/scenario/asset
  descriptors, a fixture plugin pack, a fail-closed security/threat-model gate,
  load-order/conflict detection, CLI inspection, Scenario Coverage v16, and a
  deterministic demo. They do not authorize executable plugins, arbitrary
  JavaScript, native/dynamic library loading, runtime script execution, shell
  command execution, dependency installation, network plugin install/update,
  marketplace, credential access, third-party package trust, full editor
  extensibility, source/export/publish/deploy mutation, CI/workflow mutation,
  secure plugin sandbox guarantees, Godot-equivalent extension parity, a
  production-ready plugin ecosystem, or current Godot replacement positioning.
- Treat Loop Coverage Metric v1 as complete for Era E Milestone 20 after the
  merged #1458/#1460-#1464 evidence chain. Its artifacts authorize only a
  descriptive authorship/verification fraction over trusted artifacts:
  provenance attribution, Rust/local computation/verdict/regression,
  fixture-scoped computed, insufficient-data, regressed/manual-drop, stale-ref,
  and unsupported examples, offline demo smoke coverage, Scenario Coverage v21,
  and read-only dashboard/Studio inspection. They do not authorize quality, fun,
  accessibility, production, release, or Godot replacement guarantees; trusted
  browser writes; source mutation authority; command bridges; auto-fix;
  auto-apply; auto-merge; self-approval; reviewer bypass; or the full
  intent-to-promotion provenance bundle, which remains Milestone 25 scope. From
  this milestone, the recommended next milestone is Era E Milestone 21: Second
  Game Class and Loop Generalization. Layer-3 distributed orchestration / Elixir
  per ADR #92, native export, plugin runtime, and hosted/cloud scope remain
  deferred and unchanged until Milestone 26 re-evaluation. #1 and #23 remain
  open.


### Era K Production Orchestration Executor governance refresh

Era K — Production Orchestration Executor (Studio Layer) — is recorded as
**complete on merged evidence** for Milestones 62–66. The completed evidence
chain is the M62 design gate and two-plane CLI contract (#1933 / PR #1957), the
M63 executor skeleton/scheduler/CLI-drive/golden-parity demo/Scenario Coverage
v55 chain (#1934–#1938 / PR #1958, #1959, #1960, #1962, #1967), the M64
supervision/budget/retry/recovery/demo/Scenario Coverage v56 chain (#1939–#1944
/ PR #1968, #1970, #1972, #1974, #1976, #1978), the M65
concurrency/backpressure/read-only telemetry/load-demo/Scenario Coverage v57
chain (#1945–#1949 / PR #1979, #1980, #1981, #1982, #1983), and the M66
Scenario Coverage v58 autonomy regression (#1950 / PR #1984) plus this #1951
governance refresh.

The autonomy assessment is recorded in
[`docs/executor-autonomy-assessment-v1.md`](executor-autonomy-assessment-v1.md).
It measures a bounded local concept-to-release-candidate envelope: 9 of 12
operational control-plane step families are executor-driven after approved
inputs, while humans remain responsible for intent, taste, legal/release
go/no-go, mandatory gates, and review of blocked or ambiguous states. This is a
descriptive local-control-plane measurement, not a production-readiness,
quality/fun/legal, hosted-ops, or release claim.

The two-plane invariant is reaffirmed. Elixir/OTP owns only the Studio executor
control plane — scheduling, supervision, budget checks, retry/backoff,
backpressure, and read-only telemetry. The Rust kernel remains the data plane and
continues to own artifact semantics, schemas, ledgers, evidence, verdicts,
review/apply/trust-gradient acceptance, and release truth. The executor reaches
the kernel only through the frozen `ouroforge` CLI surface, never writes
artifacts/ledgers/evidence directly, never owns artifact truth, never
self-certifies, and never releases. The manual Rust-CLI loop remains a tested,
first-class local fallback.

Distributed/multi-machine orchestration, hosted/cloud execution,
servers/databases, and live-ops remain Layer-3 DEFER under ADR #92 / Milestone
45 / #1508. Era K completion does not change those boundaries. #1 and #23 remain
open governance anchors.

### Era L Autonomous Self-Validation and Improvement Loop governance refresh

Era L — Autonomous Self-Validation and Improvement Loop (Real-Title
Dogfooding) — is recorded as **complete on merged evidence** for Milestones
68-72 plus the M73 coverage lock. The completed evidence chain is the M68
real-title dogfood run and Scenario Coverage v60 (#2023-#2027), M69
self-audit/attribution and acceptance audit (#2028-#2032), M70 autonomous
diagnosis and source-apply fix proposal (#2033-#2036 and fix-proposal issue
#2048), M71 re-verify/auto-apply routing with high-risk go/no-go queue
(#2037-#2041), M72 optional human channel and Scenario Coverage v64
(#2042-#2045), and M73 Scenario Coverage v65 plus this governance refresh
(#2046-#2047).

The autonomous-loop maturity assessment is recorded in
[`docs/era-l-autonomous-loop-maturity-assessment.md`](era-l-autonomous-loop-maturity-assessment.md).
For the covered low-risk reversible self-improvement cycle, the autonomous
eligible-cycle completion fraction is `1 / 1 = 100%`: the loop completes
`detect -> explain -> trace -> attribute -> propose -> re-verify -> apply`
with zero human action. Across covered fix candidates, safe terminal routing is
`2 / 2 = 100%`: the low-risk candidate auto-applies only after independent
re-verification, while the high-risk/source-affecting candidate is verified and
queued for thin human go/no-go provenance instead of being auto-applied. The
high-risk auto-apply rate remains `0 / 1 = 0%` by design.

The autonomy-first invariant is reaffirmed. The default self-loop runs to
completion without a human, optional oversight/taste surfaces are read-only or
provenance-only and non-blocking, fun/taste and public release go/no-go stay
human Ring 2, and high-risk/source-affecting engine changes never auto-apply.
Engine fixes route through the existing source-apply path, the four gates plus
design-integrity, openchrome re-run evidence, rollback/kill-switch, and
trust-gradient; no parallel verification engine, telemetry store, persistent
store, or data plane was introduced. The Rust kernel/evaluator/source-apply
chain remains the data plane; the Elixir executor remains an unchanged control
plane. Era L improves the engine harness/pipeline, not game taste. Layer-3
distributed/hosted/live-ops scope remains DEFER. #1 and #23 remain open
governance anchors.

### Era M Active Human Intervention governance refresh

Era M — Active Human Intervention (Agent-First, Human-Steerable) — is recorded
as **complete on merged evidence** for Milestones 74-80 plus this M81
governance refresh. The completed evidence chain covers the M74 posture and
Phoenix/two-plane design gate (#2052 / PR #2101), M75 proposal amendment
(#2053-#2056 / PRs #2146, #2149, #2151, #2152; Scenario Coverage v66), M76
human-authored artifact intake (#2057-#2060 / PRs #2155, #2158, #2160, #2161;
Scenario Coverage v67), M77 live campaign steering directives (#2064 / PR
#2153; Scenario Coverage v68), M78 human constraints as first-class gates
(#2065-#2068 / PRs #2162-#2165; Scenario Coverage v69), M79 diagnosis
correction and intervention feedback (#2069-#2072 / PRs #2166, #2242, #2243,
#2244; Scenario Coverage v70), M80 stage takeover and handback (#2076 / PR
#2159; Scenario Coverage v71), and this M81 governance refresh (#2077).

The human-steerability assessment is recorded in
[`docs/era-m-human-steerability-assessment.md`](era-m-human-steerability-assessment.md).
For the covered intervention classes, coverage is `6 / 6 = 100%`: proposal
amendment, human-authored artifact intake, live campaign steering, human
constraints, diagnosis correction/attribution feedback, and stage
takeover/handback all route human input as validated, recorded intervention
evidence through the existing review/apply, scene/source-apply, evaluator,
evidence, and provenance gates. No covered class grants raw writes.

The agent-first invariant is reaffirmed. Human intervention is opt-in at defined
points and never required; the autonomous loop still completes with zero human
input and the CLI fallback remains sufficient. Studio surfaces are now read +
gated-write, but only locally: Elixir/OTP + Phoenix LiveView capture, route, and
render intervention intent as control/presentation-plane data, while Rust
remains the data plane for artifact truth, validation, determinism, evidence,
provenance, diagnosis semantics, scene/source-apply, and gated writes. Hosted,
multi-user, collaborative, or real-time remote Studio remains Layer-3 DEFER.
Fun/taste verdict and release go/no-go remain human Ring 2 decisions. Era M does
not introduce raw bypasses, a new write path, a new data store, opaque ML
authority, browser command bridges, auto-merge, production-readiness claims, or
Godot-replacement positioning. #1 and #23 remain open governance anchors.

## Active anchors

- #1 remains the broad vision and implementation-roadmap anchor until a separate
  explicit governance decision replaces it.
- #23 remains open as the repo-memory/design context anchor.

## Non-goals

Ouroforge is not currently trying to be:

- a Godot replacement;
- a production-ready or compatibility-stable public engine API;
- a hosted/cloud engine;
- a native packaged editor or native export implementation;
- a general marketplace or plugin platform;
- a browser-side trusted file writer or command bridge;
- an autonomous public-launch automation system.

Any shift in those boundaries requires a design issue, explicit maintainer
approval, and evidence that the change belongs in the current roadmap.


## Current roadmap anchors

- Full Studio Editor v1 scope contract: [`docs/full-studio-editor-v1.md`](full-studio-editor-v1.md).
- Full Studio Editor integrated demo: [`docs/full-studio-editor-integrated-demo-v1.md`](full-studio-editor-integrated-demo-v1.md).
- Full Studio Editor Scenario Coverage v17: [`docs/scenario-coverage-v17-full-studio-editor.md`](scenario-coverage-v17-full-studio-editor.md).
- Godot-Plus Demo Game v1 scope contract: [`docs/godot-plus-demo-game-v1.md`](godot-plus-demo-game-v1.md).

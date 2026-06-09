# ADR: Semantic Re-Derivation Methodology v1

## Status

Accepted for Era R Milestone 107. This ADR is a design gate for downstream
semantic-port milestones. It defines the one-way on-ramp contract that imports a
source project's declarative skeleton and re-derives logic clean-room; it does
not authorize engine absorption, a live bridge, embedded Unity/Unreal/Godot
runtime execution, or auto-porting a finished game.

## Context

Ouroforge's migration on-ramp must help teams move source-owned, open/text game
projects into Ouroforge-native artifacts without weakening the evidence-native
kernel. External engines can describe scenes, assets, presentation graphs,
metadata, and some declarative configuration, but their imperative gameplay
logic, physics feel, shaders, and runtime timing cannot be treated as trusted or
deterministic merely because an adapter can parse them.

Era R therefore separates two different acts:

1. **Skeleton import** — one-way ingestion of source-project, open/text,
   declarative artifacts into explicit intermediate records and Ouroforge-native
   drafts.
2. **Semantic re-derivation** — clean-room reconstruction of behavior from
   observed behavior, captured acceptance evidence, and interrogated human
   intent.

Re-derivation is not source-code translation. No milestone may copy decompiled
source, rip shipped builds, embed a foreign runtime, or claim a unit has been
"ported" before an oracle passes.

## Decision

Ouroforge adopts semantic re-derivation as the only approved Era R path for
legacy logic. The Rust kernel owns artifact truth, import validation, extraction,
mapping, deterministic re-expression, state hashes, and evaluator gates. The
Elixir/Phoenix Studio remains a local control and presentation plane that renders
Rust-owned evidence and routes every write through existing CLI/gated write
paths; it owns no artifact semantics and performs no trusted writes.

## Unit Model

A **behavioral unit** is the smallest externally observable gameplay obligation
that can be independently named, stimulated, observed, oracle-checked, and
re-expressed deterministically. A unit is not a source file, method, script,
prefab, node, or engine component. A unit record must include:

- a stable unit id and human-readable name;
- source provenance for the declarative skeleton elements that suggested the
  unit;
- stimuli and preconditions that exercise it;
- expected state, event, and/or presentation outcomes;
- non-deterministic or engine-owned assumptions that must be re-simulated rather
  than reproduced;
- oracle evidence ids and verdict status;
- fidelity grade and known gaps;
- the downstream re-expression target in Ouroforge-native data or deterministic
  Rust logic.

The unit boundary is outcome-level. Multiple source scripts may collapse into one
behavioral unit, and one source script may decompose into many units.

## Oracle Rule

No oracle means not ported. A behavioral unit may be recorded as discovered,
extracted, interrogated, scaffolded, or re-expression-ready, but it is not called
"ported", "complete", or "semantically equivalent" unless it has captured
acceptance evidence and passes its oracle.

An oracle must identify:

- the intended behavior in source-independent terms;
- the stimulus sequence and initial state;
- the expected deterministic state hash or outcome assertions;
- for 2.5D/3D presentation, the perceptual comparison artifact used only as
  secondary corroboration;
- allowed tolerances and the reason for each tolerance;
- gaps that remain best-effort content fidelity rather than logic equivalence.

Fidelity reports and coverage verdicts must be honest: partial content import,
missing source metadata, unsupported engine features, or unverified behavior must
be attributed to explicit gaps instead of hidden inside a green result.

## Clean-Room Re-Derivation Procedure

Downstream milestones must follow this sequence:

1. **Source-project eligibility check.** Accept only source-project open/text
   formats such as Godot `.tscn`/`.tres` or Unity Force-Text YAML with `.meta`.
   Reject shipped-build ripping, opaque binary extraction, and decompiled code.
2. **Declarative skeleton ingestion.** Parse scenes, assets, metadata,
   presentation hierarchy, references, and declarative configuration into a
   provenance-preserving IR. This is a one-way import into Ouroforge-native
   drafts; no live bridge or embedded engine runtime is created.
3. **Behavioral-unit extraction.** Identify observable behavior obligations from
   skeleton signals, inputs, collisions, state labels, scene transitions,
   animation triggers, UI bindings, and human notes. Do not treat source code as
   implementation authority.
4. **Interrogation.** Ask targeted questions or capture source-independent notes
   to resolve intent, ambiguous feel, win/loss semantics, timing expectations,
   and unsupported features.
5. **Oracle capture.** Record acceptance evidence for each unit before claiming
   it is ported. Oracles bind stimuli to outcome-level assertions and evidence
   artifacts.
6. **Deterministic re-expression.** Re-implement the behavior in
   Ouroforge-native deterministic data/Rust logic. Physics is re-simulated under
   Ouroforge rules; it is never reproduced by delegating to the source engine.
7. **Differential verification.** Compare source-observed outcomes and
   Ouroforge outcomes at the unit level. Differences are classified as pass,
   tolerated/content-fidelity gap, human-feel escalation, or blocker.
8. **Coverage and convergence.** Publish semantic-port coverage with unverified
   units, unsupported features, and human Ring 2 decisions still visible.

## Differential Verification Semantics

Verification is outcome-level, not source-shape-level. The question is whether
Ouroforge produces the accepted behavior under the captured oracle, not whether
its code, timing internals, physics solver, or scene graph exactly resembles the
source engine.

- **2D logic:** bit-exact deterministic state hashes are required for the
  accepted state trajectory.
- **2.5D/3D logic:** deterministic state-hash is primary. Pixel-diff/SSIM or
  other render comparisons are secondary evidence for presentation fidelity, not
  the source of truth for logic equivalence.
- **Physics:** source physics is re-simulated with deterministic Ouroforge rules.
  Solver-identical reproduction is not promised.
- **Shaders/VFX/audio/feel:** presentation fidelity can be reported and improved,
  but fun/feel and release go/no-go remain human Ring 2 decisions.

A failing oracle blocks a "ported" claim even if imported content looks visually
similar. A passing visual comparison cannot override a failing state hash.

## O/P/Q Hand-Off Contract

Era O/P/Q may supply skeleton and presentation artifacts to Era R; Era R supplies
verified semantic units back to the migration lanes.

| Source lane | Inputs to Era R | Era R output | Boundary |
| --- | --- | --- | --- |
| O: External-engine 2D on-ramp | 2D scenes, assets, open/text metadata, source-observed traces, adapter provenance | 2D behavioral units, oracles, deterministic re-expression records, bit-hash evidence | No auto-port without oracle; no engine runtime bridge |
| P: 2.5D on-ramp | 3D presentation with 2D-deterministic logic, glTF-normalized presentation evidence, state traces | deterministic logic units plus secondary perceptual fidelity report | State-hash primary; render comparison secondary |
| Q: Full-3D reevaluation | bounded 3D candidate evidence and DEFER/GO gate outputs | re-derivation risk/coverage evidence only when Q allows a bounded GO | Full 3D remains DEFER by default; no production 3D claim |

Era R never weakens the earlier migration gates. It consumes their source-only,
open/text, provenance-preserving artifacts and returns evidence-linked semantic
coverage.

## Fidelity Grades

Semantic-port coverage uses conservative grades:

- **Green:** oracle captured and passing; deterministic state evidence satisfies
  the lane's hash rule; presentation gaps are absent or explicitly tolerated.
- **Yellow:** skeleton/content imported or behavior scaffolded, but oracle is
  missing, partial, tolerated, or human-feel escalated. Not ported.
- **Red:** unsupported, legally ineligible, nondeterministic, copied/decompiled,
  or failing oracle. Not ported; requires re-derivation or rejection.

Reports must prefer Yellow/Red over false Green. A content import may be useful
while still carrying no semantic-port claim.

## Legal and Trust Boundary

- Source-project and open/text formats only.
- No shipped-build ripping.
- No copied or translated decompiled source.
- One-way import to Ouroforge-native artifacts.
- No live bridge and no embedded Unity/Unreal/Godot runtime.
- Rust remains data plane owner for parsers, IR, mapping, re-expression,
  determinism, evidence, and gates.
- Elixir/Phoenix remains control and presentation only and performs no trusted
  writes.
- Human Ring 2 owns fun/feel and release go/no-go.

## Consequences

This ADR intentionally slows "looks imported" demos until behavioral evidence is
captured. It protects the project from legal ambiguity, nondeterministic engine
absorption, misleading auto-port claims, and silent partial ports. Downstream
milestones must cite this ADR when adding ingestion, interrogation, oracle,
re-expression, differential verification, Studio UX, and Era R governance work.

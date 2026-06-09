# Legacy Logic Ingestion and Behavioral-Unit Extraction Contract v1

Era R Milestone 108 defines the bounded contract for taking legacy project
signals into Ouroforge's semantic re-derivation lane. It cites and inherits
`docs/semantic-re-derivation-methodology-adr-v1.md`: re-derivation is not
translation, no oracle means not ported, and the on-ramp is a one-way import into
Ouroforge-native artifacts.

## Goal

The goal of M108 is to ingest the declarative skeleton of a source-owned legacy
project and extract candidate behavioral units for later interrogation, oracle
capture, deterministic re-expression, and differential verification. This
milestone does not port logic. It defines the source-only subset, the output
records, the fidelity/oracle grading, and the hand-off to the rest of Era R.

## Source Eligibility and Boundary

Accepted inputs are source-project artifacts in open/text or source-controlled
forms:

- Godot `.tscn` and `.tres` scene/resource text;
- Unity Force-Text YAML scene, prefab, asset, and `.meta` records;
- source-owned asset manifests, glTF/texture/audio references, input maps,
  tags/layers, animation names, collision masks, and UI binding metadata;
- optional human-authored notes that describe intended behavior in
  source-independent terms;
- optional observed traces captured from a lawful source-project run for oracle
  design, without shipped-build ripping or decompiled source copying.

Rejected inputs are shipped builds, ripped assets from games the operator does
not own, decompiled source, opaque binary-only project data, editor plugins that
must run inside a foreign engine, and runtime bridges to Unity/Unreal/Godot.

The import is one-way. Parsed records become Ouroforge evidence, IR, reports, or
review-gated drafts. The source engine is never embedded, delegated to, or kept
as a live bridge.

## Exact Inputs

A M108 ingestion run may consume:

| Input | Required fields | Purpose |
| --- | --- | --- |
| Source manifest | project id, engine family/version when known, root-relative file list, ownership/source-only declaration | Establish legal/source boundary and deterministic traversal. |
| Scene/resource text | stable path, format kind, content hash, parser version | Extract hierarchy, component metadata, signal names, references, and presentation skeleton. |
| Asset/reference metadata | root-relative path, kind, source hash or declared missing reason | Preserve content provenance and produce honest fidelity reports. |
| Interaction hints | input actions, triggers, collision layers, UI events, animation callbacks, scene transitions | Suggest behavioral-unit candidates without claiming implementation equivalence. |
| Human notes | author, intent text, linked source paths or entities | Seed interrogation and oracle design. |
| Optional observed traces | stimulus, state/event observations, screenshot/perceptual refs where scoped | Inform oracle capture, not source-code translation. |

Every input path is repo-relative to the imported source snapshot or explicitly
labeled external reference. Hashes and parser versions are recorded so stale or
malformed evidence is visible.

## Exact Outputs

M108 produces Rust-owned artifacts only:

| Output | Meaning | Write authority |
| --- | --- | --- |
| `legacy_ingestion_report` | source eligibility verdict, parsed/skipped files, parser warnings, unsupported features, legal boundary status | Rust CLI/gates |
| `declarative_skeleton_ir` | normalized scene/resource/entity/reference graph with provenance hashes | Rust CLI/gates |
| `behavioral_unit_candidates` | candidate unit records named by observed obligation, not source file/function shape | Rust CLI/gates |
| `fidelity_report` | Green/Yellow/Red grades for skeleton/content coverage and semantic readiness | Rust CLI/gates |
| `era_r_handoff` | list of units requiring interrogation, oracle capture, deterministic re-expression, or rejection | Rust CLI/gates |

Elixir/Phoenix may display these reports or submit human intent through existing
CLI/gated write flows. It must not mutate reports directly, define artifact
semantics, or write trusted project state.

## Behavioral-Unit Extraction Rules

A behavioral unit candidate is extracted from observable obligations such as:

- input action produces movement, attack, jump, menu, or ability state;
- collision, trigger, area, or signal produces state/event/scene changes;
- resource variable or exported property implies a rule parameter;
- UI binding, animation marker, or scene transition implies a player-visible
  outcome;
- human notes name intended behavior, fail state, win state, pacing, or feel.

Candidates must include source provenance, stimulus hypotheses, observable
outcomes, unresolved intent questions, unsupported engine assumptions, and the
next Era R stage. They must not include copied source-code bodies, decompiled
logic, or claims that source control-flow was translated.

## Fidelity Grades and Oracle Rule

M108 uses conservative grades:

- 🟢 **Green — skeleton ready / oracle-backed.** The source input is eligible,
  parsed deterministically, provenance is complete, and any semantic claim has a
  captured passing oracle. Green behavioral units can proceed to deterministic
  re-expression or be marked semantically covered if a later gate passes.
- 🟡 **Yellow — partial or needs re-derivation.** The source input is eligible
  and useful, but behavior is inferred, an oracle is missing/partial, content is
  best-effort, or human intent is unresolved. Yellow is not ported.
- 🔴 **Red — blocked/reject/re-derive.** The input is legally ineligible,
  unsupported, malformed, stale, nondeterministic, copied/decompiled, or fails an
  oracle. Red is not ported and must be rejected or re-derived from clean-room
  behavior/intent.

No behavioral unit is called ported by M108. The only allowed semantic status is
candidate, needs-interrogation, oracle-ready, blocked, or later-stage verified.
No oracle means not ported.

## Determinism and Differential-Verification Hand-Off

M108 does not complete differential verification, but every hand-off record must
state which later evidence will be required:

- 2D units require bit-exact deterministic state hashes under captured stimuli;
- 2.5D/3D units require deterministic state-hash primary evidence, with
  perceptual SSIM/pixel-diff only as secondary presentation evidence;
- physics behavior is re-simulated in Ouroforge and never reproduced by invoking
  the source engine;
- shaders, VFX, audio, fun, feel, and release go/no-go remain explicit gaps or
  human Ring 2 decisions unless a later bounded issue supplies evidence.

## Era R Hand-Off

M108 hands each unit to exactly one next state:

| Next state | Consumer | Criteria |
| --- | --- | --- |
| `interrogate` | M109 | Intent, stimuli, or expected outcome is ambiguous. |
| `capture_oracle` | M109/M111 | Behavior is understood enough to write source-independent acceptance evidence. |
| `reexpress` | M110 | Oracle and deterministic requirements are ready for clean-room Ouroforge implementation. |
| `verify` | M111/M112 | Re-expression exists and needs A/B outcome checks or coverage roll-up. |
| `reject_or_defer` | governance/human | Legal, unsupported, nondeterministic, or human-feel blocker. |

The hand-off preserves all gaps. It is a queue of re-derivation work, not a port
completion report.

## Non-Goals

- No finished-game auto-port.
- No foreign runtime embedding or live bridge.
- No decompiled-source translation.
- No new datastore or non-Rust trusted write path.
- No Studio-owned artifact semantics.
- No claim that visual similarity, parsed scripts, or imported content equals
  behavioral equivalence.

## Verification Contract

A valid implementation/demo/coverage issue that references this contract must
prove:

1. source eligibility is checked before parsing;
2. every parsed/skipped item appears in the ingestion or fidelity report;
3. every behavioral unit has provenance, unresolved questions or oracle status,
   and a next Era R hand-off state;
4. Green/Yellow/Red grades are conservative and never hide gaps;
5. no unit is called ported without a captured passing oracle;
6. Rust owns all artifact writes; Elixir/Phoenix displays and routes only;
7. #1 and #23 remain open governance anchors.

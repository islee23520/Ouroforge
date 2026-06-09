# Tacit-Knowledge Interrogation and Oracle Capture Contract v1

Era R Milestone 109 defines the bounded contract for turning M108 behavioral-unit
candidates into source-independent intent records and oracle evidence. It
inherits `docs/semantic-re-derivation-methodology-adr-v1.md` and
`docs/legacy-logic-ingestion-behavioral-unit-contract-v1.md`: the on-ramp is
one-way, re-derivation is not translation, no oracle means not ported, and Rust
owns artifact truth while Elixir/Phoenix only presents evidence and routes gated
human input.

## Goal

M109 captures tacit knowledge that is missing from source-project skeletons and
legacy logic candidates: designer intent, expected stimuli, observable outcomes,
state tolerances, edge cases, and release/fun/feel caveats. The milestone
produces interrogation records and oracle specifications that downstream Era R
issues can use for deterministic re-expression and differential verification.
It does not implement game logic, translate source code, or declare a finished
port.

## Source Eligibility and Boundary

Accepted inputs are clean-room, source-independent evidence tied back to M108
candidate units:

- M108 `behavioral_unit_candidate` records with provenance, gaps, and next-state
  hand-off information;
- source-owned/open-text scene, asset, input, animation, collision, and UI
  metadata referenced by those candidates;
- human-authored intent answers, design notes, glossary terms, tuning targets,
  and examples supplied by an authorized operator;
- lawful observed behavior traces from a source-project run, such as stimulus
  scripts, event logs, state snapshots, screenshots, or perceptual references;
- explicit uncertainty, unsupported feature, and legal-boundary annotations.

Rejected inputs are decompiled source, shipped-build ripping, copied source-code
bodies, opaque binary-only evidence, foreign-engine plugins that must execute to
answer the question, and any live Unity/Unreal/Godot bridge. The interrogation
may cite source paths and observed outcomes, but it must phrase requirements in
Ouroforge-native behavior terms rather than source control-flow or engine API
translation.

## Exact Inputs

A M109 interrogation/oracle capture run may consume:

| Input | Required fields | Purpose |
| --- | --- | --- |
| Behavioral unit candidate | unit id, M108 source refs, fidelity grade, gaps, next hand-off state | Anchor questions to an already eligible clean-room unit. |
| Interrogation question set | question id, target unit, ambiguity being resolved, allowed answer shape | Capture tacit intent without copying source implementation. |
| Human answer record | author/role, timestamp or sequence, answer text, confidence, linked question id | Record design intent and unresolved caveats. |
| Observed trace summary | stimulus, state/event observations, render/perceptual refs where scoped, source-run provenance | Convert observed behavior into oracle candidates without engine absorption. |
| Oracle specification | stimulus fixture, expected state hash or event sequence, tolerances, required/secondary evidence | Define the acceptance evidence later gates must pass. |
| Fidelity/gap update | Green/Yellow/Red grade, gap reasons, blocked/defer reasons | Keep partial knowledge honest and visible. |

Inputs must be deterministic to read and replay as data. Human answers can be
free text, but the captured oracle specification must normalize them into stable
fields before downstream gates treat them as acceptance evidence.

## Exact Outputs

M109 produces Rust-owned artifacts and presentation-only Studio surfaces:

| Output | Meaning | Write authority |
| --- | --- | --- |
| `interrogation_session_report` | ordered questions, answers, unresolved ambiguity, provenance links, and legal/source boundary status | Rust CLI/gates |
| `tacit_intent_record` | source-independent statement of player-visible intent, stimuli, outcomes, tolerances, and caveats | Rust CLI/gates |
| `oracle_capture_spec` | deterministic acceptance fixture requirements for state hashes, event sequences, and secondary perceptual evidence | Rust CLI/gates |
| `oracle_evidence_manifest` | links to captured traces, screenshots, hashes, and known gaps; may be pending or failing | Rust CLI/gates |
| `fidelity_report_update` | conservative Green/Yellow/Red status for oracle readiness and semantic risk | Rust CLI/gates |
| `era_r_handoff` | next action: ask more, capture/repair oracle, re-express, verify, reject, or defer | Rust CLI/gates |

Elixir/Phoenix may render the session, collect local operator input, and invoke
existing `ouroforge` CLI/gated paths. It must not directly mutate reports,
create trusted oracle evidence, define artifact semantics, or bypass Rust gates.

## Fidelity Grades and Oracle Rule

M109 uses conservative grades:

- 🟢 **Green — oracle captured and replay-ready.** The unit has eligible M108
  provenance, answered intent, deterministic stimulus, expected state/event
  outcomes, declared tolerances, and captured acceptance evidence sufficient for
  downstream re-expression or A/B verification. Green is oracle-ready, not
  automatically ported.
- 🟡 **Yellow — partially understood / needs capture.** Intent is partly known,
  traces exist but are incomplete, tolerances are unresolved, secondary
  perceptual evidence is missing, or human answers leave caveats. Yellow is not
  ported and must continue interrogation or capture.
- 🔴 **Red — blocked/reject/defer.** Evidence is legally ineligible,
  decompiled/copied, nondeterministic, stale, contradictory, missing source
  ownership, dependent on a foreign runtime bridge, or scoped to fun/feel/release
  decisions that remain human Ring 2. Red is not ported.

No M109 output may claim a unit is ported. Passing oracle evidence is a
prerequisite for later semantic coverage, and even a captured oracle only
permits deterministic re-expression/verification work. No oracle means not
ported.

## Oracle Specification Rules

An oracle specification must be source-independent and measurable:

1. identify the unit, source provenance, and clean-room intent statement;
2. define deterministic stimuli and initial state in Ouroforge-native terms;
3. define expected 2D bit-exact state hashes or event/state sequences;
4. for 2.5D/3D, define deterministic state-hash primary evidence and use
   perceptual SSIM/pixel-diff only as secondary render evidence;
5. declare tolerances, unsupported features, human-feel caveats, and stale
   evidence rules;
6. link every trace or image to provenance without copying decompiled source;
7. fail closed when evidence is missing, lossy, unauthenticated, or
   nondeterministic.

Physics behavior is re-simulated in Ouroforge from captured intent and outcome
constraints. M109 may describe observed source physics results, but it must not
reproduce them by invoking the source engine or translating engine internals.

## Era R Hand-Off

M109 hands each unit to exactly one next state:

| Next state | Consumer | Criteria |
| --- | --- | --- |
| `ask_more` | M109 | Intent, stimuli, expected state, or tolerance remains ambiguous. |
| `capture_or_repair_oracle` | M109/M111 | Intent is understood but acceptance evidence is missing, stale, or failing. |
| `reexpress` | M110 | Oracle is captured and deterministic enough for clean-room Ouroforge implementation. |
| `verify` | M111/M112 | Re-expression exists and needs differential A/B or semantic-port coverage roll-up. |
| `reject_or_defer` | governance/human | Legal, unsupported, nondeterministic, or human-feel/release blocker. |

The hand-off is an evidence queue, not a completion report. Gaps remain explicit
until downstream gates close them.

## Non-Goals

- No finished-game auto-port.
- No source-code translation or decompiled-source copying.
- No foreign runtime embedding, plugin execution, or live bridge.
- No new datastore or non-Rust trusted write path.
- No Studio-owned artifact semantics or trusted oracle writes.
- No automation of fun/feel or release go/no-go.
- No claim that a captured trace, screenshot, or answered question equals a
  completed semantic port.

## Verification Contract

A valid implementation/demo/coverage issue that references this contract must
prove:

1. every interrogation session is anchored to an eligible M108 unit or explicitly
   rejected;
2. every human answer and observed trace has provenance and unresolved caveats;
3. oracle specs are deterministic, source-independent, and replayable as data;
4. Green/Yellow/Red grades are conservative and never hide missing oracle
   evidence;
5. no output is called ported without captured passing oracle evidence;
6. 2D uses bit-exact state hashes and 2.5D/3D uses state-hash primary plus
   perceptual secondary evidence;
7. Rust owns all trusted artifact writes; Elixir/Phoenix displays and routes
   through CLI/gates only;
8. #1 and #23 remain open governance anchors.

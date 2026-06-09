# Deterministic Re-Expression Engine Contract v1

Era R Milestone 110 fixes the contract for deterministic re-expression before
implementation. This is an on-ramp contract for turning captured clean-room
behavioral intent and oracle evidence into Ouroforge-native deterministic logic
specifications. It is not engine absorption, not a source-code translation pass,
and not an auto-port of a finished game.

## Scope and boundary

The deterministic re-expression engine accepts only Era R artifacts that have
already passed the source-project/open-text and clean-room gates from M107-M109:

- declarative skeleton references imported one-way from source projects (for
  example Godot `.tscn`/`.tres` or Unity Force-Text YAML plus `.meta`),
- `BehavioralUnitRecord`-style units extracted from source-only legacy logic,
- captured tacit intent answers with non-blocked provenance,
- observed behavior traces with deterministic state-hash evidence,
- oracle specs that explicitly deny any `ported` claim until verification passes.

The output is an Ouroforge-native re-expression plan and candidate deterministic
logic spec. The plan may be handed to downstream implementation and verification
milestones, but this milestone itself writes no runtime, embeds no foreign engine,
and copies no decompiled source.

## Inputs

Each re-expression request must include:

1. `unit_id` and source-system metadata from the legacy ingestion report.
2. A captured oracle spec from M109 with a deterministic state hash.
3. The behavior-level intent summary, expected events, and acceptance assertions.
4. Skeleton/IR references needed to bind scene, asset, or presentation context.
5. Provenance and legal-boundary metadata proving source-project/open-text input.
6. Target dimensionality: `2d`, `2_5d`, or `3d`.

Requests without a captured oracle remain re-derivation tasks; they are not
eligible for Green fidelity or for a ported/completed claim.

## Outputs

The re-expression engine must produce Rust-owned, deterministic, reviewable data:

- `ReExpressionPlan`: normalized unit id, target runtime substrate, source IR refs,
  oracle refs, deterministic constraints, unsupported gaps, and follow-up tasks.
- `DeterministicLogicSpec`: data-only behavior description in Ouroforge-native
  terms (events, state transitions, guards, inputs, outputs, and fixed-tick rules).
- `ReExpressionFidelityReport`: Green/Yellow/Red grades, oracle status, drift
  reasons, blocked provenance, and explicit no-port-claim flags.
- `VerificationHandoff`: the state hashes and render corroboration requirements
  that M111 differential verification must run.

These outputs are data-plane artifacts. Elixir/Phoenix Studio may render them and
route human review through gated CLI flows, but Studio has no artifact semantics,
no trusted-write authority, and no direct mutation path.

## Gated path

1. **Ingest**: read M108/M109 reports only; do not inspect decompiled or shipped
   build sources.
2. **Normalize**: bind unit intent, observed events, and skeleton refs into a
   stable Rust data model with deterministic ordering.
3. **Re-express**: map behavior into Ouroforge-native deterministic events and
   state transitions. This is a clean-room re-implementation target, not a
   syntactic translation of legacy code.
4. **Grade**: emit Green/Yellow/Red fidelity with explicit gaps.
5. **Handoff**: send only Green candidates to differential verification; Yellow
   candidates go back to interrogation/oracle capture; Red candidates are rejected
   or deferred.
6. **Verify later**: M111 must prove state-hash behavior before any human-facing
   text may claim the unit was ported or accepted.

## Fidelity and oracle rule

- 🟢 **Green — re-expression-ready**: source-only/open-text provenance is clean;
  a captured oracle exists; the deterministic state hash is present; unsupported
  behavior is absent or explicitly out of scope; the output remains a candidate
  until M111 verification passes. `ported_claim_allowed` remains false here.
- 🟡 **Yellow — needs interrogation or bounded repair**: intent is partial,
  trace evidence is missing, skeleton binding is lossy, dimensionality is
  under-specified, or perceptual corroboration is needed. The result is a task,
  not an accepted re-expression.
- 🔴 **Red — blocked or defer**: decompiled/ripped/shipped-build provenance,
  live-bridge dependency, embedded foreign-runtime requirement, nondeterministic
  physics/shader dependency that cannot be bounded, or any attempt to claim a
  port without oracle evidence.

Oracle rule: no unit may be called ported, accepted, or behaviorally equivalent
unless it has captured acceptance evidence and downstream differential
verification. Content import remains best-effort with an honest fidelity report.

## Determinism contract

- 2D behavior gates on bit-exact state hashes.
- 2.5D/3D behavior gates on deterministic state-hash primary evidence.
- Perceptual render evidence such as SSIM or pixel diff is secondary
  corroboration only; it never replaces state-hash verification.
- Physics is re-simulated as Ouroforge-native deterministic behavior and never
  reproduced by embedding or replaying the source engine.
- All output ordering, hashes, and report digests must be stable for identical
  inputs and change when oracle state hashes or behavior assertions change.

## Era R hand-off

- Missing oracle → M109 interrogation/oracle capture.
- Green re-expression candidate → M111 differential verification A/B.
- Verified candidate with coverage gaps → M112 semantic-port coverage and
  convergence tracking.
- Studio visualization or human review needs → M113 UX surface, still two-plane
  and gated through the `ouroforge` CLI.
- Governance status → M114; #1 and #23 remain open anchors.

## Non-goals

- Finished-game auto-porting or any claim that a project is ported automatically.
- Live bridge to Unity, Unreal, Godot, or any other non-deterministic engine.
- Embedded source runtime, shipped-build ripping, or decompiled-code copying.
- Direct Elixir/Phoenix trusted writes or a new Studio data plane.
- Faithful reproduction of source physics, shaders, VFX, fun, feel, or release
  go/no-go decisions.

## Verification

```bash
grep -RIlqi "one-way\|on-ramp\|re-derivation\|fidelity\|two-plane\|source-project" docs/ || true
cargo build --workspace --jobs 2
```

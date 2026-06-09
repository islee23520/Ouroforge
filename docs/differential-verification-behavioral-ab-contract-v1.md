# Differential Verification Behavioral A/B Contract v1

Era R Milestone 111 fixes the contract for differential verification before
implementation. This milestone compares a source-project/open-text behavioral
oracle captured from the legacy project against an Ouroforge-native
re-expression candidate. It is an oracle-gated behavioral A/B check, not engine
absorption, not live bridging, not source-code translation, and not a finished-game auto-port claim.

## Scope and boundary

Differential verification accepts only artifacts that already passed the Era R
on-ramp gates:

- source-project/open-text skeleton and IR references from M107-M108,
- behavioral units extracted without decompiled or shipped-build source,
- captured M109 oracle specs with source-independent intent and deterministic
  observed behavior evidence,
- M110 deterministic re-expression plans and candidate native behavior drafts,
- source-apply/review gate metadata showing that any future write is gated,
  rollback-tracked, and outside Studio trusted-write authority.

The verifier may run the Ouroforge-native candidate and compare it with captured
oracle evidence. It must not run or embed Unity, Unreal, Godot, shipped builds,
or any live foreign-engine runtime as an authoritative comparison source.
Captured evidence is replay/reference data only; native verification remains
Rust-owned and deterministic.

## Inputs

Each verification request must include:

1. `unit_id`, source skeleton/IR refs, and clean-room provenance metadata.
2. The M109 oracle spec: stimulus, expected events, primary state hash, optional
   secondary render digest, tolerance, and provenance refs.
3. The M110 re-expression candidate: plan id, behavior draft id or native spec
   ref, declared deterministic constraints, and gate handoff.
4. Target dimensionality: `2d`, `2_5d`, or `3d`.
5. Captured baseline evidence refs and native-run evidence refs, both
   repo-relative and immutable.
6. A no-port-claim flag that remains false until this contract produces a Green
   verified outcome and downstream coverage/convergence accepts it.

Missing oracle specs, stale candidate refs, ungated writes, untrusted Studio
writes, runtime bridges, or decompiled/shipped-build provenance fail before A/B
comparison.

## Outputs

The differential verifier must emit Rust-owned, deterministic, reviewable data:

- `DifferentialVerificationReport`: schema version, unit id, candidate id,
  baseline refs, native refs, dimensionality, status, deterministic digest, and
  boundary statement.
- `BehavioralABResult`: per-stimulus pass/fail/inconclusive records, expected
  events, observed native events, state-hash comparison, and optional render
  comparison.
- `DifferentialFidelityReport`: Green/Yellow/Red grade, gap attribution, oracle
  status, blocked provenance, stale evidence, and explicit no-port-claim flags.
- `SemanticPortHandoff`: only for Green outcomes, containing the evidence refs
  M112 must consume for coverage/convergence; Yellow and Red outcomes emit
  re-derivation or reject/defer tasks instead.

Elixir/Phoenix Studio may render these reports and route gated follow-up actions
through the `ouroforge` CLI, but Studio owns no artifact semantics, no trusted
writes, and no direct mutation path.

## Gated path

1. **Preflight provenance**: reject decompiled, ripped, shipped-build,
   foreign-runtime, live-bridge, or opaque binary refs.
2. **Preflight oracle**: require captured/passing oracle evidence and a primary
   deterministic state hash.
3. **Preflight candidate**: require an M110 candidate whose writes are still
   source-apply/review gated and whose `ported_claim_allowed` flag is false.
4. **Run native candidate**: execute or inspect the Ouroforge-native candidate in
   deterministic order; never run the source engine as an embedded bridge.
5. **Compare behavior**: compare expected events, terminal state, and state hash;
   compare render evidence only as secondary corroboration for 2.5D/3D.
6. **Grade and hand off**: Green goes to M112 semantic-port coverage;
   Yellow returns to M109/M110 repair; Red rejects or defers the unit.

## Fidelity and oracle rule

- 🟢 **Green — behaviorally verified candidate**: source-only/open-text
  provenance is clean; oracle evidence is captured/passing; the native candidate
  reproduces required events; 2D state hashes are bit-exact or 2.5D/3D primary
  state hashes match; optional render evidence is within declared tolerance;
  source-apply/review gates remain intact. The report may say the candidate
  passed differential verification, but it still must not claim a finished-game
  auto-port.
- 🟡 **Yellow — inconclusive or needs repair**: evidence is missing, stale, or
  partial; oracle tolerance is under-specified; render corroboration is absent
  for a 2.5D/3D claim; native behavior diverges in a bounded repairable way; or
  the candidate requires another re-expression pass. The output is a task, not a
  pass.
- 🔴 **Red — blocked or failed**: decompiled/ripped/shipped-build provenance,
  runtime bridge dependency, embedded foreign engine, ungated trusted write,
  copied source logic, auto-translation/port claim, state-hash mismatch, or a
  declared Green status contradicted by evidence.

Oracle rule: no unit may be called ported, accepted, equivalent, or complete
without captured oracle evidence and a Green differential verification report.
Even then, public wording must say the native candidate passed the specified
A/B oracle; content import remains best-effort with an honest fidelity report.

## Determinism contract

- 2D verification gates on bit-exact state-hash equality and ordered event
  equality.
- 2.5D/3D verification gates on deterministic state-hash primary evidence.
- SSIM/pixel-diff or similar render evidence is secondary corroboration only and
  cannot turn a failed/missing state hash Green.
- Physics is re-simulated as Ouroforge-native deterministic behavior. The source
  engine physics step is never reproduced by embedding, replaying, or bridging
  the source runtime.
- Report digests must be stable for identical inputs and change when oracle
  hashes, native hashes, expected events, observed events, or fidelity grades
  change.

## Era R hand-off

- Missing/incomplete oracle → M109 interrogation/oracle capture.
- Candidate mismatch with repairable gap → M110 deterministic re-expression
  repair task.
- Green differential verification → M112 semantic-port coverage and convergence.
- Human-facing presentation/review → M113 Studio UX, still two-plane and gated
  through the `ouroforge` CLI.
- Governance status → M114; #1 and #23 remain open anchors.

## Non-goals

- Finished-game auto-porting or any broad claim that a project was ported.
- Live bridge to, or embedded runtime from, Unity, Unreal, Godot, or any other
  foreign/non-deterministic engine.
- Shipped-build ripping, decompiled-code copying, or syntactic translation of
  legacy logic.
- Direct Elixir/Phoenix trusted writes, a new data store, or a new data plane.
- Faithful 1:1 reproduction of source physics, shaders, VFX, fun, feel, or
  release go/no-go decisions.

## Verification

```bash
grep -RIlqi "one-way\|on-ramp\|re-derivation\|fidelity\|two-plane\|source-project" docs/ || true
cargo build --workspace --jobs 2
```

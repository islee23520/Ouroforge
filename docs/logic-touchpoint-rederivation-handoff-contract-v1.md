# Logic Touchpoint Detection and Re-Derivation Hand-off Contract v1

Issue: #2176 — Era O, Milestone 91.

This contract narrows the Era O 2D migration on-ramp after adapter-to-IR and
IR-to-Ouroforge mapping work. It defines how Ouroforge detects logic touchpoints
inside source-project/open-text migration IR, how those touchpoints are graded,
and how they are handed to Era R for clean-room semantic re-derivation. It is
documentation only; it authorizes no new write path, no new data store, no
foreign runtime bridge, and no source-code translation.

Downstream M91 implementation/demo/coverage issues **#2177 and #2178** must cite
this document and preserve its input/output shape, fidelity floors, oracle rule,
state-hash requirement, source-only legal boundary, and two-plane architecture.

## Decision

Ouroforge may inventory logic touchpoints from validated migration IR and emit a
Rust-owned Era R hand-off bundle. The bundle is evidence and planning data: it is
not executable gameplay, not a translated script, and not proof that any source
behavior has been ported. A touchpoint becomes Ouroforge gameplay only after Era
R re-implements it clean-room from observed behavior plus interrogated intent and
passes the declared Ouroforge-native acceptance oracle.

Logic detection is part of the one-way on-ramp. It imports declarative skeleton
facts and re-derives behavior later; it does not absorb Godot/Unity/Unreal, embed
a source engine runtime, scrape shipped builds, copy decompiled source, or auto-
port a finished game.

## Accepted inputs

The detector accepts only Rust-validated migration artifacts with source-project
and open/text provenance:

- adapter IR from `docs/godot-2d-adapter-ir-contract-v1.md` and later equivalent
  source-only adapters;
- mapping artifacts from
  `docs/ir-to-ouroforge-mapping-fidelity-classifier-contract-v1.md`;
- scene/node/resource/input records with stable source ids and source locations;
- script references, signal/connection records, callback names, exported
  variables, animation method-track names, input-action reactions, physics-event
  hooks, and unsupported-engine feature records;
- optional observed behavior traces or human-intent notes as evidence refs only,
  never as trusted writes.

Rejected inputs are recorded as 🔴 boundary failures: shipped-build ripping,
decompiled source, binary-only projects, live source-engine runtime access,
network/package-manager resolution, opaque plugin execution, or Studio/Phoenix
trusted artifact writes.

## Outputs

The detector emits a deterministic, Rust-owned hand-off artifact with these
fields or their direct Rust equivalents:

1. `schema_version` — pinned hand-off schema.
2. `boundary` — one-way on-ramp, source-project/open-text, clean-room,
   oracle-gated, two-plane wording.
3. `source_project` and `source_ir_hash` — source label and canonical IR hash.
4. `touchpoints[]` — one record per logic surface, including source id,
   source provenance, trigger kind, visible declarative parameters, extracted
   non-code labels, fidelity grade, and gap reason.
5. `era_r_tasks[]` — clean-room re-derivation tasks derived from touchpoints.
6. `oracle_requirements[]` — required acceptance evidence before any equivalence
   or ported wording is allowed.
7. `fidelity_report` — 🟢/🟡/🔴 counts, gap summary, unsupported/legal boundary
   rows, and explicit no-auto-port wording.
8. `state_hash` — deterministic hash over the canonical hand-off artifact.
9. `claimed_ported_units` — must remain empty in Era O.

Outputs are displayable in Studio or dashboards, but every write to trusted
Ouroforge artifacts must route through the existing `ouroforge` CLI gates,
source-apply/scene-apply review, stale-hash checks, rollback evidence, and
review decisions.

## Detection subset

| Source surface | Touchpoint output | Grade floor | Hand-off rule |
| --- | --- | --- | --- |
| Script/resource reference attached to a node | `script-ref` task with path/id provenance only | 🔴 | Inventory the reference and visible exported metadata; never copy or translate script code. |
| Godot signal connection / Unity event binding | `event-binding` task with signal/event name and target callback name | 🔴 | Re-derive callback behavior in Era R from observed behavior plus intent. |
| Input action declaration with gameplay reaction | `input-reaction` task linked to declarative input mapping | 🔴 | Input binding may be mapped separately; behavior triggered by input is Era R. |
| Physics callbacks, colliders with behavioral implications, triggers | `physics-behavior` task | 🔴 | Physics is re-simulated in Ouroforge, never reproduced from the source engine. |
| Animation method track, timeline event, scripted VFX/audio cue | `runtime-callback` task | 🔴 | Preserve timing labels as evidence; re-express behavior only after oracle capture. |
| Exported variables, inspector-tuned constants, tags/groups/layers | `behavior-parameter` task | 🟡/🔴 | Declarative values may be preserved as evidence; semantics remain unverified. |
| Unsupported engine-specific feature (shader/plugin/particle/AI/nav/etc.) | `unsupported-feature` task | 🔴 | Report as gap or human-redesign item; do not fake an equivalent. |
| Pure declarative skeleton fact with no behavior | no logic touchpoint | 🟢 only in the adapter/mapping report | M91 does not re-grade skeleton facts as behavior. |
| Decompiled/ripped/binary-only behavior | legal boundary rejection | 🔴 | No candidate hand-off other than rejection/red diagnostic. |

## Fidelity and oracle rules

- **🟢 Green** is allowed only for the detector's own bookkeeping rows, such as a
  stable schema, deterministic hash, or proof that a pure skeleton fact produced
  no logic touchpoint. A behavior-bearing source unit is never Green in Era O.
- **🟡 Yellow** means metadata was preserved but behavior is incomplete or the
  oracle is missing. Yellow is still not ported and cannot be described as
  equivalent.
- **🔴 Red** means clean-room re-derivation, unsupported redesign, legal rejection,
  or source-engine behavior is required. All script/callback/input/physics
  behavior starts Red.

No touchpoint may be described as `ported`, `translated`, `equivalent`,
`complete`, or `verified` until Era R records a passing Ouroforge-native oracle.
For 2D behavior the primary oracle is bit-exact deterministic state-hash evidence
for the accepted scenario. For 2.5D/3D-adjacent behavior, deterministic
state-hash remains primary and perceptual render evidence is secondary only.

## Gated path

1. Adapter parses source-project/open-text artifacts into neutral IR and marks
   scripts, callbacks, unsupported features, and source-only/legal boundaries.
2. Mapping classifier preserves skeleton candidates and emits initial behavioral
   unit records without port claims.
3. M91 detector normalizes logic touchpoints into a deterministic Era R hand-off
   artifact and fidelity report.
4. Studio/Phoenix may render the report and capture human intent, but it owns no
   artifact semantics and performs no trusted writes.
5. Era R re-derives behavior in Ouroforge-native code/data from observed behavior
   plus interrogated intent.
6. Existing evaluator/source-apply/scene-apply gates and oracle evidence decide
   whether later wording may claim equivalence.

## Determinism requirements

- Identical input IR and mapping evidence must produce byte-stable touchpoint
  ordering, hand-off tasks, fidelity reports, and state hashes.
- Any source id, trigger kind, declared parameter, oracle requirement, or gap
  reason drift must change the hand-off state hash.
- Missing, stale, or malformed hashes fail closed. They are not warnings.
- Generated runs and temporary hand-off exports stay untracked unless they are
  explicitly fixture-scoped evidence.

## Two-plane boundary

Rust (`crates/ouroforge-core` and `crates/ouroforge-evaluator`) owns parsing,
normalization, artifact truth, fidelity/oracle rules, deterministic hashing, and
validation. Elixir/Phoenix Studio is a local control/presentation plane only: it
may display hand-off evidence and capture optional human intent, but every write
flows through the `ouroforge` CLI/gates. Studio must not introduce artifact
semantics, raw writes, a new database, or a bypass around review/apply.

## Legal and clean-room boundary

The detector may read only user/operator supplied source-project artifacts in
open/text formats and Rust-owned IR derived from them. It may inventory names and
metadata needed to ask the right clean-room questions, but it must not copy,
translate, summarize, or rephrase decompiled or proprietary source behavior into
Ouroforge logic. Shipped-build ripping and binary-only extraction are rejected.

## Non-goals

- No auto-port of a finished game.
- No source-code translation or decompiled-source copying.
- No live Godot/Unity/Unreal bridge and no embedded source-engine runtime.
- No faithful reproduction of source-engine physics, shader, AI, plugin, or VFX
  behavior.
- No new data plane, no Studio trusted write path, and no direct artifact writes
  from Elixir/Phoenix.
- No automated fun/feel judgment or release go/no-go.

## Downstream reference checklist

Issues #2177 and #2178 must verify that their implementation/demo/coverage work:

- cites this contract and the M88/M89/M90 contracts;
- keeps every behavior-bearing unit Yellow/Red until oracle evidence passes;
- leaves `claimed_ported_units` empty for Era O artifacts;
- records Era R hand-off tasks rather than translated logic;
- rejects decompiled/shipped-build/binary-only behavior inputs;
- preserves deterministic state-hash validation;
- keeps Rust as data plane and Studio/Phoenix as presentation/control only;
- confirms #1 and #23 remain open governance anchors.

## Verification anchors

A reviewer can validate this documentation-only milestone with:

```bash
grep -RIlqi "one-way\|on-ramp\|re-derivation\|fidelity\|two-plane\|source-project" docs/ || true
cargo build --workspace --jobs 2
```

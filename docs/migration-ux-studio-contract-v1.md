# Migration UX (Studio) Contract v1

Issue: #2186. Milestone: Era O M94. Downstream implementation/demo/coverage issues: #2187, #2188, #2189.

This ADR fixes the Migration UX contract before Studio implementation. Studio is a control and presentation surface for the 2D migration on-ramp: it shows Rust-owned import evidence, routes user intent through existing gated commands, and exposes fix-forward links for clean-room re-derivation. It is not a data plane, not a new artifact writer, not a live bridge to Godot or Unity, and not a finished-game auto-port surface.

## Decision

Ouroforge Studio may offer a local import wizard, fidelity report view, and fix-forward routing for Era O source projects, but only as a two-plane workflow:

- **Rust data plane:** `crates/ouroforge-core`, `crates/ouroforge-evaluator`, and the `ouroforge` CLI own source-project validation, adapter IR, mapping, fidelity classification, oracle requirements, deterministic state hashes, evidence bundles, and gated writes.
- **Phoenix/LiveView control + presentation plane:** `studio/` renders Rust-owned evidence and invokes existing CLI/gate paths for writes. It owns no artifact semantics, has no trusted write path, and adds no separate artifact data store.

The UX must preserve the permanent Era O boundary: one-way source-project import of a declarative skeleton plus clean-room re-derivation of behavior. A user may be guided to re-author or re-derive gaps, but Studio must not claim behavior is ported unless later oracle evidence passes.

## Importable subset and boundary

Studio may accept only source-project roots that the Rust adapters already accept. The UI does not broaden the adapter contracts.

| Source family | Accepted inputs shown in Studio | Rejected / out of scope |
| --- | --- | --- |
| Godot 2D | Source project text resources: `.tscn`, `.tres`, `project.godot`, and referenced project-local assets accepted by the Godot adapter contracts. | Shipped exports, packed projects, binary-only resources, engine runtime embedding, decompiled scripts, hidden extraction from builds. |
| Unity 2D | Force-Text `.unity`, `.prefab`, `.asset`/project settings, `.meta` GUID sidecars, and referenced source-project media accepted by the Unity adapter contracts. | Player builds, AssetBundles, Addressables bundles, `Library/`, IL2CPP/Mono dumps, binary scenes/prefabs, decompiled source. |
| Common skeleton | Scene hierarchy, transforms, sprites, tilemaps, cameras, declarative input bindings, asset references, provenance, and supported presentation metadata. | Runtime logic, physics solver equivalence, custom shaders/VFX equivalence, script translation, fun/feel/release decisions. |

A source item outside the accepted subset is not a UX failure. It is a fidelity gap, an unsupported item, or an Era R re-derivation task. Studio must show the gap honestly instead of hiding it behind a successful import banner.

## Exact inputs

The Studio workflow may collect these inputs and pass them to the Rust-owned CLI/gates:

1. local source-project path;
2. declared source engine family (`godot-2d` or `unity-2d`) or an explicit adapter selection produced by Rust preflight;
3. optional operator-supplied project label for evidence display;
4. optional license/provenance notes for user-owned assets;
5. optional destination workspace/artifact id accepted by the existing gated write path;
6. optional user intent notes for 🔴 logic gaps, stored only as re-derivation context and not as translated code.

Studio must not accept shipped builds, runtime binaries, decompiled source, engine-editor sessions, or live engine connections as inputs for this milestone.

## Exact outputs

The Rust-owned workflow produces the artifacts; Studio displays them and links actions to existing gates.

| Output | Owner | UX responsibility |
| --- | --- | --- |
| Source preflight verdict | Rust CLI/evaluator | Show accepted/rejected files and source-only/legal reasons. |
| Adapter IR summary | Rust core | Show imported scenes/assets/inputs/touchpoints with provenance and hashes. |
| Mapping proposal / native artifact draft | Rust core + gated apply path | Show only after validation; any write must go through the existing `ouroforge` CLI/gates. |
| Fidelity report | Rust core/evaluator | Render 🟢/🟡/🔴 rows, rationale, evidence links, and missing oracle requirements without upgrading grades in UI. |
| Oracle requirements | Rust evaluator | Show that `ported_claim_allowed=false` until later passing evidence exists. |
| Era R hand-off queue | Rust evidence + existing issue/task routing | Link 🔴 logic touchpoints to clean-room re-derivation work; do not translate scripts. |
| Deterministic hashes | Rust core/evaluator | Display source/IR/fidelity/state hashes and fail/blocked status when stale or missing. |

No Studio component may persist a competing semantic copy of these outputs. Any local UI cache is disposable presentation state and cannot be treated as artifact truth.

## Gated path

The M94 user path is fixed as follows:

1. **Select source:** operator chooses a local source-project root and source family.
2. **Preflight:** Studio invokes Rust preflight through the `ouroforge` CLI. Rejected paths stop with an explicit source-only/legal verdict.
3. **Parse/import skeleton:** Rust adapter emits deterministic IR and provenance for the declarative skeleton.
4. **Classify fidelity:** Rust assigns 🟢/🟡/🔴 grades and oracle requirements for every imported, partial, unsupported, or behavior-bearing unit.
5. **Validate gates:** evaluator checks legal boundary, provenance, no claimed ported units, deterministic hashes, and required Era R hand-offs.
6. **Render report:** Studio displays the Rust-owned report, including gaps and missing oracles.
7. **Fix-forward:** Studio may route a gated write or create/link a re-derivation task only through existing CLI/evidence paths. Elixir/Phoenix performs no trusted artifact mutation.

A path that bypasses any Rust gate, silently mutates artifacts from Phoenix, or creates a new data store is out of contract.

## Fidelity grading for UX

| Grade | UX label | Meaning | Allowed action | Port claim? |
| --- | --- | --- | --- | --- |
| 🟢 | Clean skeleton import | Declarative source facts mapped to Ouroforge-native IR with complete provenance and deterministic representation. | Show as imported skeleton evidence; allow gated apply when all validators pass. | No. Green is not a behavior-port claim. |
| 🟡 | Needs review / best-effort | Importable with caveats: partial metadata, renderer normalization, missing optional provenance, physics declaration without solver equivalence, or other known gap. | Show caveat, affected source, consequence, and recommended fix-forward. Human may accept skeleton loss if gates permit. | No. Report the gap. |
| 🔴 | Re-derive / unsupported | Behavior-bearing, legally unsafe, nondeterministic, unsupported, or not representable as declarative skeleton. | Block any port/equivalence claim; create/link Era R clean-room re-derivation item or fail closed. | No. Requires later oracle evidence. |

Studio must not let UI copy, color, progress bars, or success banners imply that a Yellow or Red unit is ported. A completed import means only that the skeleton/evidence workflow completed honestly.

## Oracle rule

Nothing in the Migration UX may use `ported`, `equivalent`, `migrated logic`, or `done` for behavior unless a later Era R artifact provides all required evidence:

1. captured source-observed behavior and/or interrogated human intent;
2. Ouroforge-native re-expression created clean-room without copying or translating decompiled/source script code;
3. passing deterministic verification: 2D bit-exact state hash; 2.5D/3D deterministic state-hash primary plus perceptual SSIM/pixel-diff render secondary;
4. a linked differential verdict that ties the oracle, native behavior, state hash, and fidelity report together.

For Era O/M94, all behavior touchpoints remain oracle-missing unless prior Rust evidence explicitly says otherwise. The UX must show missing evidence as missing evidence, not as an implementation backlog already satisfied.

## Era R hand-off

Every 🔴 logic touchpoint shown by Studio must route to Era R as a clean-room task with:

- source engine family and source-project provenance;
- scene/node/component/asset reference and stable IR id;
- trigger kind such as script reference, signal/event, animation callback, input reaction, physics callback, or exported variable;
- observed-behavior/oracle evidence status;
- required deterministic verification kind;
- fidelity report row id and current `ported_claim_allowed=false` status.

Era O imports nouns and structure. Era R re-derives verbs. Studio may help the human navigate that hand-off, but it does not translate source logic or decide fun/feel/release readiness.

## Downstream obligations

- #2187 must implement the Studio import wizard against this contract: local UX only, Rust-owned gates, no trusted Elixir writes, no new data store.
- #2188 must implement or demonstrate the fidelity report/fix-forward surface with honest 🟢/🟡/🔴 rows and Era R links.
- #2189 must lock the UX with Scenario Coverage v82 regressions: no auto-port claim without oracle, no shipped-build input, no Phoenix trusted write, and deterministic-hash evidence displayed.
- Downstream work must cite the prior Rust contracts: [`2d-migration-on-ramp-scope-ir-legal-v1.md`](2d-migration-on-ramp-scope-ir-legal-v1.md), [`godot-2d-adapter-ir-contract-v1.md`](godot-2d-adapter-ir-contract-v1.md), and [`unity-2d-adapter-ir-contract-v1.md`](unity-2d-adapter-ir-contract-v1.md).

## Non-goals

- No code implementation in #2186.
- No hosted/multi-user collaborative Studio, new queue service, new database, or new trusted writer.
- No live bridge to Godot/Unity, no embedded engine runtime, no editor automation requirement, and no shipped-build ripping.
- No source-script translation, decompiled-code copying, physics/shader/VFX equivalence claim, fun/feel automation, or release go/no-go automation.

## Verification

```bash
set -euo pipefail
grep -RIlqi "one-way\|on-ramp\|re-derivation\|fidelity\|two-plane\|source-project" docs/ || true
cargo build --workspace --jobs 2
```

#1 and #23 must remain open after the roadmap update and PR merge.

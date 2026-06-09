# Unity 2D Adapter to IR Contract v1

Issue: #2182. Milestone: Era O M93. Downstream implementation/demo/coverage issues: #2183, #2184, #2185.

This contract fixes the bounded Unity 2D adapter-to-IR scope before implementation. It is an engine-migration on-ramp contract: import a Unity project's declarative skeleton into Ouroforge-owned deterministic IR, then hand behavior-bearing logic to Era R for clean-room re-derivation. It is not engine absorption, not a live bridge, and not finished-game auto-porting.

## One-way / source-only / clean-room boundary

- **One-way on-ramp:** Unity project text is read and normalized into Ouroforge-native IR/evidence. Ouroforge never embeds, launches, links, or bridges to a Unity runtime.
- **Source-project/open-text only:** accepted inputs are Unity **Force Text** YAML project files plus `.meta` sidecars from a source project. The adapter must reject shipped builds, AssetBundles, Addressables bundles, IL2CPP/Mono assemblies, player data, binary scenes, package caches treated as binary engine payloads, and decompiled source.
- **Clean-room re-derivation:** `MonoBehaviour`, animation callbacks, physics callbacks, input polling, coroutines, C# method names, and serialized script refs may be inventoried as behavioral touchpoints, but script source is not copied or translated. Behavior is re-implemented later from observed behavior plus interrogated intent and captured oracle evidence.
- **Two-plane ownership:** Rust owns parsing, IR, provenance, fidelity classification, oracle requirements, and deterministic state hashes. Studio/Phoenix may later render evidence or route gated write proposals, but does not own artifact semantics and performs no trusted writes.
- **Human Ring 2:** fun/feel and release go/no-go remain human decisions; this contract does not automate subjective acceptance or shipping.

## Accepted input subset

| Input | Accepted subset | Notes |
| --- | --- | --- |
| `ProjectSettings/ProjectSettings.asset` | YAML fields needed for 2D orientation, physics timestep hints, sorting, and input backend metadata. | Records configuration provenance only; no Unity runtime dependency. |
| `ProjectSettings/InputManager.asset` | Legacy Input Manager axes/buttons as declarative input names and bindings. | New Input System assets are 🔴 unless represented as source-text `.inputactions` with a future contract. |
| `Assets/**/*.unity` | Force-Text scene YAML with 2D GameObjects, Transforms/RectTransforms, Sprites, SpriteRenderers, Cameras, Canvas/UI text/image shape, Tilemap/Grid references, Rigidbody2D, Collider2D, Animator refs, AudioSource refs, and serialized component refs. | Scene graph and presentation skeleton only. Behavior refs become Era R touchpoints. |
| `Assets/**/*.prefab` | Force-Text prefab YAML for reusable 2D skeleton entities/components. | Variants and nested prefabs are accepted as references when resolvable through GUID/fileID. |
| `Assets/**/*.asset` | Text assets for SpriteAtlas metadata, Tile palettes, AnimatorControllers, ScriptableObjects containing declarative constants. | ScriptableObject fields are data only; behavior implied by custom scripts is 🔴. |
| `Assets/**/*.meta` | GUID, importer, texture/sprite slicing, pixels-per-unit, labels, asset bundle names as provenance. | Missing `.meta` sidecars make referenced assets 🟡 or 🔴 depending on whether GUID resolution is required. |
| `Assets/**/*.{png,jpg,jpeg,webp,wav,ogg,mp3}` | Referenced media files by path/hash/provenance only. | Binary media may be referenced and hashed; no shipped-build ripping or hidden extraction. |

Rejected inputs fail closed with an explicit fidelity/provenance error: binary `.unity`/`.prefab`, `Library/`, `Temp/`, `Obj/`, `Build/`, `Builds/`, `*.apk`, `*.ipa`, `*.exe`, `*.app`, `*.dll`, `globalgamemanagers`, `resources.assets`, AssetBundles, IL2CPP dumps, Mono decompilation output, and any file whose provenance is a shipped build rather than a source project.

## Output IR contract

The adapter emits a deterministic Unity migration IR owned by `crates/ouroforge-core`. The exact Rust type names are left to #2183, but the schema must contain these logical records:

| IR record | Required fields | Purpose |
| --- | --- | --- |
| `UnityProjectSource` | project id, Unity version if declared, accepted formats, source file list, rejected file list, canonical source hash. | Source-only provenance and deterministic replay. |
| `UnitySceneNode` | scene path/GUID, fileID, object name, parent/children, active flag, layer/tag, transform, provenance span. | Declarative skeleton graph. |
| `UnityComponentRecord` | owner fileID, component type, serialized fields, asset refs, support status, fidelity grade, provenance span. | Component inventory without runtime semantics. |
| `UnityAssetRecord` | GUID, path, importer metadata, media hash, license/provenance status, accepted/rejected status. | Asset provenance and best-effort import. |
| `UnityInputRecord` | action/axis/button name, bindings, source backend, support status. | Declarative input mapping candidate. |
| `UnityBehaviorTouchpoint` | source object/component ref, trigger kind, coupling kind, script GUID/fileID/name if source-visible, Era R task ref, oracle requirement ref. | Clean-room hand-off for logic. |
| `UnityFidelityRecord` | source ref, target IR ref, grade, rationale, required evidence, gap kind. | Honest import classification. |
| `UnityOracleRequirement` | unit id, source ref, status, `ported_claim_allowed=false`, required evidence. | No-port-claim gate. |

Every output artifact must carry a canonical `sha256:` state hash. Identical source text and media-hash inputs produce identical IR and fidelity hashes. Source drift, missing `.meta` resolution, or tampered IR changes the hash or fails validation.

## Fidelity grading

| Grade | Meaning | Examples | Port claim? |
| --- | --- | --- | --- |
| 🟢 Green / clean | Declarative skeleton information maps directly to Ouroforge IR with complete source provenance and deterministic representation. | GameObject hierarchy, Transform/RectTransform positions, SpriteRenderer sprite ref/color/sorting layer, Camera orthographic settings, simple Canvas/Text/Image shape, asset GUID/path/hash, legacy input axis/button records. | No. Green means skeleton data is cleanly imported, not behavior ported. |
| 🟡 Yellow / flagged | Importable with known caveats, partial semantics, missing optional metadata, or best-effort normalization. | Sprite slicing with incomplete importer fields, nested prefab variant overrides, AnimatorController states without behavior equivalence, Tilemap palette metadata requiring later renderer normalization, Rigidbody2D/Collider2D shape mapped without Unity physics equivalence, missing optional `.meta` labels. | No. Report caveat and evidence gap. |
| 🔴 Red / re-derive | Behavior-bearing, unsupported, legally unsafe, nondeterministic, or cannot be faithfully represented as declarative skeleton. Must become an Era R task or fail closed. | MonoBehaviour logic, C# method/coroutine names beyond inventory, animation events invoking code, physics/contact callbacks, custom shaders/VFX, Timeline/Cinemachine logic, New Input System behavior maps not covered by text subset, shipped-build or decompiled sources. | No. Red requires clean-room re-derivation and captured oracle evidence before any later port/equivalence claim. |

A lossy import must not be graded clean. A report with Red re-derivation gaps and no Era R task is invalid. A report that silently drops Yellow/Red gaps is invalid.

## Oracle rule and Era R hand-off

No unit is `ported`, `equivalent`, `migrated logic`, or `done` until all of the following are present in a later Era R artifact:

1. captured acceptance oracle evidence from source-observed behavior and/or interrogated human intent;
2. an Ouroforge-native re-expression that does not copy or translate decompiled/source script code;
3. deterministic verification: for 2D, bit-exact state hash; for 2.5D/3D, deterministic state-hash primary with perceptual render secondary;
4. a passing differential verdict linking the oracle, native behavior, state hash, and fidelity report.

The Unity adapter may create `UnityBehaviorTouchpoint` and `UnityOracleRequirement` records, but `claimed_ported_units` must remain empty and every oracle starts as `status=missing`, `ported_claim_allowed=false`.

## Gated path

1. **Preflight:** reject non-source-project/binary/shipped-build inputs; verify Force-Text YAML and `.meta` sidecars where required.
2. **Parse:** Rust parses source text deterministically into Unity IR records with provenance spans.
3. **Classify:** assign 🟢/🟡/🔴 fidelity grades and gap rationale for every imported or rejected unit.
4. **Hash:** compute canonical source/IR/fidelity state hashes.
5. **Validate:** fail closed on unsupported schema, missing provenance, Red-without-Era-R-task, claimed ported units, oracle bypass, stale hashes, or Studio/Elixir artifact-semantics drift.
6. **Report:** write Rust-owned evidence only through the `ouroforge` CLI path used by downstream gates; no Studio trusted write and no new data store.

## Determinism requirements

- YAML traversal order, GUID/fileID resolution, component ordering, and fidelity rows must be canonicalized.
- Hashes include source text, relevant binary media hashes, `.meta` GUID/importer records, parser version, and fidelity records.
- 2D import verification gates on deterministic state hash. Any future render smoke is corroborating evidence only and must not override state-hash failure.
- Unity physics is re-simulated in Ouroforge later; this adapter records Rigidbody2D/Collider2D declarations but never claims Unity physics reproduction.

## Downstream obligations

- #2183 must implement only this bounded parser/IR contract in Rust and reject out-of-contract inputs.
- #2184 must add fixtures/examples that exercise 🟢/🟡/🔴 rows, `.meta` GUID resolution, no-port oracle records, and deterministic hash behavior.
- #2185 must lock the contract with coverage and negative regressions: lossy import not clean, ungated/auto-translated port fails, determinism break fails, and no decompiled source is copied.
- Any Studio/Phoenix display added later must render Rust-owned evidence and route writes through existing CLI/gates; it must not mutate artifacts or define import semantics.

## Non-goals

- No Unity runtime bridge, editor automation, package import, AssetBundle reader, IL2CPP/Mono ripping, decompiled source ingestion, or build artifact extraction.
- No finished-game auto-porting, source-script translation, physics/shader/VFX equivalence, fun/feel automation, or release go/no-go automation.
- No new data plane, persistent store, or trusted write path.

## Verification

```bash
grep -RIlqi "one-way\|on-ramp\|re-derivation\|fidelity\|two-plane\|source-project" docs/ || true
cargo build --workspace --jobs 2
```

#1 and #23 remain open.

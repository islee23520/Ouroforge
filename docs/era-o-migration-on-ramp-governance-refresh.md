# Era O Migration On-Ramp Governance Refresh

Issue: #2190. Era O, Milestone 95.

Era O is recorded as complete on merged evidence for Milestones 88-94. The era
adds a bounded External-Engine 2D Migration On-Ramp for Godot and Unity source
projects: Ouroforge can import a declarative skeleton, map it into native
candidate artifacts, report fidelity honestly, surface clean-room logic
re-derivation hand-offs, verify import evidence, and show the workflow in local
Studio. It does **not** auto-port a finished game.

## Merged evidence chain

- **M88 scope / IR / legal gate** — #2167, ADR
  `docs/2d-migration-on-ramp-scope-ir-legal-v1.md`.
- **M89 Godot adapter to IR** — #2168-#2171, including
  `docs/godot-2d-adapter-ir-contract-v1.md`, the Rust Godot source-text adapter,
  demo evidence, and Scenario Coverage v78.
- **M90 IR mapping and fidelity classifier** — #2172-#2175, including
  `docs/ir-to-ouroforge-mapping-fidelity-classifier-contract-v1.md`, Rust
  IR-to-native candidate mapping, demo evidence, and Scenario Coverage v79.
- **M91 logic touchpoint detection and Era R hand-off** — #2176-#2178, including
  `docs/logic-touchpoint-rederivation-handoff-contract-v1.md`, Rust touchpoint
  extraction, clean-room re-derivation tasks, and Scenario Coverage v80.
- **M92 import verification and fidelity report** — #2179-#2181, including
  `docs/import-verification-fidelity-report-v1.md`, demo evidence, and Scenario
  Coverage v81.
- **M93 Unity adapter to IR** — #2182-#2185, including
  `docs/unity-2d-adapter-ir-contract-v1.md`, the Unity Force-Text source adapter,
  demo evidence, and the Unity Scenario Coverage v81 suite.
- **M94 Migration UX in Studio** — #2186-#2189, including
  `docs/migration-ux-studio-contract-v1.md`, the local Studio control/presentation
  model, scripted demo, and Scenario Coverage v82.
- **M95 governance refresh** — #2190, this document and the #1 roadmap refresh.

## Permanent boundaries reaffirmed

- **One-way import only.** Era O imports source-project/open-text declarative
  skeleton facts into Ouroforge-native evidence. It does not create a live
  bridge, embed a Unity/Godot runtime, or absorb another engine.
- **Re-derivation, not translation.** Behavior-bearing source units become Era R
  clean-room re-derivation tasks backed by observed behavior and interrogated
  intent. Decompiled source is never copied, translated, or treated as input.
- **Oracle-gated claims.** No imported unit is called `ported`, behaviorally
  equivalent, complete, or done without captured passing oracle evidence. Missing
  logic remains Yellow/Red with explicit gaps.
- **Honest fidelity reports.** Green is limited to clean declarative skeleton
  import. Lossy content, behavior gaps, unsupported assets, or missing oracle
  evidence cannot be laundered into a clean result.
- **Determinism remains primary.** 2D gates on bit-exact state hashes. 2.5D/3D
  migration remains future-scoped and must use deterministic state-hash primary
  evidence with perceptual render comparison only as secondary corroboration.
  Physics is re-simulated, never reproduced.
- **Two-plane ownership.** Rust owns adapters, IR, mapping, extraction,
  validation, deterministic state hashes, fidelity reports, evidence, and gated
  writes. Elixir/Phoenix Studio is local control + presentation only and routes
  write-affecting actions through existing `ouroforge` CLI/gates.
- **Legal input boundary.** Accepted inputs are source projects in open/text
  formats, such as Godot `.tscn`/`.tres` and Unity Force-Text YAML plus `.meta`.
  Shipped-build ripping, binary extraction, player-data scraping, and decompiled
  code copying are out of scope.
- **Human authority remains.** Fun/feel, creative acceptance, and release
  go/no-go remain human Ring 2 decisions. Era O does not make a production-ready,
  Godot-replacement, compatibility, or auto-shipping claim.

## Completion statement

Era O completes the 2D migration on-ramp foundation as a conservative,
evidence-native intake path. It gives downstream Era R/P/Q work structured
skeleton evidence, fidelity gaps, deterministic hashes, and re-derivation tasks;
it does not finish or port the source game. #1 and #23 remain open governance
anchors.

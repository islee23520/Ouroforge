# Era P 2.5D Migration On-Ramp Governance Refresh

Issue: #2203. Era P, Milestone 100.

Era P is recorded as complete on merged evidence for Milestones 96-99. The era
adds a bounded 2.5D migration on-ramp for source-project/open-text presentation
facts: Ouroforge can normalize glTF geometry, orthographic cameras, billboards,
sprite stacks, and 2D-in-3D presentation metadata into deterministic
Ouroforge-native evidence, verify authoritative state with deterministic hashes,
and corroborate presentation with perceptual render evidence. It does **not**
auto-port a finished game.

## Merged evidence chain

- **M96 parent 2.5D scope / gate** — #2191, ADR
  `docs/25d-migration-on-ramp-scope-gate-v1.md`.
- **M97 glTF geometry and orthographic camera import** — #2192-#2195, including
  `docs/gltf-25d-geometry-ortho-camera-contract-v1.md`, the Rust glTF 2.5D
  import data-plane implementation, demo evidence, and Scenario Coverage v83.
- **M98 billboard, sprite-stack, and 2D-in-3D presentation import** —
  #2196-#2199, including
  `docs/billboard-sprite-stack-presentation-contract-v1.md`, Rust presentation
  normalization, demo evidence, and Scenario Coverage v84.
- **M99 2.5D verification** — #2200-#2202, including
  `docs/25d-import-verification-state-perceptual-v1.md`, Rust-owned state-hash
  primary verification, perceptual render secondary reporting, demo evidence,
  and Scenario Coverage v85.
- **M100 governance refresh** — #2203, this document, the roadmap refresh, and
  the #1 completion comment.

## Permanent boundaries reaffirmed

- **One-way import only.** Era P imports source-project/open-text declarative
  presentation facts into Ouroforge-native evidence. It does not create a live
  bridge, embed a Unity/Godot/Unreal runtime, absorb another engine, or authorize
  a finished-game auto-port.
- **Presentation evidence, not behavior authority.** glTF meshes, orthographic
  cameras, billboards, sprite stacks, 2D-in-3D planes, pivots, texture refs,
  alpha/sort metadata, and pixel-art filtering can be normalized as deterministic
  presentation evidence. Gameplay logic, state mutation, animation events,
  physics authority, shader/VFX behavior, source-engine scripts, tacit feel, and
  release judgment remain outside the import path.
- **Re-derivation, not translation.** Behavior-bearing units become clean-room
  Era R re-derivation tasks from observed behavior plus interrogated intent.
  Decompiled source is never copied, translated, or treated as input.
- **Oracle-gated claims.** No unit is called `ported`, behaviorally equivalent,
  complete, or done without captured passing oracle evidence. Missing or lossy
  acceptance evidence stays Yellow/Red with explicit attribution.
- **Honest fidelity reports.** Green is limited to clean declarative presentation
  import with matching provenance and deterministic evidence. Unsupported
  content, shader/VFX gaps, behavior gaps, or absent oracle evidence cannot be
  laundered into a clean result.
- **Determinism remains primary.** 2D still gates on bit-exact state hashes.
  2.5D/3D gates on deterministic state-hash primary evidence; perceptual render
  comparison, including SSIM/pixel-diff, is secondary corroboration only.
  Imported physics is re-simulated, never reproduced.
- **Two-plane ownership.** Rust owns adapters, IR, mapping, extraction,
  normalization, validation, deterministic state hashes, fidelity reports,
  evidence, and gated writes. Elixir/Phoenix Studio is local control +
  presentation only and routes write-affecting actions through existing
  `ouroforge` CLI/gates.
- **No new trusted write path or data store.** This era does not authorize trusted
  writes from Studio, a browser command bridge, a live runtime bridge, or any new
  persistent artifact store.
- **Legal input boundary.** Accepted inputs are source projects in open/text
  formats, such as glTF assets, Godot `.tscn`/`.tres`, and Unity Force-Text YAML
  plus `.meta`. Shipped-build ripping, binary extraction, player-data scraping,
  and decompiled-code copying are out of scope.
- **Human authority remains.** Fun/feel, creative acceptance, and release
  go/no-go remain human Ring 2 decisions. Era P does not make a production-ready,
  compatibility, engine-replacement, or auto-shipping claim.

## #1 completion comment text

Era P (2.5D Migration On-Ramp) is complete on merged evidence for M96-M99 plus
M100 governance: #2191-#2203 / PR #2319-#2329 plus this governance PR. The
completion is limited to one-way source-project/open-text presentation import,
clean-room re-derivation hand-offs for behavior, deterministic state-hash
primary verification, perceptual render secondary corroboration, honest fidelity
reports, and Rust-owned gated evidence. It does not authorize finished-game
auto-porting, live engine bridges, embedded foreign runtimes, decompiled-code
copying, trusted Studio writes, a new data store, or release/fun/feel automation.
#1 and #23 remain open governance anchors.

## Completion statement

Era P completes the bounded 2.5D presentation on-ramp as a conservative,
evidence-native intake path. It gives downstream Era Q/R work normalized
presentation evidence, deterministic hashes, perceptual corroboration, fidelity
gaps, and re-derivation tasks; it does not finish or port the source game. Full
3D remains governed by the M101 GO/DEFER decision gate and defaults to DEFER.
#1 and #23 remain open governance anchors.

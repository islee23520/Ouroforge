# Era R Semantic Re-Derivation Governance Refresh

Era R — Interrogated Semantic Re-Derivation (legacy logic → verified
deterministic behavior) — is recorded complete on merged evidence for Milestones
107-113 plus this M114 governance refresh.

## Completed evidence chain

| Milestone | Evidence |
| --- | --- |
| M107 methodology/gate ADR | #2220 / PR #2247; `docs/semantic-re-derivation-methodology-adr-v1.md` |
| M108 legacy ingestion + behavioral-unit extraction | #2221 / PR #2250, #2222 / PR #2254, #2223 / PR #2257; Scenario Coverage v90 |
| M109 interrogation + oracle capture | #2224 / PR #2259, #2225 / PR #2263, #2226 / PR #2266, #2227 / PR #2272; Scenario Coverage v91 |
| M110 deterministic re-expression | #2228 / PR #2294, #2229 / PR #2296, #2230 / PR #2298, #2231 / PR #2300; Scenario Coverage v92 |
| M111 differential verification A/B | #2232 / PR #2301, #2233 / PR #2304, #2234 / PR #2306; Scenario Coverage v93 |
| M112 semantic-port coverage + convergence | #2235 / PR #2310, #2236 / PR #2311; Scenario Coverage v94 |
| M113 re-derivation UX + human intent/feel escalation | #2237 / PR #2313, #2238 / PR #2315, #2239 / PR #2316, #2240 / PR #2317; Scenario Coverage v95 |
| M114 governance refresh | #2241; this document and the #1 completion comment |

## Permanent boundaries reaffirmed

- **One-way on-ramp only.** Era R imports declarative skeleton context and
  re-derives behavior into Ouroforge-native artifacts. It is not engine
  absorption and not finished-game auto-porting.
- **Source-project/open-text only.** Godot `.tscn`/`.tres`, Unity Force-Text YAML
  + `.meta`, normalized glTF, and other open/text source-project refs are valid
  inputs. Decompiled source, shipped-build ripping, AssetBundle/globalgamemanager
  extraction, and runtime dumping are out of scope.
- **Re-derivation, not translation.** Logic is re-implemented clean-room from
  observed behavior and interrogated intent; decompiled source is never copied or
  translated.
- **Oracle-gated claims.** No unit is called `ported`, and no game is called
  `fully ported`, without captured acceptance evidence and passing gates. Content
  imports remain best-effort with an honest fidelity report.
- **Determinism first.** 2D uses bit-exact state hashes. 2.5D/3D uses
  deterministic state-hash primary evidence; perceptual SSIM/pixel-diff render
  evidence is secondary corroboration only. Physics is re-simulated, never
  reproduced from a foreign runtime.
- **Two-plane architecture.** Rust (`crates/ouroforge-core` and
  `crates/ouroforge-evaluator`) owns artifact truth, validation, fidelity,
  deterministic evidence, and gates. Studio/Phoenix/Elixir is local control +
  presentation only: read + gated-write through the `ouroforge` CLI/review gates,
  no trusted writes, no artifact semantics, and no new data store.
- **Human Ring 2 remains human.** Intent, feel, fun/taste, and release go/no-go
  decisions can be surfaced and recorded as evidence, but automation does not
  decide them.
- **Governance anchors remain open.** #1 and #23 stay open.

## Port-tractability assessment

Era R makes port tractability evidence-based rather than promise-based:

| Area | Tractability | Boundary |
| --- | --- | --- |
| Declarative skeleton import | High for bounded source-project/open-text inputs inherited from Era O. | Best-effort fidelity report; no behavior claim. |
| Simple deterministic behavior units | High when behavior can be observed, interrogated, oracle-captured, re-expressed, and A/B verified. | Green means verified re-derived behavior evidence, not whole-game port. |
| Ambiguous tacit behavior | Medium when humans can answer intent questions. | Yellow until oracle and A/B evidence pass. |
| Feel/taste/release decisions | Human-owned. | Ring 2 escalation; automation records evidence only. |
| Engine-specific physics/shaders/VFX | Low/DEFER by default. | Re-simulate or approximate natively; never reproduce foreign runtime internals. |
| Shipped binaries/decompiled source | Not tractable / forbidden. | Reject or defer; no copying, no ripping, no translation. |

## Roadmap result

Era R is complete on merged evidence through Scenario Coverage v95 and this M114
refresh. Future O/P/Q migration work must continue to use Era R as the clean-room
semantic re-derivation path and must not widen it into live bridging, runtime
embedding, decompiled-code translation, or finished-game auto-porting.

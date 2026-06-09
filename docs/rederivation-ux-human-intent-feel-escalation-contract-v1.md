# Re-Derivation UX and Human Intent/Feel Escalation Contract v1

Era R M113 defines the Studio-facing UX contract for semantic re-derivation
review and Ring 2 human intent/feel escalation. It does not create a new data
plane, write path, engine bridge, or finished-game auto-port surface.

## Goal and scope

This contract fixes the bounded subset for re-derivation UX after M112 semantic
coverage:

- render Rust-owned re-derivation evidence, fidelity gaps, oracle status,
  deterministic state-hash status, and residual backlog;
- capture human intent/feel notes as review evidence routed through existing
  `ouroforge` CLI / source-apply / review gates;
- distinguish 🟢 verified units, 🟡 re-derivation work, and 🔴 blocked or human
  escalation work without claiming any unit is ported unless oracle-gated
  evidence passes;
- hand logic units back to Era R data-plane milestones for ingestion,
  interrogation, oracle capture, deterministic re-expression, differential A/B,
  and semantic-port coverage.

## Non-goals

- No auto-porting a finished game and no “fully ported” product claim.
- No live bridge to Unity, Unreal, Godot, or any foreign engine runtime.
- No shipped-build ripping, decompiled source copying, source translation, or
  faithful reproduction of source physics/shaders/VFX.
- No Studio trusted writes, command bridge, new persistent store, hosted
  collaboration, or browser-owned artifact semantics.
- No automation of fun/feel judgment, release go/no-go, or Ring 2 human taste.

## Inputs

All inputs are read-only evidence refs or gated human-review drafts:

| Input | Owner | Rules |
| --- | --- | --- |
| Source skeleton refs | Rust data plane | Source-project/open-text only: Godot `.tscn`/`.tres`, Unity Force-Text YAML + `.meta`, glTF normalized on import. |
| Behavioral-unit refs | Rust data plane | Extracted clean-room units from M108; decompiled, ripped, shipped-build, foreign-runtime, and live-bridge refs fail closed. |
| Captured oracle refs | Rust data plane | M109 oracle evidence; missing oracle means no port claim. |
| Re-expression and A/B evidence | Rust data plane | M110/M111 deterministic state hashes, render corroboration, rollback refs, and fidelity reports. |
| Semantic-port coverage report | Rust data plane | M112 coverage/convergence ledger, residual backlog, Ring 2 escalation status. |
| Human intent/feel notes | Studio control/presentation | Draft review evidence only; every write must route through `ouroforge` CLI / review-apply gates. |

## Outputs

| Output | Owner | Rules |
| --- | --- | --- |
| UX evidence view model | Studio presentation | Read-only rendering of Rust-owned evidence; no artifact semantics. |
| Escalation draft | Studio control | Human-authored note/request, never a trusted artifact mutation. |
| Gated review transaction ref | Existing gates | Any accepted human note is applied through the CLI/review path with provenance and rollback. |
| Era R hand-off | Rust data plane | Logic units return to `interrogate`, `oracle_capture`, `reexpress`, `verify`, `semantic_coverage`, or `reject_or_defer`. |
| Fidelity report update | Rust data plane | Honest gaps and coverage verdicts; no silent clean grade. |

## Gated UX path

1. Studio reads the latest Rust-owned semantic-port coverage report and related
   evidence refs.
2. Studio displays unit status, oracle availability, fidelity grade, primary
   state hash, optional render digest, residual backlog, and Ring 2 flags.
3. A human may add intent/feel clarification or reject/defer notes as draft
   evidence.
4. Studio submits the draft to the existing `ouroforge` CLI / review-apply path;
   Studio never mutates artifacts directly.
5. Rust validates provenance, source-only refs, deterministic evidence, and gate
   status before accepting any transaction.
6. Accepted notes become evidence for downstream Era R re-derivation; rejected or
   unsafe notes remain visible as blocked review evidence.

## Fidelity and oracle rules

| Grade | UX meaning | Required evidence | Hand-off |
| --- | --- | --- | --- |
| 🟢 Green | Behavior unit is verified for this bounded semantic slice. | Captured oracle, M111 differential pass, deterministic primary state hash, no residual gaps, source-apply/review gates intact, and M112 coverage row verified. | `semantic_coverage` / `verify`; display as verified evidence only, not as a finished-game port. |
| 🟡 Yellow | More clean-room re-derivation is required. | Missing/incomplete oracle, lossy import, event mismatch, pending review, or human intent clarification needed. | `interrogate`, `oracle_capture`, `reexpress`, or `verify`; keep visible residual tasks. |
| 🔴 Red | Unsafe, blocked, or non-importable for this milestone. | Decompiled/ripped/shipped-build ref, live bridge, trusted-write attempt, deterministic state-hash break, exhausted convergence budget, or unresolvable Ring 2 feel/release decision. | `reject_or_defer`; never force a lossy auto-translation. |

Oracle rule: no unit may be displayed or described as “ported” without captured
acceptance evidence and passing gates. Even when a semantic slice is Green,
Studio copy should say “verified re-derived behavior evidence,” not “finished
game ported” or “fully ported.”

## Determinism requirements

- 2D: bit-exact primary state hash is the gate.
- 2.5D/3D: deterministic state-hash remains primary; perceptual SSIM/pixel-diff
  render evidence is secondary corroboration only.
- Physics is re-simulated in Ouroforge-native behavior; source-engine physics is
  never reproduced or embedded.
- Any state-hash drift is 🔴 until re-derived and re-verified.

## Two-plane boundary

- Rust (`crates/ouroforge-core`, `crates/ouroforge-evaluator`) owns artifact
  truth, validation, deterministic digests, fidelity grading, and convergence.
- Studio (Elixir/Phoenix LiveView) owns local control and presentation only:
  evidence rendering, human note capture, and gated CLI submission.
- Studio has no trusted-write authority, no direct artifact mutation authority,
  no imported-format semantics, and no new data store.

## Era R hand-off states

| UX finding | Hand-off |
| --- | --- |
| Missing intent or tacit behavior | Interrogate / oracle capture. |
| Captured oracle but no deterministic native behavior | Deterministic re-expression. |
| Native behavior exists but A/B mismatch remains | Differential verification repair. |
| Verified evidence exists but coverage incomplete | Semantic-port coverage backlog. |
| Human feel/taste/release decision | Ring 2 human escalation, reject or defer. |
| Unsafe source/runtime/write boundary | Reject or defer. |

## Governance anchors

This contract keeps #1 and #23 open. It may be referenced by downstream M113
implementation, demo, and scenario-coverage issues, but it does not itself edit
or close the governance anchors.

## Verification

```bash
grep -RIlqi "one-way\|on-ramp\|re-derivation\|fidelity\|two-plane\|source-project" docs/ || true
gh issue view 1 --json state --jq .state
gh issue view 23 --json state --jq .state
```

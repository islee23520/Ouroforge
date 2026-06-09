# Diagnosis Correction and Intervention Feedback Loop Demo v1

Era M Milestone 79 demo showing the local diagnosis correction path without adding a new write path or data store.

## Demo shape

- **Autonomous default:** the loop completes without a human correction surface and keeps the original evidence-linked attribution visible.
- **Optional correction:** local Studio captures a correction as intervention-as-evidence and read + gated-write control-plane data.
- **Rust route:** the captured correction is routed to the Rust data plane through `diagnosis-correction validate` and the existing review/apply, scene/source-apply, evaluator, evidence/provenance gates.
- **Re-attribution:** accepted corrections update transparent heuristic priors so later attribution can select the corrected cause; no opaque ML is introduced.
- **Rejected correction:** failed gate evidence remains visible and the autonomous loop continues without waiting for a human.

## Boundaries

- Human writes are optional, validated, and recorded; no raw bypass is available.
- Rust owns diagnosis semantics, validation, determinism, evidence, provenance, and prior re-attribution.
- Elixir/OTP + Phoenix LiveView own only local control and presentation; they render/capture/route inert data and never write artifacts.
- Local-first CLI fallback remains sufficient; hosted or multi-user Studio is Layer-3 DEFER.
- Fun/taste and release go/no-go remain human Ring 2 decisions and are not inferred by the demo.
- Governance anchors #1 and #23 remain open.

## Verification

```bash
cargo build --workspace --jobs 2
cd studio/executor && mix test --only demo
```

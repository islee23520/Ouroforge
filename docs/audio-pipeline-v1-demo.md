# Audio Generation and QA Demo v1

Issue: **#1645** (Era G Milestone 37). Part of Audio Generation and Audio-QA v1
(#1641). This is a **deterministic, fixture-scoped** demo: it composes the
already-merged Audio Generation Proposal Model (#1642), the Audio-QA Check
(#1643), and the Adaptive-Audio Runtime Hooks (#1644) into one end-to-end
walkthrough. It adds **no** new engine, runtime, or writer — it sequences
existing surfaces.

The demo reproduces from a fresh clone with no network and no live browser. It
asserts **behavior and gate states**, never subjective quality: it does not claim
audio "sounds good" or "is fun". Sound direction stays a human decision.

## Fixtures

All inputs are fixture-scoped under `examples/audio-pipeline-v1/demo/`:

| Fixture | Stage | Role |
| --- | --- | --- |
| `audio-brief.json` | Generation (#1642) | A licensed `AudioGenerationBrief` for an SFX chime carrying license/provenance. |
| `audio-brief-unlicensed.json` | Generation (#1642) | A brief that **omits the license**; generation fails closed. |
| `audio-qa-blocked.json` | Audio-QA (#1643) | An `AudioQaArtifact` whose loudness is **out of range** (`status: "fail"`). |
| `audio-qa-pass.json` | Audio-QA (#1643) | An `AudioQaArtifact` that passes format, loudness, license/provenance, and regression. |
| `audio-hooks.json` | Runtime hooks (#1644) | An `AudioHookSet` plus a signal sequence with the expected deterministic intents. |

No generated audio binaries, runs, or release artifacts are committed.

## Walkthrough

### 1. Generate (proposal-only)

`audio_generation::generate_audio` turns `audio-brief.json` into a
`MutationProposal` carrying license/provenance, routed through the existing
review/apply/trust-gradient path. A freshly generated proposal is **proposed /
pending / unverified** — it is never a direct trusted write and never
auto-promoted. The unlicensed brief (`audio-brief-unlicensed.json`) **fails
closed** at generation: unlicensed audio can never enter the pipeline.

### 2. Audio-QA gate — blocked

`audio_qa::AudioQaArtifact::computed_status` on `audio-qa-blocked.json` returns
`fail` (loudness out of range). The gate **fails closed**: an invalid (or
unlicensed, or un-provenanced) audio asset is **not** promotable. Composed into
the evaluator's `declared-gate-and` aggregation via `gate_verdict`, the audio-QA
category is `fail`.

### 3. Audio-QA gate — pass — promotable

`computed_status` on `audio-qa-pass.json` returns `pass`. Only an asset that
clears the audio-QA gate — valid format/loudness, complete license/provenance,
and no regression versus its baseline — is promotable. Promotion routes through
the audio-QA gate, never around it.

### 4. Adaptive hooks fire deterministically

`audio_hooks::AudioHookSet::evaluate` over the `audio-hooks.json` signal sequence
emits the documented, ordered audio intents (`ambient_loop` while calm;
`music_combat` then `ambient_loop` in combat). Evaluation is a pure function of
the signals, so it is **deterministic** and reproduces identically across runs and
across a snapshot/restore.

```text
brief -> generate (proposal-only) ----> unlicensed brief: fails closed
        verified brief -> audio-QA gate
                            |-- blocked (loudness out of range) -> not promoted
                            `-- pass -> promotable
adaptive hooks: world-state signals -> deterministic ordered audio intents
```

## Reproduce

The deterministic smoke test
`crates/ouroforge-core/tests/audio_pipeline_demo_contract.rs` asserts the blocked
generation, the blocked and passing audio-QA states, and deterministic hook
emission. It runs with no network and no live browser:

```bash
cargo test -p ouroforge-core --test audio_pipeline_demo_contract
```

## Boundaries

Rust/local owns the trusted generation, QA, and hook-evaluation logic; the
browser/Studio surfaces remain read-only. The demo is additive and
backward-compatible, commits no generated state, and makes no auto-merge,
quality/fun, production-readiness, or Godot-replacement claim. #1 and #23 remain
open.

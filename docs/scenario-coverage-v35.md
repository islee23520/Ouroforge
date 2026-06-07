# Scenario Coverage v35 — Audio Pipeline Regression Suite

Issue: **#1646** (Era G Milestone 37). Part of Audio Generation and Audio-QA v1
(#1641). Scenario Coverage v35 locks the behavior of the Audio Generation
Proposal Model (#1642), the Audio-QA Check (#1643), and the Adaptive-Audio
Runtime Hooks (#1644), and guards the backward compatibility of the existing
runtime audio-intent emission.

Scenario Coverage v35 is an enumerated, fixture-scoped regression suite. It
asserts **states and shapes only** — no flaky or timing-based assertions — so a
breaking change to the audio pipeline fails CI. Coverage numbering continues from
v34 onward.

## What is covered

The matrix
`examples/audio-pipeline-v1/scenario-coverage-v35/matrix.fixture.json` enumerates
every case; the runner
`crates/ouroforge-core/tests/scenario_coverage_v35_audio_pipeline.rs` executes
them against the real merged surfaces.

| Area | Case | Expected |
| --- | --- | --- |
| Audio proposal (#1642) | `proposal-valid` | generates a proposal (proposed / pending / proposal-only) |
| Audio proposal | `proposal-missing-license` | rejected fail-closed (missing license) |
| Audio proposal | `proposal-malformed` | rejected fail-closed (unsupported format) |
| Audio-QA check (#1643) | `qa-pass` | `pass` |
| Audio-QA check | `qa-loudness-fail` | `fail` (loudness out of range) |
| Audio-QA check | `qa-provenance-fail` | `fail` (missing provenance) |
| Adaptive hooks (#1644) | `hook-calm` | emits `ambient_loop` |
| Adaptive hooks | `hook-combat` | emits `music_combat`, `ambient_loop` (priority order) |
| Backward compatibility | audio-intent emission | the existing behavior-runtime audio-intent surface still emits deterministically |

The proposal, audio-QA, and hook cases are fixture-scoped under
`examples/audio-pipeline-v1/scenario-coverage-v35/`. The backward-compatibility
case reuses the existing `examples/behavior-runtime-v1/` execution fixture.

## Backward compatibility

The adaptive-audio hooks reuse the existing runtime audio-intent surface
(`startAudioIntent` → `world_state.audio_intents`). The suite re-runs the
behavior-runtime execution fixture and asserts it still emits its audio intent
deterministically, so a regression in that surface fails CI.

## Boundaries and governance

The suite asserts behavior, gate states, and shapes only — never subjective
quality. Sound direction stays a human decision. It is additive and
backward-compatible, commits no generated audio/runs/release artifacts (only
tiny deterministic source-like JSON fixtures), and makes no auto-merge,
quality/fun, production-readiness, or Godot-replacement claim. Rust/local owns
the trusted logic; browser/Studio surfaces remain read-only. #1 and #23 remain
open.

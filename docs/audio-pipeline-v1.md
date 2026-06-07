# Audio Generation and Audio-QA v1 Scope and Contract

Status: **Design gate — scope and contracts only; no executable behavior**

Issue: #1641 — Audio Generation and Audio-QA v1 Scope and Contract
Anchor: #1 Era G Milestone 37 (Audio Generation and Audio-QA)

This document is the canonical Audio Generation and Audio-QA v1 design artifact.
It extends the existing verified-asset pattern to audio (SFX, music, adaptive
audio) by defining three coupled contracts — the **audio-proposal contract**, the
**audio-QA contract**, and the **adaptive-audio runtime hook contract** — plus the
dependency order and closure gates for the follow-up sequence. It adds no
executable behavior, no schemas, and no new engine.

Audio is treated exactly like every other generated asset class: a verified
function with its own verification gate, not an unverified generator. Generated
audio is a proposal that must route through the existing review/apply/trust-
gradient path, carry license/provenance, and pass an audio-QA gate before it can
ever be promoted. There is no parallel audio engine, runtime, or writer.

## Why this milestone comes next

Asset Pipeline v1 brought project images, sprite atlases, tilesets, tilemaps,
audio metadata, and font references into a bounded, hash-pinned, evidence-native
review loop with read-only Studio inspection. The Source-Apply, Trust-Gradient,
Evolve-Campaign, and Provenance-Bundle work that followed established the
proposal-only mutation path, the risk-tiered trust gradient, and the
verifiable-asset provenance lineage.

Audio Generation and Audio-QA v1 is the next demand-driven function on that
foundation. It does not add new authority: it reuses the existing audio surfaces
(asset manifest `audio` kind, `AssetPreviewAudioMetadata`, the runtime audio-
intent surface, and the `AudioEvidence` scenario assertion) and the existing
generation-proposal, QA, and provenance machinery so that generated audio is held
to the same verification bar as code and other assets.

## Existing surfaces this milestone reuses

This milestone is defined entirely in terms of surfaces that already exist. No
follow-up issue may introduce a parallel audio engine, runtime, or writer.

| Concern | Reused surface |
| --- | --- |
| Audio asset class | Asset manifest `audio` kind and `ProjectAssetType::Audio` (`ogg`/`mp3`/`wav`), `crates/ouroforge-core/src/export_asset_manifest.rs` and `lib.rs` asset-manifest validation. |
| Audio preview metadata | `AssetPreviewAudioMetadata` (duration/channel) and `AssetPreviewEvidence` in `crates/ouroforge-core/src/lib.rs`. |
| Adaptive-audio runtime | The deterministic runtime audio-intent surface: `startAudioIntent`/`audioIntent` behaviors and `world_state.audio_intents` in `crates/ouroforge-core/src/behavior_runtime.rs` and `examples/game-runtime/runtime.js`. |
| Audio scenario evidence | The `audio_evidence` scenario assertion (`ScenarioAssertion::AudioEvidence`) and the evaluator. |
| Proposal-only mutation | The existing `source_apply_*` / `trust_gradient_*` review/apply/trust-gradient path. |
| Provenance / license | `crates/ouroforge-core/src/provenance_bundle.rs` and the asset manifest license/provenance fields. |
| Generation proposals | The existing generation-proposal model and `evolve_campaign.rs`; the QA swarm for function QA. |
| Read-only inspection | The static evidence dashboard and authoring cockpit surfaces. |

## Audio-proposal contract

Generated audio (SFX, music, adaptive layers) is a **proposal**, never a trusted
write.

- Generated audio enters the system only as a proposal through the existing
  review/apply/trust-gradient path. Generation, role agents, the producer, and
  any browser/Studio surface have no direct trusted-write authority over audio.
- Every audio proposal carries **license** and **provenance**: the source/model
  or origin, the license terms, the run/command id that produced it, and the
  observed file hash. An audio proposal without complete license/provenance fails
  closed and can never be promoted.
- Audio proposals reuse the asset-manifest `audio` kind and the provenance-bundle
  surface; they do not define a new audio asset schema or a new storage location.
- High-risk and source-affecting changes are never auto-applied; audio promotion
  is gated by review/apply and the trust gradient exactly like other proposals.
- No autonomous apply, auto-merge, self-approval, or reviewer bypass. The human
  release go/no-go is preserved.

## Audio-QA contract

Audio promotion requires a function-specific QA gate, analogous to the visual
gate for images. The audio-QA gate is mandatory and fails closed.

- **Format validity** — the file is a supported audio container/codec
  (`ogg`/`mp3`/`wav` per the existing manifest classification) and is parseable.
- **Loudness validity** — measured loudness/peak falls within a declared bounded
  range; out-of-range or unmeasurable audio fails the gate.
- **License/provenance completeness** — license terms and provenance lineage are
  present and well-formed; missing or ambiguous license fails closed.
- **Regression vs baseline** — the candidate is compared against the declared
  baseline (duration/channel/loudness metadata and provenance) using the existing
  `compare` surface; an unexplained regression fails the gate.

The audio-QA gate produces generated evidence (status, measurements, baseline
diff, and bounded diagnostics) and never makes a quality/taste judgement.
"Sounds good" and sound-direction remain human decisions; the gate only proves
format/loudness validity, license/provenance completeness, and non-regression.

## Adaptive-audio runtime hook contract

Adaptive audio reuses the existing runtime audio-intent surface; it does not add
a new runtime, mixer, or audio engine.

- Adaptive-audio behavior is expressed as **audio intents** emitted by the
  deterministic runtime (`startAudioIntent` → `world_state.audio_intents`) and
  verified through the existing `audio_evidence` scenario assertion.
- The runtime emits intent metadata only — it does not ship a trusted audio mix,
  perform real mixing, or claim audio fidelity. Browser audio remains a best-
  effort, read-only runtime concern with existing limitation warnings.
- Adaptive-audio evidence is generated/local state and is correlated with the
  scenario/worker that produced it, consistent with existing runtime evidence.
- The runtime/probe is JavaScript and read-only with respect to trusted state;
  trusted validation, persistence, and evidence writing remain Rust-owned.

## Dependency order

Follow-up Audio Generation and Audio-QA v1 issues are implemented in this order;
they must not be skipped or merged across categories without an explicit
governance comment:

1. **Scope and Contract** (this issue, #1641) — define the contracts, boundaries,
   reuse statement, and dependency order; no executable behavior.
2. **Audio Generation Proposal Model v1** (#1642) — define the audio-proposal
   model that carries license/provenance and routes through review/apply/trust-
   gradient. *(Prerequisite: #1593 merged.)*
3. **Audio-QA Check v1** (#1643) — implement the format/loudness/license/
   regression QA gate reusing the existing QA and compare surfaces.
4. **Adaptive-Audio Runtime Hooks v1** (#1644) — wire adaptive-audio hooks onto
   the existing runtime audio-intent surface with `audio_evidence` verification.
5. **Audio Generation and QA Demo v1** (#1645) — extend an existing local demo
   with manifest-backed, license/provenance-carrying, QA-gated audio evidence.
6. **Scenario Coverage v35: Audio Pipeline Regression Suite** (#1646) — add
   regression coverage for the proposal, QA gate, hooks, and negative/blocked
   cases.
7. **Roadmap and #1 Governance Refresh after Audio Generation and Audio-QA v1**
   (#1647) — record the milestone outcome, keep #1/#23 open, decide next
   sequencing.

```text
#1641 scope -> #1642 -> #1643 -> #1644 -> #1645 -> #1646 -> #1647
```

## Local audio fixture policy

Follow-up issues may add tiny deterministic source-like fixtures only when
explicitly scoped:

| Fixture class | Boundary |
| --- | --- |
| Minimal audio metadata / manifest fixtures | Prefer metadata/reference fixtures (duration, channels, loudness, license, provenance) over binary audio. |
| Tiny audio binaries | Allowed only with explicit issue scope; deterministic, license-clear, bounded size; no proprietary/licensed third-party drops. |
| Audio scenario packs | Source-like regression fixtures only; generated run/QA evidence remains ignored. |

Fixtures must avoid large binary blobs, proprietary/licensed third-party audio,
remote URLs, license ambiguity, generated caches, and platform-specific outputs.

## Generated state policy

Generated audio proposals, QA reports, loudness measurements, baseline diffs,
adaptive-audio runtime evidence, dashboard exports, and temporary outputs remain
generated/local state. They live under ignored generated roots (e.g. `runs/`,
`target/`) unless a follow-up issue scopes a tiny deterministic fixture as
tracked source-like data.

## Rust-trusted / browser-read-only boundary

Rust (or an explicitly trusted local CLI boundary) owns audio-proposal
validation, license/provenance checks, the audio-QA gate, persistence, and
generated evidence writing. TypeScript/JavaScript owns the deterministic runtime
(including audio-intent emission), the `window.__OUROFORGE__` probe, and static
read-only dashboard/cockpit display. Browser/Studio surfaces may display exported
audio/QA/provenance state only as escaped read-only data. They must not upload
audio, write trusted files, edit manifests, promote audio, execute commands, or
fetch remote audio. No new language/runtime is introduced; distributed/Elixir
remains NO-GO per ADR #92.

## Verification gates for follow-up issues

Every Audio Generation and Audio-QA v1 implementation issue defines focused
verification for its changed surface plus the broad gates:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short --ignored
```

If dashboard or cockpit UI files change, the issue also runs the Node checks:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Closure evidence includes #1/#23 state, no generated-state drift, a fixture
size/purpose audit when fixtures are added, and proof that browser surfaces
remain read-only.

## Explicit non-goals

Audio Generation and Audio-QA v1 does not authorize:

- a parallel audio engine, runtime, mixer, or writer — only reuse of existing
  runtime/evaluator/asset/provenance/QA surfaces;
- direct trusted writes from generation, role agents, the producer, or any
  browser/Studio surface; audio is proposal-only through review/apply/trust-
  gradient;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted
  writes; high-risk/source-affecting changes are never auto-applied;
- promotion of unlicensed, uncredited, or unverified audio; license/provenance
  and the audio-QA gate are mandatory;
- any automated quality/fun/taste claim; "sounds good" and sound-direction remain
  human decisions;
- shipping (native/store export), hosted/cloud audio storage, a paid/hosted audio
  store, real-player telemetry, or live-ops absent an explicit Layer-3 GO (DEFER
  per Milestone 26 / #1508);
- engine/content/system breadth beyond what a specific loop-produced rung
  (Milestone 24) justifies;
- generated runs/assets/content/release artifacts committed unless explicitly
  fixture-scoped;
- a claim of production-ready engine, Godot replacement/parity, or autonomous
  shipping of finished games; or
- closing, replacing, or narrowing #1 or #23.

## Closure policy for this milestone

Audio Generation and Audio-QA v1 is a bounded, local-first, Rust-trusted,
evidence-native function. It is complete when its ordered follow-up issues merge,
latest-main verification passes, generated/local artifacts remain untracked, and
the roadmap/#1 governance refresh records the outcome. This is a bounded audio-
function completion claim, not a claim of audio-production readiness, audio
quality, or a Godot replacement.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. Both remain open unless a separate explicit governance decision
says otherwise.

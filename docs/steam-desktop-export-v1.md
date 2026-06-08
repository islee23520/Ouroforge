# Steam Desktop Export and Steamworks v1 Scope and Design Gate

Issue: **#1837** (#1 Era I Milestone 54)

Status: **Design gate complete — bounded GO for local Steam desktop export preparation; DEFER human/Ring-3 release authority, market demand, and Layer-3 cloud/mobile.**

This is a scope/design-gate document. It adds **no executable Steam export behavior**, no Electron package, no `steamworks.js` package, no SteamPipe upload, no Steam credential handling, no code signing, no store submission, no release automation, and no new runtime. It defines the contracts, boundaries, and follow-up dependency order for turning an existing web release candidate into a shippable Steam desktop build when the human-owned release steps are supplied outside the engine.

Steam desktop export is a **local desktop export**, not Layer-3 cloud/mobile. Hosted/cloud/mobile capability remains **DEFER** per Milestone 26 / #1508 unless a separate Layer-3 GO is approved.

#1 remains the roadmap/vision anchor and #23 remains the repo memory/design anchor. This gate preserves both issues as open anchors.

## ADR question

Can Ouroforge define a bounded Steam Desktop Export and Steamworks v1 path that reuses the existing web runtime and Milestone 36 asset pipeline while keeping human/Ring-3 release obligations out of engine scope?

## Decision summary

| Area | Decision | Contract |
| --- | --- | --- |
| Web to desktop wrapper | **GO, bounded** | Wrap the existing web release candidate with **Electron + steamworks.js**. The wrapper is an export shell over the current deterministic web runtime; it is not a replacement runtime or second game engine. |
| SteamPipe build/depot pipeline | **GO, bounded** | Define local build/depot packaging, checksums, provenance, and dry-run validation. Credentialed upload/publish and Steam account operations are human/Ring-3 and out of scope. |
| Steamworks integration | **GO, bounded** | Define proposal/contract surfaces for overlay, achievements, cloud saves, and daily-seed leaderboard. Trusted validation, persistence, provenance, export decisions, and CLI behavior remain Rust/local. |
| Store-asset generation | **GO, bounded** | Reuse the Milestone 36 asset pipeline and asset-QA/provenance gates for capsule/header/library assets. Generated or derived store assets are proposals only until license/provenance, QA, and human review pass. |
| Steam account, code signing, content survey, Release button, market demand | **DEFER / Ring-3** | Human-owned release and market obligations. They are not automated, not bypassed, and not represented as engine success criteria. |
| Layer-3 cloud/mobile | **DEFER** | Steam is local desktop export only; hosted/cloud/mobile remains outside this milestone. |

The bounded GO authorizes only contract definition and later local implementation issues in the follow-up sequence. It does not authorize autonomous shipping.

## Goals

- Define the canonical export architecture: existing web release candidate → Electron shell → `steamworks.js` bridge → Steam desktop build.
- Define the SteamPipe build/depot pipeline contract with local package evidence, checksums, manifests, and provenance.
- Define the Steamworks integration surface: overlay, achievements, cloud saves, and daily-seed leaderboard.
- Define store-asset generation by reusing the Milestone 36 asset pipeline, asset manifest/loader, asset-QA, and provenance contracts.
- Define dependency order and closure gates: **#1837 scope -> #1838 -> #1839 -> #1840 -> #1841 -> #1842 -> #1843**.
- Preserve #1 and #23 as open governance anchors.

## Non-goals and human/Ring-3 split

Human/Ring-3, out of scope:

- Steam account creation, partner onboarding, app IDs, credentials, and account administration.
- Code signing certificates, signing identity, notarization decisions, and credential storage.
- Steam content survey, store questionnaire, legal/platform attestations, ratings, and policy acceptance.
- The Steam **Release** button, upload/publish approval, store launch timing, and release go/no-go.
- Market demand, wishlists, user acquisition, discoverability, pricing, discounts, community management, and support obligations.

Engine/repo non-goals:

- No executable behavior in this issue beyond documentation and regression tests.
- No new runtime, parallel engine, or Godot replacement/parity claim.
- No direct trusted write from generation, browser, Studio, dashboard, cockpit, Electron, or Steamworks surfaces.
- No autonomous apply, auto-merge, self-approval, reviewer bypass, hidden trusted writes, or release bot.
- No automated quality/fun/taste, market-readiness, commercial-readiness, or production-ready claim.
- No hosted/cloud/mobile Layer-3 capability; Steam desktop export is local desktop only.
- No generated runs/assets/builds committed unless explicitly fixture-scoped.

## Export architecture contract

The Steam desktop export starts from the existing web release candidate and wraps it for local desktop execution.

| Layer | Ownership | Contract |
| --- | --- | --- |
| Existing web runtime | TypeScript/JavaScript | Keeps deterministic runtime behavior, `window.__OUROFORGE__` probe data, UI, juice/feedback, and browser-local read-only inspection. |
| Electron wrapper | JavaScript shell | Hosts the already-built web runtime, selects packaged assets, exposes only the minimum Steam bridge, and records local export evidence. It does not own trusted validation or persistence. |
| `steamworks.js` bridge | JavaScript bridge | Provides bounded calls for overlay, achievements, cloud, and daily-seed leaderboard. It cannot write trusted game state directly. |
| Rust/local export and provenance | Rust/local | Owns trusted validation, export manifests, checksums, provenance, review/apply/trust-gradient boundaries, evidence writing, run/project binding, and CLI behavior. |

The wrapper must reuse the existing web runtime and asset surfaces. It must not introduce a second gameplay runtime, a new engine, or direct trusted writes from desktop UI code.

## SteamPipe build/depot pipeline contract

The SteamPipe path is a local packaging and evidence pipeline until a human supplies credentials and performs release actions.

1. Build the existing web release candidate using existing runtime/export surfaces.
2. Wrap the web artifact in the Electron desktop shell.
3. Produce platform-specific local desktop bundles and fixture-scoped build manifests.
4. Generate depot layout metadata, checksums, package manifests, and provenance evidence.
5. Validate the depot layout and required files in a dry-run/local mode.
6. Stop before credentialed upload, partner-site mutation, publishing, or the Release button.

Credentialed SteamPipe upload, store mutation, account actions, code signing, content survey, and release approval are human/Ring-3 and are not engine-owned.

## Steamworks integration surface

Steamworks v1 is a bounded integration surface over existing trusted state.

| Surface | Contract | Boundary |
| --- | --- | --- |
| Overlay | Enable the Steam overlay from the desktop wrapper when available. | Overlay availability is read-only evidence; no trusted state mutation. |
| Achievements | Map approved local achievements from Rust/local validated state to Steam achievement IDs. | Achievement unlock proposals must derive from trusted local state; browser/Electron cannot invent unlocks. |
| Cloud saves | Package validated local save/project state for Steam Cloud sync. | Rust/local owns serialization, validation, and conflict policy; Steam Cloud is transport/storage only. |
| Daily-seed leaderboard | Publish or read leaderboard entries for deterministic daily-seed results. | Scores must come from trusted local run evidence; no browser/Studio direct trusted writes. |

All Steamworks calls fail closed when credentials, app IDs, platform support, trusted state, or provenance are missing. The integration never asserts that a game is fun, commercially ready, or production ready.

## Store-asset generation contract

Store assets reuse the Milestone 36 asset pipeline rather than creating a parallel store-art system.

- Capsule/header/library assets are generated or selected as **asset proposals**.
- Each proposal carries license, attribution where required, source/provenance, dimensions, hash, and intended store slot.
- Asset-QA verifies format/resolution, license/provenance completeness, style-baseline conformance, and visual regression where a baseline exists.
- Human review/apply is required before promotion.
- Browser, Studio, dashboard, cockpit, Electron, and Steamworks surfaces remain read-only for trusted state.
- Generated store assets, release artifacts, depot output, and packaged builds remain untracked unless explicitly fixture-scoped.

Milestone 36 asset pipeline reuse includes asset manifest/loader/atlas validation, provenance bundle, compare/visual evidence, asset-QA, and the existing review/apply/trust-gradient path.

## Dependency order and closure gates

Follow-up sequence:

1. **#1837** — scope and contract (this design gate).
2. **#1838** — Web-to-Desktop Wrapper and Build Pipeline v1.
3. **#1839** — Steamworks Integration v1.
4. **#1840** — Store-Asset Generation v1.
5. **#1841** — Steam Desktop Export Demo v1.
6. **#1842** — Scenario Coverage v49: Steam Desktop Export Regression Suite.
7. **#1843** — Roadmap and #1 Governance Refresh after Steam Desktop Export v1.

Closure gates:

- #1838 must reuse the existing web runtime and stop before credentialed upload or release.
- #1839 must keep trusted validation/persistence in Rust/local and treat overlay/achievements/cloud/leaderboard as bounded Steamworks bridges.
- #1840 must reuse Milestone 36 asset pipeline and keep generated store assets proposal-only until QA/provenance/human review pass.
- #1841 must be a local demo with fixture-scoped artifacts only.
- #1842 must continue Scenario Coverage numbering at **v49** and cover blocked negative cases.
- #1843 must refresh roadmap/#1 governance while keeping #1 and #23 open.

## Conservative wording and governance

- This gate defines a bounded local Steam desktop export path; it does not claim autonomous shipping.
- Steam account, code signing, content survey, Release button, and market demand remain human/Ring-3.
- Steam desktop export is not Layer-3 cloud/mobile.
- Browser, Studio, dashboard, cockpit, Electron, and Steamworks surfaces remain read-only for trusted state.
- High-risk and source-affecting changes never auto-apply.
- Public wording must avoid production-ready, commercial-readiness, quality/fun, Godot replacement/parity, autonomous shipping, auto-merge, self-approval, reviewer bypass, and hidden trusted write claims.
- Distributed/Elixir remains NO-GO per ADR #92 (`docs/distributed-elixir-design.md`).
- #1 remains open.
- #23 remains open.

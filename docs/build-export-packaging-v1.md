# Build / Export / Packaging v1 Scope and Contract

Issue: #719
Roadmap anchor: #1 (Build / Export / Packaging milestone).
Status: scope contract only; no executable behavior.

Build / Export / Packaging v1 is the local-first, evidence-backed package foundation for playable game artifacts. It turns an agent-authored game/project into a reproducible local web package with export profile validation, deterministic staging, artifact checksums, runtime probe compatibility, verification evidence, and read-only Studio inspection. It does not authorize public release, deployment, signing, store publishing, CI release automation, or production distribution.

This document is the canonical contract for all follow-up build/export/package issues. It adds no package-generation behavior; each follow-up issue implements one bounded slice against the boundaries defined here.

## Bounded target

The milestone covers the following bounded capabilities, each implemented and verified independently:

- Export target matrix: the bounded set of supported local web export targets (`docs/export-target-matrix-v1.md`).
- Export profile schema: a validated declaration of an export profile.
- Export plan generator: a deterministic plan derived from a profile and project.
- Local web bundle assembly: deterministic staging of a local, runnable web bundle.
- Asset manifest and path rewriting: a manifest and bounded, validated path rewriting for bundled assets.
- Runtime probe preservation: preservation of runtime probe compatibility for evidence-native QA.
- Staging and generated-state policy: declared staging roots with generated-state remaining ignored.
- Checksums and provenance: artifact checksums and provenance metadata.
- Export verification runner: a runner using allowlisted local commands only.
- Evidence bundle: an aggregated, fixture-scopable export evidence bundle.
- Package metadata: descriptive, non-publishing package metadata.
- Studio read-only inspection: read-only inspection of export plans and evidence.
- Release/publish blocker: an explicit blocker that fails closed on publish/deploy/sign/upload attempts.
- Demo, regression suite, and roadmap governance refresh.

## Trusted boundary

- Build/export/package work assembles local, evidence-backed playable artifacts only; it does not publish, deploy, sign, upload, or distribute.
- Export verification uses allowlisted local commands only. There is no arbitrary command runner, browser command bridge, hidden command execution, network/install command, or credentialed operation.
- Rust/local validation owns trusted export validation, package planning, artifact writing, staging policy, checksums, evidence, run/project binding, and CLI contracts.
- Browser, dashboard, and Studio surfaces remain read-only inspection surfaces. They cannot publish, deploy, sign, upload, install dependencies, mutate CI/workflows, or run arbitrary commands.
- Runtime probe compatibility and scenario smoke verification are part of export-completion evidence; a probe regression is reported explicitly, never silently dropped.

## Generated-state policy

Export outputs, staging folders, package bundles, verification logs, screenshots, checksums, temp servers, and local tool state remain generated and ignored unless explicitly fixture-scoped. Each follow-up PR includes a generated-state audit (`git status --short --ignored`).

## Dependency order for follow-up issues

1. This scope and contract issue (#719) lands first.
2. The export target matrix and export profile schema land before plan generation.
3. The export plan generator and local web bundle assembly build on the profile and matrix.
4. Asset manifest/path rewriting, runtime probe preservation, checksums/provenance, and the verification runner build on bundle assembly.
5. The evidence bundle and package metadata aggregate the slices above.
6. Studio read-only inspection, the release/publish blocker, the demo, and the regression suite proceed once assembly and evidence exist.
7. A roadmap and #1 governance refresh closes the milestone.

Each follow-up issue must verify its slice independently and must not combine profile, plan, bundle assembly, asset manifest, probe preservation, verification, evidence, Studio, blockers, and demo behavior into a single PR when they can be verified separately.

## Verification and closure gates

Every follow-up PR must pass the standard repository gates (`cargo fmt --check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, the dashboard/cockpit node smokes, `git diff --check`, and a clean `git status --short --ignored`) and must add focused tests/smokes for the exact build/export/packaging behavior it implements. Closure evidence must include generated-state, no-publish, artifact-integrity, export-evidence, conservative-wording, and #1/#23 governance audits.

## Explicit non-goals

- No public release, deployment, upload, publishing, or hosting.
- No signing, notarization, installers, mobile export, console export, app-store/Steam/itch publishing, or credentialed release flow.
- No arbitrary build-script execution, dependency mutation, CI/workflow mutation, browser trusted writes, command bridges, network/install commands, or hidden command execution.
- No production-ready export claim, Godot replacement claim, secure distribution claim, commercial release readiness claim, or multi-platform export parity claim.
- No generated export/package artifacts committed unless explicitly fixture-scoped.
- No unrelated plugin/extension, full studio editor, or Godot-plus demonstration game implementation.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open through this contract issue; this milestone does not close or modify either without a separate explicit governance decision.

## Completion and governance refresh (#736)

Build / Export / Packaging v1 is complete as a local evidence-backed export
foundation after #720-#736. Completed capability includes the local export target
matrix, export profile validation, dry-run export planning, deterministic
staging under ignored generated roots, local web bundle assembly, asset
manifest/path rewriting, runtime probe preservation, checksum/provenance
primitives, package metadata/descriptor coverage, export verification coverage,
fixture-scoped Scenario Coverage v15, read-only dashboard/Studio inspection
contracts, and explicit release/publish blockers.

Remaining work is intentionally separate: Plugin / Extension System v1, Full
Studio Editor v1, native/desktop/mobile/store export design gates, signing,
release/publish workflows, and the Godot-plus demonstration game each require
their own scoped issues and evidence. This completion does not authorize public
release, deployment, upload, signing, app-store/Steam/itch publishing,
credential use, CI release automation, production distribution, commercial
release readiness, secure distribution, multi-platform parity, current Godot
replacement status, or closing #1/#23.

#1 remains open. #23 remains open.

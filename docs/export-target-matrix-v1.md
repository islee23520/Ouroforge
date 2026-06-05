# Export Target Matrix and Non-Publish Governance v1

Issue: #720
Roadmap anchor: #1 (Build / Export / Packaging milestone).
Parent scope: #719 (`docs/build-export-packaging-v1.md`).
Status: governance contract only; no executable behavior.

This document is the fail-closed source of truth for which export targets the
Build / Export / Packaging v1 milestone may produce. Export profile and export
plan validation reference this matrix to decide whether a requested target is
allowed, future-gated, or blocked. A target that is not listed as `allowed`
here is treated as blocked: validation fails closed.

The milestone produces a **local, evidence-backed, playable package** for local
inspection and QA. It does **not** produce a **public release**: it never
publishes, deploys, signs, notarizes, uploads, distributes, or changes public
visibility. "Export" in this milestone means *assemble a local runnable bundle
with evidence*, never *release to users*.

## Export target matrix

Each export target has exactly one status: `allowed`, `future`, or `blocked`.

| Target | Status | Notes |
| --- | --- | --- |
| `web-local` | allowed | Local runnable web package served from a local static server for inspection and runtime-probe QA. |
| `web-static-bundle` | allowed | Deterministic static web bundle (assets + manifest + checksums) staged into an ignored, fixture-scopable staging root. |
| `desktop-wrapper` (Tauri/Electron/native shell) | future | Design-gated. Not implemented in v1. Requires a separate explicit capability report/design gate issue before any shell, packaging, IPC, or signing code; see `docs/native-export-design.md` (NO-GO for native export now). |
| `mobile` (iOS/Android) | blocked | No mobile export. |
| `console` | blocked | No console export. |
| `app-store` (Apple/Google/Microsoft store) | blocked | No store publishing. |
| `steam` | blocked | No Steam publishing/upload. |
| `itch` | blocked | No itch.io publishing/upload. |
| `hosted-deploy` (cloud/server/CDN hosting) | blocked | No hosted deployment or public hosting. |
| `signed-release` (signing/notarization) | blocked | No signing, notarization, or credentialed release flow. |
| `ci-release` (CI/workflow release automation) | blocked | No CI release automation or release-workflow mutation. |

### Allowed (v1)

- `web-local`
- `web-static-bundle`

Allowed targets stay local: outputs, staging folders, bundles, verification
logs, screenshots, checksums, and temp servers remain generated and ignored
unless explicitly fixture-scoped.

### Future (design-gated, not implemented)

- `desktop-wrapper`

A future target is not authorized by this milestone. It may only be implemented
after a separate explicit design-gate/capability-report issue scopes it. Until
then, profile/plan validation treats it the same as a blocked target.

### Blocked (fail closed)

- `mobile`, `console`, `app-store`, `steam`, `itch`, `hosted-deploy`,
  `signed-release`, `ci-release`.
- Any target not listed as `allowed` above.

Publish, deploy, signing, notarization, credential use, upload, and CI release
automation are blocked regardless of target. These are not "missing features";
they are explicit governance blocks that validation must fail closed on.

## Relation to the prior source-apply release/export mutation blocker

Earlier milestones blocked release/export mutation entirely at the source-apply
boundary. This milestone narrows that blocker precisely:

- **Now allowed:** writing local generated export artifacts (staged bundles,
  manifests, checksums, verification logs, evidence) for `allowed` targets, into
  ignored or fixture-scoped staging roots.
- **Still blocked:** release/publish mutation — publishing, deploying, signing,
  notarizing, uploading, distributing, changing public visibility, mutating
  CI/workflows, or running credentialed/networked release operations.

Producing a local artifact is not releasing it. The release/publish blocker
remains in force for every blocked target and every publish/deploy/sign/upload
operation.

## Trusted boundary

- Rust/local trusted code owns target validation against this matrix, package
  planning, artifact writing, staging policy, checksums, and evidence writing.
- Export verification uses allowlisted local commands only. No arbitrary command
  runner, browser command bridge, hidden command execution, network/install
  command, or credentialed operation.
- Browser, dashboard, and Studio surfaces remain read-only inspection surfaces.
  They cannot publish, deploy, sign, upload, install dependencies, mutate
  CI/workflows, or run arbitrary commands.

## Governance anchors

This milestone enables local, evidence-backed package assembly, not public
release, deployment, store publishing, or production distribution. It makes no
claim of Godot replacement, production-ready export, secure distribution,
commercial release readiness, or multi-platform export parity.

#1 remains the broad roadmap/final-goal anchor and remains open. #23 remains the
repo-memory/design-context anchor and remains open. This milestone does not
close or modify either without a separate explicit governance decision.

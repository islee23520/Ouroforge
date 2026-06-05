# Desktop Packaging Capability Gate v1

Issue: #732
Roadmap anchor: #1 (Build / Export / Packaging milestone).
Parent scope: #719 (`docs/build-export-packaging-v1.md`).
Related ADR: `docs/native-export-design.md` (NO-GO for native export now).
Status: capability gate only — **desktop packaging is NOT implemented in v1**.

This document records the long-term desktop packaging requirements for a future
Godot-class target. It implements no native desktop export: no native wrapper,
installer, runtime bundling, signing, or notarization is added. The
`desktop-wrapper` export target stays future/design-gated in the export target
matrix (`docs/export-target-matrix-v1.md`) and remains blocked by export profile
validation until a separate issue explicitly scopes it.

## Future requirements (not implemented)

A future desktop packaging effort must address, per platform
(Windows / macOS / Linux):

- **Native wrapper questions:** which shell (Tauri/Electron/custom) and why,
  IPC/security boundary, process model, and how it preserves the browser-first
  evidence loop.
- **Runtime bundling constraints:** bundling the local web runtime and assets,
  offline load, and deterministic, reproducible packaging.
- **Signing / notarization constraints:** code signing, notarization, and
  credential handling — all currently blocked and out of scope.
- **Sandbox / security concerns:** local file access, IPC message passing,
  generated-evidence isolation, and update/deployment behavior.
- **Evidence requirements:** the exported desktop artifact must trace to a Seed,
  run manifest, scenario verdict, evidence index, and the export evidence
  bundle, with documented parity to browser-worker QA.

## Capability report

The capability report fixture
(`examples/desktop-packaging-capability-report-v1/capability-report.fixture.json`)
declares desktop packaging as `future` / not-implemented, lists the target
platforms and requirement areas, and is validated by the
`export_capability_report` module. The report cannot mark desktop packaging as
implemented in v1.

## Boundary

No native wrapper, installer, signing, or notarization implementation is added.
This milestone makes no claim of desktop export, production-ready export, secure
distribution, or Godot replacement. #1 remains the broad roadmap anchor and
remains open; #23 remains the repo-memory/design anchor and remains open.

# Local web package inspection handoff evidence (#2498)

Issue: #2498
M130 phase: #2393 local package / export
Stable handoff manifest: `examples/production-usability-gate-v111/local-package-inspection-handoff.fixture.json`
Generated package manifest: `dist/local-web/signal-gate-relay/manifest.json`
Generated smoke diagnostics: `runs/issue-2498/local-package-handoff-smoke.json`
Provenance anchor: `examples/production-usability-gate-v111/local-package-provenance.fixture.json`

## Closure classification

Closure classification: product-observed complete for the bounded local package inspection handoff increment only.

This evidence extends #2393 with a reviewer-readable handoff that links source project, export profile, package metadata, generated package manifest, checksums, runtime probe preservation, and copyable local smoke commands. It improves local review and install/run reproducibility without expanding distribution scope. It does not claim commercial readiness, native export, store submission, signing, upload, deployment, publishing, public release automation, browser trusted writes, command bridges, self-approval, auto-apply, or auto-merge.

## Reviewer reproduction steps

1. Read the handoff manifest and provenance fixture together:
   - `examples/production-usability-gate-v111/local-package-inspection-handoff.fixture.json`
   - `examples/production-usability-gate-v111/local-package-provenance.fixture.json`
2. Confirm `runtimeProbePreserved` is `true` in both fixtures, `sourceProjectRef` points at the source project, and that `nonGoals` match (no native export, store upload, signing, public release automation, or new export engine).
3. Confirm linked source contracts exist in the repo:
   - `examples/playable-demo-v2/collect-and-exit/export/export-profile.json`
   - `examples/playable-demo-v2/collect-and-exit/export/package-metadata.json`
4. Copy and run each command from `localSmokeSteps` in order (from the repository root). The first command generates the local package at the handoff `packageRoot`, writes `dist/local-web/signal-gate-relay/manifest.json`, writes `dist/local-web/signal-gate-relay/checksums.sha256`, and records diagnostics in `runs/issue-2498/local-package-handoff-smoke.json`; the commands do not execute from the browser or Studio.
5. After the local export/package run (ignored under `dist/`), inspect `dist/local-web/signal-gate-relay/checksums.sha256` and compare against the handoff `checksumsRef`.
6. Serve the packaged tree locally with the final `python3 -m http.server` step and confirm the runtime probe remains available in the packaged web artifact (`runtimeProbeMode: preserve` on the export profile).
7. Run fixture smokes:

```bash
cargo test -p ouroforge-core --test local_package_inspection_handoff -- --exact local_package_handoff_generates_and_smokes_packaged_artifact
node examples/production-usability-gate-v111/gate-smoke.test.cjs
node examples/production-usability-gate-v111/local-package-handoff-smoke.test.cjs
git diff --check
```

## Generated-state audit

Package outputs, checksum files, local smoke JSON, screenshots, and temp servers remain under ignored roots (`dist/`, `runs/`, `screenshots/`, `browser-profiles/`). Tracked changes are limited to the handoff manifest, evidence doc, gate index update, and smoke test.

#1 and #23 remain open governance anchors.
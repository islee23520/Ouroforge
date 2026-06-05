# Build / Export / Packaging Demo v1

Issue: #734 — #1 Build / Export / Packaging milestone (local-first, evidence-backed
playable artifact assembly).

## What this demo is

A read-only walkthrough of the full v1 build/export/packaging flow over one small
fixture game. It composes the existing export pipeline — export profile, export
plan, local web bundle, asset manifest, build fingerprint/checksums, runtime
probe check, export verification, and evidence bundle — and asserts that the
fixture exports successfully to a local web package with a passing verification
verdict.

The demo is **local-only and evidence-backed**. It does not publish, deploy,
sign, notarize, upload, host, or commit generated outputs outside fixtures.

## Stages

| Stage id | Coverage | Focused check | Expected result |
| --- | --- | --- | --- |
| `BEP734.profile` | Allowed web-static export profile. | `ExportProfile::from_json_str`. | Profile parses; target is an allowed v1 web target. |
| `BEP734.plan` | Deterministic export plan. | `ExportPlan::from_profile`. | Plan lists the entry scene, assets, and allowlisted verification steps. |
| `BEP734.web-bundle` | Local web package under ignored staging. | `assemble_web_bundle`. | Bundle contains `index.html`, runtime bootstrap, styles, and the entry scene; the runtime probe is preserved. |
| `BEP734.asset-manifest` | Declared assets rewrite to packaged URLs. | `build_asset_manifest`. | Each entry carries a `sha256` content hash and a deterministic output path. |
| `BEP734.fingerprint-checksums` | Per-artifact checksums and provenance. | `build_fingerprint`. | Artifact checksums are non-empty; the toolchain id is recorded. |
| `BEP734.probe-check` | Runtime probe compatibility survives packaging. | `verify_export_bundle` in dev and packaged probe modes. | The runtime probe global and required methods remain present. |
| `BEP734.verification` | Allowlisted local verification only. | `verify_export_bundle` report. | Verification passes on load/probe/scenario smoke; no network/install/credentialed command is introduced. |
| `BEP734.evidence-bundle` | Aggregated evidence with a verdict. | `build_export_evidence` + read model. | Verdict is `pass`; the read model serializes for read-only dashboard/Studio inspection. |

## Commands

```bash
cargo test -p ouroforge-core --test build_export_packaging_demo
```

## Fixture policy

The demo manifest under `examples/build-export-packaging-demo-v1/` and the reused
fixture profile under `examples/export-bundle-v1/` are small, deterministic, and
fixture-scoped. Generated export/package artifacts (bundles, checksums,
verification logs, staging folders, temp servers) remain generated and ignored
under `target/ouroforge/exports/` and other ignored roots; nothing generated is
committed.

## Non-goals and guardrails

This demo does not authorize public release, deployment, hosting, upload,
signing, notarization, app-store/Steam/itch publishing, mobile/console/desktop
export, dependency mutation, CI/workflow mutation, command bridge, network/install
command, or credential use. Browser/dashboard/Studio surfaces remain read-only.
No production-ready export, secure distribution, commercial release readiness,
multi-platform export parity, or current Godot replacement claim is made.

## Known gaps

- The demo reuses the single web-static fixture profile; multi-platform export is
  out of scope for v1.
- Verification is the existing allowlisted local report; it is evidence, not a
  release gate.

## Governance

- #1 remains open.
- #23 remains open.

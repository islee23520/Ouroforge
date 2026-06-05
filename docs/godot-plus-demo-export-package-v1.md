# Godot-Plus Demo Exported Playable Package v1

Issue: #791
Status: **GPD12.13 export-package contract.** This document records the local web
export/package verification for the Godot-Plus Demonstration Game v1 vertical
slice (Signal Gate / Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on the #781 export
profile scaffold and the existing Build / Export / Packaging contracts. The legacy
`examples/godot-plus-demo-v1/` tree is superseded and is not used. #1 and #23
remain open.

## Export profile

The demo export profile `export/export-profile.json` (`export-profile-v1`,
added in #781) targets `web-local` with output under the ignored `dist/` staging
root and references the demo verification scenarios. #791 exercises the full
export pipeline over this profile.

## Export pipeline (Rust-trusted, local-only)

The contract test `godot_plus_demo_export_package_contract` walks the existing
pipeline over the demo profile and asserts a pass verdict at every stage:

| Stage | Contract | Assertion |
| --- | --- | --- |
| Plan | `ExportPlan::from_profile` | the profile plans |
| Local web bundle | `assemble_web_bundle` (temp staging) | `index.html` + `runtime/bootstrap.js` present |
| Asset manifest | `build_asset_manifest` | non-empty, every entry `sha256:`-hashed |
| Fingerprint / checksums | `build_fingerprint` | artifact checksums recorded |
| Runtime probe | `check_bundle_probe` (dev + packaged) | probe global present, passes both modes |
| Verification | `verify_export_bundle` | verification passes |
| Evidence bundle | `build_export_evidence` | verdict = `pass` |

The package is assembled under an ignored temp staging directory and removed; no
package bytes are committed. The evidence read model exposes no `publishCommand`,
`deployCommand`, or `applyCommand`.

## No publish / deploy / sign / upload

The export target is `web-local` only (the export profile schema blocks
mobile/console/app-store/steam/itch/hosted-deploy/signed-release/ci-release
targets). The flow generates and verifies a local package and records evidence; it
performs no publish, deploy, sign, or upload.

## Verification

```bash
cargo test -p ouroforge-core --test godot_plus_demo_export_package_contract
```

Generated package/run/dashboard artifacts remain local and untracked; any
generated export run id belongs in PR/issue evidence, not source control.

## Boundaries

The export reuses the existing Rust-trusted Build/Export/Packaging contracts and
adds no publish/deploy/sign/upload, native/mobile/console/store export, committed
generated output, trusted browser write, or full Godot parity / replacement /
production-ready / commercial-release claim. #1 and #23 remain open.

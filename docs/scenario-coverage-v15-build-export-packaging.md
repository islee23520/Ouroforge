# Scenario Coverage v15: Build Export Packaging Regression Suite

Issue: #735 — Scenario Coverage v15: Build Export Packaging Regression Suite.

Scenario Coverage v15 locks the Build / Export / Packaging v1 milestone as a
local-only, evidence-backed export/package regression suite. It verifies that an
allowed web export can be planned, staged, packaged, hashed, probe-checked, and
inspected as read-only evidence while unsupported release, publish, upload, sign,
desktop/mobile/store, generated-state, and checksum-drift requests fail closed.

## Regression matrix

| Scenario id | Coverage | Evidence / focused check | Expected result |
| --- | --- | --- | --- |
| `BEP15.success-local-web-export` | Valid web export profile and bundle assembly. | `ExportPlan::from_profile_json` + `assemble_web_bundle` over `examples/export-bundle-v1/export-profile.fixture.json`. | Local bundle is staged under ignored generated state and contains `index.html`, `styles.css`, runtime bootstrap, entry scene, and only declared assets. |
| `BEP15.success-asset-manifest` | Valid asset manifest and path rewriting. | `build_asset_manifest` and `AssetManifest::from_json_str`. | Source and output refs rewrite deterministically to packaged URLs with `sha256:<64 hex>` content hashes. |
| `BEP15.success-probe-preserved` | Runtime probe compatibility survives packaging. | `check_bundle_probe` in packaged and dev probe modes. | `window.__OUROFORGE__` and required probe methods remain present; no command bridge is introduced. |
| `BEP15.success-verification-pass` | Verification steps remain local and allowlisted. | Export plan verification step ids (`load-without-console-errors`, `runtime-probe-compatibility`, and scenario smokes). | Verification is evidence text/metadata only; no hidden runner, network/install command, or credentialed action is created. |
| `BEP15.success-evidence-bundle` | Export evidence bundle expectations are fixture-scoped. | `examples/build-export-packaging-regression-v15/coverage-matrix.fixture.json`. | Evidence bundle refs are represented as local fixture evidence only and do not authorize publishing or trusted writes. |
| `BEP15.success-read-only-inspection` | Dashboard/Studio inspection remains read-only. | Existing dashboard/cockpit Node smokes plus this matrix. | Browser surfaces may inspect exported JSON only; no accept/apply/write/merge/publish/deploy controls. |
| `BEP15.block-missing-asset` | Missing declared asset roots fail closed. | `examples/export-asset-manifest-v1/missing-asset-profile.fixture.json`. | Diagnostic names the missing asset root; no partial package is promoted. |
| `BEP15.block-unsafe-path` | Traversal, absolute paths, and blocked generated/source prefixes fail closed. | `examples/export-profile-v1/invalid/path-traversal-output.json`, forged plan roots, and asset-manifest invalid fixtures. | Diagnostic names unsafe path/traversal/blocked prefix before trusted writes. |
| `BEP15.block-publish-target` | Publish/release targets are blocked. | Dynamic profile target checks for `steam`, `itch`, `hosted-deploy`, `signed-release`, and `ci-release`. | Diagnostic references blocked publish/deploy/sign/upload scope. |
| `BEP15.block-missing-probe` | Probe drift is visible and blocks compatibility claims. | `examples/export-probe-v1/invalid/no-probe-global-bootstrap.js` and `missing-getevents-bootstrap.js`. | Probe report is failed with missing global/methods; it is not silently treated as compatible. |
| `BEP15.block-dirty-generated-output` | Generated outputs stay ignored or fixture-scoped. | `export_staging` policy and fixture `generatedStatePolicy`. | Staging lives under `target/ouroforge/exports/<run-id>/`; generated exports, checksums, logs, screenshots, temp servers, and local tool state remain untracked unless fixture-scoped. |
| `BEP15.block-checksum-mismatch` | Checksum/hash drift is actionable. | Built manifest hash compared with fixture/declared hash shape. | Mismatched or malformed hashes are visible as blocked evidence; no nondeterministic checksum is accepted as a promotion claim. |
| `BEP15.block-desktop-mobile-store-target` | Desktop/mobile/store targets remain design-gated or blocked. | Dynamic profile target checks for `desktop-wrapper`, `mobile`, `console`, and `app-store`. | Target validation rejects them before package planning or staging. |

## Fixture policy

The fixture under `examples/build-export-packaging-regression-v15/` is
source-like scenario coverage metadata. It is intentionally small, deterministic,
and tracked. Generated export/package artifacts remain under ignored roots such
as `target/ouroforge/exports/`, `dist/`, `build/`, or `runs/` unless a future
issue explicitly scopes a small source-like fixture.

## Non-goals and guardrails

Scenario Coverage v15 does not authorize public release, no publish/publishing, deployment, hosting,
upload, signing, notarization, app-store/Steam/itch publishing, mobile/console
export, desktop wrapper export, credential use, CI release automation, arbitrary
command execution, dependency installation, browser trusted writes, command
bridges, production-ready export claims, commercial distribution readiness, or a
Godot replacement claim.

Rust/local validation owns export profiles, package planning, artifact writing,
staging policy, checksums, probe checks, evidence, trusted persistence, and CLI
behavior. Browser/dashboard/Studio surfaces may only display escaped exported
JSON as read-only inspection data.

#1 remains open. #23 remains open.

## Verification hooks

Focused local checks for this scenario coverage are:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v15_build_export_packaging
cargo test -p ouroforge-core --test export_profile_contract
cargo test -p ouroforge-core --test export_plan_contract
cargo test -p ouroforge-core --test export_bundle_contract
cargo test -p ouroforge-core --test export_asset_manifest_contract
cargo test -p ouroforge-core --test export_probe_check_contract
cargo test -p ouroforge-core --test export_staging_contract
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

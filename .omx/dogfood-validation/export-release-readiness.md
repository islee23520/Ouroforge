# Dogfood B4 Export / Release Readiness Handoff

## Metadata

- Blocker: B4 — export/release readiness evidence is not durable on origin/main
- Report version: `dogfood-export-release-readiness-v1`
- Demo identity: `collect-and-exit-local-rc-candidate`
- Branch: `dogfood/b4-export-readiness-20260610002739`
- Base: `origin/main` after PR #2336 merge (`90c13a7f`)
- Source basis: `examples/playable-demo-v2/collect-and-exit/`
- Readiness classification: `local-manual-rc-evidence-only`
- Issue state evidence: #1 OPEN; #23 OPEN.

## Purpose

This handoff makes B4 durable by tracking local/manual export-readiness evidence in the repository. It records what is ready to inspect, what is intentionally not retained, and what remains blocked before any real release. It is coordination/evidence only: no product export implementation, release automation, signing, upload, publishing, credential flow, hosted service, native/store package, or Steam integration is added.

## Merged prerequisite evidence

| Blocker | PR | Origin-main artifact | Status for B4 |
| --- | --- | --- | --- |
| B1 claim coverage | #2334 MERGED | `.omx/dogfood-validation/claim-coverage-matrix.md` | Present; identifies B4 as an unresolved release-readiness evidence gap. |
| B2 compact demo spec | #2335 MERGED | `.omx/dogfood-validation/demo-game-spec.md` | Present; defines `collect-and-exit-local-rc-candidate` and local/manual export expectations. |
| B3 pipeline dry-run | #2336 MERGED | `.omx/dogfood-validation/pipeline-dry-run.md` | Present; provides failed-classified pipeline evidence and generated run refs, not production readiness. |

## Local/manual package evidence

| Evidence item | Path / reference | Verdict | Notes |
| --- | --- | --- | --- |
| Demo export profile | `examples/playable-demo-v2/collect-and-exit/export/export-profile.json` | pass | Profile is `web-local`, points to the Collect-and-Exit entry scene, and states no publish/deploy/sign/upload. |
| Package metadata | `examples/playable-demo-v2/collect-and-exit/export/package-metadata.json` | pass | Metadata is local fixture/package identity only; it contains no signing key, upload endpoint, registry target, or credential field. |
| Build/export packaging policy | `docs/build-export-packaging-demo-v1.md`; `docs/export-staging-policy-v1.md`; `docs/release-artifact-policy-v1.md` | pass | Existing docs keep generated export outputs under ignored local staging and deny publication authority. |
| Pipeline dry-run package-adjacent evidence | `.omx/dogfood-validation/pipeline-dry-run.md` | failed-classified | B3 produced local generated run/dashboard evidence and a classified scenario failure; B4 inherits the evidence refs without recasting them as green readiness. |
| Retained RC artifact | generated local artifact path | explicit gap | No release-candidate package artifact is durably retained on `origin/main`; future lanes must provide a run id or retained generated handoff before claiming package availability. |

## Pipeline-to-package provenance join

B4 joins the merged B1/B2/B3 evidence to the package fixtures as follows:

1. Claim basis: B1 matrix keeps #1/#23 open, guards forbidden scope, and identifies export readiness as an evidence gap.
2. Demo basis: B2 spec selects the Collect-and-Exit fixture and explicitly limits export expectations to local/manual evidence.
3. Pipeline basis: B3 dry-run records seed/project validation, run ids, evaluator verdict, journal, mutation proposal, replay comparison, dashboard export, generated-state boundaries, and a failed-classified scenario mismatch.
4. Package basis: the B4 export profile and package metadata provide the local package identity to inspect, while this report records that no retained RC package artifact is present yet.

This is a provenance handoff, not a promotion gate. A verifier may accept B4 when the tracked report/status/smoke prove the evidence boundary is durable; it must not treat B4 as a shippable package or store/public release approval.

## Package probe and performance evidence

- Export target probe boundary: `examples/playable-demo-v2/collect-and-exit/export/export-profile.json` uses `runtimeProbeMode: preserve` and `exportTarget: web-local`.
- Fixture performance boundary: `examples/godot-plus-demo-performance-v794/performance-budget-smoke.test.cjs` checks Collect-and-Exit frame-budget, load-time, console/crash-free, QA/playtest, export-verification, wording, and generated-state rows.
- Packaging demo boundary: `docs/build-export-packaging-demo-v1.md` describes local web-bundle, asset-manifest, fingerprint/checksum, probe-check, verification, and evidence-bundle stages while denying publish/deploy/sign/upload behavior.
- Current B4 validator: `examples/dogfood-export-release-readiness-v1/export-release-readiness-smoke.test.cjs` checks this handoff, status JSON, package fixture fields, guardrails, and tracked prerequisite artifacts.

## Generated-state cleanup and retention boundary

- Tracked B4 artifacts are limited to this report, `.omx/dogfood-validation/export-release-readiness.status.json`, the B4 smoke validator, and the executor status/report update.
- Generated package outputs, dashboards, screenshots, run folders, bundles, checksums, and verification logs remain under ignored/generated roots such as `/target/`, `/dist/`, or temporary run directories.
- The B3 generated work directory is a prior local verifier reference, not an origin-main artifact and not a release candidate package.
- No generated package artifact is committed by B4.

## Readiness verdict

B4 verdict: `pass-local-manual-evidence-gate`.

Accepted meaning: origin/main can durably show the export/readiness evidence boundary, package fixture refs, explicit retained-artifact gap, package probe/performance evidence refs, and no-publish guardrails.

Rejected meaning: this does not approve a public release, production/store readiness, signed artifact, hosted demo, Steam depot, upload, release automation, credentialed flow, native/mobile/console export, or commercial availability.

## Verification commands

```bash
node --test examples/dogfood-export-release-readiness-v1/export-release-readiness-smoke.test.cjs
node --test examples/godot-plus-demo-performance-v794/performance-budget-smoke.test.cjs
cargo test -p ouroforge-core --test build_export_packaging_demo --jobs 2
git diff --check origin/main...HEAD
```

## Non-goals and guardrails

- #1 and #23 remain open.
- Era Q M102–M106 remain deferred/non-goal; no full-3D implementation is added.
- No release automation, signing, notarization, upload, publishing, Steam depot flow, credential flow, hosted/cloud/multi-user behavior, trusted browser/source writes, auto-port, or foreign-runtime embedding is added.
- No production-ready, store-ready, commercial release, native export, Godot replacement, full Godot parity, or shipped-game maturity claim is made.

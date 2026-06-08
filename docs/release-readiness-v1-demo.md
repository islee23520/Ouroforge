# Release Readiness Demo v1 (#1872)

This fixture-scoped demo shows the Era J release-readiness path without network access or a live browser:

1. Recompute a planted dominant-build finding from `examples/engine-builder-balance-v1/dominant-build/dominant-build.fixture.json`.
2. Surface the `loop-engine` balance co-pilot recommendation from `examples/balance-copilot-v1/recommendation-set-v1.json`.
3. Record the human approval/tweak from `examples/balance-copilot-v1/human-approval-v1.json`.
4. Re-verify against `examples/engine-builder-balance-v1/dominant-build/balanced-builds.fixture.json` and confirm `examples/balance-copilot-v1/reverification-v1.json` reports `reverified-improved`.
5. Compose the release-readiness bundle from `examples/release-readiness-v1/complete-ready-bundle.input.json` and confirm the bundle is mechanically `ready` while still requiring a separate human go/no-go record.
6. Record the read-only human go/no-go evidence from `examples/release-readiness-v1/go-no-go.input.json` and compare it with `examples/release-readiness-v1/go-no-go.fixture.json`.

## Reproduce from a fresh clone

```bash
cargo test -p ouroforge-core --test release_readiness_demo_contract --jobs 2
```

The smoke test is deterministic and uses committed fixtures only. It does not start a browser, contact a network service, generate untracked release artifacts, sign builds, upload to Steam, or publish anything.

## Boundaries

- Rust/local owns validation and evidence composition.
- Browser/Studio surfaces are read-only inspection surfaces for this demo.
- The balance recommendation is advisory, human-approved, re-verified, and never auto-applied.
- The release-readiness bundle is composition evidence, not a release button.
- The go/no-go record is human-owned and read-only; it grants no release authority, auto-merge authority, trusted-write authority, signing authority, upload authority, or market-demand claim.
- The demo asserts mechanical gate states and human records only. It does not compute or assert a fun score.
- Generated runs/artifacts remain untracked unless fixture-scoped.
- #1 and #23 remain open.

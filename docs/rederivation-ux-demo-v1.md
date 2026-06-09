# Re-Derivation UX Demo v1

Era R M113 demo evidence for the Studio re-derivation UX and human intent/feel
escalation surface.

The runnable demo module is `OuroforgeExecutor.ReDerivationUXDemo` under
`studio/executor`. It renders a small source-project/open-text fixture through
the read-only re-derivation UX surface, emits a fidelity summary, and routes
intent/feel items to Ring 2 human review via gated `ouroforge` CLI preview
commands.

## Evidence

- Fixture: `examples/rederivation-ux-demo-v1/fidelity-report.fixture.json`
- Surface: `studio/executor/lib/ouroforge_executor/rederivation_ux.ex`
- Demo: `studio/executor/lib/ouroforge_executor/rederivation_ux_demo.ex`
- Test: `studio/executor/test/ouroforge_executor/rederivation_ux_demo_test.exs`

## Assertions

- Skeleton/content import is best-effort and clean-room.
- Logic is re-derived and verified from captured oracle evidence.
- No unit is claimed `ported` or `fully ported` without oracle-gated evidence.
- 3D determinism uses state-hash primary with SSIM/pixel-diff render evidence as
  secondary only.
- Studio has no trusted-write authority and no artifact semantics; every
  human-facing write is a gated CLI/review preview.
- #1 and #23 remain open.

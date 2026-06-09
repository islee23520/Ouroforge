# Deterministic Re-Expression Engine Demo v1

This fixture-backed demo records Era R M110 deterministic re-expression end to
end without claiming that a game or unit has been ported. It uses the M110 Rust
data-plane module to turn captured oracle evidence into a deterministic
`behavior_runtime` draft candidate, source-apply/review-gate handoff metadata,
and an M111 verification handoff.

## What the demo proves

- Skeleton import is best-effort source-project/open-text context only.
- Logic is re-derived clean-room from captured intent/oracle evidence; legacy
  source is not copied or translated.
- A unit with captured oracle evidence becomes a candidate behavior draft and
  verification handoff, not a finished port claim.
- A unit without an oracle stays Yellow with an explicit re-derivation task.
- State-hash determinism is primary: identical oracle state hashes reproduce the
  same report digest, and changed state hashes change the digest.
- Every write remains downstream `source-apply` / review-gated and rollback
  tracked; the demo writes no trusted artifacts itself.
- Studio/Elixir has no trusted-write or artifact-semantics authority here.

## Evidence

- Manifest: `examples/deterministic-reexpression-demo-v1/manifest.fixture.json`
- Contract: `docs/deterministic-re-expression-engine-contract-v1.md`
- Implementation: `crates/ouroforge-core/src/deterministic_reexpression.rs`
- Regression: `crates/ouroforge-core/tests/deterministic_reexpression_demo.rs`
- Seed smoke target: `seeds/migration-demo.yaml`

## Fidelity summary

| Unit state | Expected grade | Outcome |
| --- | --- | --- |
| Captured oracle + deterministic state hash | Green | Candidate deterministic behavior draft + M111 verification handoff; `ported_claim_allowed=false`. |
| Missing oracle | Yellow | No draft; explicit `capture_or_repair_oracle_before_reexpression` task. |
| Blocked/decompiled/live-runtime provenance | Red | Rejected/deferred by the M110 engine before draft generation. |

## Verification

```bash
cargo build --workspace --jobs 2
cargo test -p ouroforge-core --test deterministic_reexpression_demo -- --nocapture
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2 || true
echo "demo: skeleton imports best-effort; logic re-derived+verified; no auto-port claim"
```

#1 and #23 remain open governance anchors.

# Tacit-Knowledge Interrogation and Oracle Capture Demo v1

This fixture-backed demo exercises Era R M109 over a tiny source-owned Unity-style
C# unit. It demonstrates the clean-room path from M108 behavioral-unit candidate
to M109 interrogation answers and deterministic oracle specs.

## Demo Flow

1. Analyze source-project/open-text C# as read-only legacy logic evidence.
2. Extract behavioral units with oracle-missing status and no port claim.
3. Ask a required source-independent intent question for one unit.
4. Capture a human provenance answer and a lawful observed behavior trace.
5. Synthesize a Rust-owned oracle spec with a 2D bit-exact state hash.
6. Emit an honest fidelity summary: captured units are oracle-ready for later
   re-expression; oracle-less units remain explicitly flagged and not ported.

## Fidelity Summary

- One unit has captured intent and deterministic oracle evidence.
- Two sibling units intentionally remain oracle-missing to prove gaps stay visible.
- No unit is called ported; every `portedClaimAllowed` flag remains false.
- The oracle uses state-hash primary evidence. No source physics or engine runtime
  is reproduced; downstream re-expression must re-simulate behavior natively.
- The demo does not touch Studio. Elixir/Phoenix retains no trusted-write or
  artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Verification

```bash
cargo test -p ouroforge-core tacit_oracle_capture_demo -- --nocapture
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2
```

The scripted migration demo is a separate loop smoke; this document and fixture
prove the M109 oracle-capture evidence shape itself.

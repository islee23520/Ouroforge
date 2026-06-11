# Scenario Coverage v112 — Live Preview Loop v1

Scenario Coverage v112 locks Era X M131 behavior (#2518-#2521, parent SSOT
#2517) for the Live Preview Loop: the `ouroforge preview serve` validation
process, the runtime preview channel, transcript capture with fidelity
replay, draft export, and the first product-observed latency audit.

## Boundary

- The #2517 Q1 ratification (Model B) is the design authority: preview
  validation shares apply-preflight code paths; the parity golden suite is a
  permanent regression gate.
- Preview-channel writes are in-memory only; nothing reaches the trusted
  worktree except via existing review-gated CLI apply. No browser trusted
  writes, no command bridge, no auto-apply.
- Latency budgets (<1 s tweak / <5 s reload) were record-only this first
  cycle per the #2517 Q2 resolution; hard gating begins the next cycle.
- Generated audit bundles stay under ignored `runs/` roots.
- #1 and #23 remain open.

## Coverage ledger

| Row | Locked behavior |
| --- | --- |
| `v112-validation-parity` | The parity golden suite (`preview_session_parity.rs`) locks byte-identical outcomes between preview validation and apply preflight across every allowlisted scene-edit path. |
| `v112-loopback-only` | `PreviewServer::bind` rejects non-loopback hosts; the runtime client rejects non-loopback `ws://` URLs with a typed diagnostic. |
| `v112-serve-write-free` | The serve path performs no filesystem writes; transcript persistence belongs to the local CLI client (`preview transcript --output`). |
| `v112-channel-idle-determinism` | With no preview connection, runtime behavior is untouched (`preview-channel-client.test.cjs` idle state; components-v2 and e2e smokes green). |
| `v112-mechanical-application` | The JS path list is pinned to the Rust allowlist; deltas apply all-or-nothing with no interpretation (`preview-channel.test.cjs`). |
| `v112-transcript-fidelity` | `replay_preview_transcript` fails closed on stale base, non-byte-identical deltas, final-state divergence, and digest mismatch (`preview_transcript_fidelity.rs`). |
| `v112-no-auto-apply` | Draft export emits the existing `visual-edit-draft-v1` artifact with no pre-filled review gate; review/apply authority is unchanged. |
| `v112-latency-evidence-shape` | The audit records CDP-primary latency (p50/p95) with in-page timestamps secondary, record-only budgets, in an M116-layout bundle (`tools/preview-loop-audit/audit.mjs`). |
| `v112-audit-linkage` | The recorded product-observed run is summarized in `docs/preview-loop-audit-2521.md` with the review→apply→rerun leg recorded as an explicit gap owned by M132.2/M132.3. |

## Source artifacts

- Contract doc: `docs/preview-session-v1.md`
- Audit summary: `docs/preview-loop-audit-2521.md`
- Rust suites: `crates/ouroforge-core/tests/preview_session_parity.rs`, `preview_session_server_lifecycle.rs`, `preview_session_channel.rs`, `preview_transcript_fidelity.rs`
- JS suites: `examples/game-runtime/preview-channel.test.cjs`, `preview-channel-client.test.cjs`
- Audit driver: `tools/preview-loop-audit/audit.mjs`
- Smoke validator: `scripts/scenario-coverage-v112-live-preview-loop.test.cjs`

## Verification

```bash
node scripts/scenario-coverage-v112-live-preview-loop.test.cjs
CARGO_TARGET_DIR=<isolated> cargo test -p ouroforge-core --test preview_session_parity \
  --test preview_session_server_lifecycle --test preview_session_channel \
  --test preview_transcript_fidelity
node examples/game-runtime/preview-channel.test.cjs
node examples/game-runtime/preview-channel-client.test.cjs
```

## Completion statement

M131 is complete but bounded: #2518/#2519/#2520 closed contract-complete and
#2521 closed product-observed complete for the live preview latency loop and
transcript→draft export handoff claims only. These artifacts do not authorize
trusted browser writes, command bridges, auto-apply, editor-maturity claims
(M132), asset-workflow claims (M133), evaluator-depth claims (M134),
production readiness, or Godot parity/replacement positioning.

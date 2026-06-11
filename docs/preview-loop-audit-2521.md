# Live Preview Loop Audit (M131.4, #2521)

Era X parent SSOT: #2517. Completion semantics: `docs/product-observed-completion.md`.

This document indexes the first product-observed evidence bundle for the Live
Preview Loop (M131): a live browser session on the collect-and-exit scene,
driven end-to-end through `ouroforge preview serve`, the runtime preview
channel, transcript capture, and draft export.

## How to reproduce

```bash
cargo build -p ouroforge-cli
node tools/preview-loop-audit/audit.mjs --bin <target>/debug/ouroforge --run-id <id>
```

The driver starts a loopback static server and `preview serve`, launches
headless Chrome, loads `examples/game-runtime/?scene=...&preview=ws://...`,
applies 12 validated parameter tweaks and 3 scene reloads, fetches the
transcript, exports the draft, runs `edit draft-preview`, and writes an
M116-layout bundle under ignored `runs/preview-loop-audit/<run-id>/`
(manifest, console/frame/world JSONL, events, interactions, latency stats,
transcript, draft, preflight transactions, screenshots, verdict).

## Recorded result (run `audit-2521-live`, 2026-06-12)

Generated evidence stays under ignored `runs/`; the figures below are
recorded here as the stable summary, per the generated-state boundary.

| Measure | Result | Budget (#2517 Q2, record-only) |
| --- | --- | --- |
| Parameter tweaks applied | 12/12 | — |
| Tweak latency p50 / p95 | 2.5 ms / 8.7 ms | < 1000 ms (12/12 within) |
| Scene reloads | 3/3 | — |
| Reload latency p50 / p95 | 2.0 ms / 15.6 ms | < 5000 ms (3/3 within) |
| Runtime diagnostics | none recorded | must be none or owned |
| Transcript | 17 entries, semantic digest recorded | fidelity-replayable |
| Draft export | 2 net edits, passed `edit draft-preview` preflight | existing artifact |

Latency evidence is CDP-primary (harness clock around `POST /intent` until
the page's applied counter advances, observed via `Runtime.evaluate`);
in-page `performance.now` receivedAt/appliedAt timestamps are secondary
corroboration. Budgets were record-only this cycle; hard gating begins the
next cycle per the #2517 Q2 resolution.

## Boundary and recorded gaps

- The review → apply → rerun-comparison leg beyond draft preflight is **not**
  exercised by this automated audit. That leg was product-observed under M130
  (#2392); its Era X re-exercise through the Studio surface is owned by
  M132.2/M132.3 (#2524/#2525). Recorded as a gap, not a pass.
- This audit does not claim editor usability (M132), asset fidelity (M133),
  or evaluator-depth (M134) coverage; it claims exactly the live preview
  latency loop and the transcript→draft export handoff.
- No trusted browser writes, command bridges, or auto-apply occurred; the
  serve path performed no filesystem writes (bundle persistence is the
  harness/CLI client's).

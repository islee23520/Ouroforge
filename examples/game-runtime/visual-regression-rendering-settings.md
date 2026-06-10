# Visual Regression Rendering Settings (M120.3)

Issue: #2361

Committed pixel baselines are not introduced in this PR. If a future fixture-scoped
baseline is committed, it must live under `examples/game-runtime/**` and document
these settings next to the baseline:

- Viewport: 1280 x 900 CSS pixels.
- Device scale factor: 1.
- Color scheme: dark.
- Font policy: system UI plus `ui-monospace` fallbacks as declared in `index.html`.
- Source URL: `examples/game-runtime/index.html` served from a local static HTTP root.
- Screenshot naming: `screenshots/state-<name>.png`, where `<name>` is one of
  `start`, `key-collected`, `gate-open`, `win`, `fail`, `paused`, or `restarted`.
- Generated run root: ignored `runs/session-f-2361/` or another ignored `runs/`
  bundle. Do not commit generated screenshots by default.

The current implementation uses report-based comparison. It materializes
`runs/session-f-2361/visual-regression-report.json` and
`runs/session-f-2361/screenshot-manifest.json` only when
`OUROFORGE_WRITE_RUNS=1` is set.

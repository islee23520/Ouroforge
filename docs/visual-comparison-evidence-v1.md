# Visual Comparison Evidence v1

Status: **QA14.7.1 visual comparison artifact** for issue #688.

`visual-comparison-evidence-v1` records deterministic screenshot comparison evidence for QA regressions. It stores before/after screenshot refs, optional metadata refs, dimensions/formats, pixel-diff summary metrics, changed regions, thresholds, missing/unsupported/blocked states, evidence refs, and conservative guardrails.

Supported outcomes are `unchanged`, `changed`, `missing_screenshot`, `malformed_screenshot`, `mismatched_dimensions`, `unsupported`, and `blocked`.

Fixtures:

- `examples/visual-comparison-evidence-v1/visual-comparison-unchanged.sample.json`
- `examples/visual-comparison-evidence-v1/visual-comparison-changed.sample.json`
- `examples/visual-comparison-evidence-v1/invalid/missing-thresholds.json`
- `examples/visual-comparison-evidence-v1/invalid/missing-screenshot-state.json`
- `examples/visual-comparison-evidence-v1/invalid/malformed-screenshot-ref.json`

Guardrails:

- Visual comparison evidence is deterministic metadata/pixel evidence only.
- No aesthetic, subjective quality, fun, production safety, accessibility compliance, market-readiness, or shipped-game claim is implied.
- Browser/dashboard/Studio surfaces remain read-only and do not compute trusted diffs, write files, execute commands, or apply fixes.

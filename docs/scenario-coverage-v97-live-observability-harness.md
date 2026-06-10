# Scenario Coverage v97 — Live Dogfood Observability Harness

Scenario Coverage v97 locks Era S M116 behavior for generated live observability bundles, local browser capture, runtime state sampling/replay, and deterministic verdict generation. The Rust `crates/ouroforge-observability` crate owns schema validation and verdict semantics. JS/Node runner code is CDP capture glue only.

## Boundary

- Generated live evidence uses `runs/live-observability/<run-id>/` or an explicitly temporary/generated root.
- `manifest.json` keeps the canonical schema version, run id, target URL, run kind, retry attempts, artifact inventory, and sampled API key usage.
- Target URLs fail closed unless they are `http://127.0.0.1:<port>/...` or `http://localhost:<port>/...`.
- The JS runner must invoke the Rust validator; it must not implement a second validator.
- Runtime sampling may call only existing `window.__OUROFORGE__` keys and records those keys in the manifest.
- Missing runtime keys produce `unsupported-target` diagnostics; page JS is not patched or extended by the harness.
- Replay evidence records objective flag sequences and screenshot checkpoints.
- Verdicts reference M115.3 `po-check-*` ids and concrete artifact paths.
- `contract-pass` and `product-observed FAIL` can coexist; diagnostics and usability gaps are preserved.
- #1 and #23 remain open.

## Coverage ledger

| Row | Locked behavior |
| --- | --- |
| `v97-schema-required-artifacts` | Validator rejects bundles missing required artifacts such as `manifest.json` or `verdict.md`. |
| `v97-local-url-fail-closed` | Runner rejects non-local, non-HTTP, or portless target URLs before browser launch. |
| `v97-single-rust-validator` | Runner validation goes through `crates/ouroforge-observability`; JS remains capture glue. |
| `v97-sampler-existing-api-keys-only` | Manifest records existing `window.__OUROFORGE__` keys used by sampler/replay. |
| `v97-unsupported-target-diagnostic` | Missing runtime API keys are recorded as diagnostics, not fixed by page injection. |
| `v97-replay-objective-progression` | Collect-and-exit replay records start/after-key/after-exit flags and reaches `exit_reached=true`. |
| `v97-replay-determinism-or-flake` | Two consecutive collect-and-exit replay runs produce identical objective flag sequences or record flake evidence. |
| `v97-verdict-checklist-trace` | Generated verdict references all M115.3 `po-check-*` item ids and concrete artifact paths. |
| `v97-verdict-deterministic` | Same bundle input renders byte-identical verdict output except the generation timestamp line. |
| `v97-contract-pass-product-fail-golden` | Current collect-and-exit golden example is `contract-pass` / `product-observed FAIL` when diagnostics remain. |
| `v97-generated-state-clean` | Live run bundles stay ignored/generated unless explicitly fixture-scoped. |

## Verification

```bash
CARGO_TARGET_DIR=/Users/jh0927/.cache/ouroforge-targets/session-b-2346 \
  cargo test --manifest-path crates/ouroforge-observability/Cargo.toml
node --check tools/live-observability-runner/runner.mjs
python3 - <<'PY'
from pathlib import Path
text = Path('docs/scenario-coverage-v97-live-observability-harness.md').read_text()
required = [
    'v97-schema-required-artifacts',
    'v97-local-url-fail-closed',
    'v97-single-rust-validator',
    'v97-sampler-existing-api-keys-only',
    'v97-unsupported-target-diagnostic',
    'v97-replay-objective-progression',
    'v97-replay-determinism-or-flake',
    'v97-verdict-checklist-trace',
    'v97-verdict-deterministic',
    'v97-contract-pass-product-fail-golden',
    'v97-generated-state-clean',
]
missing = [row for row in required if row not in text]
if missing:
    raise SystemExit(f'missing v97 rows: {missing}')
PY
```

## Product-observed evidence from M116 closure

- #2347 produced and validated a live collect-and-exit bundle under `/tmp/ouroforge-live-test/collect-and-exit-2347b`.
- #2348 produced two consecutive validated replay bundles and compared their objective flag sequences.
- #2349 rendered the live bundle and the committed golden fixture as `contract-pass` / `product-observed FAIL` because the runtime diagnostic `missing_asset` remains visible.

Generated live bundles were not committed; the committed fixture is explicitly scoped under `crates/ouroforge-observability/fixtures/collect-and-exit-product-fail/` for deterministic renderer regression coverage.

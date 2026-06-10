# Scenario Coverage v96: Product-Observed Rebaseline

Scenario Coverage v96 locks the M115 governance regression suite. It ensures
future roadmap and closure language cannot convert contract fixtures into
practical engine/editor/gameplay usability claims without itemized
product-observed evidence.

## Coverage scope

- M115.1 completion semantics and closure template: `docs/product-observed-completion.md`.
- M115.2 historical classification ledger: `docs/roadmap/milestone-classification-ledger.json` and rendered `.md`.
- M115.3 product-observed closure checklist: `docs/product-observed-closure-checklist.md`.

## Regression scenarios

| Scenario id | Guarded regression | Required evidence | Expected result |
| --- | --- | --- | --- |
| `v96-semantics-classification-required` | An issue closes with unqualified `complete` for runtime/Studio/gameplay/asset/export/agentic-loop usability. | Closure comment cites `docs/product-observed-completion.md` and includes a classification line. | Fails unless the closure states `contract-complete` or `product-observed complete`. |
| `v96-checklist-stable-anchors` | Future checklist users cannot reference item-level evidence because anchors drift. | `docs/product-observed-closure-checklist.md` contains all `po-check-*` stable ids. | Fails if any stable id is missing. |
| `v96-fixture-only-fails-product-observed` | The collect-and-exit fixture is treated as practical engine usability because smoke tests exist. | Canonical FAIL table in `docs/product-observed-closure-checklist.md`. | Fails product-observed closure; remains valid contract evidence. |
| `v96-historical-ledger-required-fields` | Historical milestones lack classification, evidence refs, confidence, or gap rationale. | `docs/roadmap/milestone-ledger-validator.rs` over `docs/roadmap/milestone-classification-ledger.json`. | Fails validation on missing/empty required fields. |
| `v96-generated-state-audit-required` | Product evidence is generated but trusted source is polluted or unaudited. | `po-check-generated-state` item and closure template generated-state audit section. | Fails product-observed closure without an audit. |
| `v96-anchor-preservation` | M115 closure narrows or closes #1/#23. | Closure comments verify #1 and #23 state. | Fails if either anchor is closed by M115 work. |

## Verification commands

Run from the repository root:

```bash
python3 - <<'PY'
from pathlib import Path
checklist = Path('docs/product-observed-closure-checklist.md').read_text()
ids = [
    'po-check-live-url',
    'po-check-console',
    'po-check-screenshot',
    'po-check-replay',
    'po-check-world-sample',
    'po-check-event-sample',
    'po-check-frame-stats',
    'po-check-before-after',
    'po-check-verdict',
    'po-check-generated-state',
]
for item in ids:
    assert f'id="{item}"' in checklist, item
    assert f'`{item}`' in checklist, item
assert 'Canonical result: `product-observed FAIL`' in checklist
semantics = Path('docs/product-observed-completion.md').read_text()
assert 'Closure classification: contract-complete' in semantics
assert 'Closure classification: product-observed complete' in semantics
scenario = Path('docs/scenario-coverage-v96-product-observed-rebaseline.md').read_text()
for scenario_id in [
    'v96-semantics-classification-required',
    'v96-checklist-stable-anchors',
    'v96-fixture-only-fails-product-observed',
    'v96-historical-ledger-required-fields',
    'v96-generated-state-audit-required',
    'v96-anchor-preservation',
]:
    assert scenario_id in scenario
print('scenario coverage v96 anchors ok')
PY
rustc --edition=2021 --test docs/roadmap/milestone-ledger-validator.rs \
  -o /tmp/ouroforge-milestone-ledger-validator
/tmp/ouroforge-milestone-ledger-validator --nocapture
```

## Boundary

Scenario Coverage v96 is governance coverage only. It does not claim
product-observed practical engine, Studio, gameplay, asset-workflow, export, or
agentic-loop usability. It makes fixture-only evidence fail product-observed
closure while preserving contract-complete historical evidence.

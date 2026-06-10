# Scenario Coverage v111 — Production Usability Gate

Owner: #2394. Covered issues: #2391-#2394.

Coverage v111 locks the final M130 gate as an evidence-index regression suite.
It validates that #2391, #2392, #2393, and #2394 remain ordered, that each phase
links the required generated product-observed evidence, and that gaps are
recorded instead of greenwashed.

| Scenario | Locks | Failure mode |
| --- | --- | --- |
| `v111.ordered-m130-phases` | The gate contains exactly #2391 -> #2392 -> #2393 -> #2394. | Missing, duplicated, or reordered final-gate phase. |
| `v111.2391.workflow-screenshots-gaps` | New game workflow references transcript + screenshots and lists manual steps in the gap ledger. | Narrative-only first playable claim or hidden manual step. |
| `v111.2392.comparison-verdict` | Studio edit/rerun phase carries an `improvement` or honest `regression` comparison verdict from the #2380 lineage. | Before/after claim without comparison artifact. |
| `v111.2393.local-package-only` | Package/export phase references existing local web package/provenance refs and forbids new distribution scope. | Native/store/sign/upload/export-engine expansion. |
| `v111.2394.postmortem-governance` | Postmortem phase references #1/roadmap/backlog status and preserves #1/#23 open. | Marketing language or closed governance anchors. |

Run:

```sh
cargo test -p ouroforge-protocols scenario_coverage_v111 -- --nocapture
node examples/production-usability-gate-v111/gate-smoke.test.cjs
```

Generated browser screenshots and package outputs are not committed by this
suite. They are referenced as generated evidence paths under ignored roots and
must be attached/cited in PR and issue closure evidence when #2391-#2394 close.

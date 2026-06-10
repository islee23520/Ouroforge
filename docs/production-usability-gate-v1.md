# Production Usability Gate v1 (M130)

Issue range: #2391-#2394. Scenario Coverage owner: #2394 / v111.
Closure target: `product-observed complete` only when the referenced generated
workflow, screenshot, comparison, package, and postmortem evidence exists and no
blocking gap is hidden.

This gate closes the Product-Observed Rebaseline by indexing the final dogfood
workflow evidence. It is deliberately conservative: it proves a bounded local
Signal Gate Relay workflow, not commercial readiness, native export, hosted
collaboration, a secure sandbox, Godot parity, or a public release.

## Phase evidence index

| Issue | Product-observed claim | Required evidence | Honest gap rule |
| --- | --- | --- | --- |
| #2391 | Seed/GDD -> scaffold -> first playable can be followed without raw JSON as the primary workflow. | `examples/playable-demo-v2/signal-gate-dogfood/`, `runs/m130/2391-first-playable/workflow-transcript.md`, `runs/m130/2391-first-playable/screenshots/state-*.png`, `runs/m130/2391-first-playable/bundle.json`. | Every manual Studio/CLI/reviewer step must appear in the gate fixture `manualGaps` array or the issue remains product-observed fail. |
| #2392 | Studio edit / agent proposal / review / safe apply / live rerun produces an observable before/after result. | `examples/production-usability-gate-v111/studio-edit-proposal.fixture.json`, generated before/after screenshots, and `runs/m130/2392-studio-edit/comparison.json`. | The #2380 comparison verdict is authoritative: `improvement` passes; `regression` may close only as honestly recorded regression/fail, never as narrative pass. |
| #2393 | Existing local web package/export path can assemble an inspectable local package with provenance. | Existing export profile, `examples/production-usability-gate-v111/local-package-provenance.fixture.json`, generated checksums, package smoke. | No new distribution scope; native/store/sign/upload/public release remains out of scope. |
| #2394 | Final postmortem updates #1 status using M115.1 semantics and M115.2 ledger shape. | This document, v111 coverage, #1 final status update, milestone classification ledger update. | Any unresolved item is backlog, not marketing copy. #1 and #23 remain open. |

## Generated-state boundary

Generated workflow logs, screenshots, browser profiles, package outputs, and
smoke results stay in ignored roots such as `runs/`, `screenshots/`,
`browser-profiles/`, and `dist/` unless a later issue explicitly scopes a tiny
fixture. Tracked source contains only deterministic contracts, examples, and
indexes.

## Closure wording

Each #2391-#2394 closure comment must include:

```text
Closure classification: product-observed complete
Evidence: <workflow/comparison/package/postmortem refs>
Gaps/backlog: <none or explicit owner issue>
#1 state: OPEN verified
#23 state: OPEN verified
```

If any evidence is missing, use `product-observed fail` in the postmortem and do
not soften the failure to `contract-complete` for a practical usability claim.

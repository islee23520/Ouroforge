# Scenario Coverage v20: Evolve Depth Regression Suite

Issue: #1297  
Scope: Evolve Loop Depth v1 (#1290) under #1 Milestone 5.1.  
Status: deterministic fixture-scoped regression coverage; not generated run output, browser automation, subjective quality scoring, release-readiness evidence, or engine-replacement evidence.

Scenario Coverage v20 locks the Evolve Loop Depth proposal, selection, and comparison behavior with a deterministic matrix under `examples/evolve-loop-depth-v1/scenario-coverage-v20/` and the runner `examples/evolve-loop-depth-v1/scenario-coverage-v20-evolve-depth.test.cjs`.

## Matrix files

- `matrix.json` enumerates fixture refs, the four required gates, and the generated-state policy.
- `proposal-states/*.json` covers proposal citation, evidence-derived confidence, missing evidence, stale refs, and bounded-type enforcement.
- `class-to-bounded-type-mapping.json` records class-to-bounded-type mapping and keeps unsupported, unknown, and flaky classes backlog-only.
- `rerun-deltas/*.json` covers visual and semantic `fail_to_pass`, visual regression, and non-comparable states that must not fabricate deltas.
- `legacy-evolve-v0-proposal-comparison.golden.json` records the legacy evolve v0 golden shape that remains readable without v1 rationale or four-gate fields.

## Proposal and selection states

| Case | Expected state | Gate | Bounded type | Notes |
| --- | --- | --- | --- | --- |
| visual-linked-high-confidence-scene | linked proposal | visual | scene-only | Proposal cites visual evidence and stays scene bounded. |
| semantic-linked-high-confidence-data | linked proposal | semantic | data-only | Proposal cites runtime invariant evidence. |
| runtime-linked-high-confidence-data | linked proposal | runtime | data-only | Runtime assertion failures stay data bounded. |
| mechanical-linked-high-confidence-scenario | linked proposal | mechanical | scenario-only | Scenario failures stay scenario bounded. |
| visual-missing-evidence-no-proposal | missing-evidence | visual | backlog-only | Missing evidence does not fabricate a proposal. |
| semantic-stale-ref-no-proposal | stale-ref | semantic | backlog-only | Stale references remain visible and proposal-free. |
| bounded-type-enforcement-rejects-source-apply-for-visual | blocked-bounded-type | visual | backlog-only | Visual failures cannot escalate to source patch authority in this lane. |

## Class-to-bounded-type mapping

- `scenario_failed` → mechanical → scenario-only.
- `behavior_assertion_failed` → runtime → data-only.
- `visual_gate_failed` → visual → scene-only.
- `semantic_gate_failed` → semantic → data-only.
- Unsupported, unknown, and flaky evidence classes are backlog-only and cannot fabricate trusted mutation authority.

## Four-gate rerun deltas

Coverage asserts four-gate rerun deltas for:

- visual `fail_to_pass` with mechanical/runtime/semantic unchanged pass;
- semantic `fail_to_pass` with mechanical/runtime/visual unchanged pass;
- visual `pass_to_fail` regression;
- non-comparable different-scenario and missing-after-verdict states that must not invent gate deltas.

## Legacy compatibility

The legacy evolve v0 golden remains valid and intentionally omits v1-only fields such as `rationale.failing_gate_category`, `rationale.bounded_mutation_type`, `fourGateDeltas`, and `gateCategories`. This preserves backward compatibility for old proposal/comparison artifacts while newer v1 fixtures exercise the richer shapes.

## Reproduction

Run:

```bash
node examples/evolve-loop-depth-v1/scenario-coverage-v20-evolve-depth.test.cjs
```

The runner is deterministic and fixture-scoped. It requires no network access, live browser, timing assertions, local server, generated run directory, screenshot capture, or trusted write path.

## Boundaries

Scenario Coverage v20 asserts states and artifact shapes only. It does not assert fun, aesthetics, release readiness, shipped-game quality, compatibility-stable engine behavior, engine-replacement status, source mutation authority, browser trusted writes, command bridges, auto-fix, auto-apply, auto-merge, self-approval, reviewer bypass, or unattended integration.

Generated runs and artifacts remain ignored unless a future issue explicitly scopes them as deterministic checked-in fixtures. #1 and #23 remain open governance anchors.

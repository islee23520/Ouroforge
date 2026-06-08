# Real-Title Dogfooding Campaign Scope & Contract v1

Era L Milestone 68 fixes the autonomous self-validation campaign contract before
any real-title run begins. The dogfood subject is the existing Era I
engine-builder deckbuilder: the engine must build that real title through the
current Ouroforge chain, collect friction as evidence, and improve the engine
through the existing source-apply and gate pipeline. This is an engine
self-validation loop, not a new game pitch and not a new verification system.

## Goal

Fix the dogfooding contract before running it: the real title is the existing
Era I engine-builder deckbuilder, friction is evidence, and the whole-chain run
is recorded through the existing verdict / `journal.md` / `ledger.jsonl` /
loop-coverage attribution / evolve / source-apply / trust-gradient pipeline.

## Final Implementation Scope

- **Dogfood title:** `era-i-engine-builder-deckbuilder`, the existing Era I
  engine-builder deckbuilder. The real-title run target is
  `seeds/dogfood-deckbuilder.yaml` and the intended command is:

  ```bash
  cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2
  ```

- **Friction taxonomy:** `stall`, `retry`, `manual-intervention`,
  `budget-halt`, and `gate-fail`.
- **Whole-chain success:** concept -> trusted Rust/local artifact production or
  verification -> openchrome scenario run -> four gates plus design-integrity ->
  evidence-linked diagnosis/proposal -> source-apply/trust-gradient ->
  re-verified RC or high-risk queue.

No new game, new verification engine, hosted layer, live-ops surface, or new
persistent data plane is in scope.

## Success Criteria

- The dogfood title is specified as the existing Era I engine-builder
  deckbuilder.
- The friction taxonomy maps to existing ledger/journal/verdict/evidence refs.
- Whole-chain success is defined from concept to verified RC.
- The autonomous path requires zero human input.
- HIGH-RISK/source-affecting fixes are never auto-applied.
- #1 and #23 remain open governance anchors.

## Verification Method

```bash
set -euo pipefail
# Contract/scope present and reusing the existing evidence pipeline (no new verification engine)
grep -RIlqi "loop.coverage\|ledger\|journal\|verdict" docs/ || true
cargo build --workspace --jobs 2
```

The lane-level verification additionally runs `cargo fmt --all`,
`cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2`, and asserts that
no new data store or parallel verifier was introduced.

## Guardrails

- Autonomous by default: detect -> explain -> trace -> attribute -> propose ->
  re-verify -> apply/queue must run to completion without a human on the normal
  path.
- Engine source changes flow through source-apply, the four gates,
  design-integrity, openchrome re-run, and trust-gradient.
- HIGH-RISK/source-affecting changes keep only a thin human go/no-go tail and are
  never auto-applied.
- The campaign reuses openchrome, scenario verdicts, `journal.md`,
  `ledger.jsonl`, loop-coverage attribution, evolve, source-apply, and
  trust-gradient.
- This improves the engine/harness/pipeline, not the game itself.
- Rust kernel/evaluator/source-apply remains the data plane; the Elixir executor
  remains the unchanged control plane.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions;
  distributed/hosted/live-ops Layer 3 remains deferred.

## Explicit Non-Goals

- Creating a new game or replacing the existing Era I engine-builder
  deckbuilder dogfood subject.
- Creating a new verification engine, browser authority, durable database, data
  plane, or control-plane mutation path.
- Making human input mandatory in the autonomous path.
- Self-applying HIGH-RISK/source-affecting changes.
- Automating fun/taste or release go/no-go.
- Layer 3 distributed, hosted, cloud, or live-ops work.
- Closing, narrowing, or otherwise modifying #1 or #23.

## Implementation Approach

Represent the campaign as a Rust evaluator contract plus this documentation. The
Rust contract names the title, seed/run command, required loop stages, evidence
pipeline components, friction-to-ledger mappings, and autonomy/risk boundaries.
Downstream implementation must consume that contract instead of inventing a
parallel verification stack.

### Whole-chain success definition

A whole-chain success is a replayable record where:

1. the Era I deckbuilder seed/source inputs are bound to the run;
2. openchrome executes the title and records scenario evidence;
3. the evaluator records the four gates plus design-integrity verdicts;
4. `journal.md` and `ledger.jsonl` link friction, diagnosis, proposal,
   re-verification, and terminal state;
5. loop-coverage attribution identifies which artifacts were loop-produced,
   loop-verified, or manual;
6. source-apply and trust-gradient classify any engine fix;
7. eligible low-risk fixes are applied only after re-verification;
8. HIGH-RISK/source-affecting fixes are queued for human go/no-go;
9. the autonomous path reaches terminal success/failure/budget-halt with no
   required human input.

### Friction taxonomy and existing evidence mapping

| Friction kind | Meaning | Existing evidence mapping |
| --- | --- | --- |
| `stall` | The loop stops making progress before a terminal verdict. | `ledger.jsonl` event plus `journal.md` narrative and current verdict/run ref. |
| `retry` | A bounded retry is required after a failed run, flaky probe, or failed gate. | `ledger.jsonl` retry event, scenario verdict refs, openchrome evidence refs, and journal delta. |
| `manual-intervention` | A human action was requested or used outside the allowed high-risk go/no-go tail. | `ledger.jsonl` intervention event, `journal.md` explanation, and trust-gradient/risk refs; fails the autonomous-path claim. |
| `budget-halt` | The campaign reaches a configured budget/stop condition. | `ledger.jsonl` budget event, `journal.md` summary, loop-coverage refs, and last verdict. |
| `gate-fail` | One of the four gates or design-integrity fails. | Existing gate verdict, scenario verdict, openchrome evidence, `journal.md`, and ledger event. |

These are ledger event kinds and evidence references, not a new table or store.

## PR Decomposition

PR 1 adds the dogfooding contract doc and Rust evaluator contract only. Later Era
L PRs may add the real-title run, coverage v60+, diagnosis, fix proposal,
re-verification, optional read-only human observability, and governance maturity.

## Over-Engineering Checklist

- [x] No capability beyond the milestone scope.
- [x] No speculative ML/heuristics where evidence attribution suffices.
- [x] Reuses loop-coverage attribution, evaluator gates, and source-apply.
- [x] Does not build a parallel diagnosis/verification stack.
- [x] Keeps state in existing artifacts; no new persistent store.
- [x] Human surface remains minimal and non-blocking except the high-risk tail.

## Drift-Prevention Checklist

- [x] Autonomous-first: the loop runs to completion without a human on the normal
  path.
- [x] Engine fixes go through source-apply, gates, openchrome re-run, and
  trust-gradient.
- [x] HIGH-RISK/source-affecting fixes are queued and never auto-applied.
- [x] Existing evidence pipeline is reused; no new verification engine or data
  plane.
- [x] Improves the engine, not the game.
- [x] Rust remains the data plane; Elixir executor remains the control plane.
- [x] Fun/taste and release go/no-go stay human; Layer 3 remains deferred.
- [x] #1 and #23 remain open.

## Language Boundary

Documentation plus Rust contract. Self-validation/diagnosis/improvement logic
belongs in the Rust kernel/evaluator/source-apply data plane; browser evidence
reuses openchrome/JS probes; the Elixir executor remains unchanged as control
plane. The contract adds no new data plane.

## Critical Risk Review

- **Self-certification risk:** mitigated by independent gates, source-apply,
  trust-gradient, rollbackability, and high-risk human go/no-go.
- **Hidden human dependency:** mitigated by an explicit zero-human-input
  autonomous-path invariant and a `manual-intervention` friction kind that fails
  the claim outside the allowed tail.
- **Reinventing verification:** mitigated by explicitly requiring openchrome,
  scenario verdicts, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
  evolve, source-apply, and trust-gradient.
- **Unbounded self-modification:** mitigated by budgets/stop conditions,
  source-apply classification, trust-gradient, and high-risk queueing.

## Definition of Done

This contract exists in docs and Rust; it selects the existing Era I
engine-builder deckbuilder; it maps friction to current ledger/journal/verdict
artifacts; it defines whole-chain success; the verification commands pass; no new
verification engine or data plane is introduced; high-risk/source-affecting fixes
are never auto-applied; and #1/#23 remain open.

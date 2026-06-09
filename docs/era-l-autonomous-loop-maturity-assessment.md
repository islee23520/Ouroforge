# Era L Autonomous-Loop Maturity Assessment

Era L measures the real-title dogfood loop over the existing Era I
engine-builder deckbuilder. The assessed loop is the existing evidence-native
pipeline only:

```text
detect -> explain -> trace -> attribute -> propose -> re-verify -> apply-or-queue
```

It reuses openchrome run evidence, scenario verdicts, the four evaluator gates
plus design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
evolve, source-apply, rollback/kill-switch, and trust-gradient routing. Era L
introduces no new verifier, no new telemetry/store schema, and no new data
plane.

## Evidence chain

| Milestone | Evidence | Result |
| --- | --- | --- |
| M68 real-title dogfood run | `docs/scenario-coverage-v60-real-title-dogfood.md`, `examples/real-title-dogfood-v1/run/ledger.jsonl` | The real-title run starts from the Era I engine-builder deckbuilder and records the autonomous stage chain with `requiresHumanInputOnAutonomousPath: false`. |
| M69 self-audit attribution | `docs/scenario-coverage-v61-self-audit-attribution.md` | Failures and slowdowns are attributed through loop-coverage evidence, not by a human report. |
| M70 diagnosis and fix proposal | `docs/scenario-coverage-v62-self-diagnosis-fix-proposal.md` | Root-cause and source-apply proposals are emitted as bounded previews; source-affecting fixes are not self-applied. |
| M71 re-verify/apply routing | `docs/scenario-coverage-v63-self-improvement-loop.md`, `examples/real-title-dogfood-v1/self-improvement-loop-demo-v1/demo.fixture.json` | The low-risk reversible path re-verifies through independent gates and auto-applies with zero human input; high-risk/source-affecting changes queue. |
| M72 optional human channel | `docs/scenario-coverage-v64-optional-human-channel.md`, `examples/real-title-dogfood-v1/optional-human-channel-v1/demo.fixture.json` | Oversight, override, and taste feedback are read-only/provenance-only and never block the autonomous path. |
| M73 coverage lock | `docs/scenario-coverage-v65-autonomous-self-improvement-e2e.md`, `examples/real-title-dogfood-v1/scenario-coverage-v65/matrix.fixture.json` | M68-M72 compose end to end with the same artifacts and boundaries. |

## Maturity measurement

Era L reports two fractions because the trust-gradient intentionally separates
eligible low-risk repairs from high-risk/source-affecting tails:

1. **Autonomous eligible-cycle completion:** `1 / 1 = 100%` for the covered
   reversible low-risk self-improvement cycle. It completes
   detect→explain→trace→attribute→propose→re-verify→apply without human action.
2. **Safe terminal routing across covered fix candidates:** `2 / 2 = 100%`.
   The low-risk candidate is auto-applied after re-verification; the
   high-risk/source-affecting candidate reaches the correct terminal autonomous
   state by being queued for thin human go/no-go provenance and is never
   auto-applied.
3. **Human action on the autonomous path:** `0` required actions. The ledger and
   coverage fixtures assert `requiresHumanInputOnAutonomousPath: false`, optional
   human surfaces are non-blocking, and taste feedback is provenance only.
4. **High-risk auto-apply rate:** `0 / 1 = 0%` for the covered high-risk/source-
   affecting candidate, by design. This is a safety success, not a maturity gap.

## Human-retained boundaries

Humans remain only at irreducible Ring-2 decisions:

- fun/taste judgment and creative release curation;
- public release go/no-go;
- thin go/no-go provenance for high-risk/source-affecting engine changes until
  trust-gradient evidence and reversibility justify broader automation in a
  future scoped milestone.

Read-only oversight and override surfaces are optional observability. They do
not gate, approve, or unblock the default autonomous loop.

## Plane and scope reaffirmation

- Rust kernel/evaluator/source-apply remains the data plane and owns artifact
  truth, schemas, validation, gates, ledger/evidence semantics, and trusted
  writes.
- The Elixir executor remains the control plane only and is unchanged by Era L.
- Engine fixes route through source-apply, independent gates, openchrome rerun,
  rollback/kill-switch, and trust-gradient. They do not bypass review/risk
  classification and do not self-certify.
- The loop improves the engine/harness/pipeline. It is distinct from evolve's
  game-content improvement loop.
- Layer-3 distributed/hosted/live-ops scope remains deferred. #1 and #23 remain
  open governance anchors.

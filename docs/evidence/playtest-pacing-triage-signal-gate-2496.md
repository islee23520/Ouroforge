# Signal Gate Relay playtest pacing handoff template (#2496)

Issue: #2496
M129/M130 context: deferred `pacing-blocker` from Scenario Coverage v110 playtest gap backlog
Stable fixture: `examples/playable-demo-v2/signal-gate-dogfood/playtest-pacing-triage.fixture.json`

## Status

Closure classification: contract-complete

This artifact is an environment and observation-template handoff only. It does
not record a fresh human playtest session, does not resolve `pacing-blocker`, and
does not close #2496 as product-observed complete. Human fun/feel and release
go/no-go judgment remains user-owned and must be supplied later as evidence, not
as an automated pass/fail verdict.

## Prepared local environment

Use this local product surface when the human playtest is ready:

```bash
python3 -m http.server 8896 --bind 127.0.0.1
```

Runtime URL:

```text
http://127.0.0.1:8896/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json
```

Optional mechanical replay/evidence capture command for context before or after
the human notes are recorded:

```bash
export CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2496
node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8896/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json' \
  --run-id issue-2496-signal-gate-playtest-context \
  --out-root runs/issue-2496/live-observability \
  --replay signal-gate-relay \
  --wait-ms 3000
```

Generated run artifacts must remain under ignored `runs/` storage.

## Observation record template

The fixture contains the template fields that must be completed by the human
playtester before #2496 can be closed:

- scenario/replay context;
- observed behavior;
- expected behavior;
- product impact;
- human fun/feel notes;
- mechanical completion vs fun/feel separation;
- release go/no-go separation;
- recommended backlog action;
- owner/follow-up issue for any unresolved blocker;
- generated evidence refs for screenshots, transcript, world/event samples, and
  diagnostics.

## Current backlog visibility

`pacing-blocker` remains visible and unresolved:

| Finding | Category | Status | blocksProductObserved | Owner | Next action |
| --- | --- | --- | --- | --- | --- |
| `pacing-blocker` | `dogfood_game_quality` | `awaiting-human-playtest` | `true` | #2496 / user human playtester | Fill the observation template, then split or resolve with evidence. |

## Handoff request for the user

Please run a bounded human playtest of Signal Gate Relay using the local URL
above and provide the completed observation fields. Do not treat mechanical replay
success as a fun/feel verdict. I will skip closing #2496 until those human notes
exist.

## Generated-state audit

Tracked source contains only this handoff note, the template fixture, and its
smoke test. Future screenshots, browser profiles, transcripts, and replay bundles
must stay under ignored `runs/` unless a later issue explicitly fixture-scopes a
small deterministic artifact.

#1 and #23 remain open governance anchors.

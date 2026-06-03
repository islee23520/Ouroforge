# Scenario Pack v1

Scenario Pack v1 lets a project workspace group QA scenarios as named,
project-level contracts while preserving the existing Seed/Scenario/Evaluator v1
semantics. It is project-local data validated by Rust; it is not a new scenario
language and does not execute QA by itself.

## Schema

Scenario pack files are JSON source files referenced by `ouroforge.project.json`.

```json
{
  "schemaVersion": "scenario-pack-v1",
  "id": "regression",
  "description": "Project workspace regression scenario pack fixture.",
  "seed": "seeds/smoke.yaml",
  "scenes": ["scenes/main.scene.json"],
  "scenarioGroups": [
    {
      "id": "smoke",
      "description": "Smoke and feature sanity contracts.",
      "scenarios": [
        {
          "id": "project-smoke",
          "description": "World state smoke assertion.",
          "assertions": [
            { "world_state": { "path": "tick", "exists": true } }
          ]
        }
      ]
    }
  ]
}
```

## Fields

| Field | Required | Meaning |
| --- | --- | --- |
| `schemaVersion` | yes | Must be `scenario-pack-v1`. |
| `id` | yes | Stable scenario pack id. It must match the project manifest `scenarioPacks[].id` that references the file. |
| `description` | yes | Human-readable contract summary. |
| `seed` | yes | Project-relative Seed path. It must be declared in the project manifest `seeds[]`. |
| `scenes[]` | yes, non-empty | Project-relative scene paths. Each path must be declared in the project manifest `scenes[]`. |
| `scenarioGroups[]` | yes, non-empty | Ordered groups of existing Scenario DSL scenarios. |
| `scenarioGroups[].scenarios[]` | yes, non-empty | Existing `Scenario` records with `id`, `description`, optional `steps`, and optional `assertions`. |

Scenario order is deterministic: groups are read in file order and scenarios are
read in group order. Duplicate group ids and duplicate scenario ids are rejected.

## Validation

Project validation resolves and validates scenario packs:

```bash
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid
```

A valid project summary includes the resolved pack count:

```text
Project manifest valid: project_workspace_fixture
Source refs: 3
Asset roots: 1
Scenario packs: 1
Runs root: runs
```

Malformed pack errors are reported through the manifest reference, for example:

```text
project manifest scenarioPacks ref unsupported failed validation: scenarios/regression.json
unknown field `aiJudge`
```

## Supported scenario semantics

Scenario Pack v1 reuses the existing bounded Scenario DSL:

- existing `steps` such as wait/input/replay/snapshot/restore/visual checkpoints;
- existing assertion targets such as `world_state`, `frame_stats`,
  `runtime_events`, `performance_metrics`, `console_errors`,
  `collision_evidence`, `audio_evidence`, and `animation_evidence`;
- existing bounded assertion operators such as `equals`, `notEquals`, `exists`,
  `contains`, `greaterThan`, `lessThan`, and count comparisons.

HUD v1 scenario checks use the existing `world_state` target. Runtime probes
publish HUD display state under `componentModel.hudValues`, so scenario packs can
assert HUD presence without adding browser-side semantics or an AI judge.

Unsupported fields are rejected by Rust `deny_unknown_fields`. Scenario Pack v1
intentionally does not add AI judging, natural-language verdicts, Playwright, or
new evaluator semantics.

## Current execution boundary

Scenario packs are validation/resolution contracts today. They make project QA
intent explicit and reusable, but project-bound execution is still future scope.

Current compatible commands remain seed-based:

```bash
cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml
cargo run -p ouroforge-cli -- run seeds/platformer.yaml
```

The project-run issue (#249) owns any run metadata binding or execution behavior
that consumes resolved scenario packs. Studio v3 (#252) may display scenario pack
state later, but browser JavaScript must not execute QA or become a command
bridge.

## Fixtures

Tracked fixtures:

- valid project with scenario pack: `examples/project-workspace-fixtures/valid/`
- invalid duplicate scenario ids: `examples/project-workspace-fixtures/invalid/scenario-pack-duplicate-id.json`
- invalid unsupported field: `examples/project-workspace-fixtures/invalid/scenario-pack-unsupported-field.json`
- invalid unsafe path: `examples/project-workspace-fixtures/invalid/scenario-pack-unsafe-path.json`
- invalid project-level pack resolution: `examples/project-workspace-fixtures/invalid/bad-scenario-pack/`

These fixtures are source-like test data, not generated run output.

## Non-goals

Scenario Pack v1 does not add:

- a second scenario language;
- AI semantic judging;
- Playwright or a new browser automation framework;
- hosted/cloud/distributed QA;
- project-run execution;
- project run metadata binding;
- Studio v3 UI;
- mutation application;
- source-code mutation;
- native export, plugin runtime, server/database/auth, production editor, or
  public launch automation.

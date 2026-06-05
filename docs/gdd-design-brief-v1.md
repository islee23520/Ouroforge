# GDD Design Brief Schema v1

Issue: #645  
Status: schema and fixture contract

`gdd-design-brief-v1` represents a bounded game design brief as structured data before any GDD-to-prototype generation, extraction, feasibility, planning, draft, apply, run evidence, or Studio behavior occurs.

The schema is input validation, not generation authority. GDD-derived output remains untrusted until Rust/local validation and review-gated apply. Browser, dashboard, and Studio surfaces may inspect later read models but do not gain trusted writes, command bridges, source mutation, script execution, asset generation, or no autonomous unrestricted game creation authority.

## Required top-level fields

`schemaVersion`, `briefId`, `status`, `gameTitle`, `genre`, `targetGameClass`, `playerFantasy`, `coreLoop`, `mechanics`, `controls`, `winLossConditions`, `scenesLevels`, `entities`, `assetStyleRefs`, `constraints`, `nonGoals`, `acceptanceGoals`, `blockedReasons`, and `boundary`.

`targetGameClass` is intentionally narrow: `small2d-prototype` first, with `compatibility3d-prototype` only for later slices that can cite prior 3D gate evidence.

## Validation gates

Rust validation rejects missing or one-step ready-state core loops, unsupported target classes, unsafe local refs, unsupported asset/style ref kinds or unclear license evidence, overbroad scope, contradictory constraints/non-goals, missing acceptance goals, and unclear ready-state win/loss conditions. These checks are guardrails for later evidence-gated planning; they do not authorize generation or apply behavior.

## Fixtures

- `examples/gdd-design-brief-v1/design-brief.valid.fixture.json`
- `examples/gdd-design-brief-v1/design-brief.partial.fixture.json`
- `examples/gdd-design-brief-v1/design-brief.blocked.fixture.json`
- `examples/gdd-design-brief-v1/invalid/design-brief.unsafe-ref.fixture.json`
- `examples/gdd-design-brief-v1/invalid/design-brief.overbroad-scope.fixture.json`
- `examples/gdd-design-brief-v1/invalid/design-brief.contradictory.fixture.json`
- `examples/gdd-design-brief-v1/invalid/design-brief.unclear-win-loss.fixture.json`
- `examples/gdd-design-brief-v1/invalid/design-brief.unsupported-asset-kind.fixture.json`

## Read-model compatibility

The display read model summarizes validated briefs with schema/version identity, status, target class, count fields, validation summary text, compatibility notes, blocked reason count, and the original boundary. It is intentionally display-only: malformed briefs fail before read-model creation, and browser/dashboard/Studio consumers must treat the model as inspection data rather than trusted persistence or generation authority.

## Boundaries

This contract does not authorize requirement extraction, mechanics mapping, feasibility decisions, project scaffold generation, prototype draft bundles, review-gated apply, Studio UI, command bridges, source/script mutation, generated proprietary assets, native export, production-game claims, or current engine replacement claims.

Generated prototype drafts, plans, reviews, applies, runs, evidence, screenshots, dashboard exports, temporary projects, and local tool state remain ignored unless explicitly fixture-scoped by a later issue.

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor.

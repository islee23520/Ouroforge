# GDD Design Brief Schema v1

Issue: #645  
Status: schema and fixture contract

`gdd-design-brief-v1` represents a bounded game design brief as structured data before any GDD-to-prototype generation, extraction, feasibility, planning, draft, apply, run evidence, or Studio behavior occurs.

The schema is input validation, not generation authority. GDD-derived output remains untrusted until Rust/local validation and review-gated apply. Browser, dashboard, and Studio surfaces may inspect later read models but do not gain trusted writes, command bridges, source mutation, script execution, asset generation, or no autonomous unrestricted game creation authority.

## Required top-level fields

`schemaVersion`, `briefId`, `status`, `gameTitle`, `genre`, `targetGameClass`, `playerFantasy`, `coreLoop`, `mechanics`, `controls`, `winLossConditions`, `scenesLevels`, `entities`, `assetStyleRefs`, `constraints`, `nonGoals`, `acceptanceGoals`, `blockedReasons`, and `boundary`.

`targetGameClass` is intentionally narrow: `small2d-prototype` first, with `compatibility3d-prototype` only for later slices that can cite prior 3D gate evidence.

## Fixtures

- `examples/gdd-design-brief-v1/design-brief.valid.fixture.json`
- `examples/gdd-design-brief-v1/design-brief.partial.fixture.json`
- `examples/gdd-design-brief-v1/design-brief.blocked.fixture.json`
- `examples/gdd-design-brief-v1/invalid/design-brief.unsafe-ref.fixture.json`

## Boundaries

This contract does not authorize requirement extraction, mechanics mapping, feasibility decisions, project scaffold generation, prototype draft bundles, review-gated apply, Studio UI, command bridges, source/script mutation, generated proprietary assets, native export, production-game claims, or current engine replacement claims.

Generated prototype drafts, plans, reviews, applies, runs, evidence, screenshots, dashboard exports, temporary projects, and local tool state remain ignored unless explicitly fixture-scoped by a later issue.

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor.

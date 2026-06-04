# Level Intent v1

Issue: #628 - Level Intent and Design Constraint Model v1.

Level Intent v1 is a source-like planning input contract. It records what a
level should achieve before any generation plan, draft, evidence bundle, review
decision, or apply record exists. It does not generate scenes, does not write
trusted files, does not apply drafts, and does not make browser output
authoritative.

## Contract

The `level-intent-v1` artifact includes:

- stable intent and level ids;
- target game mode;
- bounded level size in tiles, with width and height from 1 through 512;
- player goals, objectives, and design constraints;
- required and forbidden mechanics;
- allowed and forbidden asset/entity refs;
- difficulty and pacing targets;
- forbidden placement refs;
- linked Seed and scenario-pack goals;
- draft, partial, or blocked status with blocked reasons only for blocked
  intents.

Supported mechanics are intentionally small: movement, collision, trigger,
collect, exit, dialogue, enemy patrol, platforming, puzzle switch, audio intent,
and animation state. Difficulty targets are tutorial, easy, normal, and hard.
Pacing targets are calm, steady, escalating, and intense.

## Validation Boundary

Rust validation owns the trusted interpretation of the artifact. Validation
rejects missing objectives, unsupported mechanics, unsafe repo-relative refs,
contradictory allowed/forbidden constraints, unbounded level sizes, unsupported
difficulty or pacing targets, malformed linked Seed/scenario refs, and blocked
intents without reasons.

The artifact is not generated state authority. Later generation plans, layout
solvers, drafts, evidence, reviews, and apply records must remain separate
artifacts with their own validation and review gates.

## Fixtures

- `examples/level-intent-v1/level-intent.valid.fixture.json`
- `examples/level-intent-v1/level-intent.partial.fixture.json`
- `examples/level-intent-v1/level-intent.blocked.fixture.json`
- `examples/level-intent-v1/invalid/*.fixture.json`

Generated level drafts, previews, screenshots, runs, dashboard exports, temp
projects, and local tool state remain untracked unless a later issue explicitly
scopes deterministic source-like fixtures.

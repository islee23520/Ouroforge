# Studio Level Design Inspection Surface v1

Issue: #639 - Studio Level Design Inspection Surface v1.

Studio Level Design Inspection Surface v1 is a read-only Authoring Cockpit
surface for reviewing exported Agentic Scene and Level Designer v1 evidence. It
helps humans inspect the level design chain without granting the browser trusted
write, apply, command, or approval authority.

## Read Model

The cockpit accepts `level_design_inspection`, `levelDesignInspection`,
`studio_level_design_inspection`, or `studioLevelDesignInspection` from exported
dashboard data.

The read model includes:

- `schemaVersion`: expected to be `studio-level-design-inspection-v1`;
- `status`: aggregate readiness or blocked state;
- `boundary`: conservative read-only boundary text;
- `malformedReasons`: visible malformed input diagnostics;
- `panels`: ordered inspection panels with `id`, `label`, `kind`, `status`,
  `items`, `refs`, and optional inert `commands`.

The v1 panel set covers:

- level intent and design constraints;
- scene generation plan;
- tilemap and entity/objective/encounter drafts;
- reachability and pathing evidence;
- objective completion and win/loss proof;
- difficulty, pacing, and balance heuristic evidence;
- visual and semantic diff evidence;
- review and apply status.

## Rendering Contract

The Studio/cockpit renderer must:

- escape every exported label, value, ref, status, schema, boundary, malformed
  reason, and command string;
- render missing data as a visible empty state;
- render malformed panel data as a visible warning rather than crashing;
- keep command strings inside inert copyable text;
- preserve intent, plan, draft, evidence, diff, review, and apply status as
  separate inspection rows.

The renderer must not add action buttons, browser persistence, local command
execution, local server bridges, hidden background execution, or trusted file
writes.

## Boundary

This surface may display exported generated level evidence only. Rust/local
validation owns trusted persistence, draft/apply validation, generated evidence
writing, CLI contracts, rollback metadata, and any future trusted apply path.

Studio Level Design Inspection Surface v1 does not implement:

- browser trusted writes;
- command bridge or local server bridge;
- hidden command execution;
- auto-apply or auto-merge;
- self-approval or reviewer bypass;
- unrestricted source mutation;
- arbitrary script execution, dynamic code loading, or visual scripting;
- autonomous full game generation;
- production editor, full visual level editor, hosted/cloud Studio, native
  export, plugin runtime, marketplace, account system, production-ready claim,
  or current Godot replacement.

Difficulty and pacing rows are advisory evidence. They do not prove subjective
game quality.

## Verification

Issue #639 verification should include:

- `node --check examples/authoring-cockpit/cockpit.js`;
- `node examples/authoring-cockpit/cockpit.test.cjs`;
- dashboard checks when dashboard data changes;
- Rust workspace regression checks required by the governing issue;
- a generated-state audit proving drafts, previews, runs, dashboard exports,
  and local tool state remain untracked unless fixture-scoped;
- live `gh issue view` checks proving #1 and #23 remain open.

## Closure evidence

Before closing #639, confirm that #1 and #23 remain open, cockpit coverage
proves escaped read-only rendering for populated, missing, and malformed level
design inspection data, and no browser trusted-write, command bridge,
auto-apply, auto-merge, self-approval, production-editor, visual-scripting, or
autonomous full game generation scope was added.

Recommended closure commands:

```bash
gh issue view 639 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

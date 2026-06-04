# Level Visual Semantic Diff v1

Issue: #636 - Level Visual Diff and Semantic Comparison v1.

Level Visual Semantic Diff v1 records reviewable visual and semantic summaries
for before/after level drafts. It links intent, plan, drafts, reachability,
objective proof, heuristic evidence, and optional transaction/apply artifacts
where present.

This is advisory diff evidence. Trusted diff validation belongs to local Rust
artifacts; browser, dashboard, and Studio surfaces may display exported diffs
read-only, but do not compute trusted diffs, write files, apply drafts, or
approve changes.
They add no scene write capability.

## Artifact Shape

The `level-visual-semantic-diff-v1` artifact includes:

- stable diff, intent, plan, before/after draft, scene, reachability, objective
  proof, heuristic, and optional transaction refs;
- semantic change categories for added, removed, moved, and changed entities;
  changed tiles and regions; changed objective proof, reachability, blockers,
  and heuristic warnings; unchanged state; partial state; and missing evidence;
- expected scenario impact summaries;
- expected evidence under `evidence/level-diff/<diff-id>/`;
- status, blocked reasons, and guardrails.

Malformed, partial, stale, missing-evidence, and unchanged diffs remain visible
states. This artifact adds no visual editor, no browser trusted write path, and
no auto-apply behavior.

## Read Model

The read model reports change counts, semantic change counts, missing evidence
counts, partial counts, scenario impact counts, linked evidence refs, blocked
reasons, and boundary text.

The boundary is read-only level visual and semantic diff evidence: trusted diffs
are local Rust-validated artifacts, browser and Studio display only, no scene
write, no trusted apply, no browser command bridge, no auto-apply, and no
auto-merge.

## Non-Goals

- No autonomous full game generation.
- No production editor or full visual level editor claim.
- No browser trusted writes, command bridge, local server bridge, auto-apply, or
  auto-merge.
- No unrestricted source mutation apply.
- No native export, plugin runtime, hosted/cloud behavior, production-ready
  claim, or Godot replacement claim.

# Public Demo Evidence Refresh Audit v1

Status: **audit artifact** for issue #370 PA1.4.1.

This audit inventories the public-demo evidence references that must be refreshed
or intentionally preserved before public-alpha launch governance continues. It is
documentation-only: it does not generate demo output, commit runtime artifacts,
change repository visibility, publish packages, or implement launch automation.

## Audit scope

PA1.4.1 covers stale-reference discovery for the public demo evidence path. The
tracked screenshots remain source-like demo references under `docs/assets/demo/`;
new generated `runs/`, dashboard exports, smoke logs, local browser profiles, and
machine-local tool folders remain out of scope for this PR unit.

Audited surfaces:

| Surface | Current tracked reference | Audit result | Follow-up owner |
| --- | --- | --- | --- |
| Minimal 2D runtime demo | `docs/assets/demo/runtime-demo.png` | Tracked PNG exists at 1280x720 and remains a demo reference, not generated run evidence. | PA1.4.2 may refresh the screenshot if visual drift is confirmed. |
| Evidence dashboard | `docs/assets/demo/evidence-dashboard.png` and `examples/evidence-dashboard/` | Tracked PNG exists at 1440x1000. Current docs still describe the older AL2.8.1 dashboard export even though later dashboard read models now include asset inspector, visual draft, source patch preview, regression matrix, and loop evidence surfaces. | PA1.4.2 should refresh screenshot/evidence references without committing `examples/evidence-dashboard/dashboard-data.json`. |
| Authoring cockpit / Studio surface | `docs/assets/demo/authoring-cockpit.png` and `examples/authoring-cockpit/` | Tracked PNG exists at 1440x1000. Current wording still centers on the Studio v2 cockpit and should be checked against the later loop/source-patch/read-only inspection surfaces. | PA1.4.2 should refresh cockpit references while preserving browser read-only boundaries. |
| Public demo evidence doc | `docs/public-demo-evidence.md` | Fresh-clone smoke evidence records Platformer and Engine Expansion v1 run ids from 2026-06-02. Those run ids are historical evidence and should not be rewritten unless PA1.4.2 creates new local run evidence. | PA1.4.3 should finalize current run ids/paths, cleanup steps, and known gaps. |
| Public readiness audit | `docs/public-readiness-audit.md` | References AL2.8.1/AL2.8.2 evidence. This is acceptable as history, but final docs should point readers to the current refresh audit/finalization record. | PA1.4.3 should add final cross-reference if needed. |
| Regression matrix | `docs/regression-run-matrix-v1.md` | Documents generated dashboard export path and Node dashboard/cockpit checks; no generated export is tracked. | PA1.4.2/PA1.4.3 should keep this as local generated evidence only. |
| Source patch preview review | `docs/source-mutation-preview-v1.md`, `docs/source-patch-preview-coverage-matrix-v1.md`, `docs/studio-source-patch-review-surface-v1.md` | Later docs define read-only review/bundle/matrix evidence. The public demo doc does not yet clearly name these as current dashboard/Studio evidence areas. | PA1.4.2/PA1.4.3 should reference them conservatively as docs evidence, not as source apply capability. |
| Asset inspector and visual draft | `docs/studio-asset-inspector-v1.md`, `docs/visual-edit-draft-model-v1.md`, `docs/studio-evidence-fidelity-surfaces.md` | Current public demo doc does not explicitly list these dashboard/Studio evidence areas. | PA1.4.2 should include them in refreshed references if the static surfaces already expose them. |
| Loop cockpit surfaces | `docs/authoring-loop-*.md`, `docs/studio-v3-project-workspace-cockpit.md`, `examples/authoring-cockpit/` | Current docs describe authoring/loop evidence as local and read-only; no trusted browser write is authorized. | PA1.4.2/PA1.4.3 should keep loop cockpit wording read-only and cleanup-aware. |

## Stale or drift-prone references found

- `docs/public-demo-evidence.md` still labels the dashboard/cockpit capture data
  as AL2.8.1 while also noting an AL2.8.2 image refresh. This is historical, but
  a fresh reader needs a current refresh note for issue #370 before closure.
- The same document records older local run ids. They are valid historical smoke
  evidence, but they should be identified as historical unless refreshed in
  PA1.4.2.
- Later read-only evidence surfaces exist for asset inspection, visual edit
  drafts, source patch preview review, regression matrices, and loop cockpit
  state. Public demo evidence does not yet inventory those areas together.
- The generated-state section correctly names `runs/**`, dashboard exports, and
  local browser/tool folders as untracked; PA1.4.2 and PA1.4.3 must preserve that
  policy if new screenshots or smoke outputs are produced locally.

## Generated-state audit

This PR unit intentionally does not run browser capture or dashboard export
commands. The refresh should keep these paths untracked unless a later PR unit
explicitly scopes a tiny deterministic fixture:

- `runs/`
- `target/`
- `.openchrome/`
- `.omc/`
- `.omx/`
- `.claude/`
- `examples/evidence-dashboard/dashboard-data.json`
- private screenshot, smoke-log, temp-project, browser-profile, or local tool
  output paths

## Guardrails confirmed for PA1.4.1

- No repository visibility change.
- No launch, release, package publication, signing, upload, or public
  communication automation.
- No source patch apply, auto-apply, auto-merge, command bridge, browser trusted
  write path, local server bridge, hosted/cloud/server/auth behavior, native
  export, plugin runtime, marketplace, or production editor implementation.
- No generated demo, run, dashboard, screenshot, smoke-log, or local tool
  artifact is added by this audit.
- #1 and #23 remain governance issues and must stay open through #370 closure.

## Verification commands for this audit unit

```bash
gh issue view 370 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
file docs/assets/demo/*.png
grep -RIn "AL2\\.8\\.1\|AL2\\.8\\.2\|run-178040\|issue-49-demo\|al2-8-1" docs README.md CONTRIBUTING.md .github
cargo fmt --check
cargo test
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

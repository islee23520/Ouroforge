# Dogfood B6 Studio UX Validation Evidence

## Metadata

- Blocker: B6 — Studio UX validation evidence is not durable on origin/main
- Report version: `dogfood-studio-ux-validation-v1`
- Demo identity: `collect-and-exit-local-rc-candidate`
- Branch: `dogfood/b6-studio-ux-validation-20260610015757`
- Base: `origin/main` after B5 acceptance and merge commit `6eed602c`
- Source basis: `examples/authoring-cockpit/`, `examples/godot-plus-demo-studio-walkthrough-v790/`, and `examples/playable-demo-v2/collect-and-exit/`
- Evidence classification: `local-read-only-and-review-gated-studio-ux-evidence`
- Issue state evidence: #1 OPEN; #23 OPEN.

## Purpose

This handoff makes B6 durable by tracking Studio UX validation evidence for the compact dogfood demo. It records what the current checked-in Studio surfaces can safely support: local static/read-only inspection, draft-only previews, review-gated handoff text, scenario/playtest/evidence visibility, and conservative missing-data boundaries. It is coordination/evidence only: no product Studio feature, command bridge, trusted browser write, hosted service, release flow, auto-port, foreign-runtime embedding, or Era Q full-3D work is added.

## Merged prerequisite evidence

| Blocker | PR | Origin-main artifact | Status for B6 |
| --- | --- | --- | --- |
| B1 claim coverage | #2334 MERGED | `.omx/dogfood-validation/claim-coverage-matrix.md` | Present; keeps #1/#23 and forbidden-scope guardrails visible. |
| B2 compact demo spec | #2335 MERGED | `.omx/dogfood-validation/demo-game-spec.md` | Present; defines the compact Collect-and-Exit local/manual demo target and Studio inspect path. |
| B3 pipeline dry-run | #2336 MERGED | `.omx/dogfood-validation/pipeline-dry-run.md` | Present; records failed-classified pipeline evidence without overclaiming readiness. |
| B4 export readiness | #2337 MERGED | `.omx/dogfood-validation/export-release-readiness.md` | Present; records local/manual export-readiness guardrails. |
| B5 gameplay/runtime stress | #2339 MERGED | `.omx/dogfood-validation/gameplay-runtime-stress.md` | Present; records bounded local gameplay/runtime stress evidence. |

## Commands executed

All commands were run from a fresh B6 worktree based on `origin/main`. They are local, non-destructive, and do not execute browser-trusted writes.

```bash
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
node examples/authoring-cockpit/integrated-demo-smoke.test.cjs
node examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs
```

Observed output:

- `authoring cockpit smoke test passed`
- `cockpit prototype-planning null-panel regression test passed`
- `cockpit export-package null-plan-step regression test passed`
- `{"issue":774,"panelCount":13,"fixture":"full-studio-integrated-demo-v1"}`
- `{"issue":790,"fixture":"godot-plus-demo-studio-walkthrough-v790","panelChecks":11}`

## Studio UX evidence summary

| Requirement | Verdict | Evidence / path | Notes |
| --- | --- | --- | --- |
| Static cockpit syntax and regression coverage | pass | `examples/authoring-cockpit/cockpit.js`; `examples/authoring-cockpit/cockpit.test.cjs` | Cockpit renderers, command refusal, editable/read-only field boundaries, layout persistence, null-panel regressions, and full-studio fixture checks pass. |
| Integrated Studio demo fixture | pass | `examples/authoring-cockpit/integrated-demo-smoke.test.cjs` | Fixture reports issue 774, 13 panels, and `full-studio-integrated-demo-v1`. |
| Compact-demo Studio walkthrough | pass | `docs/godot-plus-demo-studio-walkthrough-v1.md`; `examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs` | Walkthrough covers project overview, scene tree/entity inspector, read-only canvas, asset browser, scenario/playtest, evidence timeline, export/package inspection, plugin descriptor inspection, draft preview, and Safe Source Apply handoff preview. |
| Read-only/review-gated boundary | pass | `docs/studio-review-cockpit-v1.md`; `docs/studio-v3-project-workspace-cockpit.md`; cockpit command registry checks | Browser surfaces show exported JSON and copyable CLI text only; apply-source, execute-plugin, publish, auto-apply, auto-merge, and shell/command bridge behaviors remain blocked. |
| Export/package inspection boundary | pass | `docs/studio-export-package-inspection-panel-v1.md`; B4 report | Studio package/export surface is read-only; publish/release/sign/upload actions are blocked and stay trusted-CLI-owned. |
| Gameplay/runtime evidence visibility | pass | B5 report; walkthrough scenario/playtest panel; cockpit evidence panes | Studio UX can inspect scenario/playtest/evidence surfaces for the compact demo without starting runs or mutating source from the browser. |
| Non-developer production workflow claim | gap-recorded | This report; docs listed above | Current support is a local static evidence-inspection and draft/handoff shell, not a broad production editor or Phoenix/LiveView implementation claim. |

## Validated task areas

- **Project overview and scene/entity inspection:** Studio walkthrough and cockpit renderers display compact-demo project/scene context, scene tree, selected entity, and component fields.
- **Draft-only preview:** editable primitive fields and draft authoring surfaces are represented, but trusted apply remains outside the browser.
- **Review-gated handoff:** Safe Source Apply handoff and review cockpit surfaces display evidence and inert command text without executing commands.
- **Scenario/playtest/evidence visibility:** Studio surfaces can show scenario/playtest/evidence timeline/export/package/plugin panels from fixture/exported data.
- **Missing/malformed data safety:** cockpit regression tests cover null/missing panel shapes and must render explicit warning/empty states instead of optimistic readiness.

## Gaps and conservative wording

- No new Phoenix/LiveView implementation is introduced or claimed by B6.
- The current checked-in surface is static/local and evidence-first; it supports a developer/operator inspection and handoff loop, not a non-developer end-to-end production Studio.
- Browser workspace layout persistence is UI-only local browser state; it is not trusted project/source persistence.
- Dashboard/live evidence export is not executed by this B6 PR; this handoff relies on existing fixture and code smoke evidence plus accepted B1-B5 dogfood artifacts.
- Any future improvement should be narrow onboarding/missing-data guidance or fixture/live badging, not browser command execution or broad editor authority.

## Verification commands for this PR

```bash
node --test examples/dogfood-studio-ux-validation-v1/studio-ux-validation-smoke.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
node examples/authoring-cockpit/integrated-demo-smoke.test.cjs
node examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs
git diff --check origin/main...HEAD
```

## Non-goals and guardrails

- #1 and #23 remain open.
- Era Q M102-M106 remain deferred/non-goal; no full-3D implementation is added.
- No product Studio feature, hosted/cloud/multi-user behavior, trusted browser/source writes, command bridge, auto-port, embedded foreign runtime, release automation, signing, upload, publishing, credential flow, or Steam depot flow is added.
- No production-ready, store-ready, commercial release, native export, Godot replacement, full Godot parity, or shipped-game maturity claim is made.

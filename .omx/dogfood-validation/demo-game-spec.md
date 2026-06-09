# Dogfood Compact Demo Game Spec

## Spec metadata

- Spec version: `dogfood-demo-spec-v1`
- Demo identity: `collect-and-exit-local-rc-candidate`
- Linked blocker: B2 — No compact demo spec
- Linked #1 claim: https://github.com/shaun0927/Ouroforge/issues/1
- Protected issue state: #1 remains open; #23 remains open.
- Source basis: `examples/playable-demo-v2/collect-and-exit/`
- Status: shared local/manual dogfood target; evidence gaps remain until downstream lanes report against this spec.

## Existing demo basis

Use the existing Collect-and-Exit fixture because it already has a deterministic one-screen project, source-like scene, seed, scenario pack, local asset manifest, HUD model, audio intent metadata, export placeholder, plugin descriptors, QA/playtest fixtures, and smoke tests. B2 does not add gameplay, runtime, export, or Studio behavior; it only fixes the shared contract every lane must cite.

Canonical source paths:

- Project manifest: `examples/playable-demo-v2/collect-and-exit/ouroforge.project.json`
- Seed: `examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml`
- Scenario pack: `examples/playable-demo-v2/collect-and-exit/scenarios/collect-and-exit.json`
- Scenario matrix: `examples/playable-demo-v2/collect-and-exit/scenarios/demo-scenario-matrix.json`
- Scene: `examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json`
- Asset manifest: `examples/playable-demo-v2/collect-and-exit/asset-manifest.json`

## Player loop and scenarios

Player loop: start in the one-screen scene, move the player, collect the key, open or satisfy the exit condition, and finish the run with scenario evidence for both success and bounded failure.

Required scenario IDs:

- `CAE-SUCCESS-KEY-EXIT`: player collects the key and reaches the exit; expected verdict is pass when all scenario assertions and evidence refs are present.
- `CAE-FAIL-MISSING-KEY`: player reaches the exit condition without required key state; expected verdict is fail or blocked with explicit evidence refs.
- `CAE-STRESS-REPLAY`: repeat the success path under the runtime/performance budget; expected verdict is pass only if deterministic replay and budget evidence are present.

Success conditions: scenario assertions pass, required evidence artifacts exist, replay or state digest is deterministic, and generated outputs stay in ignored/generated roots.

Failure conditions: missing key, missing scenario evidence, stale refs, malformed evidence, performance budget breach, unsafe generated-output retention, or any forbidden-scope claim.

## Controls and input assumptions

- Keyboard movement is the default input assumption for local smoke runs.
- Scenario/replay input must be deterministic and fixture-scoped.
- Synthetic or scripted input may be used only as test evidence; it does not imply hosted playtest services, live browser command bridges, or trusted browser writes.

## Content inventory

Required compact-demo content:

- Assets: `assets/sprites/collect-and-exit-sheet.png`, `assets/atlases/collect-and-exit.atlas.json`, `assets/tilemaps/collect-and-exit.tilemap.json`, `assets/tilesets/collect-and-exit.tileset.json`, and `assets/audio/collect.ogg`.
- Entities/logic: player, key/collectible trigger, exit/door trigger, collision boundaries, and bounded hazard-drone behavior from `behaviors/hazard-drone.json` where cited by lane evidence.
- HUD: `hud-model.json` with visible objective/progress state for key and exit completion.
- Audio: collect audio intent metadata only; no new audio runtime or generated audio asset pipeline is added by B2.
- Levels: `levels/level-set.json` and the one-screen `collect-and-exit` scene remain the bounded level set.

## Studio UX author/inspect path

Studio UX validation must remain read-only or review-gated over existing artifacts:

1. Inspect the project manifest, scene, seed, scenario pack, asset manifest, HUD model, and lane evidence refs.
2. Draft or preview changes only through existing source-like draft fixtures or review-gated CLI paths when a later lane explicitly owns that work.
3. Export dashboard/Studio read models as generated evidence only under ignored/generated roots.
4. Do not let browser/Studio surfaces write project files, run local commands, persist trusted state, fetch assets, or apply drafts directly.

## Runtime, performance, and stress budget

- Runtime target: local deterministic smoke execution of the existing Collect-and-Exit fixture.
- Frame/performance budget: use the existing bounded frame-budget defaults declared by the fixture and any lane-specific runtime-frame-budget evidence.
- Stress target: at least one passing deterministic replay path and one bounded failure path must be documented by the runtime-stress lane before runtime readiness is claimed.
- Budget failure is a blocker or failed verdict, not a silent pass.

## Local export and readiness expectation

B2 defines only local/manual readiness expectations. Later export/readiness evidence must cite:

- `examples/playable-demo-v2/collect-and-exit/export/export-profile.json`
- `examples/playable-demo-v2/collect-and-exit/export/package-metadata.json`
- local package/evidence outputs retained only as generated artifacts or explicit PR evidence refs

No release automation, signing, upload, publishing, credential flow, native/store export, or commercial readiness is added by this spec.

## Retained and generated artifact policy

- Source fixtures and this spec may be tracked.
- Generated runs, dashboard data, screenshots, previews, package outputs, and temporary smoke outputs must stay under ignored/generated roots unless a later PR explicitly adds fixture-scoped evidence.
- PRs must cite generated evidence paths or run ids in reports instead of committing broad runtime output.

## Lane evidence artifacts expected

Each dogfood lane should cite this spec version and write or update its own report:

- Pipeline dry-run: `.omx/dogfood-validation/pipeline-dry-run.md`
- Studio UX validation: `.omx/dogfood-validation/studio-ux-validation.md`
- Gameplay runtime stress: `.omx/dogfood-validation/gameplay-runtime-stress.md`
- Asset/content pipeline: `.omx/dogfood-validation/asset-content-pipeline.md`
- QA/evidence/evolve: `.omx/dogfood-validation/qa-evidence-evolve.md`
- Export readiness: `.omx/dogfood-validation/export-release-readiness.md`
- Claim coverage matrix: `.omx/dogfood-validation/claim-coverage-matrix.md`

## Explicit non-goals and forbidden scope

- Leave #1 and #23 open.
- Do not implement or activate Era Q full-3D M102–M106.
- Do not add hosted/cloud/multi-user scope.
- Do not add trusted browser writes or trusted source writes.
- Do not add auto-port, live bridge, foreign runtime embedding, or source-runtime reproduction.
- Do not add release automation, signing, upload, publishing, credential automation, native export, or Steam publishing.
- Do not claim production readiness, store readiness, commercial release readiness, full Godot parity, Godot replacement status, or shipped-game maturity.
- Do not add speculative gameplay, export, runtime, Studio, plugin, or asset features in B2.

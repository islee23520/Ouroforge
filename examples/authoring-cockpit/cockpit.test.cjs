const assert = require('node:assert/strict');
const fs = require('node:fs');
const cockpit = require('./cockpit.js');
const scene = require('../game-runtime/scene.json');

const moved = cockpit.applyEdit(scene, 'player', 'components.transform.x', '48');
assert.equal(cockpit.getValue(moved.entities[0], 'components.transform.x'), 48);
const recolored = cockpit.applyEdit(scene, 'player', 'sprite.color', '#ffffff');
assert.equal(cockpit.getValue(recolored.entities[0], 'sprite.color'), '#ffffff');
assert.throws(() => cockpit.applyEdit(scene, 'player', 'components.size.width', '0'), /Invalid numeric/);
assert.match(cockpit.renderTree(scene, 'player'), /player/);
assert.match(cockpit.renderInspector(scene, 'player'), /components\.transform\.x/);
assert.match(cockpit.renderInspector(scene, 'player'), /Read-only \/ unsupported fields/);
assert.match(cockpit.renderInspector(scene, 'player'), /components\.animation/);
assert.match(cockpit.renderInspector(scene, 'player', 'examples/game-runtime/scene.json', 'Invalid numeric value for components.size.width'), /id="edit-error"/);
assert.match(cockpit.renderInspector(scene, 'player', 'examples/game-runtime/scene.json', 'Invalid numeric value for components.size.width'), /Invalid numeric value/);
assert.deepEqual(cockpit.EDITABLE_FIELDS.map(([path]) => path), [
  'sprite.color',
  'components.transform.x',
  'components.transform.y',
  'components.velocity.x',
  'components.velocity.y',
  'components.size.width',
  'components.size.height',
  'components.controllable',
]);
assert.ok(cockpit.READ_ONLY_FIELDS.includes('components.collider'));
assert.throws(() => cockpit.applyEdit(scene, 'player', 'components.collider', '{}'), /Unsupported edit path/);
assert.equal(cockpit.applyEdit(scene, 'player', 'components.controllable', 'false').entities[0].components.controllable, false);
assert.match(cockpit.cliCommand('examples/game-runtime/scene.json', 'player', 'sprite.color', '#ffffff'), /ouroforge-cli -- scene edit/);
assert.equal(
  cockpit.cliCommand('examples/game-runtime/scene.json', 'player', 'components.transform.x', 48),
  "cargo run -p ouroforge-cli -- scene edit examples/game-runtime/scene.json --entity player --path components.transform.x --value '48'"
);
assert.equal(
  cockpit.transactionCommand('examples/game-runtime/scene.json', 'player', 'components.transform.x', 48, 'runs/manual/transactions/player-x-48.json'),
  "cargo run -p ouroforge-cli -- scene edit examples/game-runtime/scene.json --entity player --path components.transform.x --value '48' --transaction-output runs/manual/transactions/player-x-48.json"
);

const run = {
  summary: { id: 'run-1', run_dir: 'runs/run-1', verdict_status: 'passed', scenario_status: 'passed' },
  command_context: {
    schemaVersion: 'run-command-context-v1',
    command: 'cargo run -p ouroforge-cli -- run seeds/platformer.yaml --project examples/project --workers 4 --scenario-pack smoke',
    seedPath: 'seeds/platformer.yaml',
    workers: 4,
    runsRoot: 'runs',
    projectRoot: 'examples/project',
    manifestPath: 'examples/project/ouroforge.project.json',
    scenarioPackId: 'smoke',
    runtimeTarget: 'local-static-browser',
    browserBoundary: 'openchrome_cdp',
    cdpTransport: 'chrome_devtools_protocol',
    environmentHints: ['The cockpit does not execute commands'],
  },
  project: {
    id: 'minimal_2d',
    name: 'Minimal 2D Ouroforge Project',
    projectRoot: 'examples/project',
    manifestPath: 'examples/project/ouroforge.project.json',
    manifestHash: { algorithm: 'fnv1a64-file-v1', value: 'manifesthash' },
    seedPath: 'seeds/platformer.yaml',
    scenes: [{ id: 'main', path: 'scenes/main.scene.json', hash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'scenehash' } }],
    scenarioPack: { id: 'smoke', path: 'scenarios/smoke.scenario-pack.json', scenarioIds: ['scaffold-smoke'] },
  },
  evidence: [{ id: 'evidence-1', path: 'evidence/indexed.json' }],
  mutations: [{ id: 'mutation-1' }],
  screenshots: [{ id: 'shot-1', path: 'evidence/shot.png' }],
  journal: '# Journal',
  journal_view: { exists: true, path: 'journal.md', summary: 'journal summary', entries: [{ id: 'entry-1' }], evidence_refs: ['evidence/indexed.json'] },
  transaction_provenance: {
    transactionId: 'scene-edit-abc123',
    transactionArtifactPath: '.omx/tmp/transaction.json',
    scenePath: 'examples/game-runtime/scene.json',
    beforeSceneHash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'beforehash' },
    afterSceneHash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'afterhash' },
  },
  mutation_lifecycle: {
    terminal_state: 'pending_review',
    command_hints: ['cargo run -p ouroforge-cli -- mutation review runs/run-1 --reject --reason "manual"'],
    stages: [
      { id: 'proposed', label: 'Proposed', state: 'proposed', artifact_path: 'mutation/proposals.json', record_count: 1, records: [{ id: 'proposal-1', evidence_id: 'verdict-1' }] },
      { id: 'scene_applied', label: 'Applied scene mutation', state: 'applied', artifact_path: 'mutation/scene-applications.json', record_count: 1, records: [{ id: 'scene-application-1', proposalId: 'proposal-1', transactionId: 'scene-edit-abc123', targetScenePath: 'examples/project/scenes/main.scene.json', transactionArtifactPath: 'mutation/scene-edit.json', beforeSceneHash: { value: 'beforehash' }, afterSceneHash: { value: 'afterhash' }, project: { projectId: 'minimal_2d', manifestPath: 'examples/project/ouroforge.project.json', scenePath: 'scenes/main.scene.json' }, rollback: { scenePath: 'examples/project/scenes/main.scene.json', restoreHash: { value: 'beforehash' } }, status: 'applied' }] },
    ],
  },
  replay: { present: true, sequences: [{ id: 'replay-1', event_count: 2, frames: [0, 4], evidence_refs: ['evidence/replay.json'] }] },
  comparison: { present: true, artifacts: [{ before_run_id: 'before', after_run_id: 'after', classification: 'improved', path: 'mutation/run-comparison-before--after.json', evidence_refs: ['runs/before/verdict.json', 'runs/after/verdict.json'], semantic: { schemaVersion: 'run-semantic-diff-v1', reasons: [{ kind: 'transaction_provenance', severity: 'changed', summary: 'scene edit transaction provenance changed' }], scenarios: [], worldState: { changed: [] }, project: { relation: 'same_project', changed: true, changes: [{ kind: 'scene_hash', summary: 'scene hash changed for scenes/main.scene.json', before: 'before-scene', after: 'after-scene' }], warnings: ['project fixture warning'] }, transactionProvenance: { changed: true }, warnings: ['fixture warning'] } }] },
  engine_summaries: {
    present: true,
    source_world_state: 'evidence/world.json',
    scene: { sceneId: 'foundation-scene', entityCount: 3, tick: 4 },
    renderer: { version: '1', renderedEntities: 3, camera: { x: 0, y: 0 } },
    tilemaps: { tilemapCount: 1, layerCount: 4 },
    assets: { manifestId: 'runtime-v1-assets', assetCount: 3 },
    animation: { animatedEntityCount: 1 },
    audio: { audioEntityCount: 1, audioEventCount: 1 },
    physics: { colliderEntityCount: 3, collisionEventCount: 1 },
    reload: { reloadCount: 1, lastStatus: 'succeeded' },
    composition: { entityCount: 3, parentedEntityCount: 1 },
  },
};
assert.match(cockpit.qaCommand(), /run seeds\/platformer\.yaml --workers 4/);
assert.equal(cockpit.qaTransactionCommand('seeds/platformer.yaml', '.omx/tmp/transaction.json', 4), 'cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4 --transaction .omx/tmp/transaction.json');
assert.match(cockpit.dashboardExportCommand(), /dashboard export/);
assert.equal(cockpit.sceneValidateCommand('examples/game-runtime/scene.json'), 'cargo run -p ouroforge-cli -- scene validate examples/game-runtime/scene.json');
assert.equal(cockpit.sceneReloadValidateCommand('examples/game-runtime/scene.json'), 'cargo run -p ouroforge-cli -- scene reload-validate examples/game-runtime/scene.json');
assert.match(cockpit.runtimeReloadPayloadCommand('examples/game-runtime/scene.json'), /display-only payload shape/);
assert.match(cockpit.renderCommandGenerationPanel('examples/game-runtime/scene.json'), /browser never executes commands/);
assert.match(cockpit.renderCommandGenerationPanel('examples/game-runtime/scene.json'), /--transaction-output/);
assert.match(cockpit.renderCommandGenerationPanel('examples/game-runtime/scene.json'), /Transaction output is a Rust CLI artifact/);
assert.match(cockpit.renderCommandGenerationPanel('examples/game-runtime/scene.json'), /scene reload-validate/);
assert.equal(cockpit.latestRun([{ summary: { id: 'old', created_at_unix_ms: 1 } }, { summary: { id: 'new', created_at_unix_ms: 2 } }]).summary.id, 'new');
assert.match(cockpit.renderPreview(), /runtime-preview/);
assert.match(cockpit.renderQaPanel(), /Run QA/);
assert.match(cockpit.renderEvidencePane(run), /journal summary/);
assert.match(cockpit.renderStudioNavigation(run), /Studio v2 demo surfaces/);
assert.equal(cockpit.studioSurfaceSummary(run).filter((surface) => surface.present).length, 11);
assert.match(cockpit.renderEvidenceBrowser(run), /Open full evidence dashboard/);
assert.equal(cockpit.projectRunCommand('seeds/platformer.yaml', 'examples/project/ouroforge.project.json', 4, 'smoke'), 'cargo run -p ouroforge-cli -- run seeds/platformer.yaml --project examples/project/ouroforge.project.json --workers 4 --scenario-pack smoke');
assert.equal(cockpit.compareRunsCommand('runs/before', 'runs/after', 'runs/after/comparisons'), 'cargo run -p ouroforge-cli -- compare runs/before runs/after --output-dir runs/after/comparisons');
assert.equal(cockpit.projectValidateCommand('examples/project/ouroforge.project.json'), 'cargo run -p ouroforge-cli -- project validate examples/project/ouroforge.project.json');
assert.equal(cockpit.seedValidateCommand('seeds/platformer.yaml'), 'cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml');
assert.match(cockpit.renderProjectWorkspaceSurface(run), /Project workspace/);
assert.match(cockpit.renderProjectWorkspaceSurface(run), /minimal_2d/);
assert.match(cockpit.renderProjectWorkspaceSurface(run), /scenes\/main\.scene\.json/);
assert.match(cockpit.renderProjectWorkspaceSurface(run), /scaffold-smoke/);
assert.match(cockpit.renderProjectWorkspaceSurface(run), /project validate examples\/project\/ouroforge\.project\.json/);
assert.match(cockpit.renderProjectWorkspaceSurface({ summary: { id: 'run-no-project' }, evidence: [] }), /No project workspace metadata/);
assert.match(cockpit.renderProjectWorkspaceSurface({ summary: { id: 'run-bad-project' }, project: '<script>alert(1)</script>' }), /Malformed project metadata/);
assert.match(cockpit.renderProjectWorkspaceSurface({ summary: { id: 'run-bad-project' }, project: '<script>alert(1)</script>' }), /&lt;script&gt;alert/);
assert.match(cockpit.renderProjectWorkspaceSurface({ summary: { id: 'run-escaped' }, project: { id: '<img>', name: '<script>', manifestPath: '<manifest>', scenes: [{ id: '<scene>', path: '<path>', hash: { value: '<hash>' } }], scenarioPack: { id: '<pack>', path: '<pack-path>', scenarioIds: ['<scenario>'] } } }), /&lt;img&gt;/);
assert.match(cockpit.renderProjectRunSurface(run), /Project run summary/);
assert.match(cockpit.renderProjectRunSurface(run), /run-1/);
assert.match(cockpit.renderProjectRunSurface(run), /local\/untracked expected/);
assert.match(cockpit.renderProjectRunSurface(run), /--project examples\/project\/ouroforge\.project\.json --workers 4 --scenario-pack smoke/);
assert.match(cockpit.renderProjectRunSurface(run), /Reproducible command context/);
assert.match(cockpit.renderProjectRunSurface(run), /openchrome_cdp/);
assert.match(cockpit.renderRunCommandContext({}), /No run command context/);
assert.match(cockpit.renderProjectRunSurface(null), /No dashboard-data\.json run/);
assert.match(cockpit.renderProjectRunSurface({ summary: { id: 'legacy-run' } }), /No project-bound run metadata/);
assert.match(cockpit.renderAuthoringProvenanceSurface(run), /Authoring provenance/);
assert.match(cockpit.renderAuthoringProvenanceSurface(run), /scene-edit-abc123/);
assert.match(cockpit.renderAuthoringProvenanceSurface(run), /beforehash/);
assert.match(cockpit.renderAuthoringProvenanceSurface(run), /--transaction \.omx\/tmp\/transaction\.json/);
assert.match(cockpit.renderAuthoringProvenanceSurface({ summary: { id: 'run-no-tx' }, evidence: [] }), /no scene edit transaction binding/i);
assert.match(cockpit.renderAuthoringProvenanceSurface(null), /No dashboard-data\.json run/);
assert.match(cockpit.renderAuthoringProvenanceSurface({ summary: { id: '<script>' }, evidence: [], transaction_provenance: { transactionId: '<script>alert(1)</script>', scenePath: '<img>', beforeSceneHash: { value: '<bad>' }, afterSceneHash: { value: '<worse>' } } }), /&lt;script&gt;alert/);
assert.match(cockpit.renderJournalSurface(run), /journal summary/);
assert.match(cockpit.renderMutationReviewSurface(run), /mutation review runs\/run-1 --reject/);
assert.match(cockpit.renderMutationReviewSurface(run), /Project-scoped scene mutation lifecycle/);
assert.match(cockpit.renderMutationReviewSurface(run), /scene-application-1/);
assert.match(cockpit.renderMutationReviewSurface(run), /minimal_2d/);
assert.match(cockpit.renderMutationReviewSurface(run), /examples\/project\/ouroforge\.project\.json/);
assert.match(cockpit.renderMutationReviewSurface(run), /--project examples\/project\/ouroforge\.project\.json/);
assert.match(cockpit.renderMutationReviewSurface(run), /project validate examples\/project\/ouroforge\.project\.json/);
assert.match(cockpit.renderMutationReviewSurface(run), /Project-scoped applications/);
assert.match(cockpit.renderMutationReviewSurface(run), /rollback/);
assert.match(cockpit.renderMutationReviewSurface(run), /does not apply, accept, reject, rollback, or merge/);
assert.match(cockpit.renderSceneMutationLifecycleSurface(run), /proposal-1/);
assert.match(cockpit.renderSceneMutationLifecycleSurface(run), /1 record\(s\)/);
assert.match(cockpit.renderSceneMutationLifecycleSurface({ summary: { id: 'legacy-run' }, mutation_lifecycle: { stages: [{ id: 'scene_applied', state: 'applied', record_count: 1, records: [{ id: 'legacy-application', status: 'applied', proposalId: 'proposal-legacy', transactionId: 'scene-edit-legacy', beforeSceneHash: { value: 'before' }, afterSceneHash: { value: 'after' } }] }] } }), /legacy\/no project mutation context recorded/);
assert.match(cockpit.renderSceneMutationLifecycleSurface({ summary: { id: 'run-empty' }, mutation_lifecycle: { stages: [] } }), /No scene-safe proposal records loaded/);
assert.equal(
  cockpit.sceneMutationApplyCommand('runs/run-1', 'mutation/scene-operation.json', 'mutation/scene-edit.json'),
  'cargo run -p ouroforge-cli -- mutation apply-scene runs/run-1 --operation mutation/scene-operation.json --transaction-output mutation/scene-edit.json'
);
assert.equal(
  cockpit.sceneMutationApplyCommand('runs/run-1', 'mutation/scene-operation.json', 'mutation/scene-edit.json', 'ouroforge.project.json'),
  'cargo run -p ouroforge-cli -- mutation apply-scene runs/run-1 --project ouroforge.project.json --operation mutation/scene-operation.json --transaction-output mutation/scene-edit.json'
);
assert.match(cockpit.renderSceneMutationLifecycleSurface({ summary: { id: '<script>' }, mutation_lifecycle: { stages: [{ id: 'scene_applied', state: '<bad>', records: [{ id: '<script>', status: '<img>', proposalId: '<p>', transactionId: '<t>', beforeSceneHash: { value: '<b>' }, afterSceneHash: { value: '<a>' }, project: { projectId: '<project>', manifestPath: '<manifest>', scenePath: '<scene>' }, rollback: { scenePath: '<rollback>', restoreHash: { value: '<hash>' } } }] }] } }), /&lt;script&gt;/);
assert.match(cockpit.renderSceneMutationLifecycleSurface({ summary: { id: '<script>' }, mutation_lifecycle: { stages: [{ id: 'scene_applied', state: '<bad>', records: [{ id: '<script>', status: '<img>', proposalId: '<p>', transactionId: '<t>', beforeSceneHash: { value: '<b>' }, afterSceneHash: { value: '<a>' }, project: { projectId: '<project>', manifestPath: '<manifest>', scenePath: '<scene>' }, rollback: { scenePath: '<rollback>', restoreHash: { value: '<hash>' } } }] }] } }), /&lt;manifest&gt;/);
assert.match(cockpit.renderReplaySurface(run), /replay-1/);
assert.match(cockpit.renderComparisonSurface(run), /before/);
assert.match(cockpit.renderComparisonSurface(run), /after/);
assert.match(cockpit.renderComparisonSurface(run), /Semantic evidence diff/);
assert.match(cockpit.renderComparisonSurface(run), /scene edit transaction provenance changed/);
assert.match(cockpit.renderComparisonSurface(run), /Project comparison summary/);
assert.match(cockpit.renderComparisonSurface(run), /same_project/);
assert.match(cockpit.renderComparisonSurface(run), /scene hash changed for scenes\/main\.scene\.json/);
assert.match(cockpit.renderComparisonSurface(run), /project fixture warning/);
assert.match(cockpit.renderComparisonSurface(run), /Display-only compare command/);
assert.match(cockpit.renderComparisonSurface(run), /compare runs\/before runs\/after/);
assert.match(cockpit.renderComparisonSurface(run), /fixture warning/);
assert.match(cockpit.renderSemanticComparisonSummary({}), /No semantic comparison summary/);
assert.match(cockpit.renderSemanticComparisonSummary({ value: { semantic: { reasons: [{ kind: 'fallback', severity: 'changed', summary: 'fallback semantic' }] } } }), /fallback semantic/);
assert.match(cockpit.renderSemanticComparisonSummary({ value: { semantic: { project: { relation: 'legacy', changed: false, changes: [] } } } }), /No project context changes recorded/);
assert.match(cockpit.renderSemanticComparisonSummary({ value: { semantic: { project: '<bad>' } } }), /No project comparison fields/);
assert.match(cockpit.renderSemanticComparisonSummary({ semantic: { reasons: [{ kind: '<script>', severity: '<img>', summary: '<bad>' }], warnings: ['<warn>'] } }), /&lt;bad&gt;/);
assert.match(cockpit.renderEngineExpansionSurface(run), /Engine Expansion state/);
assert.match(cockpit.renderEngineExpansionSurface(run), /foundation-scene/);
assert.match(cockpit.renderEngineExpansionSurface(run), /runtime-v1-assets/);
assert.match(cockpit.renderEngineExpansionSurface({ engine_summaries: { present: false, empty_state: '<script>x</script>' } }), /&lt;script&gt;x&lt;\/script&gt;/);
assert.match(cockpit.renderComparisonSurface(run), /\.\.\/\.\.\/runs\/before\/verdict\.json/);
assert.match(cockpit.renderStudioGaps(), /No production editor/);
assert.match(cockpit.renderIntegration(run), /Live browser preview/);
assert.match(cockpit.renderIntegration(run), /Pause/);
assert.match(cockpit.renderIntegration(run), /Step 1 frame/);
assert.match(cockpit.renderIntegration(run), /Project workspace/);
assert.match(cockpit.renderIntegration(run), /Project run summary/);
assert.match(cockpit.renderIntegration(run), /Run\/evidence browser/);
assert.match(cockpit.renderIntegration(run), /Authoring provenance/);
assert.match(cockpit.renderIntegration(run), /Journal viewer/);
assert.match(cockpit.renderIntegration(run), /Mutation review state/);
assert.match(cockpit.renderIntegration(run), /Replay controls/);
assert.match(cockpit.renderIntegration(run), /Run comparison/);
assert.match(cockpit.renderIntegration(run), /Engine Expansion state/);
assert.match(cockpit.renderIntegration(run), /Scene editing commands/);
assert.match(cockpit.renderIntegration(run), /Validation command generation/);
assert.match(cockpit.renderIntegration(run), /display-only/);
assert.match(cockpit.renderIntegration(run), /does not write files directly/);
assert.match(cockpit.renderPreviewControls({ ok: false, error: 'probe missing' }), /probe missing/);

let paused = false;
let tick = 0;
let reloads = 0;
const probeWindow = {
  location: { reload: () => { reloads += 1; } },
  __OUROFORGE__: {
    getWorldState: () => ({ tick, paused, entities: [{ id: 'player' }] }),
    getFrameStats: () => ({ tick, fixedDeltaMs: 16 }),
    pause: () => { paused = true; return { tick, paused }; },
    resume: () => { paused = false; return { tick, paused }; },
    step: (frames = 1) => { tick += frames; return { tick }; },
  },
};
assert.equal(cockpit.previewWindow({ contentWindow: probeWindow }), probeWindow);
assert.equal(cockpit.resolvePreviewProbe(probeWindow, ['pause']).ok, true);
const probeRead = cockpit.readPreviewProbe(probeWindow);
assert.equal(probeRead.ok, true);
assert.equal(probeRead.frameStats.tick, 0);
assert.equal(cockpit.callPreviewProbe(probeWindow, 'pause').worldState.paused, true);
assert.equal(cockpit.callPreviewProbe(probeWindow, 'resume').worldState.paused, false);
assert.equal(cockpit.callPreviewProbe(probeWindow, 'step', 2).frameStats.tick, 2);
const previewStateMarkup = cockpit.renderPreviewControls(cockpit.readPreviewProbe(probeWindow));
assert.match(previewStateMarkup, /probe ready/);
assert.match(previewStateMarkup, /Current tick/);
assert.match(previewStateMarkup, /&quot;tick&quot;: 2/);
assert.equal(cockpit.reloadPreview(probeWindow).ok, true);
assert.equal(reloads, 1);
assert.match(cockpit.resolvePreviewProbe(null).error, /window is unavailable/);
assert.match(cockpit.resolvePreviewProbe({}, ['pause']).error, /probe is unavailable/);
assert.match(cockpit.resolvePreviewProbe({ __OUROFORGE__: {} }, ['pause']).error, /missing method/);
assert.match(cockpit.callPreviewProbe({ __OUROFORGE__: { getWorldState: () => { throw new Error('boom'); }, getFrameStats: () => ({}) } }, 'getFrameStats').error, /read failed/);
assert.match(cockpit.reloadPreview({}).error, /reload is unavailable/);

// A cleared numeric input must be rejected, not silently coerced to 0.
assert.throws(() => cockpit.applyEdit(scene, 'player', 'components.transform.x', ''), /Invalid numeric/);

// Scene-derived and dashboard-derived content must be escaped before innerHTML insertion.
const xssScene = {
  entities: [{
    id: '<img src=x onerror=alert(1)>',
    sprite: { color: '#ffffff' },
    components: { transform: { x: 0, y: 0 }, velocity: { x: 0, y: 0 }, size: { width: 1, height: 1 }, controllable: true },
  }],
};
assert.ok(!cockpit.renderTree(xssScene, null).includes('<img src=x onerror'), 'tree entity id must be escaped');
assert.ok(!cockpit.renderInspector(xssScene, xssScene.entities[0].id).includes('<img src=x onerror'), 'inspector entity id must be escaped');
const xssRun = { summary: { id: 'r', run_dir: 'runs/x', verdict_status: 'passed' }, command_context: { command: '<script>alert(1)</script>', seedPath: '<script>seed</script>', workers: '<script>workers</script>', runsRoot: 'runs', scenarioPackId: '<script>pack</script>', runtimeTarget: '<script>runtime</script>', browserBoundary: '<script>boundary</script>', cdpTransport: '<script>transport</script>', environmentHints: ['<script>hint</script>'] }, project: { id: 'p', name: 'p', manifestPath: 'm', seedPath: 's' }, evidence: [], mutations: [], screenshots: [], journal: '<script>alert(1)</script>', replay: { present: false, empty_state: '<script>alert(1)</script>' }, comparison: { present: false, empty_state: '<script>alert(1)</script>' } };
assert.ok(!cockpit.renderEvidencePane(xssRun).includes('<script>alert(1)</script>'), 'evidence journal must be escaped');
assert.ok(!cockpit.renderProjectRunSurface(xssRun).includes('<script>hint</script>'), 'command context hints must be escaped');
const cockpitSource = fs.readFileSync(require.resolve('./cockpit.js'), 'utf8');
assert.ok(!/writeFile|localStorage|indexedDB|showSaveFilePicker|exec\(|spawn\(|child_process/.test(cockpitSource), 'cockpit browser code must not include direct persistence or command execution APIs');
console.log('authoring cockpit smoke test passed');

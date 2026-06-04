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

  loop_dry_run: {
    schemaVersion: 'authoring-loop-dry-run-v1',
    loopId: '<loop-1>',
    status: 'blocked',
    missingPrerequisites: ['record-review:missing decision:<human-review>'],
    boundary: 'Dry-run summary is inert local data; it does not execute commands.',
    steps: [{
      id: '<step-1>',
      kind: 'record-review-decision',
      status: 'pending',
      readiness: 'blocked',
      commandText: 'cargo run -p ouroforge-cli -- mutation review <run> --accept',
      prerequisites: ['artifact:proposal:runs/proposal.json'],
      missingPrerequisites: ['missing decision:<human-review>:human-review'],
      expectedArtifacts: [{ id: 'decision', path: 'runs/review-decision.json' }],
      requiredDecisions: [{ id: '<human-review>', kind: 'human-review' }],
      safetyGates: ['inert command text only'],
    }],
  },
  loop_execution: {
    schemaVersion: 'authoring-loop-step-execution-v1',
    loopId: '<loop-1>',
    stepId: '<step-1>',
    kind: 'apply-accepted-scene-mutation',
    status: 'completed',
    ledgerPath: '<ledger>',
    generatedArtifacts: [{ id: '<transaction>', kind: 'scene-edit-transaction', path: 'runs/tx.json' }],
    blockedReasons: [],
    boundary: 'CLI-only Rust trusted step runner; browser does not execute.',
  },
  loop_recovery: {
    schemaVersion: 'authoring-loop-status-v1',
    loopId: '<loop-1>',
    status: 'needs-recovery',
    nextSafeAction: 'Resolve <manual> action',
    boundary: 'Status is read-only inspection.',
    steps: [{
      id: '<step-2>',
      kind: 'compare-runs',
      status: 'blocked',
      recovery: {
        failure: { reason: '<missing comparison>' },
        manualAction: { description: 'Restore <comparison> artifact' },
      },
      missingPrerequisites: ['missing artifact:<comparison>'],
      nextSafeAction: 'Restore <comparison> artifact',
    }],
  },
  agent_handoffs: [{
    schemaVersion: 'agent-handoff-contract-v1',
    loopId: '<handoff-loop>',
    currentStep: { stepId: '<step-4>', kind: 'compare-runs', status: 'blocked' },
    status: 'blocked',
    nextSafeAction: 'Resolve <manual> blocker',
    requiredDecisions: [{ id: '<human-review>', kind: 'human-review' }],
    blockers: ['missing <comparison>'],
    allowedCommands: [{ command: 'cargo run -p ouroforge-cli -- loop status <plan>', argv: [], boundary: 'inert display text only; cockpit does not execute' }],
    forbiddenActions: ['Do not execute <browser> commands'],
    evidenceRefs: [{ id: 'handoff', path: 'runs/<handoff>/handoff.json' }],
    driftGuardrails: ['#1 remains open', '#23 remains open'],
    generatedState: { roots: ['runs'], trackedFixtureOnly: true },
    boundary: 'advisory evidence only; does not execute commands',
  }],
  loop_evidence_bundles: [{
    schemaVersion: 'authoring-loop-evidence-bundle-v1',
    loopId: '<loop-bundle>',
    status: 'partial',
    plan: { id: 'loop-plan', kind: 'loop-plan', path: '<loop-plan.json>' },
    steps: [{ stepId: '<step-3>', kind: 'compare-runs', status: 'blocked' }],
    runs: [{ id: 'baseline', kind: 'run', path: 'runs/<baseline>/run.json' }],
    comparisons: [{ id: 'comparison', kind: 'comparison', path: 'runs/comparison.json' }],
    proposals: [],
    reviewDecisions: [],
    transactions: [],
    regressionPromotions: [],
    matrixSnapshots: [],
    journalSummaries: [],
    missingRefs: ['comparison:runs/<comparison>.json'],
    boundary: 'Generated local index only; <browser> read-only.',
  }],
  loop_cockpit: {
    schemaVersion: 'studio-loop-cockpit-v1',
    boundary: 'Read-only normalized loop cockpit read model; <browser> cannot execute commands.',
    loops: [{
      loopId: '<cockpit-loop>',
      status: 'blocked',
      planPath: '<loop-plan.json>',
      currentStep: { stepId: '<step-current>', kind: 'compare-runs', status: 'blocked' },
      steps: [
        { stepId: '<step-plan>', kind: 'run-baseline', status: 'completed', path: 'runs/<baseline>/run.json' },
        { stepId: '<step-current>', kind: 'compare-runs', status: 'blocked', path: 'runs/<comparison>.json' },
      ],
      blockers: ['missing <comparison>'],
      requiredDecisions: [{ id: '<human-review>', kind: 'human-review' }],
      nextSafeAction: 'Copy <status> command manually',
      allowedCommands: [{ command: 'cargo run -p ouroforge-cli -- loop status <plan>', boundary: 'inert display text only' }],
      forbiddenActions: ['Do not click <run> controls'],
      evidenceRefs: [{ id: '<bundle>', path: 'runs/<bundle>/bundle.json' }],
      bundleStatus: 'partial',
      handoffStatus: 'blocked',
      bundleMissingRefs: ['comparison:<missing>'],
      boundary: 'Display-only cockpit row; does not execute commands.',
    }],
  },
  evidence_fidelity: {
    transaction: { id: 'transaction', label: 'Transaction provenance', status: 'present', summary: 'Transaction scene-edit-abc123 records scene edit provenance.', observed_count: 1, missing_count: 0, warnings: [], evidence_refs: ['transactions/scene-edit.json'] },
    runtime_probe: { id: 'runtime_probe', label: 'Runtime probe contract', status: 'present', summary: 'Runtime probe contract present.', observed_count: 2, missing_count: 0, warnings: [], evidence_refs: ['evidence/world.json'] },
    input_replay: { id: 'input_replay', label: 'Input replay evidence', status: 'missing', summary: 'No replay evidence.', observed_count: 0, missing_count: 1, warnings: ['Input replay evidence is unavailable.'], evidence_refs: [] },
    openchrome_cdp: { id: 'openchrome_cdp', label: 'Openchrome/CDP evidence', status: 'partial', summary: '1 screenshot, 1 console log, 0 performance metric, 1 CDP trace summary.', observed_count: 3, missing_count: 1, warnings: ['Missing performance metrics.'], evidence_refs: ['evidence/shot.png', 'evidence/console.json'] },
    command_context: { id: 'command_context', label: 'Reproducible command context', status: 'present', summary: '4 workers, seed seeds/platformer.yaml.', observed_count: 1, missing_count: 0, warnings: [], evidence_refs: [] },
  },
  asset_loading: {
    present: true,
    attempt_count: 2,
    loaded_count: 1,
    failed_count: 1,
    rejected_count: 0,
    fallback_count: 0,
    boundary: 'Read-only runtime loading evidence; cockpit never fetches remote assets or writes trusted state.',
    records: [
      { attemptId: 'load-player-sprite', assetId: 'player_sprite', path: 'assets/sprites/player.png', status: 'loaded', loadDurationMs: 8 },
      { attemptId: 'load-missing-audio', assetId: 'missing_audio', path: 'assets/audio/missing.ogg', status: 'failed', failureReason: 'Image load failed' },
    ],
  },
  asset_preview: {
    present: true,
    preview_count: 3,
    warning_count: 1,
    image_count: 1,
    atlas_frame_count: 1,
    tilemap_count: 1,
    boundary: 'Read-only asset preview evidence; cockpit never fetches remote assets or writes trusted state.',
    records: [
      { assetId: 'player_sprite', assetType: 'image', sourcePath: 'assets/sprites/player.png', previewKind: 'thumbnail', image: { width: 16, height: 16 } },
      { assetId: 'player_atlas', assetType: 'sprite_atlas', sourcePath: 'assets/atlases/player.atlas.json', previewKind: 'thumbnail', atlasFrames: [{ frameId: 'idle_0', rect: { x: 0, y: 0, width: 16, height: 16 } }] },
      { assetId: 'level_tilemap', assetType: 'tilemap', sourcePath: 'assets/tilemaps/level.json', previewKind: 'metadata', tilemap: { tilesetAssetId: 'terrain_tiles', width: 4, height: 3, layerCount: 2, tileCount: 12 } },
    ],
    warnings: [{ assetId: 'missing_audio', kind: 'missing_asset_file', message: 'missing audio preview source', path: 'assets/audio/missing.ogg' }],
  },
  visual_diff_preview: {
    present: true,
    summary_count: 2,
    operation_count: 2,
    status: 'preview-only',
    boundary: 'Visual diff previews are read-only; browser cannot apply edits, write files, execute commands, or persist drafts.',
    summaries: [
      {
        schemaVersion: 'visual-diff-summary-v1',
        summaryId: 'visual-diff-scene-draft',
        target: { type: 'scene', id: 'main', path: 'examples/game-runtime/scene.json' },
        sourceRefs: { draftId: 'draft-scene-1', transactionId: 'tx-scene-1', proposalId: 'proposal-1', journalRef: 'journal.md#visual-diff', dashboardRef: 'dashboard-data.json#run-1' },
        before: { summaryText: 'Player before scene edit.', entitySummaries: [{ entityId: 'player', change: 'updated', summary: 'player before' }], collisionTriggerSummary: { collisionCellsAffected: 0, triggerCellsAffected: 0 } },
        after: { summaryText: 'Player after scene edit.', entitySummaries: [{ entityId: 'player', change: 'updated', summary: 'player x changed' }], collisionTriggerSummary: { collisionCellsAffected: 0, triggerCellsAffected: 0 } },
        operationSummaries: [{ operationId: 'op-player-x', transactionId: 'tx-scene-1', target: 'scene', change: 'updated', path: 'components.transform.x', summary: 'Move player x from 32 to 48.', affectedEntityIds: ['player'], collisionTriggerSummary: { collisionCellsAffected: 0, triggerCellsAffected: 0 } }],
        expectedScenarioImpact: { status: 'unknown', summary: 'Scenario impact requires separate evidence.' },
        guardrail: 'read-only visual diff summary; no apply',
      },
      {
        schemaVersion: 'visual-diff-summary-v1',
        summaryId: 'visual-diff-tilemap-draft',
        target: { type: 'tilemap', id: 'level', path: 'assets/tilemaps/level.json' },
        sourceRefs: { draftId: 'draft-tile-1', dashboardRef: 'dashboard-data.json#tilemap' },
        before: { summaryText: 'Tilemap before draft.', tileSummaries: [{ tilemapId: 'level', layerId: 'terrain', change: 'unchanged', summary: 'before tile', affectedCells: 1, tileIds: ['solid_ground'] }], collisionTriggerSummary: { collisionCellsAffected: 0, triggerCellsAffected: 0 } },
        after: { summaryText: 'Tilemap after draft.', tileSummaries: [{ tilemapId: 'level', layerId: 'terrain', change: 'updated', summary: 'trigger tile added', affectedCells: 1, tileIds: ['coin_trigger'] }], collisionTriggerSummary: { collisionCellsAffected: 0, triggerCellsAffected: 1, triggerIds: ['coin_collected'] } },
        operationSummaries: [{ operationId: 'op-trigger-preview', target: 'tilemap', change: 'updated', path: 'layers.terrain[4]', summary: 'Preview trigger tile.', affectedTilemapIds: ['level'], collisionTriggerSummary: { collisionCellsAffected: 0, triggerCellsAffected: 1, triggerIds: ['coin_collected'] } }],
        expectedScenarioImpact: { status: 'unknown', summary: 'Requires scenario rerun.' },
        guardrail: 'read-only visual diff summary; no apply',
      },
    ],
  },
  tilemap_draft_preview: {
    present: true,
    preview_count: 2,
    status: 'preview-only',
    boundary: 'Tilemap draft previews are read-only; browser cannot write tilemaps, execute commands, or apply reviews.',
    records: [
      {
        operationId: 'op-collision-preview',
        kind: 'collision_preview',
        layerId: 'collision',
        affectedCells: 1,
        beforeTilemapHash: { algorithm: 'fnv1a64-file-v1', value: 'beforehash' },
        afterTilemapHash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'afterhash' },
        summary: 'Preview collision metadata for one tile.',
        collisionCells: [{ layerId: 'collision', x: 0, y: 1, index: 3, tileId: 'solid_ground' }],
        triggerCells: [],
      },
      {
        operationId: 'op-trigger-preview',
        kind: 'trigger_preview',
        layerId: 'terrain',
        affectedCells: 1,
        beforeTilemapHash: { algorithm: 'fnv1a64-file-v1', value: 'beforehash' },
        afterTilemapHash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'afterhash2' },
        summary: 'Preview trigger metadata for one tile.',
        collisionCells: [],
        triggerCells: [{ layerId: 'terrain', x: 1, y: 1, index: 4, tileId: 'coin_trigger', trigger: 'coin_collected' }],
      },
    ],
  },
  asset_inspector: {
    present: true,
    status: 'warning',
    asset_count: 4,
    warning_count: 1,
    runtime_attempt_count: 2,
    loaded_count: 1,
    failed_count: 1,
    preview_count: 3,
    atlas_frame_count: 1,
    tilemap_count: 1,
    boundary: 'Read-only Studio asset inspector; browser cannot upload assets, write manifests, fetch remote assets, or execute commands.',
    evidence_refs: ['asset-integrity.json', 'asset-loading.json', 'asset-preview.json'],
    assets: [
      { assetId: 'player_sprite', assetType: 'image', sourcePath: 'assets/sprites/player.png', contentHash: 'hash-player', runtimeStatuses: ['Loaded'], warnings: [] },
      { assetId: 'player_atlas', assetType: 'sprite_atlas', sourcePath: 'assets/atlases/player.atlas.json', contentHash: 'hash-atlas', atlasFrameCount: 1, warnings: [] },
      { assetId: 'level_tilemap', assetType: 'tilemap', sourcePath: 'assets/tilemaps/level.json', contentHash: 'hash-tilemap', tilemap: { tilesetAssetId: 'terrain_tiles', width: 4, height: 3, layerCount: 2, tileCount: 12 }, warnings: [] },
      { assetId: 'missing_audio', assetType: 'audio', sourcePath: 'assets/audio/missing.ogg', contentHash: 'hash-missing', runtimeStatuses: ['Failed'], warnings: ['missing_asset_file'] },
    ],
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
  mutations: [{ id: 'mutation-1', status: 'proposed', evidence_id: 'verdict-1', target: 'seeds/platformer.yaml', rationale: { schema_version: '1', failure_classification: 'scenario_assertion_failure', evidence_artifact_ids: ['verdict-1'], scenario_result_refs: ['evidence/scenario.json'], verdict_refs: ['verdict.json'], expected_effect: 'player reaches the goal', confidence: 'medium', reasoning_summary: 'scenario assertion failed', allowed_mutation_type: 'data_only' } }],
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
  review_cockpit: {
    schemaVersion: 'ouroforge-studio-review-cockpit-v1',
    terminalState: 'applied',
    boundary: 'read-only exported evidence; commands are inert text only',
    commandHints: ['cargo run -p ouroforge-cli -- mutation review runs/run-1 --accept --reason "manual evidence review accepted"'],
    proposals: { id: 'proposed', label: 'Proposals', state: 'proposed', artifactPath: 'mutation/proposals.json', recordCount: 1, recordIds: ['proposal-1'], evidenceRefs: ['evidence/indexed.json'] },
    decisions: { id: 'reviewed', label: 'Review decisions', state: 'accepted', artifactPath: 'mutation/review-decisions.json', recordCount: 1, recordIds: ['review-decision-1'], evidenceRefs: ['mutation/rerun-orchestration.json'] },
    applications: { id: 'scene_applied', label: 'Review-gated applications', state: 'applied', artifactPath: 'mutation/scene-applications.json', recordCount: 1, recordIds: ['scene-application-1'], evidenceRefs: ['mutation/scene-applications.json'] },
    comparisons: { id: 'compared', label: 'Rerun comparisons', state: 'compared', artifactPath: 'mutation/rerun-orchestration.json', recordCount: 1, recordIds: ['comparison-1'], evidenceRefs: ['mutation/rerun-orchestration.json'] },
    promotions: { id: 'promotions', label: 'Regression promotions', state: 'promoted', artifactPath: 'regression-promotions/*.json', recordCount: 1, recordIds: ['regression-promotion-1'], evidenceRefs: ['regression-promotions/regression-promotion-1.json'] },
    matrix: { id: 'matrix', label: 'Regression matrix', state: 'export_level', artifactPath: 'regression_matrix', recordCount: 1, recordIds: ['scaffold-smoke'], evidenceRefs: [] },
  },
  mutation_lifecycle: {
    terminal_state: 'pending_review',
    command_hints: ['cargo run -p ouroforge-cli -- mutation review runs/run-1 --reject --reason "manual"'],
    stages: [
      { id: 'reviewed', label: 'Manual review', state: 'accepted', artifact_path: 'mutation/review-decisions.json', record_count: 1, records: [{ id: 'review-decision-1', proposal_id: 'mutation-1', patch_draft_id: 'patch-draft-1', state: 'accepted', decision_status: 'accepted', reviewer_type: 'human', reviewer: 'manual-reviewer', reason: '<b>accepted</b>', evidence_refs: ['mutation/rerun-orchestration.json'] }] },
      { id: 'proposed', label: 'Proposed', state: 'proposed', artifact_path: 'mutation/proposals.json', record_count: 1, records: [{ id: 'proposal-1', evidence_id: 'verdict-1' }] },
      { id: 'scene_applied', label: 'Applied scene mutation', state: 'applied', artifact_path: 'mutation/scene-applications.json', record_count: 1, records: [{ id: 'scene-application-1', proposalId: 'proposal-1', transactionId: 'scene-edit-abc123', reviewDecisionId: 'review-decision-1', targetScenePath: 'examples/project/scenes/main.scene.json', transactionArtifactPath: 'mutation/scene-edit.json', beforeSceneHash: { value: 'beforehash' }, afterSceneHash: { value: 'afterhash' }, project: { projectId: 'minimal_2d', manifestPath: 'examples/project/ouroforge.project.json', scenePath: 'scenes/main.scene.json' }, rollback: { scenePath: 'examples/project/scenes/main.scene.json', restoreHash: { value: 'beforehash' } }, status: 'applied' }] },
    ],
  },
  regression_matrix: {
    schemaVersion: 'ouroforge-regression-run-matrix-v1',
    skippedRuns: [{ runId: 'legacy-run', runDir: 'runs/legacy-run', reason: 'missing_or_malformed_project_context' }],
    projects: [{
      projectId: 'minimal_2d',
      projectName: 'Minimal 2D Ouroforge Project',
      scenarioPacks: [{
        scenarioPackId: 'smoke',
        scenarioPackPath: 'scenarios/smoke.scenario-pack.json',
        scenarios: [{
          scenarioId: 'scaffold-smoke',
          currentStatus: 'failed',
          lastPass: { runId: 'run-pass', runDir: 'runs/run-pass', createdAtUnixMs: 1, status: 'passed', scenarioResultPath: 'evidence/scenarios/scaffold-smoke/scenario-result.json', verdictStatus: 'passed', evidenceRefs: ['evidence/scenarios/scaffold-smoke/scenario-result.json'] },
          lastFail: { runId: 'run-1', runDir: 'runs/run-1', createdAtUnixMs: 2, status: 'failed', scenarioResultPath: 'evidence/scenarios/scaffold-smoke/scenario-result.json', verdictStatus: 'failed', evidenceRefs: ['evidence/scenarios/scaffold-smoke/scenario-result.json'] },
          runs: [
            { runId: 'run-pass', runDir: 'runs/run-pass', createdAtUnixMs: 1, status: 'passed', scenarioResultPath: 'evidence/scenarios/scaffold-smoke/scenario-result.json', verdictStatus: 'passed', evidenceRefs: [] },
            { runId: 'run-1', runDir: 'runs/run-1', createdAtUnixMs: 2, status: 'failed', scenarioResultPath: 'evidence/scenarios/scaffold-smoke/scenario-result.json', verdictStatus: 'failed', evidenceRefs: [] },
          ],
          context: { mutationIds: ['mutation-1'], reviewDecisionIds: ['review-decision-1'], promotionIds: ['regression-promotion-1'] },
        }],
      }],
    }],
  },
  regression_promotions: [{
    schemaVersion: 'regression-promotion-result-v1',
    id: 'regression-promotion-1',
    draftId: 'regression-draft-1',
    scenarioId: 'promoted-smoke-regression',
    sourceRun: { runId: 'run-1', runDir: 'runs/run-1', verdictPath: 'verdict.json' },
    target: { projectManifestPath: 'ouroforge.project.json', scenarioPackId: 'smoke', scenarioPackPath: 'scenarios/smoke.scenario-pack.json', scenarioGroupId: 'promoted-regressions' },
    dryRun: false,
    createdGroup: true,
    beforeHash: { algorithm: 'fnv1a64-file-v1', value: 'beforepack' },
    afterHash: { algorithm: 'fnv1a64-file-v1', value: 'afterpack' },
    changes: ['added_scenario:promoted-smoke-regression'],
    recordPath: 'regression-promotions/regression-promotion-1.json',
  }],
  replay: { present: true, sequences: [{ id: 'replay-1', event_count: 2, frames: [0, 4], evidence_refs: ['evidence/replay.json'] }] },
  comparison: { present: true, artifacts: [{ before_run_id: 'before', after_run_id: 'after', classification: 'improved', path: 'mutation/run-comparison-before--after.json', evidence_refs: ['runs/before/verdict.json', 'runs/after/verdict.json'], semantic: { schemaVersion: 'run-semantic-diff-v1', reasons: [{ kind: 'transaction_provenance', severity: 'changed', summary: 'scene edit transaction provenance changed' }], scenarios: [], worldState: { changed: [] }, project: { relation: 'same_project', changed: true, changes: [{ kind: 'scene_hash', summary: 'scene hash changed for scenes/main.scene.json', before: 'before-scene', after: 'after-scene' }], warnings: ['project fixture warning'] }, transactionProvenance: { changed: true }, warnings: ['fixture warning'] } }] },
  engine_summaries: {
    present: true,
    source_world_state: 'evidence/world.json',
    scene: { sceneId: 'trigger-flags-v1-fixture', entityCount: 3, tick: 1 },
    renderer: { version: '1', renderedEntities: 3, camera: { x: 0, y: 0 } },
    tilemaps: { tilemapCount: 0, layerCount: 0 },
    assets: { manifestId: null, assetCount: 0 },
    animation: { animatedEntityCount: 0 },
    audio: { audioEntityCount: 0, audioEventCount: 0 },
    physics: { colliderEntityCount: 2, collisionEventCount: 1 },
    collision: {
      present: true,
      rules: { player_key: { a: 'player', b: 'key', event: 'collect_key' } },
      colliderEntityCount: 2,
      collisionCount: 1,
      collisionEventCount: 1,
      events: [{ type: 'runtime.collision.trigger', a: 'player', b: 'key', triggerId: 'collect_key' }],
    },
    gameplay: { worldFlagCount: 3, trueFlagCount: 2, triggerCollisionEventCount: 1, hudValueEntityCount: 2 },
    components: {
      present: true,
      entityCount: 2,
      componentCounts: { controllable: 1, hudValue: 2, trigger: 1 },
      entities: [
        { entityId: 'player', components: ['controllable', 'status'] },
        { entityId: 'hud_goal', components: ['hudValue', 'uiText'] },
      ],
    },
    triggers: {
      present: true,
      triggerCount: 1,
      triggerCollisionEventCount: 1,
      triggers: [{ entityId: 'key', id: 'collect_key', kind: 'overlap', targetFlag: 'coin_collected', requiredFlags: ['player_alive'], onEnterCount: 2 }],
    },
    hud: {
      present: true,
      hudValueCount: 2,
      hudValueEntityCount: 2,
      values: [
        { entityId: 'hud_goal', kind: 'goal', label: 'Goal', value: 'Collect coin', bindFlag: 'coin_collected', flagValue: true, text: 'Goal: Collect coin' },
        { entityId: 'hud_health', kind: 'health', label: 'HP', value: '3/3', text: 'HP: 3/3' },
      ],
    },
    transition: {
      present: true,
      currentSceneId: 'scene-main',
      declaredTransitionCount: 1,
      declaredTransitions: [{ id: 'to_boss', toScene: 'scenes/boss.scene.json', label: 'Boss' }],
      transitionEventCount: 1,
      transitions: [{ type: 'runtime.scene.transition.succeeded', fromSceneId: 'scene-main', toSceneId: 'scene-boss' }],
      reloadCount: 1,
      lastReloadStatus: 'ok',
    },
    events: {
      present: true,
      animationEntityCount: 1,
      audioEventCount: 1,
      collisionEventCount: 1,
      animationEntities: [{ entityId: 'player', mode: 'playing', currentClip: 'run', frameIndex: 3 }],
      audioEvents: [{ type: 'runtime.audio.play', clipId: 'coin' }],
      collisionEvents: [{ type: 'runtime.collision.trigger', triggerId: 'collect_key' }],
    },
    reload: { reloadCount: 0, lastStatus: null },
    composition: { entityCount: 3, parentedEntityCount: 0 },
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
assert.equal(cockpit.studioSurfaceSummary(run).filter((surface) => surface.present).length, 19);
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
assert.match(cockpit.renderEvidenceFidelitySurface(run), /Evidence fidelity/);
assert.match(cockpit.renderEvidenceFidelitySurface(run), /Transaction provenance/);
assert.match(cockpit.renderEvidenceFidelitySurface(run), /Missing performance metrics/);
assert.match(cockpit.renderEvidenceFidelitySurface(run), /\.\.\/\.\.\/runs\/run-1\/evidence\/world\.json/);
assert.doesNotMatch(cockpit.renderEvidenceFidelitySurface(run), /\.\.\/\.\.\/evidence\/world\.json/);
assert.match(cockpit.renderEvidenceFidelitySurface({ summary: { id: 'legacy-run' } }), /No evidence fidelity read model/);
const malformedFidelity = cockpit.renderEvidenceFidelitySurface({ summary: { id: 'malformed' }, evidence_fidelity: { transaction: '<script>bad</script>', runtime_probe: { label: 'Runtime', status: 'present evil', summary: '<b>safe</b>', warnings: '<script>warn</script>', evidence_refs: '<script>ref</script>' } } });
assert.match(malformedFidelity, /No Transaction provenance read-model data/);
assert.match(malformedFidelity, /class="fidelity-card unknown"/);
assert.ok(!malformedFidelity.includes('<b>safe</b>'), 'malformed fidelity summary must be escaped');
assert.equal(cockpit.fidelityStatusClass('present evil'), 'unknown');
assert.equal(cockpit.fidelityStatusClass('partial'), 'partial');
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
assert.match(cockpit.renderMutationReviewSurface(run), /Studio review cockpit/);
assert.match(cockpit.renderMutationReviewSurface(run), /ouroforge-studio-review-cockpit-v1/);
assert.match(cockpit.renderMutationReviewSurface(run), /proposal-1/);
assert.match(cockpit.renderMutationReviewSurface(run), /review-decision-1/);
assert.match(cockpit.renderMutationReviewSurface(run), /scene-application-1/);
assert.match(cockpit.renderMutationReviewSurface(run), /Inert copyable review commands/);
assert.doesNotMatch(cockpit.renderMutationReviewSurface(run), /<button/i);
assert.match(cockpit.renderStudioReviewCockpitCards({ review_cockpit: { schemaVersion: '<script>', terminalState: '<bad>', boundary: '<img>', commandHints: ['<script>alert(1)</script>'], proposals: { label: '<proposal>', state: 'malformed', readError: '<b>bad</b>', recordIds: ['<id>'], evidenceRefs: ['<ref>'] }, decisions: '<script>', applications: { label: 'Applications', state: 'missing', recordIds: [], evidenceRefs: [] } } }), /&lt;script&gt;/);
assert.match(cockpit.renderStudioReviewCockpitCards({ review_cockpit: { proposals: { label: 'Proposals', state: 'missing', recordIds: [], evidenceRefs: [] }, decisions: { label: 'Decisions', state: 'missing', recordIds: [], evidenceRefs: [] }, applications: { label: 'Applications', state: 'missing', recordIds: [], evidenceRefs: [] } } }), /No record ids exported/);
// Studio review cockpit must render the exported comparisons and promotions stages too, not only proposals/decisions/applications.
assert.match(cockpit.renderStudioReviewCockpitCards(run), /Rerun comparisons/);
assert.match(cockpit.renderStudioReviewCockpitCards(run), /Regression promotions/);
assert.match(cockpit.renderRegressionPromotionSurface(run), /Regression promotions/);
assert.match(cockpit.renderRegressionPromotionSurface(run), /promoted-smoke-regression/);
assert.match(cockpit.renderRegressionPromotionSurface(run), /Regression promotions/);
assert.match(cockpit.renderRegressionPromotionSurface(run), /regression-promotions\/\*\.json/);
assert.match(cockpit.renderRegressionPromotionSurface(run), /regression-promotion-1/);
assert.match(cockpit.renderRegressionPromotionSurface(run), /scenario promote &lt;draft-json&gt; --project examples\/project\/ouroforge\.project\.json --scenario-pack smoke --dry-run/);
assert.match(cockpit.renderRegressionPromotionSurface({ regression_promotions: [], review_cockpit: { promotions: { label: 'Regression promotions', state: 'malformed', readError: '<bad promotion index>', recordIds: ['<promotion>'], evidenceRefs: [] } } }), /&lt;bad promotion index&gt;/);
assert.match(cockpit.renderRegressionPromotionSurface({ regression_promotions: [] }), /No regression promotion records/);
assert.match(cockpit.renderRegressionMatrixSurface(run), /Regression run matrix/);
assert.match(cockpit.renderRegressionMatrixSurface(run), /Regression matrix/);
assert.match(cockpit.renderRegressionMatrixSurface(run), /regression_matrix/);
assert.match(cockpit.renderRegressionMatrixSurface(run), /scaffold-smoke/);
assert.match(cockpit.renderRegressionMatrixSurface(run), /does not schedule CI/);
assert.match(cockpit.renderRegressionMatrixSurface({ regression_matrix: { projects: [{ projectId: '<script>', scenarioPacks: [{ scenarioPackId: '<pack>', scenarios: [{ scenarioId: '<scenario>', currentStatus: '<bad>', runs: [], context: {} }] }] }], skippedRuns: [] } }), /&lt;script&gt;/);
// An empty-projects matrix export (e.g. all runs skipped) still surfaces the matrix stage summary status/readError.
assert.match(cockpit.renderRegressionMatrixSurface({ regression_matrix: { projects: [], skippedRuns: [] }, review_cockpit: { matrix: { label: 'Regression matrix', state: 'export_level', readError: '<empty matrix note>', recordIds: [], evidenceRefs: [] } } }), /No project-bound scenario runs available/);
assert.match(cockpit.renderRegressionMatrixSurface({ regression_matrix: { projects: [], skippedRuns: [] }, review_cockpit: { matrix: { label: 'Regression matrix', state: 'export_level', readError: '<empty matrix note>', recordIds: [], evidenceRefs: [] } } }), /&lt;empty matrix note&gt;/);
assert.match(cockpit.renderRegressionMatrixSurface({ review_cockpit: { matrix: { label: '<matrix>', state: 'malformed', readError: '<bad matrix>', recordIds: ['<scenario>'], evidenceRefs: [] } } }), /&lt;bad matrix&gt;/);
assert.match(cockpit.renderRegressionMatrixSurface({}), /No regression matrix export/);
assert.match(cockpit.renderMutationReviewSurface(run), /Proposal rationale/);
assert.match(cockpit.renderMutationReviewSurface(run), /scenario_assertion_failure/);
assert.match(cockpit.renderMutationReviewSurface(run), /player reaches the goal/);
assert.match(cockpit.renderProposalRationaleSurface(run), /verdict refs verdict\.json/);
assert.match(cockpit.renderProposalRationaleSurface({ mutations: [{ id: '<script>', rationale: { failure_classification: '<img>', evidence_artifact_ids: ['<svg>'], expected_effect: '<b>', confidence: '<i>', reasoning_summary: '<p>', allowed_mutation_type: '<bad>' } }] }), /&lt;script&gt;/);
assert.match(cockpit.renderProposalRationaleSurface({ mutations: [{ id: 'missing' }] }), /No proposal rationale recorded/);
const directMutationWithoutRationale = { id: 'proposal-with-staged-rationale', status: 'proposed', target: 'seeds/platformer.yaml' };
const stagedRationaleSurface = cockpit.renderProposalRationaleSurface({
  mutations: [directMutationWithoutRationale],
  mutation_lifecycle: {
    stages: [
      {
        id: 'proposed',
        records: [
          {
            id: 'proposal-with-staged-rationale',
            rationale: {
              failure_classification: 'staged_failure_classification',
              evidence_artifact_ids: ['staged-verdict'],
              expected_effect: 'staged rationale fills the direct mutation',
              confidence: 'high',
              reasoning_summary: 'direct summary omitted rationale',
              allowed_mutation_type: 'data_only',
            },
          },
          {
            id: 'staged-only-proposal',
            rationale: {
              failure_classification: 'staged_only_failure',
              evidence_artifact_ids: ['staged-only-verdict'],
              expected_effect: 'staged-only proposal still renders',
              confidence: 'medium',
              reasoning_summary: 'appended after direct proposals',
              allowed_mutation_type: 'data_only',
            },
          },
        ],
      },
    ],
  },
});
assert.match(stagedRationaleSurface, /staged rationale fills the direct mutation/);
assert.ok(stagedRationaleSurface.indexOf('proposal-with-staged-rationale') < stagedRationaleSurface.indexOf('staged-only-proposal'));
assert.equal(directMutationWithoutRationale.rationale, undefined);
assert.match(cockpit.renderMutationReviewSurface(run), /scene-application-1/);
assert.match(cockpit.renderMutationReviewSurface(run), /minimal_2d/);
assert.match(cockpit.renderMutationReviewSurface(run), /examples\/project\/ouroforge\.project\.json/);
assert.match(cockpit.renderMutationReviewSurface(run), /--project examples\/project\/ouroforge\.project\.json/);
assert.match(cockpit.renderMutationReviewSurface(run), /project validate examples\/project\/ouroforge\.project\.json/);
assert.match(cockpit.renderMutationReviewSurface(run), /Project-scoped applications/);
assert.match(cockpit.renderMutationReviewSurface(run), /rollback/);
assert.match(cockpit.renderMutationReviewSurface(run), /review-decision-1/);
assert.match(cockpit.renderMutationReviewSurface(run), /does not apply, accept, reject, rollback, or merge/);
assert.match(cockpit.renderSceneMutationLifecycleSurface(run), /proposal-1/);
assert.match(cockpit.renderSceneMutationLifecycleSurface(run), /1 record\(s\)/);
assert.match(cockpit.renderSceneMutationLifecycleSurface(run), /review decision review-decision-1/);
// The copyable apply-scene command must NOT embed a recorded application's
// already-consumed decision id; the Rust preflight rejects reusing it, so such
// a command would always fail for the exact records this surface displays.
assert.match(cockpit.renderSceneMutationLifecycleSurface(run), /mutation apply-scene/);
assert.doesNotMatch(cockpit.renderSceneMutationLifecycleSurface(run), /--decision review-decision-1/);
assert.match(cockpit.renderSceneMutationLifecycleSurface({ summary: { id: 'visual-run' }, mutation_lifecycle: { stages: [{ id: 'visual_draft_applied', state: 'applied', record_count: 1, records: [{ id: 'visual-edit-application-1', status: 'applied', draftId: 'draft-1', proposalId: 'proposal-1', patchDraftId: 'patch-draft-1', reviewDecisionId: 'review-decision-1', transactionId: 'scene-edit-1', beforeSceneHash: { value: 'before' }, afterSceneHash: { value: 'after' }, commandContext: { command: 'cargo run -p ouroforge-cli -- edit draft-apply draft.json' } }] }] } }), /Visual draft application records/);
assert.match(cockpit.renderSceneMutationLifecycleSurface({ summary: { id: 'visual-run' }, mutation_lifecycle: { stages: [{ id: 'visual_draft_applied', state: 'applied', record_count: 1, records: [{ id: 'visual-edit-application-1', status: 'applied', draftId: 'draft-1', proposalId: 'proposal-1', patchDraftId: 'patch-draft-1', reviewDecisionId: 'review-decision-1', transactionId: 'scene-edit-1', beforeSceneHash: { value: 'before' }, afterSceneHash: { value: 'after' }, commandContext: { command: 'cargo run -p ouroforge-cli -- edit draft-apply draft.json' } }] }] } }), /display-only rerun context/);
assert.match(cockpit.renderSceneMutationLifecycleSurface({ summary: { id: '<script>' }, mutation_lifecycle: { stages: [{ id: 'visual_draft_applied', state: 'applied', record_count: 1, records: [{ id: '<script>', status: '<img>', draftId: '<draft>', proposalId: '<proposal>', patchDraftId: '<patch>', reviewDecisionId: '<decision>', transactionId: '<tx>', beforeSceneHash: { value: '<before>' }, afterSceneHash: { value: '<after>' }, commandContext: { command: '<command>' } }] }] } }), /&lt;draft&gt;/);
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
assert.equal(
  cockpit.sceneMutationApplyCommand('runs/run-1', 'mutation/scene-operation.json', 'mutation/scene-edit.json', 'ouroforge.project.json', 'review-decision-1'),
  'cargo run -p ouroforge-cli -- mutation apply-scene runs/run-1 --project ouroforge.project.json --operation mutation/scene-operation.json --decision review-decision-1 --transaction-output mutation/scene-edit.json'
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
assert.match(cockpit.renderStudioNavigation(run), /Expressive scene inspection/);
assert.match(cockpit.renderExpressiveComponentHudSurface(run), /Component counts/);
assert.match(cockpit.renderExpressiveComponentHudSurface(run), /collect_key/);
assert.match(cockpit.renderExpressiveComponentHudSurface(run), /Goal: Collect coin/);
assert.match(cockpit.renderExpressiveComponentHudSurface({ engine_summaries: { present: true, components: '<bad>', triggers: null, hud: [] } }), /component summary missing or malformed/);
assert.match(cockpit.renderExpressiveComponentHudSurface({ engine_summaries: { present: false, empty_state: '<script>x</script>' } }), /&lt;script&gt;x&lt;\/script&gt;/);
const xssExpressive = cockpit.renderExpressiveComponentHudSurface({ engine_summaries: { present: true, components: { present: true, entityCount: 1, componentCounts: { '<script>': 1 }, entities: [{ entityId: '<img>', components: ['<svg>'] }] }, triggers: { present: true, triggerCount: 1, triggerCollisionEventCount: 0, triggers: [{ id: '<script>', entityId: '<img>', kind: '<b>', targetFlag: '<svg>', requiredFlags: ['<i>'], onEnterCount: 1 }] }, hud: { present: true, hudValueEntityCount: 1, values: [{ label: '<script>', text: '<img>', bindFlag: '<svg>', flagValue: '<b>' }] } } });
assert.doesNotMatch(xssExpressive, /<script>|<img>|<svg>|<b>/);
assert.match(xssExpressive, /&lt;script&gt;/);
assert.match(cockpit.renderStudioNavigation(run), /Collision\/transition\/event inspection/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Collision rules/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Declared scene transitions/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /to_boss/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /runtime\.scene\.transition/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /runtime\.audio\.play/);
assert.match(cockpit.renderRuntimeEventInspectionSurface({ engine_summaries: { present: true, collision: '<bad>', transition: null, events: [] } }), /collision summary missing or malformed/);
assert.match(cockpit.renderRuntimeEventInspectionSurface({ engine_summaries: { present: false, empty_state: '<script>events</script>' } }), /&lt;script&gt;events&lt;\/script&gt;/);
const xssRuntimeEvents = cockpit.renderRuntimeEventInspectionSurface({ engine_summaries: { present: true, collision: { present: true, rules: { '<script>': { bad: '<img>' } }, colliderEntityCount: 1, collisionEventCount: 1, events: [{ type: '<script>', payload: '<img>' }] }, transition: { present: true, currentSceneId: '<svg>', declaredTransitionCount: 1, declaredTransitions: [{ id: '<b>', toScene: '<img>' }], transitionEventCount: 1, transitions: [{ type: '<b>', to: '<img>' }], lastReloadStatus: '<i>' }, events: { present: true, animationEntityCount: 1, audioEventCount: 1, collisionEventCount: 1, animationEntities: [{ entityId: '<img>', mode: '<svg>', currentClip: '<script>', frameIndex: 1 }], audioEvents: [{ type: '<b>', clipId: '<i>' }] } } });
assert.doesNotMatch(xssRuntimeEvents, /<script>|<img>|<svg>|<b>|<i>/);
assert.match(xssRuntimeEvents, /&lt;script&gt;/);
assert.match(cockpit.renderEngineExpansionSurface(run), /Engine Expansion state/);
assert.match(cockpit.renderEngineExpansionSurface(run), /trigger-flags-v1-fixture/);
assert.match(cockpit.renderEngineExpansionSurface(run), /Gameplay\/HUD/);
assert.match(cockpit.renderEngineExpansionSurface(run), /3 flag\(s\), 2 true, 1 trigger event\(s\), 2 HUD value\(s\)/);
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
assert.match(cockpit.renderIntegration(run), /Regression promotions/);
assert.match(cockpit.renderIntegration(run), /Replay controls/);
assert.match(cockpit.renderIntegration(run), /Run comparison/);
assert.match(cockpit.renderIntegration(run), /Engine Expansion state/);
assert.match(cockpit.renderIntegration(run), /Expressive scene inspection/);
assert.match(cockpit.renderIntegration(run), /Collision\/transition\/event inspection/);
assert.match(cockpit.renderIntegration(run), /Component counts/);
assert.match(cockpit.renderIntegration(run), /Collision rules/);
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
const xssFidelity = { summary: { id: 'x' }, evidence_fidelity: { transaction: { label: '<script>tx</script>', status: '<script>bad</script>', summary: '<script>summary</script>', warnings: ['<script>warn</script>'], evidence_refs: ['<script>ref</script>'] } } };
assert.ok(!cockpit.renderEvidenceFidelitySurface(xssFidelity).includes('<script>warn</script>'), 'fidelity warnings must be escaped');
const xssAssetLoading = { asset_loading: { present: true, boundary: '<script>boundary</script>', records: [{ assetId: '<img src=x onerror=alert(1)>', attemptId: '<script>attempt</script>', path: '<b>bad</b>', status: '<script>failed</script>', failureReason: '<script>reason</script>' }] } };
assert.ok(!cockpit.renderRuntimeAssetLoadingSurface(xssAssetLoading).includes('<script>reason</script>'), 'asset loading rows must be escaped');
assert.ok(!cockpit.renderRuntimeAssetLoadingSurface(xssAssetLoading).includes('<script>boundary</script>'), 'asset loading boundary must be escaped');
const xssAssetPreview = { asset_preview: { present: true, boundary: '<script>preview-boundary</script>', records: [{ assetId: '<img src=x onerror=alert(1)>', assetType: '<script>type</script>', sourcePath: '<b>bad</b>', previewKind: '<script>kind</script>' }], warnings: [{ assetId: '<img>', kind: '<script>warning</script>', message: '<script>preview reason</script>' }] } };
assert.ok(!cockpit.renderAssetPreviewEvidenceSurface(xssAssetPreview).includes('<script>preview reason</script>'), 'asset preview rows must be escaped');
assert.ok(!cockpit.renderAssetPreviewEvidenceSurface(xssAssetPreview).includes('<script>preview-boundary</script>'), 'asset preview boundary must be escaped');
const xssVisualDiff = { visual_diff_preview: { present: true, boundary: '<script>diff-boundary</script>', summaries: [{ summaryId: '<img src=x onerror=alert(1)>', target: { type: '<script>target</script>', path: '<b>path</b>' }, sourceRefs: { draftId: '<script>draft</script>' }, before: { summaryText: '<script>before</script>', collisionTriggerSummary: {} }, after: { summaryText: '<script>after</script>', collisionTriggerSummary: {} }, operationSummaries: [{ operationId: '<script>op</script>', change: '<script>change</script>', path: '<b>path</b>', summary: '<script>summary</script>', affectedEntityIds: ['<script>entity</script>'], collisionTriggerSummary: {} }], expectedScenarioImpact: { status: '<script>impact</script>', summary: '<script>impact summary</script>' } }] } };
assert.ok(!cockpit.renderVisualDiffPreviewSurface(xssVisualDiff).includes('<script>summary</script>'), 'visual diff operation summary must be escaped');
assert.ok(!cockpit.renderVisualDiffPreviewSurface(xssVisualDiff).includes('<script>diff-boundary</script>'), 'visual diff boundary must be escaped');
assert.ok(!cockpit.renderVisualDiffPreviewSurface(xssVisualDiff).includes('<img src=x onerror=alert(1)>'), 'visual diff summary id must be escaped');
const xssTilemapDraft = { tilemap_draft_preview: { present: true, boundary: '<script>tilemap-boundary</script>', records: [{ operationId: '<img src=x onerror=alert(1)>', kind: '<script>kind</script>', layerId: '<b>layer</b>', summary: '<script>summary</script>', beforeTilemapHash: { algorithm: '<script>before</script>', value: '<b>hash</b>' }, afterTilemapHash: { algorithm: '<script>after</script>', value: '<b>hash</b>' }, collisionCells: ['<script>cell</script>'], triggerCells: [] }] } };
assert.ok(!cockpit.renderTilemapDraftPreviewSurface(xssTilemapDraft).includes('<script>summary</script>'), 'tilemap draft preview summary must be escaped');
assert.ok(!cockpit.renderTilemapDraftPreviewSurface(xssTilemapDraft).includes('<script>tilemap-boundary</script>'), 'tilemap draft preview boundary must be escaped');
assert.ok(!cockpit.renderTilemapDraftPreviewSurface(xssTilemapDraft).includes('<img src=x onerror=alert(1)>'), 'tilemap draft preview operation id must be escaped');
const xssAssetInspector = { asset_inspector: { present: true, status: '<script>status</script>', boundary: '<script>inspector-boundary</script>', evidence_refs: ['<script>ref</script>'], assets: [{ assetId: '<img src=x onerror=alert(1)>', assetType: '<script>type</script>', sourcePath: '<b>bad</b>', contentHash: '<script>hash</script>', runtimeStatuses: ['<script>runtime</script>'], tilemap: { tilesetAssetId: '<script>tileset</script>', width: '<b>', height: '<i>' }, warnings: ['<script>warning</script>'] }] }, asset_preview: { present: true, records: [{ assetId: '<script>atlas</script>', atlasFrames: [{ frameId: '<script>frame</script>', rect: { x: '<b>', y: '<i>', width: '<svg>', height: '<img>' } }], tilemap: { tilesetAssetId: '<script>tileset</script>', width: '<b>', height: '<i>', layerCount: '<svg>', tileCount: '<img>' } }] }, asset_loading: { present: true, records: [{ assetId: '<script>load</script>', attemptId: '<script>attempt</script>', path: '<b>path</b>', status: '<script>failed</script>', failureReason: '<script>reason</script>' }] } };
const xssAssetInspectorMarkup = cockpit.renderStudioAssetInspectorSurface(xssAssetInspector);
assert.ok(!xssAssetInspectorMarkup.includes('<script>inspector-boundary</script>'), 'asset inspector boundary must be escaped');
assert.ok(!xssAssetInspectorMarkup.includes('<script>warning</script>'), 'asset inspector warnings must be escaped');
assert.ok(!xssAssetInspectorMarkup.includes('<img src=x onerror=alert(1)>'), 'asset inspector asset id must be escaped');
assert.ok(!xssAssetInspectorMarkup.includes('<script>runtime</script>'), 'asset inspector runtime statuses must be escaped');
assert.ok(!xssAssetInspectorMarkup.includes('<script>reason</script>'), 'asset inspector runtime evidence must be escaped');
assert.ok(!xssAssetInspectorMarkup.includes('<script>frame</script>'), 'asset inspector atlas frames must be escaped');
assert.ok(!xssAssetInspectorMarkup.includes('<script>tileset</script>'), 'asset inspector tilemaps must be escaped');
assert.match(xssAssetInspectorMarkup, /&lt;script&gt;status&lt;\/script&gt;/);
const cockpitSource = fs.readFileSync(require.resolve('./cockpit.js'), 'utf8');
assert.ok(!/writeFile|localStorage|indexedDB|showSaveFilePicker|exec\(|spawn\(|child_process/.test(cockpitSource), 'cockpit browser code must not include direct persistence or command execution APIs');
console.log('authoring cockpit smoke test passed');

assert.match(cockpit.renderMutationReviewSurface(run), /Review decisions/);
assert.match(cockpit.renderMutationReviewSurface(run), /review-decision-1/);
assert.match(cockpit.renderMutationReviewSurface(run), /manual-reviewer/);
assert.ok(!cockpit.renderMutationReviewSurface(run).includes('<b>accepted</b>'));
assert.match(cockpit.renderReviewDecisionSurface({ stages: [] }, run), /No review decisions recorded/);

assert.match(cockpit.renderLoopDryRunSurface(run), /Authoring loop dry-run/);
assert.match(cockpit.renderLoopDryRunSurface(run), /blocked/);
assert.match(cockpit.renderLoopDryRunSurface(run), /&lt;loop-1&gt;/);
assert.match(cockpit.renderLoopDryRunSurface(run), /Expected artifacts: decision:runs\/review-decision\.json/);
const escapedLoopDryRun = cockpit.renderLoopDryRunSurface({ loop_dry_run: { steps: [{ id: 's', expectedArtifacts: [{ id: '<artifact>', path: '<script>alert(1)</script>' }] }] } });
assert.match(escapedLoopDryRun, /&lt;artifact&gt;:&lt;script&gt;alert\(1\)&lt;\/script&gt;/);
assert.doesNotMatch(escapedLoopDryRun, /<script>alert/);
assert.doesNotMatch(cockpit.renderLoopDryRunSurface(run), /<loop-1>/);
assert.match(cockpit.renderEvidencePane(run), /Authoring loop dry-run/);
assert.match(cockpit.renderLoopDryRunSurface({ summary: { id: 'run-no-loop' } }), /No dry-run summary/);
assert.match(cockpit.renderLoopExecutionSurface(run), /Authoring loop execution/);
assert.match(cockpit.renderLoopExecutionSurface(run), /completed/);
assert.match(cockpit.renderLoopExecutionSurface(run), /&lt;transaction&gt;/);
assert.doesNotMatch(cockpit.renderLoopExecutionSurface(run), /<transaction>/);
const blockedLoopExecution = cockpit.renderLoopExecutionSurface({
  loop_execution: {
    loopId: '<loop-blocked>',
    stepId: '<step-blocked>',
    kind: 'apply-accepted-scene-mutation',
    status: '<blocked>',
    ledgerPath: '<ledger>',
    blockedReasons: ['<missing decision>'],
  },
});
assert.match(blockedLoopExecution, /&lt;blocked&gt;/);
assert.match(blockedLoopExecution, /Blocked by: &lt;missing decision&gt;/);
assert.doesNotMatch(blockedLoopExecution, /gap \/ unavailable/);
assert.doesNotMatch(blockedLoopExecution, /<blocked>/);
assert.match(cockpit.renderEvidencePane(run), /Authoring loop execution/);
assert.match(cockpit.renderLoopExecutionSurface({ summary: { id: 'run-no-loop' } }), /No loop execution summary/);
// A non-completed loop execution status (e.g. blocked) must surface its actual label, not "gap / unavailable".
const blockedExec = cockpit.renderLoopExecutionSurface({ loop_execution: { loopId: 'l', stepId: 's', status: 'blocked', kind: 'compare-runs' } });
assert.doesNotMatch(blockedExec, /gap \/ unavailable/);
assert.match(blockedExec, /status-ok">blocked</);
assert.match(cockpit.renderLoopRecoverySurface(run), /Authoring loop recovery/);
assert.match(cockpit.renderLoopRecoverySurface(run), /needs-recovery/);
assert.match(cockpit.renderLoopRecoverySurface(run), /&lt;missing comparison&gt;/);
assert.doesNotMatch(cockpit.renderLoopRecoverySurface(run), /<missing comparison>/);
assert.match(cockpit.renderEvidencePane(run), /Authoring loop recovery/);
assert.match(cockpit.renderLoopRecoverySurface({ summary: { id: 'run-no-loop' } }), /No recovery status/);

assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Loop cockpit/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /&lt;cockpit-loop&gt;/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /&lt;step-current&gt;/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Blockers: missing &lt;comparison&gt;/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Required decisions: &lt;human-review&gt;:human-review/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Next safe action: Copy &lt;status&gt; command manually/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Allowed command text: cargo run -p ouroforge-cli -- loop status &lt;plan&gt;/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Forbidden actions: Do not click &lt;run&gt; controls/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Evidence refs: &lt;bundle&gt;:runs\/&lt;bundle&gt;\/bundle\.json/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Missing\/stale bundle refs: comparison:&lt;missing&gt;/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /No browser authority|does not execute commands/);
assert.doesNotMatch(cockpit.renderStudioLoopCockpitSurface(run), /<cockpit-loop>/);
assert.doesNotMatch(cockpit.renderStudioLoopCockpitSurface(run), /<button/i);
assert.match(cockpit.renderStudioLoopCockpitSurface({ loop_cockpit: { schemaVersion: '<schema>', loops: '<bad>' } }), /Malformed loop cockpit read-model/);
assert.match(cockpit.renderStudioLoopCockpitSurface({}), /No loop cockpit read-model/);
assert.match(cockpit.renderRuntimeAssetLoadingSurface(run), /Runtime asset loading/);
assert.match(cockpit.renderRuntimeAssetLoadingSurface(run), /player_sprite/);
assert.match(cockpit.renderRuntimeAssetLoadingSurface(run), /Image load failed/);
assert.match(cockpit.renderRuntimeAssetLoadingSurface({}), /No runtime asset loading evidence/);
assert.match(cockpit.renderAssetPreviewEvidenceSurface(run), /Asset preview evidence/);
assert.match(cockpit.renderAssetPreviewEvidenceSurface(run), /player_atlas/);
assert.match(cockpit.renderAssetPreviewEvidenceSurface(run), /missing_asset_file/);
assert.match(cockpit.renderAssetPreviewEvidenceSurface({}), /No asset preview evidence/);
const visualDiffPreviewMarkup = cockpit.renderVisualDiffPreviewSurface(run);
assert.match(visualDiffPreviewMarkup, /Visual diff preview/);
assert.match(visualDiffPreviewMarkup, /visual-diff-scene-draft/);
assert.match(visualDiffPreviewMarkup, /op-player-x/);
assert.match(visualDiffPreviewMarkup, /draft draft-scene-1/);
assert.match(visualDiffPreviewMarkup, /dashboard dashboard-data\.json#run-1/);
assert.match(visualDiffPreviewMarkup, /Scenario impact: unknown/);
assert.match(visualDiffPreviewMarkup, /collision 0 \/ trigger 1|0 \/ 1/);
assert.match(visualDiffPreviewMarkup, /browser cannot apply edits, write files, execute commands, or persist drafts/);
assert.doesNotMatch(visualDiffPreviewMarkup, /<button/i);
assert.match(cockpit.renderVisualDiffPreviewSurface({}), /No visual diff summary read model/);
const tilemapDraftPreviewMarkup = cockpit.renderTilemapDraftPreviewSurface(run);
assert.match(tilemapDraftPreviewMarkup, /Tilemap draft previews/);
assert.match(tilemapDraftPreviewMarkup, /op-collision-preview/);
assert.match(tilemapDraftPreviewMarkup, /collision_preview/);
assert.match(tilemapDraftPreviewMarkup, /op-trigger-preview/);
assert.match(tilemapDraftPreviewMarkup, /coin_trigger|trigger 1/);
assert.match(tilemapDraftPreviewMarkup, /browser cannot write tilemaps, execute commands, or apply reviews/);
assert.doesNotMatch(tilemapDraftPreviewMarkup, /<button/i);
assert.match(cockpit.renderTilemapDraftPreviewSurface({}), /No tilemap draft preview read model/);
const assetInspectorMarkup = cockpit.renderStudioAssetInspectorSurface(run);
assert.match(assetInspectorMarkup, /Asset inspector/);
assert.match(assetInspectorMarkup, /player_sprite/);
assert.match(assetInspectorMarkup, /player_atlas/);
assert.match(assetInspectorMarkup, /level_tilemap/);
assert.match(assetInspectorMarkup, /missing_audio/);
assert.match(assetInspectorMarkup, /warning/);
assert.match(assetInspectorMarkup, /missing_asset_file/);
assert.match(assetInspectorMarkup, /Atlas frame evidence/);
assert.match(assetInspectorMarkup, /idle_0/);
assert.match(assetInspectorMarkup, /rect 0,0 16×16/);
assert.match(assetInspectorMarkup, /Tilemap evidence/);
assert.match(assetInspectorMarkup, /terrain_tiles/);
assert.match(assetInspectorMarkup, /2 layer\(s\)/);
assert.match(assetInspectorMarkup, /Runtime load evidence/);
assert.match(assetInspectorMarkup, /load-player-sprite/);
assert.match(assetInspectorMarkup, /Image load failed/);
assert.match(assetInspectorMarkup, /4/);
assert.match(assetInspectorMarkup, /asset-integrity\.json/);
assert.match(assetInspectorMarkup, /no upload, write, fetch, or execute controls/);
assert.match(assetInspectorMarkup, /browser cannot upload assets, write manifests, fetch remote assets, or execute commands/);
assert.match(cockpit.renderStudioAssetInspectorSurface({}), /No asset inspector data/);
assert.match(cockpit.renderEvidencePane(run), /Runtime asset loading/);
assert.match(cockpit.renderEvidencePane(run), /Asset preview evidence/);
assert.match(cockpit.renderEvidencePane(run), /Visual diff preview/);
assert.match(cockpit.renderEvidencePane(run), /Tilemap draft previews/);
assert.match(cockpit.renderEvidencePane(run), /Asset inspector/);
assert.match(cockpit.renderEvidencePane(run), /Loop cockpit/);

assert.match(cockpit.renderAgentHandoffSurface(run), /Agent handoff/);
assert.match(cockpit.renderAgentHandoffSurface(run), /blocked/);
assert.match(cockpit.renderAgentHandoffSurface(run), /&lt;handoff-loop&gt;/);
assert.match(cockpit.renderAgentHandoffSurface(run), /Allowed command text/);
assert.doesNotMatch(cockpit.renderAgentHandoffSurface(run), /<handoff-loop>/);
assert.doesNotMatch(cockpit.renderAgentHandoffSurface(run), /<button/i);
assert.match(cockpit.renderEvidencePane(run), /Agent handoff/);
assert.match(cockpit.renderAgentHandoffSurface({}), /No agent handoff/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /Authoring loop evidence bundle/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /partial/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /&lt;loop-bundle&gt;/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /runs:/);
assert.doesNotMatch(cockpit.renderLoopEvidenceBundleSurface(run), /<loop-bundle>/);
assert.match(cockpit.renderEvidencePane(run), /Authoring loop evidence bundle/);
assert.match(cockpit.renderLoopEvidenceBundleSurface({ summary: { id: 'run-no-bundle' } }), /No loop evidence bundle/);

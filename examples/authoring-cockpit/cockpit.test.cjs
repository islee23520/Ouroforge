const assert = require('node:assert/strict');
const fs = require('node:fs');
const cockpit = require('./cockpit.js');
const scene = require('../game-runtime/scene.json');
const behaviorDraftDocs = fs.readFileSync('docs/behavior-draft-v1.md', 'utf8');
const behaviorDraftFixtureDocs = fs.readFileSync('examples/behavior-draft-v1/README.md', 'utf8');
const godotPlusDesignPillarsDoc = fs.readFileSync('docs/godot-plus-demo-design-pillars-v1.md', 'utf8');
const godotPlusDocsReadme = fs.readFileSync('docs/README.md', 'utf8');

assert.match(behaviorDraftDocs, /untrusted data/i);
assert.match(behaviorDraftDocs, /does not apply trusted files/i);
assert.match(behaviorDraftDocs, /No arbitrary script execution/);
assert.match(behaviorDraftDocs, /generated behavior drafts remain untracked/i);
assert.match(behaviorDraftFixtureDocs, /fixture-scoped/i);
assert.match(behaviorDraftFixtureDocs, /read-only validate\/preview/i);

assert.match(godotPlusDesignPillarsDoc, /Issue: #779/);
assert.match(godotPlusDesignPillarsDoc, /single-screen top-down action-puzzle escape/);
assert.match(godotPlusDesignPillarsDoc, /Signal Gate/);
assert.match(godotPlusDesignPillarsDoc, /collect the signal key, open the gate, and exit/);
assert.match(godotPlusDesignPillarsDoc, /Evidence-native agentic loop/);
assert.match(godotPlusDesignPillarsDoc, /review-gated Safe Source Apply/);
assert.match(godotPlusDesignPillarsDoc, /Status: \*\*GPD12\.2\.3 capability-mapping contract\*\*/);
assert.match(godotPlusDesignPillarsDoc, /Capability moment/);
assert.match(godotPlusDesignPillarsDoc, /Agent creates a bounded draft/);
assert.match(godotPlusDesignPillarsDoc, /QA\/playtest finds a regression/);
assert.match(godotPlusDesignPillarsDoc, /Review gates source mutation/);
assert.match(godotPlusDesignPillarsDoc, /Scoped comparison remains honest/);
assert.match(godotPlusDesignPillarsDoc, /Signal Gate demonstrates a scoped evidence-native agentic workflow/);
assert.match(godotPlusDesignPillarsDoc, /No full Godot parity, replacement, production readiness, secure sandbox, or broad superiority claim/);
assert.match(godotPlusDesignPillarsDoc, /## Playable success criteria/);
assert.match(godotPlusDesignPillarsDoc, /Deterministic win path/);
assert.match(godotPlusDesignPillarsDoc, /Deterministic failure path/);
assert.match(godotPlusDesignPillarsDoc, /Review-gated iteration proof/);
assert.match(godotPlusDesignPillarsDoc, /No self-approval, auto-apply, auto-merge, or hidden trusted writes/);
assert.match(godotPlusDesignPillarsDoc, /No broad Godot parity, production readiness, secure sandbox, or commercial release claim/);
assert.match(godotPlusDesignPillarsDoc, /does \*\*not\*\* authorize/);
assert.match(godotPlusDesignPillarsDoc, /No executable plugin runtime, marketplace, install\/update, dynamic loading, remote asset loading, or arbitrary JavaScript/);
assert.doesNotMatch(godotPlusDesignPillarsDoc, /full Godot replacement is implemented|production-ready engine is available|commercial release readiness is achieved|browser command bridge enabled|auto-apply enabled|native export ready/);
assert.match(godotPlusDocsReadme, /godot-plus-demo-design-pillars-v1\.md/);

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

  qa_swarm_inspection: {
    present: true,
    status: '<needs-review>',
    panel_count: 3,
    missing_panel_count: 0,
    malformed_panel_count: 1,
    item_count: 4,
    evidence_refs: ['evidence/qa/<candidate>.json'],
    boundary: 'Read-only QA/playtest swarm inspection; Studio must not spawn <workers>, execute commands, bridge to cloud runners, write trusted state, auto-fix, auto-apply, auto-merge, self-approve, or claim quality guarantees.',
    panels: [{
      panel_id: 'scenario-candidates',
      title: 'Scenario <candidate> panel',
      status: 'proposed',
      item_count: 1,
      malformed_count: 0,
      evidence_refs: ['evidence/qa/<candidate>.json'],
      boundary: 'Read-only scenario candidates; Studio must not run candidates.',
    }, {
      panel_id: 'worker-assignments',
      title: 'Worker budget/assignment panel',
      status: 'blocked',
      item_count: 2,
      malformed_count: 0,
      evidence_refs: ['evidence/qa/worker-assignment.json'],
      boundary: 'Read-only worker assignments; Studio must not spawn workers.',
    }, {
      panel_id: 'visual-performance-error-evidence',
      title: 'Visual/performance/error evidence panel',
      status: 'malformed',
      item_count: 1,
      malformed_count: 1,
      evidence_refs: ['evidence/qa/<bad>.json'],
      boundary: 'Read-only visual evidence; Studio must not compute trusted diffs or run probes.',
    }],
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

  qa_agent_work_queues: {
    present: true,
    status: 'blocked',
    boundary: 'Read-only QA queue model; Studio does not execute commands.',
    queues: [{
      schemaVersion: 'qa-agent-work-queue-v1',
      queueId: '<qa-queue>',
      milestone: 'multi-agent-production-pipeline-v1',
      items: [{
        queueItemId: '<qa-item>',
        scenarioTarget: { scenarioId: '<scenario>', scenarioPackRef: { id: 'scenario-pack', path: 'examples/scenario-packs/project-smoke.scenarios.json' } },
        riskArea: { riskId: '<risk>', category: 'scenario-regression', summary: 'Escaped <risk> summary.' },
        runCommandContext: { command: 'cargo run -p ouroforge-cli -- run <seed> --scenario-pack smoke', boundary: 'Inert command text only; does not execute.' },
        expectedEvidence: [{ id: 'scenario-result', path: 'runs/multi-agent-pipeline/demo/qa/scenario-result.json' }, { id: 'evaluator-summary', path: 'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json' }],
        priority: 'high',
        assignedRole: 'qa-agent',
        assignedAgent: '<agent>',
        status: 'blocked',
        taskRef: { id: 'task-board', path: 'examples/multi-agent-pipeline-v1/production-task-board.fixture.json' },
        workPackageRef: { id: 'work-package', path: 'examples/multi-agent-pipeline-v1/agent-work-package.valid.fixture.json' },
        reviewGateRef: { id: 'review-gate', path: 'examples/multi-agent-pipeline-v1/review-critic-gate.valid.fixture.json' },
        runEvidenceRefs: [{ id: 'run', path: 'runs/multi-agent-pipeline/demo/qa/scenario-result.json' }],
        evaluatorEvidenceRefs: [{ id: 'evaluator', path: 'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json' }],
        blockedReasons: ['review <gate> missing'],
        staleRunRefs: ['runs/multi-agent-pipeline/demo/qa/old-scenario-result.json'],
      }],
      forbiddenActions: ['browser command bridge', 'auto-merge'],
      boundary: 'QA agent work queue is inert local evidence; it does not execute commands.',
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
  mutation_artifacts: [{
    id: 'source-patch-evidence-bundle',
    kind: 'application/json',
    path: 'mutation/source-patch-evidence-bundle.json',
    metadata: { read_only: true },
    value: {
      bundleId: 'bundle-1',
      patchPreviewId: 'preview-1',
      status: 'complete',
      patchSummary: { title: 'Review docs patch preview', expectedBehaviorChange: 'Docs preview only; no trusted worktree apply.', targetCount: 2, changedLines: 18 },
      fileClassSummary: { allowed: 1, reviewHeld: 1, blocked: 0, highestRisk: 'review_held' },
      riskIds: ['source_patch_preview', 'review_held_target'],
      linkedEvidence: [
        { kind: 'source-patch-preview', path: 'mutation/preview.json' },
        { kind: 'sandbox-dry-run-report', path: 'sandbox/preview-1/evidence/report.json' },
        { kind: 'review-decision', path: 'mutation/review-decisions.json' },
      ],
      dryRunSummary: { status: 'passed', allowlistPolicyId: 'source-patch-preview-safe-local-checks-v1', reportRef: { kind: 'sandbox-dry-run-report', path: 'sandbox/preview-1/evidence/report.json' } },
      requiredTestSummary: { total: 2, commands: ['cargo fmt --check', 'cargo test -p ouroforge-core'], allowlistPolicyId: 'source-patch-preview-safe-local-checks-v1' },
      reviewSummary: { status: 'reviewed', decisionRef: { kind: 'review-decision', path: 'mutation/review-decisions.json' } },
      blockedReasons: ['manual review required before any future apply design'],
      previewRef: { kind: 'source-patch-preview', path: 'mutation/preview.json' },
      fileClassReportRef: { kind: 'file-class-report', path: 'evidence/file-class.json' },
      diffIntegrityReportRef: { kind: 'diff-integrity-report', path: 'evidence/diff.json' },
      sandboxReportRef: { kind: 'sandbox-dry-run-report', path: 'sandbox/preview-1/evidence/report.json' },
      testSummaryRef: { kind: 'test-summary', path: 'sandbox/preview-1/evidence/tests.json' },
      reviewDecisionRef: { kind: 'review-decision', path: 'mutation/review-decisions.json' },
      forbiddenActionNotices: [
        { action: 'apply_patch', reason: 'forbidden' },
        { action: 'merge_branch', reason: 'forbidden' },
        { action: 'execute_command', reason: 'forbidden' },
        { action: 'write_trusted_file', reason: 'forbidden' },
      ],
      guardrails: ['read-only bundle evidence', 'no source patch apply'],
    },
  }, {
    id: 'source-patch-apply-transaction',
    kind: 'application/json',
    path: 'mutation/source-patch-apply-transaction.json',
    metadata: { read_only: true },
    value: {
      transactionId: 'apply-tx-1',
      status: 'ready_for_trusted_apply',
      evidence: {
        patchPreviewRef: 'mutation/preview.json',
        sandboxReportRef: 'sandbox/preview-1/evidence/report.json',
        reviewDecisionRef: 'mutation/review-decision.json',
        fileClassReportRef: 'evidence/file-class.json',
        diffIntegrityReportRef: 'evidence/diff.json',
      },
      targets: [{ path: 'examples/source-patch-apply-transaction-v1/scenario-regression.json', fileClass: 'scenario_regression_fixture', reviewLevel: 'elevated_source_like_data_review' }],
      rollbackRef: { rollbackPlanRef: 'rollback/apply-tx-1.json' },
      readModel: {
        status: 'passed',
        readinessLabel: 'ready_metadata_only_no_apply_authority',
        blockedReasons: [],
        forbiddenActions: ['apply_patch', 'merge_branch', 'execute_command', 'write_trusted_file', 'browser_command_bridge'],
      },
    },
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
      openRisks: [{ id: '<risk>', severity: 'high', description: 'handoff <risk>', mitigation: 'manual review' }],
      staleStateIndicators: [{ id: '<stale>', reason: 'artifact <stale>', nextAction: 'refresh evidence' }],
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
  source_apply_worktree_context: {
    present: true,
    status: 'blocked',
    target_count: 2,
    blocked_count: 2,
    evidence_refs: ['evidence/source-apply/worktree-context.json'],
    boundary: 'Read-only context evidence; browser/dashboard/Studio surfaces must not apply patches, execute commands, write trusted files, merge branches, or bypass review gates.',
    reports: [{
      schemaVersion: 'source-apply-worktree-context-v1',
      status: 'blocked',
      policyId: 'source-apply-worktree-boundary-v1',
      branch: '<issue-701>',
      headCommit: '<head>',
      worktreeRoot: '<worktree>',
      lockStatus: { active: false, attemptId: '<attempt>' },
      blockedReasons: ['seeds/platformer.yaml: dirty-target: git status modified', '<script>blocked</script>'],
      guardrails: ['browser/dashboard/Studio surfaces remain read-only and command-inert'],
      targets: [
        { path: 'seeds/platformer.yaml', gitStatus: 'modified', rootZone: 'trusted-source', fileClassDecision: 'allowed', blockedReasons: ['dirty-target: git status modified'] },
        { path: '<script>bad</script>', gitStatus: 'untracked-collision', rootZone: 'trusted-source', fileClassDecision: 'allowed', blockedReasons: ['untracked-target-collision'] },
      ],
    }],
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
  studio_draft_authoring: {
    present: true,
    boundary: 'Temporary in-memory draft state only; browser cannot write files or execute commands.',
    drafts: [{
      schemaVersion: 'visual-edit-draft-v1',
      draftId: 'studio-draft-scene-1',
      target: { type: 'scene', path: 'scenes/main.scene.json', id: 'main' },
      proposedOperations: [{ id: 'op-move-player', kind: 'update', path: 'entities.player.components.transform.x', summary: 'Move player preview only', value: 48 }],
      beforeHash: 'fnv1a64-canonical-json-v1:beforehash',
      expectedAfterSummary: 'Player moves right after trusted CLI preview.',
      linkedEvidence: ['evidence/world.json'],
      validationStatus: 'partial',
      blockedReasons: ['awaiting Rust preview'],
      author: { type: 'studio', id: 'browser-memory', source: 'in-memory' },
    }],
  },
  behavior_drafts: {
    present: true,
    status: 'preview-only',
    boundary: 'Behavior draft read model is read-only; browser cannot apply drafts, write trusted files, execute scripts, open command bridges, or persist draft state.',
    records: [{
      draftId: 'draft-jump-boost',
      draftPath: 'examples/behavior-draft-v1/valid/behavior-draft.valid.json',
      validationStatus: 'drafted',
      target: { projectId: 'demo_project', scenePath: 'scenes/main.scene.json', sceneHash: 'fnv1a64-canonical-json-v1:expectedhash' },
      linkedEvidenceCount: 1,
      expectedScenarioImpactCount: 1,
      behaviorCount: 2,
      targetCheck: { stale: false, expectedHash: 'fnv1a64-canonical-json-v1:expectedhash', actualHash: 'fnv1a64-canonical-json-v1:expectedhash' },
      blockedReasons: [],
      diagnostics: [],
      guardrail: 'read-only untrusted preview; does not apply trusted files, execute scripts, open command bridges, or grant browser writes',
    }, {
      draftId: 'draft-stale-target',
      validationStatus: 'stale',
      target: { projectId: 'demo_project', scenePath: 'scenes/stale.scene.json', sceneHash: 'fnv1a64-canonical-json-v1:oldhash' },
      linkedEvidenceCount: 1,
      expectedScenarioImpactCount: 1,
      behaviorCount: 2,
      targetCheck: { stale: true, expectedHash: 'fnv1a64-canonical-json-v1:oldhash', actualHash: 'fnv1a64-canonical-json-v1:newhash' },
      blockedReasons: ['stale target hash: refresh before review'],
      diagnostics: [{ kind: 'stale-target', message: 'target hash differs' }],
      guardrail: 'stale behavior draft status only; no apply',
    }],
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
  plugin_registry: {
    present: true,
    status: 'blocked',
    registry_count: 1,
    plugin_count: 3,
    blocked_count: 1,
    malformed_count: 0,
    evidence_refs: ['evidence/plugins/plugin-registry-fixture.json'],
    boundary: 'Read-only plugin registry browser data; dashboard/Studio surfaces must not install, update, delete, enable executable plugins, run commands, mutate source, publish, deploy, or write trusted files.',
    registries: [{
      registryId: 'plugin-registry-fixture',
      projectId: 'demo_project',
      runId: 'run-plugin-registry-fixture',
      ledgerRef: 'runs/plugin-registry-fixture/ledger.jsonl',
      status: 'blocked',
      pluginCount: 3,
      blockedCount: 1,
      blockedReasons: ['blocked-command-panel:manifest requested executable command authority outside the v1 declarative catalog'],
      plugins: [
        { pluginId: 'read-only-dashboard-panel', manifestPath: 'plugins/read-only-dashboard-panel/plugin.json', manifestHash: 'fnv1a64-canonical-json-v1:1111222233334444', manifestVersion: '0.1.0', validationStatus: 'valid', compatibilityStatus: 'compatible', declaredCapabilities: ['dashboardPanel'], extensionPoints: ['dashboard.panels.readOnly'], evidenceRefs: ['runs/plugin-registry-fixture/plugin-evidence/read-only-dashboard-panel.validation.json'], dashboardPanels: [{ panelId: 'plugin-registry-summary', title: 'Plugin registry summary', dataSourceKey: 'pluginRegistry.summary', templateRef: 'pluginRegistrySummaryCard', layoutHint: 'summary', displayHints: ['compact', 'blocked-count'], boundary: 'Declarative allowlisted read-only dashboard panel descriptor; no JavaScript execution, no command hooks, no trusted writes.' }], blockedReasons: [] },
        { pluginId: 'read-only-scenario-template', manifestPath: 'plugins/read-only-scenario-template/plugin.json', manifestHash: 'fnv1a64-canonical-json-v1:2222333344445555', manifestVersion: '0.1.0', validationStatus: 'valid', compatibilityStatus: 'compatible', declaredCapabilities: ['scenarioTemplate'], extensionPoints: ['scenario.templates.readOnly'], evidenceRefs: ['runs/plugin-registry-fixture/plugin-evidence/read-only-scenario-template.validation.json'], scenarioTemplates: [{ templateId: 'collect-goal-smoke', description: 'Declarative QA smoke template for collecting a goal and exiting without plugin-owned execution.', parameters: [{ name: 'goalId', parameterType: 'string', description: 'Local scenario goal identifier to collect.', required: true }, { name: 'difficulty', parameterType: 'enum', description: 'Fixture difficulty label for expected pacing.', required: false, allowedValues: ['easy', 'normal', 'hard'] }], supportedGameTypes: ['platformer', 'prototype'], tags: ['qa-smoke', 'gdd-prototype'], expectedEvidenceType: 'scenarioPack', validationHints: ['Require existing trusted scenario runner to own execution.', 'Record input replay and runtime probe evidence refs after trusted QA execution.'], boundary: 'Declarative read-only scenario template metadata only; no executable scripts, no command hooks, no network references, no source mutation hooks, no trusted writes.' }], blockedReasons: [] },
        { pluginId: 'blocked-command-panel', manifestPath: 'plugins/blocked-command-panel/plugin.json', manifestHash: 'fnv1a64-canonical-json-v1:aaaabbbbccccdddd', manifestVersion: '0.1.0', validationStatus: 'blocked', compatibilityStatus: 'incompatible', declaredCapabilities: ['studioInspectorPanel'], extensionPoints: ['studio.inspector.readOnly'], evidenceRefs: ['runs/plugin-registry-fixture/plugin-evidence/blocked-command-panel.validation.json'], blockedReasons: ['manifest requested executable command authority outside the v1 declarative catalog'] },
      ],
    }],
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
    renderer: {
      version: '1',
      renderedEntities: 3,
      camera: { x: 80, y: 30 },
      viewport: { width: 160, height: 90 },
      layers: [
        { id: '<sky>', order: -10, visible: true, parallaxFactor: 50, cameraParticipation: true },
        { id: '<hud>', order: 10, visible: true, parallaxFactor: 100, cameraParticipation: false },
      ],
    },
    camera: {
      activeCameraId: '<follow-player>',
      rendererCamera: { x: 80, y: 30 },
      viewport: { width: 160, height: 90 },
      cameras: [{
        id: '<follow-player>',
        active: true,
        followTarget: '<player>',
        position: { x: 80, y: 30 },
        viewport: { width: 160, height: 90 },
        clampBounds: { x: 0, y: 0, width: 240, height: 120 },
        zoom: 100,
      }],
      worldToScreen: {
        '<player>': { x: 140, y: 66, layer: 'actors', cameraOffset: { x: 80, y: 30 } },
        '<hud>': { x: 12, y: 8, layer: '<hud>', cameraOffset: { x: 0, y: 0 } },
      },
    },
    render_breakdown: {
      present: true,
      frameId: 12,
      sceneId: 'trigger-flags-v1-fixture',
      elementCount: 1,
      absenceDiagnosticCount: 1,
      elements: [{
        renderableId: 'entity:player',
        entityId: 'player',
        frameId: 12,
        drawOrder: 4,
        layer: 'actors',
        primitiveCategory: 'sprite',
        debugLabel: 'player sprite',
      }],
      absenceDiagnostics: [{
        entityId: 'hidden',
        renderableId: 'entity:hidden',
        reason: 'sprite.visible=false',
        layer: 'actors',
        detail: 'Entity is intentionally hidden by scene data.',
      }],
      readOnlyInspection: {
        disallowedActions: ['writes', 'commands', 'scene mutation', 'browser runtime control'],
      },
      boundary: 'Render breakdown inspection is read-only evidence from runtime world state.',
    },
    render_queue: {
      present: true,
      frameId: 12,
      sceneId: 'trigger-flags-v1-fixture',
      layerCount: 1,
      renderableCount: 2,
      drawCallCount: 1,
      skippedRenderableCount: 1,
      validation: { status: 'ready', blockedReasons: [], warnings: [] },
      tilemapStats: { layerCount: 1, cellCount: 2, drawnTileCount: 2, missingTileRefCount: 1, assetTileCount: 1 },
      renderables: [
        { id: 'entity-player', sourceKind: 'entity', sourceId: 'player', drawOrder: 4, layer: 'actors', primitiveKind: 'sprite', visible: true },
        { id: 'tilemap-ground', sourceKind: 'tilemap-layer', sourceId: 'level:ground', drawOrder: 5, layer: 'ground', primitiveKind: 'tilemap', visible: true, tileCount: 2, missingTileRefCount: 1, assetTileCount: 1 },
        { id: 'entity-hidden', sourceKind: 'entity', sourceId: 'hidden', drawOrder: 5, layer: 'actors', primitiveKind: 'rect', visible: false, fallbackReason: 'sprite hidden' },
      ],
      readOnlyInspection: {
        disallowedActions: ['writes', 'commands', 'scene mutation', 'browser runtime control'],
      },
    },
    scene3d_hierarchy: {
      present: true,
      sceneId: 'trigger-flags-v1-fixture',
      nodeCount: 2,
      rootCount: 1,
      parentedNodeCount: 1,
      transforms: [
        { nodeId: '<root-node>', parentId: null, worldTransform: { translation: { x: 0, y: 0, z: 0 } } },
        { nodeId: '<cube-node>', parentId: '<root-node>', worldTransform: { translation: { x: 1, y: 2, z: 3 } } },
      ],
      boundary: 'Read-only bounded 3D hierarchy evidence; no 3D editor or viewport persistence claim.',
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'viewport persistence', 'scene mutation'] },
    },
    scene3d_camera: {
      present: true,
      activeCameraId: '<main-camera>',
      cameraCount: 1,
      cameras: [{
        id: '<main-camera>',
        active: true,
        projection: { kind: '<perspective>', fovDegrees: 60, near: 1, far: 1000 },
        viewport: { x: 0, y: 0, width: 320, height: 180 },
      }],
      boundary: 'Read-only 3D camera evidence; no viewport persistence or camera editor tooling.',
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'viewport persistence', 'scene mutation'] },
    },
    scene3d_probe: {
      present: true,
      status: '<present>',
      sceneKind: '3d',
      nodeCount: 2,
      cameraCount: 1,
      animationStateCount: 1,
      boundary: 'Read-only 3D runtime probe evidence; browser probe output is not trusted persistence.',
    },
    scene3d_render: {
      present: true,
      frameId: 12,
      sceneId: 'trigger-flags-v1-fixture',
      cameraId: '<main-camera>',
      meshCount: 1,
      materialCount: 1,
      attemptedObjectCount: 2,
      visibleObjectCount: 1,
      skippedObjectCount: 1,
      failedObjectCount: 0,
      screenshotArtifact: null,
      renderables: [
        { id: '<scene3d-cube>', nodeId: '<cube-node>', meshRef: '<cube-mesh>', materialRef: '<cube-mat>', primitive: 'cube', cameraId: '<main-camera>', visible: true },
        { id: '<scene3d-missing>', nodeId: '<missing-node>', meshRef: '<missing-mesh>', primitive: 'cube', cameraId: '<main-camera>', visible: false, fallbackReason: '<missing mesh>' },
      ],
      fallbackReasons: ['<missing-node>: <missing mesh>'],
      boundary: 'Read-only bounded 3D render smoke evidence; no WebGPU, GLTF import, PBR, remote fetch, or production renderer claim.',
    },
    scene3d_collision: {
      present: true,
      frameId: 12,
      sceneId: 'trigger-flags-v1-fixture',
      colliderCount: 3,
      activeColliderCount: 2,
      disabledColliderCount: 1,
      contactCount: 1,
      triggerCount: 1,
      invalidColliderCount: 1,
      events: [{ type: 'runtime.scene3d.collision.trigger', pairId: '<goal:player>' }],
      invalidColliders: [{ nodeId: '<broken>', colliderRef: '<missing-box>', reason: '<missing collider>' }],
      boundary: 'Read-only bounded 3D collision evidence; no full 3D physics engine, rigidbody parity, ragdoll, joints, vehicle, or character-controller maturity claim.',
    },
    scene3d_animation: {
      present: true,
      stateCount: 1,
      playingStateCount: 0,
      states: [{ clipId: '<idle-clip>', targetNodeId: '<cube-node>', channel: '<translation>', currentFrame: 3, playing: false }],
      events: [{ type: '<animation-event>', payload: { targetNodeId: '<cube-node>' } }],
      boundary: 'Read-only bounded 3D animation evidence; no skeletal authoring, IK, or graph editor claim.',
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'timeline persistence', 'scene mutation'] },
    },
    scene3d_scenario_verdicts: {
      present: true,
      verdictCount: 1,
      failedVerdictCount: 0,
      verdicts: [{ scenarioId: '<studio-3d-scenario>', status: 'passed', assertionCount: 3 }],
      boundary: 'Read-only 3D scenario verdict evidence; Studio cannot execute scenarios.',
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'scenario execution', 'promotion'] },
    },
    tilemaps: { tilemapCount: 0, layerCount: 0 },
    assets: { manifestId: null, assetCount: 0 },
    animation: { animatedEntityCount: 1, activeStateCount: 1 },
    vfx: { present: true, vfxEntityCount: 1, vfxEmitterCount: 1, vfxEventCount: 1, entities: [{ entityId: 'player', emitterCount: 1, emitters: [{ id: 'run-dust', kind: 'trail' }] }] },
    audio: { audioEntityCount: 1, audioEventCount: 1, audioWarningCount: 1, browserAudioAuthority: 'intent_evidence_only' },
    runtime_frame_budget: {
      schemaVersion: 'ouroforge.runtime-frame-budget.v1',
      frameId: '<frame-12>',
      sceneId: 'trigger-flags-v1-fixture',
      scenarioId: '<scaffold-smoke>',
      timings: { updateMs: 3, renderMs: 18.5, evidenceMs: 1, totalMs: 24.25 },
      budget: { updateMs: 8, renderMs: 16, evidenceMs: 4, totalMs: 20 },
      counts: { entityCount: 3, drawCallCount: 1, layerCount: 2, collisionPairCount: 1, activeAnimationCount: 1, activeVfxCount: 1, audioEventCount: 1 },
      status: 'violated',
      slowFrame: true,
      violations: [{ field: '<renderMs>', actualMs: 18.5, budgetMs: 16 }],
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'scene mutation', 'browser runtime control', 'remote telemetry'] },
      authority: 'browser_runtime_evidence_input_not_profiler_truth',
    },
    runtime_state: {
      present: true,
      stateId: '<tick-12-state>',
      sceneId: 'trigger-flags-v1-fixture',
      tick: 12,
      digest: { algorithm: 'fnv1a64-canonical-json-v1', value: '<digest-value>' },
      snapshotCount: 1,
      saveEventCount: 2,
      saveCreatedCount: 1,
      saveLoadedCount: 1,
      replayDigestComparedCount: 1,
      saveEvents: [
        { type: 'runtime.save.created', payload: { saveId: '<slot-1-tick-12>', slotId: '<slot-1>', stateDigest: { value: '<digest-value>' } } },
        { type: 'runtime.save.loaded', payload: { saveId: '<slot-1-tick-12>', slotId: '<slot-1>', stateDigest: { value: '<digest-value>' } } },
      ],
      replayEvents: [{ type: 'runtime.replay.digest_compared', payload: { frameId: '<frame-12>', status: 'matched', expected: { value: '<digest-value>' }, actual: { value: '<digest-value>' } } }],
      snapshots: [{ snapshotId: '<snapshot-12>', tick: 12 }],
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'save mutation', 'browser runtime control'] },
      authority: 'browser_runtime_evidence_input_not_trusted_persistence',
    },
    physics: {
      colliderEntityCount: 2,
      collisionEventCount: 1,
      grounded: { player: true },
      contacts: { player: [{ pairId: 'floor:player', otherEntityId: 'floor', normal: { x: 0, y: -1 } }] },
      contactPairs: [{ pairId: 'floor:player', normal: { x: 0, y: -1 } }],
      contactPairCount: 1,
      blockedMovement: { player: { x: false, y: true } },
    },
    input: {
      present: true,
      mappedActionCount: 3,
      activeActionCount: 2,
      activeActions: ['<move_right>', 'interact'],
      warningCount: 3,
      rawInput: { keys: { '<d>': true } },
      diagnostics: {
        missingActions: ['<dash>'],
        unmappedActions: ['interact'],
        duplicateActions: [],
        unresolvedOverrides: ['interact'],
        conflictingBindings: [{ key: '<d>', actions: ['<move_right>', '<dash>'] }],
        readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'scene mutation', 'browser runtime control'] },
      },
    },
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
      audioWarningCount: 1,
      collisionEventCount: 1,
      vfxEventCount: 1,
      animationEntities: [{ entityId: 'player', mode: 'playing', activeState: 'run', currentClip: 'run', frameIndex: 3 }],
      audioEvents: [{ kind: 'audio_request', name: 'coin', intentKind: 'sound', busId: 'sfx', volume: 80, type: 'runtime.audio.play', clipId: 'coin' }],
      audioWarnings: [{ warning: 'audible_output_not_verified', requestId: 'audio-1-1' }],
      collisionEvents: [{ type: 'runtime.collision.trigger', triggerId: 'collect_key' }],
      vfxEvents: [{ schemaVersion: 'runtime-vfx-event-v1', entityId: 'player', emitterId: 'run-dust', kind: 'trail', particleCount: 8 }],
    },
    reload: { reloadCount: 0, lastStatus: null },
    composition: { entityCount: 3, parentedEntityCount: 0 },
  },
};

run.route_attempts = {
  present: true,
  status: 'passed',
  attempt_count: 1,
  malformed_count: 0,
  evidence_refs: ['evidence/route-attempts/route-attempt.json'],
  boundary: 'Read-only route attempt evidence; Studio must not run solvers.',
  attempts: [{
    attemptId: 'qa14_6_collect_goal_route',
    objectiveId: 'collect-goal-then-exit',
    scenarioId: 'collect-and-exit',
    outcome: 'passed',
    startState: { stateId: 'start-left-of-goal' },
    budgetUsed: { actionsUsed: 2, maxActions: 8 },
  }],
};

run.visual_comparisons = {
  present: true,
  status: 'changed',
  comparison_count: 1,
  changed_count: 1,
  malformed_count: 0,
  evidence_refs: ['evidence/visual-comparisons/visual-comparison.json'],
  boundary: 'Read-only visual comparison evidence; Studio must not compute trusted diffs.',
  summaries: [{
    comparisonId: 'qa14_7_collect_goal_visual',
    scenarioId: 'collect-and-exit',
    checkpointId: 'goal-checkpoint',
    outcome: 'changed',
    failureClassification: 'visual_regression_candidate',
    changedPixels: 64,
    changedPercentX1000: 2,
  }],
};

run.level_design_inspection = {
  present: true,
  schemaVersion: 'studio-level-design-inspection-v1',
  status: 'ready',
  boundary: 'Read-only Studio level design inspection; no browser trusted writes, no command bridge, no auto-apply, no auto-merge, no self-approval, no production editor, no autonomous full game generation, and no visual scripting.',
  panels: [{
    id: 'intent',
    label: 'Intent and constraints',
    kind: 'level-intent',
    status: 'ready',
    items: [{ label: 'Intent', value: 'Collect key then exit without softlock.' }],
    refs: [{ id: 'intent', path: 'examples/level-intent/collect-exit.intent.json' }],
  }, {
    id: 'generation-plan',
    label: 'Generation plan',
    kind: 'scene-generation-plan',
    status: 'ready',
    items: [{ label: 'Steps', value: 'tilemap draft, placement draft, objective proof' }],
    refs: [{ id: 'plan', path: 'examples/scene-generation-plan/collect-exit.plan.json' }],
  }, {
    id: 'tile-entity-draft',
    label: 'Tile and entity drafts',
    kind: 'draft-summary',
    status: 'needs-review',
    items: [{ label: 'Drafts', value: 'terrain corridor plus key and exit placement' }],
    refs: [{ id: 'tilemap-draft', path: 'runs/level-drafts/collect/tilemap-draft.json' }],
  }, {
    id: 'reachability-pathing',
    label: 'Reachability and pathing',
    kind: 'evidence',
    status: 'passed',
    items: [{ label: 'Route', value: 'spawn -> key -> exit reachable in 8 actions' }],
    refs: [{ id: 'route', path: 'runs/level-drafts/collect/reachability.json' }],
  }, {
    id: 'objective-proof',
    label: 'Objective proof',
    kind: 'evidence',
    status: 'passed',
    items: [{ label: 'Proof', value: 'win condition can be satisfied after key pickup' }],
    refs: [{ id: 'proof', path: 'runs/level-drafts/collect/objective-proof.json' }],
  }, {
    id: 'difficulty-pacing',
    label: 'Difficulty and pacing heuristic',
    kind: 'heuristic',
    status: 'advisory',
    items: [{ label: 'Pacing', value: 'advisory only; not subjective game quality proof' }],
    refs: [{ id: 'heuristic', path: 'runs/level-drafts/collect/difficulty.json' }],
  }, {
    id: 'visual-semantic-diff',
    label: 'Visual and semantic diff',
    kind: 'diff-evidence',
    status: 'changed',
    items: [{ label: 'Diff', value: '64 changed pixels and one objective semantic delta' }],
    refs: [{ id: 'diff', path: 'runs/level-drafts/collect/visual-semantic-diff.json' }],
  }, {
    id: 'review-apply-status',
    label: 'Review and apply status',
    kind: 'review-gate',
    status: 'blocked',
    items: [{ label: 'Apply', value: 'blocked until accepted non-self review' }],
    refs: [{ id: 'apply', path: 'runs/level-drafts/collect/review-gated-apply.json' }],
    commands: [{ command: 'cargo run -p ouroforge-cli -- level apply-preview <script>alert(1)</script>' }],
  }],
};

run.mutation_artifacts.push({
  id: 'source-patch-stale-target-guard',
  kind: 'application/json',
  path: 'mutation/source-patch-stale-target-guard.json',
  metadata: { read_only: true },
  value: {
    guardId: 'stale-guard-1',
    status: 'fresh',
    evidenceFreshness: {
      patchPreviewRef: 'mutation/preview.json',
      sandboxReportRef: 'sandbox/preview-1/evidence/report.json',
      reviewDecisionRef: 'mutation/review-decision.json',
      fileClassReportRef: 'evidence/file-class.json',
      diffIntegrityReportRef: 'evidence/diff.json',
      applyTransactionRef: 'mutation/source-patch-apply-transaction.json',
    },
    worktreeContextRef: 'evidence/source-apply-worktree-context.json',
    targets: [{ path: 'examples/source-patch-apply-transaction-v1/scenario-regression.json', fileClass: 'scenario_regression_fixture', fileStatus: 'exists_and_matches_expected_before_hash' }],
    readModel: {
      status: 'fresh_current_targets_and_linked_evidence_no_apply_authority',
      readinessLabel: 'fresh_guard_metadata_only_no_apply_authority',
      blockedReasons: [],
      forbiddenActions: ['apply_patch', 'merge_branch', 'execute_command', 'write_trusted_file', 'browser_command_bridge'],
    },
  },
});

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
assert.match(cockpit.renderRouteAttemptEvidenceSurface(run), /qa14_6_collect_goal_route/);
assert.match(cockpit.renderRouteAttemptEvidenceSurface(run), /Studio must not run solvers/);
assert.match(cockpit.renderStudioNavigation(run), /Studio v2 demo surfaces/);
assert.match(cockpit.renderStudioNavigation(run), /Visual comparison evidence/);
assert.match(cockpit.renderStudioNavigation(run), /Level design inspection/);
assert.equal(cockpit.studioSurfaceSummary(run).filter((surface) => surface.present).length, 31);
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
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Physics contacts/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /floor:player/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Declared scene transitions/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /to_boss/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /runtime\.scene\.transition/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /runtime\.audio\.play/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Audio limitation warnings/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /audible_output_not_verified/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /bus sfx/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /VFX events/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /run-dust/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /state run/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /3D collision/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /1 contact\(s\), 1 trigger\(s\), 1 invalid/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /&lt;missing collider&gt;/);
assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /no full 3D physics engine/);
assert.doesNotMatch(cockpit.renderRuntimeEventInspectionSurface(run), /<missing/);
assert.match(cockpit.renderRuntimeEventInspectionSurface({ engine_summaries: { present: true, collision: '<bad>', transition: null, events: [] } }), /collision summary missing or malformed/);
assert.match(cockpit.renderRuntimeEventInspectionSurface({ engine_summaries: { present: false, empty_state: '<script>events</script>' } }), /&lt;script&gt;events&lt;\/script&gt;/);
const xssRuntimeEvents = cockpit.renderRuntimeEventInspectionSurface({ engine_summaries: { present: true, collision: { present: true, rules: { '<script>': { bad: '<img>' } }, colliderEntityCount: 1, collisionEventCount: 1, events: [{ type: '<script>', payload: '<img>' }] }, scene3d_collision: { present: true, contactCount: 0, triggerCount: 1, invalidColliderCount: 1, events: [{ type: '<script>3d</script>', pairId: '<img>' }], invalidColliders: [{ colliderRef: '<svg>', reason: '<b>' }], boundary: '<i>boundary</i>' }, transition: { present: true, currentSceneId: '<svg>', declaredTransitionCount: 1, declaredTransitions: [{ id: '<b>', toScene: '<img>' }], transitionEventCount: 1, transitions: [{ type: '<b>', to: '<img>' }], lastReloadStatus: '<i>' }, events: { present: true, animationEntityCount: 1, audioEventCount: 1, audioWarningCount: 1, collisionEventCount: 1, vfxEventCount: 1, animationEntities: [{ entityId: '<img>', mode: '<svg>', activeState: '<b>', currentClip: '<script>', frameIndex: 1 }], audioEvents: [{ name: '<script>', intentKind: '<svg>', busId: '<img>', type: '<b>', clipId: '<i>' }], audioWarnings: [{ warning: '<script>', requestId: '<img>' }], vfxEvents: [{ emitterId: '<script>', kind: '<img>' }] } } });
assert.doesNotMatch(xssRuntimeEvents, /<script>|<img>|<svg>|<b>|<i>/);
assert.match(xssRuntimeEvents, /&lt;script&gt;/);
assert.match(cockpit.renderEngineExpansionSurface(run), /Engine Expansion state/);
assert.match(cockpit.renderEngineExpansionSurface(run), /trigger-flags-v1-fixture/);
assert.match(cockpit.renderEngineExpansionSurface(run), /Gameplay\/HUD/);
assert.match(cockpit.renderEngineExpansionSurface(run), /Input actions/);
assert.match(cockpit.renderEngineExpansionSurface(run), /VFX/);
assert.match(cockpit.renderEngineExpansionSurface(run), /1 VFX entit\(ies\), 1 event\(s\)/);
assert.match(cockpit.renderEngineExpansionSurface(run), /Render breakdown/);
assert.match(cockpit.renderEngineExpansionSurface(run), /1 element\(s\), 1 absence diagnostic\(s\)/);
assert.match(cockpit.renderEngineExpansionSurface(run), /3 flag\(s\), 2 true, 1 trigger event\(s\), 2 HUD value\(s\)/);
assert.match(cockpit.renderEngineExpansionSurface({ engine_summaries: { present: false, empty_state: '<script>x</script>' } }), /&lt;script&gt;x&lt;\/script&gt;/);
assert.match(cockpit.renderStudioNavigation(run), /Camera\/layer inspection/);
assert.match(cockpit.renderCameraLayerInspectionSurface(run), /Camera\/layer inspection/);
assert.match(cockpit.renderCameraLayerInspectionSurface(run), /&lt;follow-player&gt;/);
assert.match(cockpit.renderCameraLayerInspectionSurface(run), /follow &lt;player&gt;/);
assert.match(cockpit.renderCameraLayerInspectionSurface(run), /parallax 50/);
assert.match(cockpit.renderCameraLayerInspectionSurface(run), /screen-space/);
assert.match(cockpit.renderCameraLayerInspectionSurface(run), /World-to-screen samples/);
assert.match(cockpit.renderCameraLayerInspectionSurface(run), /scene mutation/);
assert.doesNotMatch(cockpit.renderCameraLayerInspectionSurface(run), /<follow-player>/);
assert.match(cockpit.renderCameraLayerInspectionSurface({
  engine_summaries: {
    present: true,
    camera: {
      present: true,
      scene3dCamera: {
        present: true,
        activeCameraId: '<main-camera>',
        cameraCount: 1,
        cameras: [{
          id: '<main-camera>',
          active: true,
          projection: { kind: 'perspective', fovDegrees: 60, near: 1, far: 1000 },
          viewport: { width: 640, height: 360 },
          aspectRatioX1000: 1777
        }]
      }
    }
  }
}), /3D cameras/);
assert.match(cockpit.renderCameraLayerInspectionSurface({
  engine_summaries: {
    present: true,
    camera: {
      present: true,
      scene3dCamera: {
        present: true,
        activeCameraId: '<main-camera>',
        cameras: [{
          id: '<main-camera>',
          active: true,
          projection: { kind: 'perspective', fovDegrees: 60, near: 1, far: 1000 },
          viewport: { width: 640, height: 360 },
          aspectRatioX1000: 1777
        }]
      }
    }
  }
}), /&lt;main-camera&gt;/);
assert.doesNotMatch(cockpit.renderCameraLayerInspectionSurface({
  engine_summaries: {
    present: true,
    camera: {
      present: true,
      scene3dCamera: {
        present: true,
        activeCameraId: '<main-camera>',
        cameras: [{ id: '<main-camera>', projection: { kind: '<script>' }, viewport: {} }]
      }
    }
  }
}), /<script>/);
assert.match(cockpit.renderCameraLayerInspectionSurface({ engine_summaries: { present: true, camera: '<bad>' } }), /missing or malformed/);
assert.match(cockpit.renderStudioNavigation(run), /Runtime profiler inspection/);
assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /Runtime profiler inspection/);
assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /&lt;frame-12&gt;/);
assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /&lt;renderMs&gt;/);
assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /remote telemetry/);
assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /evidence inputs, not trusted profiler authority/);
assert.doesNotMatch(cockpit.renderRuntimeProfilerInspectionSurface(run), /<frame-12>|<renderMs>/);
assert.match(cockpit.renderRuntimeProfilerInspectionSurface({ engine_summaries: { present: true, runtime_frame_budget: '<bad>' } }), /missing or malformed/);
const xssRuntimeProfiler = cockpit.renderRuntimeProfilerInspectionSurface({ engine_summaries: { present: true, runtime_frame_budget: { frameId: '<script>frame</script>', sceneId: '<img>', scenarioId: '<svg>', timings: { renderMs: '<b>' }, budget: { renderMs: '<i>' }, counts: { entityCount: '<p>' }, status: '<script>bad</script>', violations: [{ field: '<script>render</script>', actualMs: '<img>', budgetMs: '<svg>' }], readOnlyInspection: { disallowedActions: ['<script>write</script>'] }, authority: '<b>authority</b>' } } });
assert.doesNotMatch(xssRuntimeProfiler, /<script>|<img>|<svg>|<b>|<i>|<p>/);
assert.match(xssRuntimeProfiler, /&lt;script&gt;render&lt;\/script&gt;/);
assert.match(cockpit.renderStudioNavigation(run), /Runtime save\/state inspection/);
assert.match(cockpit.renderRuntimeStateInspectionSurface(run), /Runtime save\/state inspection/);
assert.match(cockpit.renderRuntimeStateInspectionSurface(run), /&lt;tick-12-state&gt;/);
assert.match(cockpit.renderRuntimeStateInspectionSurface(run), /Save\/load events/);
assert.match(cockpit.renderRuntimeStateInspectionSurface(run), /Replay digest comparisons/);
assert.match(cockpit.renderRuntimeStateInspectionSurface(run), /trusted persistence remains Rust\/local generated evidence/);
assert.match(cockpit.renderRuntimeStateInspectionSurface(run), /browser runtime control/);
assert.doesNotMatch(cockpit.renderRuntimeStateInspectionSurface(run), /<tick-12-state>|<digest-value>/);
assert.match(cockpit.renderRuntimeStateInspectionSurface({ engine_summaries: { present: true, runtime_state: '<bad>' } }), /No runtime save\/load\/replay state read model/);
const xssRuntimeState = cockpit.renderRuntimeStateInspectionSurface({ engine_summaries: { present: true, runtime_state: { present: true, stateId: '<script>state</script>', sceneId: '<img>', digest: { algorithm: '<svg>', value: '<b>' }, saveEvents: [{ type: '<script>save</script>', payload: { saveId: '<img>', slotId: '<svg>', stateDigest: { value: '<b>' } } }], replayEvents: [{ payload: { frameId: '<i>', status: '<p>', expected: { value: '<em>' }, actual: { value: '<strong>' } } }], snapshots: [{ snapshotId: '<script>snap</script>', tick: '<img>' }], readOnlyInspection: { disallowedActions: ['<script>write</script>'] } } } });
assert.doesNotMatch(xssRuntimeState, /<script>|<img|<svg>|<b>|<i>|<p>|<em>/);
assert.match(xssRuntimeState, /&lt;script&gt;state&lt;\/script&gt;/);
assert.match(cockpit.renderStudioNavigation(run), /3D inspection/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /Scene hierarchy/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /Active camera\/projection/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /Mesh\/material refs/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /Collision\/trigger evidence/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /Animation state/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /Scenario verdicts/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /&lt;cube-node&gt;/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /&lt;perspective&gt;/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /&lt;cube-mesh&gt;/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /Studio cannot execute scenarios/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /not a 3D editor/);
assert.match(cockpit.renderStudio3dInspectionSurface(run), /viewport persistence/);
assert.doesNotMatch(cockpit.renderStudio3dInspectionSurface(run), /<cube-node>|<perspective>|<cube-mesh>|<studio-3d-scenario>/);
assert.match(cockpit.renderStudio3dInspectionSurface({ engine_summaries: { present: false, empty_state: '<script>missing</script>' } }), /&lt;script&gt;missing&lt;\/script&gt;/);
assert.match(cockpit.renderStudio3dInspectionSurface({ engine_summaries: { present: true, scene3d_hierarchy: '<bad>', scene3d_camera: { present: false, emptyState: '<camera missing>' }, scene3d_render: { present: false, emptyState: '<render missing>' } } }), /Malformed 3D read model/);
const xssStudio3d = cockpit.renderStudio3dInspectionSurface({
  engine_summaries: {
    present: true,
    scene3d_hierarchy: { present: true, transforms: [{ nodeId: '<script>node</script>', parentId: '<img>', worldTransform: { translation: '<svg>' } }], boundary: '<b>hierarchy</b>' },
    scene3d_camera: { present: true, cameras: [{ id: '<script>camera</script>', active: true, projection: { kind: '<img>', fovDegrees: '<svg>', near: '<b>', far: '<i>' }, viewport: { width: '<p>' } }] },
    scene3d_render: { present: true, renderables: [{ id: '<script>cube</script>', nodeId: '<img>', meshRef: '<svg>', materialRef: '<b>', cameraId: '<i>', visible: false, fallbackReason: '<p>skip</p>' }], fallbackReasons: ['<script>fallback</script>'] },
    scene3d_collision: { present: true, events: [{ type: '<script>collision</script>', pairId: '<img>' }], invalidColliders: [{ colliderRef: '<svg>', reason: '<b>' }] },
    scene3d_animation: { present: true, states: [{ clipId: '<script>clip</script>', targetNodeId: '<img>', channel: '<svg>', playing: false }], events: [{ type: '<b>event</b>' }] },
    scene3d_scenario_verdicts: { present: true, verdicts: [{ scenarioId: '<script>scenario</script>', status: '<img>', assertionCount: '<svg>' }], boundary: '<b>verdict</b>' },
  },
});
assert.doesNotMatch(xssStudio3d, /<script>|<img|<svg>|<b>|<i>|<p>/);
assert.match(xssStudio3d, /&lt;script&gt;node&lt;\/script&gt;/);
assert.match(cockpit.renderStudioNavigation(run), /Render breakdown inspection/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /Renderable draw order/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /Render queue/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /Queue status/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /Tilemap draw tiles/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /Asset-backed tiles/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /Missing tile refs/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /tiles 2/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /3D render smoke/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /3D smoke visible\/skipped/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /1\/1/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /&lt;scene3d-cube&gt;/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /no WebGPU, GLTF import, PBR/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /entity:player/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /entity-hidden/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /sprite\.visible=false/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /sprite hidden/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /browser runtime control/);
assert.match(cockpit.renderRenderBreakdownInspectionSurface({ engine_summaries: { present: true, render_breakdown: '<bad>' } }), /missing or malformed/);
assert.match(cockpit.renderInputActionInspectionSurface(run), /Input action mapping/);
assert.match(cockpit.renderInputActionInspectionSurface(run), /&lt;dash&gt;/);
assert.match(cockpit.renderInputActionInspectionSurface(run), /&lt;move_right&gt; \/ &lt;dash&gt;/);
assert.match(cockpit.renderInputActionInspectionSurface(run), /browser runtime control/);
assert.doesNotMatch(cockpit.renderInputActionInspectionSurface(run), /<dash>/);
assert.match(cockpit.renderInputActionInspectionSurface({ engine_summaries: { present: true, input: '<bad>' } }), /No input action read model/);
const xssInputAction = cockpit.renderInputActionInspectionSurface({ engine_summaries: { present: true, input: { present: true, mappedActionCount: 1, activeActions: ['<script>run</script>'], rawInput: { keys: { '<img>': true } }, diagnostics: { missingActions: ['<svg>'], conflictingBindings: [{ key: '<b>', actions: ['<i>', '<p>'] }], readOnlyInspection: { disallowedActions: ['<script>write</script>'] } } } } });
assert.doesNotMatch(xssInputAction, /<script>|<img|<svg>|<b>|<i>|<p>/);
assert.match(xssInputAction, /&lt;svg&gt;/);
const xssRenderBreakdown = cockpit.renderRenderBreakdownInspectionSurface({ engine_summaries: { present: true, render_breakdown: { present: true, frameId: '<script>frame</script>', sceneId: '<img>', elements: [{ renderableId: '<script>renderable</script>', entityId: '<img src=x onerror=alert(1)>', frameId: '<svg>', drawOrder: '<b>', layer: '<i>', primitiveCategory: '<p>', debugLabel: '<script>label</script>' }], absenceDiagnostics: [{ entityId: '<script>hidden</script>', reason: '<img>', layer: '<svg>', detail: '<b>detail</b>' }], readOnlyInspection: { disallowedActions: ['<script>write</script>'] }, boundary: '<script>boundary</script>' } } });
assert.doesNotMatch(xssRenderBreakdown, /<script>|<img|<svg>|<b>|<i>|<p>/);
assert.match(xssRenderBreakdown, /&lt;script&gt;renderable&lt;\/script&gt;/);
const xssRenderQueue = cockpit.renderRenderBreakdownInspectionSurface({ engine_summaries: { present: true, render_breakdown: { present: true, frameId: 'frame', elements: [] }, render_queue: { present: true, validation: { status: '<script>ready</script>' }, renderables: [{ id: '<script>queue</script>', sourceKind: '<img>', sourceId: '<svg>', drawOrder: '<b>', layer: '<i>', primitiveKind: '<p>', visible: false, fallbackReason: '<script>skip</script>' }] } } });
assert.doesNotMatch(xssRenderQueue, /<script>|<img|<svg>|<b>|<i>|<p>/);
assert.match(xssRenderQueue, /&lt;script&gt;queue&lt;\/script&gt;/);
const xssScene3dRenderCockpit = cockpit.renderRenderBreakdownInspectionSurface({ engine_summaries: { present: true, render_breakdown: { present: true, frameId: 'frame', elements: [] }, scene3d_render: { present: true, frameId: '<script>frame</script>', sceneId: '<img>', cameraId: '<svg>', renderables: [{ id: '<script>cube</script>', nodeId: '<img>', meshRef: '<svg>', materialRef: '<b>', primitive: '<i>', cameraId: '<p>', visible: false, fallbackReason: '<script>skip</script>' }], fallbackReasons: ['<script>fallback</script>'], boundary: '<script>boundary</script>' } } });
assert.doesNotMatch(xssScene3dRenderCockpit, /<script>|<img|<svg>|<b>|<i>|<p>/);
assert.match(xssScene3dRenderCockpit, /&lt;script&gt;cube&lt;\/script&gt;/);
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
assert.match(cockpit.renderIntegration(run), /Render breakdown inspection/);
assert.match(cockpit.renderIntegration(run), /Runtime profiler inspection/);
assert.match(cockpit.renderIntegration(run), /Runtime save\/state inspection/);
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
const xssVisualComparison = { visual_comparisons: { present: true, status: '<script>status</script>', boundary: '<script>comparison-boundary</script>', evidence_refs: ['<script>ref</script>'], summaries: [{ comparisonId: '<img src=x onerror=alert(1)>', scenarioId: '<script>scenario</script>', checkpointId: '<script>checkpoint</script>', outcome: '<script>outcome</script>', failureClassification: '<script>classification</script>', changedPixels: '<script>pixels</script>', changedPercentX1000: '<script>percent</script>' }] } };
assert.ok(!cockpit.renderVisualComparisonEvidenceSurface(xssVisualComparison).includes('<script>classification</script>'), 'visual comparison classification must be escaped');
assert.ok(!cockpit.renderVisualComparisonEvidenceSurface(xssVisualComparison).includes('<script>comparison-boundary</script>'), 'visual comparison boundary must be escaped');
assert.ok(!cockpit.renderVisualComparisonEvidenceSurface(xssVisualComparison).includes('<img src=x onerror=alert(1)>'), 'visual comparison id must be escaped');
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

const pluginExtensionCoverageMatrix = fs.readFileSync(require.resolve('../../docs/scenario-coverage-v16-plugin-extension.md'), 'utf8');
assert.match(pluginExtensionCoverageMatrix, /Scenario Coverage v16: Plugin Extension Regression Suite/);
assert.match(pluginExtensionCoverageMatrix, /PES10\.16\.3 integrates the Scenario Coverage v16 matrix with local verification/);
assert.match(pluginExtensionCoverageMatrix, /browser-facing verification fails if the Scenario\s+Coverage v16 local-gate wording/);
assert.match(pluginExtensionCoverageMatrix, /PES10\.16\.studio-display/);
assert.match(pluginExtensionCoverageMatrix, /PES10\.16\.scenario-template/);
assert.match(pluginExtensionCoverageMatrix, /PES10\.16\.block-native-extension/);
assert.match(pluginExtensionCoverageMatrix, /PES10\.16\.block-ci-mutation/);
assert.match(pluginExtensionCoverageMatrix, /No arbitrary JavaScript or runtime plugin execution/);
assert.match(pluginExtensionCoverageMatrix, /No shell command execution, browser command bridge, or local server command bridge/);
assert.match(pluginExtensionCoverageMatrix, /No native export, publish\/deploy mutation, signing, upload, or release automation/);
assert.match(pluginExtensionCoverageMatrix, /local cargo\/node integration/);
assert.match(pluginExtensionCoverageMatrix, /#1 remains the broad roadmap\/governance anchor and #23 remains the protected/);

const sourcePatchCoverageMatrix = fs.readFileSync(require.resolve('../../docs/source-patch-preview-coverage-matrix-v1.md'), 'utf8');
assert.match(sourcePatchCoverageMatrix, /Scenario Coverage v6 \/ SMP1\.11\.3 coverage matrix/);
assert.match(sourcePatchCoverageMatrix, /Forbidden file classes/);
assert.match(sourcePatchCoverageMatrix, /Sandbox dry-run pass\/fail/);
assert.match(sourcePatchCoverageMatrix, /Dashboard display/);
assert.match(sourcePatchCoverageMatrix, /Studio\/cockpit display/);
assert.match(sourcePatchCoverageMatrix, /No source patch apply to the trusted main worktree/);
assert.match(sourcePatchCoverageMatrix, /node examples\/evidence-dashboard\/dashboard\.test\.cjs/);
assert.match(sourcePatchCoverageMatrix, /node examples\/authoring-cockpit\/cockpit\.test\.cjs/);

const multiAgentCoverageMatrix = fs.readFileSync(require.resolve('../../docs/multi-agent-pipeline-coverage-matrix-v1.md'), 'utf8');
assert.match(multiAgentCoverageMatrix, /Scenario Coverage v12 \/ MAP13\.16\.3 coverage matrix/);
assert.match(multiAgentCoverageMatrix, /Role validation/);
assert.match(multiAgentCoverageMatrix, /Task board\/status transitions/);
assert.match(multiAgentCoverageMatrix, /Ownership conflicts/);
assert.match(multiAgentCoverageMatrix, /Handoff v2 \/ handoff contract/);
assert.match(multiAgentCoverageMatrix, /State snapshot\/staleness/);
assert.match(multiAgentCoverageMatrix, /Review\/critic independence/);
assert.match(multiAgentCoverageMatrix, /QA queue \/ QA worker assignment/);
assert.match(multiAgentCoverageMatrix, /Performance\/regression lane/);
assert.match(multiAgentCoverageMatrix, /Build\/release design gate/);
assert.match(multiAgentCoverageMatrix, /Decision ledger append-only/);
assert.match(multiAgentCoverageMatrix, /Production\/authoring loop evidence bundle/);
assert.match(multiAgentCoverageMatrix, /Dashboard display/);
assert.match(multiAgentCoverageMatrix, /Studio\/cockpit display/);
assert.match(multiAgentCoverageMatrix, /Malformed\/missing\/stale\/unresolved conflict evidence/);
assert.match(multiAgentCoverageMatrix, /no hidden background agents/);
assert.match(multiAgentCoverageMatrix, /no unbounded spawning/);
assert.match(multiAgentCoverageMatrix, /no auto-apply, auto-merge, or self-approval/);
assert.match(multiAgentCoverageMatrix, /no browser trusted writes or command bridge/);
assert.match(multiAgentCoverageMatrix, /no remote worker pool, hosted worker pool, cloud worker pool, or remote\/cloud swarm/);
assert.match(multiAgentCoverageMatrix, /no dependency mutation, CI mutation, or workflow mutation/);
assert.match(multiAgentCoverageMatrix, /no production-ready claim and no Godot replacement claim/);
assert.match(multiAgentCoverageMatrix, /node examples\/evidence-dashboard\/dashboard\.test\.cjs/);
assert.match(multiAgentCoverageMatrix, /node examples\/authoring-cockpit\/cockpit\.test\.cjs/);

const cockpitReadme = fs.readFileSync(require.resolve('./README.md'), 'utf8');
assert.match(cockpitReadme, /Studio Draft Authoring Surface v1/);
assert.match(cockpitReadme, /scene, tilemap, and\s+asset-reference draft rows/);
assert.match(cockpitReadme, /copyable draft JSON text/);
assert.match(cockpitReadme, /does not persist trusted draft state, write/);
assert.match(cockpitReadme, /Production 2D runtime inspection boundary/);
assert.match(cockpitReadme, /renderer\/layer\/camera, physics\/collision\/contact, input\/action\/replay/);
assert.match(cockpitReadme, /save\/load\/runtime-state digest, and profiler\/frame-budget/);
assert.match(cockpitReadme, /do not write source, scene, tilemap, asset, save, project,\s+dashboard, run, or evidence files/);
assert.match(cockpitReadme, /do not\s+execute commands; do not bridge to a local server/);
assert.match(cockpitReadme, /do not claim production editor, hosted Studio, native export,\s+plugin runtime, visual scripting, public launch, production-ready engine, or\s+Godot replacement behavior/);
assert.match(cockpitReadme, /Generated run\/dashboard\/\s*screenshot\/temp\/local tool outputs stay untracked/);
assert.match(cockpitReadme, /Studio Level Design Inspection Surface v1/);
assert.match(cockpitReadme, /level intent, generation plan, tile\/entity drafts/);
assert.match(cockpitReadme, /command\s+text remains copyable inert text only/);
assert.match(cockpitReadme, /does not write trusted files, execute commands, bridge to a local\s+server, auto-apply, auto-merge, self-approve/);
assert.match(cockpitReadme, /docs\/studio-level-design-inspection-surface-v1\.md/);
const studioLevelDesignInspectionDoc = fs.readFileSync(require.resolve('../../docs/studio-level-design-inspection-surface-v1.md'), 'utf8');
assert.match(studioLevelDesignInspectionDoc, /Issue: #639/);
assert.match(studioLevelDesignInspectionDoc, /level intent and design constraints/);
assert.match(studioLevelDesignInspectionDoc, /scene generation plan/);
assert.match(studioLevelDesignInspectionDoc, /tilemap and entity\/objective\/encounter drafts/);
assert.match(studioLevelDesignInspectionDoc, /reachability and pathing evidence/);
assert.match(studioLevelDesignInspectionDoc, /objective completion and win\/loss proof/);
assert.match(studioLevelDesignInspectionDoc, /difficulty, pacing, and balance heuristic evidence/);
assert.match(studioLevelDesignInspectionDoc, /visual and semantic diff evidence/);
assert.match(studioLevelDesignInspectionDoc, /review and apply status/);
assert.match(studioLevelDesignInspectionDoc, /escape every exported label, value, ref, status, schema, boundary/);
assert.match(studioLevelDesignInspectionDoc, /keep command strings inside inert copyable text/);
assert.match(studioLevelDesignInspectionDoc, /browser trusted writes/);
assert.match(studioLevelDesignInspectionDoc, /command bridge or local server bridge/);
assert.match(studioLevelDesignInspectionDoc, /auto-apply or auto-merge/);
assert.match(studioLevelDesignInspectionDoc, /self-approval or reviewer bypass/);
assert.match(studioLevelDesignInspectionDoc, /autonomous full game generation/);
assert.match(studioLevelDesignInspectionDoc, /production editor, full visual level editor/);
assert.match(studioLevelDesignInspectionDoc, /#1 and #23 remain open/);
assert.doesNotMatch(studioLevelDesignInspectionDoc, /trusted browser write enabled|command bridge enabled|auto-apply enabled|auto-merge enabled|production-ready claim enabled/);
const production2dStudioInspectionDoc = fs.readFileSync(require.resolve('../../docs/production-2d-studio-inspection-v1.md'), 'utf8');
assert.match(production2dStudioInspectionDoc, /Issue: #593/);
assert.match(production2dStudioInspectionDoc, /Display contract/);
assert.match(production2dStudioInspectionDoc, /save\/load snapshots, runtime state digests, and replay digest comparisons/);
assert.match(production2dStudioInspectionDoc, /Rust\/local workflows own trusted validation, generated evidence writing/);
assert.match(production2dStudioInspectionDoc, /must not:\n\n- write source, scene, tilemap, asset, save, project, dashboard, run, or evidence/);
assert.match(production2dStudioInspectionDoc, /persist trusted state through browser storage, native file APIs, hidden local\s+servers, or command bridges/);
assert.match(production2dStudioInspectionDoc, /claim to be a production editor, visual scripting system, native export path,\s+plugin runtime, hosted\/cloud Studio, public launch surface, production-ready\s+engine, secure sandbox, broad compatibility-stable API, shipped-game proof, or\s+Godot replacement/);
assert.match(production2dStudioInspectionDoc, /P2D8\.13\.3 adds documentation\/tests only/);
assert.match(production2dStudioInspectionDoc, /#1 remains open as the broad roadmap\/vision anchor/);
assert.match(production2dStudioInspectionDoc, /#23 remains open as the repo-memory\/design context anchor/);
assert.match(production2dStudioInspectionDoc, /no browser trusted write API, command execution API, command bridge, or local\s+server bridge was introduced/);
const roadmapDoc = fs.readFileSync(require.resolve('../../docs/roadmap.md'), 'utf8');
assert.match(roadmapDoc, /Production 2D Engine Core v1 \/ bounded local 2D vertical-slice core/);
assert.match(roadmapDoc, /Production 2D Engine Core v1 completion covers the #581-#593 evidence chain/);
assert.match(roadmapDoc, /3D Capability Gate v1 \/ bounded local 3D capability evidence/);
assert.match(roadmapDoc, /3D Capability Gate v1 completion covers the #596-#608 evidence chain/);
assert.match(roadmapDoc, /read-only Studio 3D inspection, and this roadmap\/#1 governance\s+refresh/);
assert.match(roadmapDoc, /It remains a bounded local capability gate/);
assert.match(roadmapDoc, /does not add a full 3D\s+editor, production 3D renderer, broad 3D compatibility promise/);
assert.match(roadmapDoc, /native export, plugin runtime, hosted\/cloud\/server\/auth behavior, browser\s+trusted writes, command bridges, unrestricted source apply/);
assert.match(roadmapDoc, /current Godot replacement positioning/);
assert.match(roadmapDoc, /Agentic Scene and Level Designer v1 completion covers the #627-#642 evidence\s+chain/);
assert.match(roadmapDoc, /After #625, Gameplay Scripting \/ Logic System v1 \(#611-#625\) is complete/);
assert.match(roadmapDoc, /not arbitrary\s+third-party code loading, plugin runtime, browser command bridge, hosted\s+execution, production scripting, source-apply authority, public-launch approval,\s+or Godot replacement scope/);
assert.match(roadmapDoc, /GDD-to-Playable Prototype v1 \(#644-#661\)/);
assert.match(roadmapDoc, /#611-#625 do\s+not authorize arbitrary executable scripts/);
assert.match(roadmapDoc, /#1 remains the broad vision and implementation-roadmap anchor/);
assert.match(roadmapDoc, /#23 remains open as the repo-memory\/design context anchor/);
const production2dCoreDoc = fs.readFileSync(require.resolve('../../docs/production-2d-engine-core-v1.md'), 'utf8');
assert.match(production2dCoreDoc, /Completion status after #594/);
assert.match(production2dCoreDoc, /Production 2D Engine Core v1 is recorded complete after issues #581-#593 closed/);
assert.match(production2dCoreDoc, /escaped read-only Studio 2D inspection surfaces \(#593\)/);
assert.match(production2dCoreDoc, /no 3D\s+implementation, native export, plugin runtime, hosted\/cloud\/server\/auth behavior/);
assert.match(production2dCoreDoc, /recommended next dependency-ordered technical branch is 3D Capability Gate v1\s+\(#596-#608\)/);
assert.match(production2dCoreDoc, /production-2d-engine-core-governance-handoff\.md/);
const production2dGovernanceHandoff = fs.readFileSync(require.resolve('../../docs/production-2d-engine-core-governance-handoff.md'), 'utf8');
assert.match(production2dGovernanceHandoff, /#1 handoff comment: <https:\/\/github\.com\/shaun0927\/Ouroforge\/issues\/1#issuecomment-4624951606>/);
assert.match(production2dGovernanceHandoff, /Production 2D Engine Core v1 is complete as a bounded local-first 2D vertical-\s*slice evidence milestone/);
assert.match(production2dGovernanceHandoff, /P2D8\.14\.1 roadmap\/docs refresh: PR #1082 merged at\s+`a03cb41920facbefd48763970d09f1144f6cd754`/);
assert.match(production2dGovernanceHandoff, /Generated-state audit showed only expected ignored local\/tool output categories/);
assert.match(production2dGovernanceHandoff, /does not authorize 3D implementation outside the scoped gate,\s+native export, plugin runtime, hosted\/cloud\/server\/auth behavior, browser trusted\s+writes, command bridges, unrestricted source apply/);
assert.match(production2dGovernanceHandoff, /next dependency-ordered technical branch is \*\*3D Capability Gate v1\s+\(#596-#608\)\*\*/);
assert.match(production2dGovernanceHandoff, /#1 remains open as the broad roadmap\/vision anchor/);
assert.match(production2dGovernanceHandoff, /#23 remains open as the repo-memory\/design context anchor/);
const threeDCapabilityGateDoc = fs.readFileSync(require.resolve('../../docs/3d-capability-gate-v1.md'), 'utf8');
assert.match(threeDCapabilityGateDoc, /Issue: #596/);
assert.match(threeDCapabilityGateDoc, /canonical scope contract for #1 Milestone 9/);
assert.match(threeDCapabilityGateDoc, /capability gate, not a full 3D engine, production-\s*ready engine, broad 3D compatibility promise/);
assert.match(threeDCapabilityGateDoc, /small deterministic local 3D demo scene/);
assert.match(threeDCapabilityGateDoc, /3D scene graph and transform hierarchy/);
assert.match(threeDCapabilityGateDoc, /3D runtime probe contract/);
assert.match(threeDCapabilityGateDoc, /Scenario Coverage v8 regression suite/);
assert.match(threeDCapabilityGateDoc, /Rust\/local code owns trusted validation, persistence, source-like fixture\s+validation/);
assert.match(threeDCapabilityGateDoc, /Generated 3D screenshots, runs, previews, dashboard data, temp projects/);
assert.match(threeDCapabilityGateDoc, /3D capability work must remain additive and must not regress existing 2D Seeds/);
assert.match(threeDCapabilityGateDoc, /no Godot replacement, production-ready, broad 3D\s+compatibility, secure-sandbox, native export, plugin runtime, hosted\/cloud, or\s+autonomous launch claims/);
assert.match(threeDCapabilityGateDoc, /Milestone 9 does not authorize/);
assert.match(threeDCapabilityGateDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(threeDCapabilityGateDoc, /#23 remains the memory\/governance anchor/);
assert.match(threeDCapabilityGateDoc, /Completion status after #608/);
assert.match(threeDCapabilityGateDoc, /3D Capability Gate v1 is recorded complete after issues #596-#608 closed/);
assert.match(threeDCapabilityGateDoc, /roadmap\/#1 governance refresh/);
assert.match(threeDCapabilityGateDoc, /not a full 3D editor, production 3D renderer,\s+broad 3D compatibility promise/);
assert.match(threeDCapabilityGateDoc, /native export path,\s+plugin runtime, hosted\/cloud\/server\/auth behavior, browser trusted-write path,\s+command bridge, unrestricted source apply/);
assert.match(threeDCapabilityGateDoc, /Gameplay Scripting \/\s+Logic System v1 \(#611-#625\)/);
assert.match(roadmapDoc, /docs\/3d-capability-gate-v1\.md/);
const docsReadme = fs.readFileSync(require.resolve('../../docs/README.md'), 'utf8');
assert.match(docsReadme, /3d-capability-gate-v1\.md/);
assert.match(docsReadme, /3d-scene-graph-v1\.md/);
assert.match(docsReadme, /3d-animation-playback-v1\.md/);
assert.match(docsReadme, /studio-3d-inspection-surface-v1\.md/);
assert.match(docsReadme, /read-only escaped Studio 3D evidence inspection/);
assert.match(docsReadme, /no 3D editor, trusted write, command bridge, viewport persistence,\s+production 3D, or Godot replacement claim/);
const gameplayLogicScopeDoc = fs.readFileSync(require.resolve('../../docs/gameplay-scripting-logic-system-v1.md'), 'utf8');
assert.match(gameplayLogicScopeDoc, /Issue: #611/);
assert.match(gameplayLogicScopeDoc, /canonical scope contract for #1 Milestone 10/);
assert.match(gameplayLogicScopeDoc, /structured gameplay behavior/);
assert.match(gameplayLogicScopeDoc, /not arbitrary\s+executable scripting/);
assert.match(gameplayLogicScopeDoc, /Gameplay Behavior Model v1/);
assert.match(gameplayLogicScopeDoc, /Event and Signal System v1/);
assert.match(gameplayLogicScopeDoc, /State Machine and Ability Action Model v1/);
assert.match(gameplayLogicScopeDoc, /Script Module Interface Design Gate v1/);
assert.match(gameplayLogicScopeDoc, /Safe Script Sandbox and Trust Boundary v1/);
assert.match(gameplayLogicScopeDoc, /Review-Gated Behavior Apply v1/);
assert.match(gameplayLogicScopeDoc, /Scenario Coverage v9/);
assert.match(gameplayLogicScopeDoc, /no `eval`, dynamic import, plugin loader, command\s+bridge, local server bridge, hidden command execution, browser trusted write/);
assert.match(gameplayLogicScopeDoc, /Rust\/local code owns trusted validation, persistence, behavior draft\/apply\s+validation/);
assert.match(gameplayLogicScopeDoc, /Existing scene, component, trigger, runtime, scenario, evidence, journal,\s+dashboard, and cockpit models should be extended before adding parallel systems/);
assert.match(gameplayLogicScopeDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(gameplayLogicScopeDoc, /#23 remains the memory\/governance anchor/);
assert.doesNotMatch(gameplayLogicScopeDoc, /arbitrary script execution is authorized|eval is allowed|dynamic import is allowed|plugin loader is implemented|command bridge enabled|trusted browser write enabled|production-stable scripting API is implemented/);
const studioBehaviorInspectionDoc = fs.readFileSync(require.resolve('../../docs/studio-behavior-inspection-surface-v1.md'), 'utf8');
assert.match(studioBehaviorInspectionDoc, /Issue: #622/);
assert.match(studioBehaviorInspectionDoc, /escaped read-only composition/);
assert.match(studioBehaviorInspectionDoc, /behavior list rows/);
assert.match(studioBehaviorInspectionDoc, /event\/signal rows/);
assert.match(studioBehaviorInspectionDoc, /state-machine rows/);
assert.match(studioBehaviorInspectionDoc, /ability\/action rows/);
assert.match(studioBehaviorInspectionDoc, /review\/apply status rows/);
assert.match(studioBehaviorInspectionDoc, /Rust\/local code remains the trusted authority/);
assert.match(studioBehaviorInspectionDoc, /must not execute CLI\s+commands, mutate source files, write trusted project state/);
assert.match(studioBehaviorInspectionDoc, /Generated behavior drafts, review\/apply records, runs, dashboard exports/);
assert.match(studioBehaviorInspectionDoc, /#622, #1, and #23 are still open/);
assert.match(studioBehaviorInspectionDoc, /no `eval`, dynamic import, plugin loader, command bridge/);
assert.doesNotMatch(studioBehaviorInspectionDoc, /arbitrary script execution is authorized|eval is allowed|dynamic import is allowed|plugin loader is implemented|command bridge enabled|trusted browser write enabled|auto-apply enabled|self-approval enabled|production-stable scripting API is implemented|production editor is ready|Godot replacement is implemented/);
const cockpitReadmeBehaviorBoundary = fs.readFileSync(require.resolve('./README.md'), 'utf8');
assert.match(cockpitReadmeBehaviorBoundary, /Studio Behavior Inspection Surface v1/);
assert.match(cockpitReadmeBehaviorBoundary, /escaped read-only panels for behavior rows, event\/signal queue rows/);
assert.match(cockpitReadmeBehaviorBoundary, /does not execute scripts, `eval`, dynamic imports, plugin loaders/);
assert.match(cockpitReadmeBehaviorBoundary, /Rust\/local validation remains the trusted authority for behavior draft\/apply/);
assert.match(cockpitReadmeBehaviorBoundary, /studio-behavior-inspection-surface-v1\.md/);
assert.doesNotMatch(cockpitReadmeBehaviorBoundary, /browser command bridge enabled|auto-apply enabled|self-approval enabled|production editor behavior is implemented|Godot replacement behavior is implemented/);
assert.match(docsReadme, /gameplay-scripting-logic-system-v1\.md/);
assert.match(docsReadme, /no arbitrary script execution, plugin loader, command bridge, or production-stable scripting API claim/);
assert.match(docsReadme, /gameplay-scripting-logic-system-governance-handoff\.md/);
const gameplayLogicGovernanceHandoff = fs.readFileSync(require.resolve('../../docs/gameplay-scripting-logic-system-governance-handoff.md'), 'utf8');
assert.match(gameplayLogicGovernanceHandoff, /#611-#625/);
assert.match(gameplayLogicGovernanceHandoff, /GDD-to-Playable Prototype v1/);
assert.match(gameplayLogicGovernanceHandoff, /#611-#625 do not authorize arbitrary executable scripts/);
assert.match(gameplayLogicGovernanceHandoff, /#1 remains open/);
assert.match(gameplayLogicGovernanceHandoff, /#23 remains open/);
assert.doesNotMatch(gameplayLogicGovernanceHandoff, /arbitrary script execution is authorized|command bridge enabled|trusted browser write enabled|production-stable scripting API is implemented|Godot replacement is implemented/);
assert.match(docsReadme, /studio-behavior-inspection-surface-v1\.md/);
assert.match(docsReadme, /escaped read-only Studio behavior\/event\/state\/ability\/draft\/review-apply inspection/);
assert.match(docsReadme, /no arbitrary script execution, command bridge, browser trusted writes, auto-apply,\s+self-approval, plugin runtime, or production-stable scripting API claim/);
const gameplayBehaviorModelDoc = fs.readFileSync(require.resolve('../../docs/gameplay-behavior-model-v1.md'), 'utf8');
assert.match(gameplayBehaviorModelDoc, /Issue: #612/);
assert.match(gameplayBehaviorModelDoc, /data-first contract/);
assert.match(gameplayBehaviorModelDoc, /behavior ids, target entities\/components, triggers,\s+conditions, actions, variables, cooldowns\/timers, evidence links, and blocked\s+reasons/);
assert.match(gameplayBehaviorModelDoc, /patrol, collect item, damage on contact, door opens\s+on flag, win condition, timed hazard, and simple ability trigger/);
assert.match(gameplayBehaviorModelDoc, /must not contain executable script bodies, `eval`, dynamic\s+imports, plugin loader instructions, command strings/);
assert.match(gameplayBehaviorModelDoc, /GL10\.2\.2 should validate duplicate ids, unsafe targets, unsupported actions/);
assert.match(gameplayBehaviorModelDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(gameplayBehaviorModelDoc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(gameplayBehaviorModelDoc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
const gameplayBehaviorFixture = JSON.parse(fs.readFileSync(require.resolve('../../examples/gameplay-behavior-model-v1/behavior-model.valid.fixture.json'), 'utf8'));
assert.equal(gameplayBehaviorFixture.schemaVersion, 'gameplay-behavior-model.v1');
assert.deepEqual(gameplayBehaviorFixture.behaviors.map((behavior) => behavior.id), [
  'patrol-guard-route-a',
  'collect-keycard',
  'spike-damage-contact',
  'open-blue-door',
  'win-after-exit',
  'timed-hazard-pulse',
  'dash-ability-trigger'
]);
assert.doesNotMatch(JSON.stringify(gameplayBehaviorFixture), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
assert.match(docsReadme, /gameplay-behavior-model-v1\.md/);
assert.match(docsReadme, /no executable scripts, plugin loaders, or command bridges/);
assert.match(gameplayBehaviorModelDoc, /read-model\/export compatibility notes/);
assert.match(gameplayBehaviorModelDoc, /gameplay-behavior-model-read-model\.v1/);
assert.match(gameplayBehaviorModelDoc, /no runtime execution, no\s+script execution, no command bridge, no browser trusted writes, no source\s+apply/);
const gameplayEventSignalDoc = fs.readFileSync(require.resolve('../../docs/gameplay-event-signal-system-v1.md'), 'utf8');
const gameplayEventSignalFixture = JSON.parse(fs.readFileSync(require.resolve('../../examples/gameplay-event-signal-system-v1/event-signal.valid.fixture.json'), 'utf8'));
assert.equal(gameplayEventSignalFixture.schemaVersion, 'gameplay-event-signal-system.v1');
assert.deepEqual(gameplayEventSignalFixture.events.map((event) => event.eventType), [
  'collision_contact',
  'item_collected',
  'flag_changed',
  'timer_elapsed',
  'input_action',
  'state_changed'
]);
assert.doesNotMatch(JSON.stringify(gameplayEventSignalFixture), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
assert.match(docsReadme, /gameplay-event-signal-system-v1\.md/);
assert.match(docsReadme, /deterministic event\/signal artifact schema/);
assert.match(gameplayEventSignalDoc, /Issue: #613/);
assert.match(gameplayEventSignalDoc, /no executable expressions, script bodies, command\s+text/);
assert.match(gameplayEventSignalDoc, /GL10\.3\.1 defines the model and validation fixtures only/);
assert.match(gameplayEventSignalDoc, /gameplay-event-signal-queue-summary\.v1/);
assert.match(gameplayEventSignalDoc, /gameplay-event-signal-read-model\.v1/);
assert.match(gameplayEventSignalDoc, /does not emit runtime events, dispatch signals, apply\s+behavior, mutate source files, or create browser write authority/);
assert.match(gameplayEventSignalDoc, /read-only, no runtime execution, no\s+script execution, no command bridge, no browser trusted writes, no source\s+apply/);
assert.match(gameplayEventSignalDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(gameplayEventSignalDoc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(gameplayEventSignalDoc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
const gameplayStateMachineDoc = fs.readFileSync(require.resolve('../../docs/gameplay-state-machine-v1.md'), 'utf8');
const gameplayStateMachineFixture = JSON.parse(fs.readFileSync(require.resolve('../../examples/gameplay-state-machine-v1/state-machine.valid.fixture.json'), 'utf8'));
assert.equal(gameplayStateMachineFixture.schemaVersion, 'gameplay-state-machine.v1');
assert.deepEqual(gameplayStateMachineFixture.stateMachines.map((machine) => machine.id), [
  'player-dash-state',
  'guard-alert-state',
  'door-lock-state'
]);
assert.doesNotMatch(JSON.stringify(gameplayStateMachineFixture), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
assert.match(docsReadme, /gameplay-state-machine-v1\.md/);
assert.match(docsReadme, /structured state-machine artifact schema/);
assert.match(gameplayStateMachineDoc, /Issue: #614/);
assert.match(gameplayStateMachineDoc, /GL10\.4\.1 defines the state-machine model and validation fixtures only/);
assert.match(gameplayStateMachineDoc, /does not authorize arbitrary JS\/Rust\/Python\/Lua\/WASM\s+execution/);
assert.match(gameplayStateMachineDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(gameplayStateMachineDoc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(gameplayStateMachineDoc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
const gameplayAbilityActionDoc = fs.readFileSync(require.resolve('../../docs/gameplay-ability-action-v1.md'), 'utf8');
const gameplayAbilityActionFixture = JSON.parse(fs.readFileSync(require.resolve('../../examples/gameplay-ability-action-v1/ability-action.valid.fixture.json'), 'utf8'));
assert.equal(gameplayAbilityActionFixture.schemaVersion, 'gameplay-ability-action.v1');
assert.deepEqual(gameplayAbilityActionFixture.abilities.map((ability) => ability.actionId), [
  'action-dash',
  'action-alert-strike',
  'action-open-blue-door',
  'action-hazard-pulse',
  'action-complete-win-state'
]);
assert.doesNotMatch(JSON.stringify(gameplayAbilityActionFixture), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
assert.match(docsReadme, /gameplay-ability-action-v1\.md/);
assert.match(docsReadme, /structured ability\/action artifact schema/);
assert.match(gameplayAbilityActionDoc, /Issue: #614/);
assert.match(gameplayAbilityActionDoc, /GL10\.4\.2 defines\s+the ability\/action model and validation fixtures only/);
assert.match(gameplayAbilityActionDoc, /does not authorize arbitrary JS\/Rust\/Python\/Lua\/WASM\s+execution/);
assert.match(gameplayAbilityActionDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(gameplayAbilityActionDoc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(gameplayAbilityActionDoc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
const gameplayStateAbilityEvidenceDoc = fs.readFileSync(require.resolve('../../docs/gameplay-state-ability-evidence-compatibility-v1.md'), 'utf8');
const gameplayStateAbilityEvidenceFixture = JSON.parse(fs.readFileSync(require.resolve('../../examples/gameplay-state-ability-evidence-v1/read-model-compatibility.fixture.json'), 'utf8'));
assert.equal(gameplayStateAbilityEvidenceFixture.schemaVersion, 'gameplay-state-ability-evidence-compatibility.v1');
assert.equal(gameplayStateAbilityEvidenceFixture.stateReadModel.schemaVersion, 'gameplay-state-machine-read-model.v1');
assert.equal(gameplayStateAbilityEvidenceFixture.abilityReadModel.schemaVersion, 'gameplay-ability-action-read-model.v1');
assert.doesNotMatch(JSON.stringify(gameplayStateAbilityEvidenceFixture), /execute_script|plugin_loader|dynamic_import|eval\(|commandBridge|trustedWrite/);
assert.match(docsReadme, /gameplay-state-ability-evidence-compatibility-v1\.md/);
assert.match(docsReadme, /read-only state-machine and ability\/action evidence\/read-model compatibility/);
assert.match(gameplayStateMachineDoc, /gameplay-state-machine-read-model\.v1/);
assert.match(gameplayAbilityActionDoc, /gameplay-ability-action-read-model\.v1/);
assert.match(gameplayStateAbilityEvidenceDoc, /Issue: #614/);
assert.match(gameplayStateAbilityEvidenceDoc, /read-only summaries/);
assert.match(gameplayStateAbilityEvidenceDoc, /gameplay-state-machine-read-model\.v1/);
assert.match(gameplayStateAbilityEvidenceDoc, /gameplay-ability-action-read-model\.v1/);
assert.match(gameplayStateAbilityEvidenceDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(gameplayStateAbilityEvidenceDoc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(gameplayStateAbilityEvidenceDoc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
const scriptModuleDesignDoc = fs.readFileSync(require.resolve('../../docs/script-module-interface-design-gate-v1.md'), 'utf8');
assert.match(docsReadme, /script-module-interface-design-gate-v1\.md/);
assert.match(scriptModuleDesignDoc, /Issue: #615/);
assert.match(scriptModuleDesignDoc, /design-only/);
assert.match(scriptModuleDesignDoc, /read validated world, scene, entity\/component/);
assert.match(scriptModuleDesignDoc, /requiredTests/);
assert.match(scriptModuleDesignDoc, /filesystem reads\/writes/);
assert.match(scriptModuleDesignDoc, /process spawning, shell execution, command execution/);
assert.match(scriptModuleDesignDoc, /network, hosted\/cloud\/server\/auth\/account behavior/);
assert.match(scriptModuleDesignDoc, /secrets, environment variables, credentials/);
assert.match(scriptModuleDesignDoc, /eval.*dynamic import.*plugin loading/s);
assert.match(scriptModuleDesignDoc, /Review, sandbox, and evidence requirements/);
assert.match(scriptModuleDesignDoc, /Deterministic execution expectations/);
assert.match(scriptModuleDesignDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(scriptModuleDesignDoc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(scriptModuleDesignDoc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled/);
const safeScriptSandboxDoc = fs.readFileSync(require.resolve('../../docs/safe-script-sandbox-trust-boundary-v1.md'), 'utf8');
assert.match(docsReadme, /safe-script-sandbox-trust-boundary-v1\.md/);
assert.match(safeScriptSandboxDoc, /Issue: #616/);
assert.match(safeScriptSandboxDoc, /design-gate only/);
assert.match(safeScriptSandboxDoc, /Policy-level operation matrix/);
assert.match(safeScriptSandboxDoc, /Deterministic execution expectations/);
assert.match(safeScriptSandboxDoc, /Resource limits and timeout behavior/);
assert.match(safeScriptSandboxDoc, /Input\/output contract/);
assert.match(safeScriptSandboxDoc, /filesystem reads\/writes outside an explicit sandbox\/generated root/);
assert.match(safeScriptSandboxDoc, /network access, local server bridges/);
assert.match(safeScriptSandboxDoc, /process spawning, shell execution, command execution/);
assert.match(safeScriptSandboxDoc, /environment\/secrets\/credentials\/tokens/);
assert.match(safeScriptSandboxDoc, /dependency, CI, workflow, build-script/);
assert.match(safeScriptSandboxDoc, /eval.*dynamic import.*runtime plugin loading/s);
assert.match(safeScriptSandboxDoc, /Future behavior evidence path/);
assert.match(safeScriptSandboxDoc, /#1 remains the roadmap\/final-goal anchor/);
assert.match(safeScriptSandboxDoc, /#23 remains the\s+memory\/governance anchor/);
assert.doesNotMatch(safeScriptSandboxDoc, /script runtime is implemented|eval is allowed|plugin loader is supported|command bridge enabled|trusted browser write enabled|secure sandbox guaranteed/);
const studio3dInspectionDoc = fs.readFileSync(require.resolve('../../docs/studio-3d-inspection-surface-v1.md'), 'utf8');
assert.match(studio3dInspectionDoc, /Issue: #607/);
assert.match(studio3dInspectionDoc, /no-write, no-command, no-3D-editor boundary/);
assert.match(studio3dInspectionDoc, /scene3d_hierarchy/);
assert.match(studio3dInspectionDoc, /scene3d_camera/);
assert.match(studio3dInspectionDoc, /scene3d_render/);
assert.match(studio3dInspectionDoc, /scene3d_collision/);
assert.match(studio3dInspectionDoc, /scene3d_animation/);
assert.match(studio3dInspectionDoc, /scene3d_scenario_verdicts/);
assert.match(studio3dInspectionDoc, /All values must be escaped/);
assert.match(studio3dInspectionDoc, /must not:\s*\n\n- write source/);
assert.match(studio3dInspectionDoc, /persist trusted browser state, viewport manipulation/);
assert.match(studio3dInspectionDoc, /execute commands, open a local server bridge/);
assert.match(studio3dInspectionDoc, /visual-scripting state/);
assert.match(studio3dInspectionDoc, /Generated 3D screenshots, run directories, preview outputs, dashboard exports/);
assert.match(studio3dInspectionDoc, /#1 remains open/);
assert.match(studio3dInspectionDoc, /#23 remains open/);
assert.match(studio3dInspectionDoc, /no dependency, CI, workflow, build-script/);
assert.match(studio3dInspectionDoc, /production\/Godot-replacement/);
assert.doesNotMatch(studio3dInspectionDoc, /is production-ready|is a Godot replacement|command bridge enabled|trusted browser write enabled|auto-merge enabled/);
assert.match(cockpitReadme, /Studio 3D Inspection Surface v1/);
assert.match(cockpitReadme, /scene hierarchy, active\s+camera\/projection, mesh\/material refs, render summaries, collision\/trigger\s+evidence, animation state/);
assert.match(cockpitReadme, /does not write files, execute commands, persist viewport manipulation, become a\s+3D editor, add visual scripting/);
assert.match(cockpitReadme, /docs\/studio-3d-inspection-surface-v1\.md/);
assert.match(cockpitReadme, /Visual Authoring Demo v1 Studio boundary audit/);
assert.match(cockpitReadme, /no production editor, public launch, native export, plugin\s+runtime, hosted service, visual scripting, command bridge, or Godot replacement/);
assert.match(cockpitReadme, /VA1\.10\.3 changes documentation only/);
const visualAuthoringDoc = fs.readFileSync(require.resolve('../../docs/visual-authoring-v1.md'), 'utf8');
assert.match(visualAuthoringDoc, /Visual Authoring Demo v1 display and public wording audit/);
assert.match(visualAuthoringDoc, /read-only dashboard/);
assert.match(visualAuthoringDoc, /pre-release private MVP/);
assert.match(visualAuthoringDoc, /do not add new Studio controls, dashboard\s+write paths, generated tracked artifacts, dependencies, or behavior changes/);
assert.match(visualAuthoringDoc, /Scenario Coverage v5 \/ VA1\.11\.3 coverage matrix/);
assert.match(visualAuthoringDoc, /Studio renders before\/after summaries, operation summaries, collision\/trigger counts/);
assert.match(visualAuthoringDoc, /Studio may render draft ids, operation summaries, blocked reasons, and copyable inert preview command text only/);
assert.match(visualAuthoringDoc, /Known gaps and out-of-scope behavior/);
assert.match(visualAuthoringDoc, /Scenario Coverage v5 is a regression suite, not a product-expansion milestone/);
assert.match(visualAuthoringDoc, /browser-side trusted file writes, uploads, fetch\/import flows, command\s+execution, local server bridges/);
assert.match(visualAuthoringDoc, /Remaining gaps after #353 are therefore roadmap scope, not regressions in this\s+suite/);
assert.match(visualAuthoringDoc, /node examples\/authoring-cockpit\/cockpit\.test\.cjs/);
const dashboardReadme = fs.readFileSync(require.resolve('../evidence-dashboard/README.md'), 'utf8');
assert.match(dashboardReadme, /Visual Authoring Demo v1 dashboard boundary audit/);
assert.match(dashboardReadme, /escaped, read-only data exported by trusted Rust\/local commands/);
assert.match(dashboardReadme, /must not execute copied commands, create review\s+decisions, apply drafts, rerun scenarios, write files, upload\/fetch assets/);
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
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Open risks: &lt;risk&gt;:high:handoff &lt;risk&gt;/);
assert.match(cockpit.renderStudioLoopCockpitSurface(run), /Stale state: &lt;stale&gt;:artifact &lt;stale&gt;:refresh evidence/);
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
const visualComparisonMarkup = cockpit.renderVisualComparisonEvidenceSurface(run);
assert.match(visualComparisonMarkup, /Visual comparison evidence/);
assert.match(visualComparisonMarkup, /qa14_7_collect_goal_visual/);
assert.match(visualComparisonMarkup, /visual_regression_candidate/);
assert.match(visualComparisonMarkup, /must not compute trusted diffs/);
assert.match(cockpit.renderVisualComparisonEvidenceSurface({}), /No visual comparison evidence/);
const levelDesignInspectionMarkup = cockpit.renderStudioLevelDesignInspectionSurface(run);
assert.match(levelDesignInspectionMarkup, /Level design inspection/);
assert.match(levelDesignInspectionMarkup, /studio-level-design-inspection-v1/);
assert.match(levelDesignInspectionMarkup, /Intent and constraints/);
assert.match(levelDesignInspectionMarkup, /Generation plan/);
assert.match(levelDesignInspectionMarkup, /Tile and entity drafts/);
assert.match(levelDesignInspectionMarkup, /Reachability and pathing/);
assert.match(levelDesignInspectionMarkup, /Objective proof/);
assert.match(levelDesignInspectionMarkup, /Difficulty and pacing heuristic/);
assert.match(levelDesignInspectionMarkup, /Visual and semantic diff/);
assert.match(levelDesignInspectionMarkup, /Review and apply status/);
assert.match(levelDesignInspectionMarkup, /Copyable command text only/);
assert.match(levelDesignInspectionMarkup, /no browser trusted writes/);
assert.match(levelDesignInspectionMarkup, /no command bridge/);
assert.match(levelDesignInspectionMarkup, /no auto-apply/);
assert.match(levelDesignInspectionMarkup, /no auto-merge/);
assert.match(levelDesignInspectionMarkup, /no self-approval/);
assert.match(levelDesignInspectionMarkup, /no production editor/);
assert.match(levelDesignInspectionMarkup, /no autonomous full game generation/);
assert.match(levelDesignInspectionMarkup, /&lt;script&gt;alert\(1\)&lt;\/script&gt;/);
assert.doesNotMatch(levelDesignInspectionMarkup, /<script>alert\(1\)<\/script>/);
assert.doesNotMatch(levelDesignInspectionMarkup, /<button|onclick|localStorage|fetch\(/i);
assert.match(cockpit.renderStudioLevelDesignInspectionSurface({}), /No level design inspection read model/);
assert.match(cockpit.renderStudioLevelDesignInspectionSurface({ level_design_inspection: { present: true, panels: '<bad>', malformedReasons: ['<script>bad</script>'] } }), /Malformed level design inspection/);
assert.match(cockpit.renderStudioLevelDesignInspectionSurface({ level_design_inspection: { present: true, panels: '<bad>', malformedReasons: ['<script>bad</script>'] } }), /&lt;script&gt;bad&lt;\/script&gt;/);
assert.equal(cockpit.normalizeStudioLevelDesignInspection({ level_design_inspection: { present: true, panels: '<bad>' } }).status, 'malformed');
assert.ok(cockpit.studioSurfaceSummary(run).some((surface) => surface.id === 'studio-level-design-inspection' && surface.present), 'Studio surface summary should include level design inspection');
assert.match(cockpit.renderEvidencePane(run), /id="studio-level-design-inspection"/);
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

const studioDraftMarkup = cockpit.renderStudioDraftAuthoringSurface(run);
assert.match(studioDraftMarkup, /Studio draft authoring/);
assert.match(studioDraftMarkup, /studio-draft-scene-1/);
assert.match(studioDraftMarkup, /Copyable draft JSON/);
assert.match(studioDraftMarkup, /Bounded draft controls/);
assert.match(studioDraftMarkup, /Scene draft controls/);
assert.match(studioDraftMarkup, /data-draft-control="scene"/);
assert.match(studioDraftMarkup, /disabled/);
assert.match(studioDraftMarkup, /Blocked state: awaiting Rust preview/);
assert.match(studioDraftMarkup, /edit draft-preview/);
assert.match(studioDraftMarkup, /browser cannot write files or execute commands/);
assert.doesNotMatch(studioDraftMarkup, /<button|onclick|localStorage|fetch\(/i);
assert.equal(cockpit.studioDraftAuthoringState(run).drafts.length, 1);
const tilemapControlRun = { studio_draft_authoring: { present: true, drafts: [{ draftId: 'studio-draft-tilemap-1', target: { type: 'tilemap', path: 'assets/tilemaps/level.json', id: 'level' }, proposedOperations: [{ id: 'op-preview-trigger', kind: 'preview', path: 'layers.terrain[4]', tileId: 'coin_trigger', summary: 'Preview trigger tile only' }], validationStatus: 'blocked', blockedReasons: ['bounds preflight required'] }] } };
const tilemapControlsMarkup = cockpit.renderStudioDraftAuthoringSurface(tilemapControlRun);
assert.match(tilemapControlsMarkup, /Tilemap draft controls/);
assert.match(tilemapControlsMarkup, /data-draft-control="tilemap"/);
assert.match(tilemapControlsMarkup, /coin_trigger/);
assert.match(tilemapControlsMarkup, /Blocked state: bounds preflight required/);
assert.doesNotMatch(tilemapControlsMarkup, /<button|onclick|localStorage|fetch\(/i);
assert.equal(cockpit.studioDraftControlModel(tilemapControlRun.studio_draft_authoring.drafts[0]).kind, 'tilemap-draft-controls');
const assetReferenceControlRun = { studio_draft_authoring: { present: true, boundary: 'Temporary asset-reference draft state; browser cannot upload, fetch, write, or execute.', drafts: [{ draftId: 'studio-draft-asset-ref-1', target: { type: 'asset-reference', path: 'assets/manifest.json', id: 'player_sprite' }, proposedOperations: [{ id: 'op-retarget-sprite', kind: 'reference-preview', path: 'assets.player_sprite.sourcePath', assetId: 'player_sprite', summary: 'Preview asset reference retarget only' }], beforeHash: 'fnv1a64-file-v1:asset-before', expectedAfterSummary: 'Asset reference preview is inert until copied to trusted CLI preflight.', linkedEvidence: ['asset-integrity.json'], validationStatus: 'unvalidated', blockedReasons: [] }] } };
const assetReferenceControlsMarkup = cockpit.renderStudioDraftAuthoringSurface(assetReferenceControlRun);
assert.match(assetReferenceControlsMarkup, /Asset-reference draft controls/);
assert.match(assetReferenceControlsMarkup, /data-draft-control="asset-reference"/);
assert.match(assetReferenceControlsMarkup, /Copyable draft JSON/);
assert.match(assetReferenceControlsMarkup, /Copyable CLI preview command/);
assert.match(assetReferenceControlsMarkup, /asset-integrity\.json/);
assert.match(assetReferenceControlsMarkup, /does not upload assets, fetch remote assets, write manifests, execute commands, or apply edits/);
assert.doesNotMatch(assetReferenceControlsMarkup, /<button|onclick|localStorage|fetch\(|showOpenFilePicker/i);
assert.equal(cockpit.studioDraftControlModel(assetReferenceControlRun.studio_draft_authoring.drafts[0]).kind, 'asset-reference-draft-controls');
assert.match(cockpit.renderStudioDraftAuthoringSurface({}), /No Studio draft authoring read model/);
const behaviorDraftMarkup = cockpit.renderBehaviorDraftStatusSurface(run);
assert.match(behaviorDraftMarkup, /Behavior draft status/);
assert.match(behaviorDraftMarkup, /draft-jump-boost/);
assert.match(behaviorDraftMarkup, /draft-stale-target/);
assert.match(behaviorDraftMarkup, /stale target hash/);
assert.match(behaviorDraftMarkup, /behavior draft preview/);
assert.match(behaviorDraftMarkup, /browser cannot run CLI commands, apply drafts, write files/);
assert.match(behaviorDraftMarkup, /does not apply trusted files, execute scripts/);
assert.doesNotMatch(behaviorDraftMarkup, /<button|onclick|localStorage|fetch\(/i);
assert.equal(cockpit.behaviorDraftReadModel(run).drafts.length, 2);
assert.ok(cockpit.studioSurfaceSummary(run).some((surface) => surface.id === 'behavior-draft-status' && surface.present), 'Studio surface summary should include behavior draft status');
assert.match(cockpit.renderEvidencePane(run), /id="behavior-draft-status"/);
assert.match(cockpit.renderBehaviorDraftStatusSurface({}), /No behavior draft read model/);
const xssBehaviorDraft = { behavior_drafts: { present: true, boundary: '<script>boundary</script>', records: [{ draftId: '<img src=x onerror=alert(1)>', draftPath: '<script>draft</script>', validationStatus: '<script>status</script>', target: { projectId: '<script>project</script>', scenePath: '<b>scene</b>', sceneHash: '<i>hash</i>' }, targetCheck: { stale: true, expectedHash: '<script>expected</script>', actualHash: '<script>actual</script>' }, blockedReasons: ['<script>blocked</script>'], diagnostics: [{ message: '<script>diagnostic</script>' }], guardrail: '<script>guardrail</script>' }] } };
const xssBehaviorDraftMarkup = cockpit.renderBehaviorDraftStatusSurface(xssBehaviorDraft);
assert.ok(!xssBehaviorDraftMarkup.includes('<script>diagnostic</script>'), 'behavior draft diagnostics must be escaped');
assert.ok(!xssBehaviorDraftMarkup.includes('<img src=x onerror=alert(1)>'), 'behavior draft ids must be escaped');
assert.match(xssBehaviorDraftMarkup, /&lt;script&gt;diagnostic&lt;\/script&gt;/);
assert.doesNotMatch(xssBehaviorDraftMarkup, /<button|onclick|localStorage|fetch\(/i);
const xssStudioDraft = { studio_draft_authoring: { present: true, boundary: '<script>boundary</script>', drafts: [{ draftId: '<img src=x onerror=alert(1)>', target: { type: '<script>scene</script>', path: '<b>path</b>' }, proposedOperations: [{ id: '<script>op</script>', kind: '<b>update</b>', path: '<i>path</i>', summary: '<script>summary</script>' }], expectedAfterSummary: '<script>after</script>', validationStatus: '<script>status</script>', blockedReasons: ['<script>blocked</script>'] }] } };
assert.ok(!cockpit.renderStudioDraftAuthoringSurface(xssStudioDraft).includes('<script>summary</script>'), 'studio draft summaries must be escaped');
assert.ok(!cockpit.renderStudioDraftAuthoringSurface(xssStudioDraft).includes('<img src=x onerror=alert(1)>'), 'studio draft ids must be escaped');
const sourceApplyContextXss = cockpit.renderEvidencePane({ source_apply_worktree_context: { present: true, status: '<script>blocked</script>', boundary: '<script>boundary</script>', reports: [{ policyId: '<img src=x onerror=alert(1)>', status: '<script>bad</script>', branch: '<script>branch</script>', headCommit: '<script>head</script>', lockStatus: { active: true, attemptId: '<script>lock</script>' }, blockedReasons: ['<script>blocked</script>'], targets: [{ path: '<img src=x onerror=alert(1)>', gitStatus: '<script>dirty</script>', rootZone: '<script>root</script>', fileClassDecision: '<script>decision</script>', blockedReasons: ['<script>target</script>'] }] }] } });
assert.ok(!sourceApplyContextXss.includes('<script>blocked</script>'), 'source apply context text must be escaped');
assert.ok(!sourceApplyContextXss.includes('<img src=x onerror=alert(1)>'), 'source apply target path must be escaped');
assert.match(sourceApplyContextXss, /&lt;script&gt;blocked&lt;\/script&gt;/);
// Attribute-boundary XSS evidence: untrusted draft fields rendered into HTML
// attribute sinks (value="...", data-draft-field="...") must escape quotes so a
// crafted field cannot break out of the attribute and inject an event handler.
const draftAttrBreakout = '" onmouseover="alert(1)';
const draftTagBreakout = '"><img src=x onerror=alert(1)>';
// Codex #836 (P2): a bare `!markup.includes('onmouseover="')` check is a weak proxy.
// A regression that escapes the attribute-closing quote but leaves the handler
// delimiter encoded (e.g. `onmouseover=&quot;alert(1)"`) still creates a live
// event-handler attribute in the browser yet would pass that substring check.
// Assert the EXACT escaped payload (so any inconsistent quote/angle-bracket
// escaping fails) and verify every rendered handler token keeps an escaped
// delimiter, i.e. no live `="` attribute survives.
const escapeAttrProbe = (value) => String(value)
  .replace(/&/g, '&amp;')
  .replace(/</g, '&lt;')
  .replace(/>/g, '&gt;')
  .replace(/"/g, '&quot;')
  .replace(/'/g, '&#39;');
const escapedAttrBreakout = escapeAttrProbe(draftAttrBreakout);
const escapedTagBreakout = escapeAttrProbe(draftTagBreakout);
const assertAttributeBoundaryEscaped = (markup, label) => {
  assert.ok(markup.includes(escapedAttrBreakout), `${label} must render the exact escaped attribute-breakout payload (handler delimiter encoded)`);
  assert.ok(markup.includes(escapedTagBreakout), `${label} must render the exact escaped tag-breakout payload`);
  // Belt-and-braces: the live breakout literals (a real `"` delimiter or a real
  // `<tag>`) must never appear — only their fully-escaped forms above may.
  assert.ok(!markup.includes('onmouseover="'), `${label} must not emit a live quoted event-handler attribute`);
  assert.ok(!markup.includes('"><img'), `${label} must escape quote+tag breakout`);
  assert.ok(markup.includes('&quot;'), `${label} must escape double quotes in attribute values`);
};
const xssSceneControlRun = { studio_draft_authoring: { present: true, drafts: [{ draftId: 'studio-draft-scene-attr', target: { type: 'scene', path: 'scenes/main.scene.json', id: 'main' }, proposedOperations: [{ id: 'op-attr', kind: 'update', path: draftAttrBreakout, value: draftTagBreakout, summary: 'attribute boundary probe' }], validationStatus: 'unvalidated', blockedReasons: [] }] } };
const xssSceneControlMarkup = cockpit.renderStudioDraftAuthoringSurface(xssSceneControlRun);
assertAttributeBoundaryEscaped(xssSceneControlMarkup, 'scene draft control');
const xssTilemapControlMarkup = cockpit.renderTilemapDraftControl({ operationId: 'op-attr', layerId: draftAttrBreakout, tileId: draftTagBreakout, validationStatus: 'preview-only', collisionCells: [], triggerCells: [] });
assertAttributeBoundaryEscaped(xssTilemapControlMarkup, 'tilemap draft control');
const xssAssetRefControlRun = { studio_draft_authoring: { present: true, drafts: [{ draftId: 'studio-draft-asset-ref-attr', target: { type: 'asset-reference', path: 'assets/manifest.json', id: 'player_sprite' }, proposedOperations: [{ id: 'op-attr', kind: 'reference-preview', path: draftAttrBreakout, assetId: draftTagBreakout, summary: 'attribute boundary probe' }], validationStatus: 'unvalidated', blockedReasons: [] }] } };
const xssAssetRefControlMarkup = cockpit.renderStudioDraftAuthoringSurface(xssAssetRefControlRun);
assertAttributeBoundaryEscaped(xssAssetRefControlMarkup, 'asset-reference draft control');
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
assert.match(cockpit.renderPluginRegistryBrowserSurface({}), /No plugin registry evidence/);
assert.match(cockpit.renderEvidencePane(run), /Runtime asset loading/);
assert.match(cockpit.renderEvidencePane(run), /Asset preview evidence/);
assert.match(cockpit.renderEvidencePane(run), /Source apply context/);
assert.match(cockpit.renderSourceApplyWorktreeContextSurface(run), /source-apply-worktree-boundary-v1/);
assert.match(cockpit.renderSourceApplyWorktreeContextSurface(run), /dirty-target/);
assert.match(cockpit.renderSourceApplyWorktreeContextSurface(run), /must not apply patches|does not apply patches/);
assert.doesNotMatch(cockpit.renderSourceApplyWorktreeContextSurface(run), /<script>bad<\/script>/);
assert.doesNotMatch(cockpit.renderSourceApplyWorktreeContextSurface(run), /<button/i);
assert.match(cockpit.renderSourceApplyWorktreeContextSurface({}), /No source apply worktree context evidence/);
assert.match(cockpit.renderEvidencePane(run), /Visual diff preview/);
assert.match(cockpit.renderEvidencePane(run), /Tilemap draft previews/);
assert.match(cockpit.renderEvidencePane(run), /Asset inspector/);
assert.match(cockpit.renderEvidencePane(run), /Plugin registry browser/);
assert.match(cockpit.renderPluginRegistryBrowserSurface(run), /plugin-registry-summary/);
assert.match(cockpit.renderPluginRegistryBrowserSurface(run), /pluginRegistrySummaryCard/);
assert.match(cockpit.renderPluginRegistryBrowserSurface(run), /pluginRegistry\.summary/);
assert.match(cockpit.renderPluginRegistryBrowserSurface(run), /no JavaScript execution/);
assert.match(cockpit.renderEvidencePane(run), /Loop cockpit/);

const studioPipelineInspection = {
  schemaVersion: 'studio-multi-agent-pipeline-inspection-read-model-v1',
  status: 'blocked',
  sections: [
    { id: 'task-board', label: 'Task board', status: 'present', itemCount: 8, blockers: [], malformedReasons: [] },
    { id: 'work-package', label: 'Work package', status: 'blocked', itemCount: 1, blockers: ['ownership <review> required'], malformedReasons: [] },
    { id: 'decision-ledger', label: '<script>Decision</script>', status: 'malformed', itemCount: 0, blockers: [], malformedReasons: ['reviewDecisions missing'] },
  ],
  malformedReasons: ['production_task_boards[0] must be object'],
  boundary: 'Read-only Studio multi-agent pipeline inspection model; it does not execute commands, spawn agents, write trusted browser state, bridge to local commands, use cloud orchestration, auto-apply, auto-merge, or self-approve.',
};
run.studio_multi_agent_pipeline_inspection = studioPipelineInspection;
const pipelineInspectionMarkup = cockpit.renderStudioMultiAgentPipelineInspectionSurface(run);
assert.match(pipelineInspectionMarkup, /Studio multi-agent pipeline inspection/);
assert.match(pipelineInspectionMarkup, /ownership &lt;review&gt; required/);
assert.match(pipelineInspectionMarkup, /reviewDecisions missing/);
assert.match(pipelineInspectionMarkup, /production_task_boards\[0\] must be object/);
assert.match(pipelineInspectionMarkup, /cloud orchestration/);
assert.doesNotMatch(pipelineInspectionMarkup, /<script>Decision<\/script>/);
assert.doesNotMatch(pipelineInspectionMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(cockpit.renderEvidencePane(run), /Studio multi-agent pipeline inspection/);
assert.match(cockpit.renderStudioMultiAgentPipelineInspectionSurface({}), /No Studio multi-agent pipeline inspection/);

const productionTaskBoardFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-task-board.fixture.json', 'utf8'));
const blockedProductionTaskBoardFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-task-board.blocked.fixture.json', 'utf8'));
run.production_task_board = productionTaskBoardFixture;
const taskBoardMarkup = cockpit.renderProductionTaskBoardSurface(run);
assert.match(taskBoardMarkup, /Production task board/);
assert.match(taskBoardMarkup, /multi-agent-production-v1/);
assert.match(taskBoardMarkup, /task-board-schema/);
assert.match(taskBoardMarkup, /ready-for-review/);
assert.match(taskBoardMarkup, /Read-only production task board surface/);
assert.doesNotMatch(taskBoardMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const blockedTaskBoardMarkup = cockpit.renderProductionTaskBoardSurface({ production_task_board: blockedProductionTaskBoardFixture });
assert.match(blockedTaskBoardMarkup, /ownership-policy/);
assert.match(blockedTaskBoardMarkup, /Waiting for #668/);
assert.match(cockpit.renderEvidencePane(run), /Production task board/);
assert.match(cockpit.renderProductionTaskBoardSurface({}), /No production task board/);

const ownershipPolicyConflictFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/ownership-policy.conflict.fixture.json', 'utf8'));
const ownershipPolicyEscalationFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/ownership-policy.escalation.fixture.json', 'utf8'));
run.ownership_policy = ownershipPolicyConflictFixture;
const ownershipPolicyMarkup = cockpit.renderOwnershipPolicySurface(run);
assert.match(ownershipPolicyMarkup, /Ownership policy/);
assert.match(ownershipPolicyMarkup, /ownership-policy-conflict/);
assert.match(ownershipPolicyMarkup, /scene-write-a/);
assert.match(ownershipPolicyMarkup, /Conflicts with reviewer write hold/);
assert.match(ownershipPolicyMarkup, /Read-only ownership policy surface/);
assert.doesNotMatch(ownershipPolicyMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const escalationPolicyMarkup = cockpit.renderOwnershipPolicySurface({ ownership_policy: ownershipPolicyEscalationFixture });
assert.match(escalationPolicyMarkup, /independent reviewer and critic approval/);
assert.match(cockpit.renderEvidencePane(run), /Ownership policy/);
assert.match(cockpit.renderOwnershipPolicySurface({}), /No file\/artifact ownership policy/);

const agentRoleModelFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/agent-roles.fixture.json', 'utf8'));
run.agent_role_model = agentRoleModelFixture;
const roleModelMarkup = cockpit.renderAgentRoleModelSurface(run);
assert.match(roleModelMarkup, /Agent role model/);
assert.match(roleModelMarkup, /designer/);
assert.match(roleModelMarkup, /build-release-candidate-agent/);
assert.match(roleModelMarkup, /no-self-review/);
assert.match(roleModelMarkup, /self-approval/);
assert.match(roleModelMarkup, /Read-only role accountability surface/);
assert.doesNotMatch(roleModelMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(cockpit.renderAgentRoleModelSurface({ agent_role_model: { schemaVersion: 'agent-role-model-v1', milestone: '<bad>', roles: '<bad>', separationRequirements: [] } }), /Missing or malformed roles list/);
assert.doesNotMatch(cockpit.renderAgentRoleModelSurface({ agent_role_model: { schemaVersion: 'agent-role-model-v1', milestone: '<script>bad<\/script>', roles: [], separationRequirements: [] } }), /<script>bad<\/script>/);
assert.match(cockpit.renderEvidencePane(run), /Agent role model/);
assert.match(cockpit.renderAgentRoleModelSurface({}), /No agent role model/);

const agentWorkPackageFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/agent-work-package.blocked.fixture.json', 'utf8'));
const malformedAgentWorkPackage = { schemaVersion: 'agent-work-package-read-model-v1', workPackageId: '<script>bad</script>', status: 'malformed', malformedReasons: ['acceptanceCriteria missing'], blockers: ['blocked <unsafe>'] };
run.agent_work_package = agentWorkPackageFixture;
const workPackageMarkup = cockpit.renderAgentWorkPackageSurface(run);
assert.match(workPackageMarkup, /Agent work package/);
assert.match(workPackageMarkup, /work-package-scene-design-blocked/);
assert.match(workPackageMarkup, /ownership evidence must be reviewed/);
assert.match(workPackageMarkup, /Inert verification command text/);
assert.match(workPackageMarkup, /agent-handoff-v2.valid.fixture.json/);
assert.match(workPackageMarkup, /Read-only agent work package surface/);
assert.doesNotMatch(workPackageMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const malformedWorkPackageMarkup = cockpit.renderAgentWorkPackageSurface({ agent_work_package: malformedAgentWorkPackage });
assert.match(malformedWorkPackageMarkup, /Malformed: acceptanceCriteria missing/);
assert.match(malformedWorkPackageMarkup, /&lt;script&gt;bad&lt;\/script&gt;/);
assert.doesNotMatch(malformedWorkPackageMarkup, /<script>bad<\/script>/);
assert.match(cockpit.renderEvidencePane(run), /Agent work package/);
assert.match(cockpit.renderAgentWorkPackageSurface({}), /No agent work package/);

assert.match(cockpit.renderAgentHandoffSurface(run), /Agent handoff/);
assert.match(cockpit.renderAgentHandoffSurface(run), /blocked/);
assert.match(cockpit.renderAgentHandoffSurface(run), /&lt;handoff-loop&gt;/);
assert.match(cockpit.renderAgentHandoffSurface(run), /Allowed command text/);
assert.doesNotMatch(cockpit.renderAgentHandoffSurface(run), /<handoff-loop>/);
assert.doesNotMatch(cockpit.renderAgentHandoffSurface(run), /<button/i);
assert.match(cockpit.renderEvidencePane(run), /Agent handoff/);
assert.match(cockpit.renderAgentHandoffSurface({}), /No agent handoff/);

const reviewCriticGateFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/review-critic-gate.valid.fixture.json', 'utf8'));
run.review_critic_gate = reviewCriticGateFixture;
const reviewGateMarkup = cockpit.renderReviewCriticGateSurface(run);
assert.match(reviewGateMarkup, /Review\/critic gate/);
assert.match(reviewGateMarkup, /demo-review-critic-gate/);
assert.match(reviewGateMarkup, /agent-reviewer-1/);
assert.match(reviewGateMarkup, /agent-critic-1/);
assert.match(reviewGateMarkup, /agent-work-package\.valid\.fixture\.json/);
assert.match(reviewGateMarkup, /agent-handoff-v2\.valid\.fixture\.json/);
assert.match(reviewGateMarkup, /agent-shared-state-snapshot\.fresh\.fixture\.json/);
assert.match(reviewGateMarkup, /qa-worker-assignment\.valid\.fixture\.json/);
assert.match(reviewGateMarkup, /production-evidence-bundle\.complete\.fixture\.json/);
assert.match(reviewGateMarkup, /agent-decision-ledger\.valid\.fixture\.json/);
assert.match(reviewGateMarkup, /does not execute commands/);
assert.doesNotMatch(reviewGateMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const reviewGateXss = cockpit.renderReviewCriticGateSurface({ review_critic_gate: {
  schemaVersion: 'review-critic-gate-read-model-v1',
  gateId: '<script>gate</script>',
  taskId: '<img>',
  decision: '<script>blocked</script>',
  promotionRecommendation: '<script>block</script>',
  reviewerActorId: '<reviewer>',
  criticActorId: '<critic>',
  blockers: ['<script>blocked</script>'],
  evidenceReviewedRefPaths: ['<script>evidence</script>'],
  boundary: '<script>boundary</script>',
} });
assert.match(reviewGateXss, /&lt;script&gt;gate&lt;\/script&gt;/);
assert.match(reviewGateXss, /&lt;script&gt;blocked&lt;\/script&gt;/);
assert.doesNotMatch(reviewGateXss, /<script>gate<\/script>|<script>boundary<\/script>/);
assert.match(cockpit.renderEvidencePane(run), /Review\/critic gate/);
assert.match(cockpit.renderReviewCriticGateSurface({}), /No review\/critic gate/);

const handoffV2Fixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/agent-handoff-v2.blocked.fixture.json', 'utf8'));
const handoffV2Markup = cockpit.renderAgentHandoffSurface({ agent_handoff_v2s: [handoffV2Fixture] });
assert.match(handoffV2Markup, /handoff-v2-blocked/);
assert.match(handoffV2Markup, /task-board-schema/);
assert.match(handoffV2Markup, /Open risks/);
assert.match(handoffV2Markup, /missing-review-risk/);
assert.match(handoffV2Markup, /Acceptance checklist/);
assert.doesNotMatch(handoffV2Markup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /Authoring loop evidence bundle/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /partial/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /&lt;loop-bundle&gt;/);
assert.match(cockpit.renderLoopEvidenceBundleSurface(run), /runs:/);
assert.doesNotMatch(cockpit.renderLoopEvidenceBundleSurface(run), /<loop-bundle>/);
assert.match(cockpit.renderEvidencePane(run), /Authoring loop evidence bundle/);
assert.match(cockpit.renderLoopEvidenceBundleSurface({ summary: { id: 'run-no-bundle' } }), /No loop evidence bundle/);
const demoHandoffFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/demo-handoff-v2.fixture.json', 'utf8'));
const demoEvidenceBundleFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/demo-evidence-bundle.fixture.json', 'utf8'));
const demoPipelineRun = { agent_handoffs: [demoHandoffFixture], loop_evidence_bundles: [demoEvidenceBundleFixture] };
const demoHandoffMarkup = cockpit.renderAgentHandoffSurface(demoPipelineRun);
assert.match(demoHandoffMarkup, /multi-agent-demo-fixture/);
assert.match(demoHandoffMarkup, /ready/);
assert.match(demoHandoffMarkup, /hidden background agents/);
assert.match(demoHandoffMarkup, /#1 remains open/);
assert.match(demoHandoffMarkup, /#23 remains open/);
assert.doesNotMatch(demoHandoffMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const demoBundleMarkup = cockpit.renderLoopEvidenceBundleSurface(demoPipelineRun);
assert.match(demoBundleMarkup, /multi-agent-demo-fixture/);
assert.match(demoBundleMarkup, /completed/);
assert.match(demoBundleMarkup, /runs:1/);
assert.match(demoBundleMarkup, /matrices:1/);
assert.match(demoBundleMarkup, /No missing refs reported/);
assert.match(demoBundleMarkup, /does not move artifacts/);
assert.doesNotMatch(demoBundleMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);

const multiAgentDemoDoc = fs.readFileSync(require.resolve('../../docs/multi-agent-prototype-production-demo-v1.md'), 'utf8');
assert.match(multiAgentDemoDoc, /Multi-Agent Prototype Production Demo v1/);
assert.match(multiAgentDemoDoc, /demo-task-board\.fixture\.json/);
assert.match(multiAgentDemoDoc, /demo-handoff-v2\.fixture\.json/);
assert.match(multiAgentDemoDoc, /demo-evidence-bundle\.fixture\.json/);
assert.match(multiAgentDemoDoc, /cargo test -p ouroforge-core multi_agent_demo_pipeline/);
assert.match(multiAgentDemoDoc, /node examples\/evidence-dashboard\/dashboard\.test\.cjs/);
assert.match(multiAgentDemoDoc, /node examples\/authoring-cockpit\/cockpit\.test\.cjs/);
assert.match(multiAgentDemoDoc, /Cleanup policy/);
assert.match(multiAgentDemoDoc, /Known gaps and out-of-scope behavior/);
assert.match(multiAgentDemoDoc, /hidden background agents or unbounded spawning/);
assert.match(multiAgentDemoDoc, /auto-apply, auto-merge, self-approval/);
assert.match(multiAgentDemoDoc, /browser trusted writes, command bridge/);
assert.match(multiAgentDemoDoc, /remote worker pool/);
assert.match(multiAgentDemoDoc, /current Godot replacement, production-ready/);
assert.match(multiAgentDemoDoc, /Issues #1 and #23 must remain open/);

assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /Source patch evidence bundle/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /bundle-1/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /Review docs patch preview/);

assert.match(cockpit.renderQaSwarmInspectionSurface(run), /QA swarm inspection/);
assert.match(cockpit.renderQaSwarmInspectionSurface(run), /Scenario &lt;candidate&gt; panel/);
assert.match(cockpit.renderQaSwarmInspectionSurface(run), /Worker budget\/assignment panel/);
assert.match(cockpit.renderQaSwarmInspectionSurface(run), /Visual\/performance\/error evidence panel/);
assert.match(cockpit.renderQaSwarmInspectionSurface(run), /evidence\/qa\/&lt;candidate&gt;\.json/);
assert.match(cockpit.renderQaSwarmInspectionSurface(run), /must not spawn &lt;workers&gt;, execute commands/);
assert.doesNotMatch(cockpit.renderQaSwarmInspectionSurface(run), /<script>|<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand|selfApprovalCommand|cloudRunnerCommand/);
assert.match(cockpit.renderQaSwarmInspectionSurface(null), /No QA swarm inspection summary/);
assert.match(cockpit.renderIntegration(run), /QA swarm inspection/);

assert.match(cockpit.renderQaAgentWorkQueueSurface(run), /QA agent work queue/);
assert.match(cockpit.renderQaAgentWorkQueueSurface(run), /&lt;qa-item&gt;/);
assert.match(cockpit.renderQaAgentWorkQueueSurface(run), /review &lt;gate&gt; missing/);
assert.match(cockpit.renderQaAgentWorkQueueSurface(run), /runs\/multi-agent-pipeline\/demo\/qa\/old-scenario-result\.json/);
assert.match(cockpit.renderQaAgentWorkQueueSurface(run), /Inert command text/);
assert.doesNotMatch(cockpit.renderQaAgentWorkQueueSurface(run), /<script>|<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand|selfApprovalCommand/);
assert.match(cockpit.renderQaAgentWorkQueueSurface(null), /No QA agent work queue/);
assert.match(cockpit.renderIntegration(run), /QA agent work queue/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /review-held:1/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /review_held_target/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /manual review required/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /Dry-run: passed/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /cargo fmt --check/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /Review: reviewed/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /Linked evidence: source-patch-preview:mutation\/preview.json/);
assert.match(cockpit.renderSourcePatchEvidenceBundleSurface(run), /apply_patch/);
assert.doesNotMatch(cockpit.renderSourcePatchEvidenceBundleSurface(run), /<button|applyCommand|mergeCommand/);
const sourcePatchBundleXssRun = {
  mutation_artifacts: [{
    id: 'source-patch-evidence-bundle',
    path: 'mutation/source-patch-evidence-bundle.json',
    value: {
      bundleId: '<bundle-xss>',
      patchPreviewId: '<preview-xss>',
      status: 'complete',
      patchSummary: { title: '<script>preview</script>', expectedBehaviorChange: '<img src=x>', targetCount: 1, changedLines: 2 },
      fileClassSummary: { allowed: 0, reviewHeld: 1, blocked: 0, highestRisk: '<risk>' },
      riskIds: ['<risk-id>'],
      linkedEvidence: [{ kind: '<script>kind</script>', path: 'sandbox/<bad>/evidence/report.json' }],
      dryRunSummary: { status: '<passed>', allowlistPolicyId: '<policy>', reportRef: { kind: 'sandbox-dry-run-report', path: 'sandbox/<bad>/evidence/report.json' } },
      requiredTestSummary: { total: 1, commands: ['cargo fmt --check <script>'], allowlistPolicyId: '<policy>' },
      reviewSummary: { status: '<reviewed>', decisionRef: { kind: 'review-decision', path: 'mutation/<review>.json' } },
      blockedReasons: ['<manual review required>'],
      previewRef: { kind: 'source-patch-preview', path: 'mutation/<preview>.json' },
      fileClassReportRef: { kind: 'file-class-report', path: 'evidence/<file-class>.json' },
      diffIntegrityReportRef: { kind: 'diff-integrity-report', path: 'evidence/<diff>.json' },
      sandboxReportRef: { kind: 'sandbox-dry-run-report', path: 'sandbox/<bad>/evidence/report.json' },
      testSummaryRef: { kind: 'test-summary', path: 'sandbox/<bad>/evidence/tests.json' },
      reviewDecisionRef: { kind: 'review-decision', path: 'mutation/<review>.json' },
      forbiddenActionNotices: [{ action: '<apply_patch>', reason: '<forbidden>' }],
      guardrails: ['read-only <guardrail>'],
    },
  }],
};
const sourcePatchBundleXssMarkup = cockpit.renderSourcePatchEvidenceBundleSurface(sourcePatchBundleXssRun);
assert.match(sourcePatchBundleXssMarkup, /&lt;bundle-xss&gt;/);
assert.match(sourcePatchBundleXssMarkup, /sandbox\/&lt;bad&gt;\/evidence\/report\.json/);
assert.doesNotMatch(sourcePatchBundleXssMarkup, /<script>|<img|<button|applyCommand|mergeCommand|browserCommandBridge|executeCommand/);

const behaviorEvidenceRun = {
  behavior_evidence: {
    present: true,
    status: 'ready',
    bundle_count: 1,
    malformed_count: 0,
    lifecycle_ref_count: 8,
    observed_failure_count: 1,
    next_step_hypothesis_count: 1,
    boundary: 'read-only behavior evidence; no command bridge, auto-apply, or trusted writes.',
    bundles: [{
      bundle_id: 'behavior-evidence-jump-boost',
      status: 'complete',
      path: 'evidence/behavior/behavior-evidence-bundle.json',
      lifecycle_ref_count: 8,
      observed_failures: [{ scenario_id: 'jump_boost_cooldown_regression', summary: 'Cooldown regressed', evidence_ref: 'evidence/scenarios/jump/scenario-result.json' }],
      next_step_hypotheses: [{ id: 'hypothesis-rerun-after-rollback', summary: 'Rerun rollback comparison' }],
      blocked_reasons: [],
      evidence_refs: ['evidence/scenarios/jump/scenario-result.json'],
      guardrails: ['read-only rust/local untracked evidence'],
    }],
  },
};
assert.match(cockpit.renderBehaviorEvidenceLifecycleSurface(behaviorEvidenceRun), /Behavior evidence lifecycle/);
assert.match(cockpit.renderBehaviorEvidenceLifecycleSurface(behaviorEvidenceRun), /behavior-evidence-jump-boost/);
assert.match(cockpit.renderBehaviorEvidenceLifecycleSurface(behaviorEvidenceRun), /Cooldown regressed/);
assert.match(cockpit.renderBehaviorEvidenceLifecycleSurface(behaviorEvidenceRun), /hypothesis-rerun-after-rollback/);
assert.match(cockpit.renderBehaviorEvidenceLifecycleSurface(behaviorEvidenceRun), /no command bridge/);
assert.doesNotMatch(cockpit.renderBehaviorEvidenceLifecycleSurface(behaviorEvidenceRun), /<button|applyCommand|mergeCommand|browserCommandBridge/);
const behaviorEvidenceXssSurface = cockpit.renderBehaviorEvidenceLifecycleSurface({
  behavior_evidence: {
    present: true,
    status: '<script>ready</script>',
    boundary: '<script>boundary</script>',
    bundles: [{
      bundle_id: '<img src=x onerror=alert(1)>',
      status: '<script>complete</script>',
      path: 'evidence/<bad>/behavior.json',
      read_error: '<script>malformed</script>',
      observed_failures: [{ scenario_id: '<script>scenario</script>', summary: '<img src=x>', evidence_ref: 'evidence/<bad>/result.json' }],
      next_step_hypotheses: [{ id: '<script>hypothesis</script>', summary: '<img src=x>' }],
      blocked_reasons: ['<script>blocked</script>'],
      evidence_refs: ['evidence/<bad>/result.json'],
      guardrails: ['read-only <guardrail>'],
    }],
  },
});
assert.match(behaviorEvidenceXssSurface, /&lt;img src=x onerror=alert\(1\)&gt;/);
assert.match(behaviorEvidenceXssSurface, /evidence\/&lt;bad&gt;\/result\.json/);
assert.doesNotMatch(behaviorEvidenceXssSurface, /<script>|<img|<button|applyCommand|mergeCommand|browserCommandBridge|executeCommand/);
assert.match(cockpit.renderEvidencePane({ ...run, ...behaviorEvidenceRun }), /Behavior evidence lifecycle/);

const behaviorInspectionRun = {
  behavior_inspection: {
    present: true,
    status: 'ready',
    boundary: 'Read-only behavior inspection; no command bridge, no eval, no dynamic import, no plugin loader, no browser trusted writes, no auto-apply.',
    behaviors: JSON.parse(fs.readFileSync(require.resolve('../gameplay-behavior-model-v1/behavior-model.valid.fixture.json'), 'utf8')),
    event_signals: JSON.parse(fs.readFileSync(require.resolve('../gameplay-event-signal-system-v1/event-signal.valid.fixture.json'), 'utf8')),
    state_machines: JSON.parse(fs.readFileSync(require.resolve('../gameplay-state-machine-v1/state-machine.valid.fixture.json'), 'utf8')),
    ability_actions: JSON.parse(fs.readFileSync(require.resolve('../gameplay-ability-action-v1/ability-action.valid.fixture.json'), 'utf8')),
    review_apply: {
      status: 'ready_for_trusted_rust_apply',
      boundary: 'Review/apply rows are status-only; no auto-apply, self-approval, command execution, or browser trusted writes.',
      reviews: [{ decision_id: 'review-jump-boost', status: 'accepted', reviewer: 'human-reviewer', evidence_refs: ['runs/behavior/review.json'] }],
      applies: [{ transaction_id: 'behavior-apply-jump-boost', status: 'ready_for_trusted_apply', rollback_ref: { path: 'runs/behavior/rollback.json' }, blocked_reasons: [], evidence_refs: ['runs/behavior/apply.json'] }],
    },
  },
};
const behaviorInspectionModel = cockpit.behaviorInspectionModel(behaviorInspectionRun);
assert.equal(behaviorInspectionModel.behaviors.length, 7);
assert.equal(behaviorInspectionModel.events.length, 6);
assert.equal(behaviorInspectionModel.stateMachines.length, 3);
assert.equal(behaviorInspectionModel.abilities.length, 5);
assert.match(cockpit.renderBehaviorListPanel(behaviorInspectionRun), /Behavior list panel/);
assert.match(cockpit.renderBehaviorListPanel(behaviorInspectionRun), /patrol-guard-route-a/);
assert.match(cockpit.renderBehaviorEventSignalPanel(behaviorInspectionRun), /Event\/signal panel/);
assert.match(cockpit.renderBehaviorEventSignalPanel(behaviorInspectionRun), /player-spike-contact/);
assert.match(cockpit.renderBehaviorStateMachinePanel(behaviorInspectionRun), /State machine panel/);
assert.match(cockpit.renderBehaviorStateMachinePanel(behaviorInspectionRun), /player-dash-state/);
assert.match(cockpit.renderBehaviorAbilityActionPanel(behaviorInspectionRun), /Ability\/action panel/);
assert.match(cockpit.renderBehaviorAbilityActionPanel(behaviorInspectionRun), /player-dash/);
assert.match(cockpit.renderBehaviorReviewApplyStatusSurface(behaviorInspectionRun), /Review\/apply status panel/);
assert.match(cockpit.renderBehaviorReviewApplyStatusSurface(behaviorInspectionRun), /review-jump-boost/);
assert.match(cockpit.renderBehaviorReviewApplyStatusSurface(behaviorInspectionRun), /behavior-apply-jump-boost/);
assert.ok(cockpit.studioSurfaceSummary({ ...run, ...behaviorInspectionRun }).some((surface) => surface.id === 'behavior-event-signal-panel' && surface.present), 'Studio surface summary should include behavior event/signal panel');
assert.match(cockpit.renderEvidencePane({ ...run, ...behaviorInspectionRun }), /Behavior list panel/);
assert.match(cockpit.renderEvidencePane({ ...run, ...behaviorInspectionRun }), /Review\/apply status panel/);
const behaviorInspectionXssRun = {
  behavior_inspection: {
    present: true,
    status: '<script>ready</script>',
    boundary: '<script>boundary</script>',
    behaviors: { status: '<img src=x>', behaviors: [{ id: '<img src=x onerror=alert(1)>', status: '<script>blocked</script>', label: '<script>label</script>', target: { entityId: '<b>player</b>' }, trigger: { kind: '<script>trigger</script>' }, conditions: [{ kind: '<img src=x>' }], actions: [{ kind: '<script>action</script>' }], blockedReasons: ['<script>blocked</script>'], evidenceRefs: ['evidence/<bad>/behavior.json'] }] },
    event_signals: { events: [{ id: '<script>event</script>', eventType: '<img src=x>', signalName: '<script>signal</script>', source: { entityId: '<b>source</b>' }, target: { entityId: '<i>target</i>' }, tick: 1, orderingIndex: 0, consumed: false, blockedReason: '<script>blocked</script>', consumedBy: ['<script>consumer</script>'], evidenceRefs: ['evidence/<bad>/event.json'] }] },
    state_machines: { stateMachines: [{ id: '<script>machine</script>', label: '<img src=x>', status: '<script>status</script>', target: { entityId: '<b>entity</b>' }, initialStateId: '<script>state</script>', states: [{ id: '<script>idle</script>' }], transitions: [{ id: '<script>transition</script>', from: '<b>a</b>', to: '<i>b</i>', trigger: { kind: '<script>trigger</script>' } }], evidenceRefs: ['evidence/<bad>/state.json'] }] },
    ability_actions: { abilities: [{ id: '<script>ability</script>', label: '<img src=x>', runtimeStatus: '<script>runtime</script>', target: { entityId: '<b>player</b>' }, trigger: { kind: '<script>input</script>' }, effect: { kind: '<script>effect</script>' }, costs: [{ kind: '<script>cost</script>' }], evidenceRefs: ['evidence/<bad>/ability.json'] }] },
    review_apply: { reviews: [{ decision_id: '<script>review</script>', status: '<img src=x>', reviewer: '<script>reviewer</script>', evidence_refs: ['evidence/<bad>/review.json'] }], applies: [{ transaction_id: '<script>apply</script>', status: '<img src=x>', rollback_ref: { path: 'rollback/<bad>.json' }, blocked_reasons: ['<script>blocked</script>'], evidence_refs: ['evidence/<bad>/apply.json'] }] },
  },
};
const behaviorInspectionXssMarkup = [
  cockpit.renderBehaviorListPanel(behaviorInspectionXssRun),
  cockpit.renderBehaviorEventSignalPanel(behaviorInspectionXssRun),
  cockpit.renderBehaviorStateMachinePanel(behaviorInspectionXssRun),
  cockpit.renderBehaviorAbilityActionPanel(behaviorInspectionXssRun),
  cockpit.renderBehaviorReviewApplyStatusSurface(behaviorInspectionXssRun),
].join('\n');
assert.match(behaviorInspectionXssMarkup, /&lt;img src=x onerror=alert\(1\)&gt;/);
assert.match(behaviorInspectionXssMarkup, /evidence\/&lt;bad&gt;\/apply\.json/);
assert.doesNotMatch(behaviorInspectionXssMarkup, /<script>|<img|<button|onclick|localStorage|fetch\(|applyCommand|mergeCommand|browserCommandBridge|executeCommand/);

assert.match(cockpit.renderSourcePatchApplyTransactionSurface(run), /Source patch apply transaction/);
assert.match(cockpit.renderSourcePatchApplyTransactionSurface(run), /apply-tx-1/);
assert.match(cockpit.renderSourcePatchApplyTransactionSurface(run), /ready_metadata_only_no_apply_authority/);
assert.match(cockpit.renderSourcePatchApplyTransactionSurface(run), /apply_patch/);
assert.doesNotMatch(cockpit.renderSourcePatchApplyTransactionSurface(run), /<button|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(cockpit.renderSourcePatchStaleTargetGuardSurface(run), /Source patch stale target guard/);
assert.match(cockpit.renderSourcePatchStaleTargetGuardSurface(run), /stale-guard-1/);
assert.match(cockpit.renderSourcePatchStaleTargetGuardSurface(run), /fresh_guard_metadata_only_no_apply_authority/);
assert.match(cockpit.renderSourcePatchStaleTargetGuardSurface(run), /apply_patch/);
assert.doesNotMatch(cockpit.renderSourcePatchStaleTargetGuardSurface(run), /<button|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(cockpit.renderEvidencePane(run), /Source patch evidence bundle/);

const demoDisplayAudit = JSON.parse(fs.readFileSync('examples/source-mutation-preview-demo-v1/display-audit.sample.json', 'utf8'));
const demoDisplayAuditDoc = fs.readFileSync('docs/source-mutation-preview-demo-v1-audit.md', 'utf8');
assert.equal(demoDisplayAudit.prUnitId, 'SMP1.10.3');
assert.ok(demoDisplayAudit.allowedDisplay.includes('sandbox dry-run status'));
assert.ok(demoDisplayAudit.forbiddenControls.includes('merge_branch'));
assert.ok(demoDisplayAudit.wordingGuardrails.includes('read-only display'));
assert.match(demoDisplayAuditDoc, /Studio and dashboard surfaces must not provide/);
assert.match(demoDisplayAuditDoc, /#1 and #23 remain open/);
assert.doesNotMatch(demoDisplayAuditDoc, /can apply patches|can merge branches|trusted file write control/i);

const evidenceTimelineFixture = JSON.parse(fs.readFileSync('examples/studio-evidence-timeline-v1/timeline.fixture.json', 'utf8'));
const evidenceTimeline = cockpit.buildEvidenceTimelineModel(evidenceTimelineFixture.runs);
assert.equal(evidenceTimeline.schemaVersion, 'studio-evidence-timeline-model-v1');
assert.equal(evidenceTimeline.status, 'diagnostics');
assert.deepEqual(evidenceTimeline.entries.map((entry) => entry.runId), ['run-before', 'run-after']);
assert.equal(evidenceTimeline.entries[0].evidence.screenshots.count, 1);
assert.equal(evidenceTimeline.entries[0].mutationLinks.length, 2);
assert.equal(evidenceTimeline.entries[0].sourceApplyLinks.length, 1);
assert.equal(evidenceTimeline.entries[1].sourceApplyLinks.length, 2);
assert.deepEqual(evidenceTimeline.comparisonCandidates, [{ beforeRunId: 'run-before', afterRunId: 'run-after', comparisonRefs: ['comparisons/run-before--run-after.json'] }]);
assert.equal(evidenceTimeline.comparisonView.length, 1);
assert.equal(evidenceTimeline.comparisonView[0].beforeRunId, 'run-before');
assert.equal(evidenceTimeline.comparisonView[0].afterRunId, 'run-after');
assert.equal(evidenceTimeline.comparisonView[0].screenshots.before[0].path, 'evidence/before.png');
assert.equal(evidenceTimeline.comparisonView[0].screenshots.after[0].exists, false);
assert.equal(evidenceTimeline.comparisonView[0].worldState.changed, true);
assert.match(evidenceTimeline.comparisonView[0].worldState.before[0].valuePreview, /\"x\":10/);
assert.match(evidenceTimeline.comparisonView[0].worldState.after[0].valuePreview, /\"x\":42/);
assert.match(evidenceTimeline.comparisonView[0].guardrail, /display-only/);
assert.ok(evidenceTimeline.diagnostics.some((diagnostic) => diagnostic.kind === 'missing-evidence' && diagnostic.artifactId === 'after-shot'));
assert.ok(evidenceTimeline.diagnostics.some((diagnostic) => diagnostic.kind === 'broken-evidence' && diagnostic.artifactId === 'after-frame'));
assert.equal(evidenceTimeline.diagnosticSummary.status, 'attention_required');
assert.equal(evidenceTimeline.diagnosticSummary.total, 2);
assert.deepEqual(evidenceTimeline.diagnosticSummary.byKind, { 'missing-evidence': 1, 'broken-evidence': 1 });
assert.deepEqual(evidenceTimeline.diagnosticSummary.affectedRuns, ['run-after']);
assert.ok(evidenceTimeline.diagnosticSummary.reviewerActions.includes('keep Studio browser surface read-only'));
assert.ok(evidenceTimeline.diagnosticSummary.forbiddenActions.includes('browser_write_evidence'));
assert.ok(evidenceTimeline.guardrails.join(' ').includes('read-only'));
assert.ok(evidenceTimeline.forbiddenActions.includes('execute_command'));
assert.ok(evidenceTimeline.forbiddenActions.includes('write_trusted_file'));

const evidenceTimelineMarkup = cockpit.renderEvidenceTimelineSurface(evidenceTimeline);
assert.match(evidenceTimelineMarkup, /Evidence timeline/);
assert.match(evidenceTimelineMarkup, /run-before/);
assert.match(evidenceTimelineMarkup, /run-after/);
assert.match(evidenceTimelineMarkup, /Source-apply links/);
assert.match(evidenceTimelineMarkup, /source-patch-apply-transaction/);
assert.match(evidenceTimelineMarkup, /Before\/after comparison candidates/);
assert.match(evidenceTimelineMarkup, /Evidence diagnostics/);
assert.match(evidenceTimelineMarkup, /2 missing\/broken fixture evidence diagnostic/);
assert.match(evidenceTimelineMarkup, /missing-evidence: 1/);
assert.match(evidenceTimelineMarkup, /broken-evidence: 1/);
assert.match(evidenceTimelineMarkup, /Restore or regenerate|restore or regenerate/);
assert.match(evidenceTimelineMarkup, /Before\/after evidence comparison/);
assert.match(evidenceTimelineMarkup, /before screenshots/);
assert.match(evidenceTimelineMarkup, /after world-state/);
assert.match(evidenceTimelineMarkup, /World-state changed: true/);
assert.match(evidenceTimelineMarkup, /\{&quot;player&quot;:\{&quot;x&quot;:42\}\}/);
assert.match(evidenceTimelineMarkup, /comparisons\/run-before--run-after\.json/);
assert.match(evidenceTimelineMarkup, /missing-evidence/);
assert.match(evidenceTimelineMarkup, /broken-evidence/);
assert.doesNotMatch(evidenceTimelineMarkup, /<button|applyCommand|mergeCommand|executeCommand|browserCommandBridge|publishCommand|deployCommand/);
assert.match(cockpit.renderEvidenceDiagnosticsSurface(evidenceTimeline), /Evidence diagnostics/);
assert.match(cockpit.renderEvidenceDiagnosticsSurface({ diagnostics: [] }), /No missing or broken fixture evidence/);
assert.match(cockpit.renderEvidenceComparisonView(evidenceTimeline), /Read-only fixture comparison view/);
assert.match(cockpit.renderEvidenceComparisonView({ comparisonView: [] }), /No fixture before\/after comparison/);
assert.match(cockpit.renderEvidencePane(evidenceTimelineFixture.runs[1]), /Evidence timeline/);
assert.match(cockpit.renderEvidenceTimelineSurface([]), /No run evidence/);

const productionBundleComplete = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.complete.fixture.json', 'utf8'));
const productionBundlePartial = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.partial.fixture.json', 'utf8'));
const productionBundleConflict = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.unresolved-conflict.fixture.json', 'utf8'));
const productionBundleMissingReview = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.missing-review.fixture.json', 'utf8'));
const productionBundleSurface = cockpit.renderProductionEvidenceBundleSurface({ productionEvidenceBundles: [productionBundleComplete, productionBundlePartial, productionBundleConflict, productionBundleMissingReview] });
assert.match(productionBundleSurface, /Production evidence bundle/);
assert.match(productionBundleSurface, /demo-production-evidence-bundle/);
assert.match(productionBundleSurface, /demo-production-evidence-bundle-partial/);
assert.match(productionBundleSurface, /Missing refs: qaResultRefs\[0\] awaits fixture refresh/);
assert.match(productionBundleSurface, /ownership-conflict-demo:Two work packages claim the same generated output root/);
assert.match(productionBundleSurface, /missing-review-demo:reviewer/);
assert.match(productionBundleSurface, /task-board:present/);
assert.match(productionBundleSurface, /Generated roots: runs\/multi-agent-pipeline/);
assert.match(productionBundleSurface, /hidden background agents/);
assert.match(productionBundleSurface, /does not spawn agents/);
assert.match(productionBundleSurface, /does not spawn agents, execute commands, apply changes, auto-merge, self-approve, or write trusted state/);
assert.doesNotMatch(productionBundleSurface, /<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand|selfApprovalCommand/);
assert.match(cockpit.renderProductionEvidenceBundleSurface(null), /No production evidence bundle is attached/);
const productionBundleXssSurface = cockpit.renderProductionEvidenceBundleSurface({ productionEvidenceBundle: {
  bundleId: '<bundle-xss>',
  milestone: '<script>milestone</script>',
  status: 'blocked',
  taskBoardRef: { id: '<task>', path: 'runs/<task>.json' },
  laneOutputs: [{ lane: '<script>lane</script>', status: '<blocked>', blockedReasons: ['<img src=x>'] }],
  missingRefs: ['<missing-ref>'],
  staleRefs: ['<stale-ref>'],
  blockedReasons: ['<blocked-reason>'],
  malformedReasons: ['<malformed-reason>'],
  unresolvedConflicts: [{ id: '<conflict>', summary: '<summary>' }],
  missingReviews: [{ id: '<review>', requiredReviewerRole: '<reviewer>' }],
  generatedState: { roots: ['runs/<generated>'] },
  forbiddenActions: ['<execute-command>'],
  boundary: '<script>browser writes</script>',
} });
assert.match(productionBundleXssSurface, /&lt;bundle-xss&gt;/);
assert.match(productionBundleXssSurface, /&lt;script&gt;lane&lt;\/script&gt;/);
assert.doesNotMatch(productionBundleXssSurface, /<script>|<img|<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand/);
assert.match(cockpit.renderEvidencePane({ ...run, productionEvidenceBundle: productionBundleMissingReview }), /Production evidence bundle/);
assert.match(cockpit.renderEvidencePane({ ...run, productionEvidenceBundle: productionBundleMissingReview }), /missing-review-demo/);

const performanceRegressionLane = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/performance-regression-lane.stale.fixture.json', 'utf8'));
const performanceRegressionSurface = cockpit.renderPerformanceRegressionLaneSurface({ performanceRegressionLanes: { lanes: [performanceRegressionLane], boundary: 'Read-only performance/regression lanes; Studio does not execute commands, promote regressions, auto-apply, auto-merge, or self-approve.' } });
assert.match(performanceRegressionSurface, /Performance\/regression lane/);
assert.match(performanceRegressionSurface, /demo-performance-regression-lane-stale/);
assert.match(performanceRegressionSurface, /frame budget/);
assert.match(performanceRegressionSurface, /QA queue/);
assert.match(performanceRegressionSurface, /review gate/);
assert.match(performanceRegressionSurface, /stale-baseline-run\.json/);
assert.match(performanceRegressionSurface, /Browser metrics are advisory evidence inputs/);
assert.doesNotMatch(performanceRegressionSurface, /<script>|<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand|selfApprovalCommand/);
assert.match(cockpit.renderPerformanceRegressionLaneSurface(null), /No performance\/regression lane/);
assert.match(cockpit.renderEvidencePane({ ...run, performanceRegressionLane }), /Performance\/regression lane/);

const pluginRegistrySurfaceXss = cockpit.renderPluginRegistryBrowserSurface({ plugin_registry: { present: true, status: '<script>blocked</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], registries: [{ registryId: '<img src=x onerror=alert(1)>', plugins: [{ pluginId: '<img src=x onerror=alert(1)>', manifestPath: '<script>manifest</script>', manifestHash: '<script>hash</script>', manifestVersion: '<script>version</script>', validationStatus: '<script>valid</script>', compatibilityStatus: '<script>compat</script>', declaredCapabilities: ['<script>cap</script>'], extensionPoints: ['<script>point</script>'], blockedReasons: ['<script>reason</script>'] }] }] } });
assert.ok(!pluginRegistrySurfaceXss.includes('<script>blocked</script>'), 'plugin registry Studio status must be escaped');
assert.ok(!pluginRegistrySurfaceXss.includes('<img src=x onerror=alert(1)>'), 'plugin registry Studio rows must be escaped');


const pluginDashboardPanelXss = cockpit.renderPluginRegistryBrowserSurface({ plugin_registry: { present: true, status: 'ready', registries: [{ registryId: 'xss-registry', plugins: [{ pluginId: 'xss-plugin', validationStatus: 'valid', compatibilityStatus: 'compatible', declaredCapabilities: ['dashboardPanel'], extensionPoints: ['dashboard.panels.readOnly'], dashboardPanels: [{ panelId: '<img src=x onerror=alert(1)>', title: '<script>alert(1)</script>', dataSourceKey: 'javascript:alert(1)', templateRef: 'https://example.com/remote.js', layoutHint: 'onclick=alert(1)', displayHints: ['<script>hint</script>'], boundary: '<script>boundary</script>' }] }] }] } });
assert.doesNotMatch(pluginDashboardPanelXss, /<script>alert\(1\)<\/script>/);
assert.doesNotMatch(pluginDashboardPanelXss, /<img src=x onerror=alert\(1\)>/);
assert.doesNotMatch(pluginDashboardPanelXss, /<button/i);
assert.doesNotMatch(pluginDashboardPanelXss, /data-action=/i);
assert.doesNotMatch(pluginDashboardPanelXss, /href="javascript:/i);
assert.match(pluginDashboardPanelXss, /javascript:alert\(1\)/);
assert.match(pluginDashboardPanelXss, /&lt;script&gt;boundary&lt;\/script&gt;/);

const pluginScenarioTemplateXss = cockpit.renderPluginRegistryBrowserSurface({ plugin_registry: { present: true, status: 'ready', registries: [{ registryId: 'xss-registry', plugins: [{ pluginId: 'xss-scenario-template', validationStatus: 'valid', compatibilityStatus: 'compatible', declaredCapabilities: ['scenarioTemplate'], extensionPoints: ['scenario.templates.readOnly'], scenarioTemplates: [{ templateId: '<img src=x onerror=alert(1)>', description: '<script>alert(1)</script>', parameters: [{ name: '<script>name</script>', parameterType: 'enum', required: true, allowedValues: ['<script>easy</script>'] }], supportedGameTypes: ['<script>platformer</script>'], tags: ['<script>tag</script>'], expectedEvidenceType: '<script>scenarioPack</script>', validationHints: ['<script>hint</script>'], boundary: '<script>boundary</script>' }] }] }] } });
assert.doesNotMatch(pluginScenarioTemplateXss, /<script>alert\(1\)<\/script>/);
assert.doesNotMatch(pluginScenarioTemplateXss, /<img src=x onerror=alert\(1\)>/);
assert.doesNotMatch(pluginScenarioTemplateXss, /<button/i);
assert.doesNotMatch(pluginScenarioTemplateXss, /data-action=/i);
assert.match(pluginScenarioTemplateXss, /&lt;script&gt;scenarioPack&lt;\/script&gt;/);
assert.match(pluginScenarioTemplateXss, /&lt;script&gt;boundary&lt;\/script&gt;/);

const pluginRegistryBrowserSmokeMarkup = cockpit.renderPluginRegistryBrowserSurface(run);
assert.match(pluginRegistryBrowserSmokeMarkup, /Plugin registry browser/);
assert.match(pluginRegistryBrowserSmokeMarkup, /plugin-registry-summary/);
assert.match(pluginRegistryBrowserSmokeMarkup, /pluginRegistrySummaryCard/);
assert.match(pluginRegistryBrowserSmokeMarkup, /read-only-dashboard-panel/);
assert.match(pluginRegistryBrowserSmokeMarkup, /read-only-scenario-template/);
assert.match(pluginRegistryBrowserSmokeMarkup, /collect-goal-smoke/);
assert.match(pluginRegistryBrowserSmokeMarkup, /scenarioPack/);
assert.match(pluginRegistryBrowserSmokeMarkup, /no executable scripts/);
assert.match(pluginRegistryBrowserSmokeMarkup, /blocked-command-panel/);
assert.match(pluginRegistryBrowserSmokeMarkup, /manifest requested executable command authority/);
assert.ok(!/<button/i.test(pluginRegistryBrowserSmokeMarkup), 'plugin registry Studio smoke must not render action buttons');
assert.ok(!/data-action=/i.test(pluginRegistryBrowserSmokeMarkup), 'plugin registry Studio smoke must not render action hooks');
assert.ok(!/href=["']javascript:/i.test(pluginRegistryBrowserSmokeMarkup), 'plugin registry Studio smoke must not render javascript links');
assert.ok(!/command bridge/i.test(pluginRegistryBrowserSmokeMarkup), 'plugin registry Studio smoke must not advertise a command bridge');

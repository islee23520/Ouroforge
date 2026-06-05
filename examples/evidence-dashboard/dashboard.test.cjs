const assert = require('node:assert/strict');
const fs = require('node:fs');
const dashboard = require('./dashboard.js');

const run = {
  summary: {
    id: 'run-1',
    run_dir: 'runs/run-1',
    seed_id: 'platformer.v0',
    project: {
      id: 'minimal_2d',
      name: 'Minimal 2D Ouroforge Project',
      projectRoot: '.',
      manifestPath: 'ouroforge.project.json',
      manifestHash: { algorithm: 'fnv1a64-file-v1', value: 'manifesthash' },
      seedPath: 'seeds/platformer.yaml',
      scenes: [{ id: 'main', path: 'scenes/main.scene.json', hash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'scenehash' } }],
      scenarioPack: { id: 'smoke', path: 'scenarios/smoke.scenario-pack.json', scenarioIds: ['scaffold-smoke'] },
      transactionId: 'scene-edit-abc123',
    },
    run_status: 'created',
    verdict_status: 'failed',
    scenario_status: 'passed',
    worker_count: 4,
    evidence_count: 7,
    mutation_count: 1,
    evidence_categories: [
      { id: 'screenshots', label: 'Screenshots', count: 1, missing_count: 0, malformed_count: 0 },
      { id: 'world_states', label: 'World-state snapshots', count: 1, missing_count: 1, malformed_count: 0 },
      { id: 'frame_performance_metrics', label: 'Frame/performance metrics', count: 2, missing_count: 0, malformed_count: 0 },
      { id: 'console_cdp_summaries', label: 'Console/CDP summaries', count: 2, missing_count: 0, malformed_count: 1 },
      { id: 'scenario_results', label: 'Scenario results', count: 1, missing_count: 0, malformed_count: 0 },
      { id: 'mutation_artifacts', label: 'Mutation artifacts', count: 1, missing_count: 0, malformed_count: 0 },
    ],
    probe_contract_status: { status: 'present', contract_name: 'ouroforge-runtime-probe', version: 'v2', observed_count: 2, missing_count: 0, malformed_count: 0, evidence_refs: ['evidence/world.json', 'evidence/frame.json'] },
  },
  evidence: [
    { id: 'artifact-1', kind: 'application/json', path: 'evidence/a.json', metadata: {}, exists: true },
    { id: 'asset-reference-integrity', kind: 'application/json', path: 'evidence/assets/asset-reference-integrity.json', metadata: { artifact: 'asset_reference_integrity' }, exists: true },
  ],
  asset_integrity: {
    present: true,
    empty_state: '',
    warning_count: 2,
    stale_hash_count: 1,
    missing_ref_count: 1,
    invalid_type_count: 0,
    evidence_refs: ['evidence/assets/asset-reference-integrity.json'],
    warnings: [
      { field: 'scene entity player sprite asset', assetId: 'player_sprite', kind: 'stale_asset_hash', message: 'contentHash mismatch', path: 'assets/sprites/player.png' },
      { field: 'scene entity ghost audio event spawn asset', assetId: 'missing_audio', kind: 'missing_asset_ref', message: 'unknown project asset id' },
    ],
  },
  asset_loading: {
    present: true,
    empty_state: '',
    attempt_count: 2,
    loaded_count: 1,
    failed_count: 1,
    rejected_count: 0,
    fallback_count: 0,
    evidence_refs: ['evidence/scenarios/scaffold-smoke/asset-load-evidence.json'],
    boundary: 'Read-only runtime loading evidence; dashboard never fetches remote assets or writes trusted state.',
    records: [
      { attemptId: 'load-player-sprite', assetId: 'player_sprite', path: 'assets/sprites/player.png', status: 'loaded', loadDurationMs: 8, width: 16, height: 16 },
      { attemptId: 'load-missing-audio', assetId: 'missing_audio', path: 'assets/audio/missing.ogg', status: 'failed', loadDurationMs: 5, failureReason: 'Image load failed' },
    ],
  },
  asset_preview: {
    present: true,
    preview_count: 2,
    warning_count: 1,
    image_count: 1,
    atlas_frame_count: 1,
    tilemap_count: 0,
    audio_count: 0,
    font_count: 0,
    evidence_refs: ['evidence/assets/asset-preview-evidence.json'],
    boundary: 'Read-only asset preview evidence; dashboard never fetches remote assets or writes trusted state.',
    records: [
      { assetId: 'player_sprite', assetType: 'image', sourcePath: 'assets/sprites/player.png', previewKind: 'thumbnail', image: { width: 16, height: 16 } },
      { assetId: 'player_atlas', assetType: 'sprite_atlas', sourcePath: 'assets/atlases/player.atlas.json', previewKind: 'thumbnail', atlasFrames: [{ frameId: 'idle_0', rect: { x: 0, y: 0, width: 16, height: 16 } }] },
    ],
    warnings: [{ assetId: 'missing_audio', kind: 'missing_asset_file', message: 'missing audio preview source', path: 'assets/audio/missing.ogg' }],
  },
  plugin_registry: {
    present: true,
    status: 'blocked',
    registry_count: 1,
    plugin_count: 2,
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
      pluginCount: 2,
      blockedCount: 1,
      blockedReasons: ['blocked-command-panel:manifest requested executable command authority outside the v1 declarative catalog'],
      plugins: [
        { pluginId: 'read-only-dashboard-panel', manifestPath: 'plugins/read-only-dashboard-panel/plugin.json', manifestHash: 'fnv1a64-canonical-json-v1:1111222233334444', manifestVersion: '0.1.0', validationStatus: 'valid', compatibilityStatus: 'compatible', declaredCapabilities: ['dashboardPanel'], extensionPoints: ['dashboard.panels.readOnly'], evidenceRefs: ['runs/plugin-registry-fixture/plugin-evidence/read-only-dashboard-panel.validation.json'], blockedReasons: [] },
        { pluginId: 'blocked-command-panel', manifestPath: 'plugins/blocked-command-panel/plugin.json', manifestHash: 'fnv1a64-canonical-json-v1:aaaabbbbccccdddd', manifestVersion: '0.1.0', validationStatus: 'blocked', compatibilityStatus: 'incompatible', declaredCapabilities: ['studioInspectorPanel'], extensionPoints: ['studio.inspector.readOnly'], evidenceRefs: ['runs/plugin-registry-fixture/plugin-evidence/blocked-command-panel.validation.json'], blockedReasons: ['manifest requested executable command authority outside the v1 declarative catalog'] },
      ],
    }],
  },
  runtime_invariants: {
    present: true,
    status: 'failed',
    check_count: 3,
    passed_count: 1,
    failed_count: 1,
    unsupported_count: 1,
    missing_count: 0,
    malformed_count: 0,
    stale_count: 0,
    evidence_refs: ['evidence/scenarios/scaffold-smoke/runtime-invariant-evidence-qa.json'],
    boundary: 'Read-only runtime invariant evidence; dashboard never mutates source or executes commands.',
    summaries: [{ modelId: 'qa14_5_runtime_dashboard', runId: 'run-1', scenarioId: 'scaffold-smoke', checkCount: 3, passedCount: 1, failedCount: 1, unsupportedCount: 1, missingCount: 0, malformedCount: 0, staleCount: 0 }],
    evidence: [{ checks: [
      { invariantId: 'health-non-negative', invariantType: 'health_non_negative', status: 'failed', targetPath: 'player.health', message: 'health was negative', evidenceRefs: ['evidence/scenarios/scaffold-smoke/world-state.json'] },
      { invariantId: '<script>bad</script>', invariantType: 'finite_transform', status: 'passed', targetPath: 'player.transform', evidenceRefs: ['evidence/scenarios/scaffold-smoke/world-state.json'] },
    ] }],
  },
  route_attempts: {
    present: true,
    status: 'passed',
    attempt_count: 1,
    passed_count: 1,
    failed_count: 0,
    blocked_count: 0,
    inconclusive_count: 0,
    unsupported_count: 0,
    malformed_count: 0,
    evidence_refs: ['evidence/route-attempts/route-attempt.json', 'evidence/scenarios/collect-and-exit/world-state-start.json'],
    boundary: 'Read-only route attempt evidence; dashboard must not run solvers.',
    attempts: [{
      attemptId: 'qa14_6_collect_goal_route',
      runId: 'run-1',
      objectiveId: 'collect-goal-then-exit',
      scenarioId: 'collect-and-exit',
      startState: { stateId: 'start-left-of-goal', worldStateRef: 'evidence/scenarios/collect-and-exit/world-state-start.json' },
      strategyKind: 'simple_heuristic',
      outcome: 'passed',
      budgetUsed: { maxActions: 8, actionsUsed: 2, maxRouteNodes: 8, routeNodesUsed: 2 },
    }],
  },
  visual_comparisons: {
    present: true,
    status: 'changed',
    comparison_count: 1,
    unchanged_count: 0,
    changed_count: 1,
    missing_screenshot_count: 0,
    malformed_screenshot_count: 0,
    mismatched_dimensions_count: 0,
    unsupported_count: 0,
    blocked_count: 0,
    malformed_count: 0,
    evidence_refs: ['evidence/visual-comparisons/visual-comparison.json', 'evidence/screenshots/before-goal.png', 'evidence/screenshots/after-goal.png'],
    boundary: 'Read-only visual comparison evidence; dashboard must not compute trusted diffs.',
    summaries: [{
      comparisonId: 'qa14_7_collect_goal_visual',
      runId: 'run-1',
      scenarioId: 'collect-and-exit',
      checkpointId: 'goal-checkpoint',
      outcome: 'changed',
      failureClassification: 'visual_regression_candidate',
      changedPixels: 64,
      changedPercentX1000: 2,
      changedRegionCount: 1,
      beforeScreenshotRef: 'evidence/screenshots/before-goal.png',
      afterScreenshotRef: 'evidence/screenshots/after-goal.png',
    }],
  },
  qa_scenario_candidates: {
    present: true,
    status: 'proposed',
    candidate_count: 1,
    blocked_count: 0,
    deferred_count: 0,
    high_priority_count: 1,
    malformed_count: 0,
    evidence_refs: ['evidence/qa-scenario-candidates/scenario-candidate.json'],
    boundary: 'Read-only QA scenario candidates; dashboard must not run candidates.',
    candidates: [{
      candidateId: 'qa14_2_collect_and_exit_gap',
      runId: 'run-1',
      priority: 'high',
      status: 'proposed',
      sourceRisk: { riskId: 'objective-regression', description: 'Risk.' },
      targetObjective: { objectiveId: 'collect-goal-then-exit', description: 'Confirm objective.' },
      inputStrategy: { kind: 'replay_ref' },
      budget: { maxRuns: 2, maxSteps: 24 },
      expectedEvidence: [{ evidenceId: 'scenario-result' }, { evidenceId: 'world-state' }],
    }],
  },

  qa_agent_work_queues: {
    present: true,
    status: 'needs-rerun',
    queue_count: 1,
    item_count: 1,
    passed_count: 0,
    failed_count: 0,
    deferred_count: 0,
    blocked_count: 0,
    flaky_count: 0,
    needs_rerun_count: 1,
    malformed_count: 0,
    evidence_refs: [
      'evidence/qa-agent-work-queue/queue.json',
      'examples/scenario-packs/project-smoke.scenarios.json',
      'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json',
      'runs/multi-agent-pipeline/demo/qa/old-scenario-result.json',
    ],
    boundary: 'Read-only QA agent work queues; dashboard must not execute queue commands.',
    queues: [{
      schemaVersion: 'qa-agent-work-queue-v1',
      queueId: '<qa-queue>',
      milestone: 'multi-agent-production-pipeline-v1',
      items: [{
        queueItemId: '<qa-item>',
        scenarioTarget: { scenarioId: '<scenario>', scenarioPackRef: { id: 'scenario-pack', path: 'examples/scenario-packs/project-smoke.scenarios.json' } },
        riskArea: { riskId: '<risk>', category: 'scenario-regression', summary: 'Escaped <risk> summary.' },
        runCommandContext: { command: 'cargo run -p ouroforge-cli -- run <seed> --scenario-pack smoke', argv: [], boundary: 'Inert command text only; does not execute.' },
        expectedEvidence: [{ id: 'scenario-result', path: 'runs/multi-agent-pipeline/demo/qa/scenario-result.json' }, { id: 'evaluator-summary', path: 'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json' }],
        priority: 'high',
        assignedRole: 'qa-agent',
        assignedAgent: '<agent>',
        status: 'needs-rerun',
        failureClassification: 'stale-run-ref',
        taskRef: { id: 'task-board', path: 'examples/multi-agent-pipeline-v1/production-task-board.fixture.json' },
        workPackageRef: { id: 'work-package', path: 'examples/multi-agent-pipeline-v1/agent-work-package.valid.fixture.json' },
        reviewGateRef: { id: 'review-gate', path: 'examples/multi-agent-pipeline-v1/review-critic-gate.valid.fixture.json' },
        runEvidenceRefs: [{ id: 'run', path: 'runs/multi-agent-pipeline/demo/qa/scenario-result.json' }],
        evaluatorEvidenceRefs: [{ id: 'evaluator', path: 'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json' }],
        blockedReasons: ['refresh <stale> run evidence'],
        staleRunRefs: ['runs/multi-agent-pipeline/demo/qa/old-scenario-result.json'],
      }],
      forbiddenActions: ['browser command bridge', 'auto-merge'],
      boundary: 'QA agent work queue is inert local evidence; it does not execute commands.',
    }],
  },
  fuzzing_plans: {
    present: true,
    status: 'planned',
    plan_count: 1,
    blocked_count: 0,
    exhausted_count: 0,
    malformed_count: 0,
    evidence_refs: ['evidence/fuzzing-plans/fuzzing-plan.json'],
    boundary: 'Read-only adversarial input fuzzing plans; dashboard must not run fuzzers.',
    plans: [{
      planId: 'qa14_3_seeded_movement_fuzz',
      runId: 'run-1',
      deterministicSeed: 424242,
      inputDomain: { scenarioId: 'collect-and-exit', domainId: 'movement-buttons' },
      budget: { maxRuns: 4, maxSteps: 32 },
      outputRoot: 'evidence/fuzz/qa14_3_seeded_movement_fuzz/',
      cleanupPolicy: { mode: 'retain_for_review' },
      status: 'planned',
      expectedEvidence: [{ evidenceId: 'seeded-inputs' }, { evidenceId: 'runtime-probe' }],
    }],
  },
  qa_worker_assignments: {
    present: true,
    status: 'blocked',
    assignment_count: 2,
    passed_count: 0,
    failed_count: 0,
    deferred_count: 0,
    blocked_count: 1,
    exhausted_count: 1,
    malformed_count: 0,
    evidence_refs: ['evidence/qa-worker-assignment/worker-assignment.json'],
    boundary: 'Read-only QA worker assignment evidence; dashboard must not spawn workers.',
    plans: [{
      planId: 'qa14_4_worker_assignment_smoke',
      runId: 'run-1',
      assignments: [
        { assignmentId: 'scenario-worker', workerId: 'qa-worker-1', assignedLane: 'scenario-playtest', status: 'blocked', target: { targetType: 'scenario_candidate', targetId: 'collect-and-exit' }, budget: { maxRuns: 3 }, timeoutMs: 10000, outputRoot: 'evidence/qa-workers/qa-worker-1/run/', cleanupPolicy: { mode: 'retain_on_failure' }, blockedReasons: ['budget gate pending'] },
        { assignmentId: '<script>worker</script>', workerId: 'qa-worker-2', assignedLane: 'fuzz-smoke', status: 'exhausted', target: { targetType: 'fuzz_target', targetId: 'movement-fuzz' }, budget: { maxRuns: 2 }, timeoutMs: 8000, outputRoot: 'evidence/qa-workers/qa-worker-2/run/', cleanupPolicy: { mode: 'retain_for_review' } },
      ],
    }],
  },
  source_apply_worktree_context: {
    present: true,
    status: 'blocked',
    target_count: 2,
    blocked_count: 2,
    evidence_refs: ['evidence/source-apply/worktree-context.json'],
    boundary: 'Read-only context evidence; dashboard cannot apply patches, execute commands, write trusted files, merge branches, or bypass review gates.',
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
  probe_contract_status: { status: 'present', contract_name: 'ouroforge-runtime-probe', version: 'v2', observed_count: 2, missing_count: 0, malformed_count: 0, evidence_refs: ['evidence/world.json', 'evidence/frame.json'] },
  engine_summaries: {
    present: true,
    source_world_state: 'evidence/world.json',
    tilemaps: {
      present: true,
      tilemapCount: 1,
      layerCount: 2,
      authoring: { present: true, collisionCellCount: 3, triggerCellCount: 1, hazardCellCount: 1, goalCellCount: 1 },
      tilemaps: [{
        id: '<platformer-ground>',
        grid: { width: 20, height: 3 },
        tileCount: 4,
        layerCount: 2,
        authoring: { collisionCellCount: 3, triggerCellCount: 1, hazardCellCount: 1, goalCellCount: 1 },
      }],
    },
    render_breakdown: {
      present: true,
      frameId: 'frame-0003',
      sceneId: '<scene-render-test>',
      elements: [{ renderableId: '<entity:player>', entityId: 'player', drawOrder: 0, layer: 'actors', primitiveCategory: 'sprite' }],
      absenceDiagnostics: [{ entityId: '<hidden>', reason: 'hidden', detail: 'sprite.visible=false' }],
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    },
    render_queue: {
      present: true,
      frameId: 'frame-0003',
      sceneId: '<scene-render-test>',
      layerCount: 1,
      renderableCount: 2,
      drawCallCount: 1,
      skippedRenderableCount: 1,
      validation: { status: 'ready', blockedReasons: [], warnings: [] },
      tilemapStats: { layerCount: 1, cellCount: 2, drawnTileCount: 2, missingTileRefCount: 1, assetTileCount: 1 },
      renderables: [
        { id: '<queue:player>', sourceKind: 'entity', sourceId: 'player', drawOrder: 0, layer: 'actors', primitiveKind: 'sprite', visible: true },
        { id: '<tilemap:ground>', sourceKind: 'tilemap-layer', sourceId: 'level:ground', drawOrder: 1, layer: 'ground', primitiveKind: 'tilemap', visible: true, tileCount: 2, missingTileRefCount: 1, assetTileCount: 1 },
        { id: '<queue:hidden>', sourceKind: 'entity', sourceId: 'hidden', drawOrder: 1, layer: 'actors', primitiveKind: 'rect', visible: false, fallbackReason: 'sprite hidden' },
      ],
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    },
    scene3d_render: {
      present: true,
      frameId: 'frame-0003',
      sceneId: '<scene-render-test>',
      cameraId: '<main-camera>',
      meshCount: 1,
      materialCount: 1,
      attemptedObjectCount: 2,
      visibleObjectCount: 1,
      skippedObjectCount: 1,
      failedObjectCount: 0,
      screenshotArtifact: null,
      renderables: [
        { id: '<scene3d:cube>', nodeId: '<cube-node>', meshRef: '<cube-mesh>', materialRef: '<cube-mat>', primitive: 'cube', cameraId: '<main-camera>', visible: true },
        { id: '<scene3d:missing>', nodeId: '<missing-node>', meshRef: '<missing-mesh>', primitive: 'cube', cameraId: '<main-camera>', visible: false, fallbackReason: '<missing mesh>' },
      ],
      fallbackReasons: ['<missing-node>: <missing mesh>'],
      boundary: 'Read-only bounded 3D render smoke evidence; no WebGPU, GLTF import, PBR, remote fetch, or production renderer claim.',
    },
    scene3d_collision: {
      present: true,
      frameId: '<frame-0003>',
      sceneId: '<scene-render-test>',
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
    renderer: {
      version: '1',
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
        readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
      },
    },
    gameplay: {
      present: true,
      declaredFlagCount: 2,
      worldFlagCount: 3,
      trueFlagCount: 2,
      falseFlagCount: 1,
      triggerEntityCount: 1,
      goalFlagEntityCount: 1,
      hudValueEntityCount: 2,
      hudValues: [
        { entityId: 'hud_goal', kind: 'goal', label: 'Goal', value: 'Collect coin', bindFlag: 'coin_collected', flagValue: true, text: 'Goal: Collect coin' },
        { entityId: 'hud_health', kind: 'health', label: 'HP', value: '3/3', text: 'HP: 3/3' },
      ],
      triggerCollisionEventCount: 1,
      trueFlags: ['coin_collected', 'door_open'],
    },
    animation: { animatedEntityCount: 1, activeStateCount: 1 },
    vfx: { present: true, vfxEntityCount: 1, vfxEmitterCount: 1, vfxEventCount: 1 },
    audio: {
      audioEntityCount: 1,
      audioEventCount: 1,
      audioWarningCount: 1,
      browserAudioAuthority: 'intent_evidence_only',
      audioEvents: [{ name: '<coin>', intentKind: '<sound>', busId: '<sfx>', volume: 80 }],
      audioWarnings: [{ warning: '<audible_output_not_verified>', requestId: '<audio-1-1>' }],
    },
    runtime_frame_budget: {
      schemaVersion: 'ouroforge.runtime-frame-budget.v1',
      frameId: '<frame-0003>',
      sceneId: '<scene-render-test>',
      scenarioId: '<scaffold-smoke>',
      timings: { updateMs: 3, renderMs: 18.5, evidenceMs: 1, totalMs: 24.25 },
      budget: { updateMs: 8, renderMs: 16, evidenceMs: 4, totalMs: 20 },
      counts: { entityCount: 3, drawCallCount: 1, layerCount: 2, collisionPairCount: 1, activeAnimationCount: 1, activeVfxCount: 1, audioEventCount: 1 },
      status: 'violated',
      slowFrame: true,
      violations: [{ field: '<renderMs>', actualMs: 18.5, budgetMs: 16 }],
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'live mutation', 'remote telemetry'] },
      authority: 'browser_runtime_evidence_input_not_profiler_truth',
    },
    events: {
      present: true,
      animationEntityCount: 1,
      audioEventCount: 1,
      audioWarningCount: 1,
      vfxEventCount: 1,
      animationEntities: [{ entityId: '<player>', activeState: '<run>', currentClip: '<run-clip>', frameIndex: 2 }],
      audioEvents: [{ name: '<coin>', intentKind: '<sound>', busId: '<sfx>', volume: 80 }],
      audioWarnings: [{ warning: '<audible_output_not_verified>', requestId: '<audio-1-1>' }],
      vfxEvents: [{ emitterId: '<run-dust>', kind: '<trail>', particleCount: 8 }],
    },
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
    planPath: '<plan>',
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
    allowedCommands: [{ command: 'cargo run -p ouroforge-cli -- loop status <plan>', argv: [], boundary: 'inert display text only; browser does not execute' }],
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
  command_context: {
    schemaVersion: 'run-command-context-v1',
    command: 'cargo run -p ouroforge-cli -- run seeds/platformer.yaml --project examples/project --workers 4 --scenario-pack smoke',
    argv: ['cargo', 'run', '-p', 'ouroforge-cli', '--', 'run', 'seeds/platformer.yaml', '--project', 'examples/project', '--workers', '4', '--scenario-pack', 'smoke'],
    seedPath: 'seeds/platformer.yaml',
    workers: 4,
    runsRoot: 'runs',
    projectRoot: 'examples/project',
    manifestPath: 'examples/project/ouroforge.project.json',
    scenarioPackId: 'smoke',
    runtimeTarget: 'local-static-browser',
    browserBoundary: 'openchrome_cdp',
    cdpTransport: 'chrome_devtools_protocol',
    environmentHints: ['Local Chrome/CDP required', 'Dashboard does not execute commands'],
  },
  project: {
    id: 'minimal_2d',
    name: 'Minimal 2D Ouroforge Project',
    projectRoot: '.',
    manifestPath: 'ouroforge.project.json',
    manifestHash: { algorithm: 'fnv1a64-file-v1', value: 'manifesthash' },
    seedPath: 'seeds/platformer.yaml',
    scenes: [{ id: 'main', path: 'scenes/main.scene.json', hash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'scenehash' } }],
    scenarioPack: { id: 'smoke', path: 'scenarios/smoke.scenario-pack.json', scenarioIds: ['scaffold-smoke'] },
    transactionId: 'scene-edit-abc123',
  },
  screenshots: [{ id: 'shot-1', kind: 'image/png', path: 'evidence/shot.png', metadata: {}, exists: true }],
  world_states: [{ id: 'world-1', kind: 'application/json', path: 'evidence/world.json', value: { object: { x: 40 } }, metadata: {} }],
  frame_metrics: [{ id: 'frame-1', kind: 'application/json', path: 'evidence/frame.json', value: { frame: 1 }, metadata: {} }],
  performance_metrics: [{ id: 'perf-1', kind: 'application/json', path: 'evidence/perf.json', value: { metrics: [] }, metadata: { artifact: 'performance_metrics', worker_id: 'worker-1', worker_session_id: 'run-1:worker-1', run_id: 'run-1', execution_boundary: 'openchrome_cdp', cdp_transport: 'chrome_devtools_protocol', bounded: true, optional: true } }],
  console_logs: [{ id: 'console-1', kind: 'application/json', path: 'evidence/console.json', value: [{ text: 'ready' }], metadata: { artifact: 'console_log', worker_id: 'worker-1', worker_session_id: 'run-1:worker-1', run_id: 'run-1', execution_boundary: 'openchrome_cdp', cdp_transport: 'chrome_devtools_protocol', bounded: true, limit: 100 } }],
  cdp_trace_summaries: [{ id: 'cdp-1', kind: 'application/json', path: 'evidence/cdp.json', value: null, read_error: 'bad json', metadata: { artifact: 'cdp_trace_summary', worker_id: '<script>worker</script>', worker_session_id: 'run-1:<script>worker</script>', run_id: 'run-1', execution_boundary: 'openchrome_cdp', cdp_transport: 'chrome_devtools_protocol', bounded: true, limit: 32 } }],
  scenario_results: [{ id: 'scenario-1', kind: 'application/json', path: 'evidence/scenario.json', value: { status: 'passed' }, metadata: {} }],
  mutation_artifacts: [{ id: 'mutation-proposals', kind: 'application/json', path: 'mutation/proposals.json', value: { proposals: [] }, metadata: {} }, { id: 'source-patch-evidence-bundle', kind: 'application/json', path: 'mutation/source-patch-evidence-bundle.json', metadata: { read_only: true }, value: { bundleId: 'bundle-1', patchPreviewId: 'preview-1', status: 'complete', patchSummary: { title: 'Review docs patch preview', expectedBehaviorChange: 'Docs preview only; no trusted worktree apply.', targetCount: 2, changedLines: 18 }, fileClassSummary: { allowed: 1, reviewHeld: 1, blocked: 0, highestRisk: 'review_held' }, riskIds: ['source_patch_preview', 'review_held_target'], linkedEvidence: [{ kind: 'source-patch-preview', path: 'mutation/preview.json' }, { kind: 'sandbox-dry-run-report', path: 'sandbox/preview-1/evidence/report.json' }, { kind: 'review-decision', path: 'mutation/review-decisions.json' }], dryRunSummary: { status: 'passed', allowlistPolicyId: 'source-patch-preview-safe-local-checks-v1', reportRef: { kind: 'sandbox-dry-run-report', path: 'sandbox/preview-1/evidence/report.json' } }, requiredTestSummary: { total: 2, commands: ['cargo fmt --check', 'cargo test -p ouroforge-core'], allowlistPolicyId: 'source-patch-preview-safe-local-checks-v1' }, reviewSummary: { status: 'reviewed', decisionRef: { kind: 'review-decision', path: 'mutation/review-decisions.json' } }, blockedReasons: ['manual review required before any future apply design'], previewRef: { kind: 'source-patch-preview', path: 'mutation/preview.json' }, fileClassReportRef: { kind: 'file-class-report', path: 'evidence/file-class.json' }, diffIntegrityReportRef: { kind: 'diff-integrity-report', path: 'evidence/diff.json' }, sandboxReportRef: { kind: 'sandbox-dry-run-report', path: 'sandbox/preview-1/evidence/report.json' }, testSummaryRef: { kind: 'test-summary', path: 'sandbox/preview-1/evidence/tests.json' }, reviewDecisionRef: { kind: 'review-decision', path: 'mutation/review-decisions.json' }, forbiddenActionNotices: [{ action: 'apply_patch', reason: 'forbidden' }, { action: 'merge_branch', reason: 'forbidden' }, { action: 'execute_command', reason: 'forbidden' }, { action: 'write_trusted_file', reason: 'forbidden' }], guardrails: ['read-only bundle evidence', 'no source patch apply'] } }, { id: 'source-patch-apply-transaction', kind: 'application/json', path: 'mutation/source-patch-apply-transaction.json', metadata: { read_only: true }, value: { transactionId: 'apply-tx-1', status: 'ready_for_trusted_apply', evidence: { patchPreviewRef: 'mutation/preview.json', sandboxReportRef: 'sandbox/preview-1/evidence/report.json', reviewDecisionRef: 'mutation/review-decision.json', fileClassReportRef: 'evidence/file-class.json', diffIntegrityReportRef: 'evidence/diff.json' }, targets: [{ path: 'examples/source-patch-apply-transaction-v1/scenario-regression.json', fileClass: 'scenario_regression_fixture', reviewLevel: 'elevated_source_like_data_review' }], rollbackRef: { rollbackPlanRef: 'rollback/apply-tx-1.json' }, readModel: { status: 'passed', readinessLabel: 'ready_metadata_only_no_apply_authority', blockedReasons: [], forbiddenActions: ['apply_patch', 'merge_branch', 'execute_command', 'write_trusted_file', 'browser_command_bridge'] } } }],
  mutations: [{ id: 'mutation-1', evidence_id: 'artifact-1', status: 'proposed', target: 'seeds/platformer.yaml', rationale: { schema_version: '1', failure_classification: 'scenario_assertion_failure', evidence_artifact_ids: ['artifact-1'], scenario_result_refs: ['evidence/scenario.json'], verdict_refs: ['verdict.json'], expected_effect: 'player reaches the goal', confidence: 'medium', reasoning_summary: 'scenario assertion failed', allowed_mutation_type: 'data_only' } }],
  mutation_lifecycle: {
    terminal_state: 'accepted',
    command_hints: [
      'cargo run -p ouroforge-cli -- mutation review runs/run-1 --accept --reason "manual evidence review accepted"',
      'cargo run -p ouroforge-cli -- mutation review runs/run-1 --reject --reason "manual evidence review rejected"',
    ],
    stages: [
      { id: 'proposed', label: 'Proposed', state: 'proposed', artifact_path: 'mutation/proposals.json', record_count: 1, evidence_refs: ['artifact-1'], records: [{ id: 'mutation-1', rationale: { failure_classification: 'legacy_stage_rationale', evidence_artifact_ids: ['artifact-1'], expected_effect: 'stage fallback visible', confidence: 'low', reasoning_summary: 'stage record rationale', allowed_mutation_type: 'data_only' } }] },
      { id: 'classified', label: 'Classified', state: 'classified', artifact_path: 'mutation/classifications.json', record_count: 1, evidence_refs: ['evidence/world.json'], records: [{ id: 'classification-1' }] },
      { id: 'drafted', label: 'Drafted', state: 'drafted', artifact_path: 'mutation/patch-drafts.json', record_count: 1, evidence_refs: ['evidence/world.json'], records: [{ id: 'patch-draft-1' }] },
      { id: 'sandboxed', label: 'Sandboxed', state: 'sandboxed', artifact_path: 'sandbox/*/evidence/result.json', record_count: 1, evidence_refs: ['sandbox/patch-draft-1/evidence/result.json'], records: [{ patch_draft_id: 'patch-draft-1' }] },
      { id: 'compared', label: 'Compared', state: 'compared', artifact_path: 'mutation/rerun-orchestration.json', record_count: 1, evidence_refs: ['mutation/rerun-orchestration.json'], records: [{ comparison_artifact_path: 'mutation/run-comparison-before--after.json' }] },
      { id: 'scene_applied', label: 'Applied scene mutation', state: 'applied', artifact_path: 'mutation/scene-applications.json', record_count: 1, evidence_refs: [], records: [{ id: 'scene-application-1', proposalId: 'mutation-1', transactionId: 'scene-edit-abc123', reviewDecisionId: 'review-decision-1', targetScenePath: 'examples/project/scenes/main.scene.json', transactionArtifactPath: 'mutation/scene-edit.json', beforeSceneHash: { value: 'beforehash' }, afterSceneHash: { value: 'afterhash' }, project: { projectId: 'minimal_2d', manifestPath: 'ouroforge.project.json', manifestHash: { algorithm: 'fnv1a64-file-v1', value: 'manifesthash' }, scenePath: 'scenes/main.scene.json', sceneHash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'beforehash' } }, rollback: { scenePath: 'examples/project/scenes/main.scene.json', restoreHash: { value: 'beforehash' }, strategy: 'restore beforeSceneHash' }, status: 'applied' }] },
      { id: 'visual_draft_applied', label: 'Applied visual draft', state: 'applied', artifact_path: 'mutation/visual-edit-applications.json', record_count: 1, evidence_refs: ['mutation/visual-edit-applications.json'], records: [{ id: 'visual-edit-application-1', draftId: 'draft-collect-and-exit-scene-demo', proposalId: 'mutation-1', patchDraftId: 'patch-draft-1', reviewDecisionId: 'review-decision-1', transactionId: 'scene-edit-visual-abc123', transactionArtifactPath: 'mutation/visual-scene-edit.json', targetScenePath: 'examples/project/scenes/main.scene.json', beforeSceneHash: { value: 'beforehash' }, afterSceneHash: { value: 'visualafterhash' }, commandContext: { command: 'cargo run -p ouroforge-cli -- edit draft-apply examples/visual-edit-draft-v1/valid/collect-and-exit-scene-demo.visual-edit-draft.json' } }] },
      { id: 'reviewed', label: 'Manual review', state: 'accepted', artifact_path: 'mutation/review-decisions.json', record_count: 1, evidence_refs: ['mutation/rerun-orchestration.json'], records: [{ id: 'review-decision-1', proposal_id: 'mutation-1', patch_draft_id: 'patch-draft-1', state: 'accepted', decision_status: 'accepted', reviewer_type: 'agent', reviewer: 'agent-reviewer', reason: '<script>accepted</script>', evidence_refs: ['mutation/rerun-orchestration.json'], guardrail_checklist: { proposal_is_record_only: true, accepted_does_not_apply: true, browser_read_only: true, evidence_refs_checked: true } }] },
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
    changes: ['created_group:promoted-regressions', 'added_scenario:promoted-smoke-regression'],
    recordPath: 'regression-promotions/regression-promotion-1.json',
  }],
  transaction_provenance: {
    transactionId: 'scene-edit-abc123',
    transactionArtifactPath: 'transactions/scene-edit.json',
    scenePath: 'examples/game-runtime/scene.json',
    beforeSceneHash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'beforehash' },
    afterSceneHash: { algorithm: 'fnv1a64-canonical-json-v1', value: 'afterhash' },
  },
  comparison: {
    present: true,
    empty_state: '',
    artifacts: [
      {
        id: 'run-comparison-before--after',
        path: 'mutation/run-comparison-before--after.json',
        exists: true,
        read_error: null,
        before_run_id: 'before-run',
        after_run_id: 'after-run',
        classification: 'improved',
        deltas: {
          scenario_results: 1,
          verdict_status: 'failed -> passed',
          failed_scenarios: -1,
          assertion_failures: -2,
          performance_artifacts: 1,
          evidence_artifacts: 3,
        },
        semantic: {
          schemaVersion: 'run-semantic-diff-v1',
          reasons: [
            { kind: 'scenario_verdict', severity: 'improved', summary: 'scenario smoke changed from failed to passed', evidenceRefs: [] },
            { kind: 'world_state', severity: 'changed', summary: '2 world-state values changed, 0 added, 0 removed', evidenceRefs: [] },
          ],
          scenarios: [{ scenarioId: 'smoke', before: 'failed', after: 'passed', classification: 'improved', evidenceRefs: [] }],
          worldState: { changed: [{ path: 'world/player/x', before: 1, after: 4 }], added: [], removed: [] },
          events: { added: ['console:warn:changed'], removed: [] },
          performance: { changed: [{ path: 'frame_stats/fps', before: 60, after: 55 }], warnings: [] },
          evidence: { added: ['artifact|application/json|evidence/new.json'], removed: [] },
          project: {
            relation: 'same_project',
            changed: true,
            changes: [{ kind: 'scene_hash', summary: 'scene hash changed for scenes/main.scene.json', before: 'before-scene', after: 'after-scene' }],
            warnings: ['project fixture warning'],
          },
          transactionProvenance: { changed: true },
          warnings: ['after world_state artifact could not be read: evidence/missing.json'],
        },
        evidence_refs: [
          'runs/before-run/verdict.json',
          'runs/after-run/verdict.json',
          'evidence/scenarios/replay-smoke/scenario-result.json',
        ],
        unsupported: ['semantic gameplay quality is not inferred'],
        value: {
          before_run_id: 'before-run',
          after_run_id: 'after-run',
          classification: 'improved',
          evidence_refs: ['runs/before-run/verdict.json', 'runs/after-run/verdict.json'],
          semantic: {
            schemaVersion: 'run-semantic-diff-v1',
            reasons: [{ kind: 'fallback', severity: 'changed', summary: 'fallback semantic', evidenceRefs: [] }],
          },
        },
      },
    ],
  },
  replay: {
    present: true,
    empty_state: '',
    sequences: [
      {
        id: 'move-right-four-frames',
        source: 'inline',
        scenario_id: 'replay-smoke',
        replay_path: 'evidence/scenarios/replay-smoke/input-replay.json',
        event_count: 2,
        frames: [0, 4],
        first_frame: 0,
        last_frame: 4,
        evidence_refs: [
          'evidence/scenarios/replay-smoke/input-replay.json',
          'evidence/scenarios/replay-smoke/scenario-result.json',
        ],
        checkpoints: [
          {
            id: 'initial',
            label: 'Initial state',
            frame: 0,
            tick: 0,
            world_state_path: 'evidence/scenarios/replay-smoke/world-state-initial.json',
            frame_stats_path: 'evidence/scenarios/replay-smoke/frame-stats-initial.json',
            world_state: { tick: 0, entities: [{ id: 'player', components: { transform: { x: 32, y: 72 } } }] },
          },
          {
            id: 'post-replay',
            label: 'Post-replay world state',
            frame: 4,
            tick: 4,
            world_state_path: 'evidence/scenarios/replay-smoke/world-state.json',
            frame_stats_path: 'evidence/scenarios/replay-smoke/frame-stats.json',
            world_state: { tick: 4, entities: [{ id: 'player', components: { transform: { x: 40, y: 72 } } }] },
          },
        ],
      },
    ],
  },
  journal_view: {
    path: 'journal.md',
    exists: true,
    read_error: null,
    summary: 'fixture journal summary',
    evidence_refs: ['evidence/world.json'],
    verdict_refs: ['verdict.json'],
    mutation_refs: ['mutation-1'],
    entries: [
      {
        id: 'journal-entry-1-observations',
        heading: 'Observations',
        category: 'observation',
        body: 'Evidence `world-1` at `evidence/world.json`.',
        evidence_refs: ['evidence/world.json'],
        verdict_refs: [],
        mutation_refs: [],
      },
      {
        id: 'journal-entry-2-authoring-governance-lifecycle',
        heading: 'Authoring Governance Lifecycle',
        category: 'mutation',
        body: 'Schema `journal-authoring-governance-v2` links proposal `proposal-1` review `review-1` promotion `regression-promotion-1`.',
        evidence_refs: ['mutation/rerun-orchestration.json'],
        verdict_refs: [],
        mutation_refs: ['proposal-1', 'review-1', 'regression-promotion-1'],
      },
      {
        id: 'journal-entry-3-verdict-summary',
        heading: 'Verdict Summary',
        category: 'verdict',
        body: '<script>alert(1)</script>',
        evidence_refs: [],
        verdict_refs: ['verdict.json'],
        mutation_refs: ['mutation-1'],
      },
    ],
  },
  verdict: { status: 'failed' },
  journal: '# Journal',
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

assert.equal(dashboard.statusClass('passed'), 'status status-passed');
assert.equal(dashboard.artifactHref(run.evidence[0], run), '../../runs/run-1/evidence/a.json');
const runList = dashboard.renderRunList([run], 'run-1');
assert.match(runList, /platformer\.v0/);
assert.match(runList, /project minimal_2d/);
assert.match(runList, /4 workers/);
assert.match(runList, /scenario passed/);
const detail = dashboard.renderRunDetail(run);
assert.match(detail, /World-state snapshots/);
assert.match(detail, /Frame\/performance metrics/);
assert.match(detail, /Console\/CDP summaries/);
assert.match(detail, /worker session id/);
assert.match(detail, /run-1:worker-1/);
assert.match(detail, /openchrome_cdp/);
assert.match(detail, /chrome_devtools_protocol/);
assert.match(detail, /bad json/);
assert.ok(!detail.includes('<script>worker</script>'), 'CDP metadata must be escaped');
assert.match(detail, /Scenario results/);
assert.match(detail, /Mutation artifacts/);
assert.match(detail, /Journal Viewer/);
assert.match(detail, /Mutation Review/);
assert.match(detail, /Gameplay trigger\/flags/);
assert.match(detail, /coin_collected, door_open/);
assert.match(detail, /Regression Promotions/);
assert.match(detail, /promoted-smoke-regression/);
assert.match(detail, /scenario promote &lt;draft-json&gt; --project ouroforge\.project\.json --scenario-pack smoke --dry-run/);
assert.match(detail, /Replay Controls/);
assert.match(detail, /Project Context/);
assert.match(detail, /Minimal 2D Ouroforge Project/);
assert.match(detail, /manifesthash/);
assert.match(detail, /scenes\/main\.scene\.json/);
assert.match(detail, /scenehash/);
assert.match(detail, /Scene Edit Transaction/);
assert.match(detail, /scene-edit-abc123/);
assert.match(detail, /beforehash/);
assert.match(detail, /afterhash/);
assert.match(detail, /Run Comparison/);
assert.match(detail, /Read-only\. Displays existing comparison artifacts only/);
assert.match(detail, /Semantic evidence diff/);
assert.match(detail, /scenario smoke changed from failed to passed/);
assert.match(detail, /run-semantic-diff-v1/);
assert.match(detail, /Project context diff/);
assert.match(detail, /same_project/);
assert.match(detail, /scene hash changed for scenes\/main\.scene\.json/);
assert.match(detail, /project fixture warning/);
assert.match(detail, /Semantic warnings/);
assert.match(detail, /before-run/);
assert.match(detail, /after-run/);
assert.match(detail, /improved/);
assert.match(detail, /scenario results/);
assert.match(detail, /verdict status/);
assert.match(detail, /assertion failures/);
assert.match(detail, /semantic gameplay quality is not inferred/);
assert.ok(detail.includes('../../runs/before-run/verdict.json'));
assert.ok(detail.includes('../../runs/after-run/verdict.json'));
assert.ok(detail.includes('../../runs/run-1/evidence/scenarios/replay-smoke/scenario-result.json'));
assert.match(detail, /Inspect-only\. Controls are local\/in-memory/);
assert.match(detail, /Current frame/);
assert.match(detail, /Initial state/);
assert.match(detail, /Post-replay world state/);
assert.match(detail, /Step forward/);
assert.match(detail, /Reset/);
assert.match(detail, /evidence\/scenarios\/replay-smoke\/input-replay\.json/);
assert.match(detail, /Inspect-only/);
assert.match(detail, /Proposed/);
assert.match(detail, /Classified/);
assert.match(detail, /Drafted/);
assert.match(detail, /Sandboxed/);
assert.match(detail, /Compared/);
assert.match(detail, /Applied scene mutation/);
assert.match(detail, /Applied visual draft/);
assert.match(detail, /Visual draft application/);
assert.match(detail, /draft-collect-and-exit-scene-demo/);
assert.match(detail, /Display-only rerun context/);
assert.match(detail, /Project-scoped scene mutation/);
assert.match(detail, /minimal_2d/);
assert.match(detail, /manifesthash/);
assert.match(detail, /scenes\/main\.scene\.json/);
assert.match(detail, /Rollback/);
assert.match(detail, /Manual review/);
assert.match(detail, /accepted/);
assert.match(detail, /mutation\/review-decisions\.json/);
assert.match(detail, /Review decision ledger/);
assert.match(detail, /review-decision-1/);
assert.match(detail, /agent-reviewer/);
assert.match(detail, /accepted_does_not_apply=true/);
assert.ok(!detail.includes('<script>accepted</script>'));
assert.match(detail, /mutation review runs\/run-1 --accept/);
assert.match(detail, /No lifecycle records for this stage|patch-draft-1/);
assert.match(detail, /fixture journal summary/);
assert.match(detail, /Observations/);
assert.match(detail, /Authoring Governance Lifecycle/);
assert.match(detail, /journal-authoring-governance-v2/);
assert.match(detail, /regression-promotion-1/);
assert.match(detail, /Runtime probe contract/);
assert.match(detail, /ouroforge-runtime-probe v2/);
assert.match(detail, /observed 2/);
assert.ok(detail.includes('../../runs/run-1/evidence/world.json'));
assert.ok(detail.includes('../../runs/run-1/verdict.json'));
assert.match(detail, /mutation-1/);
assert.ok(!detail.includes('<script>alert(1)</script>'), 'journal entry markup must be escaped');
assert.match(detail, /&lt;script&gt;/);
assert.match(detail, /mutation\/proposals\.json/);
assert.match(detail, /Proposal rationale/);
assert.match(detail, /scenario_assertion_failure/);
assert.match(detail, /player reaches the goal/);
assert.match(detail, /data_only/);
assert.match(dashboard.renderProposalRationaleList({ mutations: [{ id: '<script>', status: '<bad>', evidence_id: '<img>', target: '<svg>', rationale: { failure_classification: '<script>', evidence_artifact_ids: ['<img>'], expected_effect: '<b>effect</b>', confidence: '<i>', reasoning_summary: '<svg>', allowed_mutation_type: '<bad>' } }] }), /&lt;script&gt;/);
assert.match(dashboard.renderProposalRationaleList({ mutations: [{ id: 'missing-rationale', status: 'proposed' }] }), /No proposal rationale recorded/);
assert.match(detail, /1 missing/);
assert.match(detail, /1 malformed/);
assert.match(detail, /bad json/);
assert.match(dashboard.renderCategorySummary(run.summary.evidence_categories), /Frame\/performance metrics/);
assert.match(dashboard.renderProbeContractStatus(run.probe_contract_status), /present/);
assert.match(dashboard.renderProbeContractStatus({ status: 'malformed', contract_name: 'ouroforge-runtime-probe', version: 'v2', observed_count: 1, missing_count: 1, malformed_count: 1, evidence_refs: ['evidence/failure.json'] }), /1 malformed/);
assert.match(dashboard.renderTilemapSummary(run.engine_summaries), /Tilemaps/);
assert.match(dashboard.renderTilemapSummary(run.engine_summaries), /Collision cells/);
assert.match(dashboard.renderTilemapSummary(run.engine_summaries), /3 collision \/ 1 trigger \/ 1 hazard \/ 1 goal/);
assert.match(dashboard.renderTilemapSummary(run.engine_summaries), /&lt;platformer-ground&gt;/);
assert.match(dashboard.renderTilemapSummary({}), /No tilemap world-state summary/);
assert.match(dashboard.renderCameraLayerSummary(run.engine_summaries), /Active camera/);
assert.match(dashboard.renderCameraLayerSummary(run.engine_summaries), /&lt;follow-player&gt;/);
assert.match(dashboard.renderCameraLayerSummary(run.engine_summaries), /parallax 50/);
assert.match(dashboard.renderCameraLayerSummary(run.engine_summaries), /camera disabled/);
assert.match(dashboard.renderCameraLayerSummary(run.engine_summaries), /World-to-screen samples/);
assert.match(dashboard.renderCameraLayerSummary(run.engine_summaries), /Read-only camera\/layer evidence/);
assert.doesNotMatch(dashboard.renderCameraLayerSummary(run.engine_summaries), /<follow-player>/);
assert.match(dashboard.renderCameraLayerSummary({
  present: true,
  camera: {
    activeCameraId: 'legacy-camera',
    cameras: [],
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
}), /3D camera records/);
assert.match(dashboard.renderCameraLayerSummary({
  present: true,
  camera: {
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
}), /&lt;main-camera&gt;/);
assert.doesNotMatch(dashboard.renderCameraLayerSummary({
  present: true,
  camera: {
    scene3dCamera: {
      present: true,
      activeCameraId: '<main-camera>',
      cameras: [{ id: '<main-camera>', projection: { kind: '<script>' }, viewport: {} }]
    }
  }
}), /<script>/);
assert.match(dashboard.renderCameraLayerSummary({}), /No camera\/layer read model/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /Renderable elements/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /Render queue/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /Queue status/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /Tilemap draw tiles/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /Asset-backed tiles/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /Missing tile refs/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /3D render smoke/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /3D smoke visible\/skipped/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /1\/1/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /&lt;scene3d:cube&gt;/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /no WebGPU, GLTF import, PBR/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /tiles 2/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /&lt;queue:player&gt;/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /&lt;entity:player&gt;/);
assert.match(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /trusted writes, command bridge, live mutation/);
assert.doesNotMatch(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /<entity:player>/);
assert.doesNotMatch(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /<queue:player>/);
assert.doesNotMatch(dashboard.renderRenderBreakdownSummary(run.engine_summaries), /<scene3d:cube>/);
const xssScene3dRenderDashboard = dashboard.renderRenderBreakdownSummary({
  present: true,
  render_breakdown: { present: true, frameId: 'frame', elements: [] },
  scene3d_render: {
    present: true,
    frameId: '<script>frame</script>',
    sceneId: '<img>',
    cameraId: '<svg>',
    renderables: [{ id: '<script>cube</script>', nodeId: '<img>', meshRef: '<svg>', materialRef: '<b>', primitive: '<i>', cameraId: '<p>', visible: false, fallbackReason: '<script>skip</script>' }],
    fallbackReasons: ['<script>fallback</script>'],
    boundary: '<script>boundary</script>',
  },
});
assert.doesNotMatch(xssScene3dRenderDashboard, /<script>|<img|<svg>|<b>|<i>|<p>/);
assert.match(xssScene3dRenderDashboard, /&lt;script&gt;cube&lt;\/script&gt;/);
assert.match(dashboard.renderRenderBreakdownSummary({}), /No scene render breakdown evidence/);
assert.match(dashboard.renderGameplaySummary(run.engine_summaries), /Declared flags/);
assert.match(dashboard.renderGameplaySummary(run.engine_summaries), /2 true \/ 1 false/);
assert.match(dashboard.renderGameplaySummary(run.engine_summaries), /HUD value components/);
assert.match(dashboard.renderGameplaySummary(run.engine_summaries), /Goal: Collect coin/);
assert.match(dashboard.renderGameplaySummary({}), /No trigger\/flag world-state summary/);
assert.match(dashboard.renderAnimationVfxSummary(run.engine_summaries), /Animated entities/);
assert.match(dashboard.renderAnimationVfxSummary(run.engine_summaries), /VFX events/);
assert.match(dashboard.renderAnimationVfxSummary(run.engine_summaries), /&lt;run-dust&gt;/);
assert.match(dashboard.renderAnimationVfxSummary(run.engine_summaries), /state &lt;run&gt;/);
assert.doesNotMatch(dashboard.renderAnimationVfxSummary(run.engine_summaries), /<run-dust>|<run>/);
assert.match(dashboard.renderAnimationVfxSummary({}), /No animation\/VFX read model/);
assert.match(dashboard.renderAudioEvidenceSummary(run.engine_summaries), /Audio intent events/);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /Runtime|Frame|Status|Budget violations/);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /&lt;frame-0003&gt;/);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /&lt;renderMs&gt;/);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /1 contact \/ 1 trigger \/ 1 invalid/);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /&lt;missing collider&gt;/);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /no full 3D physics engine/);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /browser observations are evidence inputs, not trusted authority/i);
assert.match(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /remote telemetry/);
assert.doesNotMatch(dashboard.renderRuntimeProfilerSummary(run.engine_summaries), /<frame-0003>|<renderMs>|<missing/);
assert.match(dashboard.renderRuntimeProfilerSummary({}), /No runtime profiler\/frame-budget read model/);
// present engine summaries without any profiler read model must still render the
// absence state, not a default within-budget grid.
assert.match(dashboard.renderRuntimeProfilerSummary({ present: true }), /No runtime profiler\/frame-budget read model/);
assert.doesNotMatch(dashboard.renderRuntimeProfilerSummary({ present: true }), /within-budget|Budget violations/);
const xssProfilerSummary = dashboard.renderRuntimeProfilerSummary({ present: true, runtime_frame_budget: { frameId: '<script>frame</script>', sceneId: '<img>', scenarioId: '<svg>', timings: { renderMs: '<b>' }, budget: { renderMs: '<i>' }, counts: { entityCount: '<p>' }, status: '<script>bad</script>', violations: [{ field: '<script>render</script>', actualMs: '<img>', budgetMs: '<svg>' }], readOnlyInspection: { disallowedActions: ['<script>write</script>'] }, authority: '<b>authority</b>' } });
assert.doesNotMatch(xssProfilerSummary, /<script>|<img>|<svg>|<b>|<i>|<p>/);
assert.match(xssProfilerSummary, /&lt;script&gt;render&lt;\/script&gt;/);
assert.match(dashboard.renderAudioEvidenceSummary(run.engine_summaries), /Browser limitation warnings/);
assert.match(dashboard.renderAudioEvidenceSummary(run.engine_summaries), /&lt;sfx&gt;/);
assert.match(dashboard.renderAudioEvidenceSummary(run.engine_summaries), /&lt;audible_output_not_verified&gt;/);
assert.doesNotMatch(dashboard.renderAudioEvidenceSummary(run.engine_summaries), /<sfx>|<audible/);
assert.match(dashboard.renderAudioEvidenceSummary({}), /No audio intent evidence/);
assert.match(dashboard.renderInputActionSummary(run.engine_summaries), /input action evidence/);
assert.match(dashboard.renderInputActionSummary(run.engine_summaries), /3/);
assert.match(dashboard.renderInputActionSummary(run.engine_summaries), /&lt;dash&gt;/);
assert.match(dashboard.renderInputActionSummary(run.engine_summaries), /&lt;move_right&gt; \/ &lt;dash&gt;/);
assert.match(dashboard.renderInputActionSummary(run.engine_summaries), /trusted writes, command bridge, live mutation/);
assert.doesNotMatch(dashboard.renderInputActionSummary(run.engine_summaries), /<dash>/);
assert.match(dashboard.renderInputActionSummary({}), /No input action read model/);
assert.match(dashboard.renderJournalViewer({ ...run, journal_view: { path: 'journal.md', exists: false, read_error: 'missing journal artifact', entries: [] } }), /missing journal artifact/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: 'missing', stages: [], command_hints: [] } }), /No mutation lifecycle stages/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: '<script>', command_hints: [], stages: [{ id: 'scene_applied', label: '<img>', state: '<bad>', artifact_path: 'mutation/scene-applications.json', record_count: 1, records: [{ id: '<script>', project: { projectId: '<img>', manifestPath: '<script>', manifestHash: { algorithm: '<b>', value: '<i>' }, scenePath: '<p>', sceneHash: { algorithm: '<u>', value: '<em>' } }, rollback: { scenePath: '<svg>', restoreHash: { value: '<hash>' } } }] }] } }), /&lt;script&gt;/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: 'applied', command_hints: [], stages: [{ id: 'visual_draft_applied', label: 'Applied visual draft', state: 'applied', artifact_path: 'mutation/visual-edit-applications.json', record_count: 1, records: [{ id: 'visual-edit-application-1', draftId: '<draft>', proposalId: 'proposal-1', patchDraftId: 'patch-draft-1', reviewDecisionId: 'review-decision-1', transactionId: 'scene-edit-1', transactionArtifactPath: 'mutation/tx.json', targetScenePath: 'scenes/main.scene.json', beforeSceneHash: { value: 'before' }, afterSceneHash: { value: 'after' }, commandContext: { command: 'cargo run -p ouroforge-cli -- edit draft-apply draft.json' } }] }] } }), /Visual draft application/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: 'applied', command_hints: [], stages: [{ id: 'visual_draft_applied', label: 'Applied visual draft', state: 'applied', artifact_path: 'mutation/visual-edit-applications.json', record_count: 1, records: [{ id: 'visual-edit-application-1', draftId: '<draft>', proposalId: 'proposal-1', patchDraftId: 'patch-draft-1', reviewDecisionId: 'review-decision-1', transactionId: 'scene-edit-1', transactionArtifactPath: 'mutation/tx.json', targetScenePath: 'scenes/main.scene.json', beforeSceneHash: { value: 'before' }, afterSceneHash: { value: 'after' }, commandContext: { command: 'cargo run -p ouroforge-cli -- edit draft-apply draft.json' } }] }] } }), /Display-only rerun context/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: 'applied', command_hints: [], stages: [{ id: 'visual_draft_applied', label: 'Applied visual draft', state: 'applied', artifact_path: 'mutation/visual-edit-applications.json', record_count: 1, records: [{ id: 'visual-edit-application-1', draftId: '<draft>', proposalId: 'proposal-1', patchDraftId: 'patch-draft-1', reviewDecisionId: 'review-decision-1', transactionId: 'scene-edit-1', transactionArtifactPath: 'mutation/tx.json', targetScenePath: 'scenes/main.scene.json', beforeSceneHash: { value: 'before' }, afterSceneHash: { value: 'after' }, commandContext: { command: 'cargo run -p ouroforge-cli -- edit draft-apply draft.json' } }] }] } }), /&lt;draft&gt;/);
assert.match(dashboard.renderRegressionPromotions({ regression_promotions: [] }), /No regression promotion records/);
assert.match(dashboard.renderRegressionPromotions({ regression_promotions: [{ id: '<script>', scenarioId: '<img>', target: { scenarioPackId: '<svg>', scenarioPackPath: '<b>' }, beforeHash: { value: '<before>' }, afterHash: { value: '<after>' }, recordPath: '<record>' }] }), /&lt;script&gt;/);
assert.match(detail, /Regression Run Matrix/);
assert.match(detail, /scaffold-smoke/);
assert.match(detail, /legacy or malformed run\(s\) skipped/);
assert.ok(detail.includes('../../runs/run-1/evidence/scenarios/scaffold-smoke/scenario-result.json'));
assert.match(dashboard.renderRegressionMatrix(run.regression_matrix), /Read-only local evidence projection/);
assert.match(dashboard.renderRegressionMatrix({ projects: [{ projectId: '<script>', projectName: '<img>', scenarioPacks: [{ scenarioPackId: '<svg>', scenarioPackPath: '<b>', scenarios: [{ scenarioId: '<scenario>', currentStatus: '<bad>', runs: [], context: { mutationIds: ['<m>'], reviewDecisionIds: [], promotionIds: [] } }] }] }], skippedRuns: [] }), /&lt;script&gt;/);
assert.match(dashboard.renderRegressionMatrix(null), /No regression matrix export/);
assert.match(dashboard.renderReplayControls({ replay: { present: false, empty_state: 'no replay fixture', sequences: [] } }), /no replay fixture/);
assert.match(dashboard.renderRunComparison({ comparison: { present: false, empty_state: 'no comparison fixture', artifacts: [] } }), /no comparison fixture/);
assert.match(dashboard.renderSemanticDiffSummary({}), /No semantic diff section/);
assert.match(dashboard.renderSemanticDiffSummary({ value: { semantic: { reasons: [{ kind: 'fallback', severity: 'changed', summary: 'fallback semantic' }] } } }), /fallback semantic/);
assert.match(dashboard.renderSemanticDiffSummary({ value: { semantic: { reasons: [], project: { relation: 'legacy', changed: false, changes: [] } } } }), /No project context changes recorded/);
assert.match(dashboard.renderSemanticDiffSummary({ value: { semantic: { reasons: [], project: '<bad>' } } }), /No project comparison fields/);
assert.match(dashboard.renderTransactionProvenance({}), /No scene edit transaction provenance/);
assert.match(dashboard.renderProjectContext({}), /No project workspace metadata/);
assert.match(dashboard.renderProjectContext(run), /Scenario pack/);
assert.match(dashboard.renderCommandContext({}), /No run command context/);
assert.match(dashboard.renderCommandContext(run), /openchrome_cdp/);
assert.equal(dashboard.comparisonRefHref('runs/before-run/verdict.json', run), '../../runs/before-run/verdict.json');
assert.equal(dashboard.comparisonRefHref('evidence/world.json', run), '../../runs/run-1/evidence/world.json');
assert.match(dashboard.renderRunList([], null), /No runs found/);

let replayState = dashboard.createReplayState(run);
let replayView = dashboard.currentReplayView(run, replayState);
assert.equal(replayView.frame, 0);
assert.equal(replayView.checkpoint.world_state.entities[0].components.transform.x, 32);
replayState = dashboard.stepReplayForward(run, replayState);
replayView = dashboard.currentReplayView(run, replayState);
assert.equal(replayView.frame, 4);
assert.equal(replayView.checkpoint.tick, 4);
assert.equal(replayView.checkpoint.world_state.entities[0].components.transform.x, 40);
replayState = dashboard.resetReplay(run);
assert.equal(dashboard.currentReplayView(run, replayState).frame, 0);
replayState = dashboard.jumpReplayToCheckpoint(run, replayState, 1);
assert.equal(dashboard.currentReplayView(run, replayState).frame, 4);
assert.match(dashboard.renderReplayControls(run, replayState), /Current tick/);
assert.match(dashboard.renderAssetIntegrity(run), /Asset reference|Warnings|stale_asset_hash/);
assert.match(dashboard.renderAssetLoading(run), /Runtime asset loading evidence refs/);
assert.match(dashboard.renderAssetLoading(run), /player_sprite/);
assert.match(dashboard.renderAssetLoading(run), /Image load failed/);
assert.match(dashboard.renderAssetPreview(run), /Asset preview evidence refs/);
assert.match(dashboard.renderAssetPreview(run), /player_atlas/);
assert.match(dashboard.renderAssetPreview(run), /missing_asset_file/);
assert.match(dashboard.renderPluginRegistry(run), /Plugin registry evidence refs/);
assert.match(dashboard.renderPluginRegistry(run), /read-only-dashboard-panel/);
assert.match(dashboard.renderPluginRegistry(run), /fnv1a64-canonical-json-v1:1111222233334444/);
assert.match(dashboard.renderPluginRegistry(run), /dashboardPanel/);
assert.match(dashboard.renderPluginRegistry(run), /studio.inspector.readOnly/);
assert.match(dashboard.renderPluginRegistry(run), /executable command authority/);
assert.match(dashboard.renderSourceApplyWorktreeContext(run), /Source apply context evidence refs/);
assert.match(dashboard.renderSourceApplyWorktreeContext(run), /source-apply-worktree-boundary-v1/);
assert.match(dashboard.renderSourceApplyWorktreeContext(run), /dirty-target/);
assert.match(dashboard.renderSourceApplyWorktreeContext(run), /browser\/dashboard\/Studio surfaces remain read-only/);
assert.match(dashboard.renderRuntimeInvariants(run), /Runtime invariant evidence refs/);
assert.match(dashboard.renderRuntimeInvariants(run), /health was negative/);
assert.match(dashboard.renderRuntimeInvariants(run), /qa14_5_runtime_dashboard/);
assert.match(dashboard.renderRouteAttempts(run), /Route attempt evidence refs/);
assert.match(dashboard.renderRouteAttempts(run), /qa14_6_collect_goal_route/);
assert.match(dashboard.renderRouteAttempts(run), /collect-goal-then-exit/);
assert.match(dashboard.renderQaScenarioCandidates(run), /QA scenario candidate refs/);
assert.match(dashboard.renderQaScenarioCandidates(run), /qa14_2_collect_and_exit_gap/);
assert.match(dashboard.renderQaScenarioCandidates(run), /collect-goal-then-exit/);
assert.match(dashboard.renderFuzzingPlans(run), /Adversarial input fuzzing plan refs/);
assert.match(dashboard.renderFuzzingPlans(run), /qa14_3_seeded_movement_fuzz/);
assert.match(dashboard.renderFuzzingPlans(run), /seed 424242/);
assert.match(dashboard.renderQaWorkerAssignments(run), /QA worker assignment refs/);
assert.match(dashboard.renderQaWorkerAssignments(run), /budget gate pending/);
assert.match(dashboard.renderQaWorkerAssignments(run), /qa-worker-1/);
assert.doesNotMatch(dashboard.renderQaWorkerAssignments(run), /<script>worker<\/script>/);
assert.doesNotMatch(dashboard.renderRuntimeInvariants(run), /<script>bad<\/script>/);
assert.doesNotMatch(dashboard.renderSourceApplyWorktreeContext(run), /<script>bad<\/script>/);
assert.doesNotMatch(dashboard.renderSourceApplyWorktreeContext(run), /<button|onclick|fetch\(/i);
assert.match(dashboard.renderRunDetail(run), /Asset reference integrity/);
assert.match(dashboard.renderRunDetail(run), /Runtime asset loading/);
assert.match(dashboard.renderRunDetail(run), /Asset preview evidence/);
assert.match(dashboard.renderRunDetail(run), /Plugin registry browser/);
assert.match(dashboard.renderRunDetail(run), /Source apply worktree context/);
assert.match(dashboard.renderRunDetail(run), /Runtime invariant evidence/);
assert.match(dashboard.renderRunDetail(run), /Route attempt evidence/);
assert.match(dashboard.renderRunDetail(run), /Visual comparison evidence/);
assert.match(dashboard.renderRunDetail(run), /QA scenario candidates/);
assert.match(dashboard.renderRunDetail(run), /Adversarial input fuzzing plans/);
assert.match(dashboard.renderRunDetail(run), /QA worker assignments/);
assert.match(dashboard.renderRunDetail(run), /stale_asset_hash/);
assert.match(dashboard.renderAssetIntegrity({ asset_integrity: { present: false, empty_state: 'No integrity evidence' } }), /No integrity evidence/);
assert.match(dashboard.renderAssetLoading({ asset_loading: { present: false, empty_state: 'No loading evidence' } }), /No loading evidence/);
assert.match(dashboard.renderAssetPreview({ asset_preview: { present: false, empty_state: 'No preview evidence' } }), /No preview evidence/);
assert.match(dashboard.renderPluginRegistry({ plugin_registry: { present: false, empty_state: 'No plugin evidence' } }), /No plugin evidence/);
assert.match(dashboard.renderSourceApplyWorktreeContext({ source_apply_worktree_context: { present: false, empty_state: 'No context evidence' } }), /No context evidence/);
assert.match(dashboard.renderRuntimeInvariants({ runtime_invariants: { present: false, empty_state: 'No invariant evidence' } }), /No invariant evidence/);
assert.match(dashboard.renderRouteAttempts({ route_attempts: { present: false, empty_state: 'No route attempt evidence' } }), /No route attempt evidence/);
assert.match(dashboard.renderVisualComparisons({ visual_comparisons: { present: false, empty_state: 'No visual comparison evidence' } }), /No visual comparison evidence/);
assert.match(dashboard.renderVisualComparisons(run), /visual_regression_candidate/);
assert.match(dashboard.renderVisualComparisons(run), /collect-and-exit/);
assert.match(dashboard.renderVisualComparisons(run), /must not compute trusted diffs/);
assert.match(dashboard.renderQaScenarioCandidates({ qa_scenario_candidates: { present: false, empty_state: 'No candidate evidence' } }), /No candidate evidence/);
assert.match(dashboard.renderFuzzingPlans({ fuzzing_plans: { present: false, empty_state: 'No fuzzing plan evidence' } }), /No fuzzing plan evidence/);
assert.match(dashboard.renderQaWorkerAssignments({ qa_worker_assignments: { present: false, empty_state: 'No worker assignment evidence' } }), /No worker assignment evidence/);

const visualAuthoringDoc = fs.readFileSync(require.resolve('../../docs/visual-authoring-v1.md'), 'utf8');
assert.match(visualAuthoringDoc, /Scenario Coverage v5 \/ VA1\.11\.3 coverage matrix/);
assert.match(visualAuthoringDoc, /Review-gated visual apply/);
assert.match(visualAuthoringDoc, /Dashboard mutation lifecycle and Studio mutation surfaces show draft\/proposal\/patch\/decision\/transaction ids/);
assert.match(visualAuthoringDoc, /Known gaps and out-of-scope behavior/);
assert.match(visualAuthoringDoc, /committed generated runs, transactions, previews, dashboard exports, smoke\s+outputs, screenshots, logs, or package bundles/);
assert.match(visualAuthoringDoc, /broader editor ergonomics, richer visual diff UI affordances, production\s+asset import, source mutation apply/);
assert.match(visualAuthoringDoc, /node examples\/evidence-dashboard\/dashboard\.test\.cjs/);

// Untrusted artifact/journal content must be HTML-escaped, not rendered as markup.
const xssRun = {
  summary: { id: '<img src=x onerror=alert(1)>', run_dir: 'runs/x', seed_id: 's', run_status: 'created', verdict_status: 'failed', scenario_status: 'pending', evidence_count: 0, mutation_count: 0, worker_count: 0 },
  command_context: { command: '<script>alert(1)</script>', argv: ['<img src=x onerror=alert(1)>'], seedPath: '<script>seed</script>', workers: '<script>workers</script>', runsRoot: 'runs', scenarioPackId: '<script>pack</script>', runtimeTarget: '<script>runtime</script>', browserBoundary: '<script>boundary</script>', cdpTransport: '<script>transport</script>', environmentHints: ['<script>hint</script>'] },
  evidence: [], screenshots: [], world_states: [], frame_metrics: [], performance_metrics: [{ id: '<script>perf</script>', kind: 'application/json', path: 'evidence/<script>perf</script>.json', value: null, read_error: '<script>bad perf</script>', metadata: { worker_id: '<script>worker</script>', execution_boundary: '<script>boundary</script>' } }], console_logs: [{ id: '<script>console</script>', kind: 'application/json', path: 'evidence/<script>console</script>.json', value: [{ text: '<script>log</script>' }], metadata: { worker_session_id: '<img src=x onerror=alert(1)>', cdp_transport: '<script>transport</script>' } }], cdp_trace_summaries: [], scenario_results: [], mutation_artifacts: [], mutations: [],
  mutation_lifecycle: { terminal_state: '<img>', stages: [{ id: 'x', label: '<img>', state: '<script>', artifact_path: '<b>', record_count: 0, evidence_refs: [], records: [] }], command_hints: ['<script>alert(1)</script>'] },
  regression_promotions: [{ id: '<script>', scenarioId: '<img>', target: { scenarioPackId: '<svg>', scenarioPackPath: '<b>' }, beforeHash: { value: '<before>' }, afterHash: { value: '<after>' }, recordPath: '<record>' }],
  replay: { present: true, empty_state: '', sequences: [{ id: '<script>', source: '<img>', event_count: 1, frames: [0], evidence_refs: ['<script>'], checkpoints: [{ label: '<img>', frame: 0, tick: 0, world_state_path: '<b>', world_state: { unsafe: '<script>alert(1)</script>' } }] }] },
  asset_integrity: { present: true, warning_count: 1, stale_hash_count: 1, missing_ref_count: 0, invalid_type_count: 0, evidence_refs: ['javascript:alert(1)'], warnings: [{ kind: '<script>alert(1)</script>', assetId: '<img src=x onerror=alert(1)>', message: '<script>alert(1)</script>', path: '<b>bad</b>' }] },
  asset_loading: { present: true, attempt_count: 1, loaded_count: 0, failed_count: 1, rejected_count: 0, fallback_count: 0, evidence_refs: ['javascript:alert(1)'], boundary: '<script>boundary</script>', records: [{ attemptId: '<script>attempt</script>', assetId: '<img src=x onerror=alert(1)>', path: '<b>bad</b>', status: '<script>failed</script>', failureReason: '<script>reason</script>' }] },
  asset_preview: { present: true, preview_count: 1, warning_count: 1, evidence_refs: ['javascript:alert(1)'], boundary: '<script>preview-boundary</script>', records: [{ assetId: '<img src=x onerror=alert(1)>', assetType: '<script>type</script>', sourcePath: '<b>bad</b>', previewKind: '<script>kind</script>', image: { width: '<script>', height: '<img>' } }], warnings: [{ assetId: '<img>', kind: '<script>warning</script>', message: '<script>preview reason</script>', path: '<b>bad</b>' }] },
  source_apply_worktree_context: { present: true, status: '<script>blocked</script>', target_count: 1, blocked_count: 1, evidence_refs: ['javascript:alert(1)'], boundary: '<script>context-boundary</script>', reports: [{ status: '<script>blocked</script>', policyId: '<script>policy</script>', branch: '<script>branch</script>', headCommit: '<script>head</script>', worktreeRoot: '<b>worktree</b>', lockStatus: { active: true, attemptId: '<script>attempt</script>' }, blockedReasons: ['<script>blocked reason</script>'], guardrails: ['<script>guardrail</script>'], targets: [{ path: '<img src=x onerror=alert(1)>', gitStatus: '<script>modified</script>', rootZone: '<script>root</script>', fileClassDecision: '<script>allowed</script>', blockedReasons: ['<script>target reason</script>'] }] }] },
  route_attempts: {
    present: true,
    status: 'passed',
    attempt_count: 1,
    passed_count: 1,
    failed_count: 0,
    blocked_count: 0,
    inconclusive_count: 0,
    unsupported_count: 0,
    malformed_count: 0,
    evidence_refs: ['evidence/route-attempts/route-attempt.json', 'evidence/scenarios/collect-and-exit/world-state-start.json'],
    boundary: 'Read-only route attempt evidence; dashboard must not run solvers.',
    attempts: [{
      attemptId: 'qa14_6_collect_goal_route',
      runId: 'run-1',
      objectiveId: 'collect-goal-then-exit',
      scenarioId: 'collect-and-exit',
      startState: { stateId: 'start-left-of-goal', worldStateRef: 'evidence/scenarios/collect-and-exit/world-state-start.json' },
      strategyKind: 'simple_heuristic',
      outcome: 'passed',
      budgetUsed: { maxActions: 8, actionsUsed: 2, maxRouteNodes: 8, routeNodesUsed: 2 },
    }],
  },
  qa_scenario_candidates: {
    present: true,
    status: '<script>proposed</script>',
    boundary: '<script>candidate-boundary</script>',
    evidence_refs: ['javascript:alert(1)'],
    candidates: [{
      candidateId: '<script>candidate</script>',
      priority: '<script>priority</script>',
      status: '<script>status</script>',
      sourceRisk: { riskId: '<script>risk</script>' },
      targetObjective: { objectiveId: '<script>objective</script>', description: '<script>description</script>' },
      inputStrategy: { kind: '<script>input</script>' },
      budget: { maxRuns: '<script>runs</script>' },
      expectedEvidence: [{ evidenceId: '<script>evidence</script>' }],
    }],
  },

  qa_agent_work_queues: {
    present: true,
    status: 'needs-rerun',
    queue_count: 1,
    item_count: 1,
    passed_count: 0,
    failed_count: 0,
    deferred_count: 0,
    blocked_count: 0,
    flaky_count: 0,
    needs_rerun_count: 1,
    malformed_count: 0,
    evidence_refs: [
      'evidence/qa-agent-work-queue/queue.json',
      'examples/scenario-packs/project-smoke.scenarios.json',
      'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json',
      'runs/multi-agent-pipeline/demo/qa/old-scenario-result.json',
    ],
    boundary: 'Read-only QA agent work queues; dashboard must not execute queue commands.',
    queues: [{
      schemaVersion: 'qa-agent-work-queue-v1',
      queueId: '<qa-queue>',
      milestone: 'multi-agent-production-pipeline-v1',
      items: [{
        queueItemId: '<qa-item>',
        scenarioTarget: { scenarioId: '<scenario>', scenarioPackRef: { id: 'scenario-pack', path: 'examples/scenario-packs/project-smoke.scenarios.json' } },
        riskArea: { riskId: '<risk>', category: 'scenario-regression', summary: 'Escaped <risk> summary.' },
        runCommandContext: { command: 'cargo run -p ouroforge-cli -- run <seed> --scenario-pack smoke', argv: [], boundary: 'Inert command text only; does not execute.' },
        expectedEvidence: [{ id: 'scenario-result', path: 'runs/multi-agent-pipeline/demo/qa/scenario-result.json' }, { id: 'evaluator-summary', path: 'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json' }],
        priority: 'high',
        assignedRole: 'qa-agent',
        assignedAgent: '<agent>',
        status: 'needs-rerun',
        failureClassification: 'stale-run-ref',
        taskRef: { id: 'task-board', path: 'examples/multi-agent-pipeline-v1/production-task-board.fixture.json' },
        workPackageRef: { id: 'work-package', path: 'examples/multi-agent-pipeline-v1/agent-work-package.valid.fixture.json' },
        reviewGateRef: { id: 'review-gate', path: 'examples/multi-agent-pipeline-v1/review-critic-gate.valid.fixture.json' },
        runEvidenceRefs: [{ id: 'run', path: 'runs/multi-agent-pipeline/demo/qa/scenario-result.json' }],
        evaluatorEvidenceRefs: [{ id: 'evaluator', path: 'runs/multi-agent-pipeline/demo/qa/evaluator-summary.json' }],
        blockedReasons: ['refresh <stale> run evidence'],
        staleRunRefs: ['runs/multi-agent-pipeline/demo/qa/old-scenario-result.json'],
      }],
      forbiddenActions: ['browser command bridge', 'auto-merge'],
      boundary: 'QA agent work queue is inert local evidence; it does not execute commands.',
    }],
  },
  fuzzing_plans: {
    present: true,
    status: '<script>planned</script>',
    boundary: '<script>fuzz-boundary</script>',
    evidence_refs: ['javascript:alert(1)'],
    plans: [{
      planId: '<script>plan</script>',
      status: '<script>status</script>',
      deterministicSeed: '<script>seed</script>',
      inputDomain: { scenarioId: '<script>scenario</script>' },
      budget: { maxRuns: '<script>runs</script>', maxSteps: '<script>steps</script>' },
      outputRoot: '<script>output</script>',
      cleanupPolicy: { mode: '<script>cleanup</script>' },
      expectedEvidence: [{ evidenceId: '<script>evidence</script>' }],
    }],
  },
  journal_view: { path: 'journal.md', exists: true, summary: '<b>unsafe</b>', entries: [{ heading: '<img>', category: 'summary', body: '<script>alert(1)</script>', evidence_refs: [], verdict_refs: [], mutation_refs: [] }], evidence_refs: [], verdict_refs: [], mutation_refs: [] },
  comparison: { present: true, empty_state: '', artifacts: [{ id: '<img>', path: 'mutation/<script>.json', exists: true, read_error: '<script>alert(1)</script>', before_run_id: '<script>', after_run_id: '<img>', classification: '<script>', deltas: { '<script>': '<img>' }, evidence_refs: ['javascript:alert(1)', '<script>'], unsupported: ['<script>alert(1)</script>'], value: { unsafe: '<script>alert(1)</script>' } }] },
  verdict: {}, journal: '<script>alert(1)</script>',
};
const xssDetail = dashboard.renderRunDetail(xssRun);
assert.ok(!xssDetail.includes('<script>alert(1)</script>'), 'journal markup must be escaped');
assert.match(xssDetail, /&lt;script&gt;/);
assert.ok(!xssDetail.includes('<img>'), 'journal headings must be escaped');
assert.ok(!xssDetail.includes('<script>worker</script>'), 'artifact metadata must be escaped');
assert.ok(!xssDetail.includes('<img src=x onerror=alert(1)>'), 'artifact session metadata must be escaped');
assert.ok(!xssDetail.includes('<script>hint</script>'), 'command context hints must be escaped');
assert.ok(!xssDetail.includes('<script>reason</script>'), 'asset loading reason must be escaped');
assert.ok(!xssDetail.includes('<script>boundary</script>'), 'asset loading boundary must be escaped');
assert.ok(!xssDetail.includes('<script>preview reason</script>'), 'asset preview warning must be escaped');
assert.ok(!xssDetail.includes('<script>preview-boundary</script>'), 'asset preview boundary must be escaped');
assert.ok(!xssDetail.includes('<script>context-boundary</script>'), 'source apply context boundary must be escaped');
assert.ok(!xssDetail.includes('<script>target reason</script>'), 'source apply context reasons must be escaped');
assert.ok(!dashboard.renderRunList([xssRun], null).includes('<img src=x onerror'), 'run id markup must be escaped');
const sourceApplyXss = dashboard.renderSourceApplyWorktreeContext({ source_apply_worktree_context: { present: true, status: '<script>blocked</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], reports: [{ policyId: '<img src=x onerror=alert(1)>', status: '<script>bad</script>', branch: '<script>branch</script>', headCommit: '<script>head</script>', worktreeRoot: '<script>root</script>', lockStatus: { active: true, attemptId: '<script>lock</script>' }, blockedReasons: ['<script>blocked</script>'], guardrails: ['<script>guardrail</script>'], targets: [{ path: '<img src=x onerror=alert(1)>', gitStatus: '<script>dirty</script>', rootZone: '<script>root</script>', fileClassDecision: '<script>decision</script>', blockedReasons: ['<script>target</script>'] }] }] } });
assert.ok(!sourceApplyXss.includes('<script>blocked</script>'), 'source apply blocked reasons must be escaped');
assert.ok(!sourceApplyXss.includes('<img src=x onerror=alert(1)>'), 'source apply target paths must be escaped');
const invariantXss = dashboard.renderRuntimeInvariants({ runtime_invariants: { present: true, status: '<script>bad</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], summaries: [{ modelId: '<script>model</script>', runId: '<script>run</script>' }], evidence: [{ checks: [{ invariantId: '<script>check</script>', invariantType: '<script>type</script>', status: '<script>status</script>', targetPath: '<script>target</script>', message: '<script>message</script>' }] }] } });
assert.ok(!invariantXss.includes('<script>check</script>'), 'runtime invariant check ids must be escaped');
assert.ok(!invariantXss.includes('<script>boundary</script>'), 'runtime invariant boundary must be escaped');
const routeAttemptXss = dashboard.renderRouteAttempts({ route_attempts: { present: true, status: '<script>bad</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], attempts: [{ attemptId: '<script>attempt</script>', outcome: '<script>outcome</script>', strategyKind: '<script>strategy</script>', objectiveId: '<script>objective</script>', scenarioId: '<script>scenario</script>', startState: { stateId: '<script>state</script>' }, budgetUsed: { actionsUsed: '<script>actions</script>' }, blockers: [{ reason: '<script>blocked</script>' }] }] } });
assert.ok(!routeAttemptXss.includes('<script>attempt</script>'), 'route attempt ids must be escaped');
assert.ok(!routeAttemptXss.includes('<script>outcome</script>'), 'route attempt outcome must be escaped');
assert.ok(!routeAttemptXss.includes('<script>strategy</script>'), 'route attempt strategy must be escaped');
assert.ok(!routeAttemptXss.includes('<script>state</script>'), 'route attempt start state must be escaped');
assert.ok(!routeAttemptXss.includes('<script>actions</script>'), 'route attempt budget usage must be escaped');
assert.ok(!routeAttemptXss.includes('<script>blocked</script>'), 'route attempt blocker reason must be escaped');
assert.ok(!routeAttemptXss.includes('<script>boundary</script>'), 'route attempt boundary must be escaped');
assert.match(routeAttemptXss, /&lt;script&gt;attempt&lt;\/script&gt;/);
const visualComparisonXss = dashboard.renderVisualComparisons({ visual_comparisons: { present: true, status: '<script>bad</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], summaries: [{ comparisonId: '<script>comparison</script>', scenarioId: '<script>scenario</script>', checkpointId: '<script>checkpoint</script>', outcome: '<script>outcome</script>', failureClassification: '<script>classification</script>', beforeScreenshotRef: '<script>before</script>', afterScreenshotRef: '<script>after</script>', changedPixels: '<script>pixels</script>', changedPercentX1000: '<script>percent</script>', changedRegionCount: '<script>regions</script>' }] } });
assert.ok(!visualComparisonXss.includes('<script>comparison</script>'), 'visual comparison ids must be escaped');
assert.ok(!visualComparisonXss.includes('<script>classification</script>'), 'visual comparison classification must be escaped');
assert.ok(!visualComparisonXss.includes('<script>boundary</script>'), 'visual comparison boundary must be escaped');
assert.match(visualComparisonXss, /&lt;script&gt;comparison&lt;\/script&gt;/);
const candidateXss = dashboard.renderQaScenarioCandidates({ qa_scenario_candidates: { present: true, status: '<script>bad</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], candidates: [{ candidateId: '<script>candidate</script>', priority: '<script>priority</script>', status: '<script>status</script>', sourceRisk: { riskId: '<script>risk</script>' }, targetObjective: { objectiveId: '<script>objective</script>', description: '<script>description</script>' }, inputStrategy: { kind: '<script>input</script>' }, budget: { maxRuns: '<script>runs</script>' }, blockedReasons: ['<script>blocked</script>'], expectedEvidence: [{ evidenceId: '<script>evidence</script>' }] }] } });
assert.ok(!candidateXss.includes('<script>candidate</script>'), 'scenario candidate ids must be escaped');
assert.ok(!candidateXss.includes('<script>boundary</script>'), 'scenario candidate boundary must be escaped');
const fuzzingPlanXss = dashboard.renderFuzzingPlans({ fuzzing_plans: { present: true, status: '<script>bad</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], plans: [{ planId: '<script>plan</script>', status: '<script>status</script>', deterministicSeed: '<script>seed</script>', inputDomain: { scenarioId: '<script>scenario</script>' }, budget: { maxRuns: '<script>runs</script>', maxSteps: '<script>steps</script>' }, outputRoot: '<script>output</script>', cleanupPolicy: { mode: '<script>cleanup</script>' }, blockedReasons: ['<script>blocked</script>'], expectedEvidence: [{ evidenceId: '<script>evidence</script>' }] }] } });
assert.ok(!fuzzingPlanXss.includes('<script>plan</script>'), 'fuzzing plan ids must be escaped');
assert.ok(!fuzzingPlanXss.includes('<script>boundary</script>'), 'fuzzing plan boundary must be escaped');
const qaWorkerXss = dashboard.renderQaWorkerAssignments({ qa_worker_assignments: { present: true, status: '<script>bad</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], plans: [{ planId: '<script>plan</script>', assignments: [{ assignmentId: '<script>assignment</script>', workerId: '<script>worker</script>', assignedLane: '<script>lane</script>', status: '<script>status</script>', target: { targetType: '<script>target</script>', targetId: '<script>target-id</script>' }, budget: { maxRuns: '<script>runs</script>' }, timeoutMs: '<script>timeout</script>', outputRoot: '<script>output</script>', cleanupPolicy: { mode: '<script>cleanup</script>' }, blockedReasons: ['<script>blocked</script>'] }] }] } });
assert.ok(!qaWorkerXss.includes('<script>assignment</script>'), 'qa worker assignment ids must be escaped');
assert.ok(!qaWorkerXss.includes('<script>boundary</script>'), 'qa worker assignment boundary must be escaped');
assert.match(sourceApplyXss, /&lt;script&gt;blocked&lt;\/script&gt;/);

const rawMalformedCommandContextRun = {
  summary: { id: 'raw-malformed', run_dir: 'runs/raw-malformed', verdict_status: 'passed' },
  run: { run_command_context: { schemaVersion: 7, command: 'untrusted raw command' } },
  evidence: [], screenshots: [], world_states: [], frame_metrics: [], performance_metrics: [], console_logs: [], cdp_trace_summaries: [], scenario_results: [], mutation_artifacts: [], mutations: [],
};
const rawMalformedCommandContextDetail = dashboard.renderRunDetail(rawMalformedCommandContextRun);
assert.match(rawMalformedCommandContextDetail, /No run command context is recorded/);
assert.ok(!rawMalformedCommandContextDetail.includes('untrusted raw command'), 'raw malformed run_command_context must not render');
console.log('dashboard smoke test passed');

assert.match(dashboard.renderLoopDryRunSummary(run.loop_dry_run), /Authoring loop dry-run/);
assert.match(dashboard.renderLoopDryRunSummary(run.loop_dry_run), /blocked/);
assert.match(dashboard.renderLoopDryRunSummary(run.loop_dry_run), /&lt;loop-1&gt;/);
assert.doesNotMatch(dashboard.renderLoopDryRunSummary(run.loop_dry_run), /<loop-1>/);
assert.match(dashboard.renderRunDetail(run), /Authoring loop dry-run/);
assert.match(dashboard.renderLoopDryRunSummary(null), /No dry-run summary/);
assert.match(dashboard.renderLoopExecutionSummary(run.loop_execution), /Authoring loop execution/);
assert.match(dashboard.renderLoopExecutionSummary(run.loop_execution), /completed/);
assert.match(dashboard.renderLoopExecutionSummary(run.loop_execution), /&lt;transaction&gt;/);
assert.doesNotMatch(dashboard.renderLoopExecutionSummary(run.loop_execution), /<transaction>/);
assert.match(dashboard.renderRunDetail(run), /Authoring loop execution/);
assert.match(dashboard.renderLoopExecutionSummary(null), /No loop execution summary/);
assert.match(dashboard.renderLoopRecoveryStatus(run.loop_recovery), /Authoring loop recovery/);
assert.match(dashboard.renderLoopRecoveryStatus(run.loop_recovery), /needs-recovery/);
assert.match(dashboard.renderLoopRecoveryStatus(run.loop_recovery), /&lt;missing comparison&gt;/);
assert.doesNotMatch(dashboard.renderLoopRecoveryStatus(run.loop_recovery), /<missing comparison>/);
assert.match(dashboard.renderRunDetail(run), /Authoring loop recovery/);
assert.match(dashboard.renderLoopRecoveryStatus(null), /No recovery status/);

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
const pipelineInspectionMarkup = dashboard.renderStudioMultiAgentPipelineInspection(studioPipelineInspection);
assert.match(pipelineInspectionMarkup, /Studio multi-agent pipeline inspection/);
assert.match(pipelineInspectionMarkup, /ownership &lt;review&gt; required/);
assert.match(pipelineInspectionMarkup, /reviewDecisions missing/);
assert.match(pipelineInspectionMarkup, /production_task_boards\[0\] must be object/);
assert.match(pipelineInspectionMarkup, /cloud orchestration/);
assert.doesNotMatch(pipelineInspectionMarkup, /<script>Decision<\/script>/);
assert.doesNotMatch(pipelineInspectionMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(dashboard.renderRunDetail(run), /Studio multi-agent pipeline inspection/);
assert.match(dashboard.renderStudioMultiAgentPipelineInspection(null), /No Studio multi-agent pipeline inspection/);

const productionTaskBoardFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-task-board.fixture.json', 'utf8'));
const blockedProductionTaskBoardFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-task-board.blocked.fixture.json', 'utf8'));
run.production_task_board = productionTaskBoardFixture;
const taskBoardMarkup = dashboard.renderProductionTaskBoards(productionTaskBoardFixture);
assert.match(taskBoardMarkup, /Production task board/);
assert.match(taskBoardMarkup, /multi-agent-production-v1/);
assert.match(taskBoardMarkup, /task-board-schema/);
assert.match(taskBoardMarkup, /ready-for-review/);
assert.match(taskBoardMarkup, /Read-only production task board/);
assert.doesNotMatch(taskBoardMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const blockedTaskBoardMarkup = dashboard.renderProductionTaskBoards(blockedProductionTaskBoardFixture);
assert.match(blockedTaskBoardMarkup, /ownership-policy/);
assert.match(blockedTaskBoardMarkup, /Waiting for #668/);
assert.match(dashboard.renderRunDetail(run), /Production task board/);
assert.match(dashboard.renderProductionTaskBoards(null), /No production task board/);

const ownershipPolicyConflictFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/ownership-policy.conflict.fixture.json', 'utf8'));
const ownershipPolicyEscalationFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/ownership-policy.escalation.fixture.json', 'utf8'));
run.ownership_policy = ownershipPolicyConflictFixture;
const ownershipPolicyMarkup = dashboard.renderOwnershipPolicies(ownershipPolicyConflictFixture);
assert.match(ownershipPolicyMarkup, /Ownership policy/);
assert.match(ownershipPolicyMarkup, /ownership-policy-conflict/);
assert.match(ownershipPolicyMarkup, /scene-write-a/);
assert.match(ownershipPolicyMarkup, /Conflicts with reviewer write hold/);
assert.match(ownershipPolicyMarkup, /Read-only ownership policy/);
assert.doesNotMatch(ownershipPolicyMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const escalationPolicyMarkup = dashboard.renderOwnershipPolicies(ownershipPolicyEscalationFixture);
assert.match(escalationPolicyMarkup, /independent reviewer and critic approval/);
assert.match(dashboard.renderRunDetail(run), /Ownership policy/);
assert.match(dashboard.renderOwnershipPolicies(null), /No file\/artifact ownership policy/);

const agentRoleModelFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/agent-roles.fixture.json', 'utf8'));
run.agent_role_model = agentRoleModelFixture;
const roleModelMarkup = dashboard.renderAgentRoleModels(agentRoleModelFixture);
assert.match(roleModelMarkup, /Agent role model/);
assert.match(roleModelMarkup, /designer/);
assert.match(roleModelMarkup, /build-release-candidate-agent/);
assert.match(roleModelMarkup, /no-self-review/);
assert.match(roleModelMarkup, /self-approval/);
assert.match(roleModelMarkup, /Read-only role accountability metadata/);
assert.doesNotMatch(roleModelMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(dashboard.renderAgentRoleModels({ schemaVersion: 'agent-role-model-v1', milestone: '<bad>', roles: '<bad>', separationRequirements: [] }), /Missing or malformed roles list/);
assert.doesNotMatch(dashboard.renderAgentRoleModels({ schemaVersion: 'agent-role-model-v1', milestone: '<script>bad</script>', roles: [], separationRequirements: [] }), /<script>bad<\/script>/);
assert.match(dashboard.renderRunDetail(run), /Agent role model/);
assert.match(dashboard.renderAgentRoleModels(null), /No agent role model/);

const agentWorkPackageFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/agent-work-package.blocked.fixture.json', 'utf8'));
const malformedAgentWorkPackage = { schemaVersion: 'agent-work-package-read-model-v1', workPackageId: '<script>bad</script>', status: 'malformed', malformedReasons: ['acceptanceCriteria missing'], blockers: ['blocked <unsafe>'] };
run.agent_work_package = agentWorkPackageFixture;
const workPackageMarkup = dashboard.renderAgentWorkPackages(agentWorkPackageFixture);
assert.match(workPackageMarkup, /Agent work package/);
assert.match(workPackageMarkup, /work-package-scene-design-blocked/);
assert.match(workPackageMarkup, /ownership evidence must be reviewed/);
assert.match(workPackageMarkup, /Inert verification command text/);
assert.match(workPackageMarkup, /agent-handoff-v2.valid.fixture.json/);
assert.match(workPackageMarkup, /Read-only agent work package/);
assert.doesNotMatch(workPackageMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const malformedWorkPackageMarkup = dashboard.renderAgentWorkPackages(malformedAgentWorkPackage);
assert.match(malformedWorkPackageMarkup, /Malformed: acceptanceCriteria missing/);
assert.match(malformedWorkPackageMarkup, /&lt;script&gt;bad&lt;\/script&gt;/);
assert.doesNotMatch(malformedWorkPackageMarkup, /<script>bad<\/script>/);
assert.match(dashboard.renderRunDetail(run), /Agent work package/);
assert.match(dashboard.renderAgentWorkPackages(null), /No agent work package/);

assert.match(dashboard.renderAgentHandoffs(run.agent_handoffs), /Agent handoff/);
assert.match(dashboard.renderAgentHandoffs(run.agent_handoffs), /blocked/);
assert.match(dashboard.renderAgentHandoffs(run.agent_handoffs), /&lt;handoff-loop&gt;/);
assert.match(dashboard.renderAgentHandoffs(run.agent_handoffs), /Allowed command text/);
assert.doesNotMatch(dashboard.renderAgentHandoffs(run.agent_handoffs), /<handoff-loop>/);
assert.doesNotMatch(dashboard.renderAgentHandoffs(run.agent_handoffs), /<button/i);
assert.match(dashboard.renderRunDetail(run), /Agent handoff/);
assert.match(dashboard.renderAgentHandoffs(null), /No agent handoff/);

const reviewCriticGateFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/review-critic-gate.valid.fixture.json', 'utf8'));
run.review_critic_gate = reviewCriticGateFixture;
const reviewGateMarkup = dashboard.renderReviewCriticGates(reviewCriticGateFixture);
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
const reviewGateXss = dashboard.renderReviewCriticGates({
  schemaVersion: 'review-critic-gate-read-model-v1',
  gateId: '<script>gate</script>',
  taskId: '<img>',
  decision: '<script>blocked</script>',
  promotionRecommendation: '<script>block</script>',
  reviewerActorId: '<reviewer>',
  criticActorId: '<critic>',
  blockedReasons: ['<script>blocked</script>'],
  evidenceReviewedRefPaths: ['<script>evidence</script>'],
  boundary: '<script>boundary</script>',
});
assert.match(reviewGateXss, /&lt;script&gt;gate&lt;\/script&gt;/);
assert.match(reviewGateXss, /&lt;script&gt;blocked&lt;\/script&gt;/);
assert.doesNotMatch(reviewGateXss, /<script>gate<\/script>|<script>boundary<\/script>/);
assert.match(dashboard.renderRunDetail(run), /Review\/critic gate/);
assert.match(dashboard.renderReviewCriticGates(null), /No review\/critic gate/);

const handoffV2Fixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/agent-handoff-v2.blocked.fixture.json', 'utf8'));
const handoffV2Markup = dashboard.renderAgentHandoffs([handoffV2Fixture]);
assert.match(handoffV2Markup, /handoff-v2-blocked/);
assert.match(handoffV2Markup, /task-board-schema/);
assert.match(handoffV2Markup, /Open risks/);
assert.match(handoffV2Markup, /missing-review-risk/);
assert.match(handoffV2Markup, /Acceptance checklist/);
assert.doesNotMatch(handoffV2Markup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(dashboard.renderRunDetail({ agent_handoff_v2s: [handoffV2Fixture] }), /Open risks/);
assert.match(dashboard.renderLoopEvidenceBundles(run.loop_evidence_bundles), /Authoring loop evidence bundle/);
assert.match(dashboard.renderLoopEvidenceBundles(run.loop_evidence_bundles), /partial/);
assert.match(dashboard.renderLoopEvidenceBundles(run.loop_evidence_bundles), /&lt;loop-bundle&gt;/);
assert.match(dashboard.renderLoopEvidenceBundles(run.loop_evidence_bundles), /runs\//);
assert.doesNotMatch(dashboard.renderLoopEvidenceBundles(run.loop_evidence_bundles), /<loop-bundle>/);
assert.match(dashboard.renderRunDetail(run), /Authoring loop evidence bundle/);
assert.match(dashboard.renderLoopEvidenceBundles(null), /No loop evidence bundle/);
const demoHandoffFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/demo-handoff-v2.fixture.json', 'utf8'));
const demoEvidenceBundleFixture = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/demo-evidence-bundle.fixture.json', 'utf8'));
const demoPipelineRun = { agent_handoffs: [demoHandoffFixture], loop_evidence_bundles: [demoEvidenceBundleFixture] };
const demoHandoffMarkup = dashboard.renderAgentHandoffs(demoPipelineRun.agent_handoffs);
assert.match(demoHandoffMarkup, /multi-agent-demo-fixture/);
assert.match(demoHandoffMarkup, /ready/);
assert.match(demoHandoffMarkup, /hidden background agents/);
assert.match(demoHandoffMarkup, /#1 remains open/);
assert.match(demoHandoffMarkup, /#23 remains open/);
assert.doesNotMatch(demoHandoffMarkup, /<button|executeCommand|applyCommand|mergeCommand|browserCommandBridge/);
const demoBundleMarkup = dashboard.renderLoopEvidenceBundles(demoPipelineRun.loop_evidence_bundles);
assert.match(demoBundleMarkup, /multi-agent-demo-fixture/);
assert.match(demoBundleMarkup, /completed/);
assert.match(demoBundleMarkup, /Runs: 1/);
assert.match(demoBundleMarkup, /Matrices: 1/);
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

assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /Source patch evidence bundle/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /bundle-1/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /Review docs patch preview/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /review-held:1/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /review_held_target/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /manual review required/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /Dry-run: passed/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /cargo fmt --check/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /Review: reviewed/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /Linked evidence: source-patch-preview:mutation\/preview.json/);
assert.match(dashboard.renderSourcePatchEvidenceBundles(run), /apply_patch/);
assert.doesNotMatch(dashboard.renderSourcePatchEvidenceBundles(run), /<button|applyCommand|mergeCommand/);
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
const sourcePatchBundleXssMarkup = dashboard.renderSourcePatchEvidenceBundles(sourcePatchBundleXssRun);
assert.match(sourcePatchBundleXssMarkup, /&lt;bundle-xss&gt;/);
assert.match(sourcePatchBundleXssMarkup, /sandbox\/&lt;bad&gt;\/evidence\/report\.json/);
assert.doesNotMatch(sourcePatchBundleXssMarkup, /<script>|<img|<button|applyCommand|mergeCommand|browserCommandBridge|executeCommand/);
assert.match(dashboard.renderSourcePatchApplyTransactions(run), /Source patch apply transaction/);
assert.match(dashboard.renderSourcePatchApplyTransactions(run), /apply-tx-1/);
assert.match(dashboard.renderSourcePatchApplyTransactions(run), /ready_metadata_only_no_apply_authority/);
assert.match(dashboard.renderSourcePatchApplyTransactions(run), /apply_patch/);
assert.doesNotMatch(dashboard.renderSourcePatchApplyTransactions(run), /<button|applyCommand|mergeCommand|browserCommandBridge/);
assert.match(dashboard.renderSourcePatchStaleTargetGuards(run), /Source patch stale target guard/);
assert.match(dashboard.renderSourcePatchStaleTargetGuards(run), /stale-guard-1/);
assert.match(dashboard.renderSourcePatchStaleTargetGuards(run), /fresh_guard_metadata_only_no_apply_authority/);
assert.match(dashboard.renderSourcePatchStaleTargetGuards(run), /apply_patch/);
assert.doesNotMatch(dashboard.renderSourcePatchStaleTargetGuards(run), /<button|applyCommand|mergeCommand|browserCommandBridge/);

assert.match(dashboard.renderQaAgentWorkQueues(run), /QA queue items/);
assert.match(dashboard.renderQaAgentWorkQueues(run), /&lt;qa-item&gt;/);
assert.match(dashboard.renderQaAgentWorkQueues(run), /needs-rerun/);
assert.match(dashboard.renderQaAgentWorkQueues(run), /runs\/multi-agent-pipeline\/demo\/qa\/evaluator-summary\.json/);
assert.match(dashboard.renderQaAgentWorkQueues(run), /Inert command text/);
assert.doesNotMatch(dashboard.renderQaAgentWorkQueues(run), /<script>|<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand|selfApprovalCommand/);
assert.match(dashboard.renderQaAgentWorkQueues({ qaAgentWorkQueues: { present: false, empty_state: 'No queues.' } }), /No queues/);
assert.match(dashboard.renderRunDetail(run), /QA agent work queues/);
assert.match(dashboard.renderRunDetail(run), /Source patch evidence bundle/);


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

const demoDisplayAudit = JSON.parse(fs.readFileSync('examples/source-mutation-preview-demo-v1/display-audit.sample.json', 'utf8'));
const demoDisplayAuditDoc = fs.readFileSync('docs/source-mutation-preview-demo-v1-audit.md', 'utf8');
assert.equal(demoDisplayAudit.status, 'read-only-display-audited');
assert.ok(demoDisplayAudit.displayRefs.includes('docs/source-mutation-preview-demo-v1-audit.md'));
assert.ok(demoDisplayAudit.forbiddenControls.includes('apply_patch'));
assert.ok(demoDisplayAudit.forbiddenControls.includes('browser_command_bridge'));
assert.match(demoDisplayAuditDoc, /read-only display/i);
assert.match(demoDisplayAuditDoc, /Generated reports remain untracked/);
assert.doesNotMatch(demoDisplayAuditDoc, /can apply patches|can merge branches|trusted file write control|executes commands from the browser/i);

const sourceMutationRoadmap = fs.readFileSync('docs/roadmap.md', 'utf8');
const sourceMutationPreviewDoc = fs.readFileSync('docs/source-mutation-preview-v1.md', 'utf8');
assert.match(sourceMutationRoadmap, /Source Mutation Preview v1 is complete as an inert preview\/evidence milestone/);
assert.match(sourceMutationRoadmap, /source patch application to the trusted\s+maintainer worktree[\s\S]*remain\s+out of scope/);
assert.match(sourceMutationPreviewDoc, /Status after #366: complete as inert preview\/review\/sandbox evidence/);
assert.match(sourceMutationPreviewDoc, /does \*\*not\*\* authorize trusted\s+source apply/);

const sourceMutationGovernanceHandoff = fs.readFileSync('docs/source-mutation-preview-governance-handoff.md', 'utf8');
assert.match(sourceMutationGovernanceHandoff, /Source Mutation Preview v1 is complete as inert\s+preview\/review\/sandbox evidence/);
assert.match(sourceMutationGovernanceHandoff, /Source patch apply to the trusted maintainer worktree remains blocked/);
assert.match(sourceMutationGovernanceHandoff, /#1 and #23 remain open/);
assert.doesNotMatch(sourceMutationGovernanceHandoff, /can apply patches|can merge branches|trusted file write control|executes commands from the browser/i);

const productionBundleComplete = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.complete.fixture.json', 'utf8'));
const productionBundlePartial = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.partial.fixture.json', 'utf8'));
const productionBundleConflict = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.unresolved-conflict.fixture.json', 'utf8'));
const productionBundleMissingReview = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/production-evidence-bundle.missing-review.fixture.json', 'utf8'));
const productionBundleMarkup = dashboard.renderProductionEvidenceBundles([productionBundleComplete, productionBundlePartial, productionBundleConflict, productionBundleMissingReview]);
assert.match(productionBundleMarkup, /Production evidence bundle/);
assert.match(productionBundleMarkup, /demo-production-evidence-bundle/);
assert.match(productionBundleMarkup, /demo-production-evidence-bundle-partial/);
assert.match(productionBundleMarkup, /Missing refs: qaResultRefs\[0\] awaits fixture refresh/);
assert.match(productionBundleMarkup, /ownership-conflict-demo:Two work packages claim the same generated output root/);
assert.match(productionBundleMarkup, /missing-review-demo:reviewer/);
assert.match(productionBundleMarkup, /task-board ·/);
assert.match(productionBundleMarkup, /Generated roots: runs\/multi-agent-pipeline/);
assert.match(productionBundleMarkup, /hidden background agents/);
assert.match(productionBundleMarkup, /does not spawn agents/);
assert.match(productionBundleMarkup, /does not spawn agents, execute commands, apply changes, auto-merge, self-approve, or write trusted state/);
assert.doesNotMatch(productionBundleMarkup, /<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand|selfApprovalCommand/);
assert.match(dashboard.renderProductionEvidenceBundles(null), /No production evidence bundle is attached/);
const productionBundleXssMarkup = dashboard.renderProductionEvidenceBundles({
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
});
assert.match(productionBundleXssMarkup, /&lt;bundle-xss&gt;/);
assert.match(productionBundleXssMarkup, /&lt;script&gt;lane&lt;\/script&gt;/);
assert.doesNotMatch(productionBundleXssMarkup, /<script>|<img|<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand/);
assert.match(dashboard.renderRunDetail({ ...run, productionEvidenceBundle: productionBundleConflict }), /Production evidence bundle/);
assert.match(dashboard.renderRunDetail({ ...run, productionEvidenceBundle: productionBundleConflict }), /ownership-conflict-demo/);

const performanceRegressionLane = JSON.parse(fs.readFileSync('examples/multi-agent-pipeline-v1/performance-regression-lane.stale.fixture.json', 'utf8'));
const performanceRegressionMarkup = dashboard.renderPerformanceRegressionLanes({
  performanceRegressionLanes: {
    present: true,
    status: 'stale',
    laneCount: 1,
    staleCount: 1,
    evidenceRefs: [
      'runs/multi-agent-pipeline/demo/performance/run-comparison.json',
      'examples/runtime-frame-budget-v1/frame-budget.sample.json',
      'docs/multi-agent-pipeline-coverage-matrix-v1.md',
      'examples/multi-agent-pipeline-v1/qa-agent-work-queue.valid.fixture.json',
      'examples/multi-agent-pipeline-v1/review-critic-gate.valid.fixture.json',
    ],
    lanes: [performanceRegressionLane],
    boundary: 'Read-only performance/regression lanes; dashboard surfaces do not execute commands, spawn agents, write trusted state, promote regressions, auto-apply, auto-merge, or self-approve.',
  },
});
assert.match(performanceRegressionMarkup, /Performance\/regression lanes/);
assert.match(performanceRegressionMarkup, /demo-performance-regression-lane-stale/);
assert.match(performanceRegressionMarkup, /stale-baseline-run\.json/);
assert.match(performanceRegressionMarkup, /frame-budget\.sample\.json/);
assert.match(performanceRegressionMarkup, /qa-agent-work-queue\.valid\.fixture\.json/);
assert.match(performanceRegressionMarkup, /Read-only performance\/regression lanes/);
assert.doesNotMatch(performanceRegressionMarkup, /<script>|<button|executeCommand|browserCommandBridge|applyCommand|mergeCommand|selfApprovalCommand/);
assert.match(dashboard.renderPerformanceRegressionLanes({ performanceRegressionLanes: { present: false, emptyState: 'No lanes.' } }), /No lanes/);
assert.match(dashboard.renderRunDetail({ ...run, performanceRegressionLanes: { present: true, lanes: [performanceRegressionLane] } }), /Performance\/regression lanes/);

const pluginRegistryXss = dashboard.renderPluginRegistry({ plugin_registry: { present: true, status: '<script>blocked</script>', boundary: '<script>boundary</script>', evidence_refs: ['javascript:alert(1)'], registries: [{ registryId: '<img src=x onerror=alert(1)>', projectId: '<script>project</script>', runId: '<script>run</script>', ledgerRef: '<script>ledger</script>', status: '<script>status</script>', blockedReasons: ['<script>blocked</script>'], plugins: [{ pluginId: '<img src=x onerror=alert(1)>', manifestPath: '<script>manifest</script>', manifestHash: '<script>hash</script>', manifestVersion: '<script>version</script>', validationStatus: '<script>valid</script>', compatibilityStatus: '<script>compat</script>', declaredCapabilities: ['<script>cap</script>'], extensionPoints: ['<script>point</script>'], blockedReasons: ['<script>reason</script>'] }] }] } });
assert.ok(!pluginRegistryXss.includes('<script>blocked</script>'), 'plugin registry status must be escaped');
assert.ok(!pluginRegistryXss.includes('<img src=x onerror=alert(1)>'), 'plugin registry rows must be escaped');

const pluginRegistrySmokeMarkup = dashboard.renderPluginRegistry(run);
assert.match(pluginRegistrySmokeMarkup, /Plugin registry evidence refs/);
assert.match(pluginRegistrySmokeMarkup, /read-only-dashboard-panel/);
assert.match(pluginRegistrySmokeMarkup, /blocked-command-panel/);
assert.match(pluginRegistrySmokeMarkup, /manifest requested executable command authority/);
assert.ok(!/<button/i.test(pluginRegistrySmokeMarkup), 'plugin registry dashboard smoke must not render action buttons');
assert.ok(!/data-action=/i.test(pluginRegistrySmokeMarkup), 'plugin registry dashboard smoke must not render action hooks');
assert.ok(!/href=["']javascript:/i.test(pluginRegistrySmokeMarkup), 'plugin registry dashboard smoke must not render javascript links');
assert.ok(!/command bridge/i.test(pluginRegistrySmokeMarkup), 'plugin registry dashboard smoke must not advertise a command bridge');

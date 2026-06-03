const assert = require('node:assert/strict');
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
  evidence: [{ id: 'artifact-1', kind: 'application/json', path: 'evidence/a.json', metadata: {}, exists: true }],
  probe_contract_status: { status: 'present', contract_name: 'ouroforge-runtime-probe', version: 'v2', observed_count: 2, missing_count: 0, malformed_count: 0, evidence_refs: ['evidence/world.json', 'evidence/frame.json'] },
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
  mutation_artifacts: [{ id: 'mutation-proposals', kind: 'application/json', path: 'mutation/proposals.json', value: { proposals: [] }, metadata: {} }],
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
        id: 'journal-entry-2-verdict-summary',
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
assert.match(dashboard.renderJournalViewer({ ...run, journal_view: { path: 'journal.md', exists: false, read_error: 'missing journal artifact', entries: [] } }), /missing journal artifact/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: 'missing', stages: [], command_hints: [] } }), /No mutation lifecycle stages/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: '<script>', command_hints: [], stages: [{ id: 'scene_applied', label: '<img>', state: '<bad>', artifact_path: 'mutation/scene-applications.json', record_count: 1, records: [{ id: '<script>', project: { projectId: '<img>', manifestPath: '<script>', manifestHash: { algorithm: '<b>', value: '<i>' }, scenePath: '<p>', sceneHash: { algorithm: '<u>', value: '<em>' } }, rollback: { scenePath: '<svg>', restoreHash: { value: '<hash>' } } }] }] } }), /&lt;script&gt;/);
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

// Untrusted artifact/journal content must be HTML-escaped, not rendered as markup.
const xssRun = {
  summary: { id: '<img src=x onerror=alert(1)>', run_dir: 'runs/x', seed_id: 's', run_status: 'created', verdict_status: 'failed', scenario_status: 'pending', evidence_count: 0, mutation_count: 0, worker_count: 0 },
  command_context: { command: '<script>alert(1)</script>', argv: ['<img src=x onerror=alert(1)>'], seedPath: '<script>seed</script>', workers: '<script>workers</script>', runsRoot: 'runs', scenarioPackId: '<script>pack</script>', runtimeTarget: '<script>runtime</script>', browserBoundary: '<script>boundary</script>', cdpTransport: '<script>transport</script>', environmentHints: ['<script>hint</script>'] },
  evidence: [], screenshots: [], world_states: [], frame_metrics: [], performance_metrics: [{ id: '<script>perf</script>', kind: 'application/json', path: 'evidence/<script>perf</script>.json', value: null, read_error: '<script>bad perf</script>', metadata: { worker_id: '<script>worker</script>', execution_boundary: '<script>boundary</script>' } }], console_logs: [{ id: '<script>console</script>', kind: 'application/json', path: 'evidence/<script>console</script>.json', value: [{ text: '<script>log</script>' }], metadata: { worker_session_id: '<img src=x onerror=alert(1)>', cdp_transport: '<script>transport</script>' } }], cdp_trace_summaries: [], scenario_results: [], mutation_artifacts: [], mutations: [],
  mutation_lifecycle: { terminal_state: '<img>', stages: [{ id: 'x', label: '<img>', state: '<script>', artifact_path: '<b>', record_count: 0, evidence_refs: [], records: [] }], command_hints: ['<script>alert(1)</script>'] },
  regression_promotions: [{ id: '<script>', scenarioId: '<img>', target: { scenarioPackId: '<svg>', scenarioPackPath: '<b>' }, beforeHash: { value: '<before>' }, afterHash: { value: '<after>' }, recordPath: '<record>' }],
  replay: { present: true, empty_state: '', sequences: [{ id: '<script>', source: '<img>', event_count: 1, frames: [0], evidence_refs: ['<script>'], checkpoints: [{ label: '<img>', frame: 0, tick: 0, world_state_path: '<b>', world_state: { unsafe: '<script>alert(1)</script>' } }] }] },
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
assert.ok(!dashboard.renderRunList([xssRun], null).includes('<img src=x onerror'), 'run id markup must be escaped');

const rawMalformedCommandContextRun = {
  summary: { id: 'raw-malformed', run_dir: 'runs/raw-malformed', verdict_status: 'passed' },
  run: { run_command_context: { schemaVersion: 7, command: 'untrusted raw command' } },
  evidence: [], screenshots: [], world_states: [], frame_metrics: [], performance_metrics: [], console_logs: [], cdp_trace_summaries: [], scenario_results: [], mutation_artifacts: [], mutations: [],
};
const rawMalformedCommandContextDetail = dashboard.renderRunDetail(rawMalformedCommandContextRun);
assert.match(rawMalformedCommandContextDetail, /No run command context is recorded/);
assert.ok(!rawMalformedCommandContextDetail.includes('untrusted raw command'), 'raw malformed run_command_context must not render');
console.log('dashboard smoke test passed');

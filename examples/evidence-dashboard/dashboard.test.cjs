const assert = require('node:assert/strict');
const dashboard = require('./dashboard.js');

const run = {
  summary: {
    id: 'run-1',
    run_dir: 'runs/run-1',
    seed_id: 'platformer.v0',
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
  },
  evidence: [{ id: 'artifact-1', kind: 'application/json', path: 'evidence/a.json', metadata: {}, exists: true }],
  screenshots: [{ id: 'shot-1', kind: 'image/png', path: 'evidence/shot.png', metadata: {}, exists: true }],
  world_states: [{ id: 'world-1', kind: 'application/json', path: 'evidence/world.json', value: { object: { x: 40 } }, metadata: {} }],
  frame_metrics: [{ id: 'frame-1', kind: 'application/json', path: 'evidence/frame.json', value: { frame: 1 }, metadata: {} }],
  performance_metrics: [{ id: 'perf-1', kind: 'application/json', path: 'evidence/perf.json', value: { metrics: [] }, metadata: {} }],
  console_logs: [{ id: 'console-1', kind: 'application/json', path: 'evidence/console.json', value: [{ text: 'ready' }], metadata: {} }],
  cdp_trace_summaries: [{ id: 'cdp-1', kind: 'application/json', path: 'evidence/cdp.json', value: null, read_error: 'bad json', metadata: {} }],
  scenario_results: [{ id: 'scenario-1', kind: 'application/json', path: 'evidence/scenario.json', value: { status: 'passed' }, metadata: {} }],
  mutation_artifacts: [{ id: 'mutation-proposals', kind: 'application/json', path: 'mutation/proposals.json', value: { proposals: [] }, metadata: {} }],
  mutations: [{ id: 'mutation-1', evidence_id: 'artifact-1', status: 'proposed' }],
  mutation_lifecycle: {
    terminal_state: 'accepted',
    command_hints: [
      'cargo run -p ouroforge-cli -- mutation review runs/run-1 --accept --reason "manual evidence review accepted"',
      'cargo run -p ouroforge-cli -- mutation review runs/run-1 --reject --reason "manual evidence review rejected"',
    ],
    stages: [
      { id: 'proposed', label: 'Proposed', state: 'proposed', artifact_path: 'mutation/proposals.json', record_count: 1, evidence_refs: ['artifact-1'], records: [{ id: 'mutation-1' }] },
      { id: 'classified', label: 'Classified', state: 'classified', artifact_path: 'mutation/classifications.json', record_count: 1, evidence_refs: ['evidence/world.json'], records: [{ id: 'classification-1' }] },
      { id: 'drafted', label: 'Drafted', state: 'drafted', artifact_path: 'mutation/patch-drafts.json', record_count: 1, evidence_refs: ['evidence/world.json'], records: [{ id: 'patch-draft-1' }] },
      { id: 'sandboxed', label: 'Sandboxed', state: 'sandboxed', artifact_path: 'sandbox/*/evidence/result.json', record_count: 1, evidence_refs: ['sandbox/patch-draft-1/evidence/result.json'], records: [{ patch_draft_id: 'patch-draft-1' }] },
      { id: 'compared', label: 'Compared', state: 'compared', artifact_path: 'mutation/rerun-orchestration.json', record_count: 1, evidence_refs: ['mutation/rerun-orchestration.json'], records: [{ comparison_artifact_path: 'mutation/run-comparison-before--after.json' }] },
      { id: 'reviewed', label: 'Manual review', state: 'accepted', artifact_path: 'mutation/review-decisions.json', record_count: 1, evidence_refs: ['mutation/rerun-orchestration.json'], records: [{ state: 'accepted' }] },
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
assert.match(runList, /4 workers/);
assert.match(runList, /scenario passed/);
const detail = dashboard.renderRunDetail(run);
assert.match(detail, /World-state snapshots/);
assert.match(detail, /Frame\/performance metrics/);
assert.match(detail, /Console\/CDP summaries/);
assert.match(detail, /Scenario results/);
assert.match(detail, /Mutation artifacts/);
assert.match(detail, /Journal Viewer/);
assert.match(detail, /Mutation Review/);
assert.match(detail, /Inspect-only/);
assert.match(detail, /Proposed/);
assert.match(detail, /Classified/);
assert.match(detail, /Drafted/);
assert.match(detail, /Sandboxed/);
assert.match(detail, /Compared/);
assert.match(detail, /Manual review/);
assert.match(detail, /accepted/);
assert.match(detail, /mutation\/review-decisions\.json/);
assert.match(detail, /mutation review runs\/run-1 --accept/);
assert.match(detail, /No lifecycle records for this stage|patch-draft-1/);
assert.match(detail, /fixture journal summary/);
assert.match(detail, /Observations/);
assert.ok(detail.includes('../../runs/run-1/evidence/world.json'));
assert.ok(detail.includes('../../runs/run-1/verdict.json'));
assert.match(detail, /mutation-1/);
assert.ok(!detail.includes('<script>alert(1)</script>'), 'journal entry markup must be escaped');
assert.match(detail, /&lt;script&gt;/);
assert.match(detail, /mutation\/proposals\.json/);
assert.match(detail, /1 missing/);
assert.match(detail, /1 malformed/);
assert.match(detail, /bad json/);
assert.match(dashboard.renderCategorySummary(run.summary.evidence_categories), /Frame\/performance metrics/);
assert.match(dashboard.renderJournalViewer({ ...run, journal_view: { path: 'journal.md', exists: false, read_error: 'missing journal artifact', entries: [] } }), /missing journal artifact/);
assert.match(dashboard.renderMutationLifecycle({ mutation_lifecycle: { terminal_state: 'missing', stages: [], command_hints: [] } }), /No mutation lifecycle stages/);
assert.match(dashboard.renderRunList([], null), /No runs found/);

// Untrusted artifact/journal content must be HTML-escaped, not rendered as markup.
const xssRun = {
  summary: { id: '<img src=x onerror=alert(1)>', run_dir: 'runs/x', seed_id: 's', run_status: 'created', verdict_status: 'failed', scenario_status: 'pending', evidence_count: 0, mutation_count: 0, worker_count: 0 },
  evidence: [], screenshots: [], world_states: [], frame_metrics: [], performance_metrics: [], console_logs: [], cdp_trace_summaries: [], scenario_results: [], mutation_artifacts: [], mutations: [],
  mutation_lifecycle: { terminal_state: '<img>', stages: [{ id: 'x', label: '<img>', state: '<script>', artifact_path: '<b>', record_count: 0, evidence_refs: [], records: [] }], command_hints: ['<script>alert(1)</script>'] },
  journal_view: { path: 'journal.md', exists: true, summary: '<b>unsafe</b>', entries: [{ heading: '<img>', category: 'summary', body: '<script>alert(1)</script>', evidence_refs: [], verdict_refs: [], mutation_refs: [] }], evidence_refs: [], verdict_refs: [], mutation_refs: [] },
  verdict: {}, journal: '<script>alert(1)</script>',
};
const xssDetail = dashboard.renderRunDetail(xssRun);
assert.ok(!xssDetail.includes('<script>alert(1)</script>'), 'journal markup must be escaped');
assert.match(xssDetail, /&lt;script&gt;/);
assert.ok(!xssDetail.includes('<img>'), 'journal headings must be escaped');
assert.ok(!dashboard.renderRunList([xssRun], null).includes('<img src=x onerror'), 'run id markup must be escaped');
console.log('dashboard smoke test passed');

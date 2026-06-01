const assert = require('node:assert/strict');
const dashboard = require('./dashboard.js');

const run = {
  summary: { id: 'run-1', run_dir: 'runs/run-1', seed_id: 'platformer.v0', verdict_status: 'failed', evidence_count: 2, mutation_count: 1 },
  evidence: [{ id: 'artifact-1', kind: 'application/json', path: 'evidence/a.json', metadata: {} }],
  screenshots: [{ id: 'shot-1', kind: 'image/png', path: 'evidence/shot.png', metadata: {} }],
  world_states: [{ id: 'world-1', kind: 'application/json', path: 'evidence/world.json', value: { object: { x: 40 } }, metadata: {} }],
  console_logs: [{ id: 'console-1', kind: 'application/json', path: 'evidence/console.json', value: [{ text: 'ready' }], metadata: {} }],
  mutations: [{ id: 'mutation-1', evidence_id: 'artifact-1', status: 'proposed' }],
  verdict: { status: 'failed' },
  journal: '# Journal',
};

assert.equal(dashboard.statusClass('passed'), 'status status-passed');
assert.equal(dashboard.artifactHref(run.evidence[0], run), '../../runs/run-1/evidence/a.json');
assert.match(dashboard.renderRunList([run], 'run-1'), /platformer\.v0/);
assert.match(dashboard.renderRunDetail(run), /World state/);
assert.match(dashboard.renderRunDetail(run), /mutation-1/);

// Untrusted artifact/journal content must be HTML-escaped, not rendered as markup.
const xssRun = {
  summary: { id: '<img src=x onerror=alert(1)>', run_dir: 'runs/x', seed_id: 's', verdict_status: 'failed', evidence_count: 0, mutation_count: 0 },
  evidence: [], screenshots: [], world_states: [], console_logs: [], mutations: [],
  verdict: {}, journal: '<script>alert(1)</script>',
};
const xssDetail = dashboard.renderRunDetail(xssRun);
assert.ok(!xssDetail.includes('<script>alert(1)</script>'), 'journal markup must be escaped');
assert.match(xssDetail, /&lt;script&gt;/);
assert.ok(!dashboard.renderRunList([xssRun], null).includes('<img src=x onerror'), 'run id markup must be escaped');
console.log('dashboard smoke test passed');

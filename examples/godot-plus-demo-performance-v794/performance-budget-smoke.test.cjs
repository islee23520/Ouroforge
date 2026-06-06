const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '../..');
const docPath = path.join(root, 'docs/godot-plus-demo-performance-budget-v1.md');
const doc = fs.readFileSync(docPath, 'utf8');
const scenePath = 'examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json';
const scene = JSON.parse(fs.readFileSync(path.join(root, scenePath), 'utf8'));
const exportProfilePath = 'examples/playable-demo-v2/collect-and-exit/export/export-profile.json';
const exportProfile = JSON.parse(fs.readFileSync(path.join(root, exportProfilePath), 'utf8'));
const gitignore = fs.readFileSync(path.join(root, '.gitignore'), 'utf8');

const frameBudget = scene.metadata.runtimeDebug.frameBudget;
const frameTimings = scene.metadata.runtimeDebug.frameTimings;
assert.equal(frameBudget.totalMs, 20);
assert.ok(frameTimings.totalMs <= frameBudget.totalMs, 'fixture frame timing must fit the declared budget');
assert.ok(frameTimings.updateMs <= frameBudget.updateMs, 'fixture update timing must fit the declared budget');
assert.ok(frameTimings.renderMs <= frameBudget.renderMs, 'fixture render timing must fit the declared budget');
assert.ok(frameTimings.evidenceMs <= frameBudget.evidenceMs, 'fixture evidence timing must fit the declared budget');

const requiredRows = [
  {
    budget: 'Frame budget',
    evidence: [scenePath, 'metadata.runtimeDebug.frameBudget', 'metadata.runtimeDebug.frameTimings'],
    expectations: [/totalMs <= 20/i, /<= frameBudget\.totalMs/i],
  },
  {
    budget: 'Load-time budget',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/plugin-usage-v792-smoke.test.cjs',
    ],
    expectations: [/No network install\/update/i, /deterministic/i],
  },
  {
    budget: 'Console / crash-free budget',
    evidence: ['node --check', 'cargo test -p ouroforge-core --test godot_plus_demo_export_package_contract'],
    expectations: [/without uncaught exceptions/i, /without panics\/failures/i],
  },
  {
    budget: 'QA / playtest stability',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs',
    ],
    expectations: [/bounded checks\/workers/i, /fixture-scoped/i],
  },
  {
    budget: 'Export verification stability',
    evidence: [
      'crates/ouroforge-core/tests/godot_plus_demo_export_package_contract.rs',
      exportProfilePath,
      'root `.gitignore` `/dist/`',
    ],
    expectations: [/local-web-only/i, /no publish, deploy, sign, upload/i],
  },

  {
    budget: 'Boundary / wording stability',
    evidence: [
      'docs/godot-plus-demo-capability-comparison-matrix-v1.md',
      'docs/godot-plus-demo-studio-walkthrough-v1.md',
      'docs/godot-plus-demo-plugin-usage-v1.md',
      'docs/safe-source-mutation-apply-v1.md',
      'examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/plugin-usage-v792-smoke.test.cjs',
    ],
    expectations: [
      /No-overclaim/i,
      /no-commercial-release/i,
      /no-direct-trusted-write/i,
      /review-gated source apply/i,
      /plugin descriptor/i,
      /Studio walkthrough/i,
      /#1\/#23 governance/i,
    ],
  },
  {
    budget: 'Generated-state stability',
    evidence: [
      'examples/godot-plus-demo-closure-v791-794/godot-plus-open-issues-audit.test.cjs',
      'git status --short --ignored',
      'git diff --check',
    ],
    expectations: [/remain untracked unless explicitly fixture-scoped/i],
  },
];

for (const { budget, evidence, expectations } of requiredRows) {
  const row = doc
    .split('\n')
    .find((line) => line.startsWith(`| ${budget} |`));
  assert.ok(row, `missing performance budget row for ${budget}`);
  for (const evidenceRef of evidence) {
    assert.ok(row.includes(evidenceRef), `${budget} row missing ${evidenceRef}`);
    if (!evidenceRef.includes('`') && /^(docs|examples|crates)\//.test(evidenceRef)) {
      assert.ok(fs.existsSync(path.join(root, evidenceRef)), `evidence path missing on disk: ${evidenceRef}`);
    }
  }
  for (const expectation of expectations) assert.match(row, expectation, `${budget} row missing ${expectation}`);
}

const knownGaps = [
  /No shipped-game SLA/i,
  /No native\/mobile\/console export performance measurement/i,
  /No public deploy/i,
  /No executable plugin runtime/i,
  /No direct trusted source writes from Studio/i,
];
for (const pattern of knownGaps) assert.match(doc, pattern);

assert.equal(exportProfile.exportTarget, 'web-local');
assert.match(exportProfile.boundary, /no publish, deploy, sign, or upload/i);
assert.doesNotMatch(JSON.stringify(exportProfile), /native|mobile|console|store|steam|itch|production-ready/i);
assert.match(gitignore, /^\/dist\/$/m);
assert.match(doc, /## Governance/);
assert.match(doc, /Protected issues #1 and #23 must remain open/);
assert.match(doc, /#1 and #23 remain open/);

const forbiddenBudgetClaims = [
  /production-ready performance/i,
  /commercial release ready/i,
  /public deployment ready/i,
  /native export performance is validated/i,
  /Godot replacement/i,
  /full Godot parity/i,
];
for (const pattern of forbiddenBudgetClaims) assert.doesNotMatch(doc, pattern);
console.log('performance budget v794 smoke passed');

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;
const matrixPath = path.join(fixtureDir, 'coverage-matrix.fixture.json');
const repoRoot = path.resolve(fixtureDir, '..', '..');

const requiredSuccess = [
  'GPD18.success-game-start',
  'GPD18.success-player-movement',
  'GPD18.success-objective-update',
  'GPD18.success-enemy-npc-behavior',
  'GPD18.success-level-completion',
  'GPD18.success-lose-restart',
  'GPD18.success-hud-update',
  'GPD18.success-runtime-probe',
  'GPD18.success-qa-pass',
  'GPD18.success-export-package-evidence',
  'GPD18.success-studio-walkthrough-rendering',
  'GPD18.success-plugin-descriptor-validation',
  'GPD18.success-evidence-bundle-existence',
];

const requiredBlocked = [
  'GPD18.block-broken-objective',
  'GPD18.block-missing-asset',
  'GPD18.block-missing-probe',
  'GPD18.block-invalid-level-metadata',
  'GPD18.block-broken-export',
  'GPD18.block-invalid-plugin',
  'GPD18.block-unclassified-qa-failure',
  'GPD18.block-incomplete-evidence-bundle',
  'GPD18.block-direct-source-apply-attempt',
  'GPD18.block-publish-deploy-attempt',
];

const generatedRoots = [
  'runs/',
  'target/',
  'dashboard-data/',
  '.openchrome/',
  '.omc/',
  '.omx/',
  '.claude/',
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function scenarioIds(matrix) {
  return new Set(matrix.scenarios.map((scenario) => scenario.id));
}

function assertRequiredIds(ids, requiredIds) {
  for (const id of requiredIds) {
    assert.ok(ids.has(id), `missing scenario ${id}`);
  }
}

function assertEvidenceRefsAreControlled(matrix) {
  for (const scenario of matrix.scenarios) {
    assert.ok(Array.isArray(scenario.evidenceRefs), `${scenario.id} evidenceRefs must be an array`);
    assert.ok(scenario.evidenceRefs.length > 0, `${scenario.id} must include evidence refs`);
    for (const ref of scenario.evidenceRefs) {
      const normalized = ref.replaceAll('\\', '/');
      const fixtureScoped = normalized.startsWith('examples/') || normalized.startsWith('docs/');
      const generatedIgnored = generatedRoots.some((root) => normalized.startsWith(root));
      assert.ok(
        fixtureScoped || generatedIgnored,
        `${scenario.id} evidence ref ${ref} must be fixture-scoped or ignored generated state`,
      );
    }
  }
}

function assertBlockedDiagnostics(matrix) {
  const blocked = matrix.scenarios.filter((scenario) => scenario.kind === 'blocked');
  assert.equal(blocked.length, requiredBlocked.length, 'blocked scenario count');
  for (const scenario of blocked) {
    assert.match(scenario.expected, /block|reject|fail closed|diagnostic/i, `${scenario.id} expected fail-closed diagnostic`);
    assert.ok(scenario.diagnostic, `${scenario.id} includes actionable diagnostic`);
    assert.match(scenario.diagnostic, /fix|provide|review|restore|reject|evidence|metadata|descriptor/i, `${scenario.id} diagnostic is actionable`);
  }
}

function assertGuardrails(matrix) {
  const combined = JSON.stringify(matrix).toLowerCase();
  for (const term of [
    'fixture-scoped',
    'generated',
    'read-only',
    'draft-only',
    'review-gated',
    'no trusted browser write',
    'no command bridge',
    'no auto-apply',
    'no publish',
    'no deploy',
    'no executable plugin runtime',
    'no marketplace',
    'no commercial release',
    'no godot replacement',
    '#1 and #23 remain open',
  ]) {
    assert.ok(combined.includes(term), `matrix missing guardrail: ${term}`);
  }
}

function assertReferencedFixtureFilesExist(matrix) {
  for (const scenario of matrix.scenarios) {
    for (const ref of scenario.evidenceRefs) {
      if (ref.startsWith('examples/') || ref.startsWith('docs/')) {
        assert.ok(fs.existsSync(path.join(repoRoot, ref)), `${scenario.id} evidence ref exists: ${ref}`);
      }
    }
  }
}

const matrix = readJson(matrixPath);
assert.equal(matrix.schemaVersion, 'scenario-coverage-v18-godot-plus-demo-regression-v1');
assert.equal(matrix.issue, 796);
assert.equal(matrix.status, 'fixture-scoped');
assert.equal(matrix.governanceAnchors.issue1, 'open');
assert.equal(matrix.governanceAnchors.issue23, 'open');
assert.equal(matrix.governanceAnchors.closureStatement, '#1 and #23 remain open');
assert.ok(Array.isArray(matrix.scenarios), 'scenarios must be an array');

const ids = scenarioIds(matrix);
assertRequiredIds(ids, requiredSuccess);
assertRequiredIds(ids, requiredBlocked);
assert.equal(matrix.scenarios.filter((scenario) => scenario.kind === 'success').length, requiredSuccess.length);
assertBlockedDiagnostics(matrix);
assertEvidenceRefsAreControlled(matrix);
assertGuardrails(matrix);
assertReferencedFixtureFilesExist(matrix);

console.log('scenario coverage v18 Godot-plus demo regression matrix passed');

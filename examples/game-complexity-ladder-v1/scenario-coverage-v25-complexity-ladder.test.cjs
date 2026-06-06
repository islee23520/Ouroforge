const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const coverageDir = path.join(root, 'scenario-coverage-v25');
const matrixPath = path.join(coverageDir, 'matrix.json');
const repoRoot = path.resolve(root, '../..');
const docsPath = path.join(repoRoot, 'docs/scenario-coverage-v25.md');

function readText(filePath) {
  return fs.readFileSync(filePath, 'utf8');
}

function readJson(filePath) {
  return JSON.parse(readText(filePath));
}

function walkFiles(dir) {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const entryPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...walkFiles(entryPath));
    } else {
      files.push(entryPath);
    }
  }
  return files.sort();
}

function assertRepoRefExists(ref, label) {
  assert.equal(path.isAbsolute(ref), false, `${label} must be repo-relative: ${ref}`);
  assert.equal(ref.includes('..'), false, `${label} must not traverse upward: ${ref}`);
  assert.ok(fs.existsSync(path.join(repoRoot, ref)), `${label} exists: ${ref}`);
}

function assertNoAuthorization(text, label) {
  assert.doesNotMatch(text, /"autoApply"\s*:\s*true/i, `${label} must not enable autoApply`);
  assert.doesNotMatch(text, /"autoMerge"\s*:\s*true/i, `${label} must not enable autoMerge`);
  assert.doesNotMatch(text, /"trustedWritesFromBrowser"\s*:\s*true/i, `${label} must not enable browser trusted writes`);
  assert.doesNotMatch(text, /"networkRequired"\s*:\s*true/i, `${label} must not require network`);
  assert.doesNotMatch(text, /"browserRequired"\s*:\s*true/i, `${label} must not require browser`);
  assert.doesNotMatch(text, /auto[- ]?apply enabled|auto[- ]?merge enabled/i, `${label} must not authorize auto-apply or auto-merge`);
  assert.doesNotMatch(text, /browser trusted writes? enabled|trusted browser writes? enabled/i, `${label} must not authorize browser trusted writes`);
  assert.doesNotMatch(text, /network enabled|requires network|live browser required|browser command bridge enabled/i, `${label} must not authorize network or live browser execution`);
}

const matrix = readJson(matrixPath);
const allJsonPaths = walkFiles(coverageDir).filter((filePath) => filePath.endsWith('.json'));
const jsonByRel = new Map();

for (const filePath of allJsonPaths) {
  const rel = path.relative(coverageDir, filePath).split(path.sep).join('/');
  jsonByRel.set(rel, readJson(filePath));
}

assert.equal(matrix.schemaVersion, 'scenario-coverage-v25-complexity-ladder-v1');
assert.equal(matrix.issue, 1497);
assert.equal(matrix.fixtureScoped, true);
assert.equal(matrix.generatedState, false);
assert.equal(matrix.ladderContractRef, 'docs/game-complexity-ladder-v1.md');
assert.match(matrix.boundary, /Deterministic fixture coverage only/);
assert.match(matrix.boundary, /no subjective quality assertions/);
assert.match(matrix.boundary, /no browser trusted writes/);
assert.match(matrix.boundary, /no auto-fix, no auto-apply, and no auto-merge/);

assert.deepEqual(matrix.allowedRungGateStates, [
  'satisfied',
  'unsatisfied',
  'insufficient-evidence',
  'out-of-order',
]);
assert.deepEqual(matrix.allowedEngineGrowthStates, [
  'justified',
  'unjustified',
  'missing-prerequisite',
]);

const referencedFixtureRefs = [
  ...matrix.rungGateStateRefs,
  ...matrix.engineGrowthJustificationRefs,
  ...matrix.backwardCompatibilityRefs,
];
assert.deepEqual(
  new Set(referencedFixtureRefs),
  new Set([...jsonByRel.keys()].filter((rel) => rel !== 'matrix.json')),
  'matrix references every JSON fixture found under scenario-coverage-v25',
);

for (const ref of referencedFixtureRefs) {
  assert.ok(jsonByRel.has(ref), `matrix fixture ref exists: ${ref}`);
}

assertRepoRefExists(matrix.ladderContractRef, 'ladder contract ref');

const rungStates = new Set();
for (const ref of matrix.rungGateStateRefs) {
  const fixture = jsonByRel.get(ref);
  assert.equal(fixture.schemaVersion, 'complexity-ladder-rung-gate-regression-v25');
  assert.equal(fixture.fixtureScoped, true, `${ref} is fixture scoped`);
  assert.equal(fixture.generatedState, false, `${ref} is not trusted generated state`);
  assert.ok(matrix.allowedRungGateStates.includes(fixture.gateState), `${ref} has an allowed rung state`);
  assert.equal(typeof fixture.caseId, 'string');
  assert.equal(typeof fixture.rung, 'string');
  assert.equal(Number.isInteger(fixture.rungOrder), true);
  assert.equal(typeof fixture.expected.rungClaimAllowed, 'boolean');
  assert.equal(typeof fixture.expected.nextRungClaimAllowed, 'boolean');
  for (const verificationRef of fixture.evidence.verificationRefs || []) {
    assertRepoRefExists(verificationRef, `${ref} verification ref`);
  }
  rungStates.add(fixture.gateState);
}

assert.deepEqual(rungStates, new Set(matrix.allowedRungGateStates));
assert.equal(jsonByRel.get('rung-gate-states/collect-and-exit-satisfied.json').expected.rungClaimAllowed, true);
assert.equal(jsonByRel.get('rung-gate-states/platformer-unsatisfied.json').expected.rungClaimAllowed, false);
assert.equal(jsonByRel.get('rung-gate-states/top-down-insufficient-evidence.json').expected.rungClaimAllowed, false);
assert.equal(jsonByRel.get('rung-gate-states/multi-scene-out-of-order.json').expected.rungClaimAllowed, false);

const engineGrowthStates = new Set();
for (const ref of matrix.engineGrowthJustificationRefs) {
  const fixture = jsonByRel.get(ref);
  assert.equal(fixture.schemaVersion, 'complexity-ladder-engine-growth-regression-v25');
  assert.equal(fixture.fixtureScoped, true, `${ref} is fixture scoped`);
  assert.equal(fixture.generatedState, false, `${ref} is not trusted generated state`);
  assert.ok(matrix.allowedEngineGrowthStates.includes(fixture.justificationState), `${ref} has an allowed justification state`);
  assert.equal(typeof fixture.caseId, 'string');
  assert.equal(typeof fixture.requestedCapability, 'string');
  assert.equal(typeof fixture.expected.engineGrowthAllowed, 'boolean');
  if (fixture.justificationState === 'justified') {
    assert.equal(typeof fixture.rungGate, 'string');
    assert.equal(typeof fixture.evidenceGap, 'string');
    assert.equal(typeof fixture.minimalImplementation, 'string');
    assert.equal(fixture.expected.engineGrowthAllowed, true);
  } else {
    assert.equal(fixture.expected.engineGrowthAllowed, false);
    assert.equal(typeof fixture.expected.blocker, 'string');
  }
  engineGrowthStates.add(fixture.justificationState);
}

assert.deepEqual(engineGrowthStates, new Set(matrix.allowedEngineGrowthStates));

for (const ref of matrix.backwardCompatibilityRefs) {
  const fixture = jsonByRel.get(ref);
  assert.equal(fixture.schemaVersion, 'complexity-ladder-backward-compatibility-regression-v25');
  assert.equal(fixture.fixtureScoped, true, `${ref} is fixture scoped`);
  assert.equal(fixture.generatedState, false, `${ref} is not trusted generated state`);
  assert.equal(fixture.compatibilityState, 'valid');
  assert.ok(fixture.sourceRefs.includes('docs/game-complexity-ladder-v1.md'), `${ref} keeps ladder doc reference`);
  assert.ok(
    fixture.sourceRefs.includes('examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs'),
    `${ref} keeps existing Signal Gate collect-and-exit e2e path`,
  );
  assert.ok(
    fixture.sourceRefs.includes('examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs'),
    `${ref} keeps existing Signal Gate collect-and-exit evidence path`,
  );
  for (const sourceRef of fixture.sourceRefs) {
    assertRepoRefExists(sourceRef, `${ref} source ref`);
  }
  assert.deepEqual(fixture.expected, {
    existingDemoRefsRemainReadable: true,
    existingEvidenceRefsRemainReadable: true,
    requiresIssue1494To1496Branches: false,
    modifiesExistingDemo: false,
    modifiesIssue1Or23: false,
  });
}

assert.equal(matrix.guardrails.networkRequired, false);
assert.equal(matrix.guardrails.browserRequired, false);
assert.equal(matrix.guardrails.trustedWritesFromBrowser, false);
assert.equal(matrix.guardrails.commandBridge, false);
assert.equal(matrix.guardrails.autoFix, false);
assert.equal(matrix.guardrails.autoApply, false);
assert.equal(matrix.guardrails.autoMerge, false);
assert.equal(matrix.guardrails.modifyIssue1Or23, false);
assert.equal(matrix.guardrails.broadEngineClaim, false);

for (const filePath of allJsonPaths) {
  assertNoAuthorization(readText(filePath), path.relative(root, filePath));
}

const docs = readText(docsPath);
assert.match(docs, /# Scenario Coverage v25/);
assert.match(docs, /Game Complexity Ladder v1/);
assert.match(docs, /rung gate states/i);
assert.match(docs, /engine-growth justification states/i);
assert.match(docs, /backward compatibility/i);
assert.match(docs, /generated-state boundary/i);
assert.match(docs, /governance audit/i);
assert.match(docs, /#1 and #23 remain unchanged/i);
assertNoAuthorization(docs, 'docs/scenario-coverage-v25.md');
assert.doesNotMatch(docs, /subjective quality|quality score|Godot replacement|full parity|production readiness|production-ready/i);

console.log('scenario coverage v25 complexity ladder smoke passed');

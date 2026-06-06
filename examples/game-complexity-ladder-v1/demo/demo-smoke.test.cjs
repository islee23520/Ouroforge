const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '../../..');
const fixturePath = path.join(__dirname, 'rung-demo.fixture.json');
const docPath = path.join(repoRoot, 'docs/game-complexity-ladder-v1-demo.md');

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function readText(filePath) {
  return fs.readFileSync(filePath, 'utf8');
}

function assertRepoRefExists(ref) {
  assert.equal(fs.existsSync(path.join(repoRoot, ref)), true, `${ref} exists`);
}

function collectEvidenceRefs(fixture) {
  const refs = new Set();
  refs.add(fixture.demo.rootRef);
  refs.add(fixture.demo.projectRef);
  refs.add(fixture.demo.scenarioPackRef);
  refs.add(fixture.demo.seedRef);
  for (const ref of fixture.demo.canonicalDocs) refs.add(ref);
  for (const evidence of fixture.loopProducedEvidence) refs.add(evidence.ref);
  for (const gate of Object.values(fixture.fourGateEvidence)) {
    for (const ref of gate.evidenceRefs) refs.add(ref);
  }
  for (const ref of fixture.loopCoverage.evidenceRefs) refs.add(ref);
  for (const ref of fixture.engineGrowthJustification.justificationRefs) refs.add(ref);
  return [...refs].sort();
}

const fixture = readJson(fixturePath);
const docs = readText(docPath);
const ladder = readText(path.join(repoRoot, 'docs/game-complexity-ladder-v1.md'));

assert.equal(fixture.schemaVersion, 'game-complexity-ladder-rung-demo-v1');
assert.equal(fixture.issue, '#1496');
assert.equal(fixture.rungGate.ladderId, 'game-complexity-ladder-v1');
assert.equal(fixture.rungGate.rungId, 'collect-and-exit');
assert.equal(fixture.rungGate.status, 'satisfied');
assert.match(ladder, /\*\*Collect-and-exit\*\*/);

assert.ok(fixture.loopProducedEvidence.length >= 4, 'loop evidence is present');
for (const evidence of fixture.loopProducedEvidence) {
  assert.equal(evidence.producedByLoop, true, `${evidence.id} is loop-produced`);
  assert.ok(Array.isArray(evidence.covers) && evidence.covers.length > 0, `${evidence.id} records coverage`);
}

for (const [gateName, gate] of Object.entries(fixture.fourGateEvidence)) {
  assert.match(gate.status, /^(pass|satisfied)$/, `${gateName} gate passes`);
  assert.ok(gate.evidenceRefs.length > 0, `${gateName} cites evidence`);
}

assert.equal(fixture.loopCoverage.status, 'satisfied');
assert.equal(fixture.loopCoverage.verdict, 'pass');
assert.match(fixture.loopCoverage.coveredPath, /collect-and-exit/i);
assert.ok(fixture.loopCoverage.evidenceRefs.length >= 3, 'loop coverage cites concrete refs');

assert.equal(fixture.engineGrowthJustification.status, 'none');
assert.deepEqual(fixture.engineGrowthJustification.newEngineCapabilities, []);
assert.deepEqual(fixture.engineGrowthJustification.unjustifiedCapabilities, []);
assert.match(fixture.engineGrowthJustification.rationale, /No new renderer/);

assert.equal(fixture.guardrails.deterministic, true);
assert.equal(fixture.guardrails.fixtureScoped, true);
assert.equal(fixture.guardrails.generatedState, false);
assert.equal(fixture.guardrails.sourceControlled, true);
assert.equal(fixture.guardrails.network, false);
assert.equal(fixture.guardrails.liveBrowser, false);
assert.equal(fixture.guardrails.browserTrustedWrites, false);
assert.equal(fixture.guardrails.browserCommandBridge, false);
assert.equal(fixture.guardrails.autoApply, false);
assert.equal(fixture.guardrails.autoMerge, false);
assert.equal(fixture.guardrails.modifiesIssue1, false);
assert.equal(fixture.guardrails.modifiesIssue23, false);
assert.equal(fixture.guardrails.touchesFutureIssues1497Or1498, false);
assert.equal(fixture.demo.supersededTreesRecreated, false);

for (const ref of collectEvidenceRefs(fixture)) assertRepoRefExists(ref);

assert.match(docs, /fixture-scoped/i);
assert.match(docs, /one rung/i);
assert.match(docs, /rung gate status: `satisfied`/i);
assert.match(docs, /no new engine growth/i);
assert.match(docs, /Signal Gate \/ Collect-and-Exit/i);
assert.match(docs, /#1 and #23 remain open/i);
assert.match(docs, /#1497 or #1498/i);
assert.doesNotMatch(
  docs,
  /full Godot parity|Godot replacement|production-ready|commercial release|browser trusted write|command bridge|network-required/i,
);

for (const name of ['runs', 'target', 'dashboard-data', 'dist']) {
  assert.equal(fs.existsSync(path.join(__dirname, name)), false, `${name} must not exist in the rung fixture`);
}

console.log('game complexity ladder v1 rung demo smoke passed');

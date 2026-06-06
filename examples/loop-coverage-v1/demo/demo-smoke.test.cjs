const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixturesDir = path.join(__dirname, '..', 'fixtures');

function fixture(name) {
  return JSON.parse(fs.readFileSync(path.join(fixturesDir, name), 'utf8'));
}

function countInputs(inputs) {
  const counts = { loopProduced: 0, loopVerified: 0, manual: 0, totalTrusted: 0 };
  for (const input of inputs || []) {
    if (input.attributionStatus && input.attributionStatus !== 'classified') continue;
    if (input.provenanceClass === 'loop-produced') counts.loopProduced += 1;
    if (input.provenanceClass === 'loop-verified') counts.loopVerified += 1;
    if (input.provenanceClass === 'manual') counts.manual += 1;
  }
  counts.totalTrusted = counts.loopProduced + counts.loopVerified + counts.manual;
  return counts;
}

function fractions(counts) {
  if (!counts.totalTrusted) {
    return { loopProduced: 0, loopVerified: 0, manual: 0, loopCovered: 0 };
  }
  return {
    loopProduced: counts.loopProduced / counts.totalTrusted,
    loopVerified: counts.loopVerified / counts.totalTrusted,
    manual: counts.manual / counts.totalTrusted,
    loopCovered: (counts.loopProduced + counts.loopVerified) / counts.totalTrusted,
  };
}

function closeTo(actual, expected, message) {
  assert.ok(Math.abs(actual - expected) < 0.000001, `${message}: expected ${expected}, got ${actual}`);
}

function assertFixture(name, expectedState) {
  const artifact = fixture(name);
  assert.equal(artifact.schemaVersion, 'loop-coverage-metric-v1', name);
  assert.equal(artifact.verdict.state, expectedState, name);
  assert.match(artifact.boundary, /descriptive authorship attribution only/);
  assert.match(artifact.boundary, /not quality/);
  assert.match(artifact.boundary, /not production readiness/);
  assert.match(artifact.boundary, /no auto-apply/);
  assert.match(artifact.boundary, /read-only/);

  const expectedCounts = countInputs(artifact.inputs);
  assert.deepEqual(artifact.counts, expectedCounts, `${name} counts`);
  const expectedFractions = fractions(expectedCounts);
  closeTo(artifact.fractions.loopProduced, expectedFractions.loopProduced, `${name} loopProduced`);
  closeTo(artifact.fractions.loopVerified, expectedFractions.loopVerified, `${name} loopVerified`);
  closeTo(artifact.fractions.manual, expectedFractions.manual, `${name} manual`);
  closeTo(artifact.fractions.loopCovered, expectedFractions.loopCovered, `${name} loopCovered`);
  return artifact;
}

const baseline = assertFixture('baseline-loop-covered.json', 'computed');
const computed = assertFixture('computed-current.json', 'computed');
const manualDrop = assertFixture('manual-drop-regressed.json', 'regressed');
const insufficient = assertFixture('insufficient-no-baseline.json', 'insufficient-data');
const staleRef = assertFixture('stale-ref.json', 'insufficient-data');
const unsupported = assertFixture('unsupported-kind.json', 'unsupported');

assert.equal(computed.baselineRef, 'examples/loop-coverage-v1/fixtures/baseline-loop-covered.json');
assert.equal(computed.verdict.baselineLoopCovered, baseline.fractions.loopCovered);
assert.equal(computed.verdict.currentLoopCovered, computed.fractions.loopCovered);
assert.ok(computed.verdict.reasons.some((reason) => /without regression/.test(reason)));

assert.equal(manualDrop.counts.manual, 2);
assert.ok(
  manualDrop.verdict.baselineLoopCovered - manualDrop.verdict.currentLoopCovered > manualDrop.verdict.dropThreshold,
  'manual-drop fixture must cross the regression threshold',
);
assert.ok(manualDrop.verdict.reasons.some((reason) => /dropped by/.test(reason)));

assert.equal(insufficient.baselineRef, undefined);
assert.ok(insufficient.verdict.reasons.some((reason) => /no baseline/.test(reason)));
assert.equal(staleRef.counts.totalTrusted, 0);
assert.ok(staleRef.verdict.reasons.some((reason) => /stale-ref/.test(reason)));

assert.equal(unsupported.counts.totalTrusted, 0);
assert.ok(unsupported.verdict.reasons.some((reason) => /unsupported attribution kind/.test(reason)));

console.log('loop coverage v1 demo smoke test passed');

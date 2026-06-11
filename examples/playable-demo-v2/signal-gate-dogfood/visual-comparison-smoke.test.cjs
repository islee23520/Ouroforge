#!/usr/bin/env node
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const repoRoot = path.resolve(root, '..', '..', '..');
const fixturePath = path.join(root, 'visual-comparison-signal-gate-relay.fixture.json');
const sampleUnchangedPath = path.join(
  repoRoot,
  'examples/visual-comparison-evidence-v1/visual-comparison-unchanged.sample.json',
);
const sampleChangedPath = path.join(
  repoRoot,
  'examples/visual-comparison-evidence-v1/visual-comparison-changed.sample.json',
);
const evidenceDocPath = path.join(repoRoot, 'docs/evidence/visual-pixel-threshold-signal-gate-2495.md');

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

const fixture = readJson(fixturePath);
const unchangedSample = readJson(sampleUnchangedPath);
const changedSample = readJson(sampleChangedPath);

const comparisonFieldKeys = new Set(
  Object.keys(unchangedSample).filter((key) => !['guardrails', 'metadataRefs', 'changedRegions'].includes(key)),
);

assert.equal(fixture.schemaVersion, 'signal-gate-visual-comparison-fixture-v1');
assert.equal(fixture.issue, 2495);
assert.equal(fixture.fixtureScoped, true);
assert.deepEqual(fixture.viewport, { width: 756, height: 469 });
assert.equal(fixture.deviceScaleFactor, 1);
assert.equal(fixture.colorScheme, 'light');
assert.match(fixture.sourceUrl, /signal-gate-relay\.scene\.json/);
assert.match(fixture.liveEntryPointUrl, /signal-gate-relay\.scene\.json/);
assert.equal(fixture.liveEntryPointUrl, fixture.sourceUrl);

const stateNames = fixture.screenshotStates.map((entry) => entry.state);
assert.deepEqual(stateNames, ['start', 'key-gate', 'win-exit']);

const pixelThreshold = fixture.thresholds.find((entry) => entry.thresholdId === 'pixel-threshold');
assert.ok(pixelThreshold, 'fixture declares pixel-threshold');
assert.equal(typeof pixelThreshold.maxChangedPixels, 'number');
assert.ok(pixelThreshold.maxChangedPixels > 0);
assert.match(pixelThreshold.rationale, /756x469|354,564|anti-aliasing|regression/i);

assert.ok(Array.isArray(fixture.comparisons) && fixture.comparisons.length === 3);
for (const comparison of fixture.comparisons) {
  for (const key of comparisonFieldKeys) {
    assert.ok(Object.prototype.hasOwnProperty.call(comparison, key), `comparison missing ${key}`);
  }
  assert.equal(comparison.schemaVersion, 'visual-comparison-evidence-v1');
  assert.equal(comparison.scenarioId, 'signal-gate-relay');
  assert.ok(stateNames.includes(comparison.checkpointId));
  assert.equal(comparison.before.format, 'png');
  assert.equal(comparison.after.format, 'png');
  assert.equal(comparison.before.width, 756);
  assert.equal(comparison.before.height, 469);
  assert.deepEqual(comparison.before, {
    screenshotRef: comparison.before.screenshotRef,
    format: 'png',
    width: 756,
    height: 469,
  });
  assert.deepEqual(comparison.after, {
    screenshotRef: comparison.after.screenshotRef,
    format: 'png',
    width: 756,
    height: 469,
  });
  const threshold = comparison.thresholds.find((entry) => entry.thresholdId === 'pixel-threshold');
  assert.ok(threshold);
  assert.equal(threshold.maxChangedPixels, pixelThreshold.maxChangedPixels);
  assert.equal(threshold.maxChangedPercentX1000, pixelThreshold.maxChangedPercentX1000);
  assert.ok(comparison.evidenceRefs.every((ref) => ref.startsWith('runs/issue-2495/')));
  const comparisonBody = { ...comparison };
  delete comparisonBody.guardrails;
  assert.doesNotMatch(
    JSON.stringify(comparisonBody),
    /aesthetic score|fun score|production-ready claim|auto-apply enabled|auto-merge enabled/i,
  );
}

// Sample parity: thresholds shape matches visual-comparison-evidence-v1 samples.
for (const sample of [unchangedSample, changedSample]) {
  assert.equal(sample.thresholds[0].thresholdId, 'pixel-threshold');
  assert.equal(typeof sample.thresholds[0].maxChangedPixels, 'number');
  assert.equal(typeof sample.thresholds[0].maxChangedPercentX1000, 'number');
}

assert.ok(Array.isArray(fixture.requiredTrackedArtifacts));
for (const artifact of fixture.requiredTrackedArtifacts) {
  assert.ok(artifact.startsWith('examples/') || artifact.startsWith('docs/'), artifact);
  assert.equal(artifact.startsWith('runs/'), false, `runs/ must not be a required tracked artifact: ${artifact}`);
}

for (const state of fixture.screenshotStates) {
  for (const ref of [state.baselineScreenshotRef, state.actualScreenshotRef]) {
    assert.match(ref, /^runs\//, `${state.state} generated refs stay under ignored runs/`);
  }
}

assert.ok(fs.existsSync(evidenceDocPath));
const evidenceDoc = fs.readFileSync(evidenceDocPath, 'utf8');
assert.match(evidenceDoc, /Closure classification:\s*product-observed complete/i);
assert.match(evidenceDoc, /not.*pixel-perfect/i);

for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
  assert.equal(fs.existsSync(path.join(root, name)), false, `${name} must stay untracked under dogfood fixture`);
}

console.log('signal-gate visual comparison smoke passed');
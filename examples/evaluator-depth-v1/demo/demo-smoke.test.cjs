const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const seed = fs.readFileSync(path.join(root, '../../../seeds/evaluator-depth-v1-demo.yaml'), 'utf8');
const docs = fs.readFileSync(path.join(root, '../../../docs/evaluator-depth-v1-demo.md'), 'utf8');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(root, relativePath), 'utf8'));
}

function assertFourCategoryShape(verdict) {
  for (const gate of ['mechanical', 'runtime', 'visual', 'semantic']) {
    assert.equal(typeof verdict.gateCategories[gate].declared, 'boolean', `${gate} declared is boolean`);
    assert.match(verdict.gateCategories[gate].status, /^(pass|fail)$/);
    assert.equal(Number.isInteger(verdict.gateCategories[gate].resultCount), true);
    assert.equal(Number.isInteger(verdict.gateCategories[gate].failureCount), true);
  }
  assert.equal(verdict.gateCategories.aggregation.operator, 'declared-gate-and');
  assert.equal(verdict.gateCategories.aggregation.undeclaredGatePolicy, 'neutral');
}

function assertEvidenceExists(runDir, refs) {
  for (const ref of refs) {
    assert.equal(fs.existsSync(path.join(root, runDir, ref)), true, `${runDir}/${ref} exists`);
  }
}

const visual = readJson('visual-fail-run/verdict.json');
const semantic = readJson('semantic-fail-run/verdict.json');

assert.match(seed, /visual-mismatch/);
assert.match(seed, /semantic-invariant/);
assert.match(docs, /pure visual failure/i);
assert.match(docs, /pure semantic failure/i);
assert.doesNotMatch(docs, /production-ready engine is available|current Godot replacement is implemented|auto-fix enabled|auto-apply enabled|auto-merge enabled/i);

assert.equal(visual.status, 'failed');
assertFourCategoryShape(visual);
assert.equal(visual.gateCategories.mechanical.status, 'pass');
assert.equal(visual.gateCategories.runtime.status, 'pass');
assert.equal(visual.gateCategories.visual.status, 'fail');
assert.equal(visual.gateCategories.semantic.status, 'pass');
assert.equal(visual.failures.length, 1);
assert.equal(visual.failures[0].kind, 'visual_gate_failed');
assert.match(visual.failures[0].path, /visual-comparison\.json/);
assert.equal(visual.visual[0].state, 'fail');
assert.equal(visual.visual[0].changedRegionCount, 1);
assert.match(visual.visual[0].thresholdSummary.join(' '), /demo-pixel-threshold/);
assert.equal(visual.semantic[0].state, 'pass');
assertEvidenceExists('visual-fail-run', [
  visual.visual[0].comparisonRef,
  ...visual.visual[0].evidenceRefs,
  visual.semantic[0].modelRef,
  visual.semantic[0].worldStateRef,
]);

assert.equal(semantic.status, 'failed');
assertFourCategoryShape(semantic);
assert.equal(semantic.gateCategories.mechanical.status, 'pass');
assert.equal(semantic.gateCategories.runtime.status, 'pass');
assert.equal(semantic.gateCategories.visual.status, 'pass');
assert.equal(semantic.gateCategories.semantic.status, 'fail');
assert.equal(semantic.failures.length, 1);
assert.equal(semantic.failures[0].kind, 'semantic_gate_failed');
assert.equal(semantic.visual[0].state, 'pass');
assert.equal(semantic.semantic[0].state, 'fail');
assert.match(semantic.semantic[0].reason, /player\.health is -1/);
assertEvidenceExists('semantic-fail-run', [
  semantic.visual[0].comparisonRef,
  ...semantic.visual[0].evidenceRefs,
  semantic.semantic[0].modelRef,
  semantic.semantic[0].worldStateRef,
]);

console.log('evaluator depth demo smoke passed');

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const repoRoot = path.resolve(root, '..', '..');
const readJson = (relative) => JSON.parse(fs.readFileSync(path.join(root, relative), 'utf8'));
const existsRepo = (relative) => fs.existsSync(path.join(repoRoot, relative));

const bundle = readJson('evidence/behavior-evidence-bundle.fixture.json');
const events = readJson('evidence/runtime-events.fixture.json');
const outcome = readJson('evidence/scenario-outcome.fixture.json');
const review = readJson('evidence/review-decision.fixture.json');
const rollback = readJson('evidence/rollback-metadata.fixture.json');
const rerun = readJson('evidence/rerun-comparison.fixture.json');
const expectedJournal = fs.readFileSync(path.join(root, 'journal/behavior-evidence-journal.fragment.md'), 'utf8');
const gitignore = fs.readFileSync(path.join(root, '.gitignore'), 'utf8');

assert.equal(bundle.schemaVersion, 'behavior-evidence-bundle-v1');
assert.equal(bundle.bundleId, 'gameplay-logic-demo-v1-evidence-flow');
assert.equal(bundle.status, 'complete');
assert.equal(bundle.observedFailures.length, 0);
assert.equal(bundle.nextStepHypotheses[0].id, 'inspect-read-model-next');

for (const group of [
  bundle.behaviorDefinitionRefs,
  bundle.runtimeEventRefs,
  bundle.scenarioOutcomeRefs,
  bundle.draftRefs,
  bundle.reviewDecisionRefs,
  bundle.applyTransactionRefs,
  bundle.rollbackMetadataRefs,
  bundle.rerunComparisonRefs,
]) {
  assert.equal(group.length, 1);
  assert.ok(existsRepo(group[0].path), `linked evidence exists: ${group[0].path}`);
}
assert.equal(bundle.linkedEvidence.length, 8);

assert.deepEqual(events.events.map((event) => event.type), ['keyCollected', 'doorOpened', 'patrolHazard', 'victory']);
assert.deepEqual(outcome.scenarioResults.map((scenario) => scenario.status), ['passed', 'passed', 'passed']);
assert.equal(review.status, 'accepted');
assert.equal(rollback.status, 'not_required_fixture_only');
assert.equal(rerun.status, 'stable');

for (const rootName of ['runs', 'dashboard-data', 'screenshots', 'browser-profiles', 'tmp-evidence']) {
  assert.match(gitignore, new RegExp(`(^|\\n)${rootName}/`), `${rootName} stays ignored`);
}

const serialized = JSON.stringify({ bundle, events, outcome, review, rollback, rerun, expectedJournal });
assert.doesNotMatch(serialized, /execute_script|eval\(|dynamic_import|plugin_loader|commandBridge|trustedWrite|localStorage|showSaveFilePicker|autoApply|autoMerge|selfApproval/);
assert.match(serialized, /no arbitrary script execution/);
assert.doesNotMatch(serialized, /production-ready engine|current Godot replacement|production-stable scripting API is implemented|secure sandbox is implemented|native export ready|plugin runtime enabled/);
assert.match(expectedJournal, /Lifecycle refs: definitions `1`, runtime events `1`, scenario outcomes `1`/);
assert.match(expectedJournal, /Observed failures: none recorded/);

console.log('gameplay logic demo evidence flow smoke passed');

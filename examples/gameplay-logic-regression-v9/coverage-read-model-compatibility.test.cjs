const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const dashboard = require('../evidence-dashboard/dashboard.js');
const cockpit = require('../authoring-cockpit/cockpit.js');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, relativePath), 'utf8'));
}
function assertNoGeneratedFixtureState() {
  for (const name of ['runs', 'dashboard-data', 'target', 'tmp']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} remains generated/untracked`);
  }
}

const matrix = readJson('coverage-matrix.json');
assert.equal(matrix.issue, 624);
assert.match(matrix.scope, /structured gameplay logic/);
assert.ok(matrix.guardrails.some((guardrail) => /#1 and #23 remain open/.test(guardrail)));
assert.ok(matrix.guardrails.some((guardrail) => /no command bridge|browser.*read-only/i.test(guardrail)));

const requiredFeatures = [
  'behavior-validation-regressions',
  'event-ordering-signal-routing',
  'state-machine-transitions',
  'ability-cooldown-effects',
  'runtime-execution-evidence',
  'draft-validation',
  'review-gated-apply',
  'evidence-journal-lifecycle',
  'stale-evidence-visibility',
  'dashboard-read-model-compatibility',
  'studio-read-model-compatibility',
  'generated-state-audit',
];
const byId = new Map(matrix.features.map((feature) => [feature.id, feature]));
for (const featureId of requiredFeatures) {
  const feature = byId.get(featureId);
  assert.ok(feature, `${featureId} is present in coverage matrix`);
  assert.equal(feature.status, 'covered', `${featureId} is covered`);
  assert.ok(feature.scenarioIds.length > 0, `${featureId} links scenarios`);
  assert.ok(feature.evidenceRefs.length > 0, `${featureId} links evidence refs`);
  for (const ref of feature.evidenceRefs) {
    assert.equal(fs.existsSync(path.join(repoRoot, ref)), true, `${featureId} evidence ref exists: ${ref}`);
  }
}
assert.ok(matrix.knownGaps.some((gap) => gap.id === 'arbitrary-executable-scripting' && gap.status === 'out-of-scope'));
assert.ok(matrix.knownGaps.some((gap) => gap.id === 'trusted-browser-writes' && /read-only/.test(gap.reason)));

const compatibility = readJson('read-model-compatibility.fixture.json');
assert.equal(compatibility.schemaVersion, 'scenario-coverage-v9-read-model-compatibility-v1');
const run = compatibility.run;

const dashboardLifecycle = dashboard.renderBehaviorEvidenceLifecycle(run);
assert.match(dashboardLifecycle, /Behavior evidence lifecycle/);
assert.match(dashboardLifecycle, /gameplay-logic-regression-v9-draft-apply-evidence/);
assert.match(dashboardLifecycle, /complete/);
assert.match(dashboardLifecycle, /no arbitrary script execution/);
assert.doesNotMatch(dashboardLifecycle, /trusted browser write enabled|command bridge enabled|auto-apply enabled/i);

const studioLifecycle = cockpit.renderBehaviorEvidenceLifecycleSurface(run);
assert.match(studioLifecycle, /Behavior evidence lifecycle/);
assert.match(studioLifecycle, /gameplay-logic-regression-v9-draft-apply-evidence/);
assert.match(studioLifecycle, /lifecycle refs/i);
assert.match(studioLifecycle, /no arbitrary script execution/);

assert.match(cockpit.renderBehaviorDraftStatusSurface(run), /Behavior draft status/);
assert.match(cockpit.renderBehaviorDraftStatusSurface(run), /draft-gameplay-logic-regression-v9-routing/);
assert.match(cockpit.renderBehaviorListPanel(run), /pressure-plate-signal/);
assert.match(cockpit.renderBehaviorEventSignalPanel(run), /gateSignal/);
assert.match(cockpit.renderBehaviorStateMachinePanel(run), /pressure-plate-state/);
assert.match(cockpit.renderBehaviorAbilityActionPanel(run), /dash-vfx/);
assert.match(cockpit.renderBehaviorReviewApplyStatusSurface(run), /readyForTrustedApply/);
assert.doesNotMatch(cockpit.renderBehaviorReviewApplyStatusSurface(run), /self-approval enabled|auto-apply enabled|command bridge enabled/i);

const escaped = cockpit.renderBehaviorListPanel({
  behavior_inspection: {
    present: true,
    behaviors: [{ id: '<script>bad</script>', status: 'ready', actions: [{ kind: '<img>' }] }],
  },
});
assert.match(escaped, /&lt;script&gt;bad&lt;\/script&gt;/);
assert.doesNotMatch(escaped, /<script>bad<\/script>|<img>/);

assertNoGeneratedFixtureState();
console.log('scenario coverage v9 coverage/read-model compatibility passed');

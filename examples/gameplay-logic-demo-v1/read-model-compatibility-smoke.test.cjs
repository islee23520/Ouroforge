const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const dashboard = require('../evidence-dashboard/dashboard.js');
const cockpit = require('../authoring-cockpit/cockpit.js');

const root = __dirname;
const repoRoot = path.resolve(root, '..', '..');
const readJson = (relative) => JSON.parse(fs.readFileSync(path.join(root, relative), 'utf8'));
const readRepoJson = (relative) => JSON.parse(fs.readFileSync(path.join(repoRoot, relative), 'utf8'));
const readText = (relative) => fs.readFileSync(path.join(root, relative), 'utf8');
const existsRepo = (relative) => fs.existsSync(path.join(repoRoot, relative));

const compatibility = readJson('read-model-compatibility.fixture.json');
const inspection = readJson('behavior-inspection-demo.json');
const bundle = readJson('evidence/behavior-evidence-bundle.fixture.json');
const readme = readText('README.md');
const docs = readText('read-model-compatibility.md');
const compatibilityDoc = readText('read-model-compatibility.md');
const gitignore = readText('.gitignore');

assert.equal(compatibility.schemaVersion, 'gameplay-logic-demo-read-model-compatibility-v1');
assert.equal(compatibility.status, 'compatible');
assert.equal(compatibility.dashboard.surface, 'examples/evidence-dashboard/dashboard.js');
assert.equal(compatibility.studio.surface, 'examples/authoring-cockpit/cockpit.js');

for (const ref of compatibility.dashboard.requiredRefs) {
  assert.ok(existsRepo(ref), `dashboard required ref exists: ${ref}`);
}
assert.ok(compatibility.dashboard.expectedSections.includes('behaviorEvidenceBundle'));
assert.ok(compatibility.dashboard.expectedSections.includes('journalFragment'));

for (const panel of inspection.studioPanels) {
  assert.ok(compatibility.studio.expectedPanels.includes(panel), `Studio panel remains compatible: ${panel}`);
}
for (const field of ['behaviorId', 'scenarioStatus', 'evidenceRef', 'generatedStatePolicy']) {
  assert.ok(compatibility.studio.displayOnlyFields.includes(field), `display-only field listed: ${field}`);
}

assert.equal(bundle.status, 'complete');
assert.equal(bundle.linkedEvidence.length, 8);
assert.match(compatibility.studio.emptyAndMalformedPolicy, /visible warning or empty state/);
assert.match(compatibility.studio.emptyAndMalformedPolicy, /no browser mutation/);

const behaviorEvidenceRun = {
  behavior_evidence: {
    present: true,
    status: 'ready',
    bundle_count: 1,
    malformed_count: 0,
    lifecycle_ref_count: bundle.linkedEvidence.length,
    observed_failure_count: bundle.observedFailures.length,
    next_step_hypothesis_count: bundle.nextStepHypotheses.length,
    boundary: 'read-only structured behavior lifecycle evidence; no command bridge, auto-apply, or trusted writes.',
    bundles: [{
      bundle_id: bundle.bundleId,
      status: bundle.status,
      path: 'examples/gameplay-logic-demo-v1/evidence/behavior-evidence-bundle.fixture.json',
      lifecycle_ref_count: bundle.linkedEvidence.length,
      observed_failures: bundle.observedFailures,
      next_step_hypotheses: bundle.nextStepHypotheses,
      blocked_reasons: bundle.blockedReasons,
      evidence_refs: bundle.linkedEvidence.map((ref) => ref.path),
      guardrails: bundle.guardrails,
    }],
  },
};

const dashboardMarkup = dashboard.renderBehaviorEvidenceLifecycle(behaviorEvidenceRun);
assert.match(dashboardMarkup, /Behavior evidence lifecycle/);
assert.match(dashboardMarkup, /gameplay-logic-demo-v1-evidence-flow/);
assert.match(dashboardMarkup, /inspect-read-model-next/);
assert.match(dashboardMarkup, /No observed failures recorded/);
assert.match(dashboardMarkup, /read-only structured behavior lifecycle evidence/);
assert.doesNotMatch(dashboardMarkup, /<script>|<img|<button|applyCommand|mergeCommand|browserCommandBridge|executeCommand|localStorage|showSaveFilePicker/);

const studioLifecycleMarkup = cockpit.renderBehaviorEvidenceLifecycleSurface(behaviorEvidenceRun);
assert.match(studioLifecycleMarkup, /Behavior evidence lifecycle/);
assert.match(studioLifecycleMarkup, /gameplay-logic-demo-v1-evidence-flow/);
assert.match(studioLifecycleMarkup, /inspect-read-model-next/);
assert.doesNotMatch(studioLifecycleMarkup, /<script>|<img|<button|applyCommand|mergeCommand|browserCommandBridge|executeCommand|localStorage|showSaveFilePicker/);

const behaviorModel = readRepoJson(inspection.behaviorModelRef);
const eventSignals = readRepoJson(inspection.eventSignalRef);
const stateMachines = readRepoJson(inspection.stateMachineRef);
const abilities = readRepoJson(inspection.abilityActionRef);
const reviewDecision = readRepoJson('examples/gameplay-logic-demo-v1/evidence/review-decision.fixture.json');
const applyTransaction = readRepoJson('examples/behavior-apply-v1/valid/behavior-apply.ready.json');
const behaviorInspectionRun = {
  behavior_inspection: {
    present: true,
    status: 'ready',
    boundary: inspection.boundary,
    behaviors: behaviorModel,
    eventSignals,
    stateMachines,
    abilities,
    reviewApply: {
      status: 'ready',
      reviews: [reviewDecision],
      applies: [applyTransaction],
    },
  },
};
const model = cockpit.behaviorInspectionModel(behaviorInspectionRun);
assert.equal(model.present, true);
assert.ok(model.behaviors.some((behavior) => behavior.id === 'collect-keycard'));
assert.ok(model.events.some((event) => event.id === 'player-spike-contact'));
assert.ok(model.stateMachines.some((machine) => machine.id === 'player-dash-state'));
assert.ok(model.abilities.some((ability) => ability.id === 'player-dash'));
assert.equal(model.reviews.length, 1);
assert.equal(model.applies.length, 1);

const studioPanels = [
  cockpit.renderBehaviorListPanel(behaviorInspectionRun),
  cockpit.renderBehaviorEventSignalPanel(behaviorInspectionRun),
  cockpit.renderBehaviorStateMachinePanel(behaviorInspectionRun),
  cockpit.renderBehaviorAbilityActionPanel(behaviorInspectionRun),
  cockpit.renderBehaviorReviewApplyStatusSurface(behaviorInspectionRun),
].join('\n');
assert.match(studioPanels, /collect-keycard/);
assert.match(studioPanels, /player-spike-contact/);
assert.match(studioPanels, /player-dash-state/);
assert.match(studioPanels, /player-dash/);
assert.match(studioPanels, /review-gl10-13-2-evidence-flow/);
assert.doesNotMatch(studioPanels, /<script>|<img|<button|onclick|applyCommand|mergeCommand|browserCommandBridge|executeCommand|localStorage|showSaveFilePicker/);


for (const command of compatibility.commands) {
  assert.doesNotMatch(command, /rm |curl|npm install|gh pr merge|git push|eval|python -m http\.server/);
}
for (const rootName of compatibility.cleanupPolicy.ignoredGeneratedRoots) {
  assert.match(gitignore, new RegExp(`(^|\n)${rootName}/`), `${rootName} remains ignored`);
}
assert.match(readme, /Expected evidence/);
assert.match(readme, /Known gaps/);
assert.match(readme, /Cleanup policy/);
assert.match(readme, /Read-model compatibility/);
assert.match(docs, /Dashboard compatibility/);
assert.match(docs, /Studio compatibility/);
assert.match(compatibilityDoc, /Dashboard compatibility/);
assert.match(compatibilityDoc, /Studio compatibility/);
assert.match(compatibilityDoc, /Cleanup policy/);

const serialized = JSON.stringify({ compatibility, readme, compatibilityDoc, inspection });
assert.match(serialized, /read-only inspection/);
assert.doesNotMatch(serialized, /execute_script|eval\(|dynamic_import|plugin_loader|commandBridge|trustedWrite|localStorage|showSaveFilePicker|autoApply|autoMerge|selfApproval/);
assert.doesNotMatch(serialized, /production-ready engine|current Godot replacement|production-stable scripting API is implemented|secure sandbox is implemented|native export ready|plugin runtime enabled/);

console.log('gameplay logic demo read-model compatibility smoke passed');

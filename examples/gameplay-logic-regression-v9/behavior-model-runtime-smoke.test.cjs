const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, relativePath), 'utf8'));
}

const project = readJson('ouroforge.project.json');
const pack = readJson('scenarios/gameplay-logic-regression-v9.scenario-pack.json');
const behavior = readJson('behaviors/gameplay-logic-regression-v9.behavior.json');
const suite = readJson('scenarios/gameplay-logic-regression-v9.behavior-assertions.json');

assert.equal(project.project.id, 'scenario_coverage_v9_gameplay_logic');
assert.deepEqual(project.generated.roots.sort(), ['dashboard-data', 'runs', 'target', 'tmp'].sort());
assert.equal(pack.id, 'gameplay-logic-regression-v9');
assert.equal(pack.scenarioGroups[0].id, 'gl10-14-1-behavior-model-runtime');
assert.equal(pack.scenarioGroups[0].scenarios[0].id, 'behavior-model-runtime-regression');
assert.ok(fs.existsSync(path.join(fixtureDir, 'scenarios/gameplay-logic-regression-v9.behavior-assertions.json')));
assert.ok(fs.existsSync(path.join(fixtureDir, 'behaviors/gameplay-logic-regression-v9.behavior.json')));

assert.equal(behavior.schemaVersion, 'ouroforge.behavior-artifact.v1');
assert.equal(behavior.validatedBy.authority, 'ouroforge-rust-local-validator');
assert.equal(behavior.validatedBy.validationStatus, 'passed');
assert.equal(behavior.behaviors.length, 4);

const behaviorIds = new Set(behavior.behaviors.map((entry) => entry.id));
for (const required of ['pressure-plate-signal', 'dash-ability-regression', 'hazard-state-pulse', 'exit-terminal-check']) {
  assert.ok(behaviorIds.has(required), `${required} behavior is present`);
}

const allTriggers = behavior.behaviors.flatMap((entry) => entry.triggers.map((trigger) => trigger.kind));
assert.ok(allTriggers.includes('onCollision'));
assert.ok(allTriggers.includes('onInputAction'));
assert.ok(allTriggers.includes('onEvent'));
assert.ok(behavior.behaviors.some((entry) => entry.stateMachine), 'state machine coverage exists');
assert.ok(behavior.behaviors.some((entry) => entry.abilities && entry.abilities.length > 0), 'ability coverage exists');
assert.ok(behavior.behaviors.some((entry) => entry.conditions.some((condition) => condition.kind === 'cooldownReady')), 'cooldown-ready condition coverage exists');

const actionKinds = new Set(behavior.behaviors.flatMap((entry) => [
  ...entry.actions.map((action) => action.kind),
  ...(entry.abilities || []).flatMap((ability) => ability.actions.map((action) => action.kind)),
]));
for (const required of ['setFlag', 'emitEvent', 'moveEntity', 'startAudioIntent', 'startAnimationIntent', 'damage', 'markWinState']) {
  assert.ok(actionKinds.has(required), `${required} action coverage exists`);
}
for (const forbidden of ['executeScript', 'eval', 'dynamicImport', 'pluginLoader', 'commandBridge']) {
  assert.equal(actionKinds.has(forbidden), false, `${forbidden} is not a structured behavior action`);
}

assert.equal(suite.schemaVersion, 'ouroforge.behavior-scenario-assertion-suite.v1');
assert.equal(suite.assertions.length, 12);
const assertionKinds = new Set(suite.assertions.map((assertion) => assertion.kind));
for (const required of ['behaviorExecuted', 'eventEmitted', 'flagChanged', 'stateTransitionOccurred', 'abilityUsed', 'cooldownActive', 'entityAffected', 'terminalStateReached']) {
  assert.ok(assertionKinds.has(required), `${required} assertion coverage exists`);
}

for (const name of ['runs', 'dashboard-data', 'target', 'tmp']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} remains generated/untracked`);
}

console.log('scenario coverage v9 behavior model/runtime smoke passed');

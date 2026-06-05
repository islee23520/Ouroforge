const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const repoRoot = path.resolve(root, '..', '..');
const readJson = (relative) => JSON.parse(fs.readFileSync(path.join(root, relative), 'utf8'));
const readRepoJson = (relative) => JSON.parse(fs.readFileSync(path.join(repoRoot, relative), 'utf8'));

const project = readJson('ouroforge.project.json');
const scene = readJson('scenes/gameplay-logic-demo.scene.json');
const pack = readJson('scenarios/gameplay-logic-demo.scenario-pack.json');
const inspection = readJson('behavior-inspection-demo.json');
const readme = fs.readFileSync(path.join(root, 'README.md'), 'utf8');
const seed = fs.readFileSync(path.join(root, 'seeds/gameplay-logic-demo.yaml'), 'utf8');
const gitignore = fs.readFileSync(path.join(root, '.gitignore'), 'utf8');

assert.equal(project.schemaVersion, 'project-manifest-v1');
assert.equal(project.project.id, 'gameplay_logic_demo_v1');
assert.deepEqual(project.generated.roots, ['runs', 'dashboard-data', 'screenshots', 'browser-profiles', 'tmp-evidence']);
assert.equal(scene.id, 'gameplay-logic-demo-scene');
assert.equal(pack.id, 'gameplay-logic-demo');
assert.equal(inspection.status, 'ready');

const entityIds = scene.entities.map((entity) => entity.id);
for (const id of ['player', 'blue_keycard', 'blue_door', 'spike_hazard', 'guard_patrol_marker', 'dash_pickup', 'exit_zone', 'hud_goal']) {
  assert.ok(entityIds.includes(id), `scene includes ${id}`);
}

const flags = scene.gameplayRules.flags.map((flag) => flag.id).sort();
assert.deepEqual(flags, ['blue_door_open', 'dash_ready', 'escape_complete', 'guard_patrolling', 'has_blue_keycard', 'hazard_seen']);

const scenarios = pack.scenarioGroups.flatMap((group) => group.scenarios);
assert.deepEqual(scenarios.map((scenario) => scenario.id), ['collect-key', 'open-door-and-exit', 'dash-and-patrol']);
const scenarioCoverage = new Set([
  'item_collection',
  'door_flag_logic',
  'enemy_patrol',
  'hazard',
  'player_ability',
  'win_condition',
]);
for (const beat of inspection.requiredBeats) {
  assert.ok(scenarioCoverage.has(beat), `scenario pack covers ${beat}`);
}
assert.match(JSON.stringify(pack), /key|door|dash|patrol|hazard|win|victory/);

for (const ref of [inspection.behaviorRuntimeRef, inspection.behaviorModelRef, inspection.eventSignalRef, inspection.stateMachineRef, inspection.abilityActionRef, inspection.scenarioPackRef, ...inspection.behaviorDraftRefs, ...inspection.behaviorApplyRefs, ...inspection.behaviorEvidenceRefs]) {
  assert.ok(fs.existsSync(path.join(repoRoot, ref)), `linked ref exists: ${ref}`);
}

const behaviorRuntime = readRepoJson(inspection.behaviorRuntimeRef);
const behaviorModel = readRepoJson(inspection.behaviorModelRef);
const abilityModel = readRepoJson(inspection.abilityActionRef);
assert.equal(behaviorRuntime.schemaVersion, 'ouroforge.behavior-artifact.v1');
assert.ok(behaviorRuntime.behaviors.some((behavior) => behavior.id === 'key-pickup'));
assert.ok(behaviorRuntime.behaviors.some((behavior) => behavior.id === 'door-unlock'));
assert.ok(behaviorRuntime.behaviors.some((behavior) => behavior.id === 'player-dash'));
assert.ok(behaviorRuntime.behaviors.some((behavior) => behavior.id === 'enemy-patrol-hazard'));
assert.ok(behaviorRuntime.behaviors.some((behavior) => behavior.id === 'exit-win-condition'));
assert.ok(behaviorModel.behaviors.some((behavior) => behavior.id === 'collect-keycard'));
assert.ok(behaviorModel.behaviors.some((behavior) => behavior.id === 'open-blue-door'));
assert.ok(behaviorModel.behaviors.some((behavior) => behavior.id === 'win-after-exit'));
assert.ok(abilityModel.abilities.some((ability) => ability.id === 'player-dash'));

const serializedFixture = JSON.stringify({ project, scene, pack, inspection, readme, seed, gitignore });
assert.doesNotMatch(serializedFixture, /execute_script|eval\(|dynamic_import|plugin_loader|commandBridge|trustedWrite|localStorage|showSaveFilePicker|autoApply|autoMerge|selfApproval/);
assert.match(inspection.boundary, /no arbitrary script execution/);
assert.match(inspection.boundary, /no .*command bridge/);
assert.match(readme, /Generated `runs\/`, `dashboard-data\/`, screenshots, browser profiles, temp/);
assert.match(readme, /not arbitrary executable scripting/);
for (const rootName of project.generated.roots) {
  assert.match(gitignore, new RegExp(`(^|\\n)${rootName.replace(/[.*+?^${}()|[\\]\\]/g, '\\$&')}/`), `${rootName} is ignored`);
}
assert.match(seed, /not a production-stable scripting API or Godot replacement/);
assert.doesNotMatch(readme, /production-ready engine|current Godot replacement|production-stable scripting API is implemented|secure sandbox is implemented|native export ready|plugin runtime enabled/);

console.log('gameplay logic demo fixture smoke passed');

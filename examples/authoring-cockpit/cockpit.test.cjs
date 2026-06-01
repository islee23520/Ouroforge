const assert = require('node:assert/strict');
const cockpit = require('./cockpit.js');
const scene = require('../game-runtime/scene.json');

const moved = cockpit.applyEdit(scene, 'player', 'components.transform.x', '48');
assert.equal(cockpit.getValue(moved.entities[0], 'components.transform.x'), 48);
const recolored = cockpit.applyEdit(scene, 'player', 'sprite.color', '#ffffff');
assert.equal(cockpit.getValue(recolored.entities[0], 'sprite.color'), '#ffffff');
assert.throws(() => cockpit.applyEdit(scene, 'player', 'components.size.width', '0'), /Invalid numeric/);
assert.match(cockpit.renderTree(scene, 'player'), /player/);
assert.match(cockpit.renderInspector(scene, 'player'), /components\.transform\.x/);
assert.match(cockpit.cliCommand('examples/game-runtime/scene.json', 'player', 'sprite.color', '#ffffff'), /ouroforge-cli -- scene edit/);

const run = { summary: { id: 'run-1', run_dir: 'runs/run-1', verdict_status: 'passed' }, evidence: [{ id: 'evidence-1' }], mutations: [], screenshots: [{ id: 'shot-1', path: 'evidence/shot.png' }], journal: '# Journal' };
assert.match(cockpit.qaCommand(), /run seeds\/platformer\.yaml --workers 4/);
assert.match(cockpit.dashboardExportCommand(), /dashboard export/);
assert.equal(cockpit.latestRun([{ summary: { id: 'old', created_at_unix_ms: 1 } }, { summary: { id: 'new', created_at_unix_ms: 2 } }]).summary.id, 'new');
assert.match(cockpit.renderPreview(), /iframe/);
assert.match(cockpit.renderQaPanel(), /Run QA/);
assert.match(cockpit.renderEvidencePane(run), /# Journal/);
assert.match(cockpit.renderIntegration(run), /Live browser preview/);

// A cleared numeric input must be rejected, not silently coerced to 0.
assert.throws(() => cockpit.applyEdit(scene, 'player', 'components.transform.x', ''), /Invalid numeric/);

// Scene-derived and dashboard-derived content must be escaped before innerHTML insertion.
const xssScene = {
  entities: [{
    id: '<img src=x onerror=alert(1)>',
    sprite: { color: '#ffffff' },
    components: { transform: { x: 0, y: 0 }, velocity: { x: 0, y: 0 }, size: { width: 1, height: 1 }, controllable: true },
  }],
};
assert.ok(!cockpit.renderTree(xssScene, null).includes('<img src=x onerror'), 'tree entity id must be escaped');
assert.ok(!cockpit.renderInspector(xssScene, xssScene.entities[0].id).includes('<img src=x onerror'), 'inspector entity id must be escaped');
const xssRun = { summary: { id: 'r', verdict_status: 'passed' }, evidence: [], mutations: [], screenshots: [], journal: '<script>alert(1)</script>' };
assert.ok(!cockpit.renderEvidencePane(xssRun).includes('<script>alert(1)</script>'), 'evidence journal must be escaped');
console.log('authoring cockpit smoke test passed');


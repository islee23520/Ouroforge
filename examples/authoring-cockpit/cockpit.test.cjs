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
console.log('authoring cockpit smoke test passed');

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..', '..');
const demo = path.join(root, 'examples/playable-demo-v2/collect-and-exit');

assert.ok(fs.existsSync(path.join(demo, 'export/export-profile.json')), '#791 export profile');
assert.ok(fs.existsSync(path.join(demo, 'export/package-metadata.json')), '#791 package metadata');
assert.ok(fs.existsSync(path.join(demo, 'plugins/collect-and-exit-dashboard-panel/ouroforge.plugin.json')), '#792 plugin descriptor');
const pillars = fs.readFileSync(path.join(root, 'docs/godot-plus-demo-design-pillars-v1.md'), 'utf8');
assert.match(pillars, /Acceptance matrix/i, '#793 comparison matrix');
assert.match(pillars, /Scoped comparison remains honest/i);
const acceptance = fs.readFileSync(path.join(root, 'docs/godot-plus-demo-acceptance-criteria-v1.md'), 'utf8');
assert.match(acceptance, /Frame \/ performance budget/i, '#794 performance budget row');
assert.match(fs.readFileSync(path.join(demo, 'scenes/collect-and-exit.scene.json'), 'utf8'), /frameBudget|runtimeDebug/);
console.log('godot-plus open-issues audit passed');

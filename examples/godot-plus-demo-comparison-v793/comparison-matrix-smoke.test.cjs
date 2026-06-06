const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const doc = fs.readFileSync(path.join(path.resolve(__dirname,'../..'),'docs/godot-plus-demo-capability-comparison-matrix-v1.md'),'utf8');
assert.match(doc, /Comparison Matrix/i);
assert.match(doc, /gameplay-loop-smoke/);
assert.match(doc, /Remains behind Godot/);
assert.doesNotMatch(doc, /claims universal superiority|claims full Godot parity achieved|claims production-ready Godot replacement/i);
console.log('comparison matrix v793 smoke passed');

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const root = path.join(__dirname);
const plugins = ['collect-and-exit-dashboard-panel','collect-and-exit-scenario-template','collect-and-exit-asset-metadata'];
for (const id of plugins) {
  const p = path.join(root, 'plugins', id, 'ouroforge.plugin.json');
  assert.ok(fs.existsSync(p), id);
  const m = JSON.parse(fs.readFileSync(p, 'utf8'));
  assert.match(m.boundary || '', /read-only|Declarative|no executable/i);
  assert.doesNotMatch(JSON.stringify(m), /eval\(|child_process|networkInstall/i);
}
const reg = JSON.parse(fs.readFileSync(path.join(root, 'plugins/registry/demo-plugin-registry-evidence.json'), 'utf8'));
assert.equal(reg.plugins.length, 3);
console.log('plugin usage v792 smoke passed');

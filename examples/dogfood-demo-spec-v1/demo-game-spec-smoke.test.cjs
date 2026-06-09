const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const specPath = path.join(repoRoot, '.omx', 'dogfood-validation', 'demo-game-spec.md');
const spec = fs.readFileSync(specPath, 'utf8');
const normalized = spec.replace(/\s+/g, ' ').trim();

function assertContains(pattern, label) {
  assert.match(spec, pattern, label);
}

function assertPathExists(repoRelativePath) {
  assert.ok(fs.existsSync(path.join(repoRoot, repoRelativePath)), `referenced path exists: ${repoRelativePath}`);
}

const requiredSections = [
  'Spec metadata',
  'Existing demo basis',
  'Player loop and scenarios',
  'Controls and input assumptions',
  'Content inventory',
  'Studio UX author/inspect path',
  'Runtime, performance, and stress budget',
  'Local export and readiness expectation',
  'Retained and generated artifact policy',
  'Lane evidence artifacts expected',
  'Explicit non-goals and forbidden scope',
];

for (const section of requiredSections) {
  assertContains(new RegExp(`## ${section.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}`), `spec includes ${section}`);
}

for (const phrase of [
  'Spec version: `dogfood-demo-spec-v1`',
  'Demo identity: `collect-and-exit-local-rc-candidate`',
  'Source basis: `examples/playable-demo-v2/collect-and-exit/`',
  'CAE-SUCCESS-KEY-EXIT',
  'CAE-FAIL-MISSING-KEY',
  'CAE-STRESS-REPLAY',
  'Keyboard movement',
  'player, key/collectible trigger, exit/door trigger',
  'hud-model.json',
  'runtime/performance budget',
  'local/manual readiness',
  'pipeline-dry-run.md',
  'export-release-readiness.md',
]) {
  assert.ok(normalized.includes(phrase.replace(/\s+/g, ' ')), `spec contains required phrase: ${phrase}`);
}

for (const repoPath of [
  'examples/playable-demo-v2/collect-and-exit/ouroforge.project.json',
  'examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml',
  'examples/playable-demo-v2/collect-and-exit/scenarios/collect-and-exit.json',
  'examples/playable-demo-v2/collect-and-exit/scenarios/demo-scenario-matrix.json',
  'examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json',
  'examples/playable-demo-v2/collect-and-exit/asset-manifest.json',
  'examples/playable-demo-v2/collect-and-exit/export/export-profile.json',
  'examples/playable-demo-v2/collect-and-exit/export/package-metadata.json',
]) {
  assertPathExists(repoPath);
}

for (const guardrail of [
  /Leave #1 and #23 open/i,
  /Do not implement or activate Era Q full-3D M102–M106/i,
  /Do not add hosted\/cloud\/multi-user scope/i,
  /Do not add trusted browser writes or trusted source writes/i,
  /Do not add auto-port, live bridge, foreign runtime embedding/i,
  /Do not add release automation, signing, upload, publishing/i,
  /Do not claim production readiness, store readiness, commercial release readiness/i,
]) {
  assertContains(guardrail, `spec contains guardrail ${guardrail}`);
}

for (const forbiddenAffirmative of [
  /M102(?:–|-| to )M106\s+(?:are|is)?\s*(?:active|implemented|complete|ready)/i,
  /hosted multi-user mode is active/i,
  /trusted browser writes are allowed/i,
  /auto-port is enabled/i,
  /release automation is enabled/i,
  /production-ready/i,
  /store-ready/i,
  /(?:is|as|becomes|achieves) (?:a )?Godot replacement/i,
]) {
  assert.doesNotMatch(spec, forbiddenAffirmative, `spec avoids forbidden affirmative claim ${forbiddenAffirmative}`);
}

console.log('dogfood demo game spec smoke passed');

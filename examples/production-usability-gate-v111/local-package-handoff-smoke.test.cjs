#!/usr/bin/env node
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const repoRoot = path.resolve(root, '..', '..');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(root, relativePath), 'utf8'));
}

function readRepoJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

const handoff = readJson('local-package-inspection-handoff.fixture.json');
const provenance = readJson('local-package-provenance.fixture.json');
const exportProfile = readRepoJson(handoff.exportProfileRef);
const packageMetadata = readRepoJson(handoff.packageMetadataRef);

assert.equal(handoff.schemaVersion, 'local-package-inspection-handoff-v1');
assert.equal(handoff.issue, 2498);
assert.equal(handoff.m130Phase, 2393);
assert.equal(handoff.runtimeProbePreserved, true);
assert.equal(provenance.runtimeProbePreserved, true);

assert.equal(handoff.exportProfileRef, provenance.exportProfileRef);
assert.equal(handoff.checksumsRef, provenance.checksumsRef);
assert.equal(handoff.packageRoot, provenance.packageRoot);
assert.equal(handoff.sourceProjectRef, provenance.sourceProjectRef);
assert.equal(handoff.packageManifestRef, 'dist/local-web/signal-gate-relay/manifest.json');
assert.equal(handoff.localSmokeEvidenceRef, 'runs/issue-2498/local-package-handoff-smoke.json');
assert.deepEqual(handoff.nonGoals, provenance.nonGoals);

assert.equal(exportProfile.packageMetadataRef, handoff.packageMetadataRef);
assert.equal(exportProfile.exportTarget, 'web-local');
assert.equal(exportProfile.runtimeProbeMode, 'preserve');
assert.equal(packageMetadata.schemaVersion, 'export-package-metadata-v1');
assert.ok(fs.existsSync(path.join(repoRoot, handoff.sourceProjectRef)), 'source project ref exists');

assert.ok(Array.isArray(handoff.localSmokeSteps) && handoff.localSmokeSteps.length >= 5);
assert.ok(
  handoff.localSmokeSteps[0].includes('local_package_inspection_handoff'),
  'first local smoke step must generate and verify the handoff package path'
);
for (const step of handoff.localSmokeSteps) {
  assert.match(step, /^(cargo |node |python3 )/, `smoke step must be copyable CLI wording: ${step}`);
}

const handoffText = JSON.stringify(handoff);
const docText = fs.readFileSync(
  path.join(repoRoot, 'docs/evidence/local-package-inspection-handoff-2498.md'),
  'utf8'
);

for (const nonGoal of handoff.nonGoals) {
  assert.ok(handoffText.includes(nonGoal), `handoff preserves non-goal: ${nonGoal}`);
}

const forbiddenAsGoals = [
  /\bgoal\b[^]*\bsigning\b/i,
  /\bgoal\b[^]*\bstore upload\b/i,
  /\bgoal\b[^]*\bpublic release automation\b/i,
];
for (const pattern of forbiddenAsGoals) {
  assert.doesNotMatch(docText, pattern, `evidence doc must not list forbidden scope as a goal: ${pattern}`);
}

assert.doesNotMatch(
  handoffText,
  /"localSmokeSteps":\[[^\]]*(signing|store upload|public release)/i,
  'localSmokeSteps must not include signing/store/upload commands'
);

assert.ok(
  handoff.localSmokeSteps.some((step) => step.includes('local_package_handoff_generates_and_smokes_packaged_artifact')),
  'handoff smoke step must generate package manifest/checksums and diagnostics'
);
assert.ok(
  handoff.localSmokeSteps.some((step) => step.includes('godot_plus_demo_export_package_contract')),
  'export smoke step must reference godot_plus_demo_export_package_contract'
);
assert.ok(
  handoff.localSmokeSteps.some((step) => step.includes('scenario-matrix-smoke.test.cjs')),
  'export smoke step must reference collect-and-exit scenario matrix smoke'
);

console.log('local package inspection handoff smoke passed');
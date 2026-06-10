const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const read = (repoPath) => fs.readFileSync(path.join(repoRoot, repoPath), 'utf8');
const json = (repoPath) => JSON.parse(read(repoPath));
const exists = (repoPath) => fs.existsSync(path.join(repoRoot, repoPath));

const reportPath = '.omx/dogfood-validation/studio-ux-validation.md';
const statusPath = '.omx/dogfood-validation/studio-ux-validation.status.json';
const report = read(reportPath);
const normalized = report.replace(/\s+/g, ' ').trim();
const status = json(statusPath);

for (const repoPath of [
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  '.omx/dogfood-validation/export-release-readiness.md',
  '.omx/dogfood-validation/gameplay-runtime-stress.md',
  reportPath,
  statusPath,
]) {
  assert.ok(exists(repoPath), `expected dogfood artifact exists: ${repoPath}`);
}

for (const section of [
  'Metadata',
  'Purpose',
  'Merged prerequisite evidence',
  'Commands executed',
  'Studio UX evidence summary',
  'Validated task areas',
  'Gaps and conservative wording',
  'Verification commands for this PR',
  'Non-goals and guardrails',
]) {
  assert.match(report, new RegExp(`## ${section}`), `report includes ${section}`);
}

for (const required of [
  'dogfood-studio-ux-validation-v1',
  'collect-and-exit-local-rc-candidate',
  'local-read-only-and-review-gated-studio-ux-evidence',
  '#2334 MERGED',
  '#2335 MERGED',
  '#2336 MERGED',
  '#2337 MERGED',
  '#2339 MERGED',
  '.omx/dogfood-validation/claim-coverage-matrix.md',
  '.omx/dogfood-validation/demo-game-spec.md',
  '.omx/dogfood-validation/pipeline-dry-run.md',
  '.omx/dogfood-validation/export-release-readiness.md',
  '.omx/dogfood-validation/gameplay-runtime-stress.md',
  'authoring cockpit smoke test passed',
  'full-studio-integrated-demo-v1',
  'godot-plus-demo-studio-walkthrough-v790',
  'read-only',
  'draft-only',
  'review-gated',
  'No new Phoenix/LiveView implementation is introduced or claimed by B6',
  'Dashboard/live evidence export is not executed by this B6 PR',
]) {
  assert.ok(normalized.includes(required.replace(/\s+/g, ' ')), `report records ${required}`);
}

assert.equal(status.schemaVersion, 'dogfood-studio-ux-validation-status-v1');
assert.equal(status.blocker, 'B6');
assert.equal(status.status, 'ready_for_verifier');
assert.equal(status.evidenceClassification, 'local-read-only-and-review-gated-studio-ux-evidence');
assert.equal(status.protectedIssues['1'], 'OPEN');
assert.equal(status.protectedIssues['23'], 'OPEN');
assert.equal(status.forbiddenScopeIntroduced, false);
assert.equal(status.studioEvidence.browserTrustedWrites, false);
assert.equal(status.studioEvidence.commandBridge, false);
assert.equal(status.studioEvidence.reviewGatedHandoffOnly, true);
assert.equal(status.studioEvidence.integratedPanelCount, 13);
assert.equal(status.studioEvidence.panelChecks, 11);
for (const artifact of status.trackedArtifacts) assert.ok(exists(artifact), `tracked artifact exists: ${artifact}`);
for (const prereq of status.mergedPrerequisites) {
  assert.equal(prereq.state, 'MERGED', `${prereq.blocker} merged`);
  assert.ok(exists(prereq.artifact), `${prereq.blocker} artifact exists`);
}

const walkthroughDoc = read('docs/godot-plus-demo-studio-walkthrough-v1.md');
assert.match(walkthroughDoc, /read-only and draft-only/i);
assert.match(walkthroughDoc, /does not grant trusted browser writes/i);
assert.match(walkthroughDoc, /#1 and #23 remain open/i);

const reviewDoc = read('docs/studio-review-cockpit-v1.md');
assert.match(reviewDoc.replace(/\s+/g, ' '), /does not own trusted state/i);
assert.match(reviewDoc, /Not allowed:/);
assert.match(reviewDoc, /browser file writes/i);
assert.match(reviewDoc, /auto-rerun, auto-apply, auto-promote, or auto-merge/i);

const workspaceDoc = read('docs/studio-v3-project-workspace-cockpit.md');
assert.match(workspaceDoc.replace(/\s+/g, ' '), /local, static, and read-only/i);
assert.match(workspaceDoc.replace(/\s+/g, ' '), /JavaScript does not run, accept, apply, rollback, merge, or schedule/i);

const cockpit = require(path.join(repoRoot, 'examples/authoring-cockpit/cockpit.js'));
assert.equal(cockpit.resolveStudioCommand('apply-source').allowed, false);
assert.equal(cockpit.resolveStudioCommand('execute-plugin').allowed, false);
assert.equal(cockpit.resolveStudioCommand('publish').allowed, false);

for (const guardrail of [
  /#1 and #23 remain open/i,
  /Era Q M102-M106 remain deferred\/non-goal/i,
  /No product Studio feature/i,
  /No production-ready, store-ready, commercial release/i,
]) {
  assert.match(report, guardrail, `guardrail present: ${guardrail}`);
}

for (const forbiddenOverclaim of [
  /production-ready\s+(?:studio|editor|ux)/i,
  /store-ready\s+(?:studio|editor|ux)/i,
  /commercial release ready/i,
  /full Godot parity is verified|claims full Godot parity/i,
  /Godot replacement status is verified/i,
  /M102(?:–|-| to )M106\s+(?:active|implemented|complete|ready)/i,
  /trusted browser writes are allowed/i,
  /command bridge enabled/i,
  /auto-apply enabled/i,
]) {
  assert.doesNotMatch(report, forbiddenOverclaim, `forbidden overclaim absent: ${forbiddenOverclaim}`);
}

console.log('dogfood Studio UX validation smoke passed');

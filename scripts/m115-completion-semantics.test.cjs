#!/usr/bin/env node
/**
 * M115 Completion Semantics TDD Smoke (test-first, RED before impl, GREEN after)
 * Per ULW + Ouroforge AGENTS.md: asserts new semantics artifacts, #1/#23 open, no overclaim, generated-state (runs/ dashboard-data/) clean.
 */
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const repoRoot = path.resolve(__dirname, '..');

const completionDoc = path.join(repoRoot, 'docs', 'product-observed-completion.md');
const ledger = path.join(repoRoot, 'docs', 'm115-historical-classification-ledger.md');
const roadmap = path.join(repoRoot, 'docs', 'roadmap.md');
const contrib = path.join(repoRoot, 'CONTRIBUTING.md');

let failures = [];

function assertFileExists(p, name) { if (!fs.existsSync(p)) failures.push(`MISSING ${name}: ${p}`); }
function assertContains(p, substr, name) {
  if (!fs.existsSync(p)) return failures.push(`MISSING for contains ${name}`);
  if (!fs.readFileSync(p, 'utf8').includes(substr)) failures.push(`NO ${name} contains "${substr}"`);
}

console.log('M115 TDD Smoke: semantics/ledger/checklist + anchors + no overclaim + generated audit');

assertFileExists(completionDoc, 'product-observed-completion.md');
assertFileExists(ledger, 'm115-historical-classification-ledger.md');
assertContains(roadmap, 'contract-complete', 'roadmap contract-complete');
assertContains(roadmap, 'product-observed complete', 'roadmap product-observed');
assertContains(contrib, 'M115 Wording Guard', 'CONTRIBUTING M115 guard');

// Live GH anchors
function ghState(n) {
  try { return execSync(`gh issue view ${n} --json state,title --repo shaun0927/Ouroforge`, {encoding:'utf8'}); }
  catch(e) { return 'GH_ERR'; }
}
if (!ghState(1).includes('"state":"OPEN"')) failures.push('#1 not OPEN');
if (!ghState(23).includes('"state":"OPEN"')) failures.push('#23 not OPEN');

// Generated-state audit: only flag runs/ and dashboard-data/ (target/ is build, allowed)
const ignored = execSync('git status --short --ignored', { cwd: repoRoot, encoding: 'utf8' });
if (/^(\?\?|!!)\s+(runs\/|dashboard-data\/)/m.test(ignored)) {
  failures.push('Generated runs/ or dashboard-data/ pollution in trusted source');
}

const changed = execSync('git diff --name-only HEAD', { cwd: repoRoot, encoding: 'utf8' }).trim();
console.log('Changed files (audit):', changed.split('\n').slice(0,10).join(', ') || '(clean)');

if (failures.length) {
  console.error('RED M115 smoke:');
  failures.forEach(f => console.error('  - ' + f));
  process.exit(1);
}
console.log('GREEN M115 smoke');
process.exit(0);

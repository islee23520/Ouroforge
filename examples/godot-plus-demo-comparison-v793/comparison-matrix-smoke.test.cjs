const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '../..');
const docPath = path.join(root, 'docs/godot-plus-demo-capability-comparison-matrix-v1.md');
const doc = fs.readFileSync(docPath, 'utf8');

const requiredRows = [
  {
    area: 'Scene/node mental model',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json',
      'examples/playable-demo-v2/collect-and-exit/scaffold-audit.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs',
    ],
    gaps: [/native scene tree/i, /native editor maturity/i],
  },
  {
    area: 'Editor / Studio inspection',
    evidence: [
      'examples/authoring-cockpit/cockpit.test.cjs',
      'examples/evidence-dashboard/dashboard.test.cjs',
      'examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs',
    ],
    gaps: [/Godot editor maturity/i, /integrated authoring/i],
  },
  {
    area: 'Gameplay logic',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/gameplay-loop-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/behavior-smoke.test.cjs',
    ],
    gaps: [/scripting ecosystem/i, /gameplay framework parity/i],
  },
  {
    area: 'Export / package',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/export/export-profile.json',
      'examples/playable-demo-v2/collect-and-exit/export/package-metadata.json',
      'crates/ouroforge-core/tests/godot_plus_demo_export_package_contract.rs',
    ],
    gaps: [/platform export/i, /native\/mobile\/console/i],
  },
  {
    area: 'Plugin descriptors',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/plugin-usage-v792-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/plugin-usage-evidence.json',
      'examples/playable-demo-v2/collect-and-exit/plugins/registry/demo-plugin-registry-evidence.json',
    ],
    gaps: [/marketplace\/plugins/i, /executable plugin runtime/i],
  },
  {
    area: 'QA / playtest',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs',
    ],
    gaps: [/production QA scale/i, /production community/i],
  },
  {
    area: 'Evidence / journal',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/agentic-iteration/journal.json',
      'examples/playable-demo-v2/collect-and-exit/agentic-iteration-smoke.test.cjs',
    ],
    gaps: [/generated outputs remain untracked/i],
  },
  {
    area: 'Agentic mutation',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/agentic-iteration/draft-proposal.json',
      'examples/playable-demo-v2/collect-and-exit/agentic-iteration/review-decision.json',
      'examples/playable-demo-v2/collect-and-exit/agentic-iteration-smoke.test.cjs',
    ],
    gaps: [/direct Studio trusted-source writes/i, /auto-apply/i, /auto-merge/i],
  },
  {
    area: 'Asset pipeline',
    evidence: [
      'examples/playable-demo-v2/collect-and-exit/asset-manifest.json',
      'examples/playable-demo-v2/collect-and-exit/asset-provenance.json',
      'examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs',
      'examples/playable-demo-v2/collect-and-exit/asset-evidence-smoke.test.cjs',
    ],
    gaps: [/asset-store workflow/i, /production asset build pipeline/i],
  },
];

for (const { area, evidence, gaps } of requiredRows) {
  const row = doc
    .split('\n')
    .find((line) => line.startsWith(`| ${area} |`));
  assert.ok(row, `missing comparison row for ${area}`);
  for (const evidencePath of evidence) {
    assert.ok(row.includes(evidencePath), `${area} row missing evidence ${evidencePath}`);
    assert.ok(fs.existsSync(path.join(root, evidencePath)), `evidence path missing on disk: ${evidencePath}`);
  }
  for (const gapPattern of gaps) {
    assert.match(row, gapPattern, `${area} row missing gap ${gapPattern}`);
  }
}

const issueRequiredGapPhrases = [
  /Godot editor maturity/i,
  /platform export/i,
  /asset pipeline/i,
  /scripting ecosystem/i,
  /production community/i,
  /marketplace\/plugins/i,
  /native editor maturity/i,
];
for (const pattern of issueRequiredGapPhrases) assert.match(doc, pattern);

assert.match(doc, /## Wording audit/);
assert.match(doc, /Allowed framing:/);
assert.match(doc, /Disallowed closure claims/);
assert.match(doc, /## Governance/);
assert.match(doc, /Protected issues #1 and #23 must remain open/);
assert.match(doc, /Every positive claim.*concrete repository evidence/i);

const forbiddenUnqualifiedClaims = [
  /claims universal superiority/i,
  /claims full Godot parity achieved/i,
  /claims production-ready Godot replacement/i,
  /Ouroforge is better than Godot/i,
  /Ouroforge (is|serves as) (a )?Godot replacement/i,
  /the demo (is|serves as) (a )?production-ready engine/i,
];
for (const pattern of forbiddenUnqualifiedClaims) assert.doesNotMatch(doc, pattern);

console.log('comparison matrix v793 smoke passed');

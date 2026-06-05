const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..', '..');
const roadmapPath = path.join(repoRoot, 'docs', 'roadmap.md');
const roadmap = fs.readFileSync(roadmapPath, 'utf8');
const normalizedRoadmap = normalize(roadmap);

function normalize(value) {
  return value.replace(/\s+/g, ' ').trim();
}

function assertMentionsAll(text, phrases, label) {
  const normalizedText = normalize(text).toLowerCase();
  for (const phrase of phrases) {
    assert.ok(
      normalizedText.includes(phrase.toLowerCase()),
      `${label} names ${phrase}`,
    );
  }
}

function findFullStudioGovernanceSection() {
  const match = roadmap.match(
    /### Full Studio Editor v1 governance refresh([\s\S]*?)(?=\n### |\n## |$)/,
  );
  return match && match[0];
}

const governanceSection = findFullStudioGovernanceSection();
assert.ok(
  governanceSection,
  'docs/roadmap.md records a Full Studio Editor v1 governance refresh section',
);

assertMentionsAll(
  governanceSection,
  [
    'integrated Studio overview',
    'scene tree',
    'entity/component inspection',
    'visual scene canvas',
    'draft-only authoring',
    'Safe Source Apply handoff',
    'asset browser',
    'scenario/playtest evidence',
    'evidence timeline',
    'export/package inspection',
    'plugin/extension descriptor inspection',
    'workspace persistence',
    'command palette',
    'accessibility/performance/diagnostics coverage',
    'integrated demo',
    'Scenario Coverage v17',
  ],
  'Full Studio Editor v1 completed scope',
);

assertMentionsAll(
  governanceSection,
  [
    'full Godot parity',
    'native desktop editor',
    'advanced visual scripting',
    'full asset import pipeline',
    'executable editor plugins',
    'timeline/animation editor',
    'tilemap/terrain editor parity',
    'production collaboration features',
    'Godot-plus demonstration game',
  ],
  'Full Studio Editor v1 remaining gaps',
);

assertMentionsAll(
  governanceSection,
  [
    '#1 and #23 remain open',
    'does not directly write trusted source files',
    'execute shell commands',
    'publish',
    'deploy',
    'auto-apply',
    'auto-merge',
  ],
  'Full Studio Editor v1 guardrails and governance anchors',
);

assert.doesNotMatch(
  normalize(governanceSection),
  /production-ready editor|Godot replacement|full Godot editor parity achieved|secure sandbox guarantee|production collaboration ready/i,
  'Full Studio Editor v1 governance wording avoids affirmative overclaim phrases',
);

assert.match(
  normalizedRoadmap,
  /Full Studio Editor v1/i,
  'roadmap keeps a discoverable Full Studio Editor v1 reference',
);

console.log('full studio editor roadmap governance smoke passed');

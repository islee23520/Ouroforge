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

function findCompletedGodotPlusSection() {
  const entries = roadmap.split(/\n{2,}|(?=\n- )/);
  return entries.find((entry) => {
    const text = normalize(entry);
    return /(?:completed|complete|governance refresh|roadmap\/governance).{0,180}Godot-Plus Demo(?: Game)? v1|Godot-Plus Demo(?: Game)? v1.{0,180}(?:completed|complete|governance refresh|roadmap\/governance)/i.test(text);
  });
}

const completedEntry = findCompletedGodotPlusSection();
assert.ok(
  completedEntry,
  'docs/roadmap.md records a completed Godot-Plus Demo v1 roadmap/governance entry',
);

assertMentionsAll(
  `${completedEntry}\n${roadmap}`,
  [
    'playable demo game',
    'Studio inspect/draft workflow',
    'agentic iteration evidence',
    'Safe Source Apply chain',
    'QA swarm evidence',
    'export/package verification',
    'plugin descriptors',
    'Godot-plus comparison matrix',
    'documentation',
    'regression suite',
  ],
  'Godot-Plus Demo v1 completion scope',
);

assertMentionsAll(
  `${completedEntry}\n${roadmap}`,
  [
    'full Godot parity',
    'mature editor tooling',
    'native/mobile export',
    'large game production',
    'real marketplace',
    'executable plugin ecosystem',
    'production collaboration',
    'commercial release readiness',
    'production engine maturity',
  ],
  'Godot-Plus Demo v1 remaining gaps',
);

assert.doesNotMatch(
  normalize(completedEntry),
  /production-ready Godot replacement|full Godot parity achieved|commercial release ready|universal superiority/i,
  'Godot-Plus Demo v1 governance entry avoids affirmative overclaim phrases',
);

assert.match(
  normalizedRoadmap,
  /Godot-Plus Demo(?: Game)? v1/i,
  'roadmap keeps a discoverable Godot-Plus Demo v1 reference',
);

console.log('roadmap governance v21 smoke passed');

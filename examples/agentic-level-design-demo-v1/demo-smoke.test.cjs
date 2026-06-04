const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const cockpit = require('../authoring-cockpit/cockpit.js');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

function stageArtifactId(stage, artifact) {
  const idByKind = {
    level_intent: artifact.intentId,
    scene_generation_plan: artifact.planId,
    spatial_layout_constraints: artifact.solverId,
    tilemap_terrain_draft: artifact.draftId,
    entity_objective_encounter_placement: artifact.draftId,
    reachability_pathing: artifact.evidenceId,
    objective_completion_proof: artifact.proofId,
    difficulty_pacing_heuristic: artifact.evidenceId,
    level_visual_semantic_diff: artifact.diffId,
    agent_generated_level_draft: artifact.draftId,
    review_gated_level_apply: artifact.transactionId,
  };
  return idByKind[stage.kind];
}

const demo = readJson('examples/agentic-level-design-demo-v1/demo-chain.fixture.json');
assert.equal(demo.schemaVersion, 'agentic-level-design-demo-v1');
assert.equal(demo.status, 'ready');
assert.equal(demo.stages.length, 11);

const byStage = new Map();
for (const stage of demo.stages) {
  const artifact = readJson(stage.artifactRef);
  byStage.set(stage.stageId, { stage, artifact });
  assert.equal(artifact.schemaVersion, stage.expectedSchemaVersion, `${stage.stageId} schema`);
  assert.equal(artifact.status, stage.expectedStatus, `${stage.stageId} status`);
  assert.equal(stageArtifactId(stage, artifact), stage.expectedId, `${stage.stageId} id`);
}

assert.equal(byStage.get('generation_plan').artifact.intentId, byStage.get('intent').artifact.intentId);
assert.equal(byStage.get('layout_constraints').artifact.planId, byStage.get('generation_plan').artifact.planId);
assert.equal(byStage.get('placement_draft').artifact.solverId, byStage.get('layout_constraints').artifact.solverId);
assert.equal(byStage.get('reachability_pathing').artifact.placementDraftId, byStage.get('placement_draft').artifact.draftId);
assert.equal(byStage.get('agent_level_draft').artifact.draftId, byStage.get('review_gated_apply').artifact.draftId);
assert.equal(byStage.get('agent_level_draft').artifact.intentId, byStage.get('review_gated_apply').artifact.intentId);
assert.equal(byStage.get('agent_level_draft').artifact.planId, byStage.get('review_gated_apply').artifact.planId);

const requiredKinds = new Set([
  'level_intent',
  'scene_generation_plan',
  'spatial_layout_constraints',
  'tilemap_terrain_draft',
  'entity_objective_encounter_placement',
  'reachability_pathing',
  'objective_completion_proof',
  'difficulty_pacing_heuristic',
  'level_visual_semantic_diff',
  'agent_generated_level_draft',
  'review_gated_level_apply',
]);
assert.deepEqual(new Set(demo.stages.map((stage) => stage.kind)), requiredKinds);

const stageLabels = {
  intent: 'Intent and constraints',
  generation_plan: 'Generation plan',
  placement_draft: 'Tile and entity drafts',
  reachability_pathing: 'Reachability and pathing',
  objective_proof: 'Objective proof',
  difficulty_pacing: 'Difficulty and pacing heuristic',
  visual_semantic_diff: 'Visual and semantic diff',
  review_gated_apply: 'Review and apply status',
};
const run = {
  level_design_inspection: {
    present: true,
    schemaVersion: demo.studioInspection.schemaVersion,
    status: demo.studioInspection.status,
    boundary: demo.studioInspection.boundary,
    panels: demo.studioInspection.panelStageRefs.map((stageId) => {
      const { stage, artifact } = byStage.get(stageId);
      return {
        id: stageId,
        label: stageLabels[stageId],
        kind: stage.kind,
        status: artifact.status,
        items: [{ label: 'Artifact', value: stageArtifactId(stage, artifact) }],
        refs: [{ id: stage.stageId, path: stage.artifactRef }],
        commands: stageId === 'review_gated_apply' ? [{ command: demo.commands[0] }] : [],
      };
    }),
  },
};
const markup = cockpit.renderStudioLevelDesignInspectionSurface(run);
for (const label of Object.values(stageLabels)) {
  assert.match(markup, new RegExp(label));
}
assert.match(markup, /Copyable command text only/);
assert.match(markup, /no browser trusted writes/);
assert.match(markup, /no command bridge/);
assert.match(markup, /no autonomous full game generation/);
assert.doesNotMatch(markup, /<button|onclick|localStorage|fetch\(/i);

for (const root of demo.generatedStatePolicy.untrackedRoots) {
  assert.equal(fs.existsSync(path.join(repoRoot, root)), false, `${root} must remain generated/untracked`);
}
assert.ok(demo.guardrails.includes('no autonomous full game generation'));
assert.ok(demo.guardrails.includes('generated outputs remain untracked'));
assert.ok(demo.knownGaps.some((gap) => gap.includes('does not generate a new level autonomously')));

console.log('agentic level design demo smoke passed');

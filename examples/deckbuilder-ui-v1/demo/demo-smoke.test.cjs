'use strict';

// Deterministic Deckbuilder UI Demo v1 smoke (#1829).
// Runs in Node with network disabled; no live browser is required.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const demoDir = __dirname;
const repoRoot = path.resolve(demoDir, '../../..');
const runtimeDir = path.join(repoRoot, 'examples/game-runtime');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'deck-roguelike.js', 'deckbuilder-ui.js', 'runtime.js',
];

function readJson(relative) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relative), 'utf8'));
}

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in deckbuilder UI demo smoke')),
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

function norm(value) {
  return JSON.parse(JSON.stringify(value));
}

const manifest = readJson('examples/deckbuilder-ui-v1/demo/demo-manifest.json');
const scene = readJson('examples/deckbuilder-ui-v1/demo/deckbuilder-ui-demo-scene-v1.json');
const api = createRuntime();
api.loadScene(scene);
const state = norm(api.getWorldState());
const ui = state.deckbuilderUi;

assert.equal(manifest.schemaVersion, 'ouroforge.deckbuilder-ui-demo.v1');
assert.equal(ui.schemaVersion, manifest.expected.schemaVersion);
assert.deepEqual(ui.renderModel.hand.map((card) => card.cardId), manifest.expected.handCardIds);
assert.deepEqual(ui.renderModel.pipeline.map((slot) => slot.slotId), manifest.expected.pipelineSlotIds);
assert.deepEqual(ui.renderModel.shop.offers.map((offer) => offer.offerId), manifest.expected.shopOfferIds);
assert.deepEqual(ui.renderModel.runMap.nodes.map((node) => node.nodeId), manifest.expected.runMapNodeIds);
assert.equal(ui.renderModel.scoreDisplay.id, manifest.expected.scoreDisplayId);
assert.equal(ui.renderModel.scoreDisplay.formattedFinalScore, manifest.expected.formattedFinalScore);
assert.deepEqual(ui.renderModel.scoreDisplay.cascade.map((event) => event.phase), manifest.expected.scorePhases);
assert.equal(ui.readOnlyInspection.trustedEmitter, 'browser-runtime-deckbuilder-ui-probe');
assert.ok(ui.readOnlyInspection.disallowedActions.includes('trusted writes'));
assert.match(ui.trustedWriteBoundary, /existing review\/apply\/trust-gradient path/);
assert.equal(ui.generatedStatePolicy, 'Generated runs/artifacts remain untracked unless explicitly fixture-scoped.');

const selectedCard = norm(api.deckbuilderUiSelectCard(2));
assert.equal(selectedCard.interaction.selectedCardId, 'bash');
const queued = norm(api.deckbuilderUiQueueSelected('commit'));
assert.equal(queued.interaction.proposal.trustedWrite, false);
const offer = norm(api.deckbuilderUiSelectShopOffer('offer-strike-plus'));
assert.equal(offer.interaction.proposal.action, 'shop-offer-selection');
assert.equal(offer.interaction.proposal.trustedWrite, false);
const pathPlan = norm(api.deckbuilderUiPlanRunMapNode('shop'));
assert.equal(pathPlan.interaction.proposal.action, 'run-map-path-plan');
assert.equal(pathPlan.interaction.proposal.trustedWrite, false);
assert.equal(norm(api.getWorldState().deckRoguelike).status, 'playing', 'demo UI interactions do not mutate trusted deck run authority');

console.log('deckbuilder-ui-v1 demo smoke passed');

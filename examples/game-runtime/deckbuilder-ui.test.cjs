'use strict';

// Runtime contract test for Deckbuilder UI v1 (#1826).
// Validates card/hand/pipeline rendering, interaction state, and runtime probe
// observability without adding a trusted browser write path.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'deck-roguelike.js', 'deckbuilder-ui.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in deckbuilder UI test')),
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

function readScene(file) {
  return JSON.parse(fs.readFileSync(path.join(runtimeDir, file), 'utf8'));
}

function norm(value) {
  return JSON.parse(JSON.stringify(value));
}

const scene = readScene('deckbuilder-ui-scene-v1.json');

// --- Hand/pipeline render model is probe-observable --------------------------
{
  const api = createRuntime();
  api.loadScene(scene);
  const state = norm(api.getWorldState());
  const ui = state.deckbuilderUi;
  assert.equal(ui.schemaVersion, 'ouroforge.deckbuilder-ui-state.v1');
  assert.equal(ui.renderModel.schemaVersion, 'ouroforge.deckbuilder-ui-render.v1');
  assert.equal(ui.renderModel.hand.length, state.deckRoguelike.hand.length);
  assert.deepEqual(ui.renderModel.hand.map((card) => card.cardId), ['strike', 'strike', 'bash', 'defend', 'defend']);
  assert.deepEqual(ui.renderModel.pipeline.map((slot) => slot.slotId), ['intent', 'modifier', 'commit']);
  assert.ok(ui.handCards.every((card) => typeof card.effectText === 'string' && card.effectText.length > 0));
  assert.equal(ui.readOnlyInspection.trustedEmitter, 'browser-runtime-deckbuilder-ui-probe');
  assert.ok(ui.readOnlyInspection.disallowedActions.includes('trusted writes'));
  assert.match(ui.trustedWriteBoundary, /existing review\/apply\/trust-gradient path/);
}

// --- Selection and queueing are draft-only interaction state ------------------
{
  const api = createRuntime();
  api.loadScene(scene);
  const selected = norm(api.deckbuilderUiSelectCard(2));
  assert.equal(selected.interaction.lastAction.accepted, true);
  assert.equal(selected.interaction.selectedHandIndex, 2);
  assert.equal(selected.interaction.selectedCardId, 'bash');
  assert.equal(selected.renderModel.hand[2].selected, true);

  const queued = norm(api.deckbuilderUiQueueSelected('commit'));
  assert.equal(queued.interaction.lastAction.accepted, true);
  assert.equal(queued.pipelineSlots.find((slot) => slot.id === 'commit').queuedCard.cardId, 'bash');
  assert.equal(queued.interaction.proposal.trustedWrite, false);
  assert.equal(queued.interaction.proposal.route, 'existing review/apply/trust-gradient path');
  assert.equal(norm(api.getWorldState().deckRoguelike).hand[2], 'bash', 'queueing does not play or mutate the trusted deck run');
}

// --- Shop and run-map UI are renderable, navigable, and probe-observable ------
{
  const api = createRuntime();
  api.loadScene(scene);
  const state = norm(api.getWorldState());
  const ui = state.deckbuilderUi;
  assert.equal(ui.renderModel.shop.id, 'act1-shop');
  assert.deepEqual(ui.renderModel.shop.offers.map((offer) => offer.offerId), ['offer-strike-plus', 'offer-vigor-charm']);
  assert.equal(ui.renderModel.shop.offers[0].available, true);
  assert.equal(ui.renderModel.shop.offers[1].available, false);
  assert.equal(ui.renderModel.runMap.currentNodeId, 'start');
  assert.deepEqual(ui.renderModel.runMap.knownUpcomingNodeIds, ['shop', 'elite']);
  assert.deepEqual(ui.renderModel.runMap.nodes.map((node) => node.nodeId), ['start', 'shop', 'elite', 'boss']);

  const selectedOffer = norm(api.deckbuilderUiSelectShopOffer('offer-strike-plus'));
  assert.equal(selectedOffer.interaction.lastAction.accepted, true);
  assert.equal(selectedOffer.interaction.selectedShopOfferId, 'offer-strike-plus');
  assert.equal(selectedOffer.interaction.proposal.trustedWrite, false);
  assert.equal(selectedOffer.interaction.proposal.draftOnly, true);

  const plannedNode = norm(api.deckbuilderUiPlanRunMapNode('shop'));
  assert.equal(plannedNode.interaction.lastAction.accepted, true);
  assert.equal(plannedNode.interaction.navigation.currentNodeId, 'start');
  assert.equal(plannedNode.interaction.navigation.plannedNodeId, 'shop');
  assert.equal(plannedNode.interaction.proposal.trustedWrite, false);
  assert.equal(norm(api.getWorldState().deckRoguelike).status, 'playing', 'draft navigation does not advance trusted run state');
}

// --- Deck actions resync UI hand state through the existing runtime path ------
{
  const api = createRuntime();
  api.loadScene(scene);
  api.deckbuilderUiSelectCard(0);
  const deckAfterPlay = norm(api.deckRoguelikePlayCard(0));
  const uiAfterPlay = norm(api.getWorldState().deckbuilderUi);
  assert.equal(deckAfterPlay.lastAction.accepted, true);
  assert.equal(uiAfterPlay.handCards.length, deckAfterPlay.hand.length);
  assert.deepEqual(uiAfterPlay.handCards.map((card) => card.id), deckAfterPlay.hand);
  assert.equal(uiAfterPlay.interaction.selectedHandIndex, null, 'selection clears when the selected card leaves hand');
}

// --- Invalid interaction fails closed but stays observable --------------------
{
  const api = createRuntime();
  api.loadScene(scene);
  const invalidSelect = norm(api.deckbuilderUiSelectCard(99));
  assert.equal(invalidSelect.interaction.lastAction.accepted, false);
  assert.equal(invalidSelect.interaction.lastAction.reason, 'invalid-hand-index');
  const invalidQueue = norm(api.deckbuilderUiQueueSelected('missing-slot'));
  assert.equal(invalidQueue.interaction.lastAction.accepted, false);
  assert.equal(invalidQueue.interaction.lastAction.reason, 'invalid-slot');
  const invalidOffer = norm(api.deckbuilderUiSelectShopOffer('missing-offer'));
  assert.equal(invalidOffer.interaction.lastAction.accepted, false);
  assert.equal(invalidOffer.interaction.lastAction.reason, 'invalid-offer');
  const unavailableOffer = norm(api.deckbuilderUiSelectShopOffer('offer-vigor-charm'));
  assert.equal(unavailableOffer.interaction.lastAction.accepted, false);
  assert.equal(unavailableOffer.interaction.lastAction.reason, 'insufficient gold');
  const blockedNode = norm(api.deckbuilderUiPlanRunMapNode('elite'));
  assert.equal(blockedNode.interaction.lastAction.accepted, false);
  assert.equal(blockedNode.interaction.lastAction.reason, 'elite path locked in fixture');
}

console.log('deckbuilder-ui.test.cjs: all deckbuilder UI cases passed');

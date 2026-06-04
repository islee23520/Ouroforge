const assert = require('node:assert/strict');
const { normalizeAnimation, inferAnimationState, advanceAnimation, advanceAnimations, activeSpriteFrame } = require('./animation.js');

const animation = normalizeAnimation({
  mode: 'sprite_frame',
  frameDuration: 2,
  currentClip: 'idle',
  clips: [{
    id: 'idle',
    frameDuration: 2,
    loop: true,
    frames: [
      { color: '#5eead4', asset: 'player-idle-1' },
      { color: '#2dd4bf', asset: 'player-idle-2' },
      { color: '#0f766e', asset: 'player-idle-3' },
    ],
  }],
});

assert.deepEqual(animation.state, { currentClip: 'idle', elapsedFrames: 0, frameIndex: 0 });
assert.deepEqual(activeSpriteFrame(animation), { color: '#5eead4', asset: 'player-idle-1' });
advanceAnimation(animation, 1);
assert.deepEqual(animation.state, { currentClip: 'idle', elapsedFrames: 1, frameIndex: 0 });
advanceAnimation(animation, 1);
assert.deepEqual(animation.state, { currentClip: 'idle', elapsedFrames: 2, frameIndex: 1 });
assert.deepEqual(activeSpriteFrame(animation), { color: '#2dd4bf', asset: 'player-idle-2' });
advanceAnimation(animation, 4);
assert.deepEqual(animation.state, { currentClip: 'idle', elapsedFrames: 6, frameIndex: 0 });

const clamped = normalizeAnimation({
  mode: 'sprite_frame',
  frameDuration: 1,
  currentClip: 'once',
  clips: [{
    id: 'once',
    frameDuration: 1,
    loop: false,
    frames: [
      { color: '#111111' },
      { color: '#222222' },
    ],
  }],
});
advanceAnimation(clamped, 9);
assert.deepEqual(clamped.state, { currentClip: 'once', elapsedFrames: 9, frameIndex: 1 });

const legacy = normalizeAnimation({
  mode: 'sprite_frame',
  frameDuration: 1,
  loop: true,
  frames: [{ color: '#111111' }, { color: '#222222' }],
});
assert.equal(legacy.currentClip, 'default');
advanceAnimation(legacy, 1);
assert.deepEqual(activeSpriteFrame(legacy), { color: '#222222' });

const entity = { components: { animation: normalizeAnimation({ mode: 'sprite_frame', frameDuration: 1, frames: [{ color: '#111111' }, { color: '#222222' }] }) } };
advanceAnimations([entity], 1);
assert.equal(entity.components.animation.state.frameIndex, 1);
assert.equal(normalizeAnimation({ mode: 'transform', frames: [] }), null);


const stateful = normalizeAnimation({
  mode: 'sprite_frame',
  frameDuration: 1,
  currentClip: 'idle',
  stateClips: { idle: 'idle', run: 'run', jump: 'jump', collect: 'collect', hit: 'hit' },
  clips: [
    { id: 'idle', frameDuration: 3, frames: [{ color: '#111111' }] },
    { id: 'run', frameDuration: 1, frames: [{ color: '#222222' }, { color: '#333333' }] },
    { id: 'jump', frameDuration: 1, loop: false, frames: [{ color: '#444444' }] },
    { id: 'collect', frameDuration: 1, loop: false, frames: [{ color: '#555555' }] },
    { id: 'hit', frameDuration: 1, loop: false, frames: [{ color: '#666666' }] },
  ],
});
assert.equal(stateful.state.activeState, 'idle');
advanceAnimation(stateful, 1, { stateName: 'run' });
assert.deepEqual(stateful.state, { activeState: 'run', currentClip: 'run', elapsedFrames: 1, frameIndex: 1 });
advanceAnimation(stateful, 1, { stateName: 'run' });
assert.deepEqual(stateful.state, { activeState: 'run', currentClip: 'run', elapsedFrames: 2, frameIndex: 0 });
advanceAnimation(stateful, 1, { stateName: 'jump' });
assert.deepEqual(stateful.state, { activeState: 'jump', currentClip: 'jump', elapsedFrames: 1, frameIndex: 0 });

assert.equal(inferAnimationState({ components: { animation: stateful, velocity: { x: 2, y: 0 } } }), 'run');
assert.equal(inferAnimationState({ components: { animation: stateful, velocity: { x: 0, y: -3 } } }), 'jump');
assert.equal(inferAnimationState({ components: { animation: stateful, velocity: { x: 0, y: 0 }, status: { states: ['hit'] } } }), 'hit');
assert.equal(inferAnimationState({ components: { animation: stateful, velocity: { x: 0, y: 0 }, status: { states: ['collect'] } } }), 'collect');

const assert = require('node:assert/strict');
const { normalizeAnimation, advanceAnimation, advanceAnimations, activeSpriteFrame } = require('./animation.js');

const animation = normalizeAnimation({
  mode: 'sprite_frame',
  frameDuration: 2,
  loop: true,
  frames: [
    { color: '#5eead4' },
    { color: '#2dd4bf' },
    { color: '#0f766e' },
  ],
});

assert.deepEqual(animation.state, { elapsedFrames: 0, frameIndex: 0 });
assert.deepEqual(activeSpriteFrame(animation), { color: '#5eead4' });
advanceAnimation(animation, 1);
assert.deepEqual(animation.state, { elapsedFrames: 1, frameIndex: 0 });
advanceAnimation(animation, 1);
assert.deepEqual(animation.state, { elapsedFrames: 2, frameIndex: 1 });
assert.deepEqual(activeSpriteFrame(animation), { color: '#2dd4bf' });
advanceAnimation(animation, 4);
assert.deepEqual(animation.state, { elapsedFrames: 6, frameIndex: 0 });

const clamped = normalizeAnimation({
  mode: 'sprite_frame',
  frameDuration: 1,
  loop: false,
  frames: [
    { color: '#111111' },
    { color: '#222222' },
  ],
});
advanceAnimation(clamped, 9);
assert.deepEqual(clamped.state, { elapsedFrames: 9, frameIndex: 1 });

const entity = { components: { animation: normalizeAnimation({ mode: 'sprite_frame', frameDuration: 1, frames: [{ color: '#111111' }, { color: '#222222' }] }) } };
advanceAnimations([entity], 1);
assert.equal(entity.components.animation.state.frameIndex, 1);
assert.equal(normalizeAnimation({ mode: 'transform', frames: [] }), null);

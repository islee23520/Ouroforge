(function attachAnimation(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function normalizeAnimation(animation) {
    if (!animation || animation.mode !== 'sprite_frame') return null;
    const frames = Array.isArray(animation.frames) ? animation.frames : [];
    if (frames.length === 0) return null;
    const frameDuration = Number.isFinite(animation.frameDuration) && animation.frameDuration > 0
      ? Math.floor(animation.frameDuration)
      : 1;
    return {
      mode: 'sprite_frame',
      frames: frames.map((frame) => ({ color: frame.color })),
      frameDuration,
      loop: animation.loop !== false,
      state: {
        elapsedFrames: 0,
        frameIndex: 0,
      },
    };
  }

  function advanceAnimation(animation, frames = 1) {
    if (!animation || animation.mode !== 'sprite_frame' || animation.frames.length === 0) return null;
    const stepCount = Number.isFinite(frames) ? Math.max(0, Math.floor(frames)) : 0;
    for (let index = 0; index < stepCount; index += 1) {
      animation.state.elapsedFrames += 1;
      const nextFrameIndex = Math.floor(animation.state.elapsedFrames / animation.frameDuration);
      if (animation.loop) {
        animation.state.frameIndex = nextFrameIndex % animation.frames.length;
      } else {
        animation.state.frameIndex = Math.min(nextFrameIndex, animation.frames.length - 1);
      }
    }
    return animation;
  }

  function activeSpriteFrame(animation) {
    if (!animation || animation.mode !== 'sprite_frame' || animation.frames.length === 0) return null;
    return clone(animation.frames[animation.state.frameIndex] || animation.frames[0]);
  }

  function advanceAnimations(entities, frames = 1) {
    for (const entity of entities) {
      advanceAnimation(entity.components && entity.components.animation, frames);
    }
  }

  const api = Object.freeze({ normalizeAnimation, advanceAnimation, advanceAnimations, activeSpriteFrame });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAnimation = api;
})(typeof window !== 'undefined' ? window : globalThis);

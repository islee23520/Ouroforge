(function attachAnimation(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function normalizeFrame(frame = {}) {
    const normalized = { color: typeof frame.color === 'string' ? frame.color : '#f2f6f8' };
    if (typeof frame.asset === 'string' && frame.asset.length > 0) normalized.asset = frame.asset;
    return normalized;
  }

  function normalizeClip(clip = {}, fallback = {}) {
    const frames = Array.isArray(clip.frames) ? clip.frames.map(normalizeFrame) : [];
    if (frames.length === 0) return null;
    const frameDuration = Number.isFinite(clip.frameDuration) && clip.frameDuration > 0
      ? Math.floor(clip.frameDuration)
      : fallback.frameDuration;
    return {
      id: String(clip.id || fallback.id || 'default'),
      frameDuration: Math.max(1, frameDuration || 1),
      loop: clip.loop !== false,
      frames,
    };
  }

  function normalizeAnimation(animation) {
    if (!animation || animation.mode !== 'sprite_frame') return null;
    const defaultFrameDuration = Number.isFinite(animation.frameDuration) && animation.frameDuration > 0
      ? Math.floor(animation.frameDuration)
      : 1;
    const clips = [];
    if (Array.isArray(animation.clips) && animation.clips.length > 0) {
      for (const clip of animation.clips) {
        const normalized = normalizeClip(clip, { frameDuration: defaultFrameDuration });
        if (normalized) clips.push(normalized);
      }
    }
    if (clips.length === 0) {
      const fallbackClip = normalizeClip({
        id: 'default',
        frameDuration: defaultFrameDuration,
        loop: animation.loop,
        frames: Array.isArray(animation.frames) ? animation.frames : [],
      }, { frameDuration: defaultFrameDuration, id: 'default' });
      if (fallbackClip) clips.push(fallbackClip);
    }
    if (clips.length === 0) return null;
    const requestedClip = typeof animation.currentClip === 'string' ? animation.currentClip : null;
    const stateClip = animation.state && typeof animation.state.currentClip === 'string' ? animation.state.currentClip : null;
    const currentClip = clips.some((clip) => clip.id === (stateClip || requestedClip))
      ? (stateClip || requestedClip)
      : clips[0].id;
    const activeClip = clips.find((clip) => clip.id === currentClip) || clips[0];
    const requestedFrameIndex = animation.state && Number.isInteger(animation.state.frameIndex) ? animation.state.frameIndex : 0;
    return {
      mode: 'sprite_frame',
      frameDuration: defaultFrameDuration,
      loop: animation.loop !== false,
      frames: activeClip.frames.map(clone),
      clips,
      currentClip,
      state: {
        currentClip,
        elapsedFrames: animation.state && Number.isInteger(animation.state.elapsedFrames) && animation.state.elapsedFrames > 0
          ? animation.state.elapsedFrames
          : 0,
        frameIndex: Math.max(0, Math.min(requestedFrameIndex, activeClip.frames.length - 1)),
      },
    };
  }

  function activeClip(animation) {
    if (!animation || animation.mode !== 'sprite_frame') return null;
    const clips = Array.isArray(animation.clips) ? animation.clips : [];
    return clips.find((clip) => clip.id === animation.state.currentClip)
      || clips.find((clip) => clip.id === animation.currentClip)
      || clips[0]
      || null;
  }

  function advanceAnimation(animation, frames = 1) {
    const clip = activeClip(animation);
    if (!animation || !clip || clip.frames.length === 0) return null;
    const stepCount = Number.isFinite(frames) ? Math.max(0, Math.floor(frames)) : 0;
    for (let index = 0; index < stepCount; index += 1) {
      animation.state.elapsedFrames += 1;
      animation.state.currentClip = clip.id;
      animation.currentClip = clip.id;
      const nextFrameIndex = Math.floor(animation.state.elapsedFrames / clip.frameDuration);
      if (clip.loop) {
        animation.state.frameIndex = nextFrameIndex % clip.frames.length;
      } else {
        animation.state.frameIndex = Math.min(nextFrameIndex, clip.frames.length - 1);
      }
      animation.frames = clip.frames.map(clone);
    }
    return animation;
  }

  function activeSpriteFrame(animation) {
    const clip = activeClip(animation);
    if (!animation || !clip || clip.frames.length === 0) return null;
    return clone(clip.frames[animation.state.frameIndex] || clip.frames[0]);
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

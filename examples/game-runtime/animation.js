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

  function normalizeStateClips(animation = {}) {
    const source = animation.stateClips && typeof animation.stateClips === 'object' && !Array.isArray(animation.stateClips)
      ? animation.stateClips
      : {};
    const normalized = {};
    for (const [stateName, clipId] of Object.entries(source)) {
      if (typeof stateName === 'string' && stateName && typeof clipId === 'string' && clipId) normalized[stateName] = clipId;
    }
    return normalized;
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
    const stateClips = normalizeStateClips(animation);
    const activeState = animation.state && typeof animation.state.activeState === 'string' && stateClips[animation.state.activeState]
      ? animation.state.activeState
      : Object.keys(stateClips).find((stateName) => stateClips[stateName] === (stateClip || requestedClip)) || null;
    const stateMappedClip = activeState ? stateClips[activeState] : null;
    const selectedClip = stateMappedClip || stateClip || requestedClip;
    const currentClip = clips.some((clip) => clip.id === selectedClip)
      ? selectedClip
      : clips[0].id;
    const activeClip = clips.find((clip) => clip.id === currentClip) || clips[0];
    const requestedFrameIndex = animation.state && Number.isInteger(animation.state.frameIndex) ? animation.state.frameIndex : 0;
    const state = {
      currentClip,
      elapsedFrames: animation.state && Number.isInteger(animation.state.elapsedFrames) && animation.state.elapsedFrames > 0
        ? animation.state.elapsedFrames
        : 0,
      frameIndex: Math.max(0, Math.min(requestedFrameIndex, activeClip.frames.length - 1)),
    };
    const result = {
      mode: 'sprite_frame',
      frameDuration: defaultFrameDuration,
      loop: animation.loop !== false,
      frames: activeClip.frames.map(clone),
      clips,
      currentClip,
      state,
    };
    if (Object.keys(stateClips).length > 0) {
      result.stateClips = stateClips;
      result.state.activeState = activeState;
    }
    return result;
  }

  function activeClip(animation) {
    if (!animation || animation.mode !== 'sprite_frame') return null;
    const clips = Array.isArray(animation.clips) ? animation.clips : [];
    return clips.find((clip) => clip.id === animation.state.currentClip)
      || clips.find((clip) => clip.id === animation.currentClip)
      || clips[0]
      || null;
  }

  function inferAnimationState(entity) {
    const component = entity && entity.components ? entity.components : {};
    const animation = component.animation;
    const stateClips = animation && animation.stateClips && typeof animation.stateClips === 'object' ? animation.stateClips : {};
    if (Object.keys(stateClips).length === 0) return null;
    const statusStates = component.status && Array.isArray(component.status.states) ? component.status.states : [];
    if (statusStates.includes('hit') || statusStates.includes('damaged')) return stateClips.hit ? 'hit' : null;
    if (statusStates.includes('collect') || statusStates.includes('collected')) return stateClips.collect ? 'collect' : null;
    const velocity = component.velocity || {};
    if (Number.isFinite(velocity.y) && velocity.y < 0 && stateClips.jump) return 'jump';
    if (Number.isFinite(velocity.x) && velocity.x !== 0 && stateClips.run) return 'run';
    return stateClips.idle ? 'idle' : null;
  }

  function setAnimationState(animation, stateName) {
    if (!animation || !stateName || !animation.stateClips || !animation.stateClips[stateName]) return false;
    const clipId = animation.stateClips[stateName];
    const clip = Array.isArray(animation.clips) ? animation.clips.find((candidate) => candidate.id === clipId) : null;
    if (!clip || clip.frames.length === 0) return false;
    const priorClip = animation.state && animation.state.currentClip;
    if (!animation.state) animation.state = { currentClip: clipId, elapsedFrames: 0, frameIndex: 0, activeState: stateName };
    if (priorClip !== clipId) {
      animation.state.elapsedFrames = 0;
      animation.state.frameIndex = 0;
    }
    animation.state.activeState = stateName;
    animation.state.currentClip = clipId;
    animation.currentClip = clipId;
    animation.frames = clip.frames.map(clone);
    return priorClip !== clipId;
  }

  function advanceAnimation(animation, frames = 1, context = {}) {
    if (context && context.stateName) setAnimationState(animation, context.stateName);
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
      const stateName = inferAnimationState(entity);
      advanceAnimation(entity.components && entity.components.animation, frames, { stateName });
    }
  }

  const api = Object.freeze({ normalizeAnimation, inferAnimationState, setAnimationState, advanceAnimation, advanceAnimations, activeSpriteFrame });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAnimation = api;
})(typeof window !== 'undefined' ? window : globalThis);

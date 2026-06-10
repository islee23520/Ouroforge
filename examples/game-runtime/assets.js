(function attachAssets(root) {
  const IMAGE_EXTENSIONS = new Set(['png', 'jpg', 'jpeg', 'svg', 'webp']);
  const AUDIO_EXTENSIONS = new Set(['ogg', 'mp3', 'wav']);
  const ATLAS_EXTENSIONS = new Set(['json']);

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function compareCodeUnits(a, b) {
    return a < b ? -1 : a > b ? 1 : 0;
  }

  function isPlainObject(value) {
    return value && typeof value === 'object' && !Array.isArray(value);
  }

  function safeLocalAssetPath(value) {
    return typeof value === 'string'
      && value.startsWith('assets/')
      && !/^https?:\/\//i.test(value)
      && !value.startsWith('/')
      && /^[A-Za-z0-9/._-]+$/.test(value)
      && !value.split('/').some((part) => part === '..');
  }

  function extensionFor(path) {
    const match = String(path).toLowerCase().match(/\.([a-z0-9]+)$/);
    return match ? match[1] : '';
  }

  function kindSupportsPath(kind, path) {
    const ext = extensionFor(path);
    if (kind === 'image' || kind === 'sprite') return IMAGE_EXTENSIONS.has(ext);
    if (kind === 'sprite_atlas') return ATLAS_EXTENSIONS.has(ext);
    if (kind === 'audio') return AUDIO_EXTENSIONS.has(ext);
    return false;
  }


  function normalizeAtlas(atlas = null) {
    if (!isPlainObject(atlas)) return null;
    const frames = Array.isArray(atlas.frames) ? atlas.frames.map((frame) => {
      const rect = isPlainObject(frame.rect) ? frame.rect : frame;
      return {
        id: typeof frame.id === 'string' ? frame.id : (typeof frame.frameId === 'string' ? frame.frameId : ''),
        rect: {
          x: Number.isFinite(rect.x) ? rect.x : 0,
          y: Number.isFinite(rect.y) ? rect.y : 0,
          width: Number.isFinite(rect.width) && rect.width > 0 ? rect.width : 0,
          height: Number.isFinite(rect.height) && rect.height > 0 ? rect.height : 0,
        },
      };
    }).filter((frame) => frame.id && frame.rect.width > 0 && frame.rect.height > 0) : [];
    return {
      imageAssetId: typeof atlas.imageAssetId === 'string' ? atlas.imageAssetId : '',
      frames,
    };
  }

  function normalizeManifest(manifest = null) {
    if (manifest && manifest.byId instanceof Map && Array.isArray(manifest.entries)) return manifest;
    if (!isPlainObject(manifest)) {
      return { id: null, entries: [], byId: new Map(), errors: [], enabled: false };
    }
    const normalized = {
      schemaVersion: String(manifest.schemaVersion || '1'),
      id: typeof manifest.id === 'string' && manifest.id.length > 0 ? manifest.id : 'asset-manifest',
      entries: [],
      byId: new Map(),
      errors: [],
      enabled: true,
    };
    if (normalized.schemaVersion !== '1' && normalized.schemaVersion !== 'asset-manifest-v1') {
      normalized.errors.push('asset manifest schemaVersion must be 1 or asset-manifest-v1');
    }
    const ids = new Set();
    const paths = new Set();
    for (const asset of Array.isArray(manifest.assets) ? manifest.assets : []) {
      if (!isPlainObject(asset)) continue;
      const entry = {
        id: typeof asset.id === 'string' ? asset.id : '',
        kind: typeof asset.kind === 'string' ? asset.kind : (typeof asset.type === 'string' ? asset.type : 'image'),
        path: typeof asset.path === 'string' ? asset.path : '',
        metadata: isPlainObject(asset.metadata) ? clone(asset.metadata) : {},
        atlas: normalizeAtlas(asset.atlas),
      };
      if (!entry.id) normalized.errors.push('asset manifest entry id must not be empty');
      if (ids.has(entry.id)) normalized.errors.push(`duplicate asset manifest entry id: ${entry.id}`);
      if (paths.has(entry.path)) normalized.errors.push(`duplicate asset manifest entry path: ${entry.path}`);
      if (!safeLocalAssetPath(entry.path)) normalized.errors.push(`asset manifest entry ${entry.id} must use a safe local assets/ path`);
      if (!kindSupportsPath(entry.kind, entry.path)) normalized.errors.push(`asset manifest entry ${entry.id} has unsupported ${entry.kind} path`);
      if (entry.kind === 'sprite_atlas') {
        if (!entry.atlas) normalized.errors.push(`sprite atlas ${entry.id} requires atlas metadata`);
        if (entry.atlas && !entry.atlas.imageAssetId) normalized.errors.push(`sprite atlas ${entry.id} requires atlas.imageAssetId`);
        if (entry.atlas && entry.atlas.frames.length === 0) normalized.errors.push(`sprite atlas ${entry.id} requires atlas frames`);
      }
      ids.add(entry.id);
      paths.add(entry.path);
      normalized.entries.push(entry);
      normalized.byId.set(entry.id, entry);
    }
    if (normalized.entries.length === 0) normalized.errors.push('asset manifest assets must not be empty');
    normalized.entries.sort((left, right) => compareCodeUnits(left.id, right.id) || compareCodeUnits(left.kind, right.kind) || compareCodeUnits(left.path, right.path));
    return normalized;
  }

  function collectSpriteAssets(sceneOrEntities, manifest = null) {
    const scene = Array.isArray(sceneOrEntities) ? { entities: sceneOrEntities } : (sceneOrEntities || {});
    const normalizedManifest = normalizeManifest(manifest || scene.assetManifest);
    const refs = new Set();
    function addFrameAssets(frames) {
      for (const frame of Array.isArray(frames) ? frames : []) {
        const frameAsset = frame && frame.asset;
        if (typeof frameAsset === 'string' && frameAsset.length > 0) refs.add(frameAsset);
      }
    }
    for (const entity of Array.isArray(scene.entities) ? scene.entities : []) {
      const asset = entity && entity.sprite && entity.sprite.asset;
      if (typeof asset === 'string' && asset.length > 0) refs.add(asset);
      const vfx = entity && entity.components && entity.components.vfx;
      for (const emitter of (vfx && Array.isArray(vfx.emitters)) ? vfx.emitters : []) {
        if (typeof emitter.asset === 'string' && emitter.asset.length > 0) refs.add(emitter.asset);
      }
      const animation = entity && entity.components && entity.components.animation;
      if (animation) {
        addFrameAssets(animation.frames);
        for (const clip of Array.isArray(animation.clips) ? animation.clips : []) {
          addFrameAssets(clip && clip.frames);
        }
      }
    }
    for (const tilemap of Array.isArray(scene.tilemaps) ? scene.tilemaps : []) {
      for (const tile of Array.isArray(tilemap.tiles) ? tilemap.tiles : []) {
        const asset = tile && tile.asset;
        if (typeof asset === 'string' && asset.length > 0) refs.add(asset);
      }
    }
    const refsArray = Array.from(refs).sort();
    if (!normalizedManifest.enabled) return refsArray;
    return refsArray.flatMap((ref) => {
      const entry = normalizedManifest.byId.get(ref);
      if (!entry) return [];
      if (entry.kind === 'sprite_atlas') return entry.atlas && entry.atlas.imageAssetId ? [entry.atlas.imageAssetId] : [];
      return (entry.kind === 'image' || entry.kind === 'sprite') ? [entry.id] : [];
    });
  }

  function createAssetTracker(options = {}) {
    const ImageCtor = options.ImageCtor || root.Image;
    const onChange = typeof options.onChange === 'function' ? options.onChange : null;
    const onEvent = typeof options.onEvent === 'function' ? options.onEvent : null;
    const now = typeof options.now === 'function' ? options.now : () => Date.now();
    let manifest = normalizeManifest(options.manifest);
    let pathResolver = typeof options.resolvePath === 'function' ? options.resolvePath : (assetPath) => assetPath;
    const records = new Map();
    const unresolvedRefs = new Set();
    const invalidSpriteRefs = new Map();

    function configureManifest(nextManifest, nextOptions = {}) {
      manifest = normalizeManifest(nextManifest);
      if (Object.prototype.hasOwnProperty.call(nextOptions, 'resolvePath')) {
        pathResolver = typeof nextOptions.resolvePath === 'function' ? nextOptions.resolvePath : (assetPath) => assetPath;
      }
      records.clear();
      unresolvedRefs.clear();
      invalidSpriteRefs.clear();
      return manifestSummary();
    }

    function manifestSummary() {
      return {
        id: manifest.id,
        enabled: manifest.enabled,
        assetCount: manifest.entries.length,
        errors: manifest.errors.slice().sort(),
        assets: manifest.entries.map((entry) => {
          const summary = { id: entry.id, kind: entry.kind, path: entry.path };
          if (entry.atlas) summary.atlas = clone(entry.atlas);
          return summary;
        }),
      };
    }

    function emit(record) {
      if (onEvent) onEvent({ ...record, image: undefined });
      if (onChange) onChange(record);
    }

    function durationSince(startedAtUnixMs, endedAtUnixMs) {
      const duration = Math.max(0, Math.round(endedAtUnixMs - startedAtUnixMs));
      return duration > 0 ? duration : 1;
    }

    function entryFor(ref) {
      if (!manifest.enabled) {
        return safeLocalAssetPath(ref) ? { id: ref, kind: 'image', path: ref, metadata: {} } : null;
      }
      return manifest.byId.get(ref) || null;
    }

    function ensure(ref) {
      const entry = entryFor(ref);
      if (!entry) {
        unresolvedRefs.add(ref);
        return null;
      }
      if (entry.kind === 'audio') return null;
      if (records.has(entry.id)) return records.get(entry.id);
      const startedAtUnixMs = now();
      const resolvedPath = pathResolver(entry.path, entry);
      const record = {
        attemptId: `load-${entry.id}`,
        id: entry.id,
        path: entry.path,
        resolvedPath,
        kind: entry.kind === 'sprite' ? 'image' : entry.kind,
        status: 'attempted',
        startedAtUnixMs,
        endedAtUnixMs: null,
        loadDurationMs: null,
        failureReason: null,
        width: null,
        height: null,
        image: null,
      };
      records.set(entry.id, record);
      if (typeof ImageCtor !== 'function') {
        record.status = 'rejected';
        record.endedAtUnixMs = now();
        record.loadDurationMs = durationSince(record.startedAtUnixMs, record.endedAtUnixMs);
        record.failureReason = 'Image constructor unavailable';
        emit(record);
        return record;
      }
      const image = new ImageCtor();
      record.image = image;
      image.onload = () => {
        record.status = 'loaded';
        record.endedAtUnixMs = now();
        record.loadDurationMs = durationSince(record.startedAtUnixMs, record.endedAtUnixMs);
        record.width = image.naturalWidth || image.width || null;
        record.height = image.naturalHeight || image.height || null;
        emit(record);
      };
      image.onerror = () => {
        record.status = 'failed';
        record.endedAtUnixMs = now();
        record.loadDurationMs = durationSince(record.startedAtUnixMs, record.endedAtUnixMs);
        record.failureReason = 'Image load failed';
        emit(record);
      };
      image.src = resolvedPath;
      return record;
    }

    function load(sceneOrEntities, nextManifest, loadOptions = {}) {
      if (arguments.length > 1 || (sceneOrEntities && sceneOrEntities.assetManifest)) {
        configureManifest(nextManifest || sceneOrEntities.assetManifest, loadOptions);
      }
      return collectSpriteAssets(sceneOrEntities, manifest).map(ensure).filter(Boolean);
    }

    function metadata() {
      const loaded = Array.from(records.values())
        .sort((a, b) => compareCodeUnits(a.id, b.id) || compareCodeUnits(a.path, b.path))
        .map((record) => ({
          attemptId: record.attemptId,
          id: record.id,
          path: record.path,
          resolvedPath: record.resolvedPath,
          kind: record.kind,
          status: record.status,
          startedAtUnixMs: record.startedAtUnixMs,
          endedAtUnixMs: record.endedAtUnixMs,
          loadDurationMs: record.loadDurationMs,
          failureReason: record.failureReason,
          width: record.width,
          height: record.height,
        }));
      const unresolved = Array.from(unresolvedRefs).sort().map((ref) => ({
        attemptId: `reject-${String(ref).replace(/[^A-Za-z0-9_-]+/g, '-').replace(/^-+|-+$/g, '') || 'asset'}`,
        id: ref,
        path: null,
        kind: 'image',
        status: 'rejected',
        startedAtUnixMs: now(),
        endedAtUnixMs: now(),
        loadDurationMs: 1,
        failureReason: 'Asset reference unresolved',
        width: null,
        height: null,
      }));
      const invalid = Array.from(invalidSpriteRefs.values()).sort((a, b) => compareCodeUnits(a.id, b.id));
      return loaded.concat(unresolved, invalid);
    }

    function rejectSpriteRef(ref, frameId, reason) {
      const id = [ref || 'asset', frameId || 'frame'].join(':');
      invalidSpriteRefs.set(id, {
        attemptId: `reject-${String(id).replace(/[^A-Za-z0-9_-]+/g, '-').replace(/^-+|-+$/g, '') || 'sprite-frame'}`,
        id,
        path: null,
        kind: 'image',
        status: 'rejected',
        startedAtUnixMs: now(),
        endedAtUnixMs: now(),
        loadDurationMs: 1,
        failureReason: reason,
        width: null,
        height: null,
      });
    }

    function imageFor(ref) {
      const entry = entryFor(ref);
      if (!entry) {
        unresolvedRefs.add(ref);
        return null;
      }
      if (entry.kind === 'sprite_atlas') {
        return entry.atlas ? imageFor(entry.atlas.imageAssetId) : null;
      }
      const record = records.get(entry.id);
      return record && record.status === 'loaded' ? record.image : null;
    }

    function spriteFor(ref, frameId = null) {
      const entry = entryFor(ref);
      if (!entry) {
        unresolvedRefs.add(ref);
        return null;
      }
      if (entry.kind !== 'sprite_atlas') return { image: imageFor(ref), frame: null, assetId: entry.id, imageAssetId: entry.id };
      if (!entry.atlas) {
        rejectSpriteRef(ref, frameId, 'Sprite atlas metadata missing');
        return null;
      }
      const frame = entry.atlas.frames.find((candidate) => candidate.id === frameId) || null;
      if (!frame) {
        rejectSpriteRef(ref, frameId, `Sprite atlas frame unresolved: ${frameId || 'missing frameId'}`);
        return null;
      }
      return { image: imageFor(entry.atlas.imageAssetId), frame: clone(frame.rect), assetId: entry.id, imageAssetId: entry.atlas.imageAssetId, frameId: frame.id };
    }

    return Object.freeze({ collectSpriteAssets, configureManifest, manifestSummary, load, metadata, imageFor, spriteFor });
  }

  const api = Object.freeze({ collectSpriteAssets, createAssetTracker, normalizeManifest });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAssets = api;
})(typeof window !== 'undefined' ? window : globalThis);

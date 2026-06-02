(function attachAssets(root) {
  const IMAGE_EXTENSIONS = new Set(['png', 'jpg', 'jpeg', 'svg', 'webp']);
  const AUDIO_EXTENSIONS = new Set(['ogg', 'mp3', 'wav']);

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
    if (kind === 'audio') return AUDIO_EXTENSIONS.has(ext);
    return false;
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
    if (normalized.schemaVersion !== '1') {
      normalized.errors.push('asset manifest schemaVersion must be 1');
    }
    const ids = new Set();
    const paths = new Set();
    for (const asset of Array.isArray(manifest.assets) ? manifest.assets : []) {
      if (!isPlainObject(asset)) continue;
      const entry = {
        id: typeof asset.id === 'string' ? asset.id : '',
        kind: typeof asset.kind === 'string' ? asset.kind : 'image',
        path: typeof asset.path === 'string' ? asset.path : '',
        metadata: isPlainObject(asset.metadata) ? clone(asset.metadata) : {},
      };
      if (!entry.id) normalized.errors.push('asset manifest entry id must not be empty');
      if (ids.has(entry.id)) normalized.errors.push(`duplicate asset manifest entry id: ${entry.id}`);
      if (paths.has(entry.path)) normalized.errors.push(`duplicate asset manifest entry path: ${entry.path}`);
      if (!safeLocalAssetPath(entry.path)) normalized.errors.push(`asset manifest entry ${entry.id} must use a safe local assets/ path`);
      if (!kindSupportsPath(entry.kind, entry.path)) normalized.errors.push(`asset manifest entry ${entry.id} has unsupported ${entry.kind} path`);
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
    return refsArray
      .map((ref) => normalizedManifest.byId.get(ref))
      .filter((entry) => entry && (entry.kind === 'image' || entry.kind === 'sprite'))
      .map((entry) => entry.id);
  }

  function createAssetTracker(options = {}) {
    const ImageCtor = options.ImageCtor || root.Image;
    const onChange = typeof options.onChange === 'function' ? options.onChange : null;
    let manifest = normalizeManifest(options.manifest);
    const records = new Map();
    const unresolvedRefs = new Set();

    function configureManifest(nextManifest) {
      manifest = normalizeManifest(nextManifest);
      records.clear();
      unresolvedRefs.clear();
      return manifestSummary();
    }

    function manifestSummary() {
      return {
        id: manifest.id,
        enabled: manifest.enabled,
        assetCount: manifest.entries.length,
        errors: manifest.errors.slice().sort(),
        assets: manifest.entries.map((entry) => ({ id: entry.id, kind: entry.kind, path: entry.path })),
      };
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
      const record = { id: entry.id, path: entry.path, kind: entry.kind === 'sprite' ? 'image' : entry.kind, status: 'pending', width: null, height: null, image: null };
      records.set(entry.id, record);
      if (typeof ImageCtor !== 'function') {
        record.status = 'unavailable';
        return record;
      }
      const image = new ImageCtor();
      record.image = image;
      image.onload = () => {
        record.status = 'loaded';
        record.width = image.naturalWidth || image.width || null;
        record.height = image.naturalHeight || image.height || null;
        if (onChange) onChange(record);
      };
      image.onerror = () => {
        record.status = 'failed';
        if (onChange) onChange(record);
      };
      image.src = entry.path;
      return record;
    }

    function load(sceneOrEntities, nextManifest) {
      if (arguments.length > 1 || (sceneOrEntities && sceneOrEntities.assetManifest)) {
        configureManifest(nextManifest || sceneOrEntities.assetManifest);
      }
      return collectSpriteAssets(sceneOrEntities, manifest).map(ensure).filter(Boolean);
    }

    function metadata() {
      const loaded = Array.from(records.values())
        .sort((a, b) => compareCodeUnits(a.id, b.id) || compareCodeUnits(a.path, b.path))
        .map((record) => ({
          id: record.id,
          path: record.path,
          kind: record.kind,
          status: record.status,
          width: record.width,
          height: record.height,
        }));
      const unresolved = Array.from(unresolvedRefs).sort().map((ref) => ({ id: ref, path: null, kind: 'unknown', status: 'unresolved', width: null, height: null }));
      return loaded.concat(unresolved);
    }

    function imageFor(ref) {
      const entry = entryFor(ref);
      if (!entry) {
        unresolvedRefs.add(ref);
        return null;
      }
      const record = records.get(entry.id);
      return record && record.status === 'loaded' ? record.image : null;
    }

    return Object.freeze({ collectSpriteAssets, configureManifest, manifestSummary, load, metadata, imageFor });
  }

  const api = Object.freeze({ collectSpriteAssets, createAssetTracker, normalizeManifest });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAssets = api;
})(typeof window !== 'undefined' ? window : globalThis);

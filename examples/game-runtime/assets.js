(function attachAssets(root) {
  function collectSpriteAssets(entities) {
    return Array.from(new Set(
      entities
        .map((entity) => entity.sprite && entity.sprite.asset)
        .filter((asset) => typeof asset === 'string' && asset.length > 0),
    )).sort();
  }

  function createAssetTracker(options = {}) {
    const ImageCtor = options.ImageCtor || root.Image;
    const records = new Map();

    function ensure(path) {
      if (records.has(path)) return records.get(path);
      const record = { path, kind: 'image', status: 'pending', width: null, height: null, image: null };
      records.set(path, record);
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
      };
      image.onerror = () => {
        record.status = 'failed';
      };
      image.src = path;
      return record;
    }

    function load(entities) {
      return collectSpriteAssets(entities).map(ensure);
    }

    function metadata() {
      return Array.from(records.values())
        .sort((a, b) => a.path.localeCompare(b.path))
        .map((record) => ({
          path: record.path,
          kind: record.kind,
          status: record.status,
          width: record.width,
          height: record.height,
        }));
    }

    function imageFor(path) {
      const record = records.get(path);
      return record && record.status === 'loaded' ? record.image : null;
    }

    return Object.freeze({ collectSpriteAssets, load, metadata, imageFor });
  }

  const api = Object.freeze({ collectSpriteAssets, createAssetTracker });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAssets = api;
})(typeof window !== 'undefined' ? window : globalThis);

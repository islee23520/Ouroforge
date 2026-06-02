(function attachRenderer(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function point(value = {}, fallback = { x: 0, y: 0 }) {
    return {
      x: Number.isFinite(value.x) ? value.x : fallback.x,
      y: Number.isFinite(value.y) ? value.y : fallback.y,
    };
  }

  function size(value = {}, fallback = { width: 320, height: 180 }) {
    return {
      width: Number.isFinite(value.width) && value.width > 0 ? value.width : fallback.width,
      height: Number.isFinite(value.height) && value.height > 0 ? value.height : fallback.height,
    };
  }

  function normalizeLayer(layer = {}, index = 0) {
    return {
      id: String(layer.id || `layer-${index}`),
      order: Number.isFinite(layer.order) ? layer.order : 0,
      visible: layer.visible !== false,
    };
  }

  function normalizeRenderer(renderer = {}, sceneBounds = { width: 320, height: 180 }) {
    const layers = Array.isArray(renderer.layers) && renderer.layers.length > 0
      ? renderer.layers.map(normalizeLayer)
      : [{ id: 'default', order: 0, visible: true }];
    return {
      version: String(renderer.version || '1'),
      camera: point(renderer.camera),
      viewport: size(renderer.viewport, sceneBounds),
      background: typeof renderer.background === 'string' ? renderer.background : '#172532',
      layers,
      debug: {
        showBounds: Boolean(renderer.debug && renderer.debug.showBounds),
        showCamera: Boolean(renderer.debug && renderer.debug.showCamera),
        showEntityIds: Boolean(renderer.debug && renderer.debug.showEntityIds),
      },
    };
  }

  function layerOrderMap(renderer) {
    const entries = renderer && Array.isArray(renderer.layers) ? renderer.layers : [];
    return new Map(entries.map((layer) => [layer.id, layer.order]));
  }

  function layerVisibilityMap(renderer) {
    const entries = renderer && Array.isArray(renderer.layers) ? renderer.layers : [];
    return new Map(entries.map((layer) => [layer.id, layer.visible !== false]));
  }

  function renderOrder(entities = [], renderer = normalizeRenderer()) {
    const orders = layerOrderMap(renderer);
    const visibility = layerVisibilityMap(renderer);
    return entities
      .filter((entity) => {
        const sprite = entity.sprite || {};
        const layer = sprite.layer || 'default';
        return sprite.visible !== false && visibility.get(layer) !== false;
      })
      .map((entity) => {
        const sprite = entity.sprite || {};
        const layer = sprite.layer || 'default';
        return {
          entity,
          entityId: String(entity.id || ''),
          layer,
          layerOrder: orders.get(layer) || 0,
          spriteOrder: Number.isFinite(sprite.order) ? sprite.order : 0,
        };
      })
      .sort((left, right) => (
        left.layerOrder - right.layerOrder
        || left.spriteOrder - right.spriteOrder
        || left.entityId.localeCompare(right.entityId)
      ));
  }

  function drawRuntime({ canvas, context, world, renderer, assets, animation }) {
    if (!canvas || !context || !world) return [];
    const activeRenderer = normalizeRenderer(renderer, world.bounds || { width: canvas.width, height: canvas.height });
    const ordered = renderOrder(world.entities || [], activeRenderer);
    context.clearRect(0, 0, canvas.width, canvas.height);
    context.fillStyle = activeRenderer.background;
    context.fillRect(0, 0, canvas.width, canvas.height);
    for (const item of ordered) {
      const entity = item.entity;
      const components = entity.components || {};
      const transform = point(components.transform);
      const entitySize = size(components.size, { width: 16, height: 16 });
      const x = transform.x - activeRenderer.camera.x;
      const y = transform.y - activeRenderer.camera.y;
      const activeFrame = animation && typeof animation.activeSpriteFrame === 'function'
        ? animation.activeSpriteFrame(components.animation)
        : null;
      const image = entity.sprite && entity.sprite.asset && assets && typeof assets.imageFor === 'function'
        ? assets.imageFor(entity.sprite.asset)
        : null;
      if (image) {
        context.drawImage(image, x, y, entitySize.width, entitySize.height);
      } else {
        context.fillStyle = (activeFrame && activeFrame.color) || (entity.sprite && entity.sprite.color) || '#f2f6f8';
        context.fillRect(x, y, entitySize.width, entitySize.height);
      }
      if (activeRenderer.debug.showEntityIds) {
        context.fillStyle = '#f2f6f8';
        context.font = '10px ui-monospace, monospace';
        context.fillText(entity.id, x, y - 2);
      }
    }
    context.fillStyle = '#f2f6f8';
    context.font = '10px ui-monospace, monospace';
    context.fillText(`scene=${world.sceneId} tick=${world.tick}`, 8, 14);
    return ordered.map(({ entityId, layer, layerOrder, spriteOrder }) => ({ entityId, layer, layerOrder, spriteOrder }));
  }

  const api = Object.freeze({ normalizeRenderer, renderOrder, drawRuntime, clone });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeRenderer = api;
})(typeof window !== 'undefined' ? window : globalThis);

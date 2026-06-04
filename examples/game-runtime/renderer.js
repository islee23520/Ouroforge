(function attachRenderer(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function compareCodeUnits(a, b) {
    return a < b ? -1 : a > b ? 1 : 0;
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
        || compareCodeUnits(left.entityId, right.entityId)
      ));
  }

  function primitiveCategory(entity = {}) {
    const components = entity.components || {};
    if (components.uiText || components.hudValue) return 'text';
    if (entity.sprite && typeof entity.sprite.asset === 'string') return 'sprite';
    return 'rect';
  }

  function elementMatchesFilter(element, filter = {}) {
    if (!filter || typeof filter !== 'object') return true;
    if (filter.sceneId && element.sceneId !== filter.sceneId) return false;
    if (filter.layer && element.layer !== filter.layer) return false;
    if (filter.entityId && element.entityId !== filter.entityId) return false;
    if (filter.renderableId && element.renderableId !== filter.renderableId) return false;
    if (filter.primitiveCategory && element.primitiveCategory !== filter.primitiveCategory) return false;
    return true;
  }

  function renderBreakdown({ world = {}, renderer = normalizeRenderer(), frameId = `tick-${world.tick ?? 0}`, filter = null } = {}) {
    const activeRenderer = normalizeRenderer(renderer, world.bounds || { width: 320, height: 180 });
    const sceneId = String(world.sceneId || 'unknown-scene');
    const entities = Array.isArray(world.entities) ? world.entities : [];
    const visibility = layerVisibilityMap(activeRenderer);
    const malformedDiagnostics = [];
    const elements = renderOrder(entities, activeRenderer).map((item, drawOrder) => {
      const entity = item.entity || {};
      const components = entity.components || {};
      if (!entity.id) malformedDiagnostics.push({ renderableId: 'entity:missing-id', entityId: '', reason: 'malformed', layer: item.layer, detail: 'entity id missing or empty' });
      if (!components.transform || !components.size) malformedDiagnostics.push({ renderableId: `entity:${item.entityId}`, entityId: item.entityId, reason: 'fallback', layer: item.layer, detail: 'missing transform or size used default fallback' });
      return {
        sceneId,
        renderableId: `entity:${item.entityId}`,
        entityId: item.entityId,
        layer: item.layer,
        layerOrder: item.layerOrder,
        spriteOrder: item.spriteOrder,
        drawOrder,
        camera: clone(activeRenderer.camera),
        transform: point(components.transform),
        size: size(components.size, { width: 16, height: 16 }),
        primitiveCategory: primitiveCategory(entity),
        assetRef: entity.sprite && typeof entity.sprite.asset === 'string' ? entity.sprite.asset : null,
        debugLabel: `${item.entityId} on layer ${item.layer}`,
        visible: true,
      };
    });
    const absenceDiagnostics = entities.flatMap((entity) => {
      const entityId = String(entity && entity.id || '');
      const sprite = entity && entity.sprite || {};
      const layer = sprite.layer || 'default';
      if (sprite.visible === false) return [{ renderableId: `entity:${entityId}`, entityId, reason: 'hidden', layer, detail: 'sprite.visible=false' }];
      if (visibility.get(layer) === false) return [{ renderableId: `entity:${entityId}`, entityId, reason: 'layer_hidden', layer, detail: 'renderer layer visible=false' }];
      if (visibility.has(layer) === false && activeRenderer.layers.length > 0) return [{ renderableId: `entity:${entityId}`, entityId, reason: 'fallback', layer, detail: 'layer missing from renderer contract; default order applied' }];
      return [];
    }).concat(malformedDiagnostics);
    return {
      schemaVersion: 'ouroforge.scene-render-breakdown.v1',
      frameId: String(frameId),
      sceneId,
      camera: clone(activeRenderer.camera),
      viewport: clone(activeRenderer.viewport),
      elements: filter ? elements.filter((element) => elementMatchesFilter(element, filter)) : elements,
      absenceDiagnostics: filter ? absenceDiagnostics.filter((diag) => (!filter.entityId || diag.entityId === filter.entityId) && (!filter.layer || diag.layer === filter.layer)) : absenceDiagnostics,
      readOnlyInspection: { trustedEmitter: 'browser-runtime-evidence-helper', browserStudioMode: 'read-only evidence inspection', disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    };
  }

  function compareBreakdowns(before = {}, after = {}) {
    const key = (element) => `${element.sceneId || ''}/${element.renderableId || element.entityId || ''}`;
    const beforeElements = new Map((before.elements || []).map((element) => [key(element), element]));
    const afterElements = new Map((after.elements || []).map((element) => [key(element), element]));
    const keys = Array.from(new Set([...beforeElements.keys(), ...afterElements.keys()])).sort(compareCodeUnits);
    return {
      schemaVersion: 'ouroforge.scene-render-breakdown-diff.v1',
      beforeFrameId: String(before.frameId || ''),
      afterFrameId: String(after.frameId || ''),
      changes: keys.flatMap((id) => {
        const left = beforeElements.get(id);
        const right = afterElements.get(id);
        if (!left) return [{ kind: 'added', renderableId: id, afterDrawOrder: right.drawOrder }];
        if (!right) return [{ kind: 'removed', renderableId: id, beforeDrawOrder: left.drawOrder }];
        return ['layer', 'layerOrder', 'spriteOrder', 'drawOrder', 'primitiveCategory'].flatMap((field) => left[field] !== right[field] ? [{ kind: 'changed', renderableId: id, field, before: left[field], after: right[field] }] : []);
      }),
    };
  }

  function debugState(renderer = normalizeRenderer(), entities = []) {
    const activeRenderer = normalizeRenderer(renderer);
    return {
      version: activeRenderer.version,
      camera: clone(activeRenderer.camera),
      viewport: clone(activeRenderer.viewport),
      background: activeRenderer.background,
      layers: activeRenderer.layers.map((layer) => clone(layer)),
      debug: clone(activeRenderer.debug),
      renderedEntities: renderOrder(entities, activeRenderer)
        .map(({ entityId, layer, layerOrder, spriteOrder }) => ({ entityId, layer, layerOrder, spriteOrder })),
    };
  }

  function drawRuntime({ canvas, context, world, renderer, assets, animation, tilemap }) {
    if (!canvas || !context || !world) return [];
    const activeRenderer = normalizeRenderer(renderer, world.bounds || { width: canvas.width, height: canvas.height });
    const ordered = renderOrder(world.entities || [], activeRenderer);
    context.clearRect(0, 0, canvas.width, canvas.height);
    context.fillStyle = activeRenderer.background;
    context.fillRect(0, 0, canvas.width, canvas.height);
    if (tilemap && typeof tilemap.drawTilemaps === 'function') {
      tilemap.drawTilemaps({ context, renderer: activeRenderer, tilemaps: world.tilemaps || [], assets });
    }
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
      const frameAsset = activeFrame && typeof activeFrame.asset === 'string' ? activeFrame.asset : null;
      const spriteAsset = entity.sprite && typeof entity.sprite.asset === 'string' ? entity.sprite.asset : null;
      const preferFrameColor = activeFrame && typeof activeFrame.color === 'string' && !frameAsset;
      const image = !preferFrameColor && assets && typeof assets.imageFor === 'function'
        ? assets.imageFor(frameAsset || spriteAsset)
        : null;
      if (image) {
        context.drawImage(image, x, y, entitySize.width, entitySize.height);
      } else {
        context.fillStyle = (activeFrame && activeFrame.color) || (entity.sprite && entity.sprite.color) || '#f2f6f8';
        context.fillRect(x, y, entitySize.width, entitySize.height);
      }
      if (components.uiText && typeof components.uiText.text === 'string') {
        context.fillStyle = (entity.sprite && entity.sprite.color) || '#f2f6f8';
        context.font = '10px ui-monospace, monospace';
        context.fillText(components.uiText.text, x, y + Math.max(10, entitySize.height));
      }
      if (components.hudValue && typeof components.hudValue === 'object') {
        const label = typeof components.hudValue.label === 'string' ? components.hudValue.label : '';
        const value = typeof components.hudValue.value === 'string' ? components.hudValue.value : '';
        const hudText = label ? `${label}: ${value}` : value;
        if (hudText) {
          context.fillStyle = (entity.sprite && entity.sprite.color) || '#f2f6f8';
          context.font = '10px ui-monospace, monospace';
          const lineOffset = components.uiText && typeof components.uiText.text === 'string' ? 22 : Math.max(10, entitySize.height);
          context.fillText(hudText, x, y + lineOffset);
        }
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

  const api = Object.freeze({ normalizeRenderer, renderOrder, renderBreakdown, compareBreakdowns, debugState, drawRuntime, clone });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeRenderer = api;
})(typeof window !== 'undefined' ? window : globalThis);

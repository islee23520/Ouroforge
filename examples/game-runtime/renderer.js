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
      parallaxFactor: Number.isFinite(layer.parallaxFactor) && layer.parallaxFactor > 0 ? layer.parallaxFactor : 100,
      cameraParticipation: layer.cameraParticipation !== false,
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

  function layerById(renderer, layerId) {
    const entries = renderer && Array.isArray(renderer.layers) ? renderer.layers : [];
    return entries.find((layer) => layer.id === layerId) || null;
  }

  function cameraOffsetForLayer(renderer = normalizeRenderer(), layerId = 'default') {
    const camera = renderer && renderer.camera ? point(renderer.camera) : { x: 0, y: 0 };
    const layer = layerById(renderer, layerId);
    if (layer && layer.cameraParticipation === false) return { x: 0, y: 0 };
    const factor = layer && Number.isFinite(layer.parallaxFactor) ? layer.parallaxFactor : 100;
    return {
      x: (camera.x * factor) / 100,
      y: (camera.y * factor) / 100,
    };
  }

  function worldToScreen(pointValue = {}, renderer = normalizeRenderer(), layerId = 'default') {
    const worldPoint = point(pointValue);
    const offset = cameraOffsetForLayer(renderer, layerId);
    return {
      x: worldPoint.x - offset.x,
      y: worldPoint.y - offset.y,
      layer: String(layerId || 'default'),
      cameraOffset: offset,
    };
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

  function renderQueue({ world = {}, renderer = normalizeRenderer(), tilemap = null, frameId = `tick-${world.tick ?? 0}` } = {}) {
    const activeRenderer = normalizeRenderer(renderer, world.bounds || { width: 320, height: 180 });
    const orders = layerOrderMap(activeRenderer);
    const visibility = layerVisibilityMap(activeRenderer);
    const tilemapLayers = tilemap && typeof tilemap.orderedLayers === 'function'
      ? tilemap.orderedLayers(world.tilemaps || [])
      : [];
    const layers = activeRenderer.layers.map((layer) => ({ id: layer.id, order: layer.order, visible: layer.visible !== false, kind: 'scene' }));
    for (const item of tilemapLayers) {
      if (!layers.some((layer) => layer.id === item.layerId)) {
        layers.push({ id: item.layerId, order: item.order, visible: item.layer && item.layer.visible !== false, kind: 'tilemap' });
      }
    }
    const tilemapStats = { layerCount: tilemapLayers.length, cellCount: 0, drawnTileCount: 0, missingTileRefCount: 0, assetTileCount: 0 };
    const tilemapWarnings = [];
    const renderables = [];
    for (const item of tilemapLayers) {
      const layerData = Array.isArray(item.layer && item.layer.data) ? item.layer.data : [];
      let tileCount = 0;
      let missingTileRefCount = 0;
      let assetTileCount = 0;
      for (const tileId of layerData) {
        if (!tileId) continue;
        tilemapStats.cellCount += 1;
        const tile = item.tilemap && item.tilemap.tileIndex && item.tilemap.tileIndex[tileId];
        if (!tile) {
          missingTileRefCount += 1;
          tilemapStats.missingTileRefCount += 1;
          tilemapWarnings.push(`tilemap ${item.tilemapId} layer ${item.layerId} references missing tile ${tileId}`);
          continue;
        }
        tileCount += 1;
        tilemapStats.drawnTileCount += 1;
        if (tile.asset) {
          assetTileCount += 1;
          tilemapStats.assetTileCount += 1;
        }
      }
      renderables.push({
        id: `tilemap-${item.tilemapId}-${item.layerId}`,
        sourceKind: 'tilemap-layer',
        sourceId: `${item.tilemapId}:${item.layerId}`,
        layer: item.layerId,
        layerOrder: Number.isFinite(item.order) ? item.order : 0,
        localOrder: 0,
        stableKey: `${item.tilemapId}:${item.layerId}`,
        drawOrder: 0,
        primitiveKind: 'tilemap',
        tileCount,
        missingTileRefCount,
        assetTileCount,
        visible: !(item.layer && item.layer.visible === false) && tileCount > 0,
        fallbackReason: item.layer && item.layer.visible === false ? 'tilemap layer hidden' : (tileCount === 0 ? (missingTileRefCount > 0 ? 'tilemap layer has missing tile refs' : 'tilemap layer empty') : null),
      });
    }
    const queueEntities = Array.isArray(world.entities) ? world.entities : [];
    for (let entityIndex = 0; entityIndex < queueEntities.length; entityIndex += 1) {
      const entity = queueEntities[entityIndex];
      const sprite = entity.sprite || {};
      const layer = sprite.layer || 'default';
      const visible = sprite.visible !== false && visibility.get(layer) !== false;
      renderables.push({
        id: `entity-${entity.id || 'missing-id'}`,
        sourceKind: 'entity',
        sourceId: String(entity.id || ''),
        // Unique queue-local index so draws resolve to THIS entity, not the last
        // entity sharing the same (possibly duplicate or empty) id.
        entityIndex,
        layer,
        layerOrder: orders.get(layer) || 0,
        localOrder: Number.isFinite(sprite.order) ? sprite.order : 0,
        stableKey: String(entity.id || ''),
        drawOrder: 0,
        primitiveKind: primitiveCategory(entity),
        visible,
        fallbackReason: visible ? null : (sprite.visible === false ? 'sprite hidden' : 'renderer layer hidden'),
      });
    }
    if (activeRenderer.debug.showCamera || activeRenderer.debug.showBounds || activeRenderer.debug.showEntityIds) {
      const debugLayer = { id: '__debug_overlay', order: 1000000, visible: true, kind: 'debug-overlay' };
      layers.push(debugLayer);
      if (activeRenderer.debug.showCamera) {
        renderables.push({ id: 'debug-camera', sourceKind: 'debug-overlay', sourceId: 'camera', layer: debugLayer.id, layerOrder: debugLayer.order, localOrder: 0, stableKey: 'debug-camera', drawOrder: 0, primitiveKind: 'debug_camera', visible: true, fallbackReason: null });
      }
      if (activeRenderer.debug.showBounds) {
        const boundsEntities = Array.isArray(world.entities) ? world.entities : [];
        for (let entityIndex = 0; entityIndex < boundsEntities.length; entityIndex += 1) {
          const entityId = String(boundsEntities[entityIndex].id || '');
          renderables.push({ id: `debug-bounds-${entityId}`, sourceKind: 'debug-overlay', sourceId: entityId, entityIndex, layer: debugLayer.id, layerOrder: debugLayer.order, localOrder: 10, stableKey: `debug-bounds-${entityId}`, drawOrder: 0, primitiveKind: 'debug_bounds', visible: true, fallbackReason: null });
        }
      }
      if (activeRenderer.debug.showEntityIds) {
        const labelEntities = Array.isArray(world.entities) ? world.entities : [];
        for (let entityIndex = 0; entityIndex < labelEntities.length; entityIndex += 1) {
          const entityId = String(labelEntities[entityIndex].id || '');
          renderables.push({ id: `debug-label-${entityId}`, sourceKind: 'debug-overlay', sourceId: entityId, entityIndex, layer: debugLayer.id, layerOrder: debugLayer.order, localOrder: 20, stableKey: `debug-label-${entityId}`, drawOrder: 0, primitiveKind: 'debug_label', visible: true, fallbackReason: null });
        }
      }
    }
    renderables.sort((left, right) => (
      left.layerOrder - right.layerOrder
      || compareCodeUnits(left.layer, right.layer)
      || left.localOrder - right.localOrder
      || compareCodeUnits(left.stableKey, right.stableKey)
    ));
    renderables.forEach((renderable, drawOrder) => { renderable.drawOrder = drawOrder; });
    const warnings = renderables
      .filter((renderable) => renderable.visible === false && !renderable.fallbackReason)
      .map((renderable) => `hidden renderable ${renderable.id} should include fallbackReason`)
      .concat(tilemapWarnings);
    return {
      schemaVersion: 'ouroforge.scene-render-queue.v1',
      frameId: String(frameId),
      sceneId: String(world.sceneId || 'unknown-scene'),
      layers,
      renderables,
      tilemapStats,
      validation: { status: warnings.length ? 'warning' : 'ready', blockedReasons: [], warnings },
      readOnlyInspection: { trustedEmitter: 'browser-runtime-renderer', browserStudioMode: 'read-only evidence inspection', disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    };
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
        assetFrameRef: entity.sprite && typeof entity.sprite.frameId === 'string' ? entity.sprite.frameId : null,
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

  function drawTilemapLayer({ context, renderer, item, assets }) {
    if (!context || !item || !item.tilemap || !item.layer) return [];
    const offset = cameraOffsetForLayer(renderer, item.layer.id);
    const { tilemap: tilemapModel, layer } = item;
    const drawn = [];
    for (let cell = 0; cell < layer.data.length; cell += 1) {
      const tileId = layer.data[cell];
      if (!tileId) continue;
      const tile = tilemapModel.tileIndex && tilemapModel.tileIndex[tileId];
      if (!tile) continue;
      const column = cell % tilemapModel.grid.width;
      const row = Math.floor(cell / tilemapModel.grid.width);
      const x = (column * tilemapModel.tileSize.width) - offset.x;
      const y = (row * tilemapModel.tileSize.height) - offset.y;
      const image = tile.asset && assets && typeof assets.imageFor === 'function' ? assets.imageFor(tile.asset) : null;
      if (image) {
        context.drawImage(image, x, y, tilemapModel.tileSize.width, tilemapModel.tileSize.height);
      } else {
        context.fillStyle = tile.color;
        context.fillRect(x, y, tilemapModel.tileSize.width, tilemapModel.tileSize.height);
      }
      drawn.push({ tilemapId: tilemapModel.id, layerId: layer.id, tileId, x, y });
    }
    return drawn;
  }

  function drawEntityRenderable({ context, renderer, entity, assets, animation }) {
    const components = entity.components || {};
    const transform = point(components.transform);
    const entitySize = size(components.size, { width: 16, height: 16 });
    const layer = entity.sprite && entity.sprite.layer ? entity.sprite.layer : 'default';
    const screen = worldToScreen(transform, renderer, layer);
    const x = screen.x;
    const y = screen.y;
    const activeFrame = animation && typeof animation.activeSpriteFrame === 'function'
      ? animation.activeSpriteFrame(components.animation)
      : null;
    const frameAsset = activeFrame && typeof activeFrame.asset === 'string' ? activeFrame.asset : null;
    const spriteAsset = entity.sprite && typeof entity.sprite.asset === 'string' ? entity.sprite.asset : null;
    const spriteFrameId = entity.sprite && typeof entity.sprite.frameId === 'string' ? entity.sprite.frameId : null;
    const preferFrameColor = activeFrame && typeof activeFrame.color === 'string' && !frameAsset;
    // An active animation frame asset (frameAsset) overrides the base sprite, so
    // skip the sprite-atlas path when one is present; otherwise the base sprite
    // sheet would be drawn and the animation override silently dropped.
    const spriteImage = !preferFrameColor && !frameAsset && assets && typeof assets.spriteFor === 'function' && spriteAsset
      ? assets.spriteFor(spriteAsset, spriteFrameId)
      : null;
    const image = spriteImage && spriteImage.image ? spriteImage.image : (!preferFrameColor && assets && typeof assets.imageFor === 'function'
      ? assets.imageFor(frameAsset || spriteAsset)
      : null);
    if (image && spriteImage && spriteImage.frame) {
      const frame = spriteImage.frame;
      context.drawImage(image, frame.x, frame.y, frame.width, frame.height, x, y, entitySize.width, entitySize.height);
    } else if (image) {
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
  }

  function drawDebugRenderable({ context, renderer, entity, primitiveKind, world }) {
    context.fillStyle = '#f2f6f8';
    context.font = '10px ui-monospace, monospace';
    if (primitiveKind === 'debug_camera') {
      context.fillText(`camera=${renderer.camera.x},${renderer.camera.y}`, 8, 28);
      return;
    }
    if (!entity) return;
    const components = entity.components || {};
    const transform = point(components.transform);
    const entitySize = size(components.size, { width: 16, height: 16 });
    const layer = entity && entity.sprite && entity.sprite.layer ? entity.sprite.layer : 'default';
    const screen = worldToScreen(transform, renderer, layer);
    const x = screen.x;
    const y = screen.y;
    if (primitiveKind === 'debug_bounds') {
      if (typeof context.strokeRect === 'function') {
        context.strokeStyle = '#f2f6f8';
        context.strokeRect(x, y, entitySize.width, entitySize.height);
      }
      return;
    }
    if (primitiveKind === 'debug_label') {
      context.fillText(entity.id || world.sceneId || 'entity', x, y - 2);
    }
  }

  function drawRuntime({ canvas, context, world, renderer, assets, animation, tilemap }) {
    if (!canvas || !context || !world) return [];
    const activeRenderer = normalizeRenderer(renderer, world.bounds || { width: canvas.width, height: canvas.height });
    const queue = renderQueue({ world, renderer: activeRenderer, tilemap, frameId: `tick-${world.tick ?? 0}` });
    // Resolve entity draws by unique queue-local index rather than by id: a Map
    // keyed on id silently collapses duplicate or missing ids to a single entity,
    // drawing the last match repeatedly and omitting the earlier entities.
    const drawEntities = Array.isArray(world.entities) ? world.entities : [];
    const tilemapLayersById = new Map((tilemap && typeof tilemap.orderedLayers === 'function' ? tilemap.orderedLayers(world.tilemaps || []) : [])
      .map((item) => [`${item.tilemapId}:${item.layerId}`, item]));
    context.clearRect(0, 0, canvas.width, canvas.height);
    context.fillStyle = activeRenderer.background;
    context.fillRect(0, 0, canvas.width, canvas.height);
    for (const renderable of queue.renderables) {
      if (renderable.visible === false) continue;
      if (renderable.sourceKind === 'tilemap-layer') {
        drawTilemapLayer({ context, renderer: activeRenderer, item: tilemapLayersById.get(renderable.sourceId), assets });
      } else if (renderable.sourceKind === 'entity') {
        const entity = drawEntities[renderable.entityIndex];
        if (entity) drawEntityRenderable({ context, renderer: activeRenderer, entity, assets, animation });
      } else if (renderable.sourceKind === 'debug-overlay') {
        const debugEntity = Number.isInteger(renderable.entityIndex) ? drawEntities[renderable.entityIndex] : undefined;
        drawDebugRenderable({ context, renderer: activeRenderer, entity: debugEntity, primitiveKind: renderable.primitiveKind, world });
      }
    }
    context.fillStyle = '#f2f6f8';
    context.font = '10px ui-monospace, monospace';
    context.fillText(`scene=${world.sceneId} tick=${world.tick}`, 8, 14);
    return queue.renderables
      .filter((renderable) => renderable.sourceKind === 'entity' && renderable.visible !== false)
      .map((renderable) => ({ entityId: renderable.sourceId, layer: renderable.layer, layerOrder: renderable.layerOrder, spriteOrder: renderable.localOrder }));
  }

  const api = Object.freeze({ normalizeRenderer, renderOrder, renderQueue, renderBreakdown, compareBreakdowns, debugState, drawRuntime, worldToScreen, cameraOffsetForLayer, clone });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeRenderer = api;
})(typeof window !== 'undefined' ? window : globalThis);

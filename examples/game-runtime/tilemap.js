(function attachTilemap(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function compareCodeUnits(a, b) {
    return a < b ? -1 : a > b ? 1 : 0;
  }

  function size(value = {}, fallback = { width: 16, height: 16 }) {
    return {
      width: Number.isFinite(value.width) && value.width > 0 ? value.width : fallback.width,
      height: Number.isFinite(value.height) && value.height > 0 ? value.height : fallback.height,
    };
  }

  function normalizeLayer(layer = {}, expectedCells = 0) {
    const data = Array.isArray(layer.data) ? layer.data.slice(0, expectedCells) : [];
    while (data.length < expectedCells) data.push(null);
    return {
      id: String(layer.id || 'layer'),
      order: Number.isFinite(layer.order) ? layer.order : 0,
      visible: layer.visible !== false,
      data: data.map((tileId) => (typeof tileId === 'string' && tileId.length > 0 ? tileId : null)),
      collisionLayer: typeof layer.collisionLayer === 'string' ? layer.collisionLayer : null,
      metadata: layer.metadata && typeof layer.metadata === 'object' && !Array.isArray(layer.metadata) ? clone(layer.metadata) : {},
    };
  }

  function normalizeTilemap(tilemap = {}, index = 0) {
    const grid = {
      width: Number.isInteger(tilemap.grid && tilemap.grid.width) && tilemap.grid.width > 0 ? tilemap.grid.width : 1,
      height: Number.isInteger(tilemap.grid && tilemap.grid.height) && tilemap.grid.height > 0 ? tilemap.grid.height : 1,
    };
    const expectedCells = grid.width * grid.height;
    const tileSize = size(tilemap.tileSize);
    const tiles = [];
    const tileIndex = Object.create(null);
    for (const tile of Array.isArray(tilemap.tiles) ? tilemap.tiles : []) {
      if (!tile || typeof tile.id !== 'string') continue;
      const normalizedTile = {
        id: tile.id,
        color: typeof tile.color === 'string' ? tile.color : '#334155',
        asset: typeof tile.asset === 'string' ? tile.asset : null,
        solid: Boolean(tile.solid),
        hazard: Boolean(tile.hazard),
        goal: Boolean(tile.goal),
        trigger: typeof tile.trigger === 'string' && tile.trigger.length > 0 ? tile.trigger : null,
      };
      tiles.push(normalizedTile);
      tileIndex[normalizedTile.id] = normalizedTile;
    }
    return {
      id: String(tilemap.id || `tilemap-${index}`),
      tileSize,
      grid,
      tiles,
      tileIndex,
      layers: (Array.isArray(tilemap.layers) ? tilemap.layers : [])
        .map((layer) => normalizeLayer(layer, expectedCells)),
      metadata: tilemap.metadata && typeof tilemap.metadata === 'object' && !Array.isArray(tilemap.metadata) ? clone(tilemap.metadata) : {},
    };
  }

  function normalizeTilemaps(tilemaps = []) {
    return (Array.isArray(tilemaps) ? tilemaps : []).map(normalizeTilemap);
  }

  function orderedLayers(tilemaps = []) {
    return tilemaps
      .flatMap((tilemap) => tilemap.layers
        .filter((layer) => layer.visible)
        .map((layer) => ({ tilemap, layer, tilemapId: tilemap.id, layerId: layer.id, order: layer.order })))
      .sort((left, right) => (
        left.order - right.order
        || compareCodeUnits(left.tilemapId, right.tilemapId)
        || compareCodeUnits(left.layerId, right.layerId)
      ));
  }

  function cellRect(tilemap, cell) {
    const column = cell % tilemap.grid.width;
    const row = Math.floor(cell / tilemap.grid.width);
    return {
      x: column * tilemap.tileSize.width,
      y: row * tilemap.tileSize.height,
      width: tilemap.tileSize.width,
      height: tilemap.tileSize.height,
      column,
      row,
    };
  }

  function extractAuthoringCells(tilemaps = []) {
    const collisionCells = [];
    const triggerCells = [];
    const hazardCells = [];
    const goalCells = [];
    for (const tilemap of tilemaps) {
      for (const layer of tilemap.layers) {
        for (let cell = 0; cell < layer.data.length; cell += 1) {
          const tileId = layer.data[cell];
          if (!tileId) continue;
          const tile = tilemap.tileIndex && tilemap.tileIndex[tileId];
          if (!tile) continue;
          const rect = cellRect(tilemap, cell);
          const extracted = {
            tilemapId: tilemap.id,
            layerId: layer.id,
            tileId,
            index: cell,
            x: rect.column,
            y: rect.row,
            worldX: rect.x,
            worldY: rect.y,
            width: rect.width,
            height: rect.height,
            trigger: tile.trigger,
          };
          if (tile.solid || layer.collisionLayer === layer.id) collisionCells.push(extracted);
          if (tile.trigger || layer.metadata.trigger === 'true') triggerCells.push(extracted);
          if (tile.hazard) hazardCells.push(extracted);
          if (tile.goal) goalCells.push(extracted);
        }
      }
    }
    return { version: '1', collisionCells, triggerCells, hazardCells, goalCells };
  }

  function syntheticEntityId(kind, cell) {
    return `tilemap.${kind}.${cell.tilemapId}.${cell.layerId}.${cell.x}.${cell.y}.${cell.tileId}`;
  }

  function syntheticEntityForCell(kind, cell) {
    const trigger = kind === 'trigger' ? {
      id: syntheticEntityId(kind, cell),
      kind: 'overlap',
      targetFlag: cell.trigger,
      requiredFlags: [],
      onEnter: cell.trigger ? [{ kind: 'setFlag', flag: cell.trigger, value: true }] : [],
    } : null;
    return {
      id: syntheticEntityId(kind, cell),
      sprite: { visible: false, color: kind === 'trigger' ? '#facc15' : '#64748b' },
      components: {
        transform: { x: cell.worldX, y: cell.worldY },
        velocity: { x: 0, y: 0 },
        size: { width: cell.width, height: cell.height },
        controllable: false,
        collider: {
          shape: 'aabb',
          offset: { x: 0, y: 0 },
          size: { width: cell.width, height: cell.height },
          sensor: kind === 'trigger',
          trigger: kind === 'trigger',
          body: 'static',
        },
        ...(trigger ? { trigger } : {}),
      },
      tags: ['tilemap', kind],
      metadata: { tilemapId: cell.tilemapId, layerId: cell.layerId, tileId: cell.tileId, cellX: String(cell.x), cellY: String(cell.y) },
    };
  }

  function collisionEntities(tilemaps = []) {
    const cells = extractAuthoringCells(tilemaps);
    const seen = new Set();
    const entities = [];
    for (const cell of cells.collisionCells) {
      const key = `${cell.worldX}:${cell.worldY}:${cell.width}:${cell.height}:collision`;
      if (seen.has(key)) continue;
      seen.add(key);
      entities.push(syntheticEntityForCell('collision', cell));
    }
    for (const cell of cells.triggerCells) {
      const key = `${cell.worldX}:${cell.worldY}:${cell.width}:${cell.height}:trigger:${cell.trigger || ''}`;
      if (seen.has(key)) continue;
      seen.add(key);
      entities.push(syntheticEntityForCell('trigger', cell));
    }
    return entities;
  }

  function entityById(tilemaps = [], entityId) {
    return collisionEntities(tilemaps).find((entity) => entity.id === entityId) || null;
  }

  function drawTilemaps({ context, renderer, tilemaps = [], assets }) {
    if (!context) return [];
    const camera = renderer && renderer.camera ? renderer.camera : { x: 0, y: 0 };
    const drawn = [];
    for (const item of orderedLayers(tilemaps)) {
      const { tilemap, layer } = item;
      for (let cell = 0; cell < layer.data.length; cell += 1) {
        const tileId = layer.data[cell];
        if (!tileId) continue;
        const tile = tilemap.tileIndex && tilemap.tileIndex[tileId];
        if (!tile) continue;
        const rect = cellRect(tilemap, cell);
        const x = rect.x - camera.x;
        const y = rect.y - camera.y;
        const image = tile.asset && assets && typeof assets.imageFor === 'function' ? assets.imageFor(tile.asset) : null;
        if (image) {
          context.drawImage(image, x, y, tilemap.tileSize.width, tilemap.tileSize.height);
        } else {
          context.fillStyle = tile.color;
          context.fillRect(x, y, tilemap.tileSize.width, tilemap.tileSize.height);
        }
        drawn.push({ tilemapId: tilemap.id, layerId: layer.id, tileId, x, y });
      }
    }
    return drawn;
  }

  function debugState(tilemaps = []) {
    return {
      version: '1',
      tilemaps: tilemaps.map((tilemap) => ({
        id: tilemap.id,
        tileSize: clone(tilemap.tileSize),
        grid: clone(tilemap.grid),
        tileCount: Array.isArray(tilemap.tiles) ? tilemap.tiles.length : 0,
        authoring: extractAuthoringCells([tilemap]),
        layers: tilemap.layers.map((layer) => ({
          id: layer.id,
          order: layer.order,
          visible: layer.visible,
          cellCount: layer.data.length,
          nonEmptyCells: layer.data.filter(Boolean).length,
          collisionLayer: layer.collisionLayer,
        })),
      })),
      layerOrder: orderedLayers(tilemaps).map(({ tilemapId, layerId, order }) => ({ tilemapId, layerId, order })),
    };
  }

  const api = Object.freeze({ normalizeTilemaps, orderedLayers, drawTilemaps, debugState, extractAuthoringCells, collisionEntities, entityById });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeTilemap = api;
})(typeof window !== 'undefined' ? window : globalThis);

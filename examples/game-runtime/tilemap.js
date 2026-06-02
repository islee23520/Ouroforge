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
        const column = cell % tilemap.grid.width;
        const row = Math.floor(cell / tilemap.grid.width);
        const x = column * tilemap.tileSize.width - camera.x;
        const y = row * tilemap.tileSize.height - camera.y;
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

  const api = Object.freeze({ normalizeTilemaps, orderedLayers, drawTilemaps, debugState });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeTilemap = api;
})(typeof window !== 'undefined' ? window : globalThis);

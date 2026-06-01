(function attachCollision(root) {
  function rectForEntity(entity) {
    const transform = entity.components.transform;
    const collider = entity.components.collider;
    if (!collider || collider.shape !== 'aabb') return null;
    const offset = collider.offset || { x: 0, y: 0 };
    const size = collider.size || entity.components.size;
    return {
      entityId: entity.id,
      body: collider.body || 'static',
      sensor: Boolean(collider.sensor),
      x: transform.x + (offset.x || 0),
      y: transform.y + (offset.y || 0),
      width: size.width,
      height: size.height,
    };
  }

  function overlaps(a, b) {
    return a.x < b.x + b.width
      && a.x + a.width > b.x
      && a.y < b.y + b.height
      && a.y + a.height > b.y;
  }

  function detectAabbCollisions(entities, tick) {
    const colliders = entities
      .map(rectForEntity)
      .filter(Boolean)
      .sort((a, b) => a.entityId.localeCompare(b.entityId));
    const dynamicColliders = colliders.filter((collider) => collider.body === 'dynamic');
    const seen = new Set();
    const events = [];

    for (const dynamic of dynamicColliders) {
      for (const other of colliders) {
        if (dynamic.entityId === other.entityId) continue;
        const pair = [dynamic.entityId, other.entityId].sort().join(':');
        if (seen.has(pair)) continue;
        if (!overlaps(dynamic, other)) continue;
        seen.add(pair);
        events.push({
          tick,
          type: 'runtime.collision.detected',
          pairId: pair,
          dynamicEntityId: dynamic.entityId,
          otherEntityId: other.entityId,
          dynamicBody: dynamic.body,
          otherBody: other.body,
          sensor: dynamic.sensor || other.sensor,
        });
      }
    }

    return events.sort((a, b) => a.pairId.localeCompare(b.pairId));
  }

  const api = Object.freeze({ rectForEntity, overlaps, detectAabbCollisions });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeCollision = api;
})(typeof window !== 'undefined' ? window : globalThis);

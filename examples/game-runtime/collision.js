(function attachCollision(root) {
  function compareCodeUnits(a, b) {
    return a < b ? -1 : a > b ? 1 : 0;
  }

  function uniqueStrings(values) {
    return Array.isArray(values) ? values.map(String).filter(Boolean) : [];
  }

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
      trigger: Boolean(collider.trigger || collider.sensor),
      collisionGroup: typeof collider.collisionGroup === 'string' ? collider.collisionGroup : null,
      collisionMask: uniqueStrings(collider.collisionMask),
      x: transform.x + (offset.x || 0),
      y: transform.y + (offset.y || 0),
      width: size.width,
      height: size.height,
      offsetX: offset.x || 0,
      offsetY: offset.y || 0,
    };
  }

  function overlaps(a, b) {
    return a.x < b.x + b.width
      && a.x + a.width > b.x
      && a.y < b.y + b.height
      && a.y + a.height > b.y;
  }

  function pairId(a, b) {
    return [a.entityId, b.entityId].sort().join(':');
  }

  function groupFor(rect) {
    return rect.collisionGroup || 'default';
  }

  function maskAllows(source, target) {
    return source.collisionMask.length === 0 || source.collisionMask.includes(groupFor(target));
  }

  function pairAllowed(a, b) {
    return maskAllows(a, b) && maskAllows(b, a);
  }

  function stableRects(entities) {
    return entities
      .map((entity) => ({ entity, rect: rectForEntity(entity) }))
      .filter((entry) => entry.rect)
      .sort((a, b) => compareCodeUnits(a.rect.entityId, b.rect.entityId));
  }

  function eventFor({ tick, type, moving, other, normal }) {
    const trigger = moving.trigger || other.trigger;
    const movingIsDynamic = moving.body === 'dynamic';
    const otherIsDynamic = other.body === 'dynamic';
    return {
      tick,
      type,
      pairId: pairId(moving, other),
      dynamicEntityId: movingIsDynamic ? moving.entityId : (otherIsDynamic ? other.entityId : null),
      otherEntityId: movingIsDynamic ? other.entityId : moving.entityId,
      movingEntityId: moving.entityId,
      staticEntityId: other.body === 'static' ? other.entityId : null,
      movingBody: moving.body,
      otherBody: other.body,
      dynamicBody: movingIsDynamic ? moving.body : (otherIsDynamic ? other.body : null),
      sensor: moving.sensor || other.sensor,
      trigger,
      normal,
    };
  }

  function penetrationNormal(a, b) {
    const overlapLeft = (a.x + a.width) - b.x;
    const overlapRight = (b.x + b.width) - a.x;
    const overlapTop = (a.y + a.height) - b.y;
    const overlapBottom = (b.y + b.height) - a.y;
    const minX = Math.min(overlapLeft, overlapRight);
    const minY = Math.min(overlapTop, overlapBottom);
    if (minX <= minY) return overlapLeft <= overlapRight ? { x: -1, y: 0 } : { x: 1, y: 0 };
    return overlapTop <= overlapBottom ? { x: 0, y: -1 } : { x: 0, y: 1 };
  }

  function detectAabbCollisions(entities, tick) {
    const colliders = stableRects(entities).map((entry) => entry.rect);
    const activeColliders = colliders.filter((collider) => collider.body === 'dynamic' || collider.body === 'kinematic');
    const seen = new Set();
    const events = [];

    for (const active of activeColliders) {
      for (const other of colliders) {
        if (active.entityId === other.entityId) continue;
        const id = pairId(active, other);
        if (seen.has(id) || !pairAllowed(active, other) || !overlaps(active, other)) continue;
        seen.add(id);
        const trigger = active.trigger || other.trigger;
        events.push(eventFor({
          tick,
          type: trigger ? 'runtime.collision.trigger' : 'runtime.collision.contact',
          moving: active,
          other,
          normal: trigger ? { x: 0, y: 0 } : penetrationNormal(active, other),
        }));
      }
    }

    return events.sort((a, b) => compareCodeUnits(a.pairId, b.pairId) || compareCodeUnits(a.type, b.type));
  }

  function clamp(value, min, max) {
    return Math.max(min, Math.min(max, value));
  }

  function moveWithoutCollider(entity, bounds) {
    const transform = entity.components.transform;
    const velocity = entity.components.velocity || { x: 0, y: 0 };
    const size = entity.components.size || { width: 0, height: 0 };
    transform.x = clamp(transform.x + (velocity.x || 0), 0, Math.max(0, bounds.width - size.width));
    transform.y = clamp(transform.y + (velocity.y || 0), 0, Math.max(0, bounds.height - size.height));
  }

  function refreshRect(entry) {
    entry.rect = rectForEntity(entry.entity);
    return entry.rect;
  }

  function resolveAxis({ entry, others, bounds, axis, tick, events, seen }) {
    const entity = entry.entity;
    const transform = entity.components.transform;
    const velocity = entity.components.velocity || { x: 0, y: 0 };
    const size = entity.components.size || { width: 0, height: 0 };
    const delta = velocity[axis] || 0;
    const limit = axis === 'x' ? Math.max(0, bounds.width - size.width) : Math.max(0, bounds.height - size.height);
    transform[axis] = clamp(transform[axis] + delta, 0, limit);
    const active = refreshRect(entry);
    if (!active || active.trigger) return;

    for (const otherEntry of others) {
      const other = refreshRect(otherEntry);
      if (!other || active.entityId === other.entityId || other.trigger || !pairAllowed(active, other) || !overlaps(active, other)) continue;
      const id = `${pairId(active, other)}:contact:${axis}`;
      const normal = axis === 'x'
        ? { x: delta >= 0 ? -1 : 1, y: 0 }
        : { x: 0, y: delta >= 0 ? -1 : 1 };
      if (axis === 'x') {
        transform.x = delta >= 0
          ? other.x - active.width - active.offsetX
          : other.x + other.width - active.offsetX;
        transform.x = clamp(transform.x, 0, limit);
      } else {
        transform.y = delta >= 0
          ? other.y - active.height - active.offsetY
          : other.y + other.height - active.offsetY;
        transform.y = clamp(transform.y, 0, limit);
      }
      refreshRect(entry);
      if (!seen.has(id)) {
        seen.add(id);
        events.push(eventFor({ tick, type: 'runtime.collision.contact', moving: active, other, normal }));
      }
    }
  }

  function stepAabbPhysics(entities, bounds, tick) {
    const worldBounds = bounds || { width: 320, height: 180 };
    const entries = stableRects(entities);
    const entryById = new Map(entries.map((entry) => [entry.rect.entityId, entry]));
    const events = [];
    const seen = new Set();

    for (const entity of entities) {
      const entry = entryById.get(entity.id);
      const body = entry && entry.rect.body;
      if (entry && body === 'static') continue;
      if (!entry || body === 'dynamic' || body === 'kinematic') {
        if (!entry) {
          moveWithoutCollider(entity, worldBounds);
          continue;
        }
        const others = entries.filter((candidate) => candidate !== entry);
        resolveAxis({ entry, others, bounds: worldBounds, axis: 'x', tick, events, seen });
        resolveAxis({ entry, others, bounds: worldBounds, axis: 'y', tick, events, seen });
      }
    }

    const triggers = detectAabbCollisions(entities, tick)
      .filter((event) => event.trigger)
      .filter((event) => {
        const id = `${event.pairId}:trigger`;
        if (seen.has(id)) return false;
        seen.add(id);
        return true;
      });

    return { events: events.concat(triggers).sort((a, b) => compareCodeUnits(a.pairId, b.pairId) || compareCodeUnits(a.type, b.type)) };
  }

  const api = Object.freeze({ rectForEntity, overlaps, detectAabbCollisions, stepAabbPhysics });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeCollision = api;
})(typeof window !== 'undefined' ? window : globalThis);

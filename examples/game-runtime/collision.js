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
    if (!collider || collider.shape !== 'aabb' || collider.disabled) return null;
    const offset = collider.offset || { x: 0, y: 0 };
    const size = collider.size || entity.components.size;
    const body = collider.body || 'static';
    const contactOnly = body === 'trigger' || body === 'sensor';
    return {
      entityId: entity.id,
      body,
      sensor: Boolean(collider.sensor || body === 'sensor'),
      trigger: Boolean(collider.trigger || collider.sensor || contactOnly),
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

  function defaultLayerFor(options) {
    return options && typeof options.defaultLayer === 'string' && options.defaultLayer
      ? options.defaultLayer
      : 'default';
  }

  function groupFor(rect, options) {
    return rect.collisionGroup || defaultLayerFor(options);
  }

  function maskAllows(source, target, options) {
    return source.collisionMask.length === 0 || source.collisionMask.includes(groupFor(target, options));
  }

  function pairAllowed(a, b, options) {
    return maskAllows(a, b, options) && maskAllows(b, a, options);
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

  function detectAabbCollisions(entities, tick, options = {}) {
    const colliders = stableRects(entities).map((entry) => entry.rect);
    const activeColliders = colliders.filter((collider) => collider.body === 'dynamic' || collider.body === 'kinematic');
    const seen = new Set();
    const events = [];

    for (const active of activeColliders) {
      for (const other of colliders) {
        if (active.entityId === other.entityId) continue;
        const id = pairId(active, other);
        if (seen.has(id) || !pairAllowed(active, other, options) || !overlaps(active, other)) continue;
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

  function intervalsOverlap(aMin, aMax, bMin, bMax) {
    return aMin < bMax && aMax > bMin;
  }

  function perpendicularOverlap(a, b, axis) {
    return axis === 'x'
      ? intervalsOverlap(a.y, a.y + a.height, b.y, b.y + b.height)
      : intervalsOverlap(a.x, a.x + a.width, b.x, b.x + b.width);
  }

  function sweptAxisCollision(before, after, other, axis, delta) {
    if (!before || !after || !other || delta === 0 || !perpendicularOverlap(after, other, axis)) return false;
    if (axis === 'x') {
      return delta > 0
        ? before.x + before.width <= other.x && after.x + after.width > other.x
        : before.x >= other.x + other.width && after.x < other.x + other.width;
    }
    return delta > 0
      ? before.y + before.height <= other.y && after.y + after.height > other.y
      : before.y >= other.y + other.height && after.y < other.y + other.height;
  }

  function resolveAxis({ entry, others, bounds, axis, tick, events, seen, options }) {
    const entity = entry.entity;
    const transform = entity.components.transform;
    const velocity = entity.components.velocity || { x: 0, y: 0 };
    const size = entity.components.size || { width: 0, height: 0 };
    const delta = velocity[axis] || 0;
    const limit = axis === 'x' ? Math.max(0, bounds.width - size.width) : Math.max(0, bounds.height - size.height);
    const before = refreshRect(entry);
    transform[axis] = clamp(transform[axis] + delta, 0, limit);
    const active = refreshRect(entry);
    if (!active || active.trigger) return;

    for (const otherEntry of others) {
      const other = refreshRect(otherEntry);
      if (!other || active.entityId === other.entityId || other.trigger || !pairAllowed(active, other, options)) continue;
      const blocked = overlaps(active, other) || sweptAxisCollision(before, active, other, axis, delta);
      if (!blocked) continue;
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

  function stepAabbPhysics(entities, bounds, tick, options = {}) {
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
        resolveAxis({ entry, others, bounds: worldBounds, axis: 'x', tick, events, seen, options });
        resolveAxis({ entry, others, bounds: worldBounds, axis: 'y', tick, events, seen, options });
      }
    }

    const triggers = detectAabbCollisions(entities, tick, options)
      .filter((event) => event.trigger)
      .filter((event) => {
        const id = `${event.pairId}:trigger`;
        if (seen.has(id)) return false;
        seen.add(id);
        return true;
      });

    return { events: events.concat(triggers).sort((a, b) => compareCodeUnits(a.pairId, b.pairId) || compareCodeUnits(a.type, b.type)) };
  }

  function vector3(value = {}, fallback = { x: 0, y: 0, z: 0 }) {
    const source = value && typeof value === 'object' ? value : {};
    return {
      x: Number.isFinite(source.x) ? source.x : fallback.x,
      y: Number.isFinite(source.y) ? source.y : fallback.y,
      z: Number.isFinite(source.z) ? source.z : fallback.z,
    };
  }

  function positiveVector3(value) {
    const source = value && typeof value === 'object' ? value : {};
    if (!Number.isFinite(source.x) || !Number.isFinite(source.y) || !Number.isFinite(source.z)) return null;
    if (source.x <= 0 || source.y <= 0 || source.z <= 0) return null;
    return { x: source.x, y: source.y, z: source.z };
  }

  function nodeColliderRef(node) {
    if (node && typeof node.colliderRef === 'string') return node.colliderRef;
    const component = node && Array.isArray(node.components)
      ? node.components.find((entry) => entry && entry.kind === 'collider' && typeof entry.reference === 'string')
      : null;
    return component ? component.reference : null;
  }

  function scene3dBoxForNode({ node, collider, invalidColliders }) {
    const nodeId = String(node && node.id || 'node');
    if (!collider) {
      invalidColliders.push({ nodeId, colliderRef: nodeColliderRef(node), reason: `missing collider ${nodeColliderRef(node) || 'unreferenced'}` });
      return null;
    }
    if (collider.disabled) return { disabled: true, nodeId, colliderId: collider.id };
    if (collider.shape !== 'box') {
      invalidColliders.push({ nodeId, colliderId: collider.id, reason: `unsupported 3d collider shape ${collider.shape || 'unknown'}` });
      return null;
    }
    const size = positiveVector3(collider.size);
    if (!size) {
      invalidColliders.push({ nodeId, colliderId: collider.id, reason: '3d collider box size must be positive x/y/z' });
      return null;
    }
    const body = ['static', 'dynamic', 'kinematic', 'trigger'].includes(collider.body) ? collider.body : 'static';
    const trigger = Boolean(collider.trigger || collider.sensor || body === 'trigger');
    const transform = node.worldTransform || node.localTransform || {};
    const translation = vector3(transform.translation);
    const offset = vector3(collider.offset);
    return {
      nodeId,
      colliderId: collider.id,
      shape: collider.shape,
      body,
      trigger,
      sensor: Boolean(collider.sensor || body === 'trigger'),
      collisionGroup: typeof collider.collisionGroup === 'string' ? collider.collisionGroup : null,
      collisionMask: uniqueStrings(collider.collisionMask),
      x: translation.x + offset.x,
      y: translation.y + offset.y,
      z: translation.z + offset.z,
      width: size.x,
      height: size.y,
      depth: size.z,
    };
  }

  function overlaps3d(a, b) {
    return a.x < b.x + b.width
      && a.x + a.width > b.x
      && a.y < b.y + b.height
      && a.y + a.height > b.y
      && a.z < b.z + b.depth
      && a.z + a.depth > b.z;
  }

  function pairId3d(a, b) {
    return [a.nodeId, b.nodeId].sort().join(':');
  }

  function groupFor3d(box) {
    return box.collisionGroup || 'default';
  }

  function maskAllows3d(source, target) {
    return source.collisionMask.length === 0 || source.collisionMask.includes(groupFor3d(target));
  }

  function pairAllowed3d(a, b) {
    return maskAllows3d(a, b) && maskAllows3d(b, a);
  }

  function penetrationNormal3d(a, b) {
    const overlapsByAxis = [
      { axis: 'x', amount: Math.min((a.x + a.width) - b.x, (b.x + b.width) - a.x), sign: (a.x + (a.width / 2)) <= (b.x + (b.width / 2)) ? -1 : 1 },
      { axis: 'y', amount: Math.min((a.y + a.height) - b.y, (b.y + b.height) - a.y), sign: (a.y + (a.height / 2)) <= (b.y + (b.height / 2)) ? -1 : 1 },
      { axis: 'z', amount: Math.min((a.z + a.depth) - b.z, (b.z + b.depth) - a.z), sign: (a.z + (a.depth / 2)) <= (b.z + (b.depth / 2)) ? -1 : 1 },
    ].sort((left, right) => left.amount - right.amount || compareCodeUnits(left.axis, right.axis));
    const winner = overlapsByAxis[0];
    return {
      axis: winner.axis,
      normal: {
        x: winner.axis === 'x' ? winner.sign : 0,
        y: winner.axis === 'y' ? winner.sign : 0,
        z: winner.axis === 'z' ? winner.sign : 0,
      },
    };
  }

  function scene3dCollisionSummary({ world = {}, frameId = `tick-${world.tick ?? 0}` } = {}) {
    const scene3d = world && world.scene3d && typeof world.scene3d === 'object' && !Array.isArray(world.scene3d)
      ? world.scene3d
      : null;
    if (!scene3d) {
      return {
        schemaVersion: 'ouroforge.scene3d-collision-evidence.v1',
        present: false,
        frameId: String(frameId),
        sceneId: String(world.sceneId || 'unknown-scene'),
        colliderCount: 0,
        activeColliderCount: 0,
        disabledColliderCount: 0,
        contactCount: 0,
        triggerCount: 0,
        invalidColliderCount: 0,
        events: [],
        invalidColliders: [],
        boundary: 'Read-only bounded 3D collision evidence; no full 3D physics engine, rigidbody parity, ragdoll, joints, vehicle, or character-controller maturity claim.',
      };
    }
    const colliders = Array.isArray(scene3d.colliders) ? scene3d.colliders : [];
    const nodes = Array.isArray(scene3d.nodes) ? scene3d.nodes : [];
    const colliderById = new Map(colliders.filter((collider) => collider && typeof collider.id === 'string').map((collider) => [collider.id, collider]));
    const invalidColliders = [];
    const boxes = [];
    let disabledColliderCount = 0;
    for (const node of nodes) {
      const colliderRef = nodeColliderRef(node);
      if (!colliderRef) continue;
      const box = scene3dBoxForNode({ node, collider: colliderById.get(colliderRef), invalidColliders });
      if (box && box.disabled) {
        disabledColliderCount += 1;
      } else if (box) {
        boxes.push(box);
      }
    }
    const activeBoxes = boxes.filter((box) => box.body === 'dynamic' || box.body === 'kinematic');
    const events = [];
    const seen = new Set();
    for (const active of activeBoxes) {
      for (const other of boxes) {
        if (active.nodeId === other.nodeId) continue;
        const id = pairId3d(active, other);
        if (seen.has(id) || !pairAllowed3d(active, other) || !overlaps3d(active, other)) continue;
        seen.add(id);
        const trigger = active.trigger || other.trigger;
        const normal = trigger ? { axis: 'none', normal: { x: 0, y: 0, z: 0 } } : penetrationNormal3d(active, other);
        events.push({
          tick: Number.isFinite(world.tick) ? world.tick : 0,
          frameId: String(frameId),
          type: trigger ? 'runtime.scene3d.collision.trigger' : 'runtime.scene3d.collision.contact',
          pairId: id,
          dynamicNodeId: active.nodeId,
          otherNodeId: other.nodeId,
          dynamicColliderId: active.colliderId,
          otherColliderId: other.colliderId,
          dynamicBody: active.body,
          otherBody: other.body,
          trigger,
          sensor: active.sensor || other.sensor,
          axis: normal.axis,
          normal: normal.normal,
        });
      }
    }
    events.sort((left, right) => compareCodeUnits(left.pairId, right.pairId) || compareCodeUnits(left.type, right.type));
    return {
      schemaVersion: 'ouroforge.scene3d-collision-evidence.v1',
      present: true,
      frameId: String(frameId),
      sceneId: String(world.sceneId || 'unknown-scene'),
      colliderCount: colliders.length,
      activeColliderCount: boxes.length,
      disabledColliderCount,
      contactCount: events.filter((event) => event.type === 'runtime.scene3d.collision.contact').length,
      triggerCount: events.filter((event) => event.type === 'runtime.scene3d.collision.trigger').length,
      invalidColliderCount: invalidColliders.length,
      events,
      invalidColliders,
      boundary: 'Read-only bounded 3D collision evidence; no full 3D physics engine, rigidbody parity, ragdoll, joints, vehicle, or character-controller maturity claim.',
    };
  }

  const api = Object.freeze({ rectForEntity, overlaps, detectAabbCollisions, scene3dCollisionSummary, stepAabbPhysics });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeCollision = api;
})(typeof window !== 'undefined' ? window : globalThis);

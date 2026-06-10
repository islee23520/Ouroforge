use super::types::{
    Axis, Physics2dBlockedMovement, Physics2dCollisionEvent, Physics2dVector, PhysicsBody, Rect,
};
use crate::{SceneCollider, SceneCollisionLayer, SceneCollisionRules, SceneDocument, SceneEntity};

pub(super) fn rect_for_entity(entity: &SceneEntity) -> Option<Rect> {
    let collider = entity.components.collider.as_ref()?;
    if collider.disabled {
        return None;
    }
    Some(Rect {
        entity_id: entity.id.clone(),
        body: physics_body(collider)?,
        sensor: collider.sensor,
        trigger: collider.trigger || collider.sensor,
        group: collider.collision_group.clone(),
        mask: collider.collision_mask.clone(),
        x: entity.components.transform.x + collider.offset.x,
        y: entity.components.transform.y + collider.offset.y,
        width: collider.size.width,
        height: collider.size.height,
        offset_x: collider.offset.x,
        offset_y: collider.offset.y,
    })
}

pub(super) fn sorted_rects(scene: &SceneDocument) -> Vec<Rect> {
    let mut rects = scene
        .entities
        .iter()
        .filter_map(rect_for_entity)
        .collect::<Vec<_>>();
    rects.sort_by(|left, right| left.entity_id.cmp(&right.entity_id));
    rects
}

pub(super) fn axis_delta(entity: &SceneEntity, axis: Axis) -> i64 {
    match axis {
        Axis::X => entity.components.velocity.x,
        Axis::Y => entity.components.velocity.y,
    }
}

pub(super) fn axis_position(entity: &SceneEntity, axis: Axis) -> i64 {
    match axis {
        Axis::X => entity.components.transform.x,
        Axis::Y => entity.components.transform.y,
    }
}

pub(super) fn is_passive_contact_body(rect: &Rect) -> bool {
    rect.trigger || rect.sensor
}

pub(super) fn pair_allowed(rules: Option<&SceneCollisionRules>, left: &Rect, right: &Rect) -> bool {
    side_allows(rules, left, right) && side_allows(rules, right, left)
}

pub(super) fn overlaps(left: &Rect, right: &Rect) -> bool {
    left.x < right.x + right.width
        && left.x + left.width > right.x
        && left.y < right.y + right.height
        && left.y + left.height > right.y
}

pub(super) fn pair_id(left: &str, right: &str) -> String {
    if left <= right {
        format!("{left}:{right}")
    } else {
        format!("{right}:{left}")
    }
}

pub(super) fn event_for(
    event_type: &str,
    moving: &Rect,
    other: &Rect,
    normal: Physics2dVector,
) -> Physics2dCollisionEvent {
    let dynamic_entity_id = match (moving.body, other.body) {
        (PhysicsBody::Dynamic, _) => Some(moving.entity_id.clone()),
        (_, PhysicsBody::Dynamic) => Some(other.entity_id.clone()),
        (
            PhysicsBody::Static | PhysicsBody::Kinematic,
            PhysicsBody::Static | PhysicsBody::Kinematic,
        ) => None,
    };
    let static_entity_id = match other.body {
        PhysicsBody::Static => Some(other.entity_id.clone()),
        PhysicsBody::Dynamic | PhysicsBody::Kinematic => None,
    };
    Physics2dCollisionEvent {
        event_type: event_type.to_string(),
        pair_id: pair_id(&moving.entity_id, &other.entity_id),
        moving_entity_id: moving.entity_id.clone(),
        other_entity_id: other.entity_id.clone(),
        dynamic_entity_id,
        static_entity_id,
        normal,
        sensor: moving.sensor || other.sensor,
        trigger: moving.trigger || other.trigger,
    }
}

pub(super) fn normal_for_axis(axis: Axis, delta: i64) -> Physics2dVector {
    match axis {
        Axis::X if delta >= 0 => Physics2dVector { x: -1, y: 0 },
        Axis::X => Physics2dVector { x: 1, y: 0 },
        Axis::Y if delta >= 0 => Physics2dVector { x: 0, y: -1 },
        Axis::Y => Physics2dVector { x: 0, y: 1 },
    }
}

pub(super) fn axis_name(axis: Axis) -> &'static str {
    match axis {
        Axis::X => "x",
        Axis::Y => "y",
    }
}

pub(super) fn sort_contact_outputs(
    contact_events: &mut [Physics2dCollisionEvent],
    blocked_movement: &mut [Physics2dBlockedMovement],
) {
    contact_events.sort_by(|left, right| left.pair_id.cmp(&right.pair_id));
    blocked_movement.sort_by(|left, right| {
        (
            left.entity_id.as_str(),
            left.axis.as_str(),
            left.blocked_by.as_str(),
        )
            .cmp(&(
                right.entity_id.as_str(),
                right.axis.as_str(),
                right.blocked_by.as_str(),
            ))
    });
}

pub(super) const fn clamp(value: i64, min: i64, max: i64) -> i64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn physics_body(collider: &SceneCollider) -> Option<PhysicsBody> {
    match collider.body.as_str() {
        "static" => Some(PhysicsBody::Static),
        "dynamic" => Some(PhysicsBody::Dynamic),
        "kinematic" => Some(PhysicsBody::Kinematic),
        _ => None,
    }
}

fn side_allows(rules: Option<&SceneCollisionRules>, source: &Rect, target: &Rect) -> bool {
    let target_group = group_for(rules, target);
    if !source.mask.is_empty() {
        return source.mask.iter().any(|mask| mask == &target_group);
    }
    let Some(layer) = layer_for(rules, source) else {
        return true;
    };
    layer.collides_with.is_empty() || layer.collides_with.iter().any(|id| id == &target_group)
}

fn group_for(rules: Option<&SceneCollisionRules>, rect: &Rect) -> String {
    rect.group.clone().unwrap_or_else(|| {
        rules.map_or_else(
            || "default".to_string(),
            |rules| rules.default_layer.clone(),
        )
    })
}

fn layer_for<'a>(
    rules: Option<&'a SceneCollisionRules>,
    rect: &Rect,
) -> Option<&'a SceneCollisionLayer> {
    let group = group_for(rules, rect);
    rules?.layers.iter().find(|layer| layer.id == group)
}

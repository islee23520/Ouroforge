use super::geometry::{
    axis_delta, axis_name, axis_position, clamp, event_for, is_passive_contact_body,
    normal_for_axis, overlaps, pair_allowed, pair_id, rect_for_entity, sort_contact_outputs,
    sorted_rects,
};
use super::types::{
    Axis, Physics2dBlockedMovement, Physics2dCollisionEvent, Physics2dStepEvidence,
    Physics2dVector, PhysicsBody, Rect,
};
use crate::SceneDocument;
use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, BTreeSet};

pub fn simulate_scene_physics_step(
    scene: &mut SceneDocument,
    tick: u64,
) -> Result<Physics2dStepEvidence> {
    let mut contact_events = Vec::new();
    let mut trigger_events = Vec::new();
    let mut blocked_movement = Vec::new();
    let mut grounded = scene
        .entities
        .iter()
        .filter_map(|entity| {
            rect_for_entity(entity)
                .filter(|rect| rect.body == PhysicsBody::Dynamic)
                .map(|_| (entity.id.clone(), false))
        })
        .collect::<BTreeMap<_, _>>();
    let mut seen_contacts = BTreeSet::new();
    let mut active_ids = scene
        .entities
        .iter()
        .filter_map(|entity| {
            rect_for_entity(entity)
                .filter(|rect| matches!(rect.body, PhysicsBody::Dynamic | PhysicsBody::Kinematic))
                .map(|_| entity.id.clone())
        })
        .collect::<Vec<_>>();
    active_ids.sort();

    for entity_id in active_ids {
        resolve_entity_axis(
            scene,
            &entity_id,
            Axis::X,
            &mut contact_events,
            &mut blocked_movement,
            &mut grounded,
            &mut seen_contacts,
        )?;
        resolve_entity_axis(
            scene,
            &entity_id,
            Axis::Y,
            &mut contact_events,
            &mut blocked_movement,
            &mut grounded,
            &mut seen_contacts,
        )?;
    }

    collect_trigger_events(scene, &mut trigger_events);
    Ok(Physics2dStepEvidence {
        schema_version: "physics-2d-step-evidence-v1".to_string(),
        scene_id: scene.id.clone(),
        tick,
        contact_events,
        trigger_events,
        grounded,
        blocked_movement,
        boundary: "Rust-owned deterministic AABB step evidence; no Box2D compatibility claim, no browser-side trusted writes.".to_string(),
    })
}

fn resolve_entity_axis(
    scene: &mut SceneDocument,
    entity_id: &str,
    axis: Axis,
    contact_events: &mut Vec<Physics2dCollisionEvent>,
    blocked_movement: &mut Vec<Physics2dBlockedMovement>,
    grounded: &mut BTreeMap<String, bool>,
    seen_contacts: &mut BTreeSet<String>,
) -> Result<()> {
    let Some(entity_index) = entity_index(scene, entity_id) else {
        return Ok(());
    };
    let Some(active_before) = rect_for_entity(&scene.entities[entity_index]) else {
        return Ok(());
    };
    if active_before.body == PhysicsBody::Static {
        return Ok(());
    }
    let delta = axis_delta(&scene.entities[entity_index], axis);
    let before_position = axis_position(&scene.entities[entity_index], axis);
    move_entity_axis(scene, entity_index, axis, delta)?;
    let Some(active) = rect_for_entity(&scene.entities[entity_index]) else {
        return Ok(());
    };
    if is_passive_contact_body(&active) {
        return Ok(());
    }

    let mut others = scene
        .entities
        .iter()
        .filter(|entity| entity.id != entity_id)
        .filter_map(rect_for_entity)
        .collect::<Vec<_>>();
    others.sort_by(|left, right| left.entity_id.cmp(&right.entity_id));
    for other in others {
        let Some(current) = rect_for_entity(&scene.entities[entity_index]) else {
            return Ok(());
        };
        if is_passive_contact_body(&other)
            || !pair_allowed(scene.collision_rules.as_ref(), &current, &other)
            || !overlaps(&current, &other)
        {
            continue;
        }
        let normal = normal_for_axis(axis, delta);
        let resolved = resolve_against_other(scene, entity_index, axis, &current, &other, delta)?;
        let pair = pair_id(&current.entity_id, &other.entity_id);
        let contact_key = format!("{pair}:{}", axis_name(axis));
        if seen_contacts.insert(contact_key) {
            contact_events.push(event_for(
                "runtime.collision.contact",
                &current,
                &other,
                normal,
            ));
        }
        if normal.y == -1 {
            grounded.insert(current.entity_id.clone(), true);
        }
        blocked_movement.push(Physics2dBlockedMovement {
            entity_id: current.entity_id,
            axis: axis_name(axis).to_string(),
            attempted_delta: delta,
            resolved_delta: resolved - before_position,
            normal,
            blocked_by: other.entity_id,
        });
    }
    sort_contact_outputs(contact_events, blocked_movement);
    Ok(())
}

fn collect_trigger_events(
    scene: &SceneDocument,
    trigger_events: &mut Vec<Physics2dCollisionEvent>,
) {
    let mut active_rects = sorted_rects(scene)
        .into_iter()
        .filter(|rect| matches!(rect.body, PhysicsBody::Dynamic | PhysicsBody::Kinematic))
        .collect::<Vec<_>>();
    active_rects.sort_by(|left, right| left.entity_id.cmp(&right.entity_id));
    let all_rects = sorted_rects(scene);
    let mut seen = BTreeSet::new();
    for active in &active_rects {
        for other in &all_rects {
            if active.entity_id == other.entity_id
                || !(active.trigger || active.sensor || other.trigger || other.sensor)
                || !pair_allowed(scene.collision_rules.as_ref(), active, other)
                || !overlaps(active, other)
            {
                continue;
            }
            let pair = pair_id(&active.entity_id, &other.entity_id);
            if seen.insert(pair) {
                trigger_events.push(event_for(
                    "runtime.collision.trigger",
                    active,
                    other,
                    Physics2dVector { x: 0, y: 0 },
                ));
            }
        }
    }
    trigger_events.sort_by(|left, right| left.pair_id.cmp(&right.pair_id));
}

fn entity_index(scene: &SceneDocument, entity_id: &str) -> Option<usize> {
    scene
        .entities
        .iter()
        .position(|entity| entity.id == entity_id)
}

fn move_entity_axis(
    scene: &mut SceneDocument,
    entity_index: usize,
    axis: Axis,
    delta: i64,
) -> Result<()> {
    let limit = axis_limit(scene, entity_index, axis)?;
    let transform = &mut scene.entities[entity_index].components.transform;
    match axis {
        Axis::X => transform.x = clamp(transform.x.saturating_add(delta), 0, limit),
        Axis::Y => transform.y = clamp(transform.y.saturating_add(delta), 0, limit),
    }
    Ok(())
}

fn resolve_against_other(
    scene: &mut SceneDocument,
    entity_index: usize,
    axis: Axis,
    current: &Rect,
    other: &Rect,
    delta: i64,
) -> Result<i64> {
    let limit = axis_limit(scene, entity_index, axis)?;
    let transform = &mut scene.entities[entity_index].components.transform;
    let resolved = match axis {
        Axis::X if delta >= 0 => other.x - current.width - current.offset_x,
        Axis::X => other.x + other.width - current.offset_x,
        Axis::Y if delta >= 0 => other.y - current.height - current.offset_y,
        Axis::Y => other.y + other.height - current.offset_y,
    };
    let resolved = clamp(resolved, 0, limit);
    match axis {
        Axis::X => {
            transform.x = resolved;
            scene.entities[entity_index].components.velocity.x = 0;
        }
        Axis::Y => {
            transform.y = resolved;
            scene.entities[entity_index].components.velocity.y = 0;
        }
    }
    Ok(resolved)
}

fn axis_limit(scene: &SceneDocument, entity_index: usize, axis: Axis) -> Result<i64> {
    let entity = scene
        .entities
        .get(entity_index)
        .ok_or_else(|| anyhow!("physics entity index out of bounds"))?;
    Ok(match axis {
        Axis::X => scene
            .bounds
            .width
            .saturating_sub(entity.components.size.width)
            .max(0),
        Axis::Y => scene
            .bounds
            .height
            .saturating_sub(entity.components.size.height)
            .max(0),
    })
}

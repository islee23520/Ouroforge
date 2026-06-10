use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Physics2dStepEvidence {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    pub tick: u64,
    #[serde(rename = "contactEvents")]
    pub contact_events: Vec<Physics2dCollisionEvent>,
    #[serde(rename = "triggerEvents")]
    pub trigger_events: Vec<Physics2dCollisionEvent>,
    pub grounded: BTreeMap<String, bool>,
    #[serde(rename = "blockedMovement")]
    pub blocked_movement: Vec<Physics2dBlockedMovement>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Physics2dCollisionEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "pairId")]
    pub pair_id: String,
    #[serde(rename = "movingEntityId")]
    pub moving_entity_id: String,
    #[serde(rename = "otherEntityId")]
    pub other_entity_id: String,
    #[serde(rename = "dynamicEntityId", skip_serializing_if = "Option::is_none")]
    pub dynamic_entity_id: Option<String>,
    #[serde(rename = "staticEntityId", skip_serializing_if = "Option::is_none")]
    pub static_entity_id: Option<String>,
    pub normal: Physics2dVector,
    pub sensor: bool,
    pub trigger: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Physics2dBlockedMovement {
    #[serde(rename = "entityId")]
    pub entity_id: String,
    pub axis: String,
    #[serde(rename = "attemptedDelta")]
    pub attempted_delta: i64,
    #[serde(rename = "resolvedDelta")]
    pub resolved_delta: i64,
    pub normal: Physics2dVector,
    #[serde(rename = "blockedBy")]
    pub blocked_by: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Physics2dVector {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Rect {
    pub(super) entity_id: String,
    pub(super) body: PhysicsBody,
    pub(super) sensor: bool,
    pub(super) trigger: bool,
    pub(super) group: Option<String>,
    pub(super) mask: Vec<String>,
    pub(super) x: i64,
    pub(super) y: i64,
    pub(super) width: i64,
    pub(super) height: i64,
    pub(super) offset_x: i64,
    pub(super) offset_y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PhysicsBody {
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Axis {
    X,
    Y,
}

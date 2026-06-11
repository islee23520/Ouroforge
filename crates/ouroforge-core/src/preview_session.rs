//! Preview session contract for Live Preview Loop v1 (M131.1, Era X #2518).
//!
//! A preview session holds an in-memory scene document and validates
//! manipulation intents through the same code paths used by review-gated
//! scene apply (`apply_scene_edit` + `validate_scene`, the pair exercised by
//! `preview_scene_edit_transaction`). The session never writes to the
//! filesystem: deltas describe in-memory state only, and nothing reaches the
//! trusted worktree except via the existing review-gated CLI apply.

use crate::{
    apply_scene_edit, hash_scene_document, read_scene, require_text, supported_scene_edit_paths,
    validate_path_component, validate_scene, SceneDocument, SceneEdit, SceneHash,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::{Path, PathBuf};

pub const PREVIEW_INTENT_SCHEMA_VERSION: &str = "ouroforge.preview-intent.v1";
pub const PREVIEW_DELTA_SCHEMA_VERSION: &str = "ouroforge.preview-delta.v1";

/// A manipulation intent sent by an untrusted preview surface. Intents carry
/// no write authority; they are requests that the session validates and
/// normalizes through apply-path-identical code.
///
/// serde cannot combine `deny_unknown_fields` with the flattened payload
/// (every deserialization would fail), so the envelope tolerates unknown
/// fields; `schemaVersion` remains the strict compatibility gate.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PreviewIntent {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(flatten)]
    pub payload: PreviewIntentPayload,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "kind")]
pub enum PreviewIntentPayload {
    /// Set one allowlisted scene-edit path on one entity.
    #[serde(rename = "parameterSet")]
    ParameterSet {
        #[serde(rename = "entityId")]
        entity_id: String,
        path: String,
        value: serde_json::Value,
    },
    /// Move one entity; x/y map onto `components.transform.{x,y}` and are
    /// applied atomically (all-or-nothing).
    #[serde(rename = "entityTransform")]
    EntityTransform {
        #[serde(rename = "entityId")]
        entity_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        x: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        y: Option<i64>,
    },
    /// Discard in-memory state and re-read the base scene from disk.
    #[serde(rename = "sceneReload")]
    SceneReload {},
}

impl PreviewIntentPayload {
    pub fn kind(&self) -> &'static str {
        match self {
            PreviewIntentPayload::ParameterSet { .. } => "parameterSet",
            PreviewIntentPayload::EntityTransform { .. } => "entityTransform",
            PreviewIntentPayload::SceneReload {} => "sceneReload",
        }
    }
}

/// Fully normalized result of validating one intent against the session.
/// Consumers apply `edits` mechanically; no interpretation is left to them.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PreviewDelta {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "deltaId")]
    pub delta_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    pub sequence: u64,
    pub kind: String,
    pub status: PreviewDeltaStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edits: Vec<SceneEdit>,
    #[serde(rename = "beforeSceneHash")]
    pub before_scene_hash: SceneHash,
    #[serde(rename = "afterSceneHash", skip_serializing_if = "Option::is_none")]
    pub after_scene_hash: Option<SceneHash>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum PreviewDeltaStatus {
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "rejected")]
    Rejected,
}

/// In-memory preview session state. Holding this struct never implies any
/// filesystem write; `scene_path` is only read (at start and on reload).
#[derive(Debug, Clone)]
pub struct PreviewSession {
    pub session_id: String,
    pub scene_path: PathBuf,
    pub base_scene_hash: SceneHash,
    pub current_scene: SceneDocument,
    pub current_scene_hash: SceneHash,
    pub sequence: u64,
}

pub fn start_preview_session(
    scene_path: impl AsRef<Path>,
    session_id: &str,
) -> Result<PreviewSession> {
    require_text("preview session id", session_id)?;
    let scene_path = scene_path.as_ref();
    let scene = read_scene(scene_path)
        .with_context(|| format!("preview session failed to read {}", scene_path.display()))?;
    validate_scene(&scene).context("preview session base scene failed validation")?;
    let hash = hash_scene_document(&scene)?;
    Ok(PreviewSession {
        session_id: session_id.to_string(),
        scene_path: scene_path.to_path_buf(),
        base_scene_hash: hash.clone(),
        current_scene: scene,
        current_scene_hash: hash,
        sequence: 0,
    })
}

/// Normalize an intent into the scene edits it requests. This is pure
/// translation; allowlist and entity checks happen during application through
/// the shared apply-path code.
pub fn normalize_preview_intent(intent: &PreviewIntent) -> Result<Vec<SceneEdit>> {
    if intent.schema_version != PREVIEW_INTENT_SCHEMA_VERSION {
        return Err(anyhow!(
            "preview intent schemaVersion must be {PREVIEW_INTENT_SCHEMA_VERSION}"
        ));
    }
    require_text("preview intent id", &intent.intent_id)?;
    require_text("preview intent session id", &intent.session_id)?;
    match &intent.payload {
        PreviewIntentPayload::ParameterSet {
            entity_id,
            path,
            value,
        } => {
            validate_path_component("preview intent entity", entity_id)?;
            require_text("preview intent path", path)?;
            if !supported_scene_edit_paths().contains(&path.as_str()) {
                return Err(anyhow!(
                    "preview intent path is not allowed: {path}; supported paths are {}",
                    supported_scene_edit_paths().join(", ")
                ));
            }
            Ok(vec![SceneEdit {
                entity_id: entity_id.clone(),
                path: path.clone(),
                value: value.clone(),
            }])
        }
        PreviewIntentPayload::EntityTransform { entity_id, x, y } => {
            validate_path_component("preview intent entity", entity_id)?;
            if x.is_none() && y.is_none() {
                return Err(anyhow!(
                    "preview entityTransform intent requires at least one of x or y"
                ));
            }
            let mut edits = Vec::new();
            if let Some(x) = x {
                edits.push(SceneEdit {
                    entity_id: entity_id.clone(),
                    path: "components.transform.x".to_string(),
                    value: json!(x),
                });
            }
            if let Some(y) = y {
                edits.push(SceneEdit {
                    entity_id: entity_id.clone(),
                    path: "components.transform.y".to_string(),
                    value: json!(y),
                });
            }
            Ok(edits)
        }
        PreviewIntentPayload::SceneReload {} => Ok(Vec::new()),
    }
}

/// Validate one intent against the session and return the normalized delta.
///
/// Validation rejections (allowlist miss, unknown entity, type mismatch,
/// candidate scene invalid) come back as `status: rejected` deltas with
/// diagnostics; `Err` is reserved for malformed envelopes (schema/session
/// mismatch). Edits from one intent apply atomically: either every edit
/// passes and the session advances, or the session state is untouched.
pub fn apply_preview_intent(
    session: &mut PreviewSession,
    intent: &PreviewIntent,
) -> Result<PreviewDelta> {
    if intent.session_id != session.session_id {
        return Err(anyhow!(
            "preview intent session id {} does not match active session {}",
            intent.session_id,
            session.session_id
        ));
    }
    let before_hash = session.current_scene_hash.clone();
    let kind = intent.payload.kind().to_string();
    let edits = match normalize_preview_intent(intent) {
        Ok(edits) => edits,
        Err(error) => {
            return finish_delta(
                session,
                intent,
                kind,
                Vec::new(),
                before_hash,
                Err(error.to_string()),
            );
        }
    };
    if matches!(intent.payload, PreviewIntentPayload::SceneReload {}) {
        let outcome = (|| -> Result<SceneDocument> {
            let scene = read_scene(&session.scene_path).with_context(|| {
                format!(
                    "preview reload failed to read {}",
                    session.scene_path.display()
                )
            })?;
            validate_scene(&scene).context("preview reload scene failed validation")?;
            Ok(scene)
        })();
        return match outcome {
            Ok(scene) => {
                let after_hash = hash_scene_document(&scene)?;
                session.current_scene = scene;
                session.current_scene_hash = after_hash;
                finish_delta(session, intent, kind, Vec::new(), before_hash, Ok(()))
            }
            Err(error) => finish_delta(
                session,
                intent,
                kind,
                Vec::new(),
                before_hash,
                Err(error.to_string()),
            ),
        };
    }
    let mut candidate = session.current_scene.clone();
    let outcome = (|| -> Result<()> {
        for edit in &edits {
            apply_scene_edit(&mut candidate, edit.clone())?;
        }
        validate_scene(&candidate).context("preview candidate scene validation failed")?;
        Ok(())
    })();
    match outcome {
        Ok(()) => {
            let after_hash = hash_scene_document(&candidate)?;
            session.current_scene = candidate;
            session.current_scene_hash = after_hash;
            finish_delta(session, intent, kind, edits, before_hash, Ok(()))
        }
        Err(error) => finish_delta(
            session,
            intent,
            kind,
            edits,
            before_hash,
            Err(error.to_string()),
        ),
    }
}

fn finish_delta(
    session: &mut PreviewSession,
    intent: &PreviewIntent,
    kind: String,
    edits: Vec<SceneEdit>,
    before_hash: SceneHash,
    outcome: std::result::Result<(), String>,
) -> Result<PreviewDelta> {
    let (status, after_hash, errors) = match outcome {
        Ok(()) => {
            session.sequence += 1;
            (
                PreviewDeltaStatus::Applied,
                Some(session.current_scene_hash.clone()),
                Vec::new(),
            )
        }
        Err(error) => (PreviewDeltaStatus::Rejected, None, vec![error]),
    };
    let sequence = session.sequence;
    let seed = json!({
        "sessionId": session.session_id,
        "intentId": intent.intent_id,
        "sequence": sequence,
        "kind": kind,
        "status": status,
        "edits": edits,
        "beforeSceneHash": before_hash,
        "afterSceneHash": after_hash,
    });
    let delta_id = format!(
        "preview-delta-{}",
        crate::fnv1a64(&serde_json::to_vec(&crate::canonical_json_value(seed))?)
    );
    Ok(PreviewDelta {
        schema_version: PREVIEW_DELTA_SCHEMA_VERSION.to_string(),
        delta_id,
        session_id: session.session_id.clone(),
        intent_id: intent.intent_id.clone(),
        sequence,
        kind,
        status,
        edits,
        before_scene_hash: before_hash,
        after_scene_hash: after_hash,
        errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intent(payload: PreviewIntentPayload) -> PreviewIntent {
        PreviewIntent {
            schema_version: PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
            intent_id: "intent-1".to_string(),
            session_id: "session-1".to_string(),
            payload,
        }
    }

    #[test]
    fn normalize_rejects_unsupported_path() {
        let error = normalize_preview_intent(&intent(PreviewIntentPayload::ParameterSet {
            entity_id: "player".to_string(),
            path: "components.transform.rotation".to_string(),
            value: serde_json::json!(1),
        }))
        .unwrap_err();
        assert!(error.to_string().contains("not allowed"));
    }

    #[test]
    fn normalize_rejects_wrong_schema_version() {
        let mut bad = intent(PreviewIntentPayload::SceneReload {});
        bad.schema_version = "ouroforge.preview-intent.v0".to_string();
        assert!(normalize_preview_intent(&bad).is_err());
    }

    #[test]
    fn normalize_requires_transform_axis() {
        let error = normalize_preview_intent(&intent(PreviewIntentPayload::EntityTransform {
            entity_id: "player".to_string(),
            x: None,
            y: None,
        }))
        .unwrap_err();
        assert!(error.to_string().contains("at least one of x or y"));
    }

    #[test]
    fn normalize_transform_emits_allowlisted_edits() {
        let edits = normalize_preview_intent(&intent(PreviewIntentPayload::EntityTransform {
            entity_id: "player".to_string(),
            x: Some(12),
            y: Some(-4),
        }))
        .unwrap();
        assert_eq!(edits.len(), 2);
        assert_eq!(edits[0].path, "components.transform.x");
        assert_eq!(edits[1].path, "components.transform.y");
        for edit in &edits {
            assert!(supported_scene_edit_paths().contains(&edit.path.as_str()));
        }
    }

    #[test]
    fn intent_round_trips_through_serde() {
        let original = intent(PreviewIntentPayload::ParameterSet {
            entity_id: "player".to_string(),
            path: "components.input.moveSpeed".to_string(),
            value: serde_json::json!(7),
        });
        let encoded = serde_json::to_string(&original).unwrap();
        assert!(encoded.contains("\"kind\":\"parameterSet\""));
        let decoded: PreviewIntent = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}

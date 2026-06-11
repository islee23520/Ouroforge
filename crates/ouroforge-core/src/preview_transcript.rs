//! Preview transcript capture and draft-proposal conversion (M131.3, Era X
//! #2520).
//!
//! A transcript is the deterministic semantic record of one preview session:
//! every intent with its normalized delta, in order. It is what turns direct
//! manipulation into an evidence producer - the transcript replays through
//! the same validation code paths to reproduce the final preview state
//! byte-identically (fidelity guarantee), and converts into the EXISTING
//! visual-edit-draft artifact that enters the review-gated apply flow. No new
//! proposal type, no auto-apply: export produces a draft only.

use crate::preview_session::{
    apply_preview_intent, start_preview_session, PreviewDelta, PreviewDeltaStatus, PreviewIntent,
    PreviewSession,
};
use crate::{
    canonical_json_value, fnv1a64, supported_scene_edit_paths, SceneEdit, SceneHash,
    VisualEditDraftArtifact, VisualEditDraftAuthor, VisualEditDraftAuthorType,
    VisualEditDraftOperation, VisualEditDraftOperationKind, VisualEditDraftTarget,
    VisualEditDraftTargetType, VisualEditDraftValidationStatus, VisualEditSceneDraftOperation,
    VisualEditSceneDraftOperationKind, VISUAL_EDIT_DRAFT_SCHEMA_VERSION,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;

pub const PREVIEW_TRANSCRIPT_SCHEMA_VERSION: &str = "ouroforge.preview-transcript.v1";

/// One recorded interaction: the intent as received and the normalized delta
/// the session answered with. `relativeMs` is integer instrumentation timing
/// (milliseconds since session start) and is excluded from the semantic
/// digest so identical interaction histories digest identically.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PreviewTranscriptEntry {
    pub index: u64,
    #[serde(rename = "relativeMs")]
    pub relative_ms: u64,
    pub intent: PreviewIntent,
    pub delta: PreviewDelta,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PreviewTranscript {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "scenePath")]
    pub scene_path: String,
    #[serde(rename = "baseSceneHash")]
    pub base_scene_hash: SceneHash,
    #[serde(rename = "finalSceneHash")]
    pub final_scene_hash: SceneHash,
    #[serde(rename = "finalSequence")]
    pub final_sequence: u64,
    pub entries: Vec<PreviewTranscriptEntry>,
    #[serde(rename = "semanticDigest")]
    pub semantic_digest: String,
}

/// Accumulates entries beside a live session. The recorder holds memory only;
/// persistence is the local CLI client's job, never the serve process.
#[derive(Debug, Default)]
pub struct PreviewTranscriptRecorder {
    entries: Vec<PreviewTranscriptEntry>,
}

impl PreviewTranscriptRecorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, intent: &PreviewIntent, delta: &PreviewDelta, relative_ms: u64) {
        self.entries.push(PreviewTranscriptEntry {
            index: self.entries.len() as u64,
            relative_ms,
            intent: intent.clone(),
            delta: delta.clone(),
        });
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn finish(&self, session: &PreviewSession) -> Result<PreviewTranscript> {
        let semantic_digest = semantic_transcript_digest(
            &session.session_id,
            &session.scene_path.to_string_lossy(),
            &session.base_scene_hash,
            &session.current_scene_hash,
            &self.entries,
        )?;
        Ok(PreviewTranscript {
            schema_version: PREVIEW_TRANSCRIPT_SCHEMA_VERSION.to_string(),
            session_id: session.session_id.clone(),
            scene_path: session.scene_path.to_string_lossy().to_string(),
            base_scene_hash: session.base_scene_hash.clone(),
            final_scene_hash: session.current_scene_hash.clone(),
            final_sequence: session.sequence,
            entries: self.entries.clone(),
            semantic_digest,
        })
    }
}

fn semantic_transcript_digest(
    session_id: &str,
    scene_path: &str,
    base_hash: &SceneHash,
    final_hash: &SceneHash,
    entries: &[PreviewTranscriptEntry],
) -> Result<String> {
    // Timing-stripped canonical form: identical interaction histories must
    // digest identically regardless of wall-clock pacing.
    let semantic_entries = entries
        .iter()
        .map(|entry| {
            json!({
                "index": entry.index,
                "intent": entry.intent,
                "delta": entry.delta,
            })
        })
        .collect::<Vec<_>>();
    let seed = json!({
        "sessionId": session_id,
        "scenePath": scene_path,
        "baseSceneHash": base_hash,
        "finalSceneHash": final_hash,
        "entries": semantic_entries,
    });
    Ok(format!(
        "fnv1a64:{}",
        fnv1a64(&serde_json::to_vec(&canonical_json_value(seed))?)
    ))
}

/// Outcome of a fidelity replay: the reconstructed session plus the verified
/// transcript invariants.
#[derive(Debug)]
pub struct PreviewTranscriptReplay {
    pub session: PreviewSession,
    pub replayed_entries: u64,
}

/// Replay a transcript through the same validation code paths. Fails closed
/// on: schema mismatch, stale base scene (the scene on disk no longer matches
/// `baseSceneHash`), any delta that does not reproduce byte-identically, or a
/// final-state divergence. A passing replay IS the fidelity guarantee.
pub fn replay_preview_transcript(
    transcript: &PreviewTranscript,
) -> Result<PreviewTranscriptReplay> {
    if transcript.schema_version != PREVIEW_TRANSCRIPT_SCHEMA_VERSION {
        return Err(anyhow!(
            "preview transcript schemaVersion must be {PREVIEW_TRANSCRIPT_SCHEMA_VERSION}"
        ));
    }
    let mut session = start_preview_session(&transcript.scene_path, &transcript.session_id)?;
    if session.base_scene_hash != transcript.base_scene_hash {
        return Err(anyhow!(
            "preview transcript base scene is stale: transcript recorded {}, disk has {}",
            transcript.base_scene_hash.value,
            session.base_scene_hash.value
        ));
    }
    for entry in &transcript.entries {
        let replayed = apply_preview_intent(&mut session, &entry.intent).with_context(|| {
            format!("preview transcript entry {} failed to replay", entry.index)
        })?;
        if replayed != entry.delta {
            return Err(anyhow!(
                "preview transcript entry {} did not reproduce byte-identically: recorded delta {}, replayed delta {}",
                entry.index,
                entry.delta.delta_id,
                replayed.delta_id
            ));
        }
    }
    if session.current_scene_hash != transcript.final_scene_hash {
        return Err(anyhow!(
            "preview transcript final state diverged: transcript recorded {}, replay reached {}",
            transcript.final_scene_hash.value,
            session.current_scene_hash.value
        ));
    }
    if session.sequence != transcript.final_sequence {
        return Err(anyhow!(
            "preview transcript final sequence diverged: transcript recorded {}, replay reached {}",
            transcript.final_sequence,
            session.sequence
        ));
    }
    let expected_digest = semantic_transcript_digest(
        &transcript.session_id,
        &transcript.scene_path,
        &transcript.base_scene_hash,
        &transcript.final_scene_hash,
        &transcript.entries,
    )?;
    if expected_digest != transcript.semantic_digest {
        return Err(anyhow!(
            "preview transcript semantic digest mismatch: recorded {}, computed {}",
            transcript.semantic_digest,
            expected_digest
        ));
    }
    Ok(PreviewTranscriptReplay {
        session,
        replayed_entries: transcript.entries.len() as u64,
    })
}

/// Net effect of a transcript: the last applied value per (entityId, path).
/// Every allowlisted scene-edit path is a leaf scalar, so base-relative
/// application of the net values reproduces the final session state - the
/// same base-relative convention the existing draft preflight validates.
pub fn net_transcript_edits(transcript: &PreviewTranscript) -> Vec<SceneEdit> {
    let mut net: BTreeMap<(String, String), serde_json::Value> = BTreeMap::new();
    for entry in &transcript.entries {
        if entry.delta.status != PreviewDeltaStatus::Applied {
            continue;
        }
        for edit in &entry.delta.edits {
            net.insert(
                (edit.entity_id.clone(), edit.path.clone()),
                edit.value.clone(),
            );
        }
    }
    // BTreeMap ordering plus the allowlist order keeps output deterministic:
    // sort by allowlist position, then entity id.
    let path_order: BTreeMap<&str, usize> = supported_scene_edit_paths()
        .iter()
        .enumerate()
        .map(|(index, path)| (*path, index))
        .collect();
    let mut edits: Vec<SceneEdit> = net
        .into_iter()
        .map(|((entity_id, path), value)| SceneEdit {
            entity_id,
            path,
            value,
        })
        .collect();
    edits.sort_by(|left, right| {
        let left_key = (
            path_order
                .get(left.path.as_str())
                .copied()
                .unwrap_or(usize::MAX),
            left.entity_id.clone(),
        );
        let right_key = (
            path_order
                .get(right.path.as_str())
                .copied()
                .unwrap_or(usize::MAX),
            right.entity_id.clone(),
        );
        left_key.cmp(&right_key)
    });
    edits
}

fn draft_operation_kind_for_path(path: &str) -> VisualEditSceneDraftOperationKind {
    match path {
        "components.transform.x" | "components.transform.y" => {
            VisualEditSceneDraftOperationKind::TransformMove
        }
        "sprite.color" => VisualEditSceneDraftOperationKind::SpriteColorChange,
        "components.size.width" | "components.size.height" => {
            VisualEditSceneDraftOperationKind::ColliderSizeChange
        }
        "components.uiText.text" => VisualEditSceneDraftOperationKind::HudTextChange,
        "components.cameraTarget.weight" => {
            VisualEditSceneDraftOperationKind::CameraTargetSelection
        }
        // Velocity, controllable, status, and input edits are generic numeric
        // parameter changes; the precise semantics live in sceneEditPath.
        _ => VisualEditSceneDraftOperationKind::HudValueChange,
    }
}

/// Convert a fidelity-verified transcript into the existing visual-edit-draft
/// artifact. The export replays the transcript first (stale-base and
/// divergence fail closed), then emits one draft operation per net edit and
/// validates the artifact through the existing scene preflight so the draft
/// is guaranteed reviewable by the existing flow. The result is a DRAFT:
/// review and apply authority stay with the existing gated CLI flow.
/// `target_path` is the scene path as the project/review flow knows it
/// (repo-relative); the draft artifact schema rejects absolute paths. When
/// `None`, the transcript's recorded scene path is used verbatim - valid only
/// if the session was recorded with a relative path.
pub fn export_preview_draft(
    transcript: &PreviewTranscript,
    draft_id: &str,
    author_id: &str,
    target_path: Option<&str>,
) -> Result<VisualEditDraftArtifact> {
    let replay = replay_preview_transcript(transcript)?;
    let target_path = target_path.unwrap_or(&transcript.scene_path).to_string();
    let edits = net_transcript_edits(transcript);
    if edits.is_empty() {
        return Err(anyhow!(
            "preview transcript has no applied edits to export as a draft"
        ));
    }
    let operations = edits
        .iter()
        .enumerate()
        .map(|(index, edit)| VisualEditDraftOperation {
            id: format!("{draft_id}-op-{index}"),
            kind: VisualEditDraftOperationKind::Update,
            path: target_path.clone(),
            summary: Some(format!(
                "preview session {} net edit: {} {}",
                transcript.session_id, edit.entity_id, edit.path
            )),
            value: None,
            scene_operation: Some(VisualEditSceneDraftOperation {
                kind: draft_operation_kind_for_path(&edit.path),
                entity_id: edit.entity_id.clone(),
                scene_edit_path: edit.path.clone(),
                value: Some(edit.value.clone()),
                summary: None,
            }),
            tilemap_operation: None,
            asset_reference_operation: None,
        })
        .collect::<Vec<_>>();
    let artifact = VisualEditDraftArtifact {
        schema_version: VISUAL_EDIT_DRAFT_SCHEMA_VERSION.to_string(),
        draft_id: draft_id.to_string(),
        target: VisualEditDraftTarget {
            target_type: VisualEditDraftTargetType::Scene,
            path: target_path,
            id: None,
        },
        proposed_operations: operations,
        before_hash: format!(
            "{}:{}",
            transcript.base_scene_hash.algorithm, transcript.base_scene_hash.value
        ),
        review_gate: None,
        expected_after_summary: format!(
            "Applying the {} net preview edit(s) from session {} reproduces final scene hash {} (transcript {})",
            edits.len(),
            transcript.session_id,
            transcript.final_scene_hash.value,
            transcript.semantic_digest
        ),
        linked_evidence: vec![format!(
            "preview-transcript/{}/{}",
            transcript.session_id,
            transcript.semantic_digest.replace(':', "-")
        )],
        author: VisualEditDraftAuthor {
            author_type: VisualEditDraftAuthorType::Agent,
            id: author_id.to_string(),
            source: Some("ouroforge preview export-proposal".to_string()),
        },
        validation_status: VisualEditDraftValidationStatus::Unvalidated,
        blocked_reasons: Vec::new(),
    };
    // The exported draft must pass the existing preflight against the base
    // scene; this also re-verifies the base-relative net-edit theorem for
    // this concrete transcript.
    let preflight_edits =
        artifact.validate_scene_preflight(&replay.session.current_scene_base()?)?;
    if preflight_edits.len() != edits.len() {
        return Err(anyhow!(
            "exported draft preflight produced {} edits, expected {}",
            preflight_edits.len(),
            edits.len()
        ));
    }
    Ok(artifact)
}

impl PreviewSession {
    /// Re-read the base scene this session started from, for preflight
    /// validation of exported drafts. Fails closed if the file changed since
    /// session start.
    pub fn current_scene_base(&self) -> Result<crate::SceneDocument> {
        let scene = crate::read_scene(&self.scene_path)?;
        let hash = crate::hash_scene_document(&scene)?;
        if hash != self.base_scene_hash {
            return Err(anyhow!(
                "preview session base scene changed on disk: expected {}, found {}",
                self.base_scene_hash.value,
                hash.value
            ));
        }
        Ok(scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preview_session::PreviewIntentPayload;

    #[test]
    fn net_edits_keep_last_value_per_entity_path() {
        let transcript = PreviewTranscript {
            schema_version: PREVIEW_TRANSCRIPT_SCHEMA_VERSION.to_string(),
            session_id: "s".to_string(),
            scene_path: "scene.json".to_string(),
            base_scene_hash: SceneHash {
                algorithm: "fnv1a64".to_string(),
                value: "0".to_string(),
            },
            final_scene_hash: SceneHash {
                algorithm: "fnv1a64".to_string(),
                value: "1".to_string(),
            },
            final_sequence: 2,
            entries: vec![
                entry(0, "components.transform.x", json!(10)),
                entry(1, "components.transform.x", json!(20)),
            ],
            semantic_digest: "fnv1a64:0".to_string(),
        };
        let edits = net_transcript_edits(&transcript);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].value, json!(20));
    }

    fn entry(index: u64, path: &str, value: serde_json::Value) -> PreviewTranscriptEntry {
        PreviewTranscriptEntry {
            index,
            relative_ms: index * 10,
            intent: PreviewIntent {
                schema_version: crate::preview_session::PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
                intent_id: format!("i{index}"),
                session_id: "s".to_string(),
                payload: PreviewIntentPayload::ParameterSet {
                    entity_id: "player".to_string(),
                    path: path.to_string(),
                    value: value.clone(),
                },
            },
            delta: PreviewDelta {
                schema_version: crate::preview_session::PREVIEW_DELTA_SCHEMA_VERSION.to_string(),
                delta_id: format!("d{index}"),
                session_id: "s".to_string(),
                intent_id: format!("i{index}"),
                sequence: index + 1,
                kind: "parameterSet".to_string(),
                status: PreviewDeltaStatus::Applied,
                edits: vec![SceneEdit {
                    entity_id: "player".to_string(),
                    path: path.to_string(),
                    value,
                }],
                before_scene_hash: SceneHash {
                    algorithm: "fnv1a64".to_string(),
                    value: "0".to_string(),
                },
                after_scene_hash: Some(SceneHash {
                    algorithm: "fnv1a64".to_string(),
                    value: "1".to_string(),
                }),
                errors: Vec::new(),
            },
        }
    }
}

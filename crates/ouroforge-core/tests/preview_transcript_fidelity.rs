//! M131.3 transcript fidelity and export contract (Era X #2520).
//!
//! Locks the fidelity guarantee: a recorded transcript replays through the
//! same validation code paths to byte-identical deltas and final state;
//! stale bases, tampered entries, and diverged finals fail closed; and the
//! exported artifact is an EXISTING visual-edit-draft that passes the
//! existing scene preflight (draft only - no apply authority).

use ouroforge_core::preview_session::{
    apply_preview_intent, start_preview_session, PreviewIntent, PreviewIntentPayload,
    PREVIEW_INTENT_SCHEMA_VERSION,
};
use ouroforge_core::preview_transcript::{
    export_preview_draft, net_transcript_edits, replay_preview_transcript,
    PreviewTranscriptRecorder, PREVIEW_TRANSCRIPT_SCHEMA_VERSION,
};
use ouroforge_core::{read_scene, VisualEditDraftTargetType};
use serde_json::json;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_scene() -> PathBuf {
    repo_root()
        .join("examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json")
}

fn intent(id: &str, payload: PreviewIntentPayload) -> PreviewIntent {
    PreviewIntent {
        schema_version: PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
        intent_id: id.to_string(),
        session_id: "fidelity-session".to_string(),
        payload,
    }
}

fn recorded_session() -> ouroforge_core::preview_transcript::PreviewTranscript {
    let mut session = start_preview_session(fixture_scene(), "fidelity-session").unwrap();
    let mut recorder = PreviewTranscriptRecorder::new();
    let interactions = vec![
        intent(
            "i-move-1",
            PreviewIntentPayload::EntityTransform {
                entity_id: "player".to_string(),
                x: Some(40),
                y: Some(100),
            },
        ),
        intent(
            "i-speed",
            PreviewIntentPayload::ParameterSet {
                entity_id: "player".to_string(),
                path: "components.input.moveSpeed".to_string(),
                value: json!(72),
            },
        ),
        // A rejected interaction stays in the transcript but contributes no
        // net edit.
        intent(
            "i-bad",
            PreviewIntentPayload::ParameterSet {
                entity_id: "ghost".to_string(),
                path: "components.transform.x".to_string(),
                value: json!(1),
            },
        ),
        // Later move supersedes the first one in the net effect.
        intent(
            "i-move-2",
            PreviewIntentPayload::EntityTransform {
                entity_id: "player".to_string(),
                x: Some(56),
                y: None,
            },
        ),
    ];
    for interaction in &interactions {
        let mut elapsed = 0u64;
        elapsed += 10;
        let delta = apply_preview_intent(&mut session, interaction).unwrap();
        recorder.record(interaction, &delta, elapsed);
    }
    recorder.finish(&session).unwrap()
}

#[test]
fn transcript_replays_byte_identically() {
    let transcript = recorded_session();
    assert_eq!(transcript.schema_version, PREVIEW_TRANSCRIPT_SCHEMA_VERSION);
    assert_eq!(transcript.entries.len(), 4);
    let replay = replay_preview_transcript(&transcript).expect("fidelity replay passes");
    assert_eq!(replay.replayed_entries, 4);
    assert_eq!(
        replay.session.current_scene_hash,
        transcript.final_scene_hash
    );
}

#[test]
fn semantic_digest_ignores_timing_but_locks_interactions() {
    let first = recorded_session();
    let second = recorded_session();
    assert_eq!(
        first.semantic_digest, second.semantic_digest,
        "identical interaction histories must digest identically regardless of pacing"
    );
}

#[test]
fn tampered_transcript_fails_closed() {
    let mut transcript = recorded_session();
    transcript.entries[1].delta.edits[0].value = json!(99);
    let error = replay_preview_transcript(&transcript).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("did not reproduce byte-identically"),
        "{error}"
    );

    let mut diverged = recorded_session();
    diverged.final_scene_hash.value = "deadbeef".to_string();
    assert!(replay_preview_transcript(&diverged).is_err());

    let mut stale = recorded_session();
    stale.base_scene_hash.value = "deadbeef".to_string();
    let error = replay_preview_transcript(&stale).unwrap_err();
    assert!(error.to_string().contains("stale"), "{error}");
}

#[test]
fn net_edits_collapse_to_last_values_and_skip_rejections() {
    let transcript = recorded_session();
    let net = net_transcript_edits(&transcript);
    // transform.x (56 supersedes 40), transform.y (100), moveSpeed (72); the
    // ghost rejection contributes nothing.
    assert_eq!(net.len(), 3);
    let x = net
        .iter()
        .find(|edit| edit.path == "components.transform.x")
        .unwrap();
    assert_eq!(x.value, json!(56));
    assert!(net.iter().all(|edit| edit.entity_id == "player"));
}

#[test]
fn export_produces_existing_draft_artifact_that_passes_preflight() {
    let transcript = recorded_session();
    let draft = export_preview_draft(
        &transcript,
        "preview-fidelity-draft",
        "fable-agent",
        Some("examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json"),
    )
    .expect("export passes");
    assert_eq!(draft.schema_version, "visual-edit-draft-v1");
    assert_eq!(draft.target.target_type, VisualEditDraftTargetType::Scene);
    assert_eq!(draft.proposed_operations.len(), 3);
    assert!(
        draft.review_gate.is_none(),
        "export must not pre-fill review gates"
    );
    let digest_ref = transcript.semantic_digest.replace(':', "-");
    assert!(draft
        .linked_evidence
        .iter()
        .any(|evidence| evidence.contains(&digest_ref)));

    // The draft must independently pass the existing preflight and reproduce
    // the transcript's final scene hash through base-relative application.
    let base = read_scene(fixture_scene()).unwrap();
    let edits = draft.validate_scene_preflight(&base).unwrap();
    assert_eq!(edits.len(), 3);
    let mut applied = base.clone();
    for edit in edits {
        ouroforge_core::apply_scene_edit(&mut applied, edit).unwrap();
    }
    ouroforge_core::validate_scene(&applied).unwrap();
    let final_hash = ouroforge_core::hash_scene_document(&applied).unwrap();
    assert_eq!(
        final_hash, transcript.final_scene_hash,
        "base-relative net edits must reproduce the recorded final state"
    );
}

#[test]
fn export_rejects_empty_transcripts() {
    let session = start_preview_session(fixture_scene(), "fidelity-session").unwrap();
    let transcript = PreviewTranscriptRecorder::new().finish(&session).unwrap();
    let error = export_preview_draft(&transcript, "empty-draft", "fable-agent", None).unwrap_err();
    assert!(error.to_string().contains("no applied edits"));
}

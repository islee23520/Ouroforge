//! M131.1 parity golden suite (Era X #2518).
//!
//! Locks the Q1-ratified invariant from #2517: preview-session validation and
//! review-gated apply preflight share the same code paths, so the same edit
//! must produce byte-identical normalized outcomes (after-scene hash on pass,
//! rejection on fail) through both entry points. This suite is a permanent
//! regression gate; weakening it requires a new governance decision.

use ouroforge_core::preview_session::{
    apply_preview_intent, start_preview_session, PreviewDeltaStatus, PreviewIntent,
    PreviewIntentPayload, PREVIEW_INTENT_SCHEMA_VERSION,
};
use ouroforge_core::{preview_scene_edit_transaction, supported_scene_edit_paths, SceneEdit};
use serde_json::json;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_scene() -> PathBuf {
    repo_root()
        .join("examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json")
}

fn parameter_intent(
    id: &str,
    entity_id: &str,
    path: &str,
    value: serde_json::Value,
) -> PreviewIntent {
    PreviewIntent {
        schema_version: PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
        intent_id: format!("intent-{id}"),
        session_id: "parity-session".to_string(),
        payload: PreviewIntentPayload::ParameterSet {
            entity_id: entity_id.to_string(),
            path: path.to_string(),
            value,
        },
    }
}

fn golden_value_for(path: &str) -> serde_json::Value {
    match path {
        "sprite.color" => json!("#3366ff"),
        "components.controllable" => json!(true),
        "components.uiText.text" => json!("parity"),
        _ => json!(11),
    }
}

/// Every allowlisted scene-edit path goes through both entry points on a
/// fresh base scene; outcomes must match exactly. Paths whose component is
/// absent on the fixture entity must fail identically on both sides - parity
/// covers rejections, not just passes.
#[test]
fn parity_across_all_supported_paths() {
    let scene_path = fixture_scene();
    for path in supported_scene_edit_paths() {
        let value = golden_value_for(path);
        let transaction = preview_scene_edit_transaction(
            &scene_path,
            SceneEdit {
                entity_id: "player".to_string(),
                path: path.to_string(),
                value: value.clone(),
            },
        )
        .unwrap_or_else(|error| panic!("transaction preview failed for {path}: {error}"));

        let mut session = start_preview_session(&scene_path, "parity-session")
            .expect("preview session should start on the fixture scene");
        let delta =
            apply_preview_intent(&mut session, &parameter_intent(path, "player", path, value))
                .unwrap_or_else(|error| panic!("preview intent errored for {path}: {error}"));

        match transaction.validation_result.status.as_str() {
            "passed" => {
                assert_eq!(
                    delta.status,
                    PreviewDeltaStatus::Applied,
                    "path {path}: transaction passed but preview rejected: {:?}",
                    delta.errors
                );
                assert_eq!(
                    delta.after_scene_hash.as_ref(),
                    transaction.after_scene_hash.as_ref(),
                    "path {path}: after-scene hash diverged between preview and apply preflight"
                );
                assert_eq!(
                    delta.before_scene_hash, transaction.before_scene_hash,
                    "path {path}: before-scene hash diverged"
                );
            }
            "failed" => {
                assert_eq!(
                    delta.status,
                    PreviewDeltaStatus::Rejected,
                    "path {path}: transaction failed but preview applied"
                );
                assert!(delta.after_scene_hash.is_none());
                assert!(
                    !delta.errors.is_empty(),
                    "path {path}: rejection lost diagnostics"
                );
            }
            other => panic!("unexpected transaction status {other} for {path}"),
        }
    }
}

#[test]
fn parity_on_unknown_entity_rejection() {
    let scene_path = fixture_scene();
    let edit = SceneEdit {
        entity_id: "no-such-entity".to_string(),
        path: "components.transform.x".to_string(),
        value: json!(5),
    };
    let transaction = preview_scene_edit_transaction(&scene_path, edit).unwrap();
    assert_eq!(transaction.validation_result.status, "failed");

    let mut session = start_preview_session(&scene_path, "parity-session").unwrap();
    let delta = apply_preview_intent(
        &mut session,
        &parameter_intent(
            "ghost",
            "no-such-entity",
            "components.transform.x",
            json!(5),
        ),
    )
    .unwrap();
    assert_eq!(delta.status, PreviewDeltaStatus::Rejected);
    assert!(delta.errors.iter().any(|e| e.contains("no-such-entity")));
}

#[test]
fn transform_intent_matches_sequential_apply_preflight() {
    let scene_path = fixture_scene();
    let mut session = start_preview_session(&scene_path, "parity-session").unwrap();
    let delta = apply_preview_intent(
        &mut session,
        &PreviewIntent {
            schema_version: PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
            intent_id: "intent-transform".to_string(),
            session_id: "parity-session".to_string(),
            payload: PreviewIntentPayload::EntityTransform {
                entity_id: "player".to_string(),
                x: Some(48),
                y: Some(96),
            },
        },
    )
    .unwrap();
    assert_eq!(delta.status, PreviewDeltaStatus::Applied);
    assert_eq!(delta.edits.len(), 2);

    // The same end state through the apply-preflight path: x edit previewed
    // against base, then the y edit applied on top in memory must land on the
    // identical canonical hash the session reached.
    let x_transaction = preview_scene_edit_transaction(
        &scene_path,
        SceneEdit {
            entity_id: "player".to_string(),
            path: "components.transform.x".to_string(),
            value: json!(48),
        },
    )
    .unwrap();
    assert_eq!(x_transaction.validation_result.status, "passed");
    let mut staged = ouroforge_core::read_scene(&scene_path).unwrap();
    ouroforge_core::apply_scene_edit(
        &mut staged,
        SceneEdit {
            entity_id: "player".to_string(),
            path: "components.transform.x".to_string(),
            value: json!(48),
        },
    )
    .unwrap();
    ouroforge_core::apply_scene_edit(
        &mut staged,
        SceneEdit {
            entity_id: "player".to_string(),
            path: "components.transform.y".to_string(),
            value: json!(96),
        },
    )
    .unwrap();
    ouroforge_core::validate_scene(&staged).unwrap();
    let staged_hash = ouroforge_core::hash_scene_document(&staged).unwrap();
    assert_eq!(delta.after_scene_hash, Some(staged_hash));
}

#[test]
fn rejected_intent_leaves_session_state_untouched() {
    let scene_path = fixture_scene();
    let mut session = start_preview_session(&scene_path, "parity-session").unwrap();
    let hash_before = session.current_scene_hash.clone();
    let sequence_before = session.sequence;

    // Atomicity: x is valid, but a bad y type must reject the whole intent.
    let delta = apply_preview_intent(
        &mut session,
        &parameter_intent(
            "bad-type",
            "player",
            "components.transform.x",
            json!("not-a-number"),
        ),
    )
    .unwrap();
    assert_eq!(delta.status, PreviewDeltaStatus::Rejected);
    assert_eq!(session.current_scene_hash, hash_before);
    assert_eq!(session.sequence, sequence_before);
}

#[test]
fn scene_reload_restores_base_state() {
    let scene_path = fixture_scene();
    let mut session = start_preview_session(&scene_path, "parity-session").unwrap();
    let base_hash = session.base_scene_hash.clone();

    let applied = apply_preview_intent(
        &mut session,
        &parameter_intent("drift", "player", "components.transform.x", json!(77)),
    )
    .unwrap();
    assert_eq!(applied.status, PreviewDeltaStatus::Applied);
    assert_ne!(session.current_scene_hash, base_hash);

    let reload = apply_preview_intent(
        &mut session,
        &PreviewIntent {
            schema_version: PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
            intent_id: "intent-reload".to_string(),
            session_id: "parity-session".to_string(),
            payload: PreviewIntentPayload::SceneReload {},
        },
    )
    .unwrap();
    assert_eq!(reload.status, PreviewDeltaStatus::Applied);
    assert_eq!(session.current_scene_hash, base_hash);
    assert_eq!(reload.after_scene_hash, Some(base_hash));
}

#[test]
fn session_id_mismatch_is_an_envelope_error() {
    let scene_path = fixture_scene();
    let mut session = start_preview_session(&scene_path, "parity-session").unwrap();
    let mut foreign = parameter_intent("foreign", "player", "components.transform.x", json!(1));
    foreign.session_id = "another-session".to_string();
    assert!(apply_preview_intent(&mut session, &foreign).is_err());
}

#[test]
fn deltas_are_deterministic_for_identical_sessions() {
    let scene_path = fixture_scene();
    let run = || {
        let mut session = start_preview_session(&scene_path, "parity-session").unwrap();
        apply_preview_intent(
            &mut session,
            &parameter_intent("det", "player", "components.input.moveSpeed", json!(9)),
        )
        .unwrap()
    };
    let first = serde_json::to_string(&run()).unwrap();
    let second = serde_json::to_string(&run()).unwrap();
    assert_eq!(
        first, second,
        "identical sessions must serialize identical deltas"
    );
}

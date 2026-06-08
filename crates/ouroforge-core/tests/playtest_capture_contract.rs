//! Contract test for Structured Human-Playtest Capture v1 (#1858).
//!
//! Playtest capture records structured human session signals and feedback as
//! evidence. It is not a fun verdict, release approval, trusted write, or new
//! engine.

use std::path::PathBuf;

use ouroforge_core::playtest_capture::{
    PlaytestSessionCapture, PLAYTEST_CAPTURE_BOUNDARY, PLAYTEST_CAPTURE_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/playtest-funfeel-gate-v1")
            .join(name),
    )
    .unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn fixture(name: &str) -> PlaytestSessionCapture {
    PlaytestSessionCapture::from_json_str(&fixture_text(name)).expect("fixture parses")
}

#[test]
fn session_capture_shape_is_structured_evidence_only() {
    let capture = fixture("playtest-session-v1.json");
    capture.validate().expect("capture validates");

    assert_eq!(capture.schema_version, PLAYTEST_CAPTURE_SCHEMA_VERSION);
    assert_eq!(capture.boundary, PLAYTEST_CAPTURE_BOUNDARY);
    assert_eq!(capture.project_id, "signal-gate");
    assert_eq!(capture.signals.run_count, 3);
    assert_eq!(capture.signals.one_more_run, "yes");
    assert_eq!(
        capture.signals.retention_proxy,
        "returned-within-local-window"
    );
    assert!(capture.actor.human_confirmed);
    assert!(!capture.trusted_write_requested);
    assert!(!capture.release_authority);
    assert!(capture.generated_state_policy.contains("untracked"));
}

#[test]
fn feedback_recording_preserves_qualitative_signals_without_fun_verdict() {
    let capture = fixture("playtest-session-feedback-v1.json");
    capture.validate().expect("feedback capture validates");

    assert_eq!(capture.feedback.severity, "low");
    assert_eq!(capture.feedback.liked_moments, vec!["fast restart"]);
    assert_eq!(capture.feedback.disliked_moments.len(), 0);
    assert_eq!(capture.feedback.friction_tags, vec!["needs-more-context"]);
    assert!(capture.feedback.notes.contains("no human verdict"));
    assert!(capture.feedback.suggested_follow_up.is_none());
    assert_eq!(capture.evidence_refs[0].kind, "playtest-log");
}

#[test]
fn malformed_session_is_rejected_fail_closed() {
    let capture = fixture("playtest-session-malformed-v1.json");
    let error = capture
        .validate()
        .expect_err("malformed capture must fail closed");
    assert!(
        error.to_string().contains("candidateRefs")
            || error.to_string().contains("humanConfirmed")
            || error.to_string().contains("evidence only"),
        "unexpected error: {error}"
    );
}

#[test]
fn unknown_automated_fun_score_field_is_rejected_by_schema() {
    let mut value: serde_json::Value =
        serde_json::from_str(&fixture_text("playtest-session-v1.json")).expect("json parses");
    value["automatedFunScore"] = serde_json::json!(0.97);
    let text = serde_json::to_string(&value).expect("json serializes");
    let error = PlaytestSessionCapture::from_json_str(&text)
        .expect_err("unknown automated fun score field must be rejected");
    assert!(error.to_string().contains("automatedFunScore"));
}

#[test]
fn scope_doc_keeps_capture_human_owned_and_read_only() {
    let doc = std::fs::read_to_string(repo_root().join("docs/playtest-funfeel-gate-v1.md"))
        .expect("scope doc exists");
    assert!(doc.contains("Structured human-playtest capture contract"));
    assert!(doc.contains("The capture is evidence only"));
    assert!(doc.contains("fun/feel verdict has one stable rule"));
    assert!(doc.contains("Browser, dashboard, and Studio surfaces"));
    assert!(doc.contains("read-only"));
}

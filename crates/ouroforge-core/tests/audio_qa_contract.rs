//! Contract test for Audio-QA Check v1 (#1643).
//!
//! Part of Audio Generation and Audio-QA v1 (#1641) under #1 Era G Milestone 37.
//! These tests machine-check the audio-QA engine room: a well-formed candidate
//! passes only when format, loudness, license/provenance, and regression all hold;
//! invalid loudness, missing provenance, and a baseline regression fail; and a
//! malformed (unsupported-format) artifact fails closed. The check composes into
//! the existing evaluator aggregation via a declared `declared-gate-and` verdict;
//! it never auto-applies and makes no quality/taste judgement.

use std::path::PathBuf;

use ouroforge_core::audio_qa::{AudioQaArtifact, AudioQaStatus, AUDIO_QA_SCHEMA_VERSION};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text(name: &str) -> String {
    let path: PathBuf = repo_root().join("examples/audio-qa-v1").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn read_artifact(name: &str) -> AudioQaArtifact {
    AudioQaArtifact::from_json_str(&fixture_text(name)).expect("fixture artifact parses")
}

#[test]
fn passing_audio_passes_all_checks() {
    let artifact = read_artifact("audio-qa-pass.fixture.json");
    assert_eq!(artifact.schema_version, AUDIO_QA_SCHEMA_VERSION);
    assert_eq!(artifact.computed_status(), AudioQaStatus::Pass);
    assert!(artifact.failures().is_empty());
}

#[test]
fn passing_audio_emits_a_passing_declared_gate_verdict() {
    let artifact = read_artifact("audio-qa-pass.fixture.json");
    let verdict = artifact.gate_verdict();
    assert_eq!(verdict["gate"], "audio_qa");
    assert_eq!(verdict["declared"], true);
    assert_eq!(verdict["status"], "pass");
    assert_eq!(verdict["failureCount"], 0);
    // Composes into the existing evaluator aggregation operator.
    assert_eq!(verdict["aggregation"]["operator"], "declared-gate-and");
    assert_eq!(verdict["aggregation"]["undeclaredGatePolicy"], "neutral");
}

#[test]
fn loudness_invalid_fails() {
    let artifact = read_artifact("audio-qa-loudness-fail.fixture.json");
    assert_eq!(artifact.computed_status(), AudioQaStatus::Fail);
    assert!(artifact
        .failures()
        .iter()
        .any(|f| f.code == "loudness_out_of_range"));

    let verdict = artifact.gate_verdict();
    assert_eq!(verdict["status"], "fail");
    assert_eq!(verdict["failureCount"], 1);
}

#[test]
fn stale_baseline_is_preserved_in_the_gate_verdict() {
    // A stale baseline must reach the aggregation boundary as a stale gate (not
    // collapsed to "fail"), while still contributing a failure (fail-closed).
    let mut artifact = read_artifact("audio-qa-pass.fixture.json");
    artifact.stale_baseline_refs = artifact.baseline_refs.clone();
    artifact.status = "stale".to_string();
    artifact.blocked_reasons = vec!["baseline evidence is stale".to_string()];
    artifact
        .validate()
        .expect("declared stale matches computed stale");
    assert_eq!(artifact.computed_status(), AudioQaStatus::Stale);

    let verdict = artifact.gate_verdict();
    assert_eq!(verdict["status"], "stale");
    assert_eq!(verdict["pass"], false);
    assert_eq!(verdict["failureCount"], 1);
    assert_eq!(verdict["aggregation"]["operator"], "declared-gate-and");
}

#[test]
fn missing_provenance_fails() {
    let artifact = read_artifact("audio-qa-provenance-fail.fixture.json");
    assert_eq!(artifact.computed_status(), AudioQaStatus::Fail);
    assert!(artifact
        .failures()
        .iter()
        .any(|f| f.code == "missing_provenance"));
}

#[test]
fn malformed_audio_qa_is_rejected_fail_closed() {
    let error = AudioQaArtifact::from_json_str(&fixture_text("audio-qa-malformed.fixture.json"))
        .expect_err("an unsupported audio format must be rejected fail-closed");
    assert!(
        error.to_string().contains("format \"flac\" is unsupported"),
        "unexpected error: {error}"
    );
}

#[test]
fn loudness_regression_vs_baseline_fails() {
    // Start from the passing artifact and drift the candidate loudness far from
    // the baseline so the regression check fails.
    let mut artifact = read_artifact("audio-qa-pass.fixture.json");
    artifact.loudness.integrated_lufs = -20.0; // baseline is -16.4, drift 3.6 > 2.0
    artifact.status = "fail".to_string();
    artifact
        .validate()
        .expect("declared fail matches computed fail");
    assert_eq!(artifact.computed_status(), AudioQaStatus::Fail);
    assert!(artifact
        .failures()
        .iter()
        .any(|f| f.code == "loudness_regression"));
}

#[test]
fn declared_status_must_match_computed_classification() {
    // A passing-shaped artifact that declares "fail" must be rejected fail-closed.
    let mut artifact = read_artifact("audio-qa-pass.fixture.json");
    artifact.status = "fail".to_string();
    let error = artifact
        .validate()
        .expect_err("a mismatched declared status must be rejected");
    assert!(
        error.to_string().contains("does not match computed"),
        "unexpected error: {error}"
    );
}

#[test]
fn blank_license_is_rejected_fail_closed() {
    let mut artifact = read_artifact("audio-qa-pass.fixture.json");
    artifact.license.holder = "   ".to_string();
    let error = artifact
        .validate()
        .expect_err("a blank license credit must be rejected fail-closed");
    assert!(
        error
            .to_string()
            .contains("audio license holder is required"),
        "unexpected error: {error}"
    );
}

#[test]
fn blank_provenance_reference_is_rejected_fail_closed() {
    // A non-empty provenanceRefs list whose entries are blank must not pass the
    // mandatory provenance gate: it is malformed and fails closed.
    let mut artifact = read_artifact("audio-qa-pass.fixture.json");
    artifact.provenance_refs = vec!["".to_string()];
    let error = artifact
        .validate()
        .expect_err("a blank provenance reference must be rejected fail-closed");
    assert!(
        error.to_string().contains("provenanceRefs[0] is required"),
        "unexpected error: {error}"
    );
}

#[test]
fn docs_record_the_audio_qa_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/audio-pipeline-v1.md"))
        .expect("audio pipeline doc exists");
    assert!(
        doc.contains("#1643"),
        "audio pipeline doc records the audio-QA follow-up (#1643)"
    );
    assert!(
        doc.contains("audio-QA contract"),
        "doc records the audio-QA contract"
    );
}

//! Score-Cascade Payoff Feedback v1 (#1820) contract tests.
//!
//! Locks deterministic, resolution-ordered payoff feedback over the existing
//! card-roguelite substrate scoring path. The feedback trace is Rust/local
//! evidence only: it never becomes browser score authority, a trusted write path,
//! or an automated fun/quality verdict.

use std::fs;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    resolve_card_roguelite_state, score_cascade_feedback_trace, CardRogueliteConfig,
    SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION, SCORE_CASCADE_FEEDBACK_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .to_path_buf()
}

fn read(path: &str) -> String {
    let full = repo_root().join(path);
    fs::read_to_string(&full)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", full.display()))
}

fn fixture() -> Value {
    serde_json::from_str(&read(
        "examples/game-runtime/score-cascade-feedback-v1.json",
    ))
    .expect("score cascade fixture parses")
}

fn fixture_config() -> CardRogueliteConfig {
    serde_json::from_value(fixture()["config"].clone()).expect("fixture config parses")
}

#[test]
fn cascade_events_follow_authoritative_resolution_order() {
    let fixture = fixture();
    let config = fixture_config();
    let authoritative =
        resolve_card_roguelite_state(&config).expect("authoritative substrate resolves");
    let trace = score_cascade_feedback_trace(&config).expect("score cascade trace resolves");

    assert_eq!(trace.schema_version, SCORE_CASCADE_FEEDBACK_SCHEMA_VERSION);
    assert_eq!(trace.final_score, authoritative.score);
    assert_eq!(trace.authoritative_score, authoritative.score);
    assert_eq!(
        trace.final_score,
        fixture["expected"]["finalScore"].as_i64().unwrap() as i32
    );
    assert_eq!(
        trace.read_only_inspection.trusted_emitter,
        "rust-score-cascade-feedback"
    );
    assert!(trace
        .boundary
        .contains("authoritative score remains the existing Rust/local substrate resolution"));

    let phases: Vec<_> = trace
        .events
        .iter()
        .map(|event| event.phase.as_str())
        .collect();
    let expected_phases: Vec<_> = fixture["expected"]["phases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|phase| phase.as_str().unwrap())
        .collect();
    assert_eq!(phases, expected_phases);

    let modifier_order: Vec<_> = trace
        .events
        .iter()
        .filter_map(|event| event.modifier_id.as_deref())
        .collect();
    let expected_modifiers: Vec<_> = fixture["expected"]["modifierOrder"]
        .as_array()
        .unwrap()
        .iter()
        .map(|modifier| modifier.as_str().unwrap())
        .collect();
    assert_eq!(modifier_order, expected_modifiers);

    for (index, event) in trace.events.iter().enumerate() {
        assert_eq!(
            event.schema_version,
            SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION
        );
        assert_eq!(event.step_index, index);
        assert_eq!(event.event_id, format!("cascade-{:04}", index + 1));
        assert_eq!(
            event.juice_trigger,
            fixture["expected"]["juiceTrigger"].as_str().unwrap()
        );
        assert!(event.read_only_evidence);
        assert!(event.boundary.contains("not score authority"));
    }

    let after_values: Vec<_> = trace.events.iter().map(|event| event.after).collect();
    assert_eq!(after_values, [5, 7, 14, 14, 5, 10, 24, 24]);
}

#[test]
fn cascade_feedback_is_deterministic_and_additive() {
    let config = fixture_config();
    let first = score_cascade_feedback_trace(&config).expect("first trace");
    let second = score_cascade_feedback_trace(&config).expect("second trace");
    assert_eq!(first, second);
    assert_eq!(first.events.last().unwrap().phase, "cascade-complete");
    assert!(first
        .generated_state_policy
        .contains("untracked unless fixture-scoped"));
    assert!(first
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "trusted writes"));
    assert!(first
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "automated fun verdict"));
}

#[test]
fn fixture_and_public_wording_preserve_governance_boundaries() {
    let fixture_text = read("examples/game-runtime/score-cascade-feedback-v1.json");
    let module_text = read("crates/ouroforge-core/src/score_cascade_feedback.rs");
    let combined = format!("{fixture_text}\n{module_text}");
    for required in [
        "fixture-scoped",
        "not a fun/quality verdict",
        "browser/Studio surfaces are read-only",
        "not a new engine",
        "trusted write",
        "automated fun verdict",
    ] {
        assert!(
            combined.contains(required),
            "missing governance wording: {required}"
        );
    }
}

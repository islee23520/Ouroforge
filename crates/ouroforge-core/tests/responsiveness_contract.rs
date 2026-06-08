//! Sub-100ms Responsiveness Verification v1 (#1821) contract tests.

use std::fs;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    verify_responsiveness, ResponsivenessEvidence, ResponsivenessStatus,
    DEFAULT_RESPONSIVENESS_BUDGET_MS, RESPONSIVENESS_EVENT_SCHEMA_VERSION,
    RESPONSIVENESS_REPORT_SCHEMA_VERSION,
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
    serde_json::from_str(&read("examples/game-runtime/responsiveness-v1.json"))
        .expect("responsiveness fixture parses")
}

fn cases() -> Vec<(ResponsivenessEvidence, String, u32)> {
    fixture()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            (
                serde_json::from_value(case.clone()).expect("case parses"),
                case["expectedStatus"].as_str().unwrap().to_string(),
                case["expectedMaxLatencyMs"].as_u64().unwrap() as u32,
            )
        })
        .collect()
}

#[test]
fn responsiveness_passes_within_budget_and_fails_over_budget() {
    assert_eq!(DEFAULT_RESPONSIVENESS_BUDGET_MS, 100);
    let results = cases()
        .into_iter()
        .map(|(evidence, expected_status, expected_latency)| {
            assert_eq!(evidence.schema_version, RESPONSIVENESS_EVENT_SCHEMA_VERSION);
            let report = verify_responsiveness(&evidence).expect("responsiveness report");
            assert_eq!(report.schema_version, RESPONSIVENESS_REPORT_SCHEMA_VERSION);
            assert_eq!(report.max_latency_ms, expected_latency);
            assert_eq!(
                match report.status {
                    ResponsivenessStatus::Pass => "pass",
                    ResponsivenessStatus::Fail => "fail",
                },
                expected_status
            );
            assert!(report.boundary.contains("Rust/local owns verification"));
            assert!(report
                .read_only_inspection
                .disallowed_actions
                .contains(&"new runtime".to_string()));
            report
        })
        .collect::<Vec<_>>();

    assert!(matches!(results[0].status, ResponsivenessStatus::Pass));
    assert!(matches!(results[1].status, ResponsivenessStatus::Fail));
}

#[test]
fn responsiveness_measurement_is_deterministic() {
    for (evidence, _status, _latency) in cases() {
        let first = verify_responsiveness(&evidence).expect("first report");
        let second = verify_responsiveness(&evidence).expect("second report");
        assert_eq!(first, second);
        for measurement in &first.measurements {
            assert_eq!(
                measurement.latency_ms,
                (measurement.feedback_tick - measurement.input_tick) * evidence.fixed_delta_ms
            );
        }
    }
}

#[test]
fn missing_or_reversed_feedback_fails_closed() {
    let (mut missing, _status, _latency) = cases().remove(0);
    missing.events.retain(|event| event.feedback_id.is_none());
    assert!(verify_responsiveness(&missing)
        .unwrap_err()
        .to_string()
        .contains("no matching feedback"));

    let (mut reversed, _status, _latency) = cases().remove(0);
    reversed.events[1].tick = reversed.events[0].tick - 1;
    assert!(verify_responsiveness(&reversed)
        .unwrap_err()
        .to_string()
        .contains("occurs before input"));
}

#[test]
fn fixture_and_public_wording_preserve_governance_boundaries() {
    let fixture_text = read("examples/game-runtime/responsiveness-v1.json");
    let module_text = read("crates/ouroforge-core/src/responsiveness.rs");
    let combined = format!("{fixture_text}\n{module_text}");
    for required in [
        "fixture-scoped",
        "not a fun/quality verdict",
        "not a new runtime",
        "trusted writes",
        "browser/probe observations are read-only",
        "automated fun verdict",
    ] {
        assert!(
            combined.contains(required),
            "missing governance wording: {required}"
        );
    }
}

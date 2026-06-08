//! Game-Feel and Juice Demo v1 (#1822) smoke contract.
//!
//! The demo composes existing fixture-scoped juice primitives, score-cascade
//! feedback, and responsiveness evidence. It asserts mechanical ordering and
//! timing only; feel/fun judgment remains human-gated.

use std::fs;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    score_cascade_feedback_trace, verify_responsiveness, CardRogueliteConfig,
    ResponsivenessEvidence, ResponsivenessStatus,
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

fn json(path: &str) -> Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|err| panic!("{path} parses: {err}"))
}

fn demo() -> Value {
    json("examples/game-feel-juice-v1/demo/game-feel-juice-demo-v1.json")
}

#[test]
fn demo_recomputes_ordered_cascade_feedback_and_responsiveness_verdicts() {
    let demo = demo();
    assert_eq!(demo["schemaVersion"], "ouroforge.game-feel-juice-demo.v1");
    assert_eq!(demo["issue"], 1822);
    assert_eq!(demo["expected"]["budgetMs"], 100);

    let cascade_fixture = json("examples/game-runtime/score-cascade-feedback-v1.json");
    let config: CardRogueliteConfig = serde_json::from_value(cascade_fixture["config"].clone())
        .expect("cascade fixture config parses");
    let trace = score_cascade_feedback_trace(&config).expect("cascade trace recomputes");

    let expected_ids: Vec<_> = demo["expected"]["cascadeEventIds"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    let actual_ids: Vec<_> = trace
        .events
        .iter()
        .map(|event| event.event_id.as_str())
        .collect();
    assert_eq!(actual_ids, expected_ids);

    let expected_phases: Vec<_> = demo["expected"]["cascadePhases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    let actual_phases: Vec<_> = trace
        .events
        .iter()
        .map(|event| event.phase.as_str())
        .collect();
    assert_eq!(actual_phases, expected_phases);
    assert_eq!(
        trace.final_score,
        demo["expected"]["finalScore"].as_i64().unwrap() as i32
    );
    assert!(trace
        .events
        .iter()
        .all(|event| event.juice_trigger == demo["expected"]["juiceTrigger"].as_str().unwrap()));

    let responsiveness_fixture = json("examples/game-runtime/responsiveness-v1.json");
    let statuses = responsiveness_fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let evidence: ResponsivenessEvidence =
                serde_json::from_value(case.clone()).expect("responsiveness case parses");
            let report = verify_responsiveness(&evidence).expect("responsiveness report");
            (
                match report.status {
                    ResponsivenessStatus::Pass => "pass",
                    ResponsivenessStatus::Fail => "fail",
                },
                report.max_latency_ms,
            )
        })
        .collect::<Vec<_>>();

    let expected_statuses: Vec<_> = demo["expected"]["responsivenessStatuses"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    let expected_latencies: Vec<_> = demo["expected"]["responsivenessMaxLatencyMs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(
        statuses
            .iter()
            .map(|(status, _)| *status)
            .collect::<Vec<_>>(),
        expected_statuses
    );
    assert_eq!(
        statuses
            .iter()
            .map(|(_, latency)| *latency)
            .collect::<Vec<_>>(),
        expected_latencies
    );
}

#[test]
fn demo_fixture_documents_read_only_generated_state_and_governance_boundaries() {
    let demo_text = read("examples/game-feel-juice-v1/demo/game-feel-juice-demo-v1.json");
    let doc_text = read("docs/game-feel-juice-v1-demo.md");
    let combined = format!("{demo_text}\n{doc_text}");

    for required in [
        "fixture-scoped",
        "not a fun/quality verdict",
        "not a new engine",
        "not browser trusted authority",
        "browser/Studio surfaces read-only",
        "Rust/local owns trusted validation",
        "#1 and #23 remain open",
        "human Era J",
        "Godot replacement",
        "production-ready",
    ] {
        assert!(
            combined.contains(required),
            "missing demo boundary wording: {required}"
        );
    }

    let demo = demo();
    assert_eq!(demo["governanceAudit"]["issue1MustRemainOpen"], true);
    assert_eq!(demo["governanceAudit"]["issue23MustRemainOpen"], true);
    let disallowed = demo["readOnlyInspection"]["disallowedActions"]
        .as_array()
        .unwrap();
    assert!(disallowed.iter().any(|value| value == "network dependency"));
    assert!(disallowed
        .iter()
        .any(|value| value == "automated fun verdict"));
}

#[test]
fn demo_refs_resolve_and_reuse_existing_surfaces_without_live_browser() {
    let demo = demo();
    for fixture_ref in demo["reusedSurfaces"].as_array().unwrap() {
        let path = fixture_ref.as_str().unwrap();
        assert!(
            repo_root().join(path).is_file(),
            "missing reused surface: {path}"
        );
    }
    for step in demo["demoSteps"].as_array().unwrap() {
        let path = step["fixtureRef"].as_str().unwrap();
        assert!(
            repo_root().join(path).is_file(),
            "missing demo step fixture: {path}"
        );
        assert!(
            step["assertion"].as_str().unwrap().contains("read-only")
                || step["assertion"].as_str().unwrap().contains("Rust/local")
                || step["assertion"].as_str().unwrap().contains("100ms")
        );
    }
}

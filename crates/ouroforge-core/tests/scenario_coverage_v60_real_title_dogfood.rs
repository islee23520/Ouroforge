//! Scenario Coverage v60 regression suite for #2027 / Era L M68.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ouroforge_core::{
    read_ledger_events, run_dogfood_campaign_harness, DogfoodCampaignHarnessConfig,
    DogfoodFrictionObservation, DOGFOOD_CAMPAIGN_HARNESS_SCHEMA_VERSION,
};
use ouroforge_evaluator::dogfood_contract::DogfoodFrictionKind;
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn temp_runs_root(test_name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("ouroforge-{test_name}-{nonce}"));
    fs::create_dir_all(&path).expect("temp runs root");
    path
}

fn ledger_stage_set(run_dir: &Path) -> BTreeSet<String> {
    read_ledger_events(run_dir)
        .expect("ledger reads")
        .iter()
        .filter(|event| event["event"] == "dogfood.campaign.stage.completed")
        .map(|event| {
            event["payload"]["campaignStage"]
                .as_str()
                .expect("stage")
                .to_string()
        })
        .collect()
}

#[test]
fn v60_matrix_records_real_title_and_harness_regression_rows() {
    let matrix =
        read_json("examples/real-title-dogfood-v1/scenario-coverage-v60/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v60-real-title-dogfood-v1"
    );
    assert_eq!(matrix["coverageVersion"], 60);
    assert_eq!(matrix["titleId"], "era-i-engine-builder-deckbuilder");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "substrate",
        "scoring",
        "run-shop",
        "balance",
        "juice",
        "ui",
        "localization",
        "steam-export",
        "harness-stage-attribution",
        "harness-friction-logging",
        "harness-resumability",
        "harness-autonomy-invariants",
    ] {
        assert!(ids.contains(required), "missing v60 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
    }

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "ledger.jsonl",
        "source-apply",
        "trust-gradient",
        "not a new verifier",
        "data plane",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v60_harness_logs_stage_attribution_and_friction_in_existing_ledger() {
    let runs_root = temp_runs_root("v60-friction");
    let report = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
        seed_path: repo_root().join("seeds/dogfood-deckbuilder.yaml"),
        runs_root: runs_root.clone(),
        workers: 2,
        resume_run_dir: None,
        friction: vec![DogfoodFrictionObservation {
            friction_id: "retry-openchrome-worker-1".to_string(),
            kind: DogfoodFrictionKind::Retry,
            stage: "re-verify".to_string(),
            summary: "Scripted retry is captured as friction, not hidden or human-blocking."
                .to_string(),
            evidence_refs: vec![
                "ledger.jsonl".to_string(),
                "journal.md".to_string(),
                "verdict.json".to_string(),
            ],
        }],
    })
    .expect("harness run");

    assert_eq!(
        report.schema_version,
        DOGFOOD_CAMPAIGN_HARNESS_SCHEMA_VERSION
    );
    assert_eq!(report.status, "completed");
    assert_eq!(
        report.friction_logged,
        vec!["retry-openchrome-worker-1".to_string()]
    );
    assert!(report.boundary.contains("no new store"));

    let run_dir = PathBuf::from(&report.run_dir);
    let stages = ledger_stage_set(&run_dir);
    for required in [
        "detect",
        "explain",
        "trace",
        "attribute",
        "propose",
        "re-verify",
        "apply-or-queue",
    ] {
        assert!(stages.contains(required), "missing stage {required}");
    }

    let events = read_ledger_events(&run_dir).expect("ledger reads");
    let friction = events
        .iter()
        .find(|event| event["event"] == "dogfood.friction.retry")
        .expect("retry friction event");
    assert_eq!(friction["payload"]["campaignStage"], "re-verify");
    assert_eq!(friction["payload"]["ledgerEventKind"], "retry");
    assert!(friction["payload"]["boundary"]
        .as_str()
        .expect("boundary")
        .contains("not a new telemetry store"));

    let completed = events
        .iter()
        .find(|event| event["event"] == "dogfood.campaign.completed")
        .expect("completed event");
    assert_eq!(
        completed["payload"]["requiresHumanInputOnAutonomousPath"],
        false
    );
    assert_eq!(completed["payload"]["highRiskAutoApply"], false);
}

#[test]
fn v60_harness_resume_is_idempotent_and_does_not_duplicate_stage_events() {
    let runs_root = temp_runs_root("v60-resume");
    let first = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
        seed_path: repo_root().join("seeds/dogfood-deckbuilder.yaml"),
        runs_root: runs_root.clone(),
        workers: 2,
        resume_run_dir: None,
        friction: vec![],
    })
    .expect("first harness run");
    let run_dir = PathBuf::from(&first.run_dir);
    let first_events = read_ledger_events(&run_dir).expect("first ledger");

    let resumed = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
        seed_path: repo_root().join("seeds/dogfood-deckbuilder.yaml"),
        runs_root,
        workers: 2,
        resume_run_dir: Some(run_dir.clone()),
        friction: vec![],
    })
    .expect("resumed harness run");
    assert!(resumed.resumed);
    assert_eq!(resumed.run_dir, first.run_dir);

    let second_events = read_ledger_events(&run_dir).expect("second ledger");
    assert_eq!(
        first_events.len(),
        second_events.len(),
        "resume must be idempotent"
    );
    assert_eq!(ledger_stage_set(&run_dir).len(), 7);
}

#[test]
fn v60_docs_preserve_test_only_no_new_engine_boundaries() {
    let doc = read_text("docs/scenario-coverage-v60-real-title-dogfood.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "cargo test --workspace --jobs 2",
        "test-only rust coverage",
        "no new data plane",
        "does not introduce a verification engine",
        "fun/taste and release go/no-go remain human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }
}

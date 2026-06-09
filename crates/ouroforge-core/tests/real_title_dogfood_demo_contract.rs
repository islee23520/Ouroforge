//! Real-title dogfood demo contract for #2026 / Era L M68.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

#[test]
fn demo_script_runs_existing_seed_harness_and_ledger_surfaces() {
    let script = read_text("examples/real-title-dogfood-v1/demo/run-demo-v1.sh");
    assert!(script.contains("cargo run -p ouroforge-cli -- run \"$seed\" --workers \"$workers\""));
    assert!(script.contains("cargo run -p ouroforge-cli -- dogfood harness"));
    assert!(script.contains("--resume-run-dir \"$run_dir\""));
    assert!(script.contains("cargo run -p ouroforge-cli -- ledger list \"$run_dir\""));
    assert!(script.contains("friction-json '[]'"));
    for forbidden in [
        "sqlite",
        "postgres",
        "new_db_schema",
        "new_store_schema",
        "new_telemetry_schema",
    ] {
        assert!(!script.to_ascii_lowercase().contains(forbidden));
    }
}

#[test]
fn friction_summary_is_derived_from_existing_evidence_and_has_no_hidden_human_gate() {
    let summary = read_json("examples/real-title-dogfood-v1/demo/friction-summary.fixture.json");
    assert_eq!(
        summary["schemaVersion"],
        "real-title-dogfood-demo-friction-summary-v1"
    );
    assert_eq!(summary["titleId"], "era-i-engine-builder-deckbuilder");
    assert_eq!(summary["hiddenFriction"], false);
    assert_eq!(summary["autonomousPathRequiresHumanInput"], false);
    assert_eq!(summary["explicitNoFrictionEvent"], "dogfood.friction.none");
    assert!(summary["frictionPoints"]
        .as_array()
        .expect("frictionPoints")
        .is_empty());

    for reference in summary["summarySourceRefs"]
        .as_array()
        .expect("summarySourceRefs")
    {
        let reference = reference.as_str().expect("summary ref");
        assert!(
            repo_root().join(reference).is_file(),
            "missing summary ref {reference}"
        );
    }
    let boundary = summary["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    assert!(boundary.contains("existing ledger/journal/verdict"));
    assert!(boundary.contains("not a telemetry store"));
    assert!(boundary.contains("not a"));
}

#[test]
fn audit_trail_links_every_loop_stage_to_existing_pipeline_refs() {
    let audit = read_json("examples/real-title-dogfood-v1/demo/audit-trail.fixture.json");
    assert_eq!(
        audit["schemaVersion"],
        "real-title-dogfood-demo-audit-trail-v1"
    );
    assert_eq!(
        audit["runCommand"],
        "cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2"
    );
    assert_eq!(audit["highRiskAutoApply"], false);
    assert_eq!(audit["sourceAffectingAutoApply"], false);
    assert_eq!(audit["newVerificationEngine"], false);
    assert_eq!(audit["newDataPlane"], false);

    let stages: BTreeSet<_> = audit["stageAudit"]
        .as_array()
        .expect("stageAudit")
        .iter()
        .map(|row| row["stage"].as_str().expect("stage"))
        .collect();
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

    for row in audit["stageAudit"].as_array().expect("stageAudit") {
        assert_eq!(row["ledgerEvent"], "dogfood.campaign.stage.completed");
        let refs = row["evidenceRefs"].as_array().expect("evidenceRefs");
        assert!(refs.iter().any(|value| value == "ledger.jsonl"));
        assert!(refs.iter().any(|value| value == "journal.md"));
        assert!(refs.iter().any(|value| value == "verdict.json"));
    }

    let pipeline_refs: BTreeSet<_> = audit["pipelineRefs"]
        .as_array()
        .expect("pipelineRefs")
        .iter()
        .map(|value| value.as_str().expect("pipeline ref"))
        .collect();
    for required in [
        "openchrome",
        "scenario-verdicts",
        "four-gates",
        "design-integrity",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage-attribution",
        "evolve",
        "source-apply",
        "trust-gradient",
    ] {
        assert!(
            pipeline_refs.contains(required),
            "missing pipeline ref {required}"
        );
    }
}

#[test]
fn demo_doc_preserves_autonomy_and_governance_boundaries() {
    let doc = read_text("docs/real-title-dogfood-demo-v1.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2",
        "no new verification engine",
        "no new data plane",
        "high-risk/source-affecting fixes are never auto-applied",
        "fun/taste and release go/no-go remain human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }
}

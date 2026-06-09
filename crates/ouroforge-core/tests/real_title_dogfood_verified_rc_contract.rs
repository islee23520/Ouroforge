//! Real-title dogfood verified RC contract for #2025 / Era L M68.
//!
//! This locks the committed evidence bundle to the existing Ouroforge evidence
//! plane: seed YAML, openchrome/scenario verdict refs, journal.md,
//! ledger.jsonl, loop-coverage attribution, source-apply/trust-gradient
//! boundaries, and release provenance composition.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::release_provenance_bundle::{ReleaseProvenanceBundle, ReleaseProvenanceStatus};
use ouroforge_core::Seed;
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

fn ledger_events() -> Vec<Value> {
    read_text("examples/real-title-dogfood-v1/run/ledger.jsonl")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .unwrap_or_else(|err| panic!("parse ledger line: {err}: {line}"))
        })
        .collect()
}

#[test]
fn real_title_seed_targets_existing_engine_builder_deckbuilder() {
    let seed = Seed::from_path(repo_root().join("seeds/dogfood-deckbuilder.yaml"))
        .expect("dogfood seed validates");

    assert_eq!(seed.id, "dogfood.deckbuilder.v1");
    assert_eq!(seed.constraints.target, "game-runtime");
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("era-i-engine-builder-deckbuilder")));
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("no human input")));
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("never auto-apply")));
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("openchrome/scenario verdicts")));
    assert_eq!(seed.scenarios.len(), 1);
    assert_eq!(seed.scenarios[0].id, "dogfood-deckbuilder-real-title-rc");
}

#[test]
fn real_title_release_provenance_is_complete_and_replayable() {
    let bundle = ReleaseProvenanceBundle::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/release-provenance.complete.json",
    ))
    .expect("release provenance parses");
    let evaluation = bundle.evaluate_with_root(&repo_root());

    assert_eq!(
        evaluation.computed_status,
        ReleaseProvenanceStatus::Complete
    );
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(evaluation.replayable, "{evaluation:#?}");
    assert!(evaluation.issues.is_empty(), "{evaluation:#?}");

    for required in [
        "intent",
        "content",
        "assets",
        "qa",
        "per-change-provenance",
        "compliance",
        "release-candidate",
    ] {
        assert_eq!(
            evaluation.link_states.get(required).map(String::as_str),
            Some("present"),
            "missing release provenance link {required}: {evaluation:#?}"
        );
    }
    assert_eq!(
        evaluation
            .per_change_bundle_states
            .get("dogfood-per-change-provenance-rc-v1")
            .map(String::as_str),
        Some("complete")
    );

    assert!(bundle.generated_state.generated);
    assert!(bundle.generated_state.tracked);
    assert!(bundle.generated_state.fixture_scoped);
    assert!(bundle
        .guardrails
        .iter()
        .any(|item| item.contains("#1 and #23 remain open")));
}

#[test]
fn real_title_coverage_v60_records_all_required_stage_gates() {
    let matrix =
        read_json("examples/real-title-dogfood-v1/scenario-coverage-v60/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v60-real-title-dogfood-v1"
    );
    assert_eq!(matrix["titleId"], "era-i-engine-builder-deckbuilder");
    assert_eq!(matrix["coverageVersion"], 60);

    let rows = matrix["rows"].as_array().expect("rows array");
    let ids: BTreeSet<_> = rows
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
    ] {
        assert!(ids.contains(required), "missing stage {required}");
    }

    for row in rows {
        assert_eq!(row["status"], "pass", "stage did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence ref {evidence_ref}"
        );
    }

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    assert!(boundary.contains("not a new verifier"));
    assert!(boundary.contains("data plane"));
}

#[test]
fn real_title_autonomous_loop_completes_without_hidden_human_dependency() {
    let events = ledger_events();
    let completed_stages: BTreeSet<_> = events
        .iter()
        .filter(|event| event["event"] == "dogfood.campaign.stage.completed")
        .map(|event| {
            event["payload"]["campaignStage"]
                .as_str()
                .expect("campaignStage")
        })
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
        assert!(
            completed_stages.contains(required),
            "missing loop stage {required}"
        );
    }

    assert!(events
        .iter()
        .any(|event| event["event"] == "dogfood.friction.none"));
    let completed = events
        .iter()
        .find(|event| event["event"] == "dogfood.campaign.completed")
        .expect("campaign completed");
    assert_eq!(completed["payload"]["status"], "completed");
    assert_eq!(
        completed["payload"]["requiresHumanInputOnAutonomousPath"],
        false
    );
    assert_eq!(completed["payload"]["highRiskAutoApply"], false);
    assert!(completed["payload"]["dataPlane"]
        .as_str()
        .expect("dataPlane")
        .contains("Rust"));
    assert!(completed["payload"]["controlPlane"]
        .as_str()
        .expect("controlPlane")
        .contains("Elixir executor unchanged"));
}

#[test]
fn real_title_evidence_reuses_existing_pipeline_and_preserves_ring2_boundaries() {
    let qa = read_json("examples/real-title-dogfood-v1/refs/qa.json");
    assert_eq!(qa["status"], "pass");
    for required in ["semantic", "visual", "performance", "content-curation"] {
        assert!(qa["fourGates"]
            .as_array()
            .expect("fourGates")
            .iter()
            .any(|gate| gate == required));
    }
    assert_eq!(qa["designIntegrity"], "pass");
    assert!(qa["boundary"]
        .as_str()
        .expect("boundary")
        .contains("no parallel verifier"));

    let release_candidate = read_json("examples/real-title-dogfood-v1/refs/release-candidate.json");
    assert_eq!(release_candidate["status"], "verified-rc");
    assert_eq!(
        release_candidate["humanReleaseGate"],
        "required-after-verified-rc"
    );
    assert!(release_candidate["boundary"]
        .as_str()
        .expect("boundary")
        .contains("not a fun/taste verdict"));

    let compliance = read_json("examples/real-title-dogfood-v1/refs/compliance.json");
    assert_eq!(compliance["humanRing2Required"], true);
    assert_eq!(compliance["funTasteAutomated"], false);
    assert_eq!(compliance["releaseGoNoGoAutomated"], false);
    assert_eq!(compliance["layer3Deferred"], true);

    let rollback = read_json("examples/real-title-dogfood-v1/refs/promotion-rollback-record.json");
    assert_eq!(rollback["highRiskAutoApply"], false);
    assert_eq!(rollback["sourceAffectingAutoApply"], false);
    assert!(rollback["boundary"]
        .as_str()
        .expect("boundary")
        .contains("source-apply and trust-gradient"));
}

#[test]
fn real_title_docs_and_fixtures_do_not_introduce_a_new_store_or_verifier() {
    let doc = read_text("docs/real-title-dogfood-verified-rc-v1.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2",
        "existing release provenance bundle validator",
        "no new verification engine",
        "no new data plane",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }

    let forbidden_terms = ["new_db_schema", "new_store_schema", "new_telemetry_schema"];
    for path in [
        "docs/real-title-dogfood-verified-rc-v1.md",
        "examples/real-title-dogfood-v1/release-provenance.complete.json",
        "examples/real-title-dogfood-v1/refs/qa.json",
        "examples/real-title-dogfood-v1/refs/validation-result.json",
    ] {
        let text = read_text(path).to_ascii_lowercase();
        for forbidden in forbidden_terms {
            assert!(
                !text.contains(forbidden),
                "{path} contains forbidden marker {forbidden}"
            );
        }
    }
}

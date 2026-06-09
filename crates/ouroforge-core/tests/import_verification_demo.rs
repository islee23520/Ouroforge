use std::path::{Path, PathBuf};

use ouroforge_core::import_verification_report::verify_godot_import;
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
fn demo_fixture_and_script_record_runnable_commands() {
    let fixture = read_json(
        "examples/godot-2d-adapter-v1/import-verification-demo/demo-summary.fixture.json",
    );
    assert_eq!(fixture["schemaVersion"], "import-verification-demo-v1");
    assert_eq!(fixture["issueRef"], "#2180");
    let script_ref = fixture["scriptRef"].as_str().unwrap();
    assert!(
        repo_root().join(script_ref).exists(),
        "missing script {script_ref}"
    );
    assert!(fixture["reportCommand"]
        .as_str()
        .unwrap()
        .contains("migration verify-demo"));
    assert!(fixture["loopCommand"]
        .as_str()
        .unwrap()
        .contains("seeds/migration-demo.yaml"));
    assert!(fixture["boundary"]
        .as_str()
        .unwrap()
        .contains("no auto-port claim"));

    let script = read_text(script_ref);
    for required in [
        "migration verify-demo",
        "seeds/migration-demo.yaml",
        "claimed_ported_units",
        "verification_state_hash",
        "openchrome-local-skeleton-smoke",
        "no auto-port claim",
    ] {
        assert!(script.contains(required), "script missing {required}");
    }
}

#[test]
fn demo_summary_matches_live_import_report_shape() {
    let fixture = read_json(
        "examples/godot-2d-adapter-v1/import-verification-demo/demo-summary.fixture.json",
    );
    let expected = &fixture["expectedSummary"];
    let report =
        verify_godot_import(repo_root().join("examples/godot-2d-adapter-v1/sample-project"))
            .unwrap();

    assert_eq!(report.source_engine, expected["sourceEngine"]);
    assert_eq!(
        report.skeleton_verification.runner,
        expected["openchromeRunner"]
    );
    assert_eq!(
        report.skeleton_verification.status,
        expected["skeletonVerificationStatus"]
    );
    assert!(report.fidelity_report.clean >= expected["minClean"].as_u64().unwrap() as usize);
    assert!(report.fidelity_report.flagged >= expected["minFlagged"].as_u64().unwrap() as usize);
    assert!(report.fidelity_report.rederive >= expected["minReDerive"].as_u64().unwrap() as usize);
    assert_eq!(
        report.claimed_ported_units.len(),
        expected["claimedPortedUnits"].as_u64().unwrap() as usize
    );
    assert_eq!(
        report.verification_state_hash.starts_with("sha256:"),
        expected["deterministicStateHash"]
    );
    assert_eq!(report.provenance.origin, expected["assetOrigin"]);
    assert_eq!(
        report.provenance.decompiled_source_copied,
        expected["decompiledSourceCopied"]
    );
    assert_eq!(
        report.data_shapes.no_elixir_artifact_semantics,
        !expected["elixirOwnsArtifactSemantics"].as_bool().unwrap()
    );
    assert!(report
        .oracle_records
        .iter()
        .all(|oracle| !oracle.ported_claim_allowed));
    assert!(!report.re_derivation_tasks.is_empty());
}

#[test]
fn docs_explain_demo_evidence_without_port_overclaim() {
    let doc = read_text("docs/import-verification-demo-v1.md").to_ascii_lowercase();
    for required in [
        "import verification and fidelity report demo v1",
        "run-demo.sh",
        "openchrome-local-skeleton-smoke",
        "clean, flagged, and re-derive",
        "claimed_ported_units",
        "deterministic state hashes",
        "not translated or claimed complete",
        "no elixir/phoenix trusted write",
        "#1 and #23 remain open",
        "no finished-game auto-port",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

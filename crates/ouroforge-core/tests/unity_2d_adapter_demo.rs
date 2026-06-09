use std::path::{Path, PathBuf};

use ouroforge_core::unity_2d_adapter_ir::{
    parse_unity_2d_project, unity_2d_adapter_demo_report, validate_unity_2d_adapter_demo_report,
    UnityFidelityGrade,
};
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

fn sample_project_root() -> PathBuf {
    repo_root().join("examples/unity-2d-adapter-v1/sample-project")
}

#[test]
fn unity_demo_fixture_and_script_record_runnable_commands() {
    let fixture = read_json("examples/unity-2d-adapter-v1/demo/demo-summary.fixture.json");
    assert_eq!(fixture["schemaVersion"], "unity-2d-adapter-demo-v1");
    assert_eq!(fixture["issueRef"], "#2184");
    let script_ref = fixture["scriptRef"].as_str().unwrap();
    assert!(
        repo_root().join(script_ref).exists(),
        "missing script {script_ref}"
    );
    assert!(fixture["reportCommand"]
        .as_str()
        .unwrap()
        .contains("migration unity-demo"));
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
        "migration unity-demo",
        "seeds/migration-demo.yaml",
        "claimed_ported_units",
        "ir_state_hash",
        "no auto-port claim",
        "clean_room_source_only",
        "decompiled_source_copied",
    ] {
        assert!(script.contains(required), "script missing {required}");
    }
}

#[test]
fn unity_demo_summary_matches_live_adapter_report_shape() {
    let fixture = read_json("examples/unity-2d-adapter-v1/demo/demo-summary.fixture.json");
    let expected = &fixture["expectedSummary"];
    let report = unity_2d_adapter_demo_report(sample_project_root()).unwrap();
    validate_unity_2d_adapter_demo_report(&report).expect("fixture report is valid");

    assert_eq!(report.source_engine, expected["sourceEngine"]);
    assert!(report.fidelity_summary.green >= expected["minGreen"].as_u64().unwrap() as usize);
    assert!(report.fidelity_summary.yellow >= expected["minYellow"].as_u64().unwrap() as usize);
    assert!(report.fidelity_summary.red >= expected["minRed"].as_u64().unwrap() as usize);
    assert_eq!(
        report.claimed_ported_units.len(),
        expected["claimedPortedUnits"].as_u64().unwrap() as usize
    );
    assert_eq!(
        report.ir_state_hash.starts_with("sha256:"),
        expected["deterministicStateHash"]
    );
    assert_eq!(
        report.provenance.decompiled_source_copied,
        expected["decompiledSourceCopied"]
    );
    assert_eq!(
        report.data_shapes.no_elixir_artifact_semantics,
        !expected["elixirOwnsArtifactSemantics"].as_bool().unwrap()
    );
    assert!(report.logic_touchpoint_count > 0);
    assert!(report.oracle_record_count > 0);
    assert!(report
        .oracle_gate
        .contains("No Unity unit is claimed ported"));
}

#[test]
fn unity_demo_imports_skeleton_honestly_without_port_claims() {
    let ir = parse_unity_2d_project(sample_project_root()).expect("Unity fixture parses");
    assert!(ir.boundary.contains("one-way"));
    assert!(ir.boundary.contains("clean-room"));
    assert!(ir.source.accepted_formats.iter().any(|fmt| fmt == ".unity"));
    assert!(ir
        .source
        .accepted_formats
        .iter()
        .any(|fmt| fmt == ".prefab"));
    assert!(ir.source.accepted_formats.iter().any(|fmt| fmt == ".meta"));
    assert_eq!(ir.scenes.len(), 1);
    assert_eq!(ir.prefabs.len(), 1);
    assert!(ir.prefabs[0].prefab_overrides_flattened);
    assert!(ir
        .assets
        .iter()
        .any(|asset| asset.guid == "playerguid123" && asset.source_path.ends_with("player.png")));
    assert!(ir
        .scenes
        .iter()
        .flat_map(|scene| scene.nodes.iter())
        .any(|node| node.name == "Player" && node.fidelity_grade == UnityFidelityGrade::Red));
    assert!(ir
        .logic_touchpoints
        .iter()
        .all(|touch| touch.era_r_status == "requires-clean-room-re-derivation"));
    assert!(ir.claimed_ported_units.is_empty());
    assert!(ir
        .oracle_records
        .iter()
        .all(|oracle| !oracle.ported_claim_allowed));
}

#[test]
fn unity_demo_rejects_forged_port_claim_or_green_logic_gap() {
    let report = unity_2d_adapter_demo_report(sample_project_root()).unwrap();
    let mut forged = report.clone();
    forged
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_unity_2d_adapter_demo_report(&forged)
        .expect_err("ungated Unity port claim must fail");
    assert!(
        err.to_string().contains("cannot claim ported units"),
        "{err}"
    );

    let mut greenwashed = report;
    greenwashed.fidelity_summary.red = 0;
    let err = validate_unity_2d_adapter_demo_report(&greenwashed)
        .expect_err("logic gaps cannot be greenwashed");
    assert!(err
        .to_string()
        .contains("must keep unsupported/logic gaps Red"));
}

#[test]
fn docs_explain_unity_demo_evidence_without_port_overclaim() {
    let doc = read_text("docs/unity-2d-adapter-demo-v1.md").to_ascii_lowercase();
    for required in [
        "unity 2d adapter demo v1",
        "run-demo.sh",
        "force-text",
        "green, yellow, and red",
        "claimed_ported_units",
        "deterministic",
        "state hash",
        "not translated or claimed complete",
        "no unity runtime bridge",
        "no elixir/phoenix trusted write",
        "#1 and #23 remain open",
        "not a finished-game auto-port",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

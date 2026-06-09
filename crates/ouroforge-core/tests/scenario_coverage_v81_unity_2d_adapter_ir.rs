//! Scenario Coverage v81 regression suite for #2185 / Era O M93.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::unity_2d_adapter_ir::{
    parse_unity_2d_project, parse_unity_2d_source_files, unity_2d_adapter_demo_report,
    validate_unity_2d_adapter_demo_report, validate_unity_2d_ir, UnityFidelityGrade,
    UnitySourceFile, UNITY_2D_ADAPTER_BOUNDARY, UNITY_2D_ADAPTER_IR_SCHEMA_VERSION,
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

fn matrix() -> Value {
    read_json("examples/unity-2d-adapter-v1/scenario-coverage-v81/matrix.fixture.json")
}

fn sample_project_root() -> PathBuf {
    repo_root().join("examples/unity-2d-adapter-v1/sample-project")
}

fn source_files_with_player_x(x: i64) -> Vec<UnitySourceFile> {
    vec![
        UnitySourceFile {
            path: "ProjectSettings/InputManager.asset".to_string(),
            contents: r#"
InputManager:
  m_Axes:
  - serializedVersion: 3
    m_Name: Horizontal
    positiveButton: d
    negativeButton: a
  - serializedVersion: 3
    m_Name: Fire
    positiveButton: space
"#
            .to_string(),
        },
        UnitySourceFile {
            path: "Assets/Sprites/player.png.meta".to_string(),
            contents: r#"
fileFormatVersion: 2
guid: playerguid123
TextureImporter:
  spritePixelsToUnits: 32
"#
            .to_string(),
        },
        UnitySourceFile {
            path: "Assets/Scripts/PlayerController.cs.meta".to_string(),
            contents: r#"
fileFormatVersion: 2
guid: scriptguid456
MonoImporter:
  executionOrder: 0
"#
            .to_string(),
        },
        UnitySourceFile {
            path: "Assets/Scenes/Main.unity".to_string(),
            contents: format!(
                r#"
%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!1 &100
GameObject:
  m_Name: Root
  m_IsActive: 1
  m_Component:
  - component: {{fileID: 101}}
--- !u!4 &101
Transform:
  m_GameObject: {{fileID: 100}}
  m_LocalPosition: {{x: 0, y: 0, z: 0}}
  m_LocalRotation: {{x: 0, y: 0, z: 0, w: 1}}
  m_LocalScale: {{x: 1, y: 1, z: 1}}
  m_Father: {{fileID: 0}}
--- !u!1 &200
GameObject:
  m_Name: Player
  m_IsActive: 1
  m_Component:
  - component: {{fileID: 201}}
  - component: {{fileID: 202}}
  - component: {{fileID: 203}}
  - component: {{fileID: 204}}
--- !u!4 &201
Transform:
  m_GameObject: {{fileID: 200}}
  m_LocalPosition: {{x: {x}, y: 2, z: 0}}
  m_LocalRotation: {{x: 0, y: 0, z: 0, w: 1}}
  m_LocalScale: {{x: 1, y: 1, z: 1}}
  m_Father: {{fileID: 101}}
--- !u!212 &202
SpriteRenderer:
  m_GameObject: {{fileID: 200}}
  m_Sprite: {{fileID: 21300000, guid: playerguid123, type: 3}}
--- !u!61 &203
BoxCollider2D:
  m_GameObject: {{fileID: 200}}
  m_IsTrigger: 1
--- !u!114 &204
MonoBehaviour:
  m_GameObject: {{fileID: 200}}
  m_Script: {{fileID: 11500000, guid: scriptguid456, type: 3}}
  speed: 7
--- !u!1 &300
GameObject:
  m_Name: Main Camera
  m_IsActive: 1
  m_Component:
  - component: {{fileID: 301}}
  - component: {{fileID: 302}}
--- !u!4 &301
Transform:
  m_GameObject: {{fileID: 300}}
  m_LocalPosition: {{x: 0, y: 0, z: -10}}
--- !u!20 &302
Camera:
  m_GameObject: {{fileID: 300}}
  m_Orthographic: 1
  m_OrthographicSize: 5
"#
            ),
        },
        UnitySourceFile {
            path: "Assets/Prefabs/Crate.prefab".to_string(),
            contents: r#"
%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!1 &400
GameObject:
  m_Name: Crate
  m_IsActive: 1
  m_Component:
  - component: {fileID: 401}
  - component: {fileID: 402}
--- !u!4 &401
Transform:
  m_GameObject: {fileID: 400}
  m_LocalPosition: {x: 3, y: 4, z: 0}
--- !u!212 &402
SpriteRenderer:
  m_GameObject: {fileID: 400}
  m_Sprite: {fileID: 21300000, guid: playerguid123, type: 3}
--- !u!1001 &900
PrefabInstance:
  m_Modification:
    m_Modifications:
    - target: {fileID: 400, guid: prefabguid789, type: 3}
      propertyPath: m_Name
      value: CrateOverride
"#
            .to_string(),
        },
    ]
}

#[test]
fn v81_unity_matrix_records_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v81-unity-2d-adapter-ir-v1"
    );
    assert_eq!(matrix["coverageVersion"], 81);
    assert_eq!(matrix["issueRef"], "#2185");
    assert_eq!(matrix["milestone"], "Era O M93");
    assert_eq!(
        matrix["demoRef"],
        "examples/unity-2d-adapter-v1/demo/run-demo.sh"
    );

    let rows = matrix["rows"].as_array().expect("rows");
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "v81.unity-force-text-skeleton-import",
        "v81.unity-lossy-import-not-clean",
        "v81.unity-no-auto-port-without-oracle",
        "v81.unity-deterministic-state-hash-break-fails",
        "v81.unity-coverage-ledger-and-boundaries",
    ] {
        assert!(ids.contains(required), "missing v81 row {required}");
    }
    for row in rows {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidence ref");
        assert!(
            repo_root().join(evidence_ref).exists(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 90);
    }

    let invariants = &matrix["invariants"];
    assert_eq!(invariants["oneWayOnRamp"], true);
    assert_eq!(invariants["sourceProjectOpenTextOnly"], true);
    assert_eq!(invariants["unityForceTextOnly"], true);
    assert_eq!(invariants["cleanRoomReDerivation"], true);
    assert_eq!(invariants["autoPortWithoutOracleAllowed"], false);
    assert_eq!(invariants["lossyImportMayGradeGreen"], false);
    assert_eq!(invariants["deterministicStateHashRequired"], true);
    assert_eq!(invariants["staleStateHashAllowed"], false);
    assert_eq!(invariants["runtimeBridgeAllowed"], false);
    assert_eq!(invariants["rustOwnsArtifactTruth"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["foreignRuntimeBridgeAllowed"], false);
    assert_eq!(invariants["decompiledSourceCopied"], false);
    assert_eq!(invariants["anchorsRemainOpen"], true);
}

#[test]
fn v81_unity_force_text_skeleton_import_is_read_only_and_honest() {
    let ir = parse_unity_2d_project(sample_project_root()).expect("Unity sample parses");
    assert_eq!(ir.schema_version, UNITY_2D_ADAPTER_IR_SCHEMA_VERSION);
    assert_eq!(ir.boundary, UNITY_2D_ADAPTER_BOUNDARY);
    assert!(ir.source.accepted_formats.iter().any(|fmt| fmt == ".unity"));
    assert!(ir
        .source
        .accepted_formats
        .iter()
        .any(|fmt| fmt == ".prefab"));
    assert!(ir.source.accepted_formats.iter().any(|fmt| fmt == ".asset"));
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
        .all(|oracle| oracle.status == "missing" && !oracle.ported_claim_allowed));

    let report = unity_2d_adapter_demo_report(sample_project_root()).expect("demo report");
    validate_unity_2d_adapter_demo_report(&report).expect("demo report validates");
    assert_eq!(report.source_engine, "unity");
    assert!(report.fidelity_summary.green > 0);
    assert!(report.fidelity_summary.yellow > 0);
    assert!(report.fidelity_summary.red > 0);
    assert!(report.claimed_ported_units.is_empty());
    assert!(report.provenance.clean_room_source_only);
    assert!(!report.provenance.decompiled_source_copied);
    assert!(report.data_shapes.no_elixir_artifact_semantics);
}

#[test]
fn v81_unity_lossy_import_and_behavioral_gaps_cannot_be_laundered_green() {
    let ir = parse_unity_2d_source_files("v81-unity-lossy", source_files_with_player_x(10))
        .expect("lossy fixture parses");
    assert!(ir.fidelity_report.summary.yellow > 0, "{ir:#?}");
    assert!(ir.fidelity_report.summary.red > 0, "{ir:#?}");
    assert!(ir.fidelity_report.records.iter().any(|record| record.grade
        == UnityFidelityGrade::Red
        && record.reason.contains("clean-room")
        || record.reason.contains("unsupported")
        || record.reason.contains("Logic")));
    assert!(ir
        .logic_touchpoints
        .iter()
        .all(|touch| touch.fidelity_grade == UnityFidelityGrade::Red));
    assert!(ir
        .unsupported
        .iter()
        .all(|feature| feature.fidelity_grade == UnityFidelityGrade::Red));

    let mut report = unity_2d_adapter_demo_report(sample_project_root()).expect("demo report");
    report.fidelity_summary.red = 0;
    let err = validate_unity_2d_adapter_demo_report(&report)
        .expect_err("lossy Unity report cannot be graded clean");
    assert!(
        err.to_string()
            .contains("must keep unsupported/logic gaps Red"),
        "{err}"
    );
}

#[test]
fn v81_unity_no_auto_port_without_oracle_or_auto_translation_claim() {
    let mut ir = parse_unity_2d_project(sample_project_root()).expect("Unity sample parses");
    assert!(ir.claimed_ported_units.is_empty());
    ir.claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_unity_2d_ir(&ir).expect_err("ungated claim fails");
    assert!(err.to_string().contains("cannot claim ported units"));

    let mut ir = parse_unity_2d_project(sample_project_root()).expect("Unity sample parses");
    ir.oracle_records[0].ported_claim_allowed = true;
    let err = validate_unity_2d_ir(&ir).expect_err("oracle bypass fails");
    assert!(err.to_string().contains("oracle-missing"));

    let mut report = unity_2d_adapter_demo_report(sample_project_root()).expect("demo report");
    report
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_unity_2d_adapter_demo_report(&report).expect_err("report claim fails");
    assert!(err.to_string().contains("cannot claim ported units"));
}

#[test]
fn v81_unity_deterministic_state_hash_is_stable_and_tamper_break_fails() {
    let first =
        parse_unity_2d_source_files("v81-unity-determinism", source_files_with_player_x(10))
            .expect("first parses");
    let second =
        parse_unity_2d_source_files("v81-unity-determinism", source_files_with_player_x(10))
            .expect("second parses");
    let changed =
        parse_unity_2d_source_files("v81-unity-determinism", source_files_with_player_x(11))
            .expect("changed parses");

    assert_eq!(first, second);
    assert_eq!(first.state_hash, second.state_hash);
    assert_ne!(first.state_hash, changed.state_hash);

    let mut tampered = first.clone();
    tampered.fidelity_report.records[0]
        .reason
        .push_str(" tampered stale report");
    let err = validate_unity_2d_ir(&tampered).expect_err("stale hash fails");
    assert!(err.to_string().contains("state hash"), "{err}");
}

#[test]
fn v81_unity_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v81-unity-2d-adapter-ir.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v81",
        "unity 2d adapter",
        "force-text",
        "source-project/open-text",
        "one-way on-ramp",
        "clean-room",
        "no auto-port",
        "passing oracle",
        "claimed_ported_units",
        "lossy import",
        "yellow/red",
        "deterministic",
        "state hashes",
        "rust remains the data plane",
        "elixir/phoenix studio is not touched",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

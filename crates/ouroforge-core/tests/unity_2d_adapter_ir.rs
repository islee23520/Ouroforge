use ouroforge_core::unity_2d_adapter_ir::{
    parse_unity_2d_source_files, validate_unity_2d_ir, UnityFidelityGrade, UnityPresentation,
    UnitySourceFile, UNITY_2D_ADAPTER_BOUNDARY, UNITY_2D_ADAPTER_IR_SCHEMA_VERSION,
};

fn unity_fixture(player_x: i64) -> Vec<UnitySourceFile> {
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
  m_Layer: 3
  m_TagString: Player
  m_Component:
  - component: {{fileID: 201}}
  - component: {{fileID: 202}}
  - component: {{fileID: 203}}
  - component: {{fileID: 204}}
--- !u!4 &201
Transform:
  m_GameObject: {{fileID: 200}}
  m_LocalPosition: {{x: {player_x}, y: 2, z: 0}}
  m_LocalRotation: {{x: 0, y: 0, z: 0, w: 1}}
  m_LocalScale: {{x: 1, y: 1, z: 1}}
  m_Father: {{fileID: 101}}
--- !u!212 &202
SpriteRenderer:
  m_GameObject: {{fileID: 200}}
  m_Sprite: {{fileID: 21300000, guid: playerguid123, type: 3}}
  m_Color: {{r: 1, g: 1, b: 1, a: 1}}
  m_SortingOrder: 5
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
  m_LocalRotation: {{x: 0, y: 0, z: 0, w: 1}}
  m_LocalScale: {{x: 1, y: 1, z: 1}}
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
  m_LocalRotation: {x: 0, y: 0, z: 0, w: 1}
  m_LocalScale: {x: 1, y: 1, z: 1}
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
fn unity_2d_source_project_parses_to_deterministic_ir_with_meta_refs() {
    let first = parse_unity_2d_source_files("unity-fixture", unity_fixture(1)).expect("parses");
    let second = parse_unity_2d_source_files("unity-fixture", unity_fixture(1)).expect("parses");

    assert_eq!(first, second);
    assert_eq!(first.schema_version, UNITY_2D_ADAPTER_IR_SCHEMA_VERSION);
    assert_eq!(first.boundary, UNITY_2D_ADAPTER_BOUNDARY);
    assert!(first.state_hash.starts_with("sha256:"));
    assert!(first
        .source
        .contract_ref
        .ends_with("unity-2d-adapter-ir-contract-v1.md"));
    assert_eq!(first.scenes.len(), 1);
    assert_eq!(first.prefabs.len(), 1);
    assert!(first.prefabs[0].prefab_overrides_flattened);
    assert_eq!(first.inputs.len(), 2);
    assert!(first
        .assets
        .iter()
        .any(|asset| asset.guid == "playerguid123"
            && asset.source_path == "Assets/Sprites/player.png"));

    let player = first.scenes[0]
        .nodes
        .iter()
        .find(|node| node.name == "Player")
        .expect("player node");
    assert_eq!(
        player.parent_id.as_deref(),
        Some("unity-node:Assets/Scenes/Main.unity:100")
    );
    assert_eq!(player.transform2d.position.as_ref().unwrap()[0], "1");
    assert_eq!(player.fidelity_grade, UnityFidelityGrade::Red);
    assert!(matches!(
        player.presentation,
        Some(UnityPresentation::SpriteRenderer { .. })
    ));
    assert!(player.collider.as_ref().unwrap().physics_re_simulated);
    assert!(player
        .components
        .iter()
        .any(|component| component.unity_type == "MonoBehaviour"
            && component.fidelity_grade == UnityFidelityGrade::Red));

    let camera = first.scenes[0]
        .nodes
        .iter()
        .find(|node| node.name == "Main Camera")
        .expect("camera node");
    assert!(camera.camera.as_ref().unwrap().orthographic);

    assert!(first.fidelity_report.summary.green > 0);
    assert!(first.fidelity_report.summary.yellow > 0);
    assert!(first.fidelity_report.summary.red > 0);
    assert!(first.logic_touchpoints.iter().any(|touchpoint| {
        touchpoint.trigger_kind == "script-ref"
            && touchpoint.era_r_status == "requires-clean-room-re-derivation"
            && touchpoint.fidelity_grade == UnityFidelityGrade::Red
    }));
    assert!(first
        .oracle_records
        .iter()
        .all(|oracle| oracle.status == "missing" && !oracle.ported_claim_allowed));
    assert!(first.claimed_ported_units.is_empty());
    assert!(first
        .fidelity_report
        .oracle_rule
        .contains("bit-exact deterministic state hashes"));
    assert!(first
        .fidelity_report
        .clean_room_notice
        .contains("never copied or translated"));
}

#[test]
fn unity_adapter_rejects_shipped_build_or_decompiled_inputs() {
    let err = parse_unity_2d_source_files(
        "bad-unity",
        vec![UnitySourceFile {
            path: "Builds/MyGame_Data/resources.assets".to_string(),
            contents: String::new(),
        }],
    )
    .expect_err("shipped build artifacts are rejected");
    assert!(err.to_string().contains("Force-Text/.meta only"));

    let err = parse_unity_2d_source_files(
        "bad-unity",
        vec![UnitySourceFile {
            path: "Assets/Decompiled/PlayerController.cs".to_string(),
            contents: String::new(),
        }],
    )
    .expect_err("decompiled source is rejected");
    assert!(err.to_string().contains("Force-Text/.meta only"));
}

#[test]
fn unity_adapter_blocks_port_claims_or_oracle_bypass() {
    let mut ir = parse_unity_2d_source_files("unity-fixture", unity_fixture(1)).expect("parses");
    ir.claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_unity_2d_ir(&ir).expect_err("claimed port fails");
    assert!(err.to_string().contains("cannot claim ported units"));

    let mut ir = parse_unity_2d_source_files("unity-fixture", unity_fixture(1)).expect("parses");
    ir.oracle_records[0].ported_claim_allowed = true;
    let err = validate_unity_2d_ir(&ir).expect_err("oracle bypass fails");
    assert!(err.to_string().contains("oracle-missing"));
}

#[test]
fn unity_adapter_state_hash_is_stable_and_tamper_break_fails() {
    let first = parse_unity_2d_source_files("unity-fixture", unity_fixture(1)).expect("first");
    let second = parse_unity_2d_source_files("unity-fixture", unity_fixture(1)).expect("second");
    let changed = parse_unity_2d_source_files("unity-fixture", unity_fixture(2)).expect("changed");

    assert_eq!(first.state_hash, second.state_hash);
    assert_ne!(first.state_hash, changed.state_hash);

    let mut tampered = first.clone();
    tampered.fidelity_report.records[0]
        .reason
        .push_str(" tampered");
    let err = validate_unity_2d_ir(&tampered).expect_err("stale hash fails");
    assert!(err.to_string().contains("state hash"));
}

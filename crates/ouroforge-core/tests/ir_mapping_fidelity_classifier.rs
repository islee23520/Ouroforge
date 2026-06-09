use ouroforge_core::godot_2d_adapter_ir::{
    parse_godot_2d_project, parse_godot_2d_source_files, GodotSourceFile,
};
use ouroforge_core::ir_mapping_fidelity_classifier::{
    map_godot_ir_to_ouroforge, validate_mapping_artifact, MappingFidelityGrade,
    IR_MAPPING_BOUNDARY, IR_MAPPING_SCHEMA_VERSION,
};

fn sample_root() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/godot-2d-adapter-v1/sample-project"
    )
}

fn source_files_with_player_x(x: i64) -> Vec<GodotSourceFile> {
    vec![
        GodotSourceFile {
            path: "project.godot".to_string(),
            contents: r#"
[input]
move_left={"events":[{"physical_keycode":65}]}
"#
            .to_string(),
        },
        GodotSourceFile {
            path: "main.tscn".to_string(),
            contents: format!(
                r#"
[gd_scene load_steps=2 format=3 uid="uid://mapping-test"]

[ext_resource type="Texture2D" path="res://player.png" id="tex_player"]

[node name="Root" type="Node2D"]

[node name="Player" type="Sprite2D" parent="."]
position = Vector2({x}, 20)
texture = ExtResource("tex_player")
script = ExtResource("player_script")

[node name="Hitbox" type="Area2D" parent="Player"]
collision_layer = 1
collision_mask = 1

[node name="Particles" type="GPUParticles2D" parent="."]
"#
            ),
        },
    ]
}

#[test]
fn maps_godot_ir_to_native_candidates_with_normalized_shapes() {
    let ir = parse_godot_2d_project(sample_root()).expect("Godot fixture parses");
    let artifact = map_godot_ir_to_ouroforge(&ir).expect("mapping succeeds");

    assert_eq!(artifact.schema_version, IR_MAPPING_SCHEMA_VERSION);
    assert_eq!(artifact.boundary, IR_MAPPING_BOUNDARY);
    assert_eq!(artifact.coordinate_space.pixels_per_unit, 1);
    assert_eq!(artifact.coordinate_space.color_space, "srgb");
    assert!(artifact.state_hash.starts_with("sha256:"));
    assert!(artifact.claimed_ported_units.is_empty());
    assert_eq!(artifact.scenes.len(), 1);

    let player = artifact.scenes[0]
        .entities
        .iter()
        .find(|entity| entity.name == "Player")
        .expect("player entity");
    assert_eq!(player.transform.position_units, Some([10.0, 20.0]));
    assert_eq!(player.transform.scale, Some([2.0, 2.0]));
    assert_eq!(player.fidelity_grade, MappingFidelityGrade::Green);
    assert!(player.presentation.is_some());

    let hitbox = artifact.scenes[0]
        .entities
        .iter()
        .find(|entity| entity.name == "Hitbox")
        .expect("hitbox entity");
    assert_eq!(hitbox.fidelity_grade, MappingFidelityGrade::Yellow);
    assert_eq!(
        hitbox.collider.as_ref().expect("collider").physics_policy,
        "re-simulated-in-ouroforge-not-reproduced"
    );
}

#[test]
fn fidelity_classifier_routes_logic_and_unsupported_units_to_era_r() {
    let ir = parse_godot_2d_project(sample_root()).expect("Godot fixture parses");
    let artifact = map_godot_ir_to_ouroforge(&ir).expect("mapping succeeds");

    assert!(artifact.fidelity_report.green > 0);
    assert!(artifact.fidelity_report.yellow > 0);
    assert!(artifact.fidelity_report.red > 0);
    assert!(artifact
        .fidelity_report
        .oracle_rule
        .contains("No mapped unit is ported"));
    assert!(artifact
        .behavioral_units
        .iter()
        .all(|unit| unit.fidelity_grade == MappingFidelityGrade::Red));
    assert!(artifact
        .behavioral_units
        .iter()
        .any(|unit| unit.era_r_status == "requires-clean-room-re-derivation"));
    assert!(artifact
        .oracle_records
        .iter()
        .all(|oracle| !oracle.ported_claim_allowed));
    assert!(artifact
        .fidelity_report
        .gap_summary
        .iter()
        .any(|gap| gap.contains("clean-room") || gap.contains("Unsupported")));
}

#[test]
fn mapping_is_deterministic_and_source_drift_changes_state_hash() {
    let first_ir = parse_godot_2d_source_files("mapping", source_files_with_player_x(10))
        .expect("first parses");
    let second_ir = parse_godot_2d_source_files("mapping", source_files_with_player_x(10))
        .expect("second parses");
    let changed_ir = parse_godot_2d_source_files("mapping", source_files_with_player_x(11))
        .expect("changed parses");

    let first = map_godot_ir_to_ouroforge(&first_ir).expect("first maps");
    let second = map_godot_ir_to_ouroforge(&second_ir).expect("second maps");
    let changed = map_godot_ir_to_ouroforge(&changed_ir).expect("changed maps");

    assert_eq!(first, second);
    assert_ne!(first.state_hash, changed.state_hash);
}

#[test]
fn ungated_port_claims_fail_mapping_validation() {
    let ir = parse_godot_2d_project(sample_root()).expect("Godot fixture parses");
    let mut artifact = map_godot_ir_to_ouroforge(&ir).expect("mapping succeeds");
    artifact
        .claimed_ported_units
        .push("auto-translated:Player".to_string());

    let err = validate_mapping_artifact(&artifact).expect_err("ported claim must fail");
    assert!(err.to_string().contains("cannot claim ported units"));
}

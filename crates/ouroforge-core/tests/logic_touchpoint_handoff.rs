use ouroforge_core::godot_2d_adapter_ir::{parse_godot_2d_source_files, GodotSourceFile};
use ouroforge_core::logic_touchpoint_handoff::{
    detect_godot_logic_touchpoints, validate_logic_touchpoint_handoff, LogicCouplingKind,
    LogicHandoffFidelityGrade, LOGIC_TOUCHPOINT_HANDOFF_BOUNDARY,
    LOGIC_TOUCHPOINT_HANDOFF_SCHEMA_VERSION,
};

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
[gd_scene load_steps=3 format=3 uid="uid://handoff-test"]

[ext_resource type="Texture2D" path="res://player.png" id="tex_player"]

[node name="Root" type="Node2D"]

[node name="Player" type="Sprite2D" parent="."]
position = Vector2({x}, 20)
texture = ExtResource("tex_player")
script = ExtResource("player_script")
script_speed = 120
script_jump_enabled = true

[node name="Hitbox" type="Area2D" parent="Player"]
collision_layer = 1
collision_mask = 1

[node name="Particles" type="GPUParticles2D" parent="."]

[connection signal="pressed" from="Player" to="." method="_on_player_pressed"]
[connection signal="body_entered" from="Hitbox" to="Player" method="_on_hitbox_body_entered"]
"#
            ),
        },
    ]
}

#[test]
fn catalogs_touchpoints_with_couplings_and_era_r_tasks() {
    let ir = parse_godot_2d_source_files("handoff", source_files_with_player_x(10)).unwrap();
    let artifact = detect_godot_logic_touchpoints(&ir).unwrap();

    assert_eq!(
        artifact.schema_version,
        LOGIC_TOUCHPOINT_HANDOFF_SCHEMA_VERSION
    );
    assert_eq!(artifact.boundary, LOGIC_TOUCHPOINT_HANDOFF_BOUNDARY);
    assert!(artifact.state_hash.starts_with("sha256:"));
    assert!(artifact.claimed_ported_units.is_empty());
    assert!(artifact.touchpoints.len() >= 4, "{artifact:#?}");
    assert_eq!(artifact.behavioral_units.len(), artifact.touchpoints.len());
    assert_eq!(artifact.era_r_tasks.len(), artifact.touchpoints.len());
    assert_eq!(
        artifact.oracle_requirements.len(),
        artifact.touchpoints.len()
    );

    assert!(artifact
        .touchpoints
        .iter()
        .any(|tp| tp.trigger_kind == "script-ref"
            && tp.coupling == LogicCouplingKind::Script
            && tp.exported_variables.iter().any(|v| v == "script_speed")));
    assert!(artifact.touchpoints.iter().any(
        |tp| tp.trigger_kind == "signal-connection" && tp.coupling == LogicCouplingKind::Input
    ));
    assert!(artifact
        .touchpoints
        .iter()
        .any(|tp| tp.trigger_kind == "signal-connection"
            && tp.coupling == LogicCouplingKind::Physics));
    assert!(artifact
        .touchpoints
        .iter()
        .any(|tp| tp.trigger_kind == "unsupported-feature"
            && tp.coupling == LogicCouplingKind::Rendering));
    assert!(artifact
        .era_r_tasks
        .iter()
        .all(|task| task.target_era == "Era R"));
}

#[test]
fn no_logic_touchpoint_is_green_or_ported_without_oracle() {
    let ir = parse_godot_2d_source_files("handoff", source_files_with_player_x(10)).unwrap();
    let artifact = detect_godot_logic_touchpoints(&ir).unwrap();

    assert!(artifact.fidelity_report.red > 0);
    assert_eq!(artifact.fidelity_report.green, 0);
    assert!(artifact
        .behavioral_units
        .iter()
        .all(|unit| unit.fidelity_grade == LogicHandoffFidelityGrade::Red
            && unit.oracle_status == "missing"
            && !unit.ported_claim_allowed));
    assert!(artifact
        .oracle_requirements
        .iter()
        .all(|oracle| oracle.status == "missing" && !oracle.ported_claim_allowed));
    assert!(artifact
        .fidelity_report
        .oracle_rule
        .contains("No logic touchpoint is ported"));

    let mut forged = artifact.clone();
    forged
        .claimed_ported_units
        .push("auto-translated:PlayerController".to_string());
    let err = validate_logic_touchpoint_handoff(&forged).unwrap_err();
    assert!(err.to_string().contains("cannot claim ported units"));
}

#[test]
fn handoff_is_deterministic_and_source_drift_changes_state_hash() {
    let first_ir = parse_godot_2d_source_files("handoff", source_files_with_player_x(10)).unwrap();
    let second_ir = parse_godot_2d_source_files("handoff", source_files_with_player_x(10)).unwrap();
    let changed_ir =
        parse_godot_2d_source_files("handoff", source_files_with_player_x(11)).unwrap();

    let first = detect_godot_logic_touchpoints(&first_ir).unwrap();
    let second = detect_godot_logic_touchpoints(&second_ir).unwrap();
    let changed = detect_godot_logic_touchpoints(&changed_ir).unwrap();

    assert_eq!(first, second);
    assert_ne!(first.source_ir_hash, changed.source_ir_hash);
    assert_ne!(first.state_hash, changed.state_hash);

    let mut tampered = first.clone();
    tampered.touchpoints[0].gap_reason.push_str(" tampered");
    let err = validate_logic_touchpoint_handoff(&tampered).unwrap_err();
    assert!(err.to_string().contains("state hash"));
}

#[test]
fn behavior_bearing_green_or_oracle_bypass_fails_closed() {
    let ir = parse_godot_2d_source_files("handoff", source_files_with_player_x(10)).unwrap();
    let mut artifact = detect_godot_logic_touchpoints(&ir).unwrap();

    artifact.behavioral_units[0].fidelity_grade = LogicHandoffFidelityGrade::Green;
    let err = validate_logic_touchpoint_handoff(&artifact).unwrap_err();
    assert!(err.to_string().contains("cannot be green"));

    let mut artifact = detect_godot_logic_touchpoints(&ir).unwrap();
    artifact.oracle_requirements[0].ported_claim_allowed = true;
    let err = validate_logic_touchpoint_handoff(&artifact).unwrap_err();
    assert!(err.to_string().contains("oracle-missing"));
}

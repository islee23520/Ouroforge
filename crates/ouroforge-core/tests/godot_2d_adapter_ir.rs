use ouroforge_core::godot_2d_adapter_ir::{
    parse_godot_2d_project, FidelityGrade, GodotPresentation, GODOT_2D_ADAPTER_BOUNDARY,
    GODOT_2D_ADAPTER_IR_SCHEMA_VERSION,
};

#[test]
fn godot_2d_project_parses_to_deterministic_ir_with_fidelity_report() {
    let root = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/godot-2d-adapter-v1/sample-project"
    );

    let first = parse_godot_2d_project(root).expect("fixture parses");
    let second = parse_godot_2d_project(root).expect("fixture parses deterministically");

    assert_eq!(first, second);
    assert_eq!(first.schema_version, GODOT_2D_ADAPTER_IR_SCHEMA_VERSION);
    assert_eq!(first.boundary, GODOT_2D_ADAPTER_BOUNDARY);
    assert!(first.boundary.contains("one-way"));
    assert!(first.boundary.contains("clean-room"));
    assert_eq!(first.scenes.len(), 1);
    assert_eq!(first.inputs.len(), 2);
    assert!(first
        .source
        .accepted_formats
        .iter()
        .any(|format| format == ".tscn"));
    assert!(first
        .source
        .accepted_formats
        .iter()
        .any(|format| format == ".tres"));

    let scene = &first.scenes[0];
    let player = scene
        .nodes
        .iter()
        .find(|node| node.name == "Player")
        .expect("player node present");
    assert_eq!(player.godot_type, "Sprite2D");
    assert_eq!(player.fidelity_grade, FidelityGrade::Green);
    assert_eq!(player.transform2d.position, Some([10.0, 20.0]));
    assert_eq!(player.transform2d.scale, Some([2.0, 2.0]));
    assert_eq!(player.transform2d.z_index, Some(3));
    assert!(matches!(
        player.presentation,
        Some(GodotPresentation::Sprite { .. })
    ));

    let hitbox = scene
        .nodes
        .iter()
        .find(|node| node.name == "Hitbox")
        .expect("hitbox node present");
    assert_eq!(hitbox.fidelity_grade, FidelityGrade::Yellow);
    assert!(
        hitbox
            .collider
            .as_ref()
            .expect("collider")
            .physics_re_simulated
    );

    assert!(first
        .unsupported
        .iter()
        .any(|feature| feature.feature_kind == "GPUParticles2D"
            && feature.fidelity_grade == FidelityGrade::Red));
    assert!(first.logic_touchpoints.iter().any(|touchpoint| {
        touchpoint.trigger_kind == "script-ref"
            && touchpoint.era_r_status == "requires-clean-room-re-derivation"
            && touchpoint.fidelity_grade == FidelityGrade::Red
    }));
    assert!(first
        .logic_touchpoints
        .iter()
        .any(|touchpoint| touchpoint.trigger_kind == "signal-connection"));
    assert!(first.fidelity_report.summary.green > 0);
    assert!(first.fidelity_report.summary.yellow > 0);
    assert!(first.fidelity_report.summary.red > 0);
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
fn godot_2d_adapter_rejects_shipped_build_artifacts() {
    let err = ouroforge_core::godot_2d_adapter_ir::parse_godot_2d_source_files(
        "bad-project",
        vec![ouroforge_core::godot_2d_adapter_ir::GodotSourceFile {
            path: "export/game.pck".to_string(),
            contents: String::new(),
        }],
    )
    .expect_err("packed builds are rejected");

    assert!(err.to_string().contains("source-project text only"));
}

#[test]
fn godot_2d_adapter_demo_report_is_honest_and_state_hashed() {
    let root = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/godot-2d-adapter-v1/sample-project"
    );

    let first = ouroforge_core::godot_2d_adapter_ir::godot_2d_adapter_demo_report(root)
        .expect("demo report builds");
    let second = ouroforge_core::godot_2d_adapter_ir::godot_2d_adapter_demo_report(root)
        .expect("demo report builds deterministically");

    assert_eq!(first, second);
    assert!(first.ir_state_hash.starts_with("sha256:"));
    assert_eq!(first.claimed_ported_units.len(), 0);
    assert_eq!(first.unsupported_count, 1);
    assert_eq!(first.logic_touchpoint_count, 2);
    assert!(first.oracle_gate.contains("No unit is claimed ported"));
    assert!(first.determinism.contains("same state hash"));
}

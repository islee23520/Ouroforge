use ouroforge_core::gltf_25d_import::{
    example_report_from_fixture, normalize_gltf_25d_import_from_str, write_report_json,
    Gltf25dImportOptions, GLTF_25D_IMPORT_REPORT_SCHEMA_VERSION,
};

#[test]
fn fixture_import_normalizes_geometry_camera_and_fidelity() {
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    report.validate().expect("report validates");
    assert_eq!(report.schema_version, GLTF_25D_IMPORT_REPORT_SCHEMA_VERSION);
    assert_eq!(report.native_scene.scene_kind, "2.5d-presentation");
    assert_eq!(report.native_scene.active_camera_id, "main-ortho");
    assert_eq!(report.native_scene.cameras[0].projection, "orthographic");
    assert_eq!(report.native_scene.cameras[0].orthographic_height, 6.0);
    assert_eq!(report.native_scene.meshes[0].id, "tile-mesh");
    assert_eq!(report.native_scene.meshes[0].fidelity_grade, "green");
    assert!(report
        .fidelity_rows
        .iter()
        .any(|row| row.unit == "extension:VENDOR_custom_shader_note" && row.grade == "yellow"));
    assert!(report
        .re_derivation_tasks
        .iter()
        .any(|task| task.unit == "logic" || task.unit == "physics"));
    assert!(report.state_hash_primary.starts_with("sha256:"));
    assert_eq!(
        report.perceptual_render_secondary.role,
        "secondary corroboration only"
    );
}

#[test]
fn fixture_report_matches_committed_render_smoke_input() {
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    let actual = write_report_json(&report).expect("report json serializes");
    let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/2-5d-gltf-import-v1/fidelity-report.fixture.json");
    let expected = std::fs::read_to_string(fixture_path).expect("committed fixture report exists");
    assert_eq!(actual.trim(), expected.trim());
}

#[test]
fn blocked_source_and_missing_orthographic_camera_fail_closed() {
    let source = r#"{
      "asset": { "version": "2.0" },
      "nodes": [{"name":"Camera","camera":0}],
      "cameras": [{"name":"Perspective","type":"perspective","perspective":{"znear":0.1,"zfar":100}}],
      "meshes": [{"name":"Mesh","primitives":[{"material":0,"mode":4}]}],
      "materials": [{"name":"Mat"}]
    }"#;
    let options = Gltf25dImportOptions {
        source_project_ref: "examples/source-project".to_string(),
        source_path: "examples/source-project/perspective.gltf".to_string(),
        unit_scale: 1.0,
        axis_convention: "gltf-y-up-right-handed-to-ouroforge-presentation".to_string(),
        color_space: "srgb-textures-linear-lighting".to_string(),
        viewport_width: 640,
        viewport_height: 360,
    };
    let error = normalize_gltf_25d_import_from_str(source, options)
        .expect_err("missing orthographic camera fails closed");
    assert!(error.to_string().contains("orthographic camera"));

    let blocked_options = Gltf25dImportOptions {
        source_project_ref: "decompiled shipped-build".to_string(),
        source_path: "bad.gltf".to_string(),
        unit_scale: 1.0,
        axis_convention: "gltf-y-up-right-handed-to-ouroforge-presentation".to_string(),
        color_space: "srgb-textures-linear-lighting".to_string(),
        viewport_width: 640,
        viewport_height: 360,
    };
    let blocked = normalize_gltf_25d_import_from_str(source, blocked_options)
        .expect_err("blocked provenance fails closed");
    assert!(blocked.to_string().contains("source-project/open-format"));
}

#[test]
fn print_fixture_report_for_regeneration() {
    if std::env::var("OUROFORGE_PRINT_GLTF25D_REPORT").is_ok() {
        let report = example_report_from_fixture().expect("fixture glTF normalizes");
        println!("{}", write_report_json(&report).expect("report serializes"));
    }
}

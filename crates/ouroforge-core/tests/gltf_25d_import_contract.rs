use ouroforge_core::gltf_25d_import::{
    example_report_from_fixture, normalize_gltf_25d_import_from_str, write_report_json,
    Gltf25dImportOptions, GLTF_25D_IMPORT_REPORT_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_repo_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|error| panic!("read {path}: {error:#}"))
}

fn read_repo_json(path: &str) -> Value {
    serde_json::from_str(&read_repo_text(path))
        .unwrap_or_else(|error| panic!("parse {path}: {error:#}"))
}

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
    assert_eq!(report.native_scene.presentation_layers.len(), 3);
    assert!(report
        .native_scene
        .presentation_layers
        .iter()
        .all(|layer| layer
            .authority
            .contains("cannot mutate deterministic logic/evidence")));
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
fn billboard_sprite_stack_and_2d_plane_primitives_are_presentation_only() {
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    report.validate().expect("report validates");

    let billboard = report
        .native_scene
        .presentation_layers
        .iter()
        .find(|layer| layer.kind == "billboard")
        .expect("billboard layer");
    assert_eq!(billboard.id, "hero-billboard");
    assert_eq!(billboard.camera_facing, "screen");
    assert_eq!(billboard.axis_lock, "y");
    assert_eq!(billboard.pixel_filter, "nearest");
    assert_eq!(billboard.fidelity_grade, "green");

    let stack = report
        .native_scene
        .presentation_layers
        .iter()
        .find(|layer| layer.kind == "sprite-stack")
        .expect("sprite-stack layer");
    assert_eq!(stack.id, "crate-sprite-stack");
    assert_eq!(stack.stack_slices, ["crate-00", "crate-01", "crate-02"]);
    assert_eq!(stack.alpha_mode, "mask");
    assert_eq!(stack.fidelity_grade, "green");

    assert!(report.fidelity_rows.iter().any(|row| {
        row.unit == "presentation:hero-billboard"
            && row.grade == "green"
            && row
                .reason
                .contains("cannot mutate deterministic logic/evidence")
    }));
    let plane = report
        .native_scene
        .presentation_layers
        .iter()
        .find(|layer| layer.kind == "2d-in-3d-plane")
        .expect("2D-in-3D plane layer");
    assert_eq!(plane.id, "dialogue-plane-2d-in-3d");
    assert_eq!(
        plane.texture_ref.as_deref(),
        Some("textures/dialogue-panel.png")
    );
    assert_eq!(plane.alpha_mode, "blend");
    assert_eq!(plane.pixel_filter, "nearest");

    assert!(report.fidelity_rows.iter().any(|row| {
        row.unit == "presentation:crate-sprite-stack"
            && row.grade == "green"
            && !row.oracle_required
    }));
    assert!(report.fidelity_rows.iter().any(|row| {
        row.unit == "presentation:dialogue-plane-2d-in-3d"
            && row.grade == "green"
            && !row.oracle_required
    }));
}

#[test]
fn fixture_report_matches_committed_render_smoke_input() {
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    let actual = write_report_json(&report).expect("report json serializes");
    let expected = read_repo_text("examples/2-5d-gltf-import-v1/fidelity-report.fixture.json");
    assert_eq!(actual.trim(), expected.trim());
}

#[test]
fn demo_summary_and_script_prove_honest_end_to_end_boundary() {
    let fixture = read_repo_json("examples/2-5d-gltf-import-v1/demo-summary.fixture.json");
    assert_eq!(fixture["schemaVersion"], "gltf-25d-import-demo-v1");
    assert_eq!(fixture["issueRef"], "#2198");
    assert_eq!(fixture["implementationIssueRef"], "#2197");
    assert!(fixture["contractRefs"]
        .as_array()
        .unwrap()
        .iter()
        .any(|value| value == "docs/billboard-sprite-stack-presentation-contract-v1.md"));
    let script_ref = fixture["scriptRef"].as_str().unwrap();
    assert!(
        repo_root().join(script_ref).exists(),
        "missing script {script_ref}"
    );
    assert!(fixture["reportCommand"]
        .as_str()
        .unwrap()
        .contains("migration gltf25d-demo"));
    assert!(fixture["renderCommand"]
        .as_str()
        .unwrap()
        .contains("render-smoke.test.cjs"));
    assert!(fixture["loopCommand"]
        .as_str()
        .unwrap()
        .contains("seeds/migration-demo.yaml"));
    assert!(fixture["boundary"]
        .as_str()
        .unwrap()
        .contains("no auto-port claim"));
    assert!(fixture["boundary"]
        .as_str()
        .unwrap()
        .contains("no Elixir/Phoenix trusted write authority"));

    let script = read_repo_text(script_ref);
    for required in [
        "migration gltf25d-demo",
        "render-smoke.test.cjs",
        "seeds/migration-demo.yaml",
        "stateHashPrimary",
        "perceptualRenderSecondary",
        "reDerivationTasks",
        "no auto-port claim",
        "2d-in-3d-plane",
        "presentationLayers",
    ] {
        assert!(script.contains(required), "script missing {required}");
    }
}

#[test]
fn demo_summary_matches_live_gltf_report_shape() {
    let fixture = read_repo_json("examples/2-5d-gltf-import-v1/demo-summary.fixture.json");
    let expected = &fixture["expectedSummary"];
    let report = example_report_from_fixture().expect("fixture glTF normalizes");
    report.validate().expect("report validates");

    assert_eq!(report.native_scene.scene_kind, expected["sceneKind"]);
    assert_eq!(
        report.native_scene.cameras[0].projection,
        expected["cameraProjection"]
    );
    assert!(
        report
            .fidelity_rows
            .iter()
            .filter(|row| row.grade == "green")
            .count()
            >= expected["minGreenRows"].as_u64().unwrap() as usize
    );
    assert!(
        report
            .fidelity_rows
            .iter()
            .filter(|row| row.grade == "yellow")
            .count()
            >= expected["minYellowRows"].as_u64().unwrap() as usize
    );
    assert_eq!(expected["claimedPortedUnits"], 0);
    assert_eq!(
        !report.re_derivation_tasks.is_empty(),
        expected["hasReDerivationTasks"]
    );
    assert_eq!(
        report.state_hash_primary.starts_with("sha256:"),
        expected["deterministicStateHashPrimary"]
    );
    assert_eq!(
        report.perceptual_render_secondary.role == "secondary corroboration only",
        expected["perceptualRenderSecondaryOnly"]
    );
    assert_eq!(expected["decompiledSourceCopied"], false);
    assert_eq!(expected["elixirOwnsArtifactSemantics"], false);
    assert_eq!(
        report.native_scene.presentation_layers.len(),
        expected["presentationLayerCount"].as_u64().unwrap() as usize
    );
    let kinds = report
        .native_scene
        .presentation_layers
        .iter()
        .map(|layer| layer.kind.as_str())
        .collect::<Vec<_>>();
    let expected_kinds = expected["presentationKinds"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(kinds, expected_kinds);
    assert_eq!(expected["demonstratesM98EndToEnd"], true);
    assert!(report
        .oracle_rule
        .contains("Nothing is claimed ported without captured acceptance evidence"));
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

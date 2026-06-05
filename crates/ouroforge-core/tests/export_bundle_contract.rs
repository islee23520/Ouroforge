//! Web Export Bundle v1 contract (#723).
//!
//! A validated plan assembles a runnable local web bundle into a staging dir:
//! HTML/CSS/runtime bootstrap plus only the planned entry scene and asset roots.
//! Assembly stays inside the staging root, copies no undeclared files, preserves
//! the window.__OUROFORGE__ probe wiring, and refuses blocked source paths.

use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_plan::ExportPlan;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn fixture_profile() -> String {
    std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .expect("fixture profile readable")
}

fn unique_staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-export-bundle-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn plan() -> ExportPlan {
    ExportPlan::from_profile_json(&fixture_profile()).expect("fixture profile plans")
}

#[test]
fn assembles_runnable_bundle_from_plan() {
    let staging = unique_staging("runnable");
    let report = assemble_web_bundle(&plan(), &repo_root(), &staging).expect("bundle assembles");

    // Core runnable surface is present.
    for required in ["index.html", "styles.css", "runtime/bootstrap.js"] {
        assert!(
            report.package_files.iter().any(|p| p == required),
            "missing {required} in {:?}",
            report.package_files
        );
        assert!(report.bundle_root.join(required).is_file());
    }

    // Entry scene was copied into the bundle.
    assert_eq!(report.entry_scene_package_path, "scene/main.json");
    assert!(report.bundle_root.join("scene/main.json").is_file());

    // The single declared asset was copied; nothing undeclared appears.
    assert_eq!(
        report.asset_files,
        vec!["assets/assets/sprites/hero.json".to_string()]
    );
    assert!(report
        .bundle_root
        .join("assets/assets/sprites/hero.json")
        .is_file());

    std::fs::remove_dir_all(&staging).ok();
}

#[test]
fn bootstrap_preserves_probe_wiring() {
    let staging = unique_staging("probe");
    let report = assemble_web_bundle(&plan(), &repo_root(), &staging).expect("bundle assembles");
    let bootstrap = std::fs::read_to_string(report.bundle_root.join("runtime/bootstrap.js"))
        .expect("bootstrap readable");

    assert!(bootstrap.contains("__OUROFORGE__"));
    for method in [
        "getWorldState",
        "getFrameStats",
        "getEvents",
        "step",
        "pause",
        "resume",
        "setInput",
        "snapshot",
        "restore",
    ] {
        assert!(bootstrap.contains(method), "probe missing {method}");
    }
    // No publish/network/command surface leaks into the generated runtime.
    let lower = bootstrap.to_lowercase();
    for forbidden in ["xmlhttprequest", "websocket", "eval(", "child_process"] {
        assert!(!lower.contains(forbidden), "bootstrap leaks {forbidden}");
    }
    assert_eq!(report.probe_mode, "preserve");

    std::fs::remove_dir_all(&staging).ok();
}

#[test]
fn assembly_is_deterministic() {
    let a = unique_staging("det-a");
    let b = unique_staging("det-b");
    let ra = assemble_web_bundle(&plan(), &repo_root(), &a).unwrap();
    let rb = assemble_web_bundle(&plan(), &repo_root(), &b).unwrap();
    assert_eq!(ra.package_files, rb.package_files);
    assert_eq!(ra.asset_files, rb.asset_files);
    // Byte-identical generated runtime.
    let boot_a = std::fs::read(a.join("runtime/bootstrap.js")).unwrap();
    let boot_b = std::fs::read(b.join("runtime/bootstrap.js")).unwrap();
    assert_eq!(boot_a, boot_b);
    std::fs::remove_dir_all(&a).ok();
    std::fs::remove_dir_all(&b).ok();
}

#[test]
fn outputs_stay_inside_staging_root() {
    let staging = unique_staging("contained");
    let report = assemble_web_bundle(&plan(), &repo_root(), &staging).expect("bundle assembles");
    for rel in &report.package_files {
        let joined = report.bundle_root.join(rel);
        let resolved = joined.canonicalize().expect("written file resolves");
        assert!(
            resolved.starts_with(&report.bundle_root),
            "{rel} escaped staging root"
        );
    }
    std::fs::remove_dir_all(&staging).ok();
}

#[test]
fn refuses_blocked_source_path() {
    // A plan whose asset root points at a blocked generated-state prefix can only
    // be built by hand; the bundle must refuse it even if it reaches assembly.
    let mut p = plan();
    p.source_inputs
        .push(ouroforge_core::export_plan::PlannedInput {
            kind: ouroforge_core::export_plan::PlannedInputKind::AssetRoot,
            path: "target/debug/leak".to_string(),
        });
    let staging = unique_staging("blocked");
    let err =
        assemble_web_bundle(&p, &repo_root(), &staging).expect_err("blocked source path refused");
    assert!(err.to_string().contains("blocked source path"));
    std::fs::remove_dir_all(&staging).ok();
}

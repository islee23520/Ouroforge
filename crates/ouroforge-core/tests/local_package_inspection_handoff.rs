//! Local package inspection handoff smoke (#2498).
//!
//! Generates the local web package at the reviewer handoff path from the existing
//! export pipeline, writes package manifest/checksum diagnostics under ignored
//! roots, and verifies the packaged runtime probe without adding signing, store,
//! upload, deploy, native export, or a new export engine.

use ouroforge_core::export_asset_manifest::build_asset_manifest;
use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_fingerprint::build_fingerprint;
use ouroforge_core::export_hash::sha256_hex;
use ouroforge_core::export_plan::ExportPlan;
use ouroforge_core::export_probe_check::{check_bundle_probe, ExportProbeMode};
use ouroforge_core::export_profile::ExportProfile;
use ouroforge_core::export_verification::verify_export_bundle;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn repo_json(rel: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(repo_root().join(rel)).expect("fixture reads"))
        .expect("fixture parses")
}

fn repo_rel_path(rel: &str) -> PathBuf {
    repo_root().join(rel.trim_end_matches('/'))
}

fn collect_files(root: &Path) -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        let mut entries = fs::read_dir(dir)
            .expect("package dir reads")
            .map(|entry| entry.expect("dir entry reads").path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(root, &path, out);
            } else if path.is_file() {
                out.push(
                    path.strip_prefix(root)
                        .expect("file under package root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }

    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}

#[test]
fn local_package_handoff_generates_and_smokes_packaged_artifact() {
    let handoff = repo_json(
        "examples/production-usability-gate-v111/local-package-inspection-handoff.fixture.json",
    );
    assert_eq!(
        handoff["schemaVersion"],
        "local-package-inspection-handoff-v1"
    );
    assert_eq!(handoff["issue"], 2498);
    assert_eq!(handoff["runtimeProbePreserved"], true);

    let export_profile_ref = handoff["exportProfileRef"]
        .as_str()
        .expect("export profile ref");
    let source_project_ref = handoff["sourceProjectRef"]
        .as_str()
        .expect("source project ref");
    let package_metadata_ref = handoff["packageMetadataRef"]
        .as_str()
        .expect("metadata ref");
    let package_root_ref = handoff["packageRoot"].as_str().expect("package root ref");
    let checksums_ref = handoff["checksumsRef"].as_str().expect("checksums ref");
    let package_manifest_ref = handoff["packageManifestRef"]
        .as_str()
        .expect("manifest ref");
    let smoke_ref = handoff["localSmokeEvidenceRef"]
        .as_str()
        .expect("smoke ref");

    for rel in [export_profile_ref, source_project_ref, package_metadata_ref] {
        assert!(
            repo_root().join(rel).is_file(),
            "handoff source ref exists: {rel}"
        );
    }

    let package_root = repo_rel_path(package_root_ref);
    let _ = fs::remove_dir_all(&package_root);

    let profile_text =
        fs::read_to_string(repo_root().join(export_profile_ref)).expect("profile reads");
    let profile = ExportProfile::from_json_str(&profile_text).expect("profile parses");
    let plan = ExportPlan::from_profile(&profile).expect("profile plans");
    let report = assemble_web_bundle(&plan, &repo_root(), &package_root).expect("bundle assembles");
    assert_eq!(
        report.bundle_root,
        package_root.canonicalize().expect("package root resolves")
    );

    let asset_manifest = build_asset_manifest(&plan, &repo_root(), &profile.project_id)
        .expect("asset manifest builds");
    let fingerprint = build_fingerprint(
        &profile,
        &plan,
        &asset_manifest,
        &report.bundle_root,
        "ouroforge-core-0.1.0",
    )
    .expect("fingerprint builds");

    let checksums_path = repo_root().join(checksums_ref);
    let checksum_lines = fingerprint
        .artifact_checksums
        .iter()
        .map(|entry| {
            format!(
                "{}  {}",
                entry.content_hash.trim_start_matches("sha256:"),
                entry.package_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&checksums_path, format!("{checksum_lines}\n")).expect("checksums write");

    let package_manifest = json!({
        "schemaVersion": "local-web-package-manifest-v1",
        "issue": 2498,
        "sourceProjectRef": source_project_ref,
        "exportProfileRef": export_profile_ref,
        "packageMetadataRef": package_metadata_ref,
        "packageRoot": package_root_ref,
        "checksumsRef": checksums_ref,
        "runtimeProbePreserved": true,
        "artifactCount": fingerprint.artifact_checksums.len(),
        "entryScenePackagePath": report.entry_scene_package_path,
        "nonGoals": handoff["nonGoals"].clone()
    });
    fs::write(
        repo_root().join(package_manifest_ref),
        serde_json::to_string_pretty(&package_manifest).expect("manifest serializes"),
    )
    .expect("package manifest writes");

    for rel in [
        "index.html",
        "runtime/bootstrap.js",
        "manifest.json",
        "checksums.sha256",
    ] {
        assert!(
            package_root.join(rel).is_file(),
            "packaged handoff file exists: {rel}"
        );
    }

    let files = collect_files(&package_root);
    for rel in fingerprint
        .artifact_checksums
        .iter()
        .map(|entry| &entry.package_path)
    {
        let bytes = fs::read(package_root.join(rel)).expect("artifact reads");
        let expected = format!("{}  {}", sha256_hex(&bytes), rel);
        assert!(
            checksum_lines.lines().any(|line| line == expected),
            "checksums include {rel}"
        );
    }

    let probe = check_bundle_probe(&package_root, ExportProbeMode::PackagedProbeLimited)
        .expect("packaged probe checks");
    assert!(
        probe.passed,
        "packaged probe failed: {:?}",
        probe.missing_methods
    );
    let verification =
        verify_export_bundle(&plan, &package_root, ExportProbeMode::PackagedProbeLimited);
    assert!(
        verification.passed(),
        "verification should pass from handoff path"
    );

    let smoke = json!({
        "schemaVersion": "local-package-handoff-smoke-v1",
        "issue": 2498,
        "packageRoot": package_root_ref,
        "packageManifestRef": package_manifest_ref,
        "checksumsRef": checksums_ref,
        "runtimeProbePreserved": probe.global_present && probe.passed,
        "probeMode": "packaged-probe-limited",
        "verificationPassed": verification.passed(),
        "artifactCount": files.len(),
        "diagnostics": {
            "missingProbeMethods": probe.missing_methods,
            "requiredFilesPresent": true,
            "sourceProjectRef": source_project_ref,
            "exportProfileRef": export_profile_ref,
            "nonGoals": handoff["nonGoals"].clone()
        }
    });
    let smoke_path = repo_root().join(smoke_ref);
    if let Some(parent) = smoke_path.parent() {
        fs::create_dir_all(parent).expect("smoke parent creates");
    }
    fs::write(
        &smoke_path,
        serde_json::to_string_pretty(&smoke).expect("smoke serializes"),
    )
    .expect("smoke writes");
}

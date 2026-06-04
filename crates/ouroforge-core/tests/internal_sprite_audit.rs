use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ouroforge_core::internal_sprite_audit::{
    audit_internal_sprite_reference, InternalSpriteAuditProfile,
};

#[test]
fn audit_internal_sprite_reference_reports_ready_without_copying_private_sprites() {
    let root = unique_temp_dir("ouroforge-internal-sprite-ready");
    write_png_fixture(
        &root,
        "ro-sprites-anim/body/male/Novice_job0/act0_dir0_f0.png",
    );
    write_png_fixture(
        &root,
        "ro-sprites-anim/body/male/Novice_job0/act1_dir0_f0.png",
    );
    write_png_fixture(
        &root,
        "ro-sprites-anim/body/female/Novice_job0/act0_dir0_f0.png",
    );
    write_png_fixture(
        &root,
        "ro-sprites-anim/body/female/Novice_job0/act1_dir0_f0.png",
    );

    let report = audit_internal_sprite_reference(&root, InternalSpriteAuditProfile::RoVibeV1)
        .expect("audit succeeds");

    assert!(
        report.render_readiness.ready,
        "all required RO-vibe sample frames are present"
    );
    assert_eq!(report.profile, "ro-vibe-v1");
    assert_eq!(report.reference_root, "internal-local-root");
    assert!(report.reference_root_redacted);
    assert_eq!(
        report.distribution_policy.license_scope,
        "internal-use-only"
    );
    assert!(!report.distribution_policy.git_commit_allowed);
    assert!(!report.distribution_policy.screenshot_allowed);
    assert!(!report.distribution_policy.upload_allowed);
    assert_eq!(report.distribution_policy.copied_private_files, 0);
    assert_eq!(report.inventory.png_frames, 4);
    assert!(report.missing_required_files.is_empty());

    fs::remove_dir_all(root).ok();
}

#[test]
fn audit_internal_sprite_reference_lists_missing_frames_as_issue_inputs() {
    let root = unique_temp_dir("ouroforge-internal-sprite-missing");
    write_png_fixture(
        &root,
        "ro-sprites-anim/body/male/Novice_job0/act0_dir0_f0.png",
    );

    let report = audit_internal_sprite_reference(&root, InternalSpriteAuditProfile::RoVibeV1)
        .expect("audit succeeds with readiness diagnostics");

    assert!(
        !report.render_readiness.ready,
        "missing required files block render readiness"
    );
    assert_eq!(report.render_readiness.status, "blocked");
    assert!(report
        .missing_required_files
        .contains(&"ro-sprites-anim/body/female/Novice_job0/act0_dir0_f0.png".to_owned()));
    assert!(report
        .issue_notes
        .iter()
        .any(|note| note.contains("Missing internal sprite reference")));

    fs::remove_dir_all(root).ok();
}

#[test]
fn audit_internal_sprite_reference_rejects_missing_root() {
    let root = unique_temp_dir("ouroforge-internal-sprite-absent");

    let error = audit_internal_sprite_reference(&root, InternalSpriteAuditProfile::RoVibeV1)
        .expect_err("missing root fails closed");

    let message = error.to_string();
    assert!(message.contains("internal sprite reference root not readable"));
    // #979: the failure diagnostic must redact the operator's local root.
    assert!(
        !message.contains(root.to_str().unwrap()),
        "missing-root error leaked the local reference path: {message}"
    );
    assert!(message.contains("internal-local-root"));
}

fn write_png_fixture(root: &Path, relative_path: &str) {
    let path = root.join(relative_path);
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("fixture dirs");
    fs::write(path, b"synthetic-fixture").expect("fixture writes");
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nanos}"))
}

//! Export Artifact Staging and Generated-State Policy v1 contract (#726).

use ouroforge_core::export_staging::{
    is_ignored_by_default, is_within_staging_root, partition_stale_runs, staging_dir_for_run,
    EXPORT_STAGING_ROOT,
};

const POLICY: &str = include_str!("../../../docs/export-staging-policy-v1.md");

#[test]
fn staging_dir_is_run_scoped_under_ignored_target() {
    assert_eq!(EXPORT_STAGING_ROOT, "target/ouroforge/exports");
    let dir = staging_dir_for_run("run_demo").unwrap();
    assert_eq!(dir, "target/ouroforge/exports/run_demo");
    assert!(is_within_staging_root(&dir));
    // Under target/, which .gitignore already ignores.
    assert!(is_ignored_by_default(&dir));
    assert!(dir.starts_with("target/"));
}

#[test]
fn outside_staging_paths_are_not_in_root() {
    assert!(!is_within_staging_root(
        "examples/export-bundle-v1/fixture-game"
    ));
    assert!(!is_within_staging_root("src/main.rs"));
}

#[test]
fn stale_runs_partition_for_cleanup() {
    let existing = vec!["old1".to_string(), "keep".to_string(), "old2".to_string()];
    let (kept, stale) = partition_stale_runs(&existing, &["keep".to_string()]);
    assert_eq!(kept, vec!["keep".to_string()]);
    assert_eq!(stale, vec!["old1".to_string(), "old2".to_string()]);
}

#[test]
fn policy_doc_documents_staging_and_generated_state() {
    assert!(POLICY.contains("target/ouroforge/exports/<run-id>/"));
    assert!(POLICY.contains("ignored by default"));
    assert!(POLICY.contains("fixture-scoped"));
    assert!(POLICY.contains("git status --short --ignored"));
    assert!(POLICY.to_lowercase().contains("stale"));
    assert!(POLICY.contains("#1 remains"));
    assert!(POLICY.contains("#23 remains"));
}

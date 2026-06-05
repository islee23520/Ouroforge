//! Export Artifact Staging and Generated-State Policy v1 (#726).
//!
//! Export runs stage artifacts under a run-scoped directory beneath `target/`,
//! which is already git-ignored, so every generated export artifact is ignored
//! by default. This module computes the staging path, validates run ids, and
//! provides deterministic stale-run partitioning for cleanup.
//!
//! See `docs/export-staging-policy-v1.md` for the full policy.

use anyhow::{anyhow, Result};

/// Root under which every export run stages its artifacts. Lives under
/// `target/`, which `.gitignore` already ignores.
pub const EXPORT_STAGING_ROOT: &str = "target/ouroforge/exports";

/// Git-ignored roots: any path under one of these is ignored by default.
const IGNORED_ROOTS: &[&str] = &["target/", "runs/", ".omo/", "dist/", "build/"];

/// Generated artifact kinds that remain ignored unless fixture-scoped.
pub const GENERATED_ARTIFACT_KINDS: &[&str] = &[
    "bundle-output",
    "asset-payload",
    "asset-manifest",
    "checksums",
    "fingerprint",
    "verification-log",
    "screenshot",
    "world-state",
    "temp-server",
    "tool-state",
];

/// Return the run-scoped staging directory for `run_id`, e.g.
/// `target/ouroforge/exports/<run-id>`.
pub fn staging_dir_for_run(run_id: &str) -> Result<String> {
    validate_run_id(run_id)?;
    Ok(format!("{EXPORT_STAGING_ROOT}/{run_id}"))
}

/// True if `path` is staged under the export staging root.
///
/// Fails closed before the prefix decision: absolute paths, backslash
/// separators, and `..` traversal components are rejected so a path that merely
/// begins with the staging prefix but resolves outside it (e.g.
/// `target/ouroforge/exports/../leak`) is never approved.
pub fn is_within_staging_root(path: &str) -> bool {
    let normalized = path.trim_start_matches("./");
    if normalized.starts_with('/')
        || normalized.contains('\\')
        || normalized.split('/').any(|component| component == "..")
    {
        return false;
    }
    normalized == EXPORT_STAGING_ROOT || normalized.starts_with(&format!("{EXPORT_STAGING_ROOT}/"))
}

/// True if `path` is ignored by default (under a git-ignored root).
pub fn is_ignored_by_default(path: &str) -> bool {
    let normalized = path.trim_start_matches("./");
    IGNORED_ROOTS
        .iter()
        .any(|root| normalized == root.trim_end_matches('/') || normalized.starts_with(root))
}

/// Partition existing run ids into (kept, stale). Run ids not in `keep` are
/// stale and may be pruned. Deterministic: stale ids preserve input order and
/// are de-duplicated against `keep`.
pub fn partition_stale_runs(existing: &[String], keep: &[String]) -> (Vec<String>, Vec<String>) {
    let mut kept = Vec::new();
    let mut stale = Vec::new();
    for run_id in existing {
        if keep.iter().any(|k| k == run_id) {
            kept.push(run_id.clone());
        } else {
            stale.push(run_id.clone());
        }
    }
    (kept, stale)
}

fn validate_run_id(run_id: &str) -> Result<()> {
    if run_id.trim().is_empty()
        || run_id.len() > 96
        || !run_id
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
        || run_id == "."
        || run_id == ".."
    {
        return Err(anyhow!(
            "export staging run id must be a bounded local id (alphanumeric, dash, underscore, dot)"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staging_path_is_run_scoped_and_ignored() {
        let dir = staging_dir_for_run("run_2026_demo").unwrap();
        assert_eq!(dir, "target/ouroforge/exports/run_2026_demo");
        assert!(is_within_staging_root(&dir));
        assert!(is_ignored_by_default(&dir));
    }

    #[test]
    fn unsafe_run_ids_are_rejected() {
        for bad in ["", "..", ".", "a/b", "a\\b", "with space"] {
            assert!(staging_dir_for_run(bad).is_err(), "accepted `{bad}`");
        }
    }

    #[test]
    fn staging_root_membership_fails_closed_on_traversal() {
        // Paths that begin with the staging prefix but escape it must be rejected.
        assert!(!is_within_staging_root("target/ouroforge/exports/../leak"));
        assert!(!is_within_staging_root(
            "target/ouroforge/exports/run/../../leak"
        ));
        assert!(!is_within_staging_root("/target/ouroforge/exports/run"));
        assert!(!is_within_staging_root(
            "target/ouroforge/exports\\..\\leak"
        ));
        // Legitimate staged paths still pass.
        assert!(is_within_staging_root("target/ouroforge/exports"));
        assert!(is_within_staging_root(
            "./target/ouroforge/exports/run_demo/index.html"
        ));
    }

    #[test]
    fn stale_partition_is_deterministic() {
        let existing = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let keep = vec!["b".to_string()];
        let (kept, stale) = partition_stale_runs(&existing, &keep);
        assert_eq!(kept, vec!["b".to_string()]);
        assert_eq!(stale, vec!["a".to_string(), "c".to_string()]);
    }
}

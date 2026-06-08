//! Scope contract coverage for Post-Launch Patch v1 (#1844).

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|error| panic!("read {rel}: {error}"))
}

#[test]
fn post_launch_patch_scope_doc_records_loop_and_reuse_contract() {
    let doc = read("docs/post-launch-patch-v1.md");
    for required in [
        "Issue: **#1844** (#1 Era I Milestone 55)",
        "no executable patching behavior",
        "no new pipeline",
        "update -> re-verify -> re-package loop",
        "A patch **must re-verify before re-packaging**",
        "reuse existing package/export evidence",
        "Scenario Coverage v50",
        "#1844 scope -> #1845 -> #1846 -> #1847 -> #1848 -> #1849",
    ] {
        assert!(
            doc.contains(required),
            "missing scope contract text: {required}"
        );
    }
}

#[test]
fn save_migration_contract_is_versioned_forward_and_fail_closed() {
    let doc = read("docs/post-launch-patch-v1.md");
    for required in [
        "Save schema version",
        "oldest supported baseline save version",
        "Migrations are forward-only, deterministic, and Rust/local-owned",
        "Downgrades are not implied",
        "Unsupported, malformed, stale, missing, or ambiguous saves fail closed",
        "old save hash, new save hash, migration id, target version",
        "cannot pass closure unless migration evidence demonstrates compatibility",
    ] {
        assert!(
            doc.contains(required),
            "missing migration contract: {required}"
        );
    }
}

#[test]
fn ownership_and_anchor_governance_remain_conservative() {
    let doc = read("docs/post-launch-patch-v1.md");
    for required in [
        "**Rust/local owns** trusted validation, persistence, source-apply/review/apply/trust-gradient, save migration",
        "**TypeScript/JavaScript owns** deterministic runtime behavior",
        "Browser, Studio, dashboard, cockpit, Electron, Steamworks, and generated surfaces remain read-only for trusted state",
        "#1 remains open",
        "#23 remains open",
        "Steam upload, code signing, content survey, Release button, support obligations, and market demand remain human/Ring-3",
        "Generated runs/artifacts/builds/migration outputs remain untracked unless explicitly fixture-scoped",
    ] {
        assert!(doc.contains(required), "missing governance boundary: {required}");
    }
}

#[test]
fn forbidden_post_launch_overclaims_are_absent() {
    let doc = read("docs/post-launch-patch-v1.md");
    for forbidden in [
        "production-ready engine",
        "is a Godot replacement",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "automated fun score is accepted",
        "Release button is automated",
        "Layer-3 cloud/mobile is GO",
        "browser trusted writes are allowed",
    ] {
        assert!(
            !doc.contains(forbidden),
            "forbidden wording leaked: {forbidden}"
        );
    }
}

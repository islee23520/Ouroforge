//! Governance contract for the Export Target Matrix v1 (#720).
//!
//! The matrix is the fail-closed source of truth for which export targets the
//! Build / Export / Packaging v1 milestone may produce. These checks assert the
//! matrix documents an explicit status for every governed target, blocks every
//! publish/release/deploy/sign target, distinguishes a local playable package
//! from a public release, and keeps the #1 / #23 governance anchors open.

const MATRIX: &str = include_str!("../../../docs/export-target-matrix-v1.md");
const SCOPE: &str = include_str!("../../../docs/build-export-packaging-v1.md");

#[test]
fn matrix_declares_allowed_v1_web_targets() {
    assert!(MATRIX.contains("`web-local`"));
    assert!(MATRIX.contains("`web-static-bundle`"));
    // Both allowed targets appear in an `allowed` row of the matrix table.
    assert!(MATRIX.contains("| `web-local` | allowed |"));
    assert!(MATRIX.contains("| `web-static-bundle` | allowed |"));
}

#[test]
fn matrix_gates_desktop_wrapper_as_future_not_implemented() {
    assert!(MATRIX.contains("| `desktop-wrapper`"));
    assert!(MATRIX.contains("future"));
    assert!(MATRIX.contains("design-gated") || MATRIX.contains("design gate"));
    // The future target must not be treated as authorized: it is validated like
    // a blocked target until a separate design-gate issue scopes it.
    assert!(MATRIX.contains("treats it the same as a blocked target")
        || MATRIX.contains("the same as a blocked target"));
}

#[test]
fn matrix_blocks_every_publish_release_target() {
    for target in [
        "mobile",
        "console",
        "app-store",
        "steam",
        "itch",
        "hosted-deploy",
        "signed-release",
        "ci-release",
    ] {
        assert!(
            MATRIX.contains(&format!("| `{target}`")),
            "matrix is missing a row for blocked target `{target}`"
        );
    }
    assert!(MATRIX.contains("blocked"));
}

#[test]
fn matrix_blocks_publish_deploy_sign_credentials_and_ci_release() {
    let lower = MATRIX.to_lowercase();
    for term in [
        "publish",
        "deploy",
        "signing",
        "notariz",
        "credential",
        "upload",
        "ci release",
    ] {
        assert!(lower.contains(term), "matrix must address `{term}`");
    }
    assert!(MATRIX.contains("fail closed") || MATRIX.contains("fail-closed"));
}

#[test]
fn matrix_distinguishes_local_package_from_public_release() {
    assert!(MATRIX.contains("local, evidence-backed") || MATRIX.contains("local, evidence backed"));
    assert!(MATRIX.contains("public release"));
    assert!(MATRIX.contains("Producing a local artifact is not releasing it"));
}

#[test]
fn matrix_narrows_prior_release_export_mutation_blocker() {
    assert!(MATRIX.contains("source-apply"));
    assert!(MATRIX.contains("Now allowed:"));
    assert!(MATRIX.contains("Still blocked:"));
    assert!(MATRIX.contains("release/publish mutation"));
}

#[test]
fn matrix_keeps_governance_anchors_open() {
    assert!(MATRIX.contains("#1 remains"));
    assert!(MATRIX.contains("#23 remains"));
    assert!(MATRIX.contains("remains open"));
}

#[test]
fn scope_doc_references_the_matrix() {
    assert!(SCOPE.contains("docs/export-target-matrix-v1.md"));
}

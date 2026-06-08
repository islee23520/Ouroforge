//! Governance refresh contract for Scaled Trust Gradient, Release Provenance and
//! Compliance v1 (#1696).
//!
//! This is documentation-only: Era H Milestone 44 is complete on merged
//! evidence, while release authority, high-risk/source-affecting auto-apply, and
//! browser/Studio trusted writes remain blocked.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn roadmap() -> String {
    std::fs::read_to_string(repo_root().join("docs/roadmap.md")).expect("roadmap exists")
}

#[test]
fn roadmap_records_milestone_44_completion_on_merged_evidence() {
    let doc = roadmap();
    for required in [
        "Scaled Trust Gradient, Release Provenance and Compliance v1 governance refresh",
        "complete for\nEra H Milestone 44",
        "#1689\n(`docs/release-trust-provenance-v1.md`, PR #1702)",
        "#1690 (`crates/ouroforge-core/src/release_auto_apply.rs`,\nPR #1906)",
        "#1691\n(`crates/ouroforge-core/src/release_provenance_bundle.rs`, PR #1910)",
        "#1693\n(`crates/ouroforge-core/src/release_compliance_gate.rs`, PR #1956)",
        "#1694 (`docs/release-trust-provenance-v1-demo.md`, PR #1964)",
        "Scenario Coverage v41 #1695 (`docs/scenario-coverage-v41.md`, PR\n#1973)",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(
            doc.contains(required),
            "missing roadmap evidence: {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_release_trust_boundaries() {
    let doc = roadmap();
    for required in [
        "High-risk and source-affecting\nchanges **never auto-apply**",
        "Release requires compliance plus a\nhuman go/no-go",
        "missing license, policy, age-rating, provenance, artifact, or\nhuman approval evidence blocks",
        "Rust/local owns trusted\nvalidation, persistence, provenance/compliance logic",
        "Browser, Studio, dashboard,\nand cockpit surfaces remain deterministic/read-only inspection surfaces",
        "Generated runs/assets/content/release artifacts remain ignored unless explicitly\nfixture-scoped",
        "Shipping, hosted/cloud, real-player telemetry, and live-ops stay\nLayer-3 gated",
    ] {
        assert!(doc.contains(required), "missing boundary: {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_authority_and_maturity_claims_out_of_milestone_44() {
    let doc = roadmap();
    let start = doc
        .find("Scaled Trust Gradient, Release Provenance and Compliance v1 governance refresh")
        .expect("milestone 44 section exists");
    let end = doc[start..]
        .find("### Autonomous Producer")
        .map(|offset| start + offset)
        .expect("next governance section exists");
    let section = &doc[start..end];

    for required in [
        "gain no trusted writes, command bridges, autonomous apply,\nauto-merge, self-approval, reviewer bypass, or hidden mutation authority",
        "no production-ready, quality/fun, Godot\nreplacement/parity, or autonomous-shipping claim is introduced",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(section.contains(required), "missing forbidden boundary: {required}");
    }

    for forbidden_positive_claim in [
        "high-risk and source-affecting changes may auto-apply",
        "release can proceed without compliance",
        "release can proceed without a human go/no-go",
        "browser/Studio trusted writes are authorized",
        "auto-merge is authorized",
        "production-ready claim is introduced",
        "Godot replacement claim is introduced",
    ] {
        assert!(
            !section.contains(forbidden_positive_claim),
            "forbidden positive claim leaked: {forbidden_positive_claim}"
        );
    }
}

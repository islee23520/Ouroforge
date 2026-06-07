//! Contract for the OSS Trust Charter and Paid-Cloud Boundary Design Gate v1
//! (#1618, Era F Milestone 34). This is a design-gate/posture doc with no
//! executable behavior; the test pins the third-rails, the per-surface
//! DEFER decision tied to #1508, and the #1/#23 governance audit so the
//! charter cannot silently drift into a paywall, a relicense, or a hosted
//! capability claim.

const CHARTER: &str = include_str!("../../../docs/oss-trust-charter.md");
const DOCS_INDEX: &str = include_str!("../../../docs/README.md");
const LICENSE: &str = include_str!("../../../LICENSE");

/// The five operational/team/scale surfaces the charter may ever monetize.
const PAID_CLOUD_SURFACES: [&str; 5] = [
    "Hosted, searchable evidence-history service",
    "Multi-seat / team collaboration accounts",
    "Managed CI runners for the verification loop",
    "Managed agent compute (hosted generation/evolve runners)",
    "Marketplace take-rate (optional listing/distribution convenience)",
];

#[test]
fn charter_reaffirms_third_rails_and_permissive_licensing() {
    // Permissive OSS licensing pledge, anchored to the real MIT LICENSE file.
    assert!(LICENSE.contains("MIT License"), "core LICENSE must be MIT");
    assert!(CHARTER.contains("Permissive OSS licensing"));
    assert!(CHARTER.contains("MIT/Apache-2.0"));

    // The four non-negotiable third-rails.
    assert!(CHARTER.contains("No-relicense pledge"));
    assert!(CHARTER.contains("No-runtime-fee pledge"));
    assert!(CHARTER.contains("No-install-fee pledge"));
    assert!(CHARTER.contains("No-revenue-share pledge"));
    assert!(CHARTER.contains("No creative-primitive paywall"));
    assert!(CHARTER.contains("Third-rails (non-negotiable)"));

    // Foundation/governance consideration is recorded.
    assert!(CHARTER.contains("Foundation / governance consideration"));
}

#[test]
fn charter_records_per_surface_defer_tied_to_1508() {
    // The gate exists and DEFER is the default.
    assert!(CHARTER.contains("Paid-cloud boundary design gate"));
    assert!(CHARTER.contains("DEFER is the default and remains in force."));

    // Each paid-cloud surface is present and gated on a #1508 Layer-3 GO.
    for surface in PAID_CLOUD_SURFACES {
        assert!(
            CHARTER.contains(surface),
            "charter missing surface: {surface}"
        );
    }

    // Every surface row carries DEFER tied to a #1508 Layer-3 hosted/cloud GO;
    // there must be one such row per surface and no surface marked GO.
    let defer_rows = CHARTER
        .matches("**DEFER** | a #1508 Layer-3 hosted/cloud GO")
        .count();
    assert_eq!(
        defer_rows,
        PAID_CLOUD_SURFACES.len(),
        "expected one DEFER-tied-to-#1508 row per paid-cloud surface"
    );

    // No paid-cloud surface is approved by this gate.
    assert!(
        !CHARTER.contains("| **GO** |"),
        "no paid-cloud surface may be marked GO by this gate"
    );

    // Layer-3 hosted/cloud is on record as DEFER (the gating dependency).
    assert!(CHARTER.contains("layer3-reevaluation-v1.md"));
    assert!(CHARTER.contains("#1508"));
}

#[test]
fn charter_reaffirms_free_local_core_absent_layer3_go() {
    assert!(CHARTER.contains("Absent a Layer-3 GO: free local OSS core only"));
    assert!(CHARTER.contains("only the free local OSS core remains"));
    assert!(CHARTER.contains("no creative primitive is ever paywalled"));
    // Generation stays proposal-only through the existing trusted path.
    assert!(CHARTER.contains("review/apply/trust-gradient"));
    assert!(CHARTER.contains("browser/Studio surfaces read-only"));
}

#[test]
fn charter_preserves_anchors_and_is_indexed() {
    assert!(CHARTER.contains("#1 remains open"));
    assert!(CHARTER.contains("#23 remains open"));
    assert!(CHARTER.contains("#1 / #23 governance audit"));
    // The docs index references the charter.
    assert!(DOCS_INDEX.contains("oss-trust-charter.md"));
}

#[test]
fn charter_keeps_conservative_wording() {
    let lower = CHARTER.to_ascii_lowercase();
    // Conservative boundary vocabulary is present.
    for required in [
        "defer",
        "additive and backward-compatible",
        "no new engine",
        "distributed/elixir remains no-go",
        "fixture-scoped",
        "read-only",
    ] {
        assert!(lower.contains(required), "missing boundary: {required}");
    }

    // No overclaim or hidden-trusted-write wording asserted as a capability.
    for forbidden in [
        "production-ready",
        "godot replacement is implemented",
        "auto-merge enabled",
        "auto-apply enabled",
        "self-approval enabled",
        "revenue share enabled",
        "runtime fee enabled",
        "hosted service enabled",
        "cloud runtime enabled",
        "creative primitive paywall enabled",
    ] {
        assert!(!lower.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

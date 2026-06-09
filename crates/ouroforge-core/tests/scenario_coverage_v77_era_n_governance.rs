const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const ASSESSMENT: &str = include_str!("../../../docs/era-n-adoption-ux-assessment.md");
const COVERAGE: &str = include_str!("../../../docs/scenario-coverage-v77-era-n-governance.md");
const V72: &str =
    include_str!("../../../docs/scenario-coverage-v72-non-developer-generative-front-door.md");
const V73: &str = include_str!("../../../docs/scenario-coverage-v73-onboarding-first-run.md");
const V74: &str = include_str!(
    "../../../docs/scenario-coverage-v74-studio-accessibility-i18n-themes-keyboard.md"
);
const V75: &str = include_str!("../../../docs/scenario-coverage-v75-human-grade-studio.md");
const V76: &str = include_str!("../../../docs/scenario-coverage-v76-studio-local-delivery.md");

#[test]
fn v77_records_era_n_completion_on_merged_evidence() {
    for needle in [
        "Scenario Coverage v77",
        "Era N is complete only on merged evidence",
        "M82-M86",
        "M87 governance refresh",
        "#1 and #23 remain open",
    ] {
        assert!(
            COVERAGE.contains(needle),
            "missing v77 governance token: {needle}"
        );
    }

    for needle in [
        "Era N is recorded complete on merged evidence",
        "M82 non-developer generative front door",
        "M83 onboarding/templates/docs",
        "M84 accessibility/i18n/themes/keyboard",
        "M85 Human-Grade Studio",
        "M86 local Studio packaging/delivery",
        "Scenario Coverage v77",
        "#1 and #23 remain open",
    ] {
        assert!(
            ROADMAP.contains(needle),
            "missing roadmap completion token: {needle}"
        );
    }
}

#[test]
fn adoption_assessment_preserves_gate_backed_newcomer_path() {
    for needle in [
        "Newcomer Time-to-First-Verified-Game Assessment",
        "guided front door",
        "template gallery",
        "first-run docs",
        "Validate through the Rust data plane",
        "gate-backed",
        "not a public launch claim",
        "not a no-code product claim",
        "not a release-readiness claim",
    ] {
        assert!(
            ASSESSMENT.contains(needle),
            "missing adoption assessment token: {needle}"
        );
    }
}

#[test]
fn accessibility_and_studio_governance_do_not_bypass_the_two_plane_boundary() {
    for needle in [
        "Accessibility and onboarding lowered friction without weakening the core",
        "do not become artifact truth",
        "Every write-affecting action routes through existing Rust-owned",
        "No raw bypass",
        "command bridge",
        "direct Elixir artifact write",
        "new data store",
        "hosted collaboration",
        "mandatory human dependency",
        "Rust remains the data plane",
        "Elixir/OTP + Phoenix LiveView remains the local single-user control/presentation plane",
        "Hosted/multi-user/collaborative Studio remains Layer-3 DEFER",
    ] {
        assert!(
            ASSESSMENT.contains(needle),
            "missing no-bypass token: {needle}"
        );
    }
}

#[test]
fn era_n_coverage_chain_v72_to_v76_is_present() {
    for (label, doc) in [
        ("v72", V72),
        ("v73", V73),
        ("v74", V74),
        ("v75", V75),
        ("v76", V76),
    ] {
        assert!(
            doc.contains("Coverage"),
            "{label} coverage doc missing Coverage heading"
        );
        assert!(
            doc.contains("read + gated-write") || doc.contains("gated"),
            "{label} missing gated boundary"
        );
        assert!(
            doc.contains("autonomous") || doc.contains("CLI fallback"),
            "{label} missing autonomy/fallback boundary"
        );
    }
}

#[test]
fn v77_forbids_marketing_scope_drift_and_anchor_closure() {
    for forbidden in [
        "no-code product claim accepted",
        "hosted Studio implemented",
        "release go/no-go automated",
        "#1 closed",
        "#23 closed",
    ] {
        assert!(
            !COVERAGE.contains(forbidden),
            "forbidden governance drift present: {forbidden}"
        );
        assert!(
            !ASSESSMENT.contains(forbidden),
            "forbidden assessment drift present: {forbidden}"
        );
    }
}

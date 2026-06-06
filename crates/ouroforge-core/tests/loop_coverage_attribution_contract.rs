use ouroforge_core::{
    validate_loop_coverage_attribution, LoopCoverageAttributionArtifact,
    LoopCoverageAttributionStatus, LoopCoverageProvenanceClass,
    LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> LoopCoverageAttributionArtifact {
    let path = workspace_path(&format!("examples/loop-coverage-attribution-v1460/{name}"));
    let text = std::fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn attribution_accepts_each_class_and_explicit_non_classified_state() {
    let cases = [
        (
            "loop-produced.fixture.json",
            LoopCoverageAttributionStatus::Classified,
            Some(LoopCoverageProvenanceClass::LoopProduced),
        ),
        (
            "loop-verified.fixture.json",
            LoopCoverageAttributionStatus::Classified,
            Some(LoopCoverageProvenanceClass::LoopVerified),
        ),
        (
            "manual.fixture.json",
            LoopCoverageAttributionStatus::Classified,
            Some(LoopCoverageProvenanceClass::Manual),
        ),
        (
            "default-manual.fixture.json",
            LoopCoverageAttributionStatus::Classified,
            Some(LoopCoverageProvenanceClass::Manual),
        ),
        (
            "missing-provenance.fixture.json",
            LoopCoverageAttributionStatus::MissingProvenance,
            None,
        ),
        (
            "ambiguous.fixture.json",
            LoopCoverageAttributionStatus::Ambiguous,
            None,
        ),
        (
            "stale-ref.fixture.json",
            LoopCoverageAttributionStatus::StaleRef,
            None,
        ),
        (
            "unsupported-kind.fixture.json",
            LoopCoverageAttributionStatus::UnsupportedKind,
            None,
        ),
    ];
    for (name, status, class) in cases {
        let read_model = validate_loop_coverage_attribution(&fixture(name)).expect(name);
        assert_eq!(
            read_model.schema_version,
            LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION
        );
        assert_eq!(read_model.status, status, "{name}");
        assert_eq!(read_model.provenance_class, class, "{name}");
        assert!(read_model.boundary.contains("descriptive"));
        assert!(read_model.boundary.contains("not quality"));
        assert!(read_model.boundary.contains("no auto-apply"));
        assert!(read_model.boundary.contains("read-only"));
    }
}

#[test]
fn attribution_rejects_overclaims_and_unblocked_non_classified_states() {
    for name in [
        "invalid/overclaim-loop-produced-as-manual.fixture.json",
        "invalid/ambiguous-without-blocker.fixture.json",
    ] {
        assert!(
            validate_loop_coverage_attribution(&fixture(name)).is_err(),
            "{name} should fail"
        );
    }
}

#[test]
fn attribution_validates_generated_state_governance_and_m20_boundary_docs() {
    let contract = include_str!("../../../docs/loop-coverage-metric-v1.md");
    assert!(contract.contains("Milestone 20 records only per-artifact provenance class"));
    assert!(contract.contains("Milestone 25 owns the full intent-to-promotion provenance bundle"));
    assert!(contract.contains("Browser, dashboard, and Studio surfaces"));
    assert!(contract.contains("#1 and #23 remain open"));
    for name in [
        "loop-produced.fixture.json",
        "loop-verified.fixture.json",
        "manual.fixture.json",
        "missing-provenance.fixture.json",
        "ambiguous.fixture.json",
        "stale-ref.fixture.json",
        "unsupported-kind.fixture.json",
    ] {
        let text = std::fs::read_to_string(workspace_path(&format!(
            "examples/loop-coverage-attribution-v1460/{name}"
        )))
        .expect("fixture exists");
        assert!(text.contains("descriptive authorship attribution only"));
        assert!(!text.contains("production-ready"));
        assert!(!text.contains("Godot replacement"));
        assert!(!text.contains("auto-merge authority"));
    }
}

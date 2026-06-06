use ouroforge_core::{
    compute_loop_coverage_evidence, validate_loop_coverage_evidence, LoopCoverageAttributionStatus,
    LoopCoverageEvidenceArtifact, LoopCoverageProvenanceClass, LoopCoverageVerdictState,
    LOOP_COVERAGE_METRIC_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> LoopCoverageEvidenceArtifact {
    let path = workspace_path(&format!("examples/loop-coverage-v1/fixtures/{name}"));
    let text = std::fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn metric_accepts_computed_insufficient_regressed_and_unsupported_fixtures() {
    let cases = [
        ("computed-current.json", LoopCoverageVerdictState::Computed),
        (
            "insufficient-no-baseline.json",
            LoopCoverageVerdictState::InsufficientData,
        ),
        (
            "manual-drop-regressed.json",
            LoopCoverageVerdictState::Regressed,
        ),
        (
            "unsupported-kind.json",
            LoopCoverageVerdictState::Unsupported,
        ),
        ("stale-ref.json", LoopCoverageVerdictState::InsufficientData),
    ];
    for (name, state) in cases {
        let artifact = fixture(name);
        validate_loop_coverage_evidence(&artifact).expect(name);
        assert_eq!(artifact.schema_version, LOOP_COVERAGE_METRIC_SCHEMA_VERSION);
        assert_eq!(artifact.verdict.state, state, "{name}");
        assert!(artifact.boundary.contains("descriptive"));
        assert!(artifact.boundary.contains("not quality"));
        assert!(artifact.boundary.contains("no auto-apply"));
        assert!(artifact.boundary.contains("read-only"));
    }
}

#[test]
fn metric_computes_counts_fractions_and_regression_against_baseline() {
    let baseline = fixture("baseline-loop-covered.json");
    let current = compute_loop_coverage_evidence(
        Some("signal-gate".to_string()),
        Some("manual-drop".to_string()),
        Some("examples/loop-coverage-v1/fixtures/baseline-loop-covered.json".to_string()),
        vec![
            ouroforge_core::LoopCoverageMetricInput {
                artifact_ref: "runs/manual-drop/evidence/scene.json".to_string(),
                artifact_kind: "trusted-change".to_string(),
                source_ref: None,
                provenance_class: Some(LoopCoverageProvenanceClass::LoopProduced),
                attribution_status: Some(LoopCoverageAttributionStatus::Classified),
                loop_stage_refs: Default::default(),
                trusted_validation_ref: Some("runs/manual-drop/verdict.json".to_string()),
                notes: None,
            },
            ouroforge_core::LoopCoverageMetricInput {
                artifact_ref: "runs/manual-drop/evidence/manual-tweak.json".to_string(),
                artifact_kind: "trusted-change".to_string(),
                source_ref: None,
                provenance_class: Some(LoopCoverageProvenanceClass::Manual),
                attribution_status: Some(LoopCoverageAttributionStatus::Classified),
                loop_stage_refs: Default::default(),
                trusted_validation_ref: Some("runs/manual-drop/verdict.json".to_string()),
                notes: None,
            },
        ],
        Some(&baseline),
        0.10,
    );
    validate_loop_coverage_evidence(&current).expect("computed evidence validates");
    assert_eq!(current.counts.loop_produced, 1);
    assert_eq!(current.counts.loop_verified, 0);
    assert_eq!(current.counts.manual, 1);
    assert_eq!(current.verdict.state, LoopCoverageVerdictState::Regressed);
}

#[test]
fn metric_fails_closed_for_missing_attribution_empty_inputs_and_malformed_baseline() {
    let missing = compute_loop_coverage_evidence(
        None,
        Some("missing".to_string()),
        None,
        vec![ouroforge_core::LoopCoverageMetricInput {
            artifact_ref: "runs/missing/evidence/a.json".to_string(),
            artifact_kind: "trusted-change".to_string(),
            source_ref: None,
            provenance_class: None,
            attribution_status: Some(LoopCoverageAttributionStatus::MissingProvenance),
            loop_stage_refs: Default::default(),
            trusted_validation_ref: None,
            notes: None,
        }],
        None,
        0.05,
    );
    assert_eq!(
        missing.verdict.state,
        LoopCoverageVerdictState::InsufficientData
    );
    validate_loop_coverage_evidence(&missing)
        .expect("missing attribution still validates as evidence");

    let empty =
        compute_loop_coverage_evidence(None, Some("empty".to_string()), None, vec![], None, 0.05);
    assert_eq!(
        empty.verdict.state,
        LoopCoverageVerdictState::InsufficientData
    );
    validate_loop_coverage_evidence(&empty)
        .expect("empty inputs are explicit insufficient-data evidence");

    let mut malformed = fixture("baseline-loop-covered.json");
    malformed.counts.manual += 1;
    assert!(validate_loop_coverage_evidence(&malformed).is_err());
}

#[test]
fn metric_docs_and_fixtures_keep_public_claims_conservative() {
    let demo = include_str!("../../../docs/loop-coverage-metric-v1-demo.md");
    let scenario = include_str!("../../../docs/scenario-coverage-v21.md");
    let governance = include_str!("../../../docs/loop-coverage-metric-v1-governance-handoff.md");
    for text in [demo, scenario, governance] {
        assert!(text.contains("#1 and #23 remain open"));
        assert!(text.contains("read-only"));
        assert!(text.contains("not a quality guarantee"));
        assert!(text.contains("no production-ready"));
        assert!(text.contains("no Godot replacement"));
        assert!(text.contains("no auto-apply"));
        assert!(text.contains("no auto-merge"));
    }
}

#[test]
fn metric_rejects_non_finite_or_negative_drop_threshold() {
    let baseline = fixture("baseline-loop-covered.json");
    let inputs = || {
        vec![ouroforge_core::LoopCoverageMetricInput {
            artifact_ref: "runs/bad-threshold/evidence/scene.json".to_string(),
            artifact_kind: "trusted-change".to_string(),
            source_ref: None,
            provenance_class: Some(LoopCoverageProvenanceClass::Manual),
            attribution_status: Some(LoopCoverageAttributionStatus::Classified),
            loop_stage_refs: Default::default(),
            trusted_validation_ref: Some("runs/bad-threshold/verdict.json".to_string()),
            notes: None,
        }]
    };

    // A negative or non-finite tolerance must never yield a regressed/computed
    // verdict; the metric refuses to decide and drops the invalid threshold.
    for threshold in [-0.1_f64, f64::NAN, f64::INFINITY] {
        let evidence = compute_loop_coverage_evidence(
            None,
            Some("bad-threshold".to_string()),
            None,
            inputs(),
            Some(&baseline),
            threshold,
        );
        assert_eq!(
            evidence.verdict.state,
            LoopCoverageVerdictState::InsufficientData,
            "threshold {threshold} must not produce a regression decision"
        );
        assert_eq!(evidence.verdict.drop_threshold, None);
        validate_loop_coverage_evidence(&evidence).expect("safe evidence still validates");
    }

    // An externally-crafted artifact with an invalid stored threshold fails closed.
    let mut tampered = fixture("computed-current.json");
    tampered.verdict.drop_threshold = Some(-0.5);
    assert!(validate_loop_coverage_evidence(&tampered).is_err());
}

#[test]
fn metric_treats_unsupported_artifact_kind_as_unsupported_even_if_marked_classified() {
    let baseline = fixture("baseline-loop-covered.json");
    // A stale/mislabeled input: unsupported artifact kind but marked classified
    // with a provenance class must not be counted as normal coverage (#1464).
    let evidence = compute_loop_coverage_evidence(
        None,
        Some("bypass".to_string()),
        None,
        vec![ouroforge_core::LoopCoverageMetricInput {
            artifact_ref: "runs/bypass/evidence/freeform.txt".to_string(),
            artifact_kind: "freeform-note".to_string(),
            source_ref: None,
            provenance_class: Some(LoopCoverageProvenanceClass::LoopProduced),
            attribution_status: Some(LoopCoverageAttributionStatus::Classified),
            loop_stage_refs: Default::default(),
            trusted_validation_ref: Some("runs/bypass/verdict.json".to_string()),
            notes: None,
        }],
        Some(&baseline),
        0.05,
    );
    assert_eq!(
        evidence.verdict.state,
        LoopCoverageVerdictState::Unsupported
    );
    // The mislabeled input is not counted as trusted coverage.
    assert_eq!(evidence.counts.total_trusted, 0);
    assert_eq!(evidence.counts.loop_produced, 0);
    validate_loop_coverage_evidence(&evidence).expect("unsupported-kind evidence validates");
}

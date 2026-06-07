use ouroforge_core::production_qa_matrix::ProductionQaMatrixArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/production-qa-matrix-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn valid_fixtures_validate_and_roll_up_status() {
    for (name, expected) in [
        ("matrix.complete.fixture.json", "complete"),
        ("matrix.regression.fixture.json", "complete"),
        ("matrix.partial.fixture.json", "partial"),
        ("matrix.stale.fixture.json", "stale"),
    ] {
        let artifact = ProductionQaMatrixArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_status(), expected, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.status, expected, "{name}");
        assert_eq!(read_model.cell_count, artifact.cells.len(), "{name}");
        assert!(
            artifact.dashboard_compat.read_only,
            "{name} must be read-only"
        );
        // The verdict is descriptive, not a quality guarantee.
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("descriptive")));
        // Reuse statement is explicit: regression matrix over existing runners.
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no new test engine")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-merge enabled"));
    }
}

#[test]
fn complete_fixture_aggregates_full_coverage() {
    let artifact =
        ProductionQaMatrixArtifact::from_json_str(&read_fixture("matrix.complete.fixture.json"))
            .expect("complete");
    let read_model = artifact.read_model();
    assert_eq!(read_model.cell_count, 8);
    assert_eq!(read_model.expected_cell_count, 8);
    assert!(read_model.coverage_complete);
    assert_eq!(read_model.variant_count, 2);
    assert_eq!(read_model.seed_count, 2);
    assert_eq!(read_model.target_count, 2);
    assert_eq!(read_model.verdict_counts.get("passed"), Some(&8));
    assert_eq!(read_model.regression_count, 0);
}

#[test]
fn complete_fixture_yields_deterministic_runner_work_items() {
    let artifact =
        ProductionQaMatrixArtifact::from_json_str(&read_fixture("matrix.complete.fixture.json"))
            .expect("complete");
    let plan = artifact.execution_plan().expect("execution plan");
    let mut reversed = artifact.clone();
    reversed.cells.reverse();
    let reversed_plan = reversed.execution_plan().expect("reversed execution plan");
    assert_eq!(plan, reversed_plan);

    assert_eq!(plan.expected_work_item_count, 8);
    assert_eq!(plan.work_items.len(), 8);
    assert_eq!(plan.detected_regressions.len(), 0);
    assert!(plan.reuse_boundary.contains("qaRunMatrixRef"));
    assert!(plan.reuse_boundary.contains("no new engine"));

    let coordinates: Vec<_> = plan
        .work_items
        .iter()
        .map(|item| {
            (
                item.content_variant.as_str(),
                item.seed.as_str(),
                item.target.as_str(),
            )
        })
        .collect();
    assert_eq!(
        coordinates,
        vec![
            ("base", "seed-1", "web"),
            ("base", "seed-1", "desktop"),
            ("base", "seed-2", "web"),
            ("base", "seed-2", "desktop"),
            ("variant-a", "seed-1", "web"),
            ("variant-a", "seed-1", "desktop"),
            ("variant-a", "seed-2", "web"),
            ("variant-a", "seed-2", "desktop"),
        ]
    );
    assert!(plan
        .work_items
        .iter()
        .all(|item| item.runner_ref.contains("qa-swarm-run-matrix-v1")));
    assert!(plan
        .work_items
        .iter()
        .all(|item| !item.replay_evidence_refs.is_empty()));
}

#[test]
fn planted_cross_variant_regression_is_detected() {
    let artifact =
        ProductionQaMatrixArtifact::from_json_str(&read_fixture("matrix.regression.fixture.json"))
            .expect("regression");
    let read_model = artifact.read_model();
    assert_eq!(read_model.cell_count, 8);
    assert_eq!(
        read_model.regression_count, 2,
        "two planted cross-variant regressions"
    );
    let regressions = &read_model.detected_regressions;
    // Deterministic ordering: sorted by (seed, target, variant).
    assert_eq!(regressions[0].seed, "seed-1");
    assert_eq!(regressions[0].target, "web");
    assert_eq!(regressions[0].content_variant, "variant-a");
    assert_eq!(regressions[0].baseline_verdict, "passed");
    assert_eq!(regressions[0].variant_verdict, "failed");
    assert_eq!(regressions[1].seed, "seed-2");
    assert_eq!(regressions[1].target, "desktop");
    assert_eq!(regressions[1].variant_verdict, "failed");
    // Replayable evidence: each regression carries baseline + variant refs.
    assert!(regressions[0]
        .evidence_refs
        .iter()
        .any(|r| r.contains("va-s1-web")));
    assert!(regressions[0]
        .evidence_refs
        .iter()
        .any(|r| r.contains("base-s1-web")));
}

#[test]
fn planted_cross_variant_regression_has_replayable_baseline_and_variant_evidence() {
    let artifact =
        ProductionQaMatrixArtifact::from_json_str(&read_fixture("matrix.regression.fixture.json"))
            .expect("regression");
    let plan = artifact.execution_plan().expect("execution plan");

    assert_eq!(plan.detected_regressions.len(), 2);
    let first = &plan.detected_regressions[0];
    assert_eq!(first.seed, "seed-1");
    assert_eq!(first.target, "web");
    assert_eq!(first.content_variant, "variant-a");
    assert!(first.baseline_work_item_id.contains(":base:seed-1:web"));
    assert!(first.variant_work_item_id.contains(":variant-a:seed-1:web"));
    assert!(first
        .baseline_evidence_refs
        .iter()
        .any(|r| r.contains("base-s1-web")));
    assert!(first
        .variant_evidence_refs
        .iter()
        .any(|r| r.contains("va-s1-web")));
}

#[test]
fn read_model_is_deterministic_regardless_of_cell_order() {
    let raw = read_fixture("matrix.regression.fixture.json");
    let forward = ProductionQaMatrixArtifact::from_json_str(&raw).expect("forward");
    let mut reversed = forward.clone();
    reversed.cells.reverse();
    // Reversed cells still validate (order-independent).
    reversed.validate().expect("reversed validates");
    assert_eq!(
        forward.read_model_json().unwrap(),
        reversed.read_model_json().unwrap(),
        "read model must be deterministic regardless of cell order"
    );
}

#[test]
fn partial_fixture_reports_incomplete_coverage() {
    let artifact =
        ProductionQaMatrixArtifact::from_json_str(&read_fixture("matrix.partial.fixture.json"))
            .expect("partial");
    let read_model = artifact.read_model();
    assert!(!read_model.coverage_complete);
    assert!(read_model.cell_count < read_model.expected_cell_count);
    // A flaky (non-failing) variant cell is not a cross-variant regression.
    assert_eq!(read_model.regression_count, 0);
}

#[test]
fn incomplete_or_missing_runner_evidence_is_not_executable() {
    let partial =
        ProductionQaMatrixArtifact::from_json_str(&read_fixture("matrix.partial.fixture.json"))
            .expect("partial");
    let partial_error = partial
        .execution_plan()
        .expect_err("partial is not executable");
    assert!(
        partial_error
            .to_string()
            .contains("cannot produce complete runner work items"),
        "{partial_error}"
    );

    let mut missing_evidence =
        ProductionQaMatrixArtifact::from_json_str(&read_fixture("matrix.complete.fixture.json"))
            .expect("complete");
    missing_evidence.cells[0].evidence_refs.clear();
    let evidence_error = missing_evidence
        .execution_plan()
        .expect_err("missing replay evidence is not executable");
    assert!(
        evidence_error
            .to_string()
            .contains("missing evidence for verdict"),
        "{evidence_error}"
    );
}

#[test]
fn invalid_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/matrix.malformed-verdict.fixture.json",
            "malformed verdict",
        ),
        (
            "invalid/matrix.duplicate-coordinate.fixture.json",
            "duplicate coordinate",
        ),
        (
            "invalid/matrix.undeclared-target.fixture.json",
            "undeclared target",
        ),
        (
            "invalid/matrix.baseline-not-declared.fixture.json",
            "must be declared in contentVariants",
        ),
        (
            "invalid/matrix.missing-evidence.fixture.json",
            "is missing evidence for verdict",
        ),
        (
            "invalid/matrix.missing-run-ref.fixture.json",
            "qaRunMatrixRef",
        ),
        (
            "invalid/matrix.not-read-only.fixture.json",
            "must remain read-only or draft-only",
        ),
        (
            "invalid/matrix.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
        (
            "invalid/matrix.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/matrix.forbidden-wording.fixture.json",
            "forbidden production QA matrix authority text",
        ),
    ] {
        let error = ProductionQaMatrixArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn scope_doc_keeps_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/production-qa-matrix-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("#1665"));
    assert!(docs.contains("Regression matrix"));
    assert!(docs.contains("descriptive"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

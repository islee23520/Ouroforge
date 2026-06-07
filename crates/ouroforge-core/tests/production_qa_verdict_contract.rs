use ouroforge_core::production_qa_verdict::ProductionQaVerdictArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/production-qa-verdict-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn valid_fixtures_validate_and_roll_up_verdict() {
    for (name, verdict) in [
        ("verdict.pass.fixture.json", "pass"),
        ("verdict.fail.fixture.json", "fail"),
        ("verdict.inconclusive.fixture.json", "inconclusive"),
    ] {
        let artifact = ProductionQaVerdictArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_verdict(), verdict, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.verdict, verdict, "{name}");
        assert_eq!(read_model.check_count, artifact.checks.len(), "{name}");
        assert!(
            artifact.dashboard_compat.read_only,
            "{name} must be read-only"
        );
        // Reuse statement: composes via the evaluator declared-gate-and aggregation.
        assert_eq!(
            read_model.aggregation_operator, "declared-gate-and",
            "{name}"
        );
        assert_eq!(read_model.undeclared_gate_policy, "neutral", "{name}");
        // Verdict is descriptive, not a quality guarantee.
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("descriptive")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no new evaluator")));
    }
}

#[test]
fn consolidated_pass_requires_every_declared_check() {
    let artifact =
        ProductionQaVerdictArtifact::from_json_str(&read_fixture("verdict.pass.fixture.json"))
            .expect("pass");
    let read_model = artifact.read_model();
    assert_eq!(read_model.verdict, "pass");
    // The undeclared assetUx check is neutral (declared-gate-and).
    assert_eq!(read_model.declared_count, 6);
    assert_eq!(read_model.passed_count, 6);
    assert!(read_model.failing_checks.is_empty());
}

#[test]
fn any_declared_check_failure_propagates() {
    let artifact =
        ProductionQaVerdictArtifact::from_json_str(&read_fixture("verdict.fail.fixture.json"))
            .expect("fail");
    let read_model = artifact.read_model();
    assert_eq!(read_model.verdict, "fail");
    assert_eq!(read_model.failing_checks.len(), 1);
    assert_eq!(read_model.failing_checks[0].kind, "visualRegression");
}

#[test]
fn read_model_is_deterministic_regardless_of_check_order() {
    let raw = read_fixture("verdict.fail.fixture.json");
    let forward = ProductionQaVerdictArtifact::from_json_str(&raw).expect("forward");
    let mut reversed = forward.clone();
    reversed.checks.reverse();
    reversed.validate().expect("reversed validates");
    assert_eq!(
        forward.read_model_json().unwrap(),
        reversed.read_model_json().unwrap(),
        "read model must be deterministic regardless of check order"
    );
}

#[test]
fn malformed_and_invalid_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/verdict.unknown-kind.fixture.json",
            "unknown check kind",
        ),
        (
            "invalid/verdict.malformed-status.fixture.json",
            "malformed check status",
        ),
        (
            "invalid/verdict.declared-skipped.fixture.json",
            "declared check `c1` must not be skipped",
        ),
        (
            "invalid/verdict.undeclared-not-skipped.fixture.json",
            "must be skipped (neutral)",
        ),
        (
            "invalid/verdict.missing-evidence.fixture.json",
            "check evidenceRef",
        ),
        (
            "invalid/verdict.verdict-mismatch.fixture.json",
            "does not match computed verdict",
        ),
        (
            "invalid/verdict.duplicate-check-id.fixture.json",
            "duplicate check id",
        ),
        (
            "invalid/verdict.not-read-only.fixture.json",
            "must remain read-only or draft-only",
        ),
        (
            "invalid/verdict.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
        (
            "invalid/verdict.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/verdict.forbidden-wording.fixture.json",
            "forbidden production QA verdict authority text",
        ),
    ] {
        let error = ProductionQaVerdictArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn scope_doc_mentions_consolidated_verdict() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/production-qa-matrix-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("production-QA verdict"));
    assert!(docs.contains("declared-gate-and"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
}

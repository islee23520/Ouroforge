use ouroforge_core::qa_flake_rerun_policy::FlakeRerunPolicyArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-flake-rerun-policy-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn policy_fixtures_validate_and_classify() {
    for (name, expected) in [
        ("policy.stable-pass.fixture.json", "stable-pass"),
        ("policy.stable-fail.fixture.json", "stable-fail"),
        ("policy.flaky.fixture.json", "flaky"),
        ("policy.inconclusive.fixture.json", "inconclusive"),
        ("policy.exhausted.fixture.json", "exhausted"),
        ("policy.unsupported.fixture.json", "unsupported"),
        ("policy.stale.fixture.json", "stale"),
    ] {
        let artifact = FlakeRerunPolicyArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_classification(), expected, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.classification, expected);
        assert!(read_model.max_reruns >= 1 && read_model.max_reruns <= 10);
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("evidence inputs")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no auto-fix")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-merge enabled"));
    }
}

#[test]
fn invalid_policy_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/policy.unbounded-reruns.fixture.json",
            "unbounded reruns are not allowed",
        ),
        (
            "invalid/policy.missing-threshold.fixture.json",
            "consistency threshold must be between 0 and 1",
        ),
        (
            "invalid/policy.overlapping-outputs.fixture.json",
            "overlapping output roots",
        ),
        (
            "invalid/policy.missing-cleanup.fixture.json",
            "cleanup policy must not be empty",
        ),
        (
            "invalid/policy.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/policy.malformed-comparison.fixture.json",
            "malformed comparison",
        ),
        (
            "invalid/policy.missing-original-evidence.fixture.json",
            "missing original evidence",
        ),
        (
            "invalid/policy.classification-mismatch.fixture.json",
            "does not match computed classification",
        ),
        (
            "invalid/policy.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
    ] {
        let error = FlakeRerunPolicyArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn policy_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-flake-rerun-policy-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #691"));
    assert!(docs.contains("evidence inputs"));
    assert!(docs.contains("bounded reruns"));
    assert!(docs.contains("review-gated"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "quality guaranteed",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

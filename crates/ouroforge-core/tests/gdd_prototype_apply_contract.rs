use ouroforge_core::gdd_prototype_apply::GddPrototypeApplyArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-prototype-apply-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn prototype_apply_fixtures_validate_and_export_read_models() {
    for name in [
        "apply.valid.fixture.json",
        "apply.missing-review.fixture.json",
        "apply.stale.fixture.json",
    ] {
        let artifact = GddPrototypeApplyArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.transaction_count, artifact.transactions.len());
        assert_eq!(
            read_model.rollback_target_count,
            artifact.rollback_metadata.targets.len()
        );
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("accepted independent review")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("untrusted until Rust/local validation")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("auto-apply enabled"));
    }
}

#[test]
fn invalid_prototype_apply_fixtures_fail_closed() {
    for (name, expected) in [
        ("invalid/apply.self-approval.fixture.json", "self-approval"),
        (
            "invalid/apply.missing-review-ready.fixture.json",
            "accepted review",
        ),
        ("invalid/apply.auto-apply.fixture.json", "autoApply"),
        (
            "invalid/apply.source-like-target.fixture.json",
            "source-like fixture policy",
        ),
        (
            "invalid/apply.generated-output-collision.fixture.json",
            "generated-output collision",
        ),
        (
            "invalid/apply.stale-no-blocker.fixture.json",
            "stale target",
        ),
        (
            "invalid/apply.missing-behavior-ref.fixture.json",
            "behavior transactions require behaviorRefs",
        ),
        (
            "invalid/apply.missing-scenario-ref.fixture.json",
            "scenario transactions require scenarioRefs",
        ),
        (
            "invalid/apply.rollback-mismatch.fixture.json",
            "rollback metadata must match",
        ),
        (
            "invalid/apply.missing-asset-source.fixture.json",
            "assetSourceRefs must not be empty",
        ),
        (
            "invalid/apply.hidden-command.fixture.json",
            "restricted to local cargo/node",
        ),
        ("invalid/apply.unsafe-boundary.fixture.json", "forbidden"),
    ] {
        let error = GddPrototypeApplyArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn prototype_apply_docs_keep_review_gate_and_governance_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-prototype-apply-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #656"));
    assert!(docs.contains("accepted review"));
    assert!(docs.contains("rollback metadata"));
    assert!(docs.contains("rerun command context"));
    assert!(docs.contains("generated-state audit"));
    assert!(docs.contains("Rust/local validation owns trusted persistence"));
    assert!(docs.contains("#1 remains open"));
    assert!(docs.contains("#23 remains open"));
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "browser trusted write enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

use ouroforge_core::safe_source_apply_demo::SafeSourceApplyDemoArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/safe-source-apply-demo-v1")
        .join(name)
}
fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn safe_source_apply_demo_fixtures_validate_and_export_read_models() {
    for name in [
        "demo.valid.fixture.json",
        "demo.partial.fixture.json",
        "demo.blocked.fixture.json",
    ] {
        let artifact = SafeSourceApplyDemoArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.stage_count, artifact.stages.len());
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("pre-apply gates")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("never executes commands")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-apply enabled"));
        assert!(!json.contains("unrestricted source mutation enabled"));
    }
}

#[test]
fn safe_source_apply_demo_valid_fixture_is_ready_with_full_chain() {
    let artifact =
        SafeSourceApplyDemoArtifact::from_json_str(&read_fixture("demo.valid.fixture.json"))
            .expect("valid fixture");
    let read_model = artifact.read_model();
    assert_eq!(read_model.status, "ready");
    assert_eq!(read_model.stage_count, 15);
    assert_eq!(read_model.pre_apply_gate_count, 9);
    assert_eq!(read_model.post_apply_evidence_count, 6);
    assert_eq!(read_model.blocked_count, 0);
}

#[test]
fn invalid_safe_source_apply_demo_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/demo.forbidden-class.fixture.json",
            "forbidden high-risk class",
        ),
        (
            "invalid/demo.self-review.fixture.json",
            "reviewerId != authorId",
        ),
        (
            "invalid/demo.unsafe-ref.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/demo.chain-order.fixture.json",
            "must precede apply-transaction",
        ),
        (
            "invalid/demo.unsupported-status.fixture.json",
            "unsupported for v1",
        ),
        (
            "invalid/demo.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
        (
            "invalid/demo.incomplete-ready.fixture.json",
            "requires the full chain",
        ),
        ("invalid/demo.duplicate-stage.fixture.json", "duplicated"),
    ] {
        let error = SafeSourceApplyDemoArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn safe_source_apply_demo_docs_keep_governance_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/safe-source-apply-demo-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #713"));
    assert!(docs.contains("review-gated trusted source apply"));
    assert!(docs.contains("not unrestricted source mutation"));
    assert!(docs.contains("#1 remains open"));
    assert!(docs.contains("#23 remains open"));
    for forbidden in [
        "auto-apply enabled",
        "self-approval enabled",
        "browser trusted write enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

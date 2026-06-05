use ouroforge_core::gdd_prototype_task_graph::GddPrototypeTaskGraphArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-prototype-task-graph-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn prototype_task_graph_fixtures_validate_and_export_read_models() {
    for name in [
        "task-graph.valid.fixture.json",
        "task-graph.blocked.fixture.json",
    ] {
        let artifact = GddPrototypeTaskGraphArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.task_count, artifact.tasks.len());
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("dependency-checked task graph")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("untrusted until Rust/local validation")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("hidden command execution enabled"));
    }
}

#[test]
fn invalid_prototype_task_graph_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/task-graph.cyclic.fixture.json",
            "cycle or out-of-order dependency",
        ),
        (
            "invalid/task-graph.missing-dependency.fixture.json",
            "missing dependency",
        ),
        (
            "invalid/task-graph.invalid-kind.fixture.json",
            "failed to parse",
        ),
        (
            "invalid/task-graph.conflicting-ownership.fixture.json",
            "conflicting file ownership",
        ),
        (
            "invalid/task-graph.missing-producer.fixture.json",
            "before a producer artifact exists",
        ),
        (
            "invalid/task-graph.apply-without-review.fixture.json",
            "must depend on a review-gate",
        ),
        (
            "invalid/task-graph.unsafe-boundary.fixture.json",
            "forbidden",
        ),
    ] {
        let error = GddPrototypeTaskGraphArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn prototype_task_graph_docs_keep_planning_and_governance_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-prototype-task-graph-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #654"));
    assert!(docs.contains("ordered dependency-checked task graph before apply"));
    assert!(docs.contains("does not execute hidden commands"));
    assert!(docs.contains("file ownership"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "hidden command execution enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "browser trusted write enabled",
        "production-ready claim enabled",
        "autonomous unrestricted game creation enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

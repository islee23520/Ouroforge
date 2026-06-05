use ouroforge_core::gdd_feasibility_gate::GddFeasibilityGateArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-feasibility-gate-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn feasibility_gate_state_fixtures_validate_and_export_read_models() {
    for name in [
        "feasibility.feasible.fixture.json",
        "feasibility.infeasible.fixture.json",
        "feasibility.deferred.fixture.json",
        "feasibility.downgraded.fixture.json",
        "feasibility.overbroad.fixture.json",
        "feasibility.blocked.fixture.json",
    ] {
        let artifact = GddFeasibilityGateArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(
            read_model.supported_mechanic_count,
            artifact.supported_mechanics.len()
        );
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("feasibility")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("read-only")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("auto-apply enabled"));
    }
}

#[test]
fn invalid_feasibility_gate_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/feasibility.missing-mapping.fixture.json",
            "must not be empty",
        ),
        (
            "invalid/feasibility.unsupported-without-risk.fixture.json",
            "missing supported mechanics",
        ),
        (
            "invalid/feasibility.overlarge-without-risk.fixture.json",
            "overlarge scope",
        ),
        (
            "invalid/feasibility.missing-acceptance.fixture.json",
            "missing acceptance criteria",
        ),
        (
            "invalid/feasibility.missing-scenario.fixture.json",
            "missing scenario plan",
        ),
        (
            "invalid/feasibility.unsatisfied-prereq.fixture.json",
            "blockedReason",
        ),
        (
            "invalid/feasibility.defer-no-slice.fixture.json",
            "sliceRecommendation",
        ),
        (
            "invalid/feasibility.unsafe-boundary.fixture.json",
            "forbidden",
        ),
    ] {
        let error = GddFeasibilityGateArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn feasibility_gate_docs_keep_generation_and_governance_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-feasibility-gate-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #648"));
    assert!(docs.contains("Prototype planning starts only after feasibility passes"));
    assert!(docs.contains("pass/fail/defer"));
    assert!(docs.contains("bounded slice"));
    assert!(docs.contains("not a prototype generator"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
        "browser trusted write enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

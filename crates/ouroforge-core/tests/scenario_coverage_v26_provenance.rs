use ouroforge_core::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleStatus};
use ouroforge_core::provenance_replay::{replay_provenance_bundle, ProvenanceReplayStatus};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn fixture_root() -> PathBuf {
    repo_root().join("examples/end-to-end-provenance-v1/scenario-coverage-v26")
}

fn read_json(name: &str) -> Value {
    let path = fixture_root().join(name);
    serde_json::from_str(
        &fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}")),
    )
    .unwrap_or_else(|error| panic!("parse {path:?}: {error}"))
}

fn read_bundle(name: &str) -> ProvenanceBundleArtifact {
    let path = fixture_root().join(name);
    ProvenanceBundleArtifact::from_json_str(
        &fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}")),
    )
    .unwrap_or_else(|error| panic!("parse {path:?}: {error}"))
}

fn replay_workspace(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ouroforge-scenario-coverage-v26-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("clear replay workspace");
    }
    path
}

#[test]
fn v26_bundle_states_are_enumerated_by_fixture_matrix() {
    let matrix = read_json("matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v26-provenance-matrix-v1"
    );
    assert_eq!(matrix["issue"], 1505);

    for case in matrix["bundleCases"].as_array().expect("bundle cases") {
        let fixture = case["fixture"].as_str().expect("fixture path");
        let expected = case["expectedStatus"].as_str().expect("expected status");
        let bundle = read_bundle(fixture);
        let evaluation = bundle.evaluate_with_root(&fixture_root());

        let expected_status = match expected {
            "complete" => ProvenanceBundleStatus::Complete,
            "incomplete" => ProvenanceBundleStatus::Incomplete,
            "dangling" => ProvenanceBundleStatus::Dangling,
            other => panic!("unexpected v26 bundle status {other}"),
        };
        assert_eq!(bundle.status, expected_status, "{fixture}");
        assert_eq!(evaluation.computed_status, expected_status, "{fixture}");
        assert!(evaluation.status_consistent, "{evaluation:#?}");

        if case["requiresIncompleteReasons"].as_bool().unwrap_or(false) {
            assert!(
                !bundle.incomplete_reasons.is_empty(),
                "{fixture} must expose incomplete reasons"
            );
        }

        let required_link_states = case["requiredLinkStates"]
            .as_object()
            .expect("required link states");
        for (kind, state) in required_link_states {
            assert_eq!(
                evaluation.link_states.get(kind).map(String::as_str),
                state.as_str(),
                "{fixture}:{kind}"
            );
        }
    }
}

#[test]
fn v26_replay_results_cover_reproduced_diverged_and_not_replayable() {
    let matrix = read_json("matrix.fixture.json");

    for case in matrix["replayCases"].as_array().expect("replay cases") {
        let fixture = case["bundleFixture"].as_str().expect("bundle fixture");
        let expected = case["expectedStatus"].as_str().expect("expected status");
        let bundle = read_bundle(fixture);
        let result = replay_provenance_bundle(&bundle, fixture_root(), replay_workspace(expected));

        let expected_status = match expected {
            "reproduced" => ProvenanceReplayStatus::Reproduced,
            "diverged" => ProvenanceReplayStatus::Diverged,
            "not-replayable" => ProvenanceReplayStatus::NotReplayable,
            other => panic!("unexpected v26 replay status {other}"),
        };
        assert_eq!(result.status, expected_status, "{fixture}");

        match expected_status {
            ProvenanceReplayStatus::Reproduced => {
                assert!(result.issues.is_empty(), "{:?}", result.issues);
                assert!(result.diff.is_empty(), "{:?}", result.diff);
                assert_eq!(result.expected_verdict, result.actual_verdict);
            }
            ProvenanceReplayStatus::Diverged => {
                assert!(result.issues.is_empty(), "{:?}", result.issues);
                assert!(
                    result.diff.iter().any(|diff| diff.path == "$.status"),
                    "{:?}",
                    result.diff
                );
            }
            ProvenanceReplayStatus::NotReplayable => {
                assert!(result.actual_verdict.is_none());
                assert!(result.replay_run_dir.is_none());
                assert!(
                    result
                        .issues
                        .iter()
                        .any(|issue| issue.contains("missing replay input: runRef")),
                    "{:?}",
                    result.issues
                );
            }
        }
    }
}

#[test]
fn v26_no_bundle_golden_remains_valid_and_additive() {
    let golden = read_json("compatibility/no-bundle-change.golden.json");

    assert_eq!(golden["schemaVersion"], "legacy-change-read-model-v1");
    assert_eq!(golden["status"], "valid");
    assert!(golden.get("provenanceBundle").is_none());
    assert!(golden["compatibilityNotes"]
        .as_array()
        .expect("compatibility notes")
        .iter()
        .any(|note| note.as_str().is_some_and(|note| note.contains("additive"))));
}

#[test]
fn v26_docs_and_fixtures_preserve_generated_state_wording_and_governance() {
    let docs = fs::read_to_string(repo_root().join("docs/scenario-coverage-v26.md")).expect("docs");
    let mut all = docs.clone();

    for relative in [
        "bundles/complete.fixture.json",
        "bundles/incomplete.fixture.json",
        "bundles/dangling.fixture.json",
        "bundles/diverged.fixture.json",
        "bundles/not-replayable.fixture.json",
        "replay-results/reproduced.fixture.json",
        "replay-results/diverged.fixture.json",
        "replay-results/not-replayable.fixture.json",
        "compatibility/no-bundle-change.golden.json",
    ] {
        all.push('\n');
        all.push_str(
            &fs::read_to_string(fixture_root().join(relative))
                .unwrap_or_else(|error| panic!("read {relative}: {error}")),
        );
    }

    assert!(all.contains("#1505"));
    assert!(all.contains("Generated state remains untracked unless explicitly fixture-scoped"));
    assert!(all.contains("#1 remains open"));
    assert!(all.contains("#23 remains open"));
    assert!(all.contains("#1506 governance remains out of scope"));
    assert!(all.contains("provenance bundles are additive"));

    for bundle_fixture in [
        "bundles/complete.fixture.json",
        "bundles/incomplete.fixture.json",
        "bundles/dangling.fixture.json",
        "bundles/diverged.fixture.json",
        "bundles/not-replayable.fixture.json",
    ] {
        let bundle = read_json(bundle_fixture);
        assert_eq!(bundle["generatedState"]["fixtureScoped"], true);
        assert_eq!(bundle["generatedState"]["tracked"], true);
    }

    let lower = all.to_ascii_lowercase();
    for forbidden in [
        "closes #1",
        "closed #1",
        "closes #23",
        "closed #23",
        "auto-promote enabled",
        "auto-approval enabled",
        "auto-merge enabled",
        "production-ready",
        "production grade",
        "godot replacement",
    ] {
        assert!(!lower.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

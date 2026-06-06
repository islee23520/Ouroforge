use ouroforge_core::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleLinkKind};
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
    repo_root().join("examples/provenance-replay-v1")
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
        "ouroforge-provenance-replay-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("clear replay workspace");
    }
    path
}

#[test]
fn reproduced_bundle_replays_referenced_run_with_existing_evaluator() {
    let bundle = read_bundle("bundle.reproduced.fixture.json");
    let workspace = replay_workspace("reproduced");

    let result = replay_provenance_bundle(&bundle, fixture_root(), &workspace);

    assert_eq!(result.status, ProvenanceReplayStatus::Reproduced);
    assert!(result.issues.is_empty(), "{:?}", result.issues);
    assert!(result.diff.is_empty(), "{:?}", result.diff);
    assert_eq!(result.expected_verdict, result.actual_verdict);
    assert!(result
        .replay_run_dir
        .as_ref()
        .map(|path| PathBuf::from(path).is_dir())
        .unwrap_or(false));

    let actual = result.actual_verdict.expect("actual replay verdict");
    assert_eq!(actual["status"], "passed");
    assert_eq!(actual["gateCategories"]["mechanical"]["status"], "pass");
    assert_eq!(actual["gateCategories"]["runtime"]["status"], "pass");
    assert_eq!(actual["gateCategories"]["visual"]["status"], "pass");
    assert_eq!(actual["gateCategories"]["semantic"]["status"], "pass");
}

#[test]
fn diverged_bundle_reports_diff_without_silent_pass() {
    let bundle = read_bundle("bundle.diverged.fixture.json");
    let workspace = replay_workspace("diverged");

    let result = replay_provenance_bundle(&bundle, fixture_root(), &workspace);

    assert_eq!(result.status, ProvenanceReplayStatus::Diverged);
    assert!(result.issues.is_empty(), "{:?}", result.issues);
    assert!(!result.diff.is_empty(), "divergence must include a diff");
    assert!(result.diff.iter().any(|diff| diff.path == "$.status"));
    assert_eq!(
        result
            .actual_verdict
            .as_ref()
            .expect("actual replay verdict")["status"],
        "passed"
    );
}

#[test]
fn missing_and_non_deterministic_inputs_are_not_replayable() {
    let bundle = read_bundle("bundle.not-replayable.fixture.json");
    let workspace = replay_workspace("not-replayable");

    let result = replay_provenance_bundle(&bundle, fixture_root(), &workspace);

    assert_eq!(result.status, ProvenanceReplayStatus::NotReplayable);
    assert!(result.diff.is_empty());
    assert!(result.actual_verdict.is_none());
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.contains("missing replay input: runRef")));
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.contains("non-deterministic inputs")));
}

#[test]
fn stale_refs_are_not_replayable() {
    let mut bundle = read_bundle("bundle.reproduced.fixture.json");
    let evaluator_link = bundle
        .chain_links
        .iter_mut()
        .find(|link| link.kind == ProvenanceBundleLinkKind::EvaluatorVerdict)
        .expect("evaluator verdict link");
    evaluator_link.stale = true;
    evaluator_link.stale_reason = Some("contract fixture marks the verdict ref stale".to_string());
    let workspace = replay_workspace("stale");

    let result = replay_provenance_bundle(&bundle, fixture_root(), &workspace);

    assert_eq!(result.status, ProvenanceReplayStatus::NotReplayable);
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.contains("stale ref: evaluator-verdict")));
    assert!(result.replay_run_dir.is_none());
}

#[test]
fn replay_docs_preserve_generated_state_wording_compatibility_and_governance_boundaries() {
    let docs = fs::read_to_string(repo_root().join("docs/provenance-replay-v1.md"))
        .expect("read replay docs");
    let reproduced = fs::read_to_string(fixture_root().join("bundle.reproduced.fixture.json"))
        .expect("read reproduced fixture");
    let diverged = fs::read_to_string(fixture_root().join("bundle.diverged.fixture.json"))
        .expect("read diverged fixture");
    let not_replayable =
        fs::read_to_string(fixture_root().join("bundle.not-replayable.fixture.json"))
            .expect("read not-replayable fixture");
    let all = format!("{docs}\n{reproduced}\n{diverged}\n{not_replayable}");

    assert!(all.contains("#1502"));
    assert!(all.contains("reuses `evaluate_run`") || all.contains("Reuse evaluate_run"));
    assert!(all.contains("Generated replay outputs remain untracked unless fixture-scoped"));
    assert!(all.contains("#1 remains open"));
    assert!(all.contains("#23 remains open"));
    assert!(all.contains("#1503 sign-off/audit"));
    assert!(all.contains("#1504 export"));
    assert!(all.contains("#1505 dashboard UI"));
    assert!(all.contains("#1506 governance"));

    for bundle in [reproduced, diverged, not_replayable] {
        let value: Value = serde_json::from_str(&bundle).expect("fixture JSON");
        assert_eq!(value["generatedState"]["fixtureScoped"], true);
        assert_eq!(value["generatedState"]["tracked"], true);
    }

    let lower = all.to_ascii_lowercase();
    assert!(!lower.contains("godot replacement"));
    assert!(!lower.contains("production-grade"));
    assert!(!lower.contains("close #1"));
    assert!(!lower.contains("close #23"));
    for issue in 760..=768 {
        assert!(
            !all.contains(&format!("#{issue}")),
            "replay wording must not touch issue #{issue}"
        );
    }
}

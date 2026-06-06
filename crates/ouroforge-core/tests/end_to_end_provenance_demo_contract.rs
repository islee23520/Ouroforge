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

fn demo_root() -> PathBuf {
    repo_root().join("examples/end-to-end-provenance-v1/demo")
}

fn read_bundle(name: &str) -> ProvenanceBundleArtifact {
    let path = demo_root().join(name);
    ProvenanceBundleArtifact::from_json_str(
        &fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}")),
    )
    .unwrap_or_else(|error| panic!("parse {path:?}: {error}"))
}

fn replay_workspace(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ouroforge-end-to-end-provenance-demo-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("clear replay workspace");
    }
    path
}

#[test]
fn demo_asserts_complete_chain_reproduced_replay_and_diverged_case() {
    let complete = read_bundle("bundle.complete.fixture.json");
    let evaluation = complete.evaluate_with_root(&demo_root());

    assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Complete);
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(evaluation.issues.is_empty(), "{evaluation:#?}");
    for required in [
        "intent-design-brief",
        "generated-edited-artifact",
        "validation-result",
        "runtime-observation",
        "evaluator-verdict",
        "regression-comparison",
        "journal-review-decision",
        "promotion-rollback-record",
    ] {
        assert_eq!(
            evaluation.link_states.get(required).map(String::as_str),
            Some("present"),
            "missing demo chain link {required}: {evaluation:#?}"
        );
    }

    assert!(complete.generated_state.generated);
    assert!(complete.generated_state.tracked);
    assert!(complete.generated_state.fixture_scoped);
    assert!(complete
        .compatibility_notes
        .iter()
        .any(|note| note.contains("additive")));
    assert!(complete
        .compatibility_notes
        .iter()
        .any(|note| note.contains("playable-demo-v2")));

    let reproduced =
        replay_provenance_bundle(&complete, demo_root(), replay_workspace("reproduced"));
    assert_eq!(reproduced.status, ProvenanceReplayStatus::Reproduced);
    assert!(reproduced.issues.is_empty(), "{:?}", reproduced.issues);
    assert!(reproduced.diff.is_empty(), "{:?}", reproduced.diff);
    assert_eq!(reproduced.expected_verdict, reproduced.actual_verdict);

    let actual = reproduced
        .actual_verdict
        .as_ref()
        .expect("actual reproduced verdict");
    assert_eq!(actual["status"], "passed");
    assert_eq!(actual["gateCategories"]["mechanical"]["status"], "pass");
    assert_eq!(actual["gateCategories"]["semantic"]["status"], "pass");

    let diverged_bundle = read_bundle("bundle.diverged.fixture.json");
    let diverged =
        replay_provenance_bundle(&diverged_bundle, demo_root(), replay_workspace("diverged"));
    assert_eq!(diverged.status, ProvenanceReplayStatus::Diverged);
    assert!(diverged.issues.is_empty(), "{:?}", diverged.issues);
    assert!(
        diverged.diff.iter().any(|diff| diff.path == "$.status"),
        "diverged replay must report evidence-linked status diff: {:?}",
        diverged.diff
    );
    assert_eq!(
        diverged
            .actual_verdict
            .as_ref()
            .expect("actual diverged verdict")["status"],
        "passed"
    );
}

#[test]
fn demo_docs_and_fixtures_preserve_boundaries_and_governance_audits() {
    let doc = fs::read_to_string(repo_root().join("docs/end-to-end-provenance-v1-demo.md"))
        .expect("read demo doc");
    let complete =
        fs::read_to_string(demo_root().join("bundle.complete.fixture.json")).expect("complete");
    let diverged =
        fs::read_to_string(demo_root().join("bundle.diverged.fixture.json")).expect("diverged");
    let all = format!("{doc}\n{complete}\n{diverged}");

    assert!(all.contains("#1504"));
    assert!(all.contains("playable-demo-v2"));
    assert!(all.contains("read-only"));
    assert!(all.contains("reproduces") || all.contains("Reproduced"));
    assert!(all.contains("diverge"));
    assert!(all.contains("Generated state remains untracked unless explicitly fixture-scoped"));
    assert!(all.contains("backward-compatible"));
    assert!(all.contains("#1 remains open"));
    assert!(all.contains("#23 remains open"));
    assert!(all.contains("#1505 dashboard UI"));
    assert!(all.contains("#1506 governance"));

    for bundle in [complete, diverged] {
        let value: Value = serde_json::from_str(&bundle).expect("bundle JSON");
        assert_eq!(value["generatedState"]["fixtureScoped"], true);
        assert_eq!(value["generatedState"]["tracked"], true);
        assert_eq!(value["generatedState"]["generated"], true);
        assert_eq!(
            value["guardrails"]
                .as_array()
                .expect("guardrails")
                .iter()
                .filter(|guardrail| guardrail.as_str().is_some())
                .count(),
            value["guardrails"].as_array().expect("guardrails").len()
        );
    }

    let lower = all.to_ascii_lowercase();
    for forbidden in [
        "closes #1",
        "closed #1",
        "closes #23",
        "closed #23",
        "auto-approval enabled",
        "auto-promote enabled",
        "production-ready",
        "production grade",
        "quality guarantee",
        "godot replacement",
    ] {
        assert!(!lower.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

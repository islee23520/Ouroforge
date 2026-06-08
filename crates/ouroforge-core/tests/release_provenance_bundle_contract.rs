use ouroforge_core::release_provenance_bundle::{
    ReleaseProvenanceBundle, ReleaseProvenanceStatus, RELEASE_PROVENANCE_BUNDLE_SCHEMA_VERSION,
};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_bundle(name: &str) -> ReleaseProvenanceBundle {
    let path = repo_root().join(format!("examples/release-provenance-bundle-v1/{name}"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    ReleaseProvenanceBundle::from_json_str(&text).unwrap_or_else(|err| panic!("{name}: {err}"))
}

#[test]
fn complete_release_bundle_composes_existing_artifacts_by_reference() {
    let bundle = read_bundle("bundle.complete.fixture.json");
    let evaluation = bundle.evaluate_with_root(&repo_root());

    assert_eq!(
        evaluation.schema_version,
        RELEASE_PROVENANCE_BUNDLE_SCHEMA_VERSION
    );
    assert_eq!(
        evaluation.computed_status,
        ReleaseProvenanceStatus::Complete
    );
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(evaluation.replayable, "{evaluation:#?}");
    assert!(evaluation.issues.is_empty(), "{evaluation:#?}");
    for required in [
        "intent",
        "content",
        "assets",
        "qa",
        "per-change-provenance",
        "compliance",
        "release-candidate",
    ] {
        assert_eq!(
            evaluation.link_states.get(required).map(String::as_str),
            Some("present")
        );
    }
    assert_eq!(
        evaluation
            .per_change_bundle_states
            .get("change-bundle-001")
            .map(String::as_str),
        Some("complete")
    );
    assert!(evaluation
        .allowed_actions
        .iter()
        .any(|a| a == "replay_referenced_bundles_locally"));
    assert!(evaluation
        .forbidden_actions
        .iter()
        .any(|a| a == "fabricate_missing_chain"));
}

#[test]
fn missing_link_is_explicit_incomplete_state_not_fabricated() {
    let bundle = read_bundle("bundle.missing-link.fixture.json");
    let evaluation = bundle.evaluate_with_root(&repo_root());

    assert_eq!(
        evaluation.computed_status,
        ReleaseProvenanceStatus::Incomplete
    );
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(!evaluation.replayable);
    assert_eq!(
        evaluation.link_states.get("compliance").map(String::as_str),
        Some("missing")
    );
    assert!(evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("missing release chain link: compliance")));
    assert!(evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("per-change provenance bundle incomplete")));
    assert!(!evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("fabricated")));
}

#[test]
fn replayability_requires_deterministic_refs() {
    let bundle = read_bundle("bundle.not-replayable.fixture.json");
    let evaluation = bundle.evaluate_with_root(&repo_root());

    assert_eq!(
        evaluation.computed_status,
        ReleaseProvenanceStatus::Incomplete
    );
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(!evaluation.replayable);
    assert!(evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("release replayability requires deterministic inputs")));
}

#[test]
fn docs_and_fixtures_preserve_generated_state_wording_compatibility_and_governance() {
    let docs = std::fs::read_to_string(repo_root().join("docs/release-provenance-bundle-v1.md"))
        .expect("docs exist");
    for required in [
        "extends the Milestone 25 provenance bundle",
        "does not create a new provenance engine",
        "does not fabricate missing chain links",
        "browser/Studio surfaces remain read-only",
        "executes no commands",
        "#1 and #23 remain open",
    ] {
        assert!(
            docs.contains(required),
            "docs missing required phrase: {required}"
        );
    }
    for forbidden in [
        "auto-merges nothing",
        "production-ready",
        "quality/fun",
        "Godot replacement/parity",
    ] {
        assert!(
            docs.contains(forbidden),
            "docs must explicitly forbid {forbidden}"
        );
    }

    for fixture in [
        "bundle.complete.fixture.json",
        "bundle.missing-link.fixture.json",
        "bundle.not-replayable.fixture.json",
    ] {
        let bundle = read_bundle(fixture);
        assert!(bundle.generated_state.generated);
        assert!(bundle.generated_state.tracked);
        assert!(bundle.generated_state.fixture_scoped);
        assert!(bundle.boundary.contains("Milestone 25"));
        assert!(bundle.boundary.contains("read-only"));
        assert!(bundle.boundary.contains("no parallel provenance engine"));
    }
}

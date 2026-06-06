use ouroforge_core::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleStatus};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_root() -> PathBuf {
    repo_root().join("examples/provenance-bundle-v1")
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture_root().join(name)).expect(name)
}

fn read_bundle(name: &str) -> ProvenanceBundleArtifact {
    ProvenanceBundleArtifact::from_json_str(&read_fixture(name))
        .unwrap_or_else(|error| panic!("{name}: {error:#}"))
}

#[test]
fn complete_bundle_composes_all_required_links_by_reference() {
    let bundle = read_bundle("bundle.complete.fixture.json");
    let evaluation = bundle.evaluate_with_root(&fixture_root());

    assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Complete);
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(evaluation.issues.is_empty(), "{evaluation:#?}");
    assert_eq!(evaluation.link_states.len(), 8);

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
            "missing required link state {required}: {evaluation:#?}"
        );
    }

    assert!(bundle.generated_state.generated);
    assert!(bundle.generated_state.tracked);
    assert!(bundle.generated_state.fixture_scoped);
    assert!(bundle
        .compatibility_notes
        .iter()
        .any(|note| note.contains("additive")));
    assert!(bundle
        .compatibility_notes
        .iter()
        .any(|note| note.contains("by reference")));
    assert!(evaluation
        .allowed_actions
        .iter()
        .any(|action| action == "replay_validation_locally"));
    assert!(evaluation
        .forbidden_actions
        .iter()
        .any(|action| action == "apply_patch"));

    let json = bundle
        .evaluation_json_with_root(&fixture_root())
        .expect("evaluation json");
    for forbidden in [
        "auto-merge enabled",
        "auto-promote enabled",
        "production-ready claim enabled",
        "quality guarantee enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(!json.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

#[test]
fn incomplete_and_missing_link_states_are_explicit() {
    let bundle = read_bundle("bundle.missing-chain-link.fixture.json");
    let evaluation = bundle.evaluate_with_root(&fixture_root());

    assert_eq!(
        evaluation.computed_status,
        ProvenanceBundleStatus::Incomplete
    );
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert_eq!(
        evaluation
            .link_states
            .get("promotion-rollback-record")
            .map(String::as_str),
        Some("missing")
    );
    assert!(evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("missing chain link: promotion-rollback-record")));
    assert!(!evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("fabricated")));
}

#[test]
fn dangling_reference_yields_dangling_state() {
    let bundle = read_bundle("bundle.dangling-reference.fixture.json");
    let evaluation = bundle.evaluate_with_root(&fixture_root());

    assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Dangling);
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert_eq!(
        evaluation
            .link_states
            .get("validation-result")
            .map(String::as_str),
        Some("dangling")
    );
    assert!(evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("dangling reference")));
}

#[test]
fn stale_reference_yields_stale_state() {
    let bundle = read_bundle("bundle.stale-reference.fixture.json");
    let evaluation = bundle.evaluate_with_root(&fixture_root());

    assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Stale);
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert_eq!(
        evaluation
            .link_states
            .get("validation-result")
            .map(String::as_str),
        Some("stale")
    );
    assert!(evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("stale ref")));
}

#[test]
fn unresolved_review_reason_keeps_bundle_incomplete_without_dangling_refs() {
    let bundle = read_bundle("bundle.incomplete-state.fixture.json");
    let evaluation = bundle.evaluate_with_root(&fixture_root());

    assert_eq!(
        evaluation.computed_status,
        ProvenanceBundleStatus::Incomplete
    );
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(evaluation
        .link_states
        .values()
        .all(|state| state == "present"));
    assert!(evaluation
        .issues
        .iter()
        .any(|issue| issue.contains("human review decision pending")));
}

#[test]
fn generated_bundles_must_be_untracked_unless_fixture_scoped() {
    let mut value: Value =
        serde_json::from_str(&read_fixture("bundle.complete.fixture.json")).expect("json");
    value["generatedState"]["fixtureScoped"] = Value::Bool(false);

    let error = ProvenanceBundleArtifact::from_json_str(
        &serde_json::to_string_pretty(&value).expect("json"),
    )
    .expect_err("generated tracked non-fixture bundle should fail")
    .to_string();
    assert!(
        error.contains("generated provenance bundles must be untracked unless fixture-scoped"),
        "{error}"
    );
}

#[test]
fn docs_preserve_wording_compatibility_and_governance_boundaries() {
    let docs = fs::read_to_string(repo_root().join("docs/provenance-bundle-v1.md")).expect("docs");
    let complete = read_fixture("bundle.complete.fixture.json");
    let audit_surface = format!("{docs}\n{complete}");

    assert!(audit_surface.contains("#1500"));
    assert!(audit_surface.contains("read-only audit"));
    assert!(audit_surface.contains("replay"));
    assert!(audit_surface.contains("by reference"));
    assert!(audit_surface.contains("Generated bundles remain untracked unless fixture-scoped"));
    assert!(audit_surface.contains("#1 remains open"));
    assert!(audit_surface.contains("#23 remains open"));
    assert!(docs.contains("does not implement #1502"));
    for scoped_out in ["#1502", "#1503", "#1504", "#1505", "#1506"] {
        assert!(
            docs.contains(scoped_out),
            "{scoped_out} must remain explicitly out of scope"
        );
    }
    for untouched in [
        "#760", "#761", "#762", "#763", "#764", "#765", "#766", "#767", "#768",
    ] {
        assert!(
            !audit_surface.contains(untouched),
            "{untouched} must not be referenced by #1500 fixtures/docs"
        );
    }
    for forbidden in [
        "closes #1",
        "closed #1",
        "closes #23",
        "closed #23",
        "auto-merge enabled",
        "auto-promote enabled",
        "production-ready claim enabled",
        "quality guarantee enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(
            !audit_surface.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
    }
}

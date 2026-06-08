//! Smoke contract for Narrative Assist Demo v1 (#1866).
//!
//! The demo is fixture-scoped and deterministic: it regenerates narrative
//! candidates from the local brief, replays the human selection, and validates
//! read-only integration provenance without network, browser, or trusted writes.

use std::path::{Path, PathBuf};

use ouroforge_core::narrative_candidate::{
    generate_narrative_candidates, NarrativeCandidateBrief, NarrativeCandidateSet,
    NARRATIVE_TONE_HUMAN_BOUNDARY,
};
use ouroforge_core::narrative_integration::{
    integrate_human_selected_candidate, record_human_narrative_selection,
    replay_narrative_selection, NarrativeIntegrationRecord, NarrativeIntegrationSelectionRecord,
    NARRATIVE_INTEGRATION_STATUS_READY,
};
use serde::Deserialize;

const DEMO_SCHEMA_VERSION: &str = "ouroforge.narrative-assist-demo.v1";
const FIXED_NOW_MS: u128 = 1_786_400_000_000;
const SELECTION_NOW_MS: u128 = 1_786_500_001_000;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DemoManifest {
    schema_version: String,
    demo_id: String,
    title: String,
    source_brief: String,
    candidate_set: String,
    selection: String,
    integration_provenance: String,
    fixed_now_unix_ms: u128,
    selection_recorded_at_unix_ms: u128,
    selected_candidate_id: String,
    target_title_id: String,
    network_required: bool,
    live_browser_required: bool,
    proposal_only: bool,
    review_apply_required: bool,
    trusted_write_authority: bool,
    human_tone_boundary: String,
    allowed_actions: Vec<String>,
    conservative_wording: String,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_repo_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|_| panic!("fixture exists: {path}"))
}

fn assert_fixture_scoped(path: &str) {
    let path = Path::new(path);
    assert!(
        path.starts_with("examples/narrative-assist-v1/demo")
            || path == Path::new("examples/narrative-assist-v1/narrative-candidate-brief-v1.json"),
        "demo fixture path must stay under narrative-assist demo fixtures: {}",
        path.display()
    );
    assert!(path.is_relative(), "demo path must be relative");
    assert!(
        !path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir)),
        "demo path must not traverse parents"
    );
}

fn manifest() -> DemoManifest {
    serde_json::from_str(&read_repo_text(
        "examples/narrative-assist-v1/demo/manifest-v1.json",
    ))
    .expect("demo manifest parses")
}

#[test]
fn demo_regenerates_candidates_and_records_selected_integration() {
    let manifest = manifest();
    assert_eq!(manifest.schema_version, DEMO_SCHEMA_VERSION);
    assert_eq!(manifest.demo_id, "narrative-assist-demo-v1");
    assert_eq!(manifest.title, "Narrative Assist Demo v1");
    assert_eq!(manifest.fixed_now_unix_ms, FIXED_NOW_MS);
    assert_eq!(manifest.selection_recorded_at_unix_ms, SELECTION_NOW_MS);
    assert_eq!(manifest.selected_candidate_id, "theme-arc-harbor-repair-v1");
    assert_eq!(manifest.target_title_id, "clockwork-harbor-demo");

    for path in [
        &manifest.source_brief,
        &manifest.candidate_set,
        &manifest.selection,
        &manifest.integration_provenance,
    ] {
        assert_fixture_scoped(path);
    }

    let brief = NarrativeCandidateBrief::from_json_str(&read_repo_text(&manifest.source_brief))
        .expect("demo source brief parses");
    let generated = generate_narrative_candidates(&brief, manifest.fixed_now_unix_ms)
        .expect("demo candidates regenerate deterministically");
    generated
        .validate()
        .expect("generated candidate set validates");

    let fixture_set: NarrativeCandidateSet =
        serde_json::from_str(&read_repo_text(&manifest.candidate_set))
            .expect("candidate set fixture parses");
    fixture_set
        .validate()
        .expect("candidate set fixture validates");
    assert_eq!(fixture_set, generated);
    assert_eq!(fixture_set.candidates.len(), 3);
    assert!(fixture_set.proposal_only);
    assert_eq!(
        fixture_set.human_tone_boundary,
        NARRATIVE_TONE_HUMAN_BOUNDARY
    );

    let computed_selection = record_human_narrative_selection(
        &generated,
        "narrative-demo-selection-harbor-repair-v1",
        &manifest.target_title_id,
        &manifest.selected_candidate_id,
        "human-curator-local",
        "Demo curator selected this beat as provenance for review/apply; no trusted source write is performed.",
        manifest.selection_recorded_at_unix_ms,
    )
    .expect("selection record computes");
    let fixture_selection =
        NarrativeIntegrationSelectionRecord::from_json_str(&read_repo_text(&manifest.selection))
            .expect("selection fixture parses");
    fixture_selection
        .validate()
        .expect("selection fixture validates");
    assert_eq!(fixture_selection, computed_selection);
    let selected = replay_narrative_selection(&generated, &fixture_selection)
        .expect("selection replays against generated candidates");
    assert_eq!(selected.candidate_id, manifest.selected_candidate_id);

    let computed_integration = integrate_human_selected_candidate(
        &generated,
        &fixture_selection,
        "narrative-demo-integration-harbor-repair-v1",
    )
    .expect("integration provenance computes");
    let fixture_integration = NarrativeIntegrationRecord::from_json_str(&read_repo_text(
        &manifest.integration_provenance,
    ))
    .expect("integration fixture parses");
    fixture_integration
        .validate()
        .expect("integration fixture validates");
    assert_eq!(fixture_integration, computed_integration);
    assert_eq!(
        fixture_integration.status,
        NARRATIVE_INTEGRATION_STATUS_READY
    );
    assert!(fixture_integration.review_apply_required);
    assert!(!fixture_integration.trusted_write_authority);
    assert_eq!(
        fixture_integration.human_tone_boundary,
        NARRATIVE_TONE_HUMAN_BOUNDARY
    );
}

#[test]
fn demo_manifest_documents_read_only_deterministic_boundaries() {
    let manifest = manifest();
    assert!(!manifest.network_required);
    assert!(!manifest.live_browser_required);
    assert!(manifest.proposal_only);
    assert!(manifest.review_apply_required);
    assert!(!manifest.trusted_write_authority);
    assert_eq!(manifest.human_tone_boundary, NARRATIVE_TONE_HUMAN_BOUNDARY);
    assert_eq!(
        manifest.allowed_actions,
        vec![
            "inspect-candidates",
            "record-human-selection",
            "route-through-review-apply"
        ]
    );

    let wording = manifest.conservative_wording.to_ascii_lowercase();
    for forbidden in [
        "assert fun",
        "tone quality",
        "production readiness",
        "market fit",
        "godot replacement",
        "auto-merge",
    ] {
        assert!(
            !wording.contains(forbidden) || wording.contains("does not"),
            "demo wording must be conservative around {forbidden}"
        );
    }
}

#[test]
fn docs_explain_the_fixture_scoped_demo_without_subjective_claims() {
    let doc = read_repo_text("docs/narrative-assist-v1-demo.md");
    assert!(doc.contains("Narrative Assist Demo v1"));
    assert!(doc.contains("examples/narrative-assist-v1/demo/manifest-v1.json"));
    assert!(doc.contains("cargo test -p ouroforge-core --test narrative_assist_demo_contract"));
    assert!(doc.contains("no network"));
    assert!(doc.contains("no live browser"));
    assert!(doc.contains("review/apply/trust-gradient"));
    assert!(doc.contains("does not assert fun"));
    assert!(doc.contains("#1 and #23 remain open"));
}

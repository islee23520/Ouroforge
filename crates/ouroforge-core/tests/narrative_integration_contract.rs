//! Contract test for Human-Curated Narrative Integration v1 (#1865).
//!
//! Narrative integration consumes a human selection over a narrative candidate
//! set and records provenance for the existing review/apply path. It remains a
//! read-only assist: no browser/Studio or generated output receives trusted
//! write/apply authority here.

use std::path::PathBuf;

use ouroforge_core::narrative_candidate::{
    generate_narrative_candidates, NarrativeCandidateBrief, NARRATIVE_TONE_HUMAN_BOUNDARY,
};
use ouroforge_core::narrative_integration::{
    integrate_human_selected_candidate, record_human_narrative_selection,
    replay_narrative_selection, NarrativeIntegrationRecord, NarrativeIntegrationSelectionRecord,
    NARRATIVE_INTEGRATION_BOUNDARY, NARRATIVE_INTEGRATION_SCHEMA_VERSION,
    NARRATIVE_INTEGRATION_STATUS_READY,
};

const FIXED_NOW_MS: u128 = 1_786_400_000_000;
const SELECTION_NOW_MS: u128 = 1_786_500_001_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text(dir: &str, name: &str) -> String {
    std::fs::read_to_string(repo_root().join(dir).join(name))
        .unwrap_or_else(|_| panic!("fixture exists: {dir}/{name}"))
}

fn candidate_set() -> ouroforge_core::narrative_candidate::NarrativeCandidateSet {
    let brief = NarrativeCandidateBrief::from_json_str(&fixture_text(
        "examples/narrative-assist-v1",
        "narrative-candidate-brief-v1.json",
    ))
    .expect("candidate brief parses");
    generate_narrative_candidates(&brief, FIXED_NOW_MS).expect("candidate set generates")
}

#[test]
fn human_selected_candidate_integrates_as_review_apply_provenance() {
    let set = candidate_set();
    let selection = record_human_narrative_selection(
        &set,
        "narrative-selection-harbor-repair-v1",
        "clockwork-harbor-demo",
        "theme-arc-harbor-repair-v1",
        "human-curator-local",
        "Human selected this beat as an integration candidate; final source changes still require review/apply.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");

    let integration = integrate_human_selected_candidate(
        &set,
        &selection,
        "narrative-integration-harbor-repair-v1",
    )
    .expect("integration provenance is created");

    integration.validate().expect("integration validates");
    assert_eq!(
        integration.schema_version,
        NARRATIVE_INTEGRATION_SCHEMA_VERSION
    );
    assert_eq!(integration.status, NARRATIVE_INTEGRATION_STATUS_READY);
    assert_eq!(integration.surface_boundary, NARRATIVE_INTEGRATION_BOUNDARY);
    assert_eq!(
        integration.selected_candidate_id,
        "theme-arc-harbor-repair-v1"
    );
    assert_eq!(integration.target_title_id, "clockwork-harbor-demo");
    assert!(integration.review_apply_required);
    assert!(!integration.trusted_write_authority);
    assert_eq!(
        integration.human_tone_boundary,
        NARRATIVE_TONE_HUMAN_BOUNDARY
    );

    let read_model = integration.read_model().expect("read model builds");
    read_model.validate().expect("read model validates");
    assert_eq!(
        read_model.allowed_actions,
        vec!["inspect-selected-candidate", "route-through-review-apply"]
    );
}

#[test]
fn read_only_enforcement_blocks_trusted_write_or_apply_drift() {
    let unsafe_selection = NarrativeIntegrationSelectionRecord::from_json_str(&fixture_text(
        "examples/narrative-integration-v1",
        "narrative-selection-readonly-violation.json",
    ))
    .expect("unsafe selection fixture parses");
    let error = unsafe_selection
        .validate()
        .expect_err("trusted write/apply authority must be blocked");
    assert!(
        error.to_string().contains("read-only provenance"),
        "unexpected error: {error}"
    );
}

#[test]
fn selection_replay_records_and_validates_provenance_identity() {
    let set = candidate_set();
    let selection = record_human_narrative_selection(
        &set,
        "narrative-selection-harbor-repair-v1",
        "clockwork-harbor-demo",
        "theme-arc-harbor-repair-v1",
        "human-curator-local",
        "Replay must resolve the selected candidate and payload hash.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");

    selection.validate().expect("selection validates");
    let selected = replay_narrative_selection(&set, &selection).expect("selection replays");
    assert_eq!(selected.candidate_id, "theme-arc-harbor-repair-v1");
    assert_eq!(
        selected.proposal.proposal.id,
        selection.selected_proposal_id.clone().unwrap()
    );
    assert_eq!(
        selected.payload_hash,
        selection.selected_payload_hash.clone().unwrap()
    );

    let mut stale = selection.clone();
    stale.selected_payload_hash = Some("0".repeat(64));
    let error = replay_narrative_selection(&set, &stale).expect_err("stale hash fails closed");
    assert!(
        error.to_string().contains("selectedPayloadHash is stale"),
        "unexpected error: {error}"
    );
}

#[test]
fn fixture_selection_and_provenance_round_trip_with_computed_hash() {
    let set = candidate_set();
    let computed = record_human_narrative_selection(
        &set,
        "narrative-selection-harbor-repair-v1",
        "clockwork-harbor-demo",
        "theme-arc-harbor-repair-v1",
        "human-curator-local",
        "Human selected this beat as an integration candidate; final source changes still require review/apply.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");
    let hash = computed.selected_payload_hash.as_deref().unwrap();

    let selection_fixture = fixture_text(
        "examples/narrative-integration-v1",
        "narrative-selection-v1.json",
    )
    .replace("__COMPUTED_IN_TEST__", hash);
    let selection = NarrativeIntegrationSelectionRecord::from_json_str(&selection_fixture)
        .expect("selection fixture parses");
    replay_narrative_selection(&set, &selection).expect("fixture selection replays");

    let provenance_fixture = fixture_text(
        "examples/narrative-integration-v1",
        "narrative-integration-provenance-v1.json",
    )
    .replace("__COMPUTED_IN_TEST__", hash);
    let provenance = NarrativeIntegrationRecord::from_json_str(&provenance_fixture)
        .expect("provenance fixture parses");
    provenance.validate().expect("provenance fixture validates");
}

#[test]
fn selected_proposal_path_cannot_be_trusted_source_target() {
    let set = candidate_set();
    let selection = record_human_narrative_selection(
        &set,
        "narrative-selection-harbor-repair-v1",
        "clockwork-harbor-demo",
        "theme-arc-harbor-repair-v1",
        "human-curator-local",
        "Source path drift should fail closed.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");
    let mut integration = integrate_human_selected_candidate(
        &set,
        &selection,
        "narrative-integration-harbor-repair-v1",
    )
    .expect("integration provenance is created");
    integration.selected_proposal_path = "crates/ouroforge-core/src/lib.rs".to_string();
    let error = integration
        .validate()
        .expect_err("source-like proposal path must fail closed");
    assert!(error.to_string().contains("trusted source write target"));
}

#[test]
fn docs_record_narrative_integration_boundaries() {
    let doc = std::fs::read_to_string(repo_root().join("docs/narrative-assist-v1.md"))
        .expect("narrative assist scope doc exists");
    assert!(doc.contains("Human-curated selection and integration contract"));
    assert!(doc.contains("selected candidate is still not trusted source"));
    assert!(doc.contains("review/apply/trust-gradient"));
    assert!(doc.contains("Tone/soul is a human decision boundary"));
}

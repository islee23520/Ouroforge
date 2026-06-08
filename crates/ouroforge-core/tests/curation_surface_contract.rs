//! Contract test for Read-Only Curation Surface v1 (#1853).
//!
//! The curation surface displays candidate proposals and records a human
//! selection as provenance. It is read-only: selection replay verifies ids and
//! payload digest, but no selected candidate becomes trusted/applied here.

use std::path::PathBuf;

use ouroforge_core::candidate_generation::{generate_candidates, CandidateGenerationBrief};
use ouroforge_core::curation_surface::{
    build_curation_read_model, record_human_selection, replay_selection, CurationSelectionRecord,
    CURATION_READ_ONLY_BOUNDARY, CURATION_SELECTION_SCHEMA_VERSION,
};

const FIXED_NOW_MS: u128 = 1_786_000_000_000;
const SELECTION_NOW_MS: u128 = 1_786_000_001_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(repo_root().join("examples/curation-cockpit-v1").join(name))
        .unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn candidate_set() -> ouroforge_core::candidate_generation::CandidateProposalSet {
    let brief = CandidateGenerationBrief::from_json_str(&fixture_text(
        "candidate-generation-brief-v1.json",
    ))
    .expect("candidate brief parses");
    generate_candidates(&brief, FIXED_NOW_MS).expect("candidate set generates")
}

#[test]
fn human_selection_is_recorded_as_provenance() {
    let set = candidate_set();
    let selection = record_human_selection(
        &set,
        "human-selection-spark-v1",
        "candidate-card-spark-v1",
        "human-curator-local",
        "Best fit for the engine-part deckbuilder theme; still only provenance until review/apply.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");

    selection.validate().expect("selection validates");
    assert_eq!(selection.schema_version, CURATION_SELECTION_SCHEMA_VERSION);
    assert_eq!(selection.candidate_set_id, set.candidate_set_id);
    assert_eq!(selection.decision, "selected");
    assert_eq!(selection.surface_boundary, CURATION_READ_ONLY_BOUNDARY);
    assert!(!selection.trusted_write_requested);
    assert!(!selection.apply_authority);
    assert_eq!(
        selection.selected_variant_id.as_deref(),
        Some("candidate-card-spark-v1")
    );
}

#[test]
fn read_only_surface_blocks_trusted_write_drift() {
    let unsafe_selection = CurationSelectionRecord::from_json_str(&fixture_text(
        "curation-selection-readonly-violation.json",
    ))
    .expect("unsafe fixture parses");
    let error = unsafe_selection
        .validate()
        .expect_err("trusted write/apply authority must be blocked");
    assert!(
        error.to_string().contains("read-only provenance"),
        "unexpected error: {error}"
    );
}

#[test]
fn selection_replay_validates_candidate_identity_and_digest() {
    let set = candidate_set();
    let selection = record_human_selection(
        &set,
        "human-selection-spark-v1",
        "candidate-card-spark-v1",
        "human-curator-local",
        "Replay must resolve the same candidate and payload digest.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");

    let selected = replay_selection(&set, &selection).expect("selection replays");
    assert_eq!(selected.variant_id, "candidate-card-spark-v1");
    assert_eq!(
        selected.proposal.proposal.id,
        selection.selected_proposal_id.clone().unwrap()
    );

    let mut stale = selection.clone();
    stale.selected_payload_digest = Some("0".repeat(64));
    let error = replay_selection(&set, &stale).expect_err("stale digest fails closed");
    assert!(
        error.to_string().contains("selectedPayloadDigest is stale"),
        "unexpected error: {error}"
    );
}

#[test]
fn read_model_is_dashboard_safe_and_read_only() {
    let set = candidate_set();
    let selection = record_human_selection(
        &set,
        "human-selection-spark-v1",
        "candidate-card-spark-v1",
        "human-curator-local",
        "Read model displays provenance only.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");
    let model = build_curation_read_model(&set, &[selection]).expect("read model builds");

    model.validate().expect("read model validates");
    assert_eq!(model.candidate_count, 4);
    assert_eq!(model.selections.len(), 1);
    assert_eq!(model.surface_boundary, CURATION_READ_ONLY_BOUNDARY);
    assert!(!model.trusted_write_authority);
    assert_eq!(
        model.allowed_actions,
        vec!["inspect-candidates", "record-selection-provenance"]
    );
}

#[test]
fn fixture_selection_can_be_replayed_after_inserting_computed_digest() {
    let set = candidate_set();
    let computed = record_human_selection(
        &set,
        "human-selection-spark-v1",
        "candidate-card-spark-v1",
        "human-curator-local",
        "Best fit for the engine-part deckbuilder theme; still only provenance until review/apply.",
        SELECTION_NOW_MS,
    )
    .expect("selection record is created");

    let fixture = fixture_text("curation-selection-v1.json").replace(
        "__COMPUTED_IN_TEST__",
        computed.selected_payload_digest.as_deref().unwrap(),
    );
    let selection = CurationSelectionRecord::from_json_str(&fixture).expect("fixture parses");
    replay_selection(&set, &selection).expect("fixture selection replays");
}

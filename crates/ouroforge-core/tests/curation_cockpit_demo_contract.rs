//! Smoke test for Curation Cockpit Demo v1 (#1854).
//!
//! The demo is fixture-scoped and deterministic: a brief generates N candidates,
//! a human selection is recorded as provenance, and replay/read-model checks stay
//! read-only with no trusted write or apply authority.

use std::path::PathBuf;

use ouroforge_core::candidate_generation::{generate_candidates, CandidateGenerationBrief};
use ouroforge_core::curation_surface::{
    build_curation_read_model, record_human_selection, replay_selection, CurationSelectionRecord,
    CURATION_READ_ONLY_BOUNDARY,
};

const FIXED_NOW_MS: u128 = 1_786_000_000_000;
const SELECTION_NOW_MS: u128 = 1_786_000_002_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn demo_fixture_text(name: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/curation-cockpit-v1/demo")
            .join(name),
    )
    .unwrap_or_else(|_| panic!("demo fixture exists: {name}"))
}

#[test]
fn demo_generates_n_candidates_and_records_human_selection() {
    let brief = CandidateGenerationBrief::from_json_str(&demo_fixture_text(
        "candidate-generation-brief-demo-v1.json",
    ))
    .expect("demo candidate brief parses");
    let set = generate_candidates(&brief, FIXED_NOW_MS).expect("demo candidates generate");

    set.validate().expect("demo candidate set validates");
    assert_eq!(
        set.candidate_set_id,
        "curation-cockpit-demo-candidate-set-v1"
    );
    assert_eq!(set.requested_count, 3);
    assert_eq!(set.candidates.len(), 3);
    assert_eq!(
        set.candidates
            .iter()
            .map(|candidate| candidate.variant_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "demo-card-rivet-strike-v1",
            "demo-tuning-calibrated-opening-v1",
            "demo-flavor-workshop-hum-v1",
        ]
    );
    assert!(set.proposal_only);
    for candidate in &set.candidates {
        assert!(candidate.proposal_only);
        assert_eq!(candidate.proposal.proposal.status, "proposed");
        assert_eq!(candidate.proposal.proposal.verdict_status, "pending");
        assert_eq!(candidate.proposal.proposal.confidence, "unverified");
        assert!(candidate.proposal.provenance.proposal_only);
    }

    let computed = record_human_selection(
        &set,
        "demo-human-selection-rivet-strike-v1",
        "demo-card-rivet-strike-v1",
        "human-curator-local-demo",
        "Chosen for the deterministic demo because it is a compact card candidate; this is provenance only and does not apply the proposal.",
        SELECTION_NOW_MS,
    )
    .expect("demo human selection records");
    assert_eq!(computed.surface_boundary, CURATION_READ_ONLY_BOUNDARY);
    assert!(!computed.trusted_write_requested);
    assert!(!computed.apply_authority);

    let selection_fixture = demo_fixture_text("curation-selection-demo-v1.json").replace(
        "__COMPUTED_IN_TEST__",
        computed.selected_payload_digest.as_deref().unwrap(),
    );
    let selection = CurationSelectionRecord::from_json_str(&selection_fixture)
        .expect("selection fixture parses");
    assert_eq!(selection, computed);

    let selected = replay_selection(&set, &selection).expect("demo selection replays");
    assert_eq!(selected.variant_id, "demo-card-rivet-strike-v1");
    assert_eq!(
        selected.proposal.proposal.id,
        "generative-demo-card-rivet-strike-run-v1"
    );

    let read_model = build_curation_read_model(&set, &[selection]).expect("read model builds");
    read_model.validate().expect("read model validates");
    assert_eq!(read_model.candidate_count, 3);
    assert_eq!(read_model.selections.len(), 1);
    assert!(!read_model.trusted_write_authority);
    assert_eq!(
        read_model.allowed_actions,
        vec!["inspect-candidates", "record-selection-provenance"]
    );
}

#[test]
fn demo_doc_records_boundaries_and_fixture_paths() {
    let doc = std::fs::read_to_string(repo_root().join("docs/curation-cockpit-v1-demo.md"))
        .expect("demo doc exists");
    assert!(doc.contains("candidate-generation-brief-demo-v1.json"));
    assert!(doc.contains("curation-selection-demo-v1.json"));
    assert!(doc.contains("3"));
    assert!(doc.contains("read-only"));
    assert!(doc.contains("does not auto-apply"));
    assert!(doc.contains("does not auto-merge"));
    assert!(doc.contains("human fun/feel"));
}

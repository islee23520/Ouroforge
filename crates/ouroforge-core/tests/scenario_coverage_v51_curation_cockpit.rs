//! Scenario Coverage v51: Curation Cockpit Regression Suite (#1855).
//!
//! State/shape-only coverage for #1852/#1853/#1854 plus Milestone 30 single
//! proposal back-compat. Local deterministic fixtures only: no network, live
//! browser, timing, trusted writes, auto-apply, auto-merge, self-approval, or
//! automated fun/release claim.

use std::path::{Path, PathBuf};

use ouroforge_core::candidate_generation::{generate_candidates, CandidateGenerationBrief};
use ouroforge_core::curation_surface::{
    build_curation_read_model, record_human_selection, replay_selection, CurationSelectionRecord,
    CURATION_READ_ONLY_BOUNDARY,
};
use ouroforge_core::generative_intake::{
    intake_brief, GenerativeBrief, GENERATIVE_INTAKE_GENERATOR, GENERATIVE_INTAKE_SOURCE,
    GRID_PUZZLE_GAME_CLASS, GRID_PUZZLE_SCHEMA_VERSION,
};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_786_000_000_000;
const SELECTION_NOW_MS: u128 = 1_786_000_001_000;
const DEMO_SELECTION_NOW_MS: u128 = 1_786_000_002_000;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative)).expect(relative)
}

fn read_candidate_brief(relative: &str) -> CandidateGenerationBrief {
    CandidateGenerationBrief::from_json_str(&read_text(relative)).expect(relative)
}

#[test]
fn v51_matrix_enumerates_required_rows_and_boundaries() {
    let matrix =
        read_json("examples/curation-cockpit-v1/scenario-coverage-v51/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "ouroforge.scenario-coverage.v51.curation-cockpit.v1"
    );
    assert_eq!(matrix["issue"], "1855");
    let boundary = matrix["boundary"].as_str().unwrap();
    for phrase in [
        "Rust/local",
        "browser/Studio read-only",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "no auto-apply",
        "no auto-merge",
        "no self-approval",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(phrase), "missing boundary {phrase}");
    }
    let rows = matrix["rows"].as_array().unwrap();
    let ids = rows
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "V51.generation.n_variant",
            "V51.curation.selection",
            "V51.curation.read_model",
            "V51.curation.readonly_block",
            "V51.demo.smoke",
            "V51.m30.backcompat",
        ]
    );
    for row in rows {
        let fixture = row["fixture"].as_str().unwrap();
        assert!(
            repo_root().join(fixture).exists(),
            "missing fixture {fixture}"
        );
    }
}

#[test]
fn v51_generation_and_curation_state_shapes_are_locked() {
    let brief =
        read_candidate_brief("examples/curation-cockpit-v1/candidate-generation-brief-v1.json");
    let set = generate_candidates(&brief, FIXED_NOW_MS).expect("candidate set generates");
    set.validate().expect("candidate set validates");
    assert_eq!(set.requested_count, 4);
    assert_eq!(set.candidates.len(), 4);
    assert!(set.proposal_only);
    assert_eq!(
        set.candidates
            .iter()
            .map(|candidate| candidate.kind.as_str())
            .collect::<Vec<_>>(),
        vec!["card", "tuning", "flavor", "store-copy"]
    );
    for candidate in &set.candidates {
        assert!(candidate.proposal_only);
        assert_eq!(candidate.proposal.proposal.status, "proposed");
        assert_eq!(candidate.proposal.proposal.verdict_status, "pending");
        assert_eq!(candidate.proposal.proposal.confidence, "unverified");
        assert!(candidate.proposal.provenance.proposal_only);
    }

    let computed = record_human_selection(
        &set,
        "human-selection-spark-v1",
        "candidate-card-spark-v1",
        "human-curator-local",
        "Best fit for the engine-part deckbuilder theme; still only provenance until review/apply.",
        SELECTION_NOW_MS,
    )
    .expect("selection records");
    let fixture_text = read_text("examples/curation-cockpit-v1/curation-selection-v1.json")
        .replace(
            "__COMPUTED_IN_TEST__",
            computed.selected_payload_digest.as_deref().unwrap(),
        );
    let selection =
        CurationSelectionRecord::from_json_str(&fixture_text).expect("selection fixture parses");
    assert_eq!(selection, computed);
    let selected = replay_selection(&set, &selection).expect("selection replays");
    assert_eq!(selected.variant_id, "candidate-card-spark-v1");

    let model = build_curation_read_model(&set, &[selection]).expect("read model builds");
    model.validate().expect("read model validates");
    assert_eq!(model.surface_boundary, CURATION_READ_ONLY_BOUNDARY);
    assert!(!model.trusted_write_authority);
    assert_eq!(
        model.allowed_actions,
        vec!["inspect-candidates", "record-selection-provenance"]
    );
}

#[test]
fn v51_negative_boundary_and_demo_smoke_are_locked() {
    let unsafe_selection = CurationSelectionRecord::from_json_str(&read_text(
        "examples/curation-cockpit-v1/curation-selection-readonly-violation.json",
    ))
    .expect("unsafe fixture parses");
    let error = unsafe_selection
        .validate()
        .expect_err("trusted write/apply authority remains rejected");
    assert!(error.to_string().contains("read-only provenance"));

    let demo_brief = read_candidate_brief(
        "examples/curation-cockpit-v1/demo/candidate-generation-brief-demo-v1.json",
    );
    let demo_set = generate_candidates(&demo_brief, FIXED_NOW_MS).expect("demo set generates");
    assert_eq!(demo_set.requested_count, 3);
    assert_eq!(
        demo_set
            .candidates
            .iter()
            .map(|candidate| candidate.variant_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "demo-card-rivet-strike-v1",
            "demo-tuning-calibrated-opening-v1",
            "demo-flavor-workshop-hum-v1",
        ]
    );
    let demo_selection = record_human_selection(
        &demo_set,
        "demo-human-selection-rivet-strike-v1",
        "demo-card-rivet-strike-v1",
        "human-curator-local-demo",
        "Chosen for the deterministic demo because it is a compact card candidate; this is provenance only and does not apply the proposal.",
        DEMO_SELECTION_NOW_MS,
    )
    .expect("demo selection records");
    replay_selection(&demo_set, &demo_selection).expect("demo selection replays");
}

#[test]
fn v51_preserves_milestone_30_single_proposal_backcompat() {
    let brief: GenerativeBrief = serde_json::from_str(&read_text(
        "examples/generative-front-door/generative-intake-brief-v1.json",
    ))
    .expect("Milestone 30 fixture parses");
    let proposal = intake_brief(&brief, FIXED_NOW_MS).expect("single proposal still accepted");

    proposal.validate().expect("single proposal validates");
    assert!(proposal.links_to(&brief).expect("provenance link checks"));
    assert_eq!(proposal.proposal.target, GRID_PUZZLE_GAME_CLASS);
    assert_eq!(proposal.proposal.status, "proposed");
    assert_eq!(proposal.proposal.verdict_status, "pending");
    assert_eq!(proposal.proposal.confidence, "unverified");
    assert_eq!(proposal.provenance.generator, GENERATIVE_INTAKE_GENERATOR);
    assert_eq!(proposal.provenance.source, GENERATIVE_INTAKE_SOURCE);
    assert!(proposal.provenance.proposal_only);

    let artifact: Value = serde_json::from_str(&proposal.proposal.to)
        .expect("proposal carries a grid-puzzle artifact");
    assert_eq!(artifact["schemaVersion"], GRID_PUZZLE_SCHEMA_VERSION);
    assert_eq!(artifact["id"], brief.puzzle_id);
}

#[test]
fn v51_doc_records_conservative_state_shape_scope() {
    let doc = read_text("docs/scenario-coverage-v51.md");
    for required in [
        "state/shape checks only",
        "Milestone 30",
        "single-proposal backward-compatibility",
        "Generated runs/artifacts remain",
        "Issues #1 and #23 remain open",
        "cargo test -p ouroforge-core --test scenario_coverage_v51_curation_cockpit",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

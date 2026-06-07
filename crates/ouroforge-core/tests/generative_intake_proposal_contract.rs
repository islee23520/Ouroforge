//! Contract test for Brief/NL Intake and Proposal Model v1 (#1593).
//!
//! Part of Generative Front Door v1 (#1592) under #1 Era F Milestone 30. These
//! tests machine-check the front-door intake contract: a well-formed brief
//! produces a validated grid-puzzle proposal with attached generation
//! provenance; a malformed brief is rejected fail-closed; and the provenance
//! links the proposal back to the exact brief that produced it.
//!
//! Boundary checks assert the proposal-only model: the proposal is never
//! promoted (it has not passed the engine room, #1594), provenance records
//! proposal-only, and intake performs no trusted write.

use std::path::PathBuf;

use ouroforge_core::generative_intake::{
    intake_brief, GenerativeBrief, GENERATIVE_INTAKE_GENERATOR, GENERATIVE_INTAKE_SOURCE,
    GRID_PUZZLE_GAME_CLASS, GRID_PUZZLE_SCHEMA_VERSION,
};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_brief(name: &str) -> GenerativeBrief {
    let path: PathBuf = repo_root()
        .join("examples/generative-front-door")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    GenerativeBrief::from_json_str(&text).expect("fixture brief parses")
}

#[test]
fn well_formed_brief_produces_a_valid_grid_puzzle_proposal() {
    let brief = read_brief("generative-intake-brief-v1.json");
    let generative = intake_brief(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    // The wrapped proposal validates against the existing model.
    generative
        .validate()
        .expect("generative proposal validates");
    assert_eq!(generative.proposal.target, GRID_PUZZLE_GAME_CLASS);
    assert_eq!(generative.proposal.created_at_unix_ms, FIXED_NOW_MS);

    // The `to` payload is the assembled grid-puzzle artifact.
    let artifact: Value = serde_json::from_str(&generative.proposal.to)
        .expect("proposal carries a grid-puzzle artifact");
    assert_eq!(artifact["schemaVersion"], GRID_PUZZLE_SCHEMA_VERSION);
    assert_eq!(artifact["id"], brief.puzzle_id);
    assert_eq!(artifact["width"], 6);
    assert_eq!(artifact["height"], 5);
    assert_eq!(artifact["win"]["type"], "all-targets-covered");
    // The author's sketch and intended solution are preserved verbatim.
    assert_eq!(artifact["rows"], serde_json::to_value(&brief.rows).unwrap());
    assert_eq!(
        artifact["intendedSolution"],
        serde_json::to_value(&brief.intended_solution).unwrap()
    );
}

#[test]
fn proposal_is_proposal_only_and_not_promoted() {
    let brief = read_brief("generative-intake-brief-v1.json");
    let generative = intake_brief(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    // A freshly generated proposal has not passed the engine room (#1594):
    // it is proposed/pending, not applied/promoted/approved.
    assert_eq!(generative.proposal.status, "proposed");
    assert_eq!(generative.proposal.verdict_status, "pending");
    assert_eq!(generative.proposal.confidence, "unverified");

    // Provenance records the proposal-only boundary and the generator identity.
    assert!(generative.provenance.proposal_only);
    assert_eq!(generative.provenance.generator, GENERATIVE_INTAKE_GENERATOR);
    assert_eq!(generative.provenance.source, GENERATIVE_INTAKE_SOURCE);
    assert_eq!(generative.provenance.game_class, GRID_PUZZLE_GAME_CLASS);
}

#[test]
fn provenance_links_proposal_to_its_brief() {
    let brief = read_brief("generative-intake-brief-v1.json");
    let generative = intake_brief(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    assert_eq!(generative.provenance.brief_id, brief.brief_id);
    assert_eq!(
        generative.provenance.brief_digest,
        brief.digest().expect("brief digest")
    );
    assert!(generative.links_to(&brief).expect("linkage check"));

    // A different brief must not link to this proposal.
    let mut other = brief.clone();
    other.brief_id = "brief-other-v1".to_string();
    assert!(!generative.links_to(&other).expect("linkage check"));
}

#[test]
fn intake_is_deterministic() {
    let brief = read_brief("generative-intake-brief-v1.json");
    let first = intake_brief(&brief, FIXED_NOW_MS).expect("accepted");
    let second = intake_brief(&brief, FIXED_NOW_MS).expect("accepted");
    assert_eq!(first, second);
}

#[test]
fn malformed_brief_is_rejected_fail_closed() {
    let brief = read_brief("generative-intake-brief-invalid.json");
    let error = intake_brief(&brief, FIXED_NOW_MS)
        .expect_err("malformed brief must be rejected fail-closed");
    assert!(
        error.to_string().contains("must be a string of length 6"),
        "unexpected error: {error}"
    );
}

#[test]
fn unsupported_game_class_is_rejected() {
    let mut brief = read_brief("generative-intake-brief-v1.json");
    brief.game_class = "deck-roguelike".to_string();
    let error =
        intake_brief(&brief, FIXED_NOW_MS).expect_err("unsupported game class must be rejected");
    assert!(
        error.to_string().contains("unsupported"),
        "unexpected error: {error}"
    );
}

#[test]
fn brief_without_a_player_is_rejected() {
    let mut brief = read_brief("generative-intake-brief-v1.json");
    // Replace the player cell with empty floor: zero players must fail closed.
    brief.rows = vec![
        "######".to_string(),
        "#@...#".to_string(),
        "#.*..#".to_string(),
        "#....#".to_string(),
        "######".to_string(),
    ];
    let error = intake_brief(&brief, FIXED_NOW_MS)
        .expect_err("a brief with no player must be rejected fail-closed");
    assert!(
        error.to_string().contains("exactly one player"),
        "unexpected error: {error}"
    );
}

#[test]
fn empty_intended_solution_is_rejected() {
    let mut brief = read_brief("generative-intake-brief-v1.json");
    brief.intended_solution = vec![];
    let error = intake_brief(&brief, FIXED_NOW_MS)
        .expect_err("a brief with no intended solution must be rejected");
    assert!(
        error.to_string().contains("intendedSolution"),
        "unexpected error: {error}"
    );
}

#[test]
fn docs_record_the_intake_contract() {
    // The intake contract is documented under Generative Front Door v1.
    let doc = std::fs::read_to_string(repo_root().join("docs/generative-front-door-v1.md"))
        .expect("generative front door doc exists");
    assert!(
        doc.contains("#1593"),
        "Generative Front Door v1 doc records the intake follow-up (#1593)"
    );
    assert!(
        doc.contains("proposals only") || doc.contains("proposals-only"),
        "doc records the proposals-only contract"
    );
}

//! Contract test for Release-Readiness Bundle and Go/No-Go Surface v1 (#1871).
//!
//! The readiness bundle composes existing gate evidence and Milestone 25/44
//! provenance into Rust/local read-only evidence. Human go/no-go is separately
//! required and recorded; no browser/Studio write, auto-merge, release authority,
//! or production/fun/quality/Godot-parity claim is granted.

use std::path::{Path, PathBuf};

use ouroforge_core::release_readiness::{
    build_release_readiness_bundle, record_release_go_no_go, ReleaseGoNoGoInput,
    ReleaseGoNoGoRecord, ReleaseReadinessBundle, ReleaseReadinessInput, ReleaseReadinessStatus,
    RELEASE_READINESS_BOUNDARY,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_fixture(name: &str) -> String {
    let path = repo_root().join("examples/release-readiness-v1").join(name);
    std::fs::read_to_string(path).expect("release-readiness fixture exists")
}

#[test]
fn complete_candidate_resolves_to_read_only_ready_bundle() {
    let input =
        ReleaseReadinessInput::from_json_str(&read_fixture("complete-ready-bundle.input.json"))
            .expect("input parses");
    let generated = build_release_readiness_bundle(&input).expect("bundle builds");
    let fixture =
        ReleaseReadinessBundle::from_json_str(&read_fixture("complete-ready-bundle.fixture.json"))
            .expect("bundle fixture parses");

    fixture.validate().expect("fixture validates");
    assert_eq!(generated, fixture);
    assert_eq!(generated.status, ReleaseReadinessStatus::Ready);
    assert_eq!(generated.gate_results.len(), 8);
    assert!(generated.missing_gate_kinds.is_empty());
    assert!(generated.blocked_reasons.is_empty());
    assert!(generated.human_go_no_go_required);
    assert!(!generated.human_go_no_go_recorded);
    assert!(generated.read_only_surface);
    assert!(!generated.release_authority_granted);
    assert!(!generated.auto_merge_allowed);
    assert!(!generated.trusted_write_authority);
    assert_eq!(generated.boundary, RELEASE_READINESS_BOUNDARY);
}

#[test]
fn missing_or_blocked_gate_blocks_readiness_bundle() {
    let input = ReleaseReadinessInput::from_json_str(&read_fixture("missing-gate.input.json"))
        .expect("input parses");
    let generated = build_release_readiness_bundle(&input).expect("bundle builds");
    let fixture = ReleaseReadinessBundle::from_json_str(&read_fixture("missing-gate.fixture.json"))
        .expect("bundle fixture parses");

    fixture.validate().expect("fixture validates");
    assert_eq!(generated, fixture);
    assert_eq!(generated.status, ReleaseReadinessStatus::Blocked);
    assert_eq!(generated.missing_gate_kinds, ["steam-export-readiness"]);
    assert!(generated
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("balance blocked")));
    assert!(generated
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("fun-feel-human-gate missing")));
    assert!(generated.human_go_no_go_required);
    assert!(!generated.human_go_no_go_recorded);
}

#[test]
fn human_go_no_go_record_is_required_and_read_only() {
    let input =
        ReleaseReadinessInput::from_json_str(&read_fixture("complete-ready-bundle.input.json"))
            .expect("input parses");
    let bundle = build_release_readiness_bundle(&input).expect("bundle builds");
    assert!(bundle.human_go_no_go_required);
    assert!(!bundle.human_go_no_go_recorded);

    let decision = ReleaseGoNoGoInput::from_json_str(&read_fixture("go-no-go.input.json"))
        .expect("decision parses");
    let generated = record_release_go_no_go(&bundle, &decision).expect("decision records");
    let fixture = ReleaseGoNoGoRecord::from_json_str(&read_fixture("go-no-go.fixture.json"))
        .expect("record fixture parses");

    fixture.validate().expect("fixture validates");
    assert_eq!(generated, fixture);
    assert!(generated.release_ready);
    assert!(generated.human_confirmed);
    assert!(generated.read_only_surface);
    assert!(!generated.release_authority_granted);
    assert!(!generated.auto_merge_allowed);
    assert!(!generated.trusted_write_authority);
}

#[test]
fn go_no_go_fails_closed_for_non_human_or_authority_drift() {
    let input =
        ReleaseReadinessInput::from_json_str(&read_fixture("complete-ready-bundle.input.json"))
            .expect("input parses");
    let bundle = build_release_readiness_bundle(&input).expect("bundle builds");
    let mut decision = ReleaseGoNoGoInput::from_json_str(&read_fixture("go-no-go.input.json"))
        .expect("decision parses");

    decision.human_confirmed = false;
    let err = record_release_go_no_go(&bundle, &decision).expect_err("human is mandatory");
    assert!(err.to_string().contains("confirmed by a human"));

    decision.human_confirmed = true;
    decision.auto_merge_requested = true;
    let err = record_release_go_no_go(&bundle, &decision).expect_err("auto-merge is blocked");
    assert!(err
        .to_string()
        .contains("grant no release/merge/write authority"));
}

#[test]
fn generated_state_boundary_and_governance_anchor_are_enforced() {
    let mut input =
        ReleaseReadinessInput::from_json_str(&read_fixture("complete-ready-bundle.input.json"))
            .expect("input parses");

    input.generated_state_policy = "generated artifacts may be tracked".to_string();
    let err = build_release_readiness_bundle(&input).expect_err("generated-state drift blocks");
    assert!(err.to_string().contains("untracked unless fixture-scoped"));

    input.generated_state_policy =
        "Generated runs/artifacts remain untracked unless fixture-scoped".to_string();
    input.boundary = "Rust/local read-only bundle".to_string();
    let err = build_release_readiness_bundle(&input).expect_err("boundary drift blocks");
    assert!(err
        .to_string()
        .contains("release-readiness boundary missing"));
}

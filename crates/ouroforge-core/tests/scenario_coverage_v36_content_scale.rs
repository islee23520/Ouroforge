//! Scenario Coverage v36 — Content-at-Scale Regression Suite (#1654).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. This enumerated, fixture-scoped regression suite locks the
//! behavior of campaign-scale generation (#1649), dedup/novelty metrics (#1650),
//! the whole-game difficulty curve (#1651), and the curation gate (#1652), and
//! guards the backward compatibility of single-level Milestone 30 generation.
//!
//! It asserts **states and shapes only** — no flaky or timing-based assertions —
//! so a breaking change to the content-at-scale pipeline fails CI. Coverage
//! numbering continues from v35 onward (v37/v38 are owned by other milestones).

use std::path::{Path, PathBuf};

use ouroforge_core::content_difficulty_curve::{verify_curve, CurveInput};
use ouroforge_core::content_novelty::{compute_novelty, DEFAULT_NOVELTY_THRESHOLD};
use ouroforge_core::content_scale_generation::{generate_campaign, CampaignBrief};
use ouroforge_core::generative_intake::{intake_brief, GenerativeBrief};
use ouroforge_evaluator::content_curation_gate::{
    evaluate_content_curation_check, ContentCurationCheck, ContentCurationGateState,
};
use serde_json::Value;

const NOW: u128 = 1_725_000_000_000;
const MATRIX: &str = "examples/content-scale-v1/scenario-coverage-v36/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v36.md";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read_text(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|_| panic!("read {rel}"))
}
fn read_json(rel: &str) -> Value {
    serde_json::from_str(&read_text(rel)).expect("json")
}
fn matrix() -> Value {
    read_json(MATRIX)
}
fn path_of(section: &str, key: &str) -> String {
    matrix()[section][key]
        .as_str()
        .unwrap_or_else(|| panic!("matrix {section}.{key}"))
        .to_string()
}
fn campaign(section: &str, key: &str) -> CampaignBrief {
    CampaignBrief::from_json_str(&read_text(&path_of(section, key))).expect("campaign brief")
}

#[test]
fn v36_matrix_pinned() {
    let m = matrix();
    assert_eq!(
        m["schemaVersion"],
        "scenario-coverage-v36-content-scale-matrix-v1"
    );
    assert_eq!(m["issue"], 1654);
}

#[test]
fn v36_all_referenced_fixtures_exist() {
    let m = matrix();
    for section in [
        "backwardCompat",
        "generation",
        "novelty",
        "curve",
        "curation",
    ] {
        for (_k, v) in m[section].as_object().unwrap() {
            let p = root().join(v.as_str().unwrap());
            assert!(p.is_file(), "fixture {}", p.display());
        }
    }
}

#[test]
fn v36_backward_compat_single_level_m30_intake() {
    // Backward compatibility: single-level Milestone 30 generation remains valid,
    // and a malformed single-level brief is still rejected fail-closed.
    let valid = GenerativeBrief::from_json_str(&read_text(&path_of(
        "backwardCompat",
        "singleLevelM30Valid",
    )))
    .expect("brief parses");
    let proposal = intake_brief(&valid, NOW).expect("single-level intake remains valid");
    assert_eq!(proposal.proposal.status, "proposed");
    assert_eq!(proposal.proposal.verdict_status, "pending");

    let invalid = GenerativeBrief::from_json_str(&read_text(&path_of(
        "backwardCompat",
        "singleLevelM30Invalid",
    )))
    .expect("brief parses");
    assert!(intake_brief(&invalid, NOW).is_err());
}

#[test]
fn v36_generation_valid_and_invalid() {
    let valid = generate_campaign(&campaign("generation", "campaignValid"), NOW)
        .expect("valid campaign generates");
    assert!(valid.proposals.len() >= 2);
    assert!(valid.covers_both_genres());

    let invalid = campaign("generation", "campaignInvalid");
    assert!(generate_campaign(&invalid, NOW).is_err());
}

#[test]
fn v36_novelty_states() {
    let mixed = generate_campaign(&campaign("novelty", "mixed"), NOW).expect("gen");
    let mixed_report =
        compute_novelty(&mixed, DEFAULT_NOVELTY_THRESHOLD, &Default::default()).expect("novelty");
    assert!(!mixed_report.low_novelty);
    assert!(mixed_report.duplicate_count >= 1);

    let low = generate_campaign(&campaign("novelty", "low"), NOW).expect("gen");
    let low_report =
        compute_novelty(&low, DEFAULT_NOVELTY_THRESHOLD, &Default::default()).expect("novelty");
    assert!(low_report.low_novelty);
}

#[test]
fn v36_curve_states() {
    let pass =
        verify_curve(&CurveInput::from_json_str(&read_text(&path_of("curve", "pass"))).unwrap())
            .expect("curve");
    assert!(pass.passed);

    let spike =
        verify_curve(&CurveInput::from_json_str(&read_text(&path_of("curve", "spike"))).unwrap())
            .expect("curve");
    assert!(!spike.passed);
    assert!(spike.spikes >= 1 && spike.regressions >= 1);

    let missing =
        CurveInput::from_json_str(&read_text(&path_of("curve", "missingEvidence"))).unwrap();
    assert!(verify_curve(&missing).is_err());
}

#[test]
fn v36_curation_gate_outcomes() {
    let cases = [
        ("admit", ContentCurationGateState::Pass),
        ("unsolvable", ContentCurationGateState::Unsolvable),
        ("imbalanced", ContentCurationGateState::Imbalanced),
        ("lowNovelty", ContentCurationGateState::LowNovelty),
        ("curveSpike", ContentCurationGateState::CurveViolation),
        ("malformed", ContentCurationGateState::MalformedEvidence),
        (
            "missingDimension",
            ContentCurationGateState::MalformedEvidence,
        ),
    ];
    for (key, expected) in cases {
        let check = ContentCurationCheck::from_json_str(&read_text(&path_of("curation", key)))
            .unwrap_or_else(|_| panic!("curation fixture {key} parses"));
        let verdict = evaluate_content_curation_check(&check);
        assert_eq!(verdict.state, expected, "curation case {key}");
    }
}

#[test]
fn v36_doc_records_the_suite() {
    let doc = read_text(DOC);
    assert!(doc.contains("#1654"), "v36 doc records this issue (#1654)");
    assert!(
        doc.contains("backward")
            && doc.contains("curation")
            && doc.contains("novelty")
            && doc.contains("curve"),
        "v36 doc records the covered areas"
    );
}

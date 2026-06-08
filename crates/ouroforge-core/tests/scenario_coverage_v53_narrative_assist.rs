//! Scenario Coverage v53: Narrative Assist Regression Suite (#1867).
//!
//! State/shape-only coverage for #1864/#1865/#1866 plus a Milestone 39
//! narrative-system backward-compatibility golden. Local deterministic fixtures
//! only: no network, live browser, timing, trusted writes, auto-apply,
//! auto-merge, self-approval, or automated fun/tone/quality/release claim.

use std::path::{Path, PathBuf};

use ouroforge_core::narrative_candidate::{
    generate_narrative_candidates, NarrativeCandidateBrief, NarrativeCandidateSet,
    NARRATIVE_CANDIDATE_SCHEMA_VERSION, NARRATIVE_TONE_HUMAN_BOUNDARY,
};
use ouroforge_core::narrative_integration::{
    integrate_human_selected_candidate, record_human_narrative_selection,
    NarrativeIntegrationRecord, NarrativeIntegrationSelectionRecord,
    NARRATIVE_INTEGRATION_STATUS_READY,
};
use ouroforge_core::narrative_system::{
    NarrativeDefinition, NARRATIVE_SYSTEM_BOUNDARY, NARRATIVE_SYSTEM_SCHEMA_VERSION,
};
use serde_json::{json, Value};

const FIXED_NOW_MS: u128 = 1_786_400_000_000;
const SELECTION_NOW_MS: u128 = 1_786_500_001_000;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative)).expect(relative)
}

fn read_brief(relative: &str) -> NarrativeCandidateBrief {
    NarrativeCandidateBrief::from_json_str(&read_text(relative)).expect(relative)
}

fn generated_candidate_set() -> NarrativeCandidateSet {
    let brief = read_brief("examples/narrative-assist-v1/narrative-candidate-brief-v1.json");
    generate_narrative_candidates(&brief, FIXED_NOW_MS).expect("candidate set generates")
}

#[test]
fn v53_matrix_enumerates_required_rows_and_boundaries() {
    let matrix =
        read_json("examples/narrative-assist-v1/scenario-coverage-v53/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "ouroforge.scenario-coverage.v53.narrative-assist.v1"
    );
    assert_eq!(matrix["issue"], "1867");
    let boundary = matrix["boundary"].as_str().unwrap();
    for phrase in [
        "Rust/local",
        "browser/Studio surfaces are read-only",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "state/shape checks only",
        "generation is proposal-only",
        "review/apply/trust-gradient",
        "no automated fun score",
        "no tone/quality claim",
        "no production-readiness claim",
        "no Godot replacement claim",
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
        .map(|row| row["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "V53.candidate_generation.valid",
            "V53.candidate_generation.rejects_malformed",
            "V53.candidate_generation.proposal_only_boundary",
            "V53.integration.human_selection",
            "V53.integration.readonly_trusted_write_blocked",
            "V53.demo.smoke",
            "V53.m39_narrative.backcompat",
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
fn v53_candidate_generation_states_are_locked() {
    let set = generated_candidate_set();
    set.validate().expect("candidate set validates");
    assert_eq!(set.schema_version, NARRATIVE_CANDIDATE_SCHEMA_VERSION);
    assert_eq!(set.candidate_count, 3);
    assert_eq!(set.candidates.len(), 3);
    assert!(set.proposal_only);
    assert_eq!(set.human_tone_boundary, NARRATIVE_TONE_HUMAN_BOUNDARY);
    assert_eq!(
        set.candidates
            .iter()
            .map(|c| c.class.as_str())
            .collect::<Vec<_>>(),
        vec!["theme-arc-beat", "dialogue-variant", "flavor-text"]
    );
    for candidate in &set.candidates {
        assert!(candidate.proposal_only);
        assert_eq!(candidate.proposal.proposal.status, "proposed");
        assert_eq!(candidate.proposal.proposal.verdict_status, "pending");
        assert_eq!(candidate.proposal.proposal.confidence, "unverified");
        assert!(candidate.proposal.provenance.proposal_only);
    }

    let malformed =
        read_brief("examples/narrative-assist-v1/narrative-candidate-malformed-v1.json");
    let error = generate_narrative_candidates(&malformed, FIXED_NOW_MS)
        .expect_err("candidateCount drift must fail closed");
    assert!(error.to_string().contains("candidateCount"));

    let drift: NarrativeCandidateSet = serde_json::from_str(&read_text(
        "examples/narrative-assist-v1/narrative-candidate-proposal-only-v1.json",
    ))
    .expect("proposal-only drift fixture parses");
    let error = drift
        .validate()
        .expect_err("proposalOnly=false must fail closed");
    assert!(error.to_string().contains("proposalOnly"));
}

#[test]
fn v53_human_integration_and_demo_states_are_locked() {
    let set = generated_candidate_set();
    let selection = record_human_narrative_selection(
        &set,
        "narrative-demo-selection-harbor-repair-v1",
        "clockwork-harbor-demo",
        "theme-arc-harbor-repair-v1",
        "human-curator-local",
        "Demo curator selected this beat as provenance for review/apply; no trusted source write is performed.",
        SELECTION_NOW_MS,
    )
    .expect("selection records");
    let integration = integrate_human_selected_candidate(
        &set,
        &selection,
        "narrative-demo-integration-harbor-repair-v1",
    )
    .expect("integration records");

    let fixture_integration = NarrativeIntegrationRecord::from_json_str(&read_text(
        "examples/narrative-assist-v1/demo/integration-provenance-v1.json",
    ))
    .expect("integration fixture parses");
    fixture_integration
        .validate()
        .expect("integration fixture validates");
    assert_eq!(fixture_integration, integration);
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

    let unsafe_selection = NarrativeIntegrationSelectionRecord::from_json_str(&read_text(
        "examples/narrative-integration-v1/narrative-selection-readonly-violation.json",
    ))
    .expect("unsafe selection fixture parses");
    let error = unsafe_selection
        .validate()
        .expect_err("trusted write/apply drift must fail closed");
    assert!(error.to_string().contains("read-only provenance"));

    let fixture_set: NarrativeCandidateSet = serde_json::from_str(&read_text(
        "examples/narrative-assist-v1/demo/candidate-set-v1.json",
    ))
    .expect("demo candidate set fixture parses");
    assert_eq!(fixture_set, set);
    let manifest = read_json("examples/narrative-assist-v1/demo/manifest-v1.json");
    assert_eq!(manifest["networkRequired"], false);
    assert_eq!(manifest["liveBrowserRequired"], false);
    assert_eq!(manifest["proposalOnly"], true);
    assert_eq!(manifest["reviewApplyRequired"], true);
    assert_eq!(manifest["trustedWriteAuthority"], false);
}

#[test]
fn v53_milestone39_narrative_backcompat_golden_remains_valid() {
    let golden = read_json(
        "examples/narrative-assist-v1/scenario-coverage-v53/milestone39-narrative-backcompat.fixture.json",
    );
    assert_eq!(
        golden["schemaVersion"],
        "ouroforge.scenario-coverage.v53.m39-narrative-backcompat.v1"
    );
    let source = golden["sourceDefinition"].as_str().unwrap();
    let def = NarrativeDefinition::from_json_str(&read_text(source))
        .expect("Milestone 39 definition remains valid");
    assert_eq!(def.schema_version, NARRATIVE_SYSTEM_SCHEMA_VERSION);
    assert_eq!(def.story_id, golden["storyId"]);
    assert_eq!(def.boundary, NARRATIVE_SYSTEM_BOUNDARY);

    let state = def.initial_state();
    assert_eq!(
        state.current_node.as_deref(),
        golden["initialNode"].as_str()
    );
    let after_accept = def
        .advance(&state, golden["acceptChoice"].as_str())
        .expect("accept branch still advances");
    def.validate_state(&after_accept)
        .expect("advanced state remains valid");

    let expected = &golden["expectedAfterAccept"];
    assert_eq!(
        after_accept.current_node.as_deref(),
        expected["currentNode"].as_str()
    );
    assert_eq!(after_accept.visited_nodes, vec!["intro"]);
    assert_eq!(after_accept.flags.get("metKing"), Some(&true));
    assert_eq!(after_accept.flags.get("acceptedQuest"), Some(&false));
    assert_eq!(after_accept.flags.get("audienceLogged"), Some(&true));
    assert_eq!(after_accept.flags.get("questBlessed"), Some(&false));
    assert!(after_accept.fired_events.contains("logAudience"));
    assert!(!after_accept.is_ended());

    let read = def.read_model(&after_accept);
    assert_eq!(
        json!({
            "storyId": read.story_id,
            "currentNode": read.current_node,
            "ended": read.ended,
            "firedEventCount": read.fired_event_count,
            "boundary": read.boundary,
        }),
        golden["expectedReadModel"]
    );
}

#[test]
fn v53_doc_records_conservative_state_shape_scope() {
    let doc = read_text("docs/scenario-coverage-v53.md");
    for required in [
        "Scenario Coverage v53",
        "state/shape\nchecks only",
        "backward-compatibility golden",
        "Milestone 39 narrative system",
        "Generated runs/artifacts remain",
        "Issues #1 and\n#23 remain open",
        "cargo test -p ouroforge-core --test scenario_coverage_v53_narrative_assist",
        "does not run a live browser",
        "auto-merge",
        "Godot parity",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

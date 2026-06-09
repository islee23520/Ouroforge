//! Optional human channel surface tests for #2043 / Era L M72.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    record_optional_human_override, record_optional_taste_feedback,
    render_optional_human_oversight_view, OptionalHumanOverrideInput, OptionalHumanOversightInput,
    OptionalHumanTasteFeedbackInput, OPTIONAL_HUMAN_OVERRIDE_INPUT_SCHEMA_VERSION,
    OPTIONAL_HUMAN_OVERRIDE_RECORD_SCHEMA_VERSION, OPTIONAL_HUMAN_OVERSIGHT_INPUT_SCHEMA_VERSION,
    OPTIONAL_HUMAN_OVERSIGHT_VIEW_SCHEMA_VERSION,
    OPTIONAL_HUMAN_TASTE_FEEDBACK_INPUT_SCHEMA_VERSION,
    OPTIONAL_HUMAN_TASTE_FEEDBACK_RECORD_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn refs() -> Vec<String> {
    vec![
        "openchrome:seeds/dogfood-deckbuilder.yaml".to_string(),
        "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
        "examples/real-title-dogfood-v1/run/journal.md".to_string(),
        "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
        "examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json".to_string(),
        "source-apply:patch-preview.v1".to_string(),
        "trust-gradient:risk-tier-v1".to_string(),
    ]
}

fn oversight_input() -> OptionalHumanOversightInput {
    let input: OptionalHumanOversightInput = serde_json::from_str(&read_text(
        "examples/real-title-dogfood-v1/optional-human-channel-v1/surface.fixture.json",
    ))
    .expect("surface fixture parses");
    assert_eq!(
        input.schema_version,
        OPTIONAL_HUMAN_OVERSIGHT_INPUT_SCHEMA_VERSION
    );
    input
}

fn override_input() -> OptionalHumanOverrideInput {
    OptionalHumanOverrideInput {
        schema_version: OPTIONAL_HUMAN_OVERRIDE_INPUT_SCHEMA_VERSION.to_string(),
        title_id: "era-i-engine-builder-deckbuilder".to_string(),
        stuck_loop_ref: "ledger.jsonl:m72-explicit-stuck-loop".to_string(),
        override_reason: "operator recorded a stop/resume nudge after repeated identical diagnosis"
            .to_string(),
        operator_provenance_ref: "journal.md:m72-override-provenance".to_string(),
        evidence_refs: refs(),
        blocks_autonomous_loop: false,
        trusted_write_requested: false,
        source_apply_requested: false,
        no_new_data_plane: true,
    }
}

fn taste_feedback_input() -> OptionalHumanTasteFeedbackInput {
    OptionalHumanTasteFeedbackInput {
        schema_version: OPTIONAL_HUMAN_TASTE_FEEDBACK_INPUT_SCHEMA_VERSION.to_string(),
        title_id: "era-i-engine-builder-deckbuilder".to_string(),
        feedback_id: "m72-taste-note-cascade-readability".to_string(),
        feedback_text: "The score cascade is readable, but the shop reroll affordance still feels low-salience.".to_string(),
        m57_curation_ref: "M57 curation cockpit human selection provenance".to_string(),
        m58_playtest_ref: "M58 playtest fun-feel provenance".to_string(),
        evidence_refs: refs(),
        auto_apply_requested: false,
        source_apply_requested: false,
        no_new_data_plane: true,
    }
}

#[test]
fn oversight_view_is_read_only_non_blocking_and_backed_by_existing_artifacts() {
    let view =
        render_optional_human_oversight_view(&oversight_input()).expect("oversight view renders");

    assert_eq!(
        view.schema_version,
        OPTIONAL_HUMAN_OVERSIGHT_VIEW_SCHEMA_VERSION
    );
    assert!(view.read_only);
    assert!(view.optional);
    assert!(!view.blocks_autonomous_loop);
    assert!(!view.trusted_writes_performed);
    assert!(view.loop_completed_without_human);
    let evidence = view.evidence_refs.join("\n").to_ascii_lowercase();
    for required in [
        "openchrome",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
    ] {
        assert!(evidence.contains(required), "missing {required}");
    }
    assert!(view
        .allowed_actions
        .contains(&"view_stage_health".to_string()));
    assert!(view
        .forbidden_actions
        .contains(&"trusted_write".to_string()));
    assert!(view.forbidden_actions.contains(&"source_apply".to_string()));
    assert!(view.forbidden_actions.contains(&"auto_apply".to_string()));
}

#[test]
fn override_records_provenance_only_without_loop_or_source_authority() {
    let record = record_optional_human_override(&override_input())
        .expect("override records provenance only");

    assert_eq!(
        record.schema_version,
        OPTIONAL_HUMAN_OVERRIDE_RECORD_SCHEMA_VERSION
    );
    assert!(record.recorded_as_provenance_only);
    assert!(!record.blocks_autonomous_loop);
    assert!(!record.trusted_writes_performed);
    assert!(!record.source_apply_performed);
    assert!(record.operator_provenance_ref.contains("journal.md"));
}

#[test]
fn taste_feedback_reuses_m57_m58_provenance_and_never_auto_applies() {
    let record = record_optional_taste_feedback(&taste_feedback_input())
        .expect("taste feedback records provenance");

    assert_eq!(
        record.schema_version,
        OPTIONAL_HUMAN_TASTE_FEEDBACK_RECORD_SCHEMA_VERSION
    );
    assert!(record.recorded_as_provenance_only);
    assert!(!record.auto_applied);
    assert!(!record.source_apply_performed);
    assert!(record.ring2_human_taste_verdict_required);
    assert!(record.m57_curation_ref.contains("M57"));
    assert!(record.m58_playtest_ref.contains("M58"));
}

#[test]
fn drift_to_blocking_write_or_auto_apply_fails_closed() {
    let mut oversight = oversight_input();
    oversight.loop_completed_without_human = false;
    let error = render_optional_human_oversight_view(&oversight)
        .expect_err("hidden human dependency rejected");
    assert!(error.to_string().contains("without human input"));

    let mut override_input = override_input();
    override_input.trusted_write_requested = true;
    let error = record_optional_human_override(&override_input)
        .expect_err("trusted write override rejected");
    assert!(error.to_string().contains("trusted write"));

    let mut feedback = taste_feedback_input();
    feedback.auto_apply_requested = true;
    let error = record_optional_taste_feedback(&feedback)
        .expect_err("auto-applied taste feedback rejected");
    assert!(error.to_string().contains("never auto-apply"));
}

#[test]
fn docs_record_surface_scope_and_guardrails() {
    let doc = read_text("docs/optional-human-channel-surface-v1.md").to_ascii_lowercase();
    for required in [
        "read-only stage health",
        "blockers",
        "diagnosis",
        "attribution",
        "non-blocking human nudge",
        "m57",
        "m58",
        "never auto-applied",
        "zero human input",
        "source-apply",
        "four gates plus design-integrity",
        "trust-gradient",
        "high-risk/source-affecting changes remain queued",
        "no new verification engine",
        "no new data plane",
        "no new store",
        "rust kernel remains the data plane",
        "elixir executor remains unchanged",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

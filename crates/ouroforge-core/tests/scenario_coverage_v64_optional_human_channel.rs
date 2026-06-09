//! Scenario Coverage v64 regression suite for #2045 / Era L M72.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    record_optional_human_override, record_optional_taste_feedback,
    render_optional_human_oversight_view, run_optional_human_channel_demo,
    OptionalHumanChannelDemoInput, OptionalHumanOverrideInput, OptionalHumanOversightInput,
    OptionalHumanTasteFeedbackInput,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn demo_input() -> OptionalHumanChannelDemoInput {
    OptionalHumanChannelDemoInput::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/optional-human-channel-v1/demo.fixture.json",
    ))
    .expect("demo fixture validates")
}

fn oversight_input() -> OptionalHumanOversightInput {
    serde_json::from_str(&read_text(
        "examples/real-title-dogfood-v1/optional-human-channel-v1/surface.fixture.json",
    ))
    .expect("surface fixture parses")
}

#[test]
fn v64_matrix_records_optional_human_channel_rows() {
    let matrix =
        read_json("examples/real-title-dogfood-v1/scenario-coverage-v64/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v64-optional-human-channel-v1"
    );
    assert_eq!(matrix["coverageVersion"], 64);
    assert_eq!(matrix["titleId"], "era-i-engine-builder-deckbuilder");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "oversight-surface-read-only",
        "loop-completes-zero-human-input",
        "optional-human-run-never-waits",
        "override-provenance-no-source-authority",
        "taste-feedback-never-auto-applies",
        "coverage-v64-boundaries",
    ] {
        assert!(ids.contains(required), "missing v64 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 80);
    }

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "test-only",
        "openchrome",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage attribution",
        "source-apply",
        "trust-gradient",
        "no new verification engine",
        "no new data plane",
        "no new persistent store",
        "zero human input",
        "never auto-applied",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v64_surface_is_read_only_and_loop_completes_without_human() {
    let view = render_optional_human_oversight_view(&oversight_input()).expect("view renders");
    assert!(view.read_only);
    assert!(view.optional);
    assert!(!view.blocks_autonomous_loop);
    assert!(!view.trusted_writes_performed);
    assert!(view.loop_completed_without_human);

    let demo = demo_input();
    assert!(demo.no_human_run.completed);
    assert!(!demo.no_human_run.waited_for_human);
    assert!(!demo.no_human_run.human_input_observed);
    assert!(demo
        .no_human_run
        .command
        .contains("cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2"));
}

#[test]
fn v64_optional_human_run_never_waits_and_records_provenance() {
    let report = run_optional_human_channel_demo(&demo_input()).expect("demo report");
    assert!(report.both_runs_completed);
    assert!(report.loop_never_waited_for_human);
    assert!(report.human_input_optional);
    assert!(!report.trusted_writes_from_surface);
    assert!(report.override_record.recorded_as_provenance_only);
    assert!(report.taste_feedback_record.recorded_as_provenance_only);
}

#[test]
fn v64_blocks_trusted_write_and_blocking_regressions() {
    let mut input = oversight_input();
    input.no_trusted_writes = false;
    let error =
        render_optional_human_oversight_view(&input).expect_err("trusted write drift rejected");
    assert!(error.to_string().contains("trusted writes"));

    let mut demo = demo_input();
    demo.with_human_run.waited_for_human = true;
    let error = run_optional_human_channel_demo(&demo).expect_err("blocking drift rejected");
    assert!(error.to_string().contains("never wait"));

    let mut override_input: OptionalHumanOverrideInput = demo_input().override_input;
    override_input.source_apply_requested = true;
    let error = record_optional_human_override(&override_input)
        .expect_err("source-apply override rejected");
    assert!(error.to_string().contains("source-apply"));
}

#[test]
fn v64_taste_feedback_never_auto_applies() {
    let record = record_optional_taste_feedback(&demo_input().taste_feedback_input)
        .expect("taste feedback record");
    assert!(record.recorded_as_provenance_only);
    assert!(!record.auto_applied);
    assert!(!record.source_apply_performed);
    assert!(record.ring2_human_taste_verdict_required);

    let mut feedback: OptionalHumanTasteFeedbackInput = demo_input().taste_feedback_input;
    feedback.auto_apply_requested = true;
    let error = record_optional_taste_feedback(&feedback)
        .expect_err("auto-applied taste feedback rejected");
    assert!(error.to_string().contains("never auto-apply"));
}

#[test]
fn v64_docs_preserve_autonomy_and_pipeline_boundaries() {
    let doc =
        read_text("docs/scenario-coverage-v64-optional-human-channel.md").to_ascii_lowercase();
    for required in [
        "coverage v64",
        "read-only",
        "zero human input",
        "loop never waits",
        "trusted writes",
        "source-apply",
        "auto-apply",
        "m57",
        "m58",
        "never auto-applied",
        "openchrome",
        "four gates plus",
        "design-integrity",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage attribution",
        "trust-gradient",
        "no new verification engine",
        "no new data plane",
        "no new persistent store",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

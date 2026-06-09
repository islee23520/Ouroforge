//! Optional human channel demo tests for #2044 / Era L M72.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    run_optional_human_channel_demo, OptionalHumanChannelDemoInput,
    OPTIONAL_HUMAN_CHANNEL_DEMO_INPUT_SCHEMA_VERSION,
    OPTIONAL_HUMAN_CHANNEL_DEMO_REPORT_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn demo_input() -> OptionalHumanChannelDemoInput {
    OptionalHumanChannelDemoInput::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/optional-human-channel-v1/demo.fixture.json",
    ))
    .expect("demo fixture validates")
}

#[test]
fn demo_compares_no_human_and_optional_human_runs_without_waiting() {
    let input = demo_input();
    assert_eq!(
        input.schema_version,
        OPTIONAL_HUMAN_CHANNEL_DEMO_INPUT_SCHEMA_VERSION
    );
    let report = run_optional_human_channel_demo(&input).expect("demo report renders");

    assert_eq!(
        report.schema_version,
        OPTIONAL_HUMAN_CHANNEL_DEMO_REPORT_SCHEMA_VERSION
    );
    assert!(report.both_runs_completed);
    assert!(report.loop_never_waited_for_human);
    assert!(report.human_input_optional);
    assert!(!report.trusted_writes_from_surface);
    assert!(!input.no_human_run.human_input_observed);
    assert!(input.with_human_run.human_input_observed);
}

#[test]
fn demo_records_optional_inputs_as_provenance_only() {
    let report = run_optional_human_channel_demo(&demo_input()).expect("demo report renders");

    assert!(report.oversight_view.read_only);
    assert!(!report.oversight_view.blocks_autonomous_loop);
    assert!(report.override_record.recorded_as_provenance_only);
    assert!(!report.override_record.trusted_writes_performed);
    assert!(!report.override_record.source_apply_performed);
    assert!(report.taste_feedback_record.recorded_as_provenance_only);
    assert!(!report.taste_feedback_record.auto_applied);
    assert!(!report.taste_feedback_record.source_apply_performed);
    assert!(
        report
            .taste_feedback_record
            .ring2_human_taste_verdict_required
    );
}

#[test]
fn demo_reuses_existing_pipeline_and_real_title_command() {
    let input = demo_input();
    assert!(input
        .no_human_run
        .command
        .contains("cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2"));
    let report = run_optional_human_channel_demo(&input).expect("demo report renders");
    let refs = report.comparison_refs.join("\n").to_ascii_lowercase();
    for required in [
        "openchrome",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "m57",
        "m58",
    ] {
        assert!(refs.contains(required), "missing {required}");
    }
}

#[test]
fn drift_to_waiting_or_trusted_write_fails_closed() {
    let mut input = demo_input();
    input.with_human_run.waited_for_human = true;
    let error = run_optional_human_channel_demo(&input).expect_err("waiting loop rejected");
    assert!(error.to_string().contains("never wait"));

    let mut input = demo_input();
    input.with_human_run.trusted_writes_from_surface = true;
    let error = run_optional_human_channel_demo(&input).expect_err("trusted write rejected");
    assert!(error.to_string().contains("trusted writes"));
}

#[test]
fn docs_record_demo_scope_and_guardrails() {
    let doc = read_text("docs/optional-human-channel-demo-v1.md").to_ascii_lowercase();
    for required in [
        "no-human autonomous run",
        "optional human spot-check",
        "non-blocking override",
        "taste/fun-feedback provenance",
        "both runs complete",
        "never waits",
        "trusted writes",
        "source-apply",
        "auto-apply",
        "no new store",
        "openchrome",
        "four gates plus",
        "design-integrity",
        "ledger.jsonl",
        "loop-coverage attribution",
        "trust-gradient",
        "fun/taste verdicts and release go/no-go remain human",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

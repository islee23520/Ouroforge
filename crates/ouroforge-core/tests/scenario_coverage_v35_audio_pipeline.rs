//! Scenario Coverage v35 — Audio Pipeline Regression Suite (#1646).
//!
//! Locks Audio Generation and Audio-QA v1 behavior: audio proposal generation
//! (#1642), the audio-QA check (#1643), adaptive-audio runtime hooks (#1644), and
//! the backward-compatibility guarantee that the existing runtime audio-intent
//! emission remains valid. State/shape assertions only — no flaky or timing-based
//! checks.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use ouroforge_core::audio_generation::{generate_audio, AudioGenerationBrief};
use ouroforge_core::audio_hooks::{AudioHookSet, AudioHookSignals};
use ouroforge_core::audio_qa::AudioQaArtifact;
use ouroforge_core::behavior_runtime::{
    BehaviorArtifact, BehaviorExecutionInput, BehaviorWorldState,
};
use serde::Deserialize;
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn coverage_root() -> PathBuf {
    repo_root().join("examples/audio-pipeline-v1/scenario-coverage-v35")
}

fn read_text(relative: &str) -> String {
    let path = coverage_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn matrix() -> Value {
    serde_json::from_str(&read_text("matrix.fixture.json")).expect("matrix parses")
}

fn reject_reason(case: &Value) -> &str {
    case["rejectReason"]
        .as_str()
        .expect("rejected case has a reason")
}

#[test]
fn v35_matrix_header_is_enumerated() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v35-audio-pipeline-matrix-v1"
    );
    assert_eq!(matrix["issue"], 1646);
    assert_eq!(
        matrix["proposalCases"].as_array().expect("proposal").len(),
        3
    );
    assert_eq!(matrix["qaCases"].as_array().expect("qa").len(), 3);
    assert_eq!(matrix["hookCases"].as_array().expect("hooks").len(), 2);
}

#[test]
fn v35_audio_proposal_cases() {
    for case in matrix()["proposalCases"]
        .as_array()
        .expect("proposal cases")
    {
        let fixture = case["fixture"].as_str().expect("fixture");
        let brief = AudioGenerationBrief::from_json_str(&read_text(fixture))
            .unwrap_or_else(|error| panic!("{}: parse {error}", case["id"]));
        let result = generate_audio(&brief, FIXED_NOW_MS);
        match case["expectedOutcome"].as_str().expect("outcome") {
            "proposed" => {
                let generated = result.unwrap_or_else(|error| panic!("{}: {error}", case["id"]));
                assert_eq!(generated.proposal.status, "proposed");
                assert_eq!(generated.proposal.verdict_status, "pending");
                assert!(generated.provenance.proposal_only);
            }
            "rejected" => {
                let error = result.expect_err(&format!("{} must be rejected", case["id"]));
                assert!(
                    error.to_string().contains(reject_reason(case)),
                    "{}: unexpected error {error}",
                    case["id"]
                );
            }
            other => panic!("unknown proposal outcome {other}"),
        }
    }
}

#[test]
fn v35_audio_qa_cases() {
    for case in matrix()["qaCases"].as_array().expect("qa cases") {
        let fixture = case["fixture"].as_str().expect("fixture");
        let check = AudioQaArtifact::from_json_str(&read_text(fixture))
            .unwrap_or_else(|error| panic!("{}: parse {error}", case["id"]));
        let status = check.computed_status().as_str();
        assert_eq!(
            status,
            case["expectedStatus"].as_str().expect("status"),
            "{}: unexpected audio-QA status",
            case["id"]
        );
    }
}

#[derive(Debug, Deserialize)]
struct HooksFixture {
    #[serde(rename = "hookSet")]
    hook_set: AudioHookSet,
    #[serde(rename = "signalSequence")]
    signal_sequence: Vec<HookStep>,
}

#[derive(Debug, Deserialize)]
struct HookStep {
    label: String,
    #[serde(default)]
    flags: BTreeMap<String, String>,
    #[serde(default)]
    numbers: BTreeMap<String, f64>,
    #[serde(default)]
    events: BTreeSet<String>,
    #[serde(rename = "expectedIntents")]
    expected_intents: Vec<String>,
}

#[test]
fn v35_adaptive_hook_cases() {
    let fixture: HooksFixture =
        serde_json::from_str(&read_text("hooks/hooks.json")).expect("hooks fixture parses");
    fixture.hook_set.validate().expect("hook set validates");

    // Every enumerated matrix hook case maps to a signal step with matching
    // expected intents, and the evaluator emits exactly those intents.
    for case in matrix()["hookCases"].as_array().expect("hook cases") {
        let label = case["signalLabel"].as_str().expect("signalLabel");
        let step = fixture
            .signal_sequence
            .iter()
            .find(|s| s.label == label)
            .unwrap_or_else(|| panic!("{}: no signal step {label}", case["id"]));
        let signals = AudioHookSignals {
            flags: step.flags.clone(),
            numbers: step.numbers.clone(),
            events: step.events.clone(),
        };
        let intents = fixture.hook_set.evaluate(&signals);
        let names: Vec<&str> = intents.iter().map(|i| i.intent.as_str()).collect();
        let expected: Vec<&str> = case["expectedIntents"]
            .as_array()
            .expect("expectedIntents")
            .iter()
            .map(|v| v.as_str().expect("intent"))
            .collect();
        assert_eq!(names, expected, "{}: unexpected hook intents", case["id"]);
        assert_eq!(
            step.expected_intents, expected,
            "{}: fixture and matrix disagree",
            case["id"]
        );
        // Deterministic: identical signals re-emit identically.
        assert_eq!(fixture.hook_set.evaluate(&signals), intents);
    }
}

#[test]
fn v35_backward_compatibility_audio_intent_emission_remains_valid() {
    // The existing runtime audio-intent surface (which the adaptive hooks reuse)
    // is unchanged: the behavior-runtime execution fixture still emits its audio
    // intent deterministically.
    let artifact = BehaviorArtifact::from_json_str(include_str!(
        "../../../examples/behavior-runtime-v1/valid/behavior-artifact.execution.json"
    ))
    .expect("execution fixture parses");
    let input = BehaviorExecutionInput::new("onInputAction").with_input_action("jump");
    let world = BehaviorWorldState::default()
        .with_flag("grounded", true)
        .with_position("player", 10, 10);

    let first = artifact.execute(input.clone(), world.clone());
    let replay = artifact.execute(input, world);
    assert_eq!(
        first, replay,
        "audio-intent emission replays deterministically"
    );
    assert_eq!(first.world_state.audio_intents[0].intent, "jump");
}

#[test]
fn v35_docs_and_fixtures_preserve_generated_state_wording_and_governance() {
    let docs =
        fs::read_to_string(repo_root().join("docs/scenario-coverage-v35.md")).expect("v35 docs");
    assert!(docs.contains("#1646"), "docs record the coverage issue");
    assert!(
        docs.contains("#1") && docs.contains("#23"),
        "docs record the governance anchors"
    );

    let mut corpus = docs;
    corpus.push_str(&read_text("matrix.fixture.json"));
    for forbidden in [
        "production-ready",
        "Godot replacement",
        "Godot parity",
        "is fun",
        "looks good",
        "sounds good",
    ] {
        assert!(
            !corpus.contains(forbidden),
            "forbidden wording present: {forbidden}"
        );
    }
}

//! Deterministic demo smoke test for Audio Generation and QA Demo v1 (#1645).
//!
//! Part of Audio Generation and Audio-QA v1 (#1641) under #1 Era G Milestone 37.
//! This composes the already-merged Audio Generation Proposal Model (#1642), the
//! Audio-QA Check (#1643), and the Adaptive-Audio Runtime Hooks (#1644) into one
//! end-to-end walkthrough and asserts the documented behavior: audio is generated
//! as a proposal, generation fails closed when unlicensed, audio-QA blocks an
//! invalid asset and passes a verified one, and adaptive hooks fire
//! deterministically. It asserts behavior and gate states, never subjective
//! quality, and runs with no network and no live browser.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ouroforge_core::audio_generation::{generate_audio, AudioGenerationBrief};
use ouroforge_core::audio_hooks::{AudioHookSet, AudioHookSignals};
use ouroforge_core::audio_qa::{AudioQaArtifact, AudioQaStatus};
use serde::Deserialize;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn demo_dir() -> PathBuf {
    repo_root().join("examples/audio-pipeline-v1/demo")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(demo_dir().join(name))
        .unwrap_or_else(|_| panic!("fixture exists: {name}"))
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
fn demo_generates_a_proposal_blocks_unlicensed_then_gates_qa() {
    // 1. Generate (proposal-only): the licensed brief becomes a proposal carrying
    //    license/provenance — proposed / pending / unverified, never auto-promoted.
    let brief =
        AudioGenerationBrief::from_json_str(&read("audio-brief.json")).expect("demo brief parses");
    let generated = generate_audio(&brief, FIXED_NOW_MS).expect("licensed brief generates");
    assert_eq!(generated.proposal.status, "proposed");
    assert_eq!(generated.proposal.verdict_status, "pending");
    assert_eq!(generated.proposal.confidence, "unverified");
    assert!(generated.provenance.proposal_only);
    assert_eq!(generated.provenance.license.license_id, "CC0-1.0");

    // Generation fails closed for the unlicensed brief: unlicensed audio can
    // never enter the pipeline.
    let unlicensed = AudioGenerationBrief::from_json_str(&read("audio-brief-unlicensed.json"))
        .expect("unlicensed brief parses");
    let error = generate_audio(&unlicensed, FIXED_NOW_MS)
        .expect_err("an unlicensed brief must fail closed at generation");
    assert!(
        error.to_string().contains("requires a license"),
        "unexpected error: {error}"
    );

    // 2. Audio-QA gate — blocked: the loudness-invalid artifact fails the gate and
    //    is not promotable; the declared gate verdict is `fail`.
    let blocked = AudioQaArtifact::from_json_str(&read("audio-qa-blocked.json"))
        .expect("blocked check parses");
    assert_eq!(blocked.computed_status(), AudioQaStatus::Fail);
    assert_eq!(blocked.gate_verdict()["status"], "fail");
    assert_eq!(blocked.gate_verdict()["failureCount"], 1);

    // 3. Audio-QA gate — pass — promotable: the verified artifact clears the gate.
    let pass =
        AudioQaArtifact::from_json_str(&read("audio-qa-pass.json")).expect("passing check parses");
    assert_eq!(pass.computed_status(), AudioQaStatus::Pass);
    assert_eq!(pass.gate_verdict()["status"], "pass");
    assert_eq!(pass.gate_verdict()["failureCount"], 0);
}

#[test]
fn demo_adaptive_hooks_fire_deterministically() {
    let fixture: HooksFixture =
        serde_json::from_str(&read("audio-hooks.json")).expect("hooks fixture parses");
    fixture.hook_set.validate().expect("hook set validates");

    for step in &fixture.signal_sequence {
        let signals = AudioHookSignals {
            flags: step.flags.clone(),
            numbers: step.numbers.clone(),
            events: step.events.clone(),
        };
        let intents = fixture.hook_set.evaluate(&signals);
        let names: Vec<&str> = intents.iter().map(|i| i.intent.as_str()).collect();
        assert_eq!(
            names, step.expected_intents,
            "step {} emits expected intents",
            step.label
        );
        // Deterministic: re-evaluating identical signals is identical.
        assert_eq!(
            fixture.hook_set.evaluate(&signals),
            intents,
            "step {} is deterministic",
            step.label
        );
    }
}

#[test]
fn demo_doc_records_the_walkthrough() {
    let doc = std::fs::read_to_string(repo_root().join("docs/audio-pipeline-v1-demo.md"))
        .expect("demo doc exists");
    assert!(doc.contains("#1645"), "doc records the demo issue");
    assert!(
        doc.contains("proposal-only") && doc.contains("fails closed"),
        "doc records proposal-only generation and the fail-closed gate"
    );
    assert!(
        doc.contains("#1642") && doc.contains("#1643") && doc.contains("#1644"),
        "doc records the composed follow-ups"
    );
}

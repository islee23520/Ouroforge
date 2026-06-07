//! Contract test for Adaptive-Audio Runtime Hooks v1 (#1644).
//!
//! Part of Audio Generation and Audio-QA v1 (#1641) under #1 Era G Milestone 37.
//! These tests machine-check the adaptive-audio hook evaluator: deterministic
//! audio-intent emission and snapshot/restore parity. The fixture is the same one
//! the runtime test (`examples/game-runtime/audio-hooks.test.cjs`) consumes, so
//! the trusted Rust evaluator and the deterministic runtime stay in lockstep.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ouroforge_core::audio_hooks::{AudioHookSet, AudioHookSignals, AUDIO_HOOKS_SCHEMA_VERSION};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Debug, Deserialize)]
struct Fixture {
    #[serde(rename = "hookSet")]
    hook_set: AudioHookSet,
    #[serde(rename = "signalSequence")]
    signal_sequence: Vec<Step>,
}

#[derive(Debug, Deserialize)]
struct Step {
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

impl Step {
    fn signals(&self) -> AudioHookSignals {
        AudioHookSignals {
            flags: self.flags.clone(),
            numbers: self.numbers.clone(),
            events: self.events.clone(),
        }
    }
}

fn load_fixture() -> Fixture {
    let path = repo_root().join("examples/audio-hooks-v1/audio-hooks.fixture.json");
    let text = std::fs::read_to_string(&path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn hook_set_validates() {
    let fixture = load_fixture();
    assert_eq!(fixture.hook_set.schema_version, AUDIO_HOOKS_SCHEMA_VERSION);
    fixture.hook_set.validate().expect("hook set validates");
}

#[test]
fn each_step_emits_expected_intents_deterministically() {
    let fixture = load_fixture();
    for step in &fixture.signal_sequence {
        let signals = step.signals();
        let intents = fixture.hook_set.evaluate(&signals);
        let names: Vec<&str> = intents.iter().map(|i| i.intent.as_str()).collect();
        assert_eq!(
            names, step.expected_intents,
            "step {} emits expected intents",
            step.label
        );
        // Emitted intents carry the audio-intent shape.
        for intent in &intents {
            assert!(!intent.behavior_id.is_empty());
            assert!(!intent.action_id.is_empty());
            assert!(!intent.target_entity_id.is_empty());
            assert!(!intent.intent.is_empty());
        }
        // Re-evaluating identical signals yields an identical result.
        assert_eq!(
            fixture.hook_set.evaluate(&signals),
            intents,
            "step {} is deterministic",
            step.label
        );
    }
}

#[test]
fn snapshot_restore_reproduces_identical_intents() {
    let fixture = load_fixture();
    let boss = fixture
        .signal_sequence
        .iter()
        .find(|s| s.label == "boss-low-health")
        .expect("boss step exists");

    let snapshot = boss.signals();
    let before = fixture.hook_set.evaluate(&snapshot);

    // Mutate into a calm state and confirm the emission changes.
    let mut live = snapshot.clone();
    live.flags.clear();
    live.numbers.insert("enemyCount".to_string(), 0.0);
    live.numbers.insert("playerHealth".to_string(), 100.0);
    live.events.clear();
    let mutated = fixture.hook_set.evaluate(&live);
    assert_ne!(mutated, before, "mutated signals change the emission");

    // Restore the snapshot and re-evaluate: identical to the pre-mutation result.
    let restored = snapshot.clone();
    let after = fixture.hook_set.evaluate(&restored);
    assert_eq!(
        after, before,
        "restored snapshot reproduces identical audio intents"
    );
}

#[test]
fn duplicate_hook_id_is_rejected_fail_closed() {
    let fixture = load_fixture();
    let mut set = fixture.hook_set.clone();
    let dup = set.hooks[0].clone();
    set.hooks.push(dup);
    let error = set
        .validate()
        .expect_err("a duplicate hookId must be rejected fail-closed");
    assert!(
        error.to_string().contains("declared more than once"),
        "unexpected error: {error}"
    );
}

#[test]
fn unsupported_schema_version_is_rejected() {
    let json = r#"{"schemaVersion":"audio-hooks-v0","hooks":[]}"#;
    let error = AudioHookSet::from_json_str(json)
        .expect_err("an unexpected schema version must be rejected");
    assert!(
        error.to_string().contains("schemaVersion"),
        "unexpected error: {error}"
    );
}

#[test]
fn non_integer_priority_is_rejected() {
    // The Rust model deserializes priority as i64, so a non-integer priority is
    // rejected fail-closed (parity with the JS runtime mirror's explicit check).
    let json = r#"{"schemaVersion":"audio-hooks-v1","hooks":[{"hookId":"h","priority":1.5,"when":{"kind":"always"},"emit":{"behaviorId":"a","actionId":"b","targetEntityId":"c","intent":"d"}}]}"#;
    AudioHookSet::from_json_str(json)
        .expect_err("a non-integer priority must be rejected fail-closed");
}

#[test]
fn docs_record_the_runtime_hook_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/audio-pipeline-v1.md"))
        .expect("audio pipeline doc exists");
    assert!(
        doc.contains("#1644"),
        "audio pipeline doc records the runtime-hooks follow-up (#1644)"
    );
    assert!(
        doc.contains("adaptive-audio runtime hook"),
        "doc records the adaptive-audio runtime hook contract"
    );
}

use serde_json::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read(relative)).expect(relative)
}

#[test]
fn fixture_declares_score_display_from_cascade_feedback_in_order() {
    let scene = read_json("examples/game-runtime/deckbuilder-ui-scene-v1.json");
    let score_display = &scene["deckbuilderUi"]["scoreDisplay"];
    assert_eq!(score_display["id"], "score-display-v1");
    assert_eq!(
        score_display["sourceSchemaVersion"],
        "ouroforge.score-cascade-feedback.v1"
    );
    assert_eq!(score_display["finalScore"], 24);
    assert_eq!(score_display["authoritativeScore"], 24);

    let events = score_display["events"].as_array().expect("cascade events");
    let phases = events
        .iter()
        .map(|event| event["phase"].as_str().expect("phase"))
        .collect::<Vec<_>>();
    assert_eq!(
        phases,
        [
            "base",
            "modifier",
            "modifier",
            "card-total",
            "cascade-complete"
        ]
    );
    for (index, event) in events.iter().enumerate() {
        assert_eq!(event["stepIndex"], index);
        assert_eq!(event["readOnlyEvidence"], true);
    }
    assert_eq!(events[1]["modifierId"], "plus-two");
    assert_eq!(events[2]["modifierId"], "double");
}

#[test]
fn runtime_sources_expose_score_display_formatting_tooltips_and_boundaries() {
    let module = read("examples/game-runtime/deckbuilder-ui.js");
    let runtime_test = read("examples/game-runtime/deckbuilder-ui.test.cjs");
    let combined = format!("{module}\n{runtime_test}");

    for required in [
        "normalizeScoreDisplay",
        "formatDisplayNumber",
        "scoreDisplay",
        "score-cascade-event",
        "1,234,567",
        "plus-two: (5 + 2) × 1 = 7",
        "score recomputation authority",
        "not browser score authority",
        "not a fun/quality verdict",
    ] {
        assert!(
            combined.contains(required),
            "missing score display surface: {required}"
        );
    }

    let lower = combined.to_ascii_lowercase();
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "production-ready engine",
        "godot replacement enabled",
        "quality/fun guaranteed",
        "browser score authority enabled",
    ] {
        assert!(!lower.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

#[test]
fn deckbuilder_ui_docs_record_score_display_governance() {
    let docs = read("docs/deckbuilder-ui-v1.md");
    let lower = docs.to_ascii_lowercase();
    for required in [
        "issue: #1828",
        "number-cascade and score display implementation compatibility",
        "rust/local scoring resolution",
        "score-cascade feedback evidence",
        "does not recompute authoritative score",
        "out-of-order cascade events",
        "generated runs/artifacts remain untracked unless explicitly fixture-scoped",
        "issues #1 and #23 remain open",
        "no production-ready",
    ] {
        assert!(lower.contains(required), "missing doc boundary: {required}");
    }
}

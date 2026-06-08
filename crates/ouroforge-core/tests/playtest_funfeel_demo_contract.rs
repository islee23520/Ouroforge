//! Smoke contract for Playtest and Fun-Feel Gate Demo v1 (#1860).

use std::path::{Path, PathBuf};

use ouroforge_core::funfeel_gate::{FunFeelGateInput, FunFeelReadiness};
use ouroforge_core::playtest_capture::PlaytestSessionCapture;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DemoManifest {
    #[serde(rename = "schemaVersion")]
    schema_version: String,
    #[serde(rename = "demoId")]
    demo_id: String,
    #[serde(rename = "projectId")]
    project_id: String,
    #[serde(rename = "titleConfigId")]
    title_config_id: String,
    fixtures: DemoFixtures,
    #[serde(rename = "expectedReadinessSequence")]
    expected_readiness_sequence: Vec<String>,
    determinism: String,
    boundary: String,
    #[serde(rename = "browserStudioMode")]
    browser_studio_mode: String,
    #[serde(rename = "generatedStatePolicy")]
    generated_state_policy: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DemoFixtures {
    capture: String,
    #[serde(rename = "noVerdictGate")]
    no_verdict_gate: String,
    #[serde(rename = "recordedVerdictGate")]
    recorded_verdict_gate: String,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_repo(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(repo_root().join(path)).expect("repo fixture exists")
}

fn manifest() -> DemoManifest {
    serde_json::from_str(&read_repo(
        "examples/playtest-funfeel-v1/demo/playtest-funfeel-demo-manifest-v1.json",
    ))
    .expect("demo manifest parses")
}

#[test]
fn demo_deterministically_blocks_then_unblocks_without_auto_fun_score() {
    let manifest = manifest();
    assert_eq!(
        manifest.schema_version,
        "ouroforge.playtest-funfeel-demo.v1"
    );
    assert_eq!(manifest.demo_id, "playtest-funfeel-demo-v1");
    assert_eq!(manifest.project_id, "signal-gate");
    assert_eq!(
        manifest.title_config_id,
        "deckbuilder-signal-gate-config-v1"
    );
    assert_eq!(
        manifest.expected_readiness_sequence,
        vec!["capture-valid", "needs-human-review", "approved-by-human"]
    );
    assert!(manifest.determinism.contains("no network"));
    assert!(manifest.determinism.contains("no live browser"));
    assert!(manifest
        .boundary
        .contains("does not produce an automated fun score"));
    assert!(manifest.browser_studio_mode.contains("read-only"));
    assert!(manifest.generated_state_policy.contains("fixture-scoped"));
    assert!(manifest.generated_state_policy.contains("untracked"));

    let capture = PlaytestSessionCapture::from_json_str(&read_repo(&manifest.fixtures.capture))
        .expect("capture parses");
    capture.validate().expect("capture validates");
    assert_eq!(capture.capture_id, "playtest-funfeel-demo-capture-v1");
    assert!(!capture.trusted_write_requested);
    assert!(!capture.release_authority);
    assert!(capture
        .feedback
        .notes
        .contains("not an automated fun score"));

    let no_verdict =
        FunFeelGateInput::from_json_str(&read_repo(&manifest.fixtures.no_verdict_gate))
            .expect("no-verdict gate parses");
    let blocked = no_verdict.evaluate();
    assert_eq!(blocked.readiness, FunFeelReadiness::NeedsHumanReview);
    assert!(!blocked.release_ready);
    assert!(blocked.reason.contains("missing human fun/feel verdict"));

    let recorded =
        FunFeelGateInput::from_json_str(&read_repo(&manifest.fixtures.recorded_verdict_gate))
            .expect("recorded verdict gate parses");
    let approved = recorded.evaluate();
    assert_eq!(approved.readiness, FunFeelReadiness::ApprovedByHuman);
    assert!(approved.release_ready);
    assert_eq!(
        approved.decided_by.as_deref(),
        Some("human-reviewer-demo-001")
    );

    let serialized_demo = [
        read_repo(&manifest.fixtures.capture),
        read_repo(&manifest.fixtures.no_verdict_gate),
        read_repo(&manifest.fixtures.recorded_verdict_gate),
        read_repo("examples/playtest-funfeel-v1/demo/playtest-funfeel-demo-manifest-v1.json"),
    ]
    .join("\n");
    assert!(!serialized_demo.contains("automatedFunScore"));
    assert!(!serialized_demo.contains("funScore"));
}

#[test]
fn demo_doc_records_read_only_fixture_scoped_boundaries() {
    let doc = read_repo("docs/playtest-funfeel-v1-demo.md");
    assert!(doc.contains("without network access or a live browser"));
    assert!(doc.contains("release-readiness remains blocked"));
    assert!(doc.contains("approved-by-human"));
    assert!(doc.contains("does not\nproduce an automated fun score"));
    assert!(doc.contains("read-only"));
    assert!(doc.contains("#1 and #23 remain open"));
}

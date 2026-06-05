use ouroforge_core::qa_playtest_demo::QaPlaytestDemoManifest;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-playtest-demo-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn demo_manifest_validates_and_wires_all_stages() {
    let manifest = QaPlaytestDemoManifest::from_json_str(&read_fixture("demo.manifest.json"))
        .unwrap_or_else(|error| panic!("demo.manifest.json: {error:#}"));
    let read_model = manifest.read_model();
    assert_eq!(read_model.stage_count, 13);
    assert_eq!(read_model.present_stage_count, 13);
    assert!(
        read_model.known_gap_count >= 1,
        "a demo must state known gaps"
    );
    // Generated output roots must stay under runs/.
    for root in &manifest.generated_output_roots {
        assert!(
            root.starts_with("runs/"),
            "generated output must stay under runs/: {root}"
        );
    }
    assert!(read_model
        .compatibility_notes
        .iter()
        .any(|note| note.contains("evidence and backlog inputs")));
    let json = manifest.read_model_json().unwrap();
    assert!(!json.contains("auto-merge enabled"));
}

#[test]
fn invalid_demo_manifests_fail_closed() {
    for (name, expected) in [
        (
            "invalid/demo.unbounded-fuzz.manifest.json",
            "unbounded fuzzing is not allowed",
        ),
        (
            "invalid/demo.unbounded-worker.manifest.json",
            "unbounded workers are not allowed",
        ),
        (
            "invalid/demo.overlapping-outputs.manifest.json",
            "overlapping output roots",
        ),
        (
            "invalid/demo.missing-cleanup.manifest.json",
            "cleanup policy must not be empty",
        ),
        (
            "invalid/demo.missing-known-gaps.manifest.json",
            "knownGaps must not be empty",
        ),
        (
            "invalid/demo.missing-stage.manifest.json",
            "missing stage `run-matrix`",
        ),
        (
            "invalid/demo.unsafe-boundary.manifest.json",
            "boundary must state",
        ),
    ] {
        let error = QaPlaytestDemoManifest::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn demo_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-playtest-demo-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #696"));
    assert!(docs.contains("Known gaps"));
    assert!(docs.contains("Cleanup policy"));
    assert!(docs.contains("evidence and backlog inputs"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "hidden workers enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

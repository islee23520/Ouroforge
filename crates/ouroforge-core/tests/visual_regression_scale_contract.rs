use ouroforge_core::visual_regression_scale::VisualRegressionScaleArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/visual-regression-scale-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn valid_fixtures_validate_and_roll_up_status() {
    for (name, expected) in [
        ("suite.match.fixture.json", "complete"),
        ("suite.diff.fixture.json", "complete"),
        ("suite.missing-baseline.fixture.json", "partial"),
    ] {
        let artifact = VisualRegressionScaleArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_status(), expected, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.status, expected, "{name}");
        assert_eq!(read_model.screen_count, artifact.screens.len(), "{name}");
        assert!(
            artifact.dashboard_compat.read_only,
            "{name} must be read-only"
        );
        // Reuse statement: scales the existing visual gate, no new engine.
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no new comparison engine")));
        // "looks good" stays a human decision.
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("human decision")));
    }
}

#[test]
fn baseline_match_has_no_regression() {
    let artifact =
        VisualRegressionScaleArtifact::from_json_str(&read_fixture("suite.match.fixture.json"))
            .expect("match");
    let read_model = artifact.read_model();
    assert_eq!(read_model.screen_count, 4);
    assert_eq!(read_model.matched_count, 4);
    assert_eq!(read_model.regression_count, 0);
    assert_eq!(read_model.missing_baseline_count, 0);
}

#[test]
fn planted_visual_diff_is_detected() {
    let artifact =
        VisualRegressionScaleArtifact::from_json_str(&read_fixture("suite.diff.fixture.json"))
            .expect("diff");
    let read_model = artifact.read_model();
    assert_eq!(read_model.regression_count, 1, "one planted visual diff");
    assert_eq!(read_model.regressions[0].screen_id, "hud");
    assert_eq!(read_model.regressions[0].content_variant, "variant-a");
    // Replayable evidence: the regression points back at the visual gate output.
    assert!(read_model.regressions[0]
        .visual_evidence_ref
        .contains("visual-comparison-evidence-v1"));
}

#[test]
fn missing_baseline_is_surfaced_explicitly() {
    let artifact = VisualRegressionScaleArtifact::from_json_str(&read_fixture(
        "suite.missing-baseline.fixture.json",
    ))
    .expect("missing");
    let read_model = artifact.read_model();
    assert_eq!(read_model.status, "partial");
    assert_eq!(read_model.missing_baseline_count, 1);
    assert_eq!(read_model.missing_baselines[0].screen_id, "new-shop");
    // A missing baseline is neither a match nor a regression.
    assert_eq!(read_model.matched_count, 1);
    assert_eq!(read_model.regression_count, 0);
}

#[test]
fn read_model_is_deterministic_regardless_of_screen_order() {
    let raw = read_fixture("suite.diff.fixture.json");
    let forward = VisualRegressionScaleArtifact::from_json_str(&raw).expect("forward");
    let mut reversed = forward.clone();
    reversed.screens.reverse();
    reversed.validate().expect("reversed validates");
    assert_eq!(
        forward.read_model_json().unwrap(),
        reversed.read_model_json().unwrap(),
        "read model must be deterministic regardless of screen order"
    );
}

#[test]
fn invalid_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/suite.duplicate-coordinate.fixture.json",
            "duplicate screen/variant coordinate",
        ),
        (
            "invalid/suite.missing-evidence.fixture.json",
            "is missing evidence for outcome",
        ),
        (
            "invalid/suite.missing-visual-ref.fixture.json",
            "visualEvidenceRef",
        ),
        (
            "invalid/suite.not-read-only.fixture.json",
            "must remain read-only or draft-only",
        ),
        (
            "invalid/suite.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
        (
            "invalid/suite.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/suite.forbidden-wording.fixture.json",
            "forbidden visual regression scale authority text",
        ),
    ] {
        let error = VisualRegressionScaleArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn scope_doc_mentions_visual_regression_at_scale() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/production-qa-matrix-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Visual-regression at scale"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
}

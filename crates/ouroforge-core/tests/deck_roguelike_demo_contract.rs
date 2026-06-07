//! Contract test for the Deck-Roguelike Game Class Demo v1 (#1602).
//!
//! Validates, offline and as part of `cargo test`, that the demo records a
//! seed-reproducible deck-roguelike run with passing four-gate + loop-coverage
//! evidence and a Milestone 24 ladder rung. The live seed-reproducibility proof
//! (driving the run twice through the runtime probe and comparing replay-state
//! digests) lives in the node smoke test
//! `examples/deck-roguelike-game-class-v1/demo/demo-smoke.test.cjs`; this Rust
//! contract validates the source-like inputs through the existing
//! ProjectManifest / Seed / ScenarioPack contracts and machine-checks the
//! evidence-fixture shapes, gate states, rung linkage, and guardrails.

use ouroforge_core::{ProjectManifest, ScenarioPack, Seed};
use serde_json::Value;

const DEMO_ROOT: &str = "examples/deck-roguelike-game-class-v1/demo";

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_json(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

fn assert_repo_ref(relative: &str) {
    assert!(
        workspace_path(relative).is_file(),
        "stale fixture ref: {relative}"
    );
}

fn demo_ref(suffix: &str) -> String {
    format!("{DEMO_ROOT}/{suffix}")
}

#[test]
fn demo_seed_scenario_and_project_manifest_use_existing_contracts() {
    let seed = Seed::from_path(workspace_path(&demo_ref("seeds/deck-roguelike-demo.yaml")))
        .expect("demo seed validates");
    assert_eq!(seed.id, "deck-roguelike-demo");
    assert_eq!(seed.constraints.target, "game-runtime");
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("digest-stable")));
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("Four-gate evidence")));

    let pack = ScenarioPack::from_path(workspace_path(&demo_ref(
        "scenarios/deck-roguelike-demo.json",
    )))
    .expect("demo scenario pack validates");
    assert_eq!(pack.schema_version, "scenario-pack-v1");
    assert_eq!(pack.seed, "seeds/deck-roguelike-demo.yaml");

    let manifest_path = workspace_path(&demo_ref("ouroforge.project.json"));
    let manifest =
        ProjectManifest::from_path(&manifest_path).expect("demo project manifest validates");
    let report = manifest
        .validate_references(manifest_path.parent().expect("manifest parent"))
        .expect("demo manifest refs validate");
    assert_eq!(report.project_id, "deck_roguelike_game_class_demo");
    assert_eq!(report.source_refs, 3);
    assert_eq!(report.scenario_packs, 1);
}

#[test]
fn demo_scene_declares_a_seeded_deck_roguelike_spec() {
    let scene = read_json(&demo_ref("scenes/deck-roguelike-demo.scene.json"));
    assert_eq!(scene["metadata"]["gameClass"], "deck-roguelike");
    assert_eq!(scene["seed"], 12345);
    let deck = &scene["deckRoguelike"];
    assert_eq!(deck["schemaVersion"], "ouroforge.deck-roguelike.v1");
    assert_eq!(deck["seed"], 12345);
    assert!(deck["deck"].as_array().is_some_and(|d| !d.is_empty()));
    assert!(deck["cards"].as_object().is_some_and(|c| !c.is_empty()));
    assert!(deck["enemy"]["intents"]
        .as_array()
        .is_some_and(|i| !i.is_empty()));
}

#[test]
fn demo_verdict_and_loop_coverage_keep_comparable_evidence_shape() {
    let verdict = read_json(&demo_ref(
        "fixtures/evidence/four-gate-verdict.fixture.json",
    ));
    assert_eq!(verdict["schemaVersion"], "four-gate-verdict-v1");
    assert_eq!(verdict["fixtureScoped"], true);
    assert_eq!(verdict["verdict"], "pass");
    let gates = verdict["gates"].as_array().expect("gates array");
    let gate_ids = gates
        .iter()
        .map(|gate| gate["id"].as_str().expect("gate id"))
        .collect::<Vec<_>>();
    assert_eq!(gate_ids, ["mechanical", "runtime", "visual", "semantic"]);
    for gate in gates {
        assert_eq!(gate["status"], "pass");
        for reference in gate["evidenceRefs"].as_array().expect("evidence refs") {
            assert_repo_ref(reference.as_str().expect("evidence ref"));
        }
    }

    let coverage = read_json(&demo_ref("fixtures/evidence/loop-coverage.fixture.json"));
    assert_eq!(coverage["schemaVersion"], "loop-coverage-metric-v1");
    assert_eq!(coverage["fixtureScoped"], true);
    assert_eq!(coverage["summary"]["status"], "computed");
    assert_eq!(coverage["summary"]["coverageFraction"], 1.0);
    for input in coverage["inputs"].as_array().expect("coverage inputs") {
        assert_repo_ref(input["artifactRef"].as_str().expect("artifact ref"));
        assert!(input["trustedValidationRef"].as_str().is_some());
    }
}

#[test]
fn demo_loop_run_records_the_six_stage_loop_shape() {
    let loop_run = read_json(&demo_ref(
        "fixtures/loop/deck-roguelike-loop-run.fixture.json",
    ));
    assert_eq!(loop_run["schemaVersion"], "authoring-loop-run-v1");
    assert_eq!(loop_run["fixtureScoped"], true);
    let shape = loop_run["loopShape"]
        .as_array()
        .expect("loop shape")
        .iter()
        .map(|stage| stage.as_str().expect("stage string"))
        .collect::<Vec<_>>();
    assert_eq!(
        shape,
        ["seed", "build", "observe", "verify", "journal", "evolve"]
    );
    for stage in loop_run["stages"].as_array().expect("stages") {
        assert_repo_ref(stage["artifactRef"].as_str().expect("artifact ref"));
    }
}

#[test]
fn demo_rung_record_links_the_deck_roguelike_ladder_rung() {
    let rung = read_json(&demo_ref("rung-demo.fixture.json"));
    assert_eq!(rung["schemaVersion"], "game-complexity-ladder-rung-demo-v1");
    assert_eq!(rung["demo"]["supersededTreesRecreated"], false);

    let gate = &rung["rungGate"];
    assert_eq!(gate["ladderId"], "game-complexity-ladder-v1");
    assert_eq!(gate["rungId"], "deck-roguelike");
    assert_eq!(gate["status"], "satisfied");

    assert_eq!(rung["loopCoverage"]["verdict"], "pass");
    assert_eq!(rung["engineGrowthJustification"]["status"], "none");

    // The rung claim must be backed by loop-produced evidence that actually exists.
    for evidence in rung["loopProducedEvidence"]
        .as_array()
        .expect("loop-produced evidence")
    {
        assert_eq!(evidence["producedByLoop"], true);
        assert_repo_ref(evidence["ref"].as_str().expect("evidence ref"));
    }

    // Guardrails: deterministic, fixture-scoped, no generated state, #1/#23 untouched.
    let guardrails = &rung["guardrails"];
    for (key, expected) in [
        ("deterministic", true),
        ("fixtureScoped", true),
        ("generatedState", false),
        ("network", false),
        ("liveBrowser", false),
        ("autoApply", false),
        ("autoMerge", false),
        ("modifiesIssue1", false),
        ("modifiesIssue23", false),
    ] {
        assert_eq!(guardrails[key], expected, "guardrail {key}");
    }
}

#[test]
fn demo_generalization_comparison_is_comparable() {
    let comparison = read_json(&demo_ref(
        "fixtures/generalization/comparable-pass.fixture.json",
    ));
    assert_eq!(
        comparison["schemaVersion"],
        "second-game-generalization-comparison-v1"
    );
    assert_eq!(comparison["fixtureScoped"], true);
    assert_eq!(comparison["status"], "comparable");
    let classes = comparison["classes"].as_array().expect("classes");
    assert_eq!(classes.len(), 2);
    for class in classes {
        for key in ["seedRef", "scenarioRef", "loopCoverageRef"] {
            assert_repo_ref(class[key].as_str().expect("class ref"));
        }
    }
}

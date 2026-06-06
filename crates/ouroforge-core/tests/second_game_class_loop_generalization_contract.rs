use ouroforge_core::{ProjectManifest, ScenarioPack, Seed};
use serde_json::Value;

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

fn required_gates() -> Vec<&'static str> {
    vec!["mechanical", "runtime", "visual", "semantic"]
}

fn required_stages() -> Vec<&'static str> {
    vec!["seed", "build", "observe", "verify", "journal", "evolve"]
}

fn string_array(value: &Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .expect("array exists")
        .iter()
        .map(|entry| entry.as_str().expect("string entry").to_string())
        .collect()
}

fn validate_comparison_fixture(relative: &str, expected_status: &str) -> bool {
    let comparison = read_json(relative);
    if comparison["schemaVersion"] != "second-game-generalization-comparison-v1"
        || comparison["fixtureScoped"] != true
        || comparison["status"] != expected_status
    {
        return false;
    }
    let Some(classes) = comparison["classes"].as_array() else {
        return false;
    };
    if classes.len() != 2 {
        return false;
    }
    for class in classes {
        for key in ["seedRef", "scenarioRef", "loopCoverageRef"] {
            let Some(reference) = class[key].as_str() else {
                return false;
            };
            if !workspace_path(reference).is_file() {
                return false;
            }
        }
        if string_array(class, "verdictGateIds") != required_gates() {
            return false;
        }
        if string_array(class, "loopStageIds") != required_stages() {
            return false;
        }
    }
    let Some(findings) = comparison["findings"].as_array() else {
        return false;
    };
    if findings.is_empty() {
        return false;
    }
    findings.iter().all(|finding| {
        finding["id"].as_str().is_some()
            && finding["summary"].as_str().is_some()
            && finding["evidenceRefs"]
                .as_array()
                .is_some_and(|refs| !refs.is_empty())
    })
}

#[test]
fn second_game_seed_scenario_and_project_manifest_use_existing_contracts() {
    let seed_path =
        workspace_path("examples/signal-gate-platformer/seeds/signal-gate-platformer.yaml");
    let seed = Seed::from_path(seed_path).expect("second game seed validates");
    assert_eq!(seed.id, "signal-gate-platformer");
    assert_eq!(seed.constraints.target, "game-runtime");
    assert!(seed
        .acceptance
        .iter()
        .any(|item| item.contains("Four-gate evidence")));

    let pack_path =
        workspace_path("examples/signal-gate-platformer/scenarios/signal-gate-platformer.json");
    let pack = ScenarioPack::from_path(pack_path).expect("second game scenario pack validates");
    assert_eq!(pack.schema_version, "scenario-pack-v1");
    assert_eq!(pack.seed, "seeds/signal-gate-platformer.yaml");

    let manifest_path = workspace_path("examples/signal-gate-platformer/ouroforge.project.json");
    let manifest =
        ProjectManifest::from_path(&manifest_path).expect("second game project manifest validates");
    let report = manifest
        .validate_references(manifest_path.parent().expect("manifest parent"))
        .expect("second game manifest refs validate");
    assert_eq!(report.project_id, "signal_gate_platformer_demo");
    assert_eq!(report.source_refs, 3);
    assert_eq!(report.scenario_packs, 1);
}

#[test]
fn second_game_verdict_and_loop_coverage_keep_comparable_evidence_shape() {
    let verdict = read_json(
        "examples/signal-gate-platformer/fixtures/evidence/four-gate-verdict.fixture.json",
    );
    assert_eq!(verdict["schemaVersion"], "four-gate-verdict-v1");
    assert_eq!(verdict["fixtureScoped"], true);
    let gates = verdict["gates"].as_array().expect("gates array");
    let gate_ids = gates
        .iter()
        .map(|gate| gate["id"].as_str().expect("gate id"))
        .collect::<Vec<_>>();
    assert_eq!(gate_ids, required_gates());
    for gate in gates {
        assert_eq!(gate["status"], "pass");
        for reference in gate["evidenceRefs"].as_array().expect("evidence refs") {
            assert_repo_ref(reference.as_str().expect("evidence ref"));
        }
    }

    for relative in [
        "examples/signal-gate-platformer/fixtures/evidence/loop-coverage.fixture.json",
        "examples/signal-gate-platformer/fixtures/evidence/collect-and-exit-loop-coverage.golden.json",
    ] {
        let coverage = read_json(relative);
        assert_eq!(coverage["schemaVersion"], "loop-coverage-metric-v1");
        assert_eq!(coverage["fixtureScoped"], true);
        assert_eq!(coverage["summary"]["status"], "computed");
        for input in coverage["inputs"].as_array().expect("coverage inputs") {
            assert_repo_ref(input["artifactRef"].as_str().expect("artifact ref"));
            assert!(input["trustedValidationRef"].as_str().is_some());
        }
    }
}

#[test]
fn generalization_comparison_accepts_comparable_and_gap_found_fixtures() {
    assert!(validate_comparison_fixture(
        "examples/signal-gate-platformer/fixtures/generalization/comparable-pass.fixture.json",
        "comparable"
    ));
    assert!(validate_comparison_fixture(
        "examples/signal-gate-platformer/fixtures/generalization/gap-found.fixture.json",
        "gap-found"
    ));
}

#[test]
fn generalization_comparison_rejects_missing_inputs_incomparable_shape_and_stale_refs() {
    for relative in [
        "examples/signal-gate-platformer/fixtures/generalization/invalid/missing-comparison-input.fixture.json",
        "examples/signal-gate-platformer/fixtures/generalization/invalid/incomparable-shape.fixture.json",
        "examples/signal-gate-platformer/fixtures/generalization/invalid/stale-ref.fixture.json",
    ] {
        assert!(
            !validate_comparison_fixture(relative, "comparable"),
            "{relative} should fail comparison validation"
        );
    }
}

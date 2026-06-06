use ouroforge_core::complexity_ladder::{
    evaluate_complexity_ladder, ComplexityLadder, ComplexityRungStatus, EvidenceRefState,
};

const FIXTURE: &str =
    include_str!("../../../examples/complexity-ladder-gates-v1/ladder.fixture.json");

#[test]
fn evaluates_satisfied_unsatisfied_and_insufficient_rungs_from_loop_evidence() {
    let ladder = ComplexityLadder::from_json_str(FIXTURE).expect("fixture validates");
    let evaluation = evaluate_complexity_ladder(&ladder).expect("ladder evaluates");

    assert_eq!(evaluation.rungs.len(), 3);
    assert_eq!(evaluation.rungs[0].rung_id, "arcade-platformer");
    assert_eq!(evaluation.rungs[0].status, ComplexityRungStatus::Satisfied);
    assert_eq!(evaluation.rungs[1].rung_id, "physics-puzzler");
    assert_eq!(
        evaluation.rungs[1].status,
        ComplexityRungStatus::Unsatisfied
    );
    assert!(evaluation.rungs[1]
        .reasons
        .contains(&"four-gate evidence is not passing".to_string()));
    assert_eq!(evaluation.rungs[2].rung_id, "systems-sandbox");
    assert_eq!(
        evaluation.rungs[2].status,
        ComplexityRungStatus::InsufficientEvidence
    );
    assert!(evaluation.rungs[2]
        .reasons
        .contains(&"rung evidence is missing".to_string()));
}

#[test]
fn assertion_only_claims_do_not_satisfy_a_rung() {
    let mut ladder = satisfied_ladder();
    ladder.rungs[0].capability_gate.loop_produced_demo = false;

    let evaluation = evaluate_complexity_ladder(&ladder).expect("ladder evaluates");

    assert_eq!(
        evaluation.rungs[0].status,
        ComplexityRungStatus::InsufficientEvidence
    );
    assert!(evaluation.rungs[0]
        .reasons
        .contains(&"loop-produced demo evidence is required".to_string()));
}

#[test]
fn missing_loop_coverage_is_insufficient_evidence() {
    let mut ladder = satisfied_ladder();
    ladder.rungs[0].capability_gate.loop_coverage = None;

    let evaluation = evaluate_complexity_ladder(&ladder).expect("ladder evaluates");

    assert_eq!(
        evaluation.rungs[0].status,
        ComplexityRungStatus::InsufficientEvidence
    );
    assert!(evaluation.rungs[0]
        .reasons
        .contains(&"loop-coverage verdict is required".to_string()));
}

#[test]
fn demo_ref_must_be_current_to_satisfy_a_rung() {
    let mut ladder = satisfied_ladder();
    ladder.rungs[0].capability_gate.demo_ref_state = EvidenceRefState::Missing;

    let evaluation = evaluate_complexity_ladder(&ladder).expect("ladder evaluates");

    assert_eq!(
        evaluation.rungs[0].status,
        ComplexityRungStatus::InsufficientEvidence
    );
    assert!(evaluation.rungs[0]
        .reasons
        .contains(&"demoRef evidence must be current".to_string()));
}

#[test]
fn out_of_order_satisfied_claims_are_rejected() {
    let mut ladder = satisfied_ladder();
    ladder.rungs.insert(0, missing_rung(1, "missing-first"));
    ladder.rungs[1].order = 2;

    let err = evaluate_complexity_ladder(&ladder).expect_err("out-of-order claim is rejected");

    assert!(err
        .to_string()
        .contains("out-of-order complexity rung claim"));
}

#[test]
fn stale_refs_are_rejected() {
    let mut ladder = satisfied_ladder();
    ladder.rungs[0].capability_gate.demo_ref_state = EvidenceRefState::StaleRef;

    let err = evaluate_complexity_ladder(&ladder).expect_err("stale refs are rejected");

    assert!(err.to_string().contains("stale-ref"));
}

fn satisfied_ladder() -> ComplexityLadder {
    serde_json::from_str(
        r#"{
          "schemaVersion": "complexity-ladder-gates-v1",
          "ladderId": "test-ladder",
          "rungs": [{
            "order": 1,
            "rungId": "arcade-platformer",
            "gameClass": "Arcade Platformer",
            "requiredCapabilities": ["deterministic-input", "collision"],
            "capabilityGate": {
              "claimedStatus": "satisfied",
              "loopProducedDemo": true,
              "demoRef": "evidence/demos/arcade-platformer/demo.json",
              "demoRefState": "current",
              "fourGate": {
                "verdictRef": "verdict.json",
                "verdictRefState": "current",
                "mechanical": "pass",
                "runtime": "pass",
                "visual": "pass",
                "semantic": "pass"
              },
              "loopCoverage": {
                "verdictRef": "evidence/coverage/arcade-platformer.json",
                "verdictRefState": "current",
                "status": "pass"
              }
            }
          }],
          "boundary": "Rust/local read model only; browser surfaces are read-only and no engine runtime is expanded."
        }"#,
    )
    .expect("satisfied fixture parses")
}

fn missing_rung(order: u32, rung_id: &str) -> ouroforge_core::complexity_ladder::ComplexityRung {
    ouroforge_core::complexity_ladder::ComplexityRung {
        order,
        rung_id: rung_id.to_string(),
        game_class: "Missing Evidence Class".to_string(),
        required_capabilities: vec!["capability".to_string()],
        capability_gate: Default::default(),
    }
}

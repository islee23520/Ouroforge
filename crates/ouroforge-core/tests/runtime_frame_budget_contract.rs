use ouroforge_core::runtime_frame_budget::{RuntimeFrameBudgetEvidence, RuntimeFrameBudgetStatus};

fn valid_fixture() -> &'static str {
    include_str!("../../../examples/runtime-frame-budget-v1/valid/frame-budget.sample.json")
}

#[test]
fn reports_within_budget_frame_when_debug_metrics_are_bounded() {
    let evidence =
        RuntimeFrameBudgetEvidence::from_json_str(valid_fixture()).expect("valid fixture parses");

    let status = evidence.status();
    let counts = evidence.debug_counts();

    assert_eq!(status, RuntimeFrameBudgetStatus::WithinBudget);
    assert_eq!(evidence.frame_id, "tick-40");
    assert_eq!(counts.draw_call_count, 7);
    assert_eq!(counts.layer_count, 3);
    assert_eq!(counts.collision_pair_count, 1);
    assert!(evidence.computed_violations().is_empty());
}

#[test]
fn computes_budget_violations_when_render_or_total_time_exceeds_threshold() {
    let input =
        include_str!("../../../examples/runtime-frame-budget-v1/violation/frame-budget.slow.json");
    let evidence = RuntimeFrameBudgetEvidence::from_json_str(input).expect("slow fixture parses");

    let violations = evidence
        .computed_violations()
        .into_iter()
        .map(|violation| violation.comparison_key())
        .collect::<Vec<_>>();

    assert_eq!(evidence.status(), RuntimeFrameBudgetStatus::Violated);
    assert_eq!(
        violations,
        vec![
            "renderMs:18.500>16.000".to_string(),
            "totalMs:24.250>20.000".to_string(),
        ]
    );
}

#[test]
fn rejects_malformed_frame_budget_metrics_at_the_rust_boundary() {
    let invalid_cases = [
        (
            include_str!(
                "../../../examples/runtime-frame-budget-v1/invalid/negative-render-ms.json"
            ),
            "renderMs must be non-negative",
        ),
        (
            include_str!(
                "../../../examples/runtime-frame-budget-v1/invalid/zero-total-budget.json"
            ),
            "budget totalMs must be positive",
        ),
        (
            include_str!("../../../examples/runtime-frame-budget-v1/invalid/unknown-field.json"),
            "unknown field",
        ),
        (
            include_str!("../../../examples/runtime-frame-budget-v1/invalid/empty-frame-id.json"),
            "frameId must not be empty",
        ),
    ];

    for (input, expected_error) in invalid_cases {
        let error = RuntimeFrameBudgetEvidence::from_json_str(input).expect_err(expected_error);

        assert!(format!("{error:?}").contains(expected_error), "{error:?}");
    }
}

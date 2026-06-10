use ouroforge_protocols::before_after_comparison::{
    BeforeAfterComparisonInput, BeforeAfterEvidenceRef, BeforeAfterVerdict, EvidenceBundleSummary,
    FrameStatsSummary, M126ControlledFixComparisonHandoff, ReplayResultSummary,
};
use std::collections::BTreeMap;

#[test]
fn scenario_coverage_v107_locks_m126_before_after_comparison_contract() {
    let comparison = planted_input().compare().unwrap();
    assert_eq!(comparison.verdict, BeforeAfterVerdict::Improvement);
    assert_eq!(comparison.dimensions.len(), 7);
    assert!(comparison.m127_journal_ready);
    assert!(comparison
        .dimensions
        .iter()
        .any(|dimension| !dimension.before_refs.is_empty() && !dimension.after_refs.is_empty()));

    let repeated = planted_input().compare().unwrap();
    assert_eq!(
        comparison, repeated,
        "v107 requires deterministic output for the same two bundles"
    );

    let handoff = M126ControlledFixComparisonHandoff {
        handoff_id: "scenario-v107-m126-handoff".to_string(),
        owner_issue: "2379".to_string(),
        controlled_failure_ref: reference("run-before", "m126/failure.json", "sha256:failure"),
        proposal_ref: reference("run-proposal", "m126/proposal.json", "sha256:proposal"),
        review_decision_ref: reference("run-review", "m126/review.json", "sha256:review"),
        sandbox_apply_ref: reference("run-apply", "m126/sandbox-apply.json", "sha256:apply"),
        rerun_ref: reference("run-after", "m126/rerun.json", "sha256:rerun"),
        comparison_artifact: comparison,
        scenario_coverage_suite: "scenario-coverage-v107".to_string(),
    };
    handoff.validate().unwrap();
    assert!(handoff.journal_markdown().unwrap().contains("#2379"));
}

#[test]
fn scenario_coverage_v107_rejects_inconclusive_final_handoff() {
    let mut input = planted_input();
    input.after.screenshots.clear();
    input.after.frame_stats = None;
    input.after.replay_result = None;
    input.after.flags = input.before.flags.clone();
    input.after.events = input.before.events.clone();
    input.after.console_diagnostics = input.before.console_diagnostics.clone();
    input.after.known_gaps = input.before.known_gaps.clone();
    let comparison = input.compare().unwrap();
    assert_eq!(comparison.verdict, BeforeAfterVerdict::Inconclusive);

    let handoff = M126ControlledFixComparisonHandoff {
        handoff_id: "scenario-v107-inconclusive".to_string(),
        owner_issue: "2379".to_string(),
        controlled_failure_ref: reference("run-before", "m126/failure.json", "sha256:failure"),
        proposal_ref: reference("run-proposal", "m126/proposal.json", "sha256:proposal"),
        review_decision_ref: reference("run-review", "m126/review.json", "sha256:review"),
        sandbox_apply_ref: reference("run-apply", "m126/sandbox-apply.json", "sha256:apply"),
        rerun_ref: reference("run-after", "m126/rerun.json", "sha256:rerun"),
        comparison_artifact: comparison,
        scenario_coverage_suite: "scenario-coverage-v107".to_string(),
    };
    let err = handoff.validate().unwrap_err().to_string();
    assert!(
        err.contains("must not hide an inconclusive"),
        "unexpected error: {err}"
    );
}

fn planted_input() -> BeforeAfterComparisonInput {
    BeforeAfterComparisonInput {
        comparison_id: "scenario-v107-before-after".to_string(),
        before: bundle("run-before", false, "before", 30, 3),
        after: bundle("run-after", true, "after", 16, 0),
        reviewed_change_ref: reference(
            "run-review",
            "m126/reviewed-change.json",
            "sha256:reviewed",
        ),
        guardrails: vec!["scenario coverage v107 data-only comparison".to_string()],
    }
}

fn bundle(
    run_id: &str,
    scenario_passed: bool,
    label: &str,
    max_frame_ms: u32,
    dropped_frames: u32,
) -> EvidenceBundleSummary {
    let mut flags = BTreeMap::new();
    flags.insert("scenario_passed".to_string(), scenario_passed);
    EvidenceBundleSummary {
        run_id: run_id.to_string(),
        artifacts: vec![
            reference(run_id, &format!("m126/{label}-flags.json"), "sha256:flags"),
            reference(
                run_id,
                &format!("m126/{label}-events.json"),
                "sha256:events",
            ),
            reference(
                run_id,
                &format!("m126/{label}-console.json"),
                "sha256:console",
            ),
            reference(run_id, &format!("m126/{label}-frame.json"), "sha256:frame"),
            reference(
                run_id,
                &format!("m126/{label}-replay.json"),
                "sha256:replay",
            ),
        ],
        flags,
        events: if scenario_passed {
            vec!["success.exit".to_string()]
        } else {
            vec!["error.missing-exit".to_string()]
        },
        screenshots: vec![reference(
            run_id,
            &format!("m126/screenshots/{label}.png"),
            &format!("sha256:{label}"),
        )],
        console_diagnostics: if scenario_passed {
            vec![]
        } else {
            vec!["error missing exit event".to_string()]
        },
        frame_stats: Some(FrameStatsSummary {
            avg_frame_ms: 12,
            max_frame_ms,
            dropped_frames,
        }),
        replay_result: Some(ReplayResultSummary {
            passed: scenario_passed,
            final_state_digest: format!("sha256:{label}state"),
            failures: if scenario_passed {
                vec![]
            } else {
                vec!["missing exit".to_string()]
            },
        }),
        known_gaps: if scenario_passed {
            vec![]
        } else {
            vec!["exit not reached".to_string()]
        },
    }
}

fn reference(run_id: &str, path: &str, digest: &str) -> BeforeAfterEvidenceRef {
    BeforeAfterEvidenceRef {
        run_id: run_id.to_string(),
        path: path.to_string(),
        digest: digest.to_string(),
    }
}

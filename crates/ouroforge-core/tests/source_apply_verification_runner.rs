use ouroforge_core::{
    SourceApplyVerificationCommand, SourceApplyVerificationCommandStatus,
    SourceApplyVerificationPolicy, SourceApplyVerificationRun, SourceApplyVerificationStatus,
    SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION,
};

fn policy() -> SourceApplyVerificationPolicy {
    SourceApplyVerificationPolicy {
        max_commands: 8,
        timeout_seconds: 600,
        max_output_bytes: 1_000_000,
        log_root: "runs/source-apply-verification".to_string(),
    }
}

fn command(
    argv: &[&str],
    status: SourceApplyVerificationCommandStatus,
) -> SourceApplyVerificationCommand {
    SourceApplyVerificationCommand {
        argv: argv.iter().map(|a| a.to_string()).collect(),
        allowlist_policy_id: "source-patch-preview-safe-local-checks-v1".to_string(),
        status,
        duration_seconds: 30,
        output_bytes: 2048,
        log_ref: "runs/source-apply-verification/log-1.txt".to_string(),
    }
}

fn run(commands: Vec<SourceApplyVerificationCommand>) -> SourceApplyVerificationRun {
    SourceApplyVerificationRun {
        schema_version: SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-707-1".to_string(),
        audit_ledger_ref: "evidence/audit-ledger.json".to_string(),
        policy: policy(),
        commands,
        guardrails: vec![
            "verification runner is allowlisted, bounded, and not a general command bridge"
                .to_string(),
        ],
    }
}

#[test]
fn allowlisted_passing_commands_pass() {
    let run = run(vec![
        command(
            &["cargo", "fmt", "--check"],
            SourceApplyVerificationCommandStatus::Passed,
        ),
        command(
            &["cargo", "test", "-p", "ouroforge-core"],
            SourceApplyVerificationCommandStatus::Passed,
        ),
        command(
            &[
                "cargo",
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
            SourceApplyVerificationCommandStatus::Passed,
        ),
        command(
            &[
                "node",
                "--check",
                "examples/evidence-dashboard/dashboard.js",
            ],
            SourceApplyVerificationCommandStatus::Passed,
        ),
    ]);
    run.validate().expect("valid");
    let evaluation = run.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyVerificationStatus::Passed,
        "blocked: {:?}",
        evaluation.blocked_reasons
    );
    assert!(run.is_passed());
    for forbidden in [
        "run_arbitrary_command",
        "install_dependency",
        "network_access",
    ] {
        assert!(evaluation.forbidden_actions.iter().any(|a| a == forbidden));
    }
}

#[test]
fn forbidden_network_command_blocks_run() {
    let run = run(vec![command(
        &["curl", "https://example.com/install.sh"],
        SourceApplyVerificationCommandStatus::Passed,
    )]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("forbidden")));
}

#[test]
fn install_command_blocks_run() {
    let run = run(vec![command(
        &["cargo", "install", "something"],
        SourceApplyVerificationCommandStatus::Passed,
    )]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
}

#[test]
fn shell_metacharacter_blocks_run() {
    let run = run(vec![command(
        &["cargo", "test", "&&", "rm", "-rf", "/"],
        SourceApplyVerificationCommandStatus::Passed,
    )]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
}

#[test]
fn non_allowlisted_command_blocks_run() {
    let run = run(vec![command(
        &["python", "build.py"],
        SourceApplyVerificationCommandStatus::Passed,
    )]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("not on the post-apply allowlist")));
}

#[test]
fn missing_command_blocks_run() {
    let run = run(vec![command(
        &[],
        SourceApplyVerificationCommandStatus::Skipped,
    )]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("missing")));
}

#[test]
fn timeout_blocks_run() {
    let mut cmd = command(
        &["cargo", "test"],
        SourceApplyVerificationCommandStatus::TimedOut,
    );
    cmd.duration_seconds = 1200;
    let run = run(vec![cmd]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("timeout")));
}

#[test]
fn output_limit_blocks_run() {
    let mut cmd = command(
        &["cargo", "test"],
        SourceApplyVerificationCommandStatus::Passed,
    );
    cmd.output_bytes = 5_000_000;
    let run = run(vec![cmd]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("output budget")));
}

#[test]
fn verification_failure_is_failed_not_blocked() {
    let run = run(vec![
        command(
            &["cargo", "fmt", "--check"],
            SourceApplyVerificationCommandStatus::Passed,
        ),
        command(
            &["cargo", "test"],
            SourceApplyVerificationCommandStatus::Failed,
        ),
    ]);
    let evaluation = run.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Failed);
    assert!(!run.is_passed());
}

#[test]
fn exceeding_command_budget_blocks_run() {
    let mut r = run(vec![command(
        &["cargo", "fmt", "--check"],
        SourceApplyVerificationCommandStatus::Passed,
    )]);
    r.policy.max_commands = 0;
    // max_commands 0 fails structural validation; rebuild with 1 but two commands.
    r.policy.max_commands = 1;
    r.commands.push(command(
        &["cargo", "test"],
        SourceApplyVerificationCommandStatus::Passed,
    ));
    let evaluation = r.evaluate();
    assert_eq!(evaluation.status, SourceApplyVerificationStatus::Blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("max command budget")));
}

#[test]
fn json_round_trip_preserves_run() {
    let run = run(vec![command(
        &["cargo", "fmt", "--check"],
        SourceApplyVerificationCommandStatus::Passed,
    )]);
    let json = serde_json::to_string_pretty(&run).expect("serializes");
    let parsed = SourceApplyVerificationRun::from_json_str(&json).expect("parses");
    assert_eq!(parsed, run);
}

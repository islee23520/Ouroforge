use ouroforge_core::{
    default_source_patch_test_command_allowlist, SourcePatchTestCommand,
    SourcePatchTestCommandMatchKind,
};

fn command(command: &str, argv: &[&str]) -> SourcePatchTestCommand {
    SourcePatchTestCommand {
        command: command.to_string(),
        argv: argv.iter().map(|arg| arg.to_string()).collect(),
    }
}

#[test]
fn safe_allowlist_matches_exact_cargo_format_and_clippy_commands() {
    let allowlist = default_source_patch_test_command_allowlist();

    let fmt = allowlist
        .match_command(&command("cargo fmt --check", &["cargo", "fmt", "--check"]))
        .expect("cargo fmt --check is an allowed check-only command");
    assert_eq!(fmt.kind, SourcePatchTestCommandMatchKind::Exact);
    assert_eq!(fmt.policy_id, "cargo-fmt-check");
    assert!(fmt.boundary.contains("does not execute"));

    let clippy = allowlist
        .match_command(&command(
            "cargo clippy --all-targets --all-features -- -D warnings",
            &[
                "cargo",
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ))
        .expect("cargo clippy warning gate is an allowed check-only command");
    assert_eq!(clippy.kind, SourcePatchTestCommandMatchKind::Exact);
    assert_eq!(
        clippy.policy_id,
        "cargo-clippy-all-targets-all-features-deny-warnings"
    );
}

#[test]
fn safe_allowlist_matches_focused_cargo_and_known_node_prefixes() {
    let allowlist = default_source_patch_test_command_allowlist();

    let focused_test = allowlist
        .match_command(&command(
            "cargo test -p ouroforge-core source_patch_preview_validation_passes_fixture_with_diff_and_file_class_evidence",
            &[
                "cargo",
                "test",
                "-p",
                "ouroforge-core",
                "source_patch_preview_validation_passes_fixture_with_diff_and_file_class_evidence",
            ],
        ))
        .expect("focused ouroforge-core cargo tests are allowlisted by prefix");
    assert_eq!(focused_test.kind, SourcePatchTestCommandMatchKind::Prefix);
    assert_eq!(focused_test.policy_id, "cargo-test-ouroforge-core-focused");

    let node_check = allowlist
        .match_command(&command(
            "node --check examples/evidence-dashboard/dashboard.js",
            &[
                "node",
                "--check",
                "examples/evidence-dashboard/dashboard.js",
            ],
        ))
        .expect("known node syntax checks are allowlisted by prefix");
    assert_eq!(node_check.kind, SourcePatchTestCommandMatchKind::Prefix);
    assert_eq!(node_check.policy_id, "node-check-known-examples");

    let node_test = allowlist
        .match_command(&command(
            "node examples/authoring-cockpit/cockpit.test.cjs",
            &["node", "examples/authoring-cockpit/cockpit.test.cjs"],
        ))
        .expect("known node test files are allowlisted by prefix");
    assert_eq!(node_test.kind, SourcePatchTestCommandMatchKind::Prefix);
    assert_eq!(node_test.policy_id, "node-test-known-examples");
}

#[test]
fn safe_allowlist_normalizes_display_command_without_using_shell_text_for_matching() {
    let allowlist = default_source_patch_test_command_allowlist();
    let command_with_shell_text_drift = SourcePatchTestCommand {
        command: "cargo test -p ouroforge-core ignored display text".to_string(),
        argv: vec![
            "cargo".to_string(),
            "test".to_string(),
            "-p".to_string(),
            "ouroforge-core".to_string(),
            "patch_diff_integrity_allows_valid_fixture_for_later_preview_checks".to_string(),
        ],
    };

    let matched = allowlist
        .match_command(&command_with_shell_text_drift)
        .expect("matching uses normalized argv data, not shell display text");

    assert_eq!(
        matched.normalized_argv,
        vec![
            "cargo",
            "test",
            "-p",
            "ouroforge-core",
            "patch_diff_integrity_allows_valid_fixture_for_later_preview_checks",
        ]
    );
    assert_eq!(matched.kind, SourcePatchTestCommandMatchKind::Prefix);
}

#[test]
fn safe_allowlist_keeps_commands_as_inert_policy_data() {
    let allowlist = default_source_patch_test_command_allowlist();

    assert!(allowlist.boundary.contains("does not execute"));
    assert!(allowlist
        .policies
        .iter()
        .all(|policy| policy.boundary.contains("does not execute")));
    assert!(allowlist.policies.iter().all(|policy| matches!(
        policy.match_kind,
        SourcePatchTestCommandMatchKind::Exact | SourcePatchTestCommandMatchKind::Prefix
    )));
}

#[test]
fn forbidden_command_classifier_rejects_shell_network_install_credentials_and_destructive_actions()
{
    let cases = [
        (
            command("bash -c cargo test", &["bash", "-c", "cargo test"]),
            "shell execution",
        ),
        (
            command(
                "curl https://example.invalid",
                &["curl", "https://example.invalid"],
            ),
            "network",
        ),
        (command("npm install", &["npm", "install"]), "install"),
        (
            command("cargo add anyhow", &["cargo", "add", "anyhow"]),
            "dependency",
        ),
        (
            command("gh auth token", &["gh", "auth", "token"]),
            "credential",
        ),
        (
            command("rm -rf target", &["rm", "-rf", "target"]),
            "destructive",
        ),
        (
            command("git apply patch.diff", &["git", "apply", "patch.diff"]),
            "source patch apply",
        ),
    ];

    for (candidate, expected_reason) in cases {
        let report = ouroforge_core::classify_source_patch_forbidden_test_command(&candidate)
            .expect("candidate is forbidden");
        assert!(
            report.reason.contains(expected_reason),
            "reason `{}` should contain `{expected_reason}`",
            report.reason
        );
        assert!(report.boundary.contains("no command is run"));
    }
}

#[test]
fn forbidden_classifier_runs_before_allowlist_prefix_matching() {
    let allowlist = default_source_patch_test_command_allowlist();
    let candidate = command(
        "cargo test -p ouroforge-core bad; rm -rf target",
        &[
            "cargo",
            "test",
            "-p",
            "ouroforge-core",
            "bad; rm -rf target",
        ],
    );

    assert!(ouroforge_core::classify_source_patch_forbidden_test_command(&candidate).is_some());
    assert!(
        allowlist.match_command(&candidate).is_none(),
        "forbidden shell composition must not pass a broad cargo-test prefix"
    );
}

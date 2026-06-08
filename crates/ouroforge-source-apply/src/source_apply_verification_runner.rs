//! Source Apply Allowlisted Verification Runner v1 (#707, #1 Milestone 15).
//!
//! Models the post-apply verification contract: only allowlisted local
//! verification commands (such as `cargo fmt --check`, focused `cargo test`,
//! `cargo clippy --all-targets --all-features -- -D warnings`, and known Node
//! syntax/smoke checks) may run, under explicit timeout/output budgets, with
//! generated logs kept as untracked evidence linked to the transaction and
//! audit ledger. This module classifies and validates a recorded verification
//! run; it is not a general command runner and executes nothing itself.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION: &str = "source-apply-verification-run-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyVerificationCommandStatus {
    Passed,
    Failed,
    TimedOut,
    Skipped,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyVerificationStatus {
    /// Every allowlisted command passed within budget.
    Passed,
    /// A fail-closed precondition blocks the verification (forbidden command,
    /// missing/malformed command, or output/timeout budget breach).
    Blocked,
    /// All commands were allowed and bounded, but at least one failed/timed out.
    Failed,
}

/// Bounded budgets for the verification run. Logs are generated and untracked.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyVerificationPolicy {
    #[serde(rename = "maxCommands")]
    pub max_commands: usize,
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: u64,
    #[serde(rename = "maxOutputBytes")]
    pub max_output_bytes: u64,
    /// Generated, untracked log root for captured command output.
    #[serde(rename = "logRoot")]
    pub log_root: String,
}

/// One recorded verification command result.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyVerificationCommand {
    pub argv: Vec<String>,
    #[serde(rename = "allowlistPolicyId")]
    pub allowlist_policy_id: String,
    pub status: SourceApplyVerificationCommandStatus,
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: u64,
    #[serde(rename = "outputBytes")]
    pub output_bytes: u64,
    /// Generated log reference under the policy `logRoot`.
    #[serde(rename = "logRef")]
    pub log_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyVerificationRun {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "auditLedgerRef")]
    pub audit_ledger_ref: String,
    pub policy: SourceApplyVerificationPolicy,
    pub commands: Vec<SourceApplyVerificationCommand>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyVerificationEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    pub status: SourceApplyVerificationStatus,
    #[serde(rename = "commandCount")]
    pub command_count: usize,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyVerificationRun {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply verification run JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply verification run schemaVersion must be {SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "source apply verification run applyTransactionId",
            &self.apply_transaction_id,
        )?;
        require_local_ref(
            "source apply verification run auditLedgerRef",
            &self.audit_ledger_ref,
        )?;
        self.policy.validate()?;
        for command in &self.commands {
            command.validate()?;
        }
        require_nonempty(
            "source apply verification run guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply verification run guardrails", guardrail)?;
        }
        Ok(())
    }

    /// Fail-closed classification. A run is `Blocked` if any command is not
    /// allowlisted, is malformed/missing, or breaches the output/timeout budget;
    /// `Failed` if all commands are allowed and bounded but one did not pass;
    /// `Passed` only when every allowlisted command passed within budget.
    pub fn evaluate(&self) -> SourceApplyVerificationEvaluation {
        let mut blocked = Vec::new();

        if self.commands.len() > self.policy.max_commands {
            blocked.push(format!(
                "verification run exceeds the max command budget ({} > {})",
                self.commands.len(),
                self.policy.max_commands
            ));
        }

        let mut any_failure = false;
        for command in &self.commands {
            let rendered = command.argv.join(" ");
            if command.argv.is_empty() {
                blocked.push("verification command is missing (empty argv)".to_string());
                continue;
            }
            if let Some(forbidden) = forbidden_reason(&command.argv) {
                blocked.push(format!(
                    "verification command `{rendered}` is forbidden: {forbidden}"
                ));
                continue;
            }
            if !is_allowlisted(&command.argv) {
                blocked.push(format!(
                    "verification command `{rendered}` is not on the post-apply allowlist"
                ));
                continue;
            }
            if command.output_bytes > self.policy.max_output_bytes {
                blocked.push(format!(
                    "verification command `{rendered}` exceeded the output budget"
                ));
            }
            if command.duration_seconds > self.policy.timeout_seconds
                || command.status == SourceApplyVerificationCommandStatus::TimedOut
            {
                blocked.push(format!(
                    "verification command `{rendered}` exceeded the timeout budget"
                ));
            }
            if command.status == SourceApplyVerificationCommandStatus::Failed {
                any_failure = true;
            }
        }

        let status = if !blocked.is_empty() {
            SourceApplyVerificationStatus::Blocked
        } else if any_failure {
            SourceApplyVerificationStatus::Failed
        } else {
            SourceApplyVerificationStatus::Passed
        };

        let mut evidence_summary = vec![
            format!("transaction:{}", self.apply_transaction_id),
            format!("commands:{}", self.commands.len()),
            format!("logRoot:{}", self.policy.log_root),
        ];
        if status == SourceApplyVerificationStatus::Passed {
            evidence_summary
                .push("all allowlisted verification commands passed within budget".to_string());
        }

        SourceApplyVerificationEvaluation {
            schema_version: SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION.to_string(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            status,
            command_count: self.commands.len(),
            evidence_summary,
            blocked_reasons: blocked,
            allowed_actions: vec![
                "inspect_verification_run".to_string(),
                "inspect_generated_logs".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "run_arbitrary_command".to_string(),
                "install_dependency".to_string(),
                "network_access".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply verification evaluation JSON")
    }

    pub fn is_passed(&self) -> bool {
        self.evaluate().status == SourceApplyVerificationStatus::Passed
    }
}

impl SourceApplyVerificationPolicy {
    fn validate(&self) -> Result<()> {
        if self.max_commands == 0 {
            return Err(anyhow!(
                "source apply verification policy maxCommands must be at least 1"
            ));
        }
        if self.timeout_seconds == 0 {
            return Err(anyhow!(
                "source apply verification policy timeoutSeconds must be at least 1"
            ));
        }
        if self.max_output_bytes == 0 {
            return Err(anyhow!(
                "source apply verification policy maxOutputBytes must be at least 1"
            ));
        }
        require_local_ref("source apply verification policy logRoot", &self.log_root)?;
        Ok(())
    }
}

impl SourceApplyVerificationCommand {
    fn validate(&self) -> Result<()> {
        for arg in &self.argv {
            require_text("source apply verification command argv", arg)?;
        }
        require_local_id(
            "source apply verification command allowlistPolicyId",
            &self.allowlist_policy_id,
        )?;
        require_local_ref("source apply verification command logRef", &self.log_ref)?;
        Ok(())
    }
}

/// Returns a reason if the command is explicitly forbidden (network, install,
/// credential, destructive, arbitrary shell, dependency/CI/build/release).
fn forbidden_reason(argv: &[String]) -> Option<&'static str> {
    let program = argv.first().map(String::as_str).unwrap_or("");
    match program {
        "curl" | "wget" | "ssh" | "scp" | "nc" | "telnet" => return Some("network command"),
        "npm" | "pnpm" | "yarn" | "pip" | "pip3" | "brew" | "apt" | "apt-get" | "gem" => {
            return Some("install/dependency command")
        }
        "sudo" | "su" | "chmod" | "chown" | "rm" | "mv" | "dd" | "mkfs" => {
            return Some("destructive/privilege command")
        }
        "sh" | "bash" | "zsh" | "eval" | "source" => return Some("arbitrary shell command"),
        "git"
            if argv
                .iter()
                .any(|arg| matches!(arg.as_str(), "push" | "merge" | "apply" | "am")) =>
        {
            return Some("git mutation/apply command");
        }
        "cargo"
            if argv
                .iter()
                .any(|arg| matches!(arg.as_str(), "install" | "publish" | "add" | "update")) =>
        {
            return Some("cargo install/publish/dependency command");
        }
        _ => {}
    }
    // Shell metacharacters anywhere indicate an attempt at chaining/redirect.
    for arg in argv {
        if arg.contains("&&")
            || arg.contains("||")
            || arg.contains(';')
            || arg.contains('|')
            || arg.contains('>')
            || arg.contains('`')
            || arg.contains("$(")
        {
            return Some("shell metacharacter / command chaining");
        }
        let lower = arg.to_ascii_lowercase();
        if lower.contains("http://") || lower.contains("https://") {
            return Some("network url");
        }
    }
    None
}

/// Returns true for the explicitly allowlisted post-apply verification commands.
fn is_allowlisted(argv: &[String]) -> bool {
    let program = argv.first().map(String::as_str).unwrap_or("");
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    match program {
        "cargo" => match args.get(1).copied() {
            Some("fmt") => args.contains(&"--check"),
            Some("test") => true,
            Some("clippy") => true,
            _ => false,
        },
        "node" => {
            args.contains(&"--check")
                || args
                    .iter()
                    .any(|arg| arg.ends_with(".test.cjs") || arg.ends_with(".test.js"))
        }
        _ => false,
    }
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 128
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, or dot"
        ));
    }
    Ok(())
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} must stay inside the local trusted worktree"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

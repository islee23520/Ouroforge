//! Dependency, CI, and Build-Script Mutation Blocker v1 (#709, #1 Milestone 15).
//!
//! Explicitly blocks high-risk file classes during source apply unless a
//! separate governance issue authorizes them. It classifies each candidate
//! target path, records blocker evidence for any high-risk target, and only
//! treats recognized source-like files as allowed. It fails closed: anything
//! unrecognized is blocked, not allowed. It applies nothing and runs nothing.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION: &str = "source-apply-high-risk-blocker-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyHighRiskClass {
    DependencyManifest,
    Lockfile,
    CiWorkflow,
    BuildScript,
    ShellScript,
    InstallScript,
    CredentialOrNetwork,
    ReleaseOrPublish,
    GeneratedOrLocalState,
    HiddenToolRoot,
    /// Not a recognized allowed source-like class; blocked fail-closed.
    UnsupportedClass,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyHighRiskStatus {
    Allowed,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyClassifiedTarget {
    pub path: String,
    #[serde(
        rename = "highRiskClass",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub high_risk_class: Option<SourceApplyHighRiskClass>,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyHighRiskBlocker {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "candidateTargets")]
    pub candidate_targets: Vec<String>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyHighRiskEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    pub status: SourceApplyHighRiskStatus,
    #[serde(rename = "allowedTargets")]
    pub allowed_targets: Vec<SourceApplyClassifiedTarget>,
    #[serde(rename = "blockedTargets")]
    pub blocked_targets: Vec<SourceApplyClassifiedTarget>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "governanceNote")]
    pub governance_note: String,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyHighRiskBlocker {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply high-risk blocker JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply high-risk blocker schemaVersion must be {SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "source apply high-risk blocker applyTransactionId",
            &self.apply_transaction_id,
        )?;
        if self.candidate_targets.is_empty() {
            return Err(anyhow!(
                "source apply high-risk blocker candidateTargets must not be empty"
            ));
        }
        for target in &self.candidate_targets {
            require_text("source apply high-risk blocker candidateTargets", target)?;
        }
        require_nonempty(
            "source apply high-risk blocker guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply high-risk blocker guardrails", guardrail)?;
        }
        Ok(())
    }

    /// Classify every candidate target. Apply is blocked when any candidate is
    /// high-risk or not a recognized allowed source-like class.
    pub fn evaluate(&self) -> SourceApplyHighRiskEvaluation {
        let mut allowed = Vec::new();
        let mut blocked = Vec::new();
        let mut blocked_reasons = Vec::new();

        for path in &self.candidate_targets {
            match classify_path(path) {
                None => allowed.push(SourceApplyClassifiedTarget {
                    path: path.clone(),
                    high_risk_class: None,
                    reason: "recognized allowed source-like class".to_string(),
                }),
                Some((class, reason)) => {
                    blocked_reasons.push(format!("`{path}` blocked: {reason}"));
                    blocked.push(SourceApplyClassifiedTarget {
                        path: path.clone(),
                        high_risk_class: Some(class),
                        reason: reason.to_string(),
                    });
                }
            }
        }

        let status = if blocked.is_empty() {
            SourceApplyHighRiskStatus::Allowed
        } else {
            SourceApplyHighRiskStatus::Blocked
        };

        SourceApplyHighRiskEvaluation {
            schema_version: SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION.to_string(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            status,
            allowed_targets: allowed,
            blocked_targets: blocked,
            blocked_reasons,
            governance_note:
                "relaxing these blockers requires a separate explicit governance issue".to_string(),
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "mutate_dependency_manifest".to_string(),
                "mutate_ci_workflow".to_string(),
                "mutate_build_script".to_string(),
                "mutate_release_config".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply high-risk evaluation JSON")
    }

    pub fn is_allowed(&self) -> bool {
        self.evaluate().status == SourceApplyHighRiskStatus::Allowed
    }
}

/// Returns `Some((class, reason))` if the path is high-risk or not an allowed
/// source-like file; returns `None` only for recognized allowed source files.
pub fn classify_path(path: &str) -> Option<(SourceApplyHighRiskClass, &'static str)> {
    use SourceApplyHighRiskClass::*;
    let normalized = path.replace('\\', "/");
    let lower = normalized.to_ascii_lowercase();
    let file_name = lower.rsplit('/').next().unwrap_or(&lower).to_string();

    if normalized.starts_with('/') || normalized.contains("..") {
        return Some((UnsupportedClass, "path escapes the trusted worktree"));
    }

    // Hidden tool roots / dotfiles (allow nothing under a leading dot segment).
    if lower.split('/').any(|segment| {
        segment.starts_with('.') && !segment.is_empty() && segment != "." && segment != ".."
    }) {
        return Some((HiddenToolRoot, "hidden tool root or dotfile path"));
    }

    if lower.starts_with(".github/workflows/")
        || lower.contains("/.github/workflows/")
        || matches!(
            file_name.as_str(),
            ".gitlab-ci.yml" | "azure-pipelines.yml" | ".travis.yml" | "jenkinsfile"
        )
        || lower.contains(".circleci/")
    {
        return Some((CiWorkflow, "CI workflow or pipeline config"));
    }

    if matches!(
        file_name.as_str(),
        "cargo.lock"
            | "package-lock.json"
            | "yarn.lock"
            | "pnpm-lock.yaml"
            | "gemfile.lock"
            | "poetry.lock"
            | "composer.lock"
    ) {
        return Some((Lockfile, "dependency lockfile"));
    }

    if matches!(
        file_name.as_str(),
        "cargo.toml"
            | "package.json"
            | "pyproject.toml"
            | "go.mod"
            | "gemfile"
            | "build.gradle"
            | "pom.xml"
            | "composer.json"
    ) || file_name.starts_with("requirements")
    {
        return Some((DependencyManifest, "dependency/package manifest"));
    }

    if matches!(
        file_name.as_str(),
        "build.rs" | "makefile" | "cmakelists.txt"
    ) || file_name.ends_with(".cmake")
        || file_name.ends_with(".mk")
    {
        return Some((BuildScript, "build script"));
    }

    if file_name.contains("install") && (file_name.ends_with(".sh") || file_name == "setup.py") {
        return Some((InstallScript, "install script"));
    }

    if file_name.ends_with(".sh")
        || file_name.ends_with(".bash")
        || file_name.ends_with(".zsh")
        || file_name.ends_with(".ps1")
        || file_name.ends_with(".bat")
        || file_name.ends_with(".cmd")
    {
        return Some((ShellScript, "shell script"));
    }

    if file_name.ends_with(".pem")
        || file_name.ends_with(".key")
        || file_name.ends_with(".crt")
        || file_name == ".env"
        || file_name.starts_with(".env.")
        || [
            "secret",
            "credential",
            "token",
            "aws",
            "gcloud",
            "azure",
            "kube",
        ]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        return Some((CredentialOrNetwork, "credential/auth/network/cloud file"));
    }

    if file_name == "dockerfile"
        || file_name.ends_with(".dockerfile")
        || ["release/", "publish/", "deploy/", "/dist/"]
            .iter()
            .any(|needle| lower.contains(needle))
    {
        return Some((ReleaseOrPublish, "release/export/publish file"));
    }

    if [
        "target/",
        "node_modules/",
        "/build/",
        "runs/",
        ".omc/",
        "generated/",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
        || file_name.ends_with(".log")
    {
        return Some((GeneratedOrLocalState, "generated or local state"));
    }

    // Recognized allowed source-like classes.
    const ALLOWED_EXTS: &[&str] = &[
        ".rs", ".js", ".ts", ".jsx", ".tsx", ".css", ".html", ".md", ".glsl", ".wgsl",
    ];
    if ALLOWED_EXTS.iter().any(|ext| file_name.ends_with(ext)) {
        return None;
    }

    Some((
        UnsupportedClass,
        "not a recognized allowed source-like class",
    ))
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

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

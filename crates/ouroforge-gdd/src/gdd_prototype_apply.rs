use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_PROTOTYPE_APPLY_SCHEMA_VERSION: &str = "gdd-prototype-apply-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GddPrototypeApplyStatus {
    ReadyForTrustedApply,
    MissingReview,
    Rejected,
    Blocked,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeApplyReviewStatus {
    Accepted,
    Missing,
    Rejected,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeTransactionKind {
    ProjectScaffold,
    SceneLevel,
    Behavior,
    Scenario,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeApplyArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyId")]
    pub apply_id: String,
    pub status: GddPrototypeApplyStatus,
    #[serde(rename = "gddRef")]
    pub gdd_ref: String,
    #[serde(rename = "bundleRef")]
    pub bundle_ref: String,
    #[serde(rename = "taskGraphRef")]
    pub task_graph_ref: String,
    #[serde(rename = "reviewDecision")]
    pub review_decision: GddPrototypeApplyReviewDecision,
    pub transactions: Vec<GddPrototypeTransaction>,
    #[serde(rename = "rollbackMetadata")]
    pub rollback_metadata: GddPrototypeRollbackMetadata,
    #[serde(rename = "rerunCommandContext")]
    pub rerun_command_context: GddPrototypeRerunCommandContext,
    #[serde(rename = "generatedStateAuditRefs")]
    pub generated_state_audit_refs: Vec<String>,
    #[serde(rename = "autoApply")]
    pub auto_apply: bool,
    #[serde(rename = "trustedPersistenceOwner")]
    pub trusted_persistence_owner: String,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeApplyReviewDecision {
    #[serde(rename = "decisionId")]
    pub decision_id: String,
    pub status: GddPrototypeApplyReviewStatus,
    #[serde(rename = "decisionRef")]
    pub decision_ref: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "reviewerId")]
    pub reviewer_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeTransaction {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub kind: GddPrototypeTransactionKind,
    #[serde(rename = "targetRef")]
    pub target_ref: String,
    #[serde(rename = "observedBeforeHash")]
    pub observed_before_hash: String,
    #[serde(rename = "expectedAfterHash")]
    pub expected_after_hash: String,
    #[serde(rename = "sourceNoteRefs")]
    pub source_note_refs: Vec<String>,
    #[serde(rename = "assetSourceRefs")]
    pub asset_source_refs: Vec<String>,
    #[serde(rename = "scenarioRefs")]
    pub scenario_refs: Vec<String>,
    #[serde(rename = "behaviorRefs")]
    pub behavior_refs: Vec<String>,
    #[serde(rename = "validationRefs")]
    pub validation_refs: Vec<String>,
    pub stale: bool,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeRollbackMetadata {
    #[serde(rename = "rollbackRef")]
    pub rollback_ref: String,
    #[serde(rename = "preApplyCommit")]
    pub pre_apply_commit: String,
    pub targets: Vec<GddPrototypeRollbackTarget>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeRollbackTarget {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "targetRef")]
    pub target_ref: String,
    #[serde(rename = "beforeHash")]
    pub before_hash: String,
    #[serde(rename = "afterHash")]
    pub after_hash: String,
    #[serde(rename = "restoreStrategy")]
    pub restore_strategy: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeRerunCommandContext {
    #[serde(rename = "contextRef")]
    pub context_ref: String,
    pub commands: Vec<GddPrototypeRerunCommand>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeRerunCommand {
    pub argv: Vec<String>,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeApplyReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyId")]
    pub apply_id: String,
    pub status: String,
    #[serde(rename = "trustedApplyReady")]
    pub trusted_apply_ready: bool,
    #[serde(rename = "transactionCount")]
    pub transaction_count: usize,
    #[serde(rename = "transactionKindCounts")]
    pub transaction_kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "rollbackTargetCount")]
    pub rollback_target_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddPrototypeApplyArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse GDD Prototype Apply JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddPrototypeApplyReadModel {
        let mut transaction_kind_counts = BTreeMap::new();
        for tx in &self.transactions {
            *transaction_kind_counts
                .entry(transaction_kind_label(tx.kind).to_string())
                .or_insert(0) += 1;
        }
        GddPrototypeApplyReadModel {
            schema_version: self.schema_version.clone(),
            apply_id: self.apply_id.clone(),
            status: apply_status_label(&self.status).to_string(),
            trusted_apply_ready: self.status == GddPrototypeApplyStatus::ReadyForTrustedApply
                && self.review_decision.status == GddPrototypeApplyReviewStatus::Accepted
                && !self.auto_apply,
            transaction_count: self.transactions.len(),
            transaction_kind_counts,
            rollback_target_count: self.rollback_metadata.targets.len(),
            validation_summary: vec![
                "prototype apply requires accepted independent review before trusted writes".to_string(),
                "target hashes, rollback metadata, rerun command context, source notes, scenarios, behavior refs, and generated-state audit are explicit".to_string(),
                "auto-apply, self-approval, stale targets, unsafe refs, generated-output collisions, and arbitrary source or script mutation fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "GDD, requirements, mechanics, feasibility, plans, drafts, task graph, review, apply, run evidence, and journal artifacts remain separated".to_string(),
                "composes existing scaffold, scene, behavior, scenario, asset, review, rollback, and evidence patterns without browser trusted writes".to_string(),
                "GDD-derived output remains untrusted until Rust/local validation and review-gated apply".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD prototype apply read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_PROTOTYPE_APPLY_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD prototype apply schemaVersion must be {GDD_PROTOTYPE_APPLY_SCHEMA_VERSION}"
            ));
        }
        require_local_id("GDD prototype apply applyId", &self.apply_id)?;
        for (field, value) in [
            ("GDD prototype apply gddRef", &self.gdd_ref),
            ("GDD prototype apply bundleRef", &self.bundle_ref),
            ("GDD prototype apply taskGraphRef", &self.task_graph_ref),
        ] {
            require_local_ref(field, value)?;
        }
        self.review_decision.validate()?;
        require_nonempty("GDD prototype apply transactions", self.transactions.len())?;
        if self.transactions.len() > 12 {
            return Err(anyhow!(
                "GDD prototype apply transactions are overbroad for v1"
            ));
        }
        let mut transaction_ids = BTreeSet::new();
        let mut target_refs = BTreeSet::new();
        for tx in &self.transactions {
            tx.validate()?;
            if !transaction_ids.insert(tx.transaction_id.clone()) {
                return Err(anyhow!(
                    "GDD prototype apply transactionId `{}` is duplicated",
                    tx.transaction_id
                ));
            }
            if !target_refs.insert(tx.target_ref.clone()) {
                return Err(anyhow!(
                    "GDD prototype apply generated-output collision for target `{}`",
                    tx.target_ref
                ));
            }
        }
        self.rollback_metadata
            .validate(&self.transactions, &transaction_ids)?;
        self.rerun_command_context.validate()?;
        validate_local_ref_list(
            "GDD prototype apply generatedStateAuditRefs",
            &self.generated_state_audit_refs,
            true,
        )?;
        if self.auto_apply {
            return Err(anyhow!(
                "GDD prototype apply autoApply must be false; auto-apply is forbidden"
            ));
        }
        if self.trusted_persistence_owner != "rust-local-validation" {
            return Err(anyhow!(
                "GDD prototype apply trustedPersistenceOwner must be rust-local-validation"
            ));
        }
        validate_string_list(
            "GDD prototype apply blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let has_blockers = !self.blocked_reasons.is_empty()
            || self
                .transactions
                .iter()
                .any(|tx| tx.stale || !tx.blocked_reasons.is_empty())
            || self.review_decision.status != GddPrototypeApplyReviewStatus::Accepted;
        match self.status {
            GddPrototypeApplyStatus::ReadyForTrustedApply => {
                if has_blockers {
                    return Err(anyhow!("ready_for_trusted_apply prototype apply requires accepted review, fresh targets, and no blockedReasons"));
                }
                if self.review_decision.author_id == self.review_decision.reviewer_id {
                    return Err(anyhow!("GDD prototype apply forbids self-approval"));
                }
            }
            GddPrototypeApplyStatus::MissingReview => {
                if self.review_decision.status != GddPrototypeApplyReviewStatus::Missing {
                    return Err(anyhow!(
                        "missing_review prototype apply requires missing review decision status"
                    ));
                }
            }
            GddPrototypeApplyStatus::Rejected => {
                if self.review_decision.status != GddPrototypeApplyReviewStatus::Rejected {
                    return Err(anyhow!(
                        "rejected prototype apply requires rejected review decision status"
                    ));
                }
            }
            GddPrototypeApplyStatus::Blocked | GddPrototypeApplyStatus::Stale => {
                if !has_blockers {
                    return Err(anyhow!(
                        "blocked or stale prototype apply requires visible blockers"
                    ));
                }
            }
        }
        require_text("GDD prototype apply boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "accepted review",
            "rollback metadata",
            "rerun command context",
            "rust/local validation",
            "no auto-apply",
            "no self-approval",
            "no arbitrary source mutation",
            "no arbitrary script execution",
            "no browser trusted writes",
            "no autonomous unrestricted game creation",
            "#1 remains open",
            "#23 remains open",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD prototype apply boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl GddPrototypeApplyReviewDecision {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD prototype apply decisionId", &self.decision_id)?;
        require_local_ref("GDD prototype apply decisionRef", &self.decision_ref)?;
        require_local_id("GDD prototype apply authorId", &self.author_id)?;
        require_local_id("GDD prototype apply reviewerId", &self.reviewer_id)?;
        if self.status == GddPrototypeApplyReviewStatus::Accepted
            && self.author_id == self.reviewer_id
        {
            return Err(anyhow!(
                "GDD prototype apply accepted review forbids self-approval"
            ));
        }
        Ok(())
    }
}

impl GddPrototypeTransaction {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD prototype apply transactionId", &self.transaction_id)?;
        require_local_ref("GDD prototype apply targetRef", &self.target_ref)?;
        if is_source_like_target(&self.target_ref) {
            return Err(anyhow!(
                "GDD prototype apply targetRef violates source-like fixture policy"
            ));
        }
        require_sha256(
            "GDD prototype apply observedBeforeHash",
            &self.observed_before_hash,
        )?;
        require_sha256(
            "GDD prototype apply expectedAfterHash",
            &self.expected_after_hash,
        )?;
        if self.observed_before_hash == self.expected_after_hash {
            return Err(anyhow!(
                "GDD prototype apply expectedAfterHash must differ from observedBeforeHash"
            ));
        }
        validate_local_ref_list(
            "GDD prototype apply sourceNoteRefs",
            &self.source_note_refs,
            true,
        )?;
        validate_local_ref_list(
            "GDD prototype apply assetSourceRefs",
            &self.asset_source_refs,
            true,
        )?;
        validate_local_ref_list(
            "GDD prototype apply scenarioRefs",
            &self.scenario_refs,
            false,
        )?;
        validate_local_ref_list(
            "GDD prototype apply behaviorRefs",
            &self.behavior_refs,
            false,
        )?;
        validate_local_ref_list(
            "GDD prototype apply validationRefs",
            &self.validation_refs,
            true,
        )?;
        match self.kind {
            GddPrototypeTransactionKind::Scenario if self.scenario_refs.is_empty() => {
                return Err(anyhow!(
                    "GDD prototype apply scenario transactions require scenarioRefs"
                ));
            }
            GddPrototypeTransactionKind::Behavior if self.behavior_refs.is_empty() => {
                return Err(anyhow!(
                    "GDD prototype apply behavior transactions require behaviorRefs"
                ));
            }
            _ => {}
        }
        validate_string_list(
            "GDD prototype apply transaction.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.stale && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD prototype apply stale target requires blockedReasons"
            ));
        }
        Ok(())
    }
}

impl GddPrototypeRollbackMetadata {
    fn validate(
        &self,
        transactions: &[GddPrototypeTransaction],
        transaction_ids: &BTreeSet<String>,
    ) -> Result<()> {
        require_local_ref("GDD prototype apply rollbackRef", &self.rollback_ref)?;
        require_shaish("GDD prototype apply preApplyCommit", &self.pre_apply_commit)?;
        require_nonempty("GDD prototype apply rollback targets", self.targets.len())?;
        if self.targets.len() != transactions.len() {
            return Err(anyhow!(
                "GDD prototype apply rollback metadata must cover every transaction"
            ));
        }
        let by_id: BTreeMap<_, _> = transactions
            .iter()
            .map(|tx| (tx.transaction_id.as_str(), tx))
            .collect();
        let mut covered = BTreeSet::new();
        for target in &self.targets {
            target.validate()?;
            if !transaction_ids.contains(&target.transaction_id) {
                return Err(anyhow!(
                    "GDD prototype apply rollback metadata references unknown transaction `{}`",
                    target.transaction_id
                ));
            }
            if !covered.insert(target.transaction_id.clone()) {
                return Err(anyhow!(
                    "GDD prototype apply rollback metadata duplicates transaction `{}`",
                    target.transaction_id
                ));
            }
            let tx = by_id[target.transaction_id.as_str()];
            if target.target_ref != tx.target_ref
                || target.before_hash != tx.observed_before_hash
                || target.after_hash != tx.expected_after_hash
            {
                return Err(anyhow!("GDD prototype apply rollback metadata must match transaction target and before/after hashes"));
            }
        }
        Ok(())
    }
}

impl GddPrototypeRollbackTarget {
    fn validate(&self) -> Result<()> {
        require_local_id(
            "GDD prototype apply rollback transactionId",
            &self.transaction_id,
        )?;
        require_local_ref("GDD prototype apply rollback targetRef", &self.target_ref)?;
        require_sha256("GDD prototype apply rollback beforeHash", &self.before_hash)?;
        require_sha256("GDD prototype apply rollback afterHash", &self.after_hash)?;
        require_text(
            "GDD prototype apply rollback restoreStrategy",
            &self.restore_strategy,
        )?;
        if !self
            .restore_strategy
            .to_ascii_lowercase()
            .contains("before")
            || !self
                .restore_strategy
                .to_ascii_lowercase()
                .contains("rollback")
        {
            return Err(anyhow!(
                "GDD prototype apply rollback restoreStrategy must describe before-hash rollback"
            ));
        }
        Ok(())
    }
}

impl GddPrototypeRerunCommandContext {
    fn validate(&self) -> Result<()> {
        require_local_ref("GDD prototype apply rerun contextRef", &self.context_ref)?;
        require_nonempty("GDD prototype apply rerun commands", self.commands.len())?;
        if self.commands.len() > 6 {
            return Err(anyhow!(
                "GDD prototype apply rerun commands are overbroad for v1"
            ));
        }
        for command in &self.commands {
            command.validate()?;
        }
        Ok(())
    }
}

impl GddPrototypeRerunCommand {
    fn validate(&self) -> Result<()> {
        require_nonempty("GDD prototype apply rerun argv", self.argv.len())?;
        let first = self.argv[0].as_str();
        if !matches!(first, "cargo" | "node") {
            return Err(anyhow!("GDD prototype apply rerun commands are restricted to local cargo/node verification"));
        }
        for arg in &self.argv {
            require_text("GDD prototype apply rerun argv", arg)?;
            if arg.contains(';')
                || arg.contains("&&")
                || arg.contains('|')
                || arg.contains('`')
                || arg.contains("$(")
            {
                return Err(anyhow!(
                    "GDD prototype apply rerun argv must not contain shell metacharacters"
                ));
            }
        }
        require_local_ref("GDD prototype apply rerun evidenceRef", &self.evidence_ref)?;
        Ok(())
    }
}

fn apply_status_label(status: &GddPrototypeApplyStatus) -> &'static str {
    match status {
        GddPrototypeApplyStatus::ReadyForTrustedApply => "ready_for_trusted_apply",
        GddPrototypeApplyStatus::MissingReview => "missing_review",
        GddPrototypeApplyStatus::Rejected => "rejected",
        GddPrototypeApplyStatus::Blocked => "blocked",
        GddPrototypeApplyStatus::Stale => "stale",
    }
}

fn transaction_kind_label(kind: GddPrototypeTransactionKind) -> &'static str {
    match kind {
        GddPrototypeTransactionKind::ProjectScaffold => "project-scaffold",
        GddPrototypeTransactionKind::SceneLevel => "scene-level",
        GddPrototypeTransactionKind::Behavior => "behavior",
        GddPrototypeTransactionKind::Scenario => "scenario",
    }
}

fn validate_local_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_local_ref(field, value)?;
    }
    Ok(())
}

fn validate_string_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
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

/// Reject targets that point at source-like files even when they sit under an
/// otherwise-allowed local fixture root (issue #656: no arbitrary source
/// mutation). Legitimate prototype apply targets are inert data artifacts
/// (`*.json` scene/behavior/scenario/manifest fixtures), never build manifests
/// or Rust/JS/script source files.
fn is_source_like_target(value: &str) -> bool {
    if value.starts_with("crates/") || value.starts_with("src/") || value.contains("/src/") {
        return true;
    }
    let file_name = value.rsplit('/').next().unwrap_or(value);
    const SOURCE_MANIFESTS: [&str; 7] = [
        "Cargo.toml",
        "Cargo.lock",
        "build.rs",
        "package.json",
        "package-lock.json",
        "pnpm-lock.yaml",
        "tsconfig.json",
    ];
    if SOURCE_MANIFESTS.contains(&file_name) {
        return true;
    }
    const SOURCE_EXTENSIONS: [&str; 13] = [
        ".rs", ".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".py", ".sh", ".rb", ".go", ".c",
        ".cpp",
    ];
    let lower = file_name.to_ascii_lowercase();
    SOURCE_EXTENSIONS.iter().any(|ext| lower.ends_with(ext))
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} contains forbidden traversal and must stay inside local fixture/reference roots"));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, or runs/ refs"
        ));
    }
    Ok(())
}

fn require_sha256(field: &str, value: &str) -> Result<()> {
    if !value.starts_with("sha256:")
        || value.len() != "sha256:".len() + 64
        || !value["sha256:".len()..]
            .chars()
            .all(|ch| ch.is_ascii_hexdigit())
    {
        return Err(anyhow!("{field} must be a sha256:<64 hex> hash"));
    }
    Ok(())
}

fn require_shaish(field: &str, value: &str) -> Result<()> {
    if value.len() < 7 || value.len() > 64 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a bounded git hash"));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "dynamic import",
        "command bridge",
        "local server bridge",
        "browser trusted write enabled",
        "auto-merge enabled",
        "auto-apply enabled",
        "self-approval enabled",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "commercial readiness",
        "hosted/cloud",
        "plugin runtime",
        "native export",
        "asset generation enabled",
        "autonomous unrestricted game creation enabled",
        "arbitrary source mutation enabled",
        "arbitrary script execution enabled",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!("{field} contains forbidden wording `{forbidden}`"));
        }
    }
    Ok(())
}

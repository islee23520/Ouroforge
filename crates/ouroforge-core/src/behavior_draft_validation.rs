use anyhow::{anyhow, Result};
use std::path::{Component, Path};

use crate::behavior_draft::{
    BehaviorDraftOperation, BehaviorDraftOperationKind, BehaviorDraftOperationStatus,
    BehaviorDraftValidationStatus,
};

pub(crate) fn validate_status(
    status: BehaviorDraftValidationStatus,
    operations: &[BehaviorDraftOperation],
    blocked_reasons: &[String],
) -> Result<()> {
    let has_missing = operations
        .iter()
        .any(|operation| operation.status == BehaviorDraftOperationStatus::MissingEvidence);
    let has_unsupported = operations.iter().any(|operation| {
        operation.kind == BehaviorDraftOperationKind::Unsupported
            || operation.status == BehaviorDraftOperationStatus::Unsupported
    });
    let has_blocked = operations
        .iter()
        .any(|operation| operation.status == BehaviorDraftOperationStatus::Blocked);
    match status {
        BehaviorDraftValidationStatus::Drafted
            if operations
                .iter()
                .all(|operation| operation.status == BehaviorDraftOperationStatus::Proposed)
                && !has_unsupported
                && blocked_reasons.is_empty() =>
        {
            Ok(())
        }
        BehaviorDraftValidationStatus::MissingEvidence
            if has_missing && !blocked_reasons.is_empty() =>
        {
            Ok(())
        }
        BehaviorDraftValidationStatus::Unsupported
            if has_unsupported && !blocked_reasons.is_empty() =>
        {
            Ok(())
        }
        BehaviorDraftValidationStatus::StaleTarget if !blocked_reasons.is_empty() => Ok(()),
        BehaviorDraftValidationStatus::Blocked
            if (has_blocked || has_unsupported) && !blocked_reasons.is_empty() =>
        {
            Ok(())
        }
        BehaviorDraftValidationStatus::Drafted => Err(anyhow!(
            "drafted behavior draft requires proposed operations with no blocked, missing, unsupported, or blockedReasons state"
        )),
        BehaviorDraftValidationStatus::MissingEvidence => Err(anyhow!(
            "missing_evidence behavior draft requires missing evidence operations and blockedReasons"
        )),
        BehaviorDraftValidationStatus::Unsupported => {
            Err(anyhow!("unsupported behavior draft requires blockedReasons"))
        }
        BehaviorDraftValidationStatus::StaleTarget => Err(anyhow!(
            "stale_target behavior draft requires blockedReasons describing stale target hashes"
        )),
        BehaviorDraftValidationStatus::Blocked => Err(anyhow!(
            "blocked behavior draft requires blocked operations or unsupported behavior plus blockedReasons"
        )),
    }
}

pub(crate) fn reject_forbidden_runtime_text(field: &str, value: &str) -> Result<()> {
    let normalized: String = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect();
    let forbidden = [
        "script",
        "eval",
        "dynamicimport",
        "pluginloader",
        "commandbridge",
        "trustedwrite",
        "localserverbridge",
    ];
    if forbidden.iter().any(|blocked| normalized.contains(blocked)) {
        return Err(anyhow!(
            "{field} is forbidden because #619 does not authorize scripts, eval, dynamic imports, plugin loaders, command bridges, local server bridges, or trusted browser writes"
        ));
    }
    Ok(())
}

pub(crate) fn validate_path_component(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.contains('/') || value.contains('\\') || value == "." || value == ".." {
        return Err(anyhow!("{field} must be a single path-safe component"));
    }
    Ok(())
}

pub(crate) fn require_text(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 512 {
        return Err(anyhow!("{field} must be non-empty text up to 512 bytes"));
    }
    Ok(())
}

pub(crate) fn validate_repo_relative_ref(field: &str, value: &str) -> Result<()> {
    validate_relative_artifact_path(field, value)?;
    if value.starts_with("runs/") || value.starts_with("target/") || value.starts_with(".omo/") {
        return Err(anyhow!(
            "{field} must reference fixture-scoped source, not generated state"
        ));
    }
    Ok(())
}

pub(crate) fn validate_relative_artifact_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let path = Path::new(value);
    if path.is_absolute() {
        return Err(anyhow!("{field} must be relative"));
    }
    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::ParentDir => return Err(anyhow!("{field} must not escape the repository")),
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {
                return Err(anyhow!("{field} must be a normalized relative path"));
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_snapshot_hash(field: &str, value: &str) -> Result<()> {
    let Some(digest) = value.strip_prefix("sha256:") else {
        return Err(anyhow!("{field} must use sha256:<64 hex chars>"));
    };
    if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must use sha256:<64 hex chars>"));
    }
    Ok(())
}

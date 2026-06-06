//! Provenance Replayability v1 (#1502).
//!
//! Replays are local, deterministic read-model checks over references recorded
//! by a provenance bundle. The implementation reconstructs a run directory in a
//! caller-provided workspace and reuses the existing run evaluator.

use crate::evaluate_run;
use crate::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleStatus};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const PROVENANCE_REPLAY_RESULT_SCHEMA_VERSION: &str = "provenance-replay-result-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProvenanceReplayStatus {
    Reproduced,
    Diverged,
    NotReplayable,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceReplayResult {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub status: ProvenanceReplayStatus,
    #[serde(rename = "expectedVerdict")]
    pub expected_verdict: Option<Value>,
    #[serde(rename = "actualVerdict")]
    pub actual_verdict: Option<Value>,
    pub diff: Vec<ProvenanceReplayDiff>,
    pub issues: Vec<String>,
    #[serde(rename = "replayRunDir")]
    pub replay_run_dir: Option<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceReplayDiff {
    pub path: String,
    pub expected: Value,
    pub actual: Value,
}

pub fn replay_provenance_bundle(
    bundle: &ProvenanceBundleArtifact,
    bundle_root: impl AsRef<Path>,
    replay_workspace: impl AsRef<Path>,
) -> ProvenanceReplayResult {
    let bundle_root = bundle_root.as_ref();
    let replay_workspace = replay_workspace.as_ref();
    match replay_provenance_bundle_inner(bundle, bundle_root, replay_workspace) {
        Ok(result) => result,
        Err(error) => not_replayable(bundle, vec![error.to_string()]),
    }
}

fn replay_provenance_bundle_inner(
    bundle: &ProvenanceBundleArtifact,
    bundle_root: &Path,
    replay_workspace: &Path,
) -> Result<ProvenanceReplayResult> {
    let mut issues = Vec::new();
    let bundle_evaluation = bundle.evaluate_with_root(bundle_root);
    if bundle_evaluation.computed_status != ProvenanceBundleStatus::Complete {
        issues.push(format!(
            "bundle references are not replayable: computed status {:?}",
            bundle_evaluation.computed_status
        ));
        issues.extend(bundle_evaluation.issues);
    }
    if bundle_evaluation
        .link_states
        .get("evaluator-verdict")
        .map(String::as_str)
        != Some("present")
    {
        issues.push("missing replay input: evaluator-verdict link is not present".to_string());
    }

    let replay_inputs = match &bundle.replay_inputs {
        Some(inputs) => inputs,
        None => {
            issues.push("missing replay inputs: replayInputs is absent".to_string());
            return Ok(not_replayable(bundle, issues));
        }
    };
    if !replay_inputs.deterministic_inputs && replay_inputs.deterministic_metadata_refs.is_empty() {
        issues.push(
            "non-deterministic inputs: deterministic metadata refs are required for replay"
                .to_string(),
        );
    }

    for reference in &replay_inputs.deterministic_metadata_refs {
        let path = resolve_local_ref(bundle_root, reference)?;
        if !path.is_file() {
            issues.push(format!("missing deterministic metadata ref: {reference}"));
        }
    }

    let source_run_dir = resolve_local_ref(bundle_root, &replay_inputs.run_ref)?;
    if !source_run_dir.is_dir() {
        issues.push(format!(
            "missing replay input: runRef {}",
            replay_inputs.run_ref
        ));
    }
    let expected_verdict_path =
        resolve_local_ref(bundle_root, &replay_inputs.expected_verdict_ref)?;
    if !expected_verdict_path.is_file() {
        issues.push(format!(
            "missing replay input: expectedVerdictRef {}",
            replay_inputs.expected_verdict_ref
        ));
    }

    if !issues.is_empty() {
        return Ok(not_replayable(bundle, issues));
    }

    let replay_run_dir = replay_workspace
        .join(&bundle.bundle_id)
        .join("reconstructed-run");
    if replay_run_dir.exists() {
        fs::remove_dir_all(&replay_run_dir)
            .with_context(|| format!("failed to clear {}", replay_run_dir.display()))?;
    }
    copy_dir_all(&source_run_dir, &replay_run_dir)?;

    let actual = evaluate_run(&replay_run_dir)
        .with_context(|| format!("failed to evaluate replay run {}", replay_run_dir.display()))?;
    let actual_value =
        serde_json::to_value(actual).context("failed to serialize replay verdict")?;
    let expected_value: Value = serde_json::from_str(
        &fs::read_to_string(&expected_verdict_path)
            .with_context(|| format!("failed to read {}", expected_verdict_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", expected_verdict_path.display()))?;

    let diff = diff_json("$", &expected_value, &actual_value);
    let status = if diff.is_empty() {
        ProvenanceReplayStatus::Reproduced
    } else {
        ProvenanceReplayStatus::Diverged
    };

    Ok(ProvenanceReplayResult {
        schema_version: PROVENANCE_REPLAY_RESULT_SCHEMA_VERSION.to_string(),
        bundle_id: bundle.bundle_id.clone(),
        status,
        expected_verdict: Some(expected_value),
        actual_verdict: Some(actual_value),
        diff,
        issues,
        replay_run_dir: Some(replay_run_dir.to_string_lossy().to_string()),
        boundary: replay_boundary(),
    })
}

fn not_replayable(
    bundle: &ProvenanceBundleArtifact,
    issues: Vec<String>,
) -> ProvenanceReplayResult {
    ProvenanceReplayResult {
        schema_version: PROVENANCE_REPLAY_RESULT_SCHEMA_VERSION.to_string(),
        bundle_id: bundle.bundle_id.clone(),
        status: ProvenanceReplayStatus::NotReplayable,
        expected_verdict: None,
        actual_verdict: None,
        diff: Vec::new(),
        issues,
        replay_run_dir: None,
        boundary: replay_boundary(),
    }
}

fn replay_boundary() -> String {
    "Local Rust replay only; reconstructs referenced run evidence and reuses evaluate_run without executing commands or mutating source."
        .to_string()
}

fn resolve_local_ref(root: &Path, reference: &str) -> Result<PathBuf> {
    let path = Path::new(reference);
    if path.is_absolute()
        || reference.contains('\\')
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(anyhow!(
            "reference must stay inside the local evidence root: {reference}"
        ));
    }
    Ok(root.join(path))
}

fn copy_dir_all(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create {}", destination.display()))?;
    for entry in
        fs::read_dir(source).with_context(|| format!("failed to read {}", source.display()))?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let destination_path = destination.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), &destination_path).with_context(|| {
                format!(
                    "failed to copy replay file to {}",
                    destination_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn diff_json(path: &str, expected: &Value, actual: &Value) -> Vec<ProvenanceReplayDiff> {
    if expected == actual {
        return Vec::new();
    }
    match (expected, actual) {
        (Value::Object(expected_object), Value::Object(actual_object)) => {
            let mut keys = expected_object
                .keys()
                .chain(actual_object.keys())
                .cloned()
                .collect::<Vec<_>>();
            keys.sort();
            keys.dedup();
            keys.into_iter()
                .flat_map(|key| {
                    let child_path = format!("{path}.{key}");
                    let expected_child = expected_object.get(&key).unwrap_or(&Value::Null);
                    let actual_child = actual_object.get(&key).unwrap_or(&Value::Null);
                    diff_json(&child_path, expected_child, actual_child)
                })
                .collect()
        }
        (Value::Array(expected_array), Value::Array(actual_array))
            if expected_array.len() == actual_array.len() =>
        {
            expected_array
                .iter()
                .zip(actual_array.iter())
                .enumerate()
                .flat_map(|(index, (expected_child, actual_child))| {
                    diff_json(&format!("{path}[{index}]"), expected_child, actual_child)
                })
                .collect()
        }
        _ => vec![ProvenanceReplayDiff {
            path: path.to_string(),
            expected: expected.clone(),
            actual: actual.clone(),
        }],
    }
}

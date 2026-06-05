//! Export Plan Generator v1 (#722).
//!
//! A deterministic, dry-run-only planning layer. It converts a validated
//! [`ExportProfile`] into an [`ExportPlan`] that separates source inputs,
//! generated outputs, blocked-file policy, verification steps, and the expected
//! artifact manifest entries. Planning performs no filesystem writes and runs
//! no commands; it only produces an in-memory plan (and a JSON rendering) so a
//! later bundle-assembly step can depend on a validated plan rather than
//! rediscovering files ad hoc.
//!
//! Planning is fail closed: it re-validates the profile, requires an allowed v1
//! target, and rejects duplicate or colliding inputs before any plan is built.

use crate::export_profile::{ExportProfile, RuntimeProbeMode};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const EXPORT_PLAN_SCHEMA_VERSION: &str = "export-plan-v1";

/// Source-input categories the plan tracks.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PlannedInputKind {
    EntryScene,
    AssetRoot,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlannedInput {
    pub kind: PlannedInputKind,
    pub path: String,
}

/// Logical role of a generated output. Roles, not concrete bytes: the bundle is
/// assembled by a later milestone slice.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PlannedOutputKind {
    HtmlEntry,
    RuntimeBootstrap,
    AssetPayload,
    AssetManifest,
    Checksums,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlannedOutput {
    pub kind: PlannedOutputKind,
    /// Package-relative path (relative to the profile output directory).
    pub package_path: String,
    /// Full repository-relative staged path (output directory joined).
    pub staged_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExpectedArtifact {
    pub package_path: String,
    pub kind: PlannedOutputKind,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VerificationStep {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExportPlan {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "exportTarget")]
    pub export_target: String,
    #[serde(rename = "outputDir")]
    pub output_dir: String,
    #[serde(rename = "sourceInputs")]
    pub source_inputs: Vec<PlannedInput>,
    #[serde(rename = "generatedOutputs")]
    pub generated_outputs: Vec<PlannedOutput>,
    /// Path-prefix policy: any source path under one of these prefixes is
    /// refused by bundle assembly. Documents the fail-closed blocklist so a
    /// later assembly step cannot silently include generated or sensitive state.
    #[serde(rename = "blockedFiles")]
    pub blocked_files: Vec<String>,
    #[serde(rename = "verificationSteps")]
    pub verification_steps: Vec<VerificationStep>,
    #[serde(rename = "expectedArtifacts")]
    pub expected_artifacts: Vec<ExpectedArtifact>,
}

/// Deterministic blocklist of source path prefixes that bundle assembly must
/// refuse. Fixed policy data — no environment or timestamp dependence.
const BLOCKED_SOURCE_PREFIXES: &[&str] = &[
    ".git/",
    ".github/",
    "node_modules/",
    "target/",
    "dist/",
    "build/",
    "runs/",
    ".omo/",
    ".env",
    "secrets/",
];

impl ExportPlan {
    /// Parse a profile from JSON and build its plan. Convenience entry point for
    /// callers and fail-closed tests.
    pub fn from_profile_json(input: &str) -> Result<Self> {
        let profile = ExportProfile::from_json_str(input)
            .context("export plan requires a valid export profile")?;
        Self::from_profile(&profile)
    }

    /// Build a deterministic dry-run plan from a validated profile.
    pub fn from_profile(profile: &ExportProfile) -> Result<Self> {
        // Defense in depth: re-validate even though `from_json_str` already did.
        profile.validate()?;

        if !profile.target_is_allowed() {
            return Err(anyhow!(
                "export plan refuses target `{}`: only allowed v1 targets can be planned",
                profile.export_target
            ));
        }

        // Reject duplicate asset roots: ambiguous, non-deterministic outputs.
        let mut seen_roots = BTreeSet::new();
        for root in &profile.asset_roots {
            if !seen_roots.insert(root.as_str()) {
                return Err(anyhow!("export plan refuses duplicate asset root `{root}`"));
            }
        }

        let output_dir = profile.output_dir.trim_end_matches('/').to_string();

        // Source inputs: entry scene first, then asset roots in declared order.
        let mut source_inputs = vec![PlannedInput {
            kind: PlannedInputKind::EntryScene,
            path: profile.entry_scene.clone(),
        }];
        for root in &profile.asset_roots {
            source_inputs.push(PlannedInput {
                kind: PlannedInputKind::AssetRoot,
                path: root.clone(),
            });
        }

        // Asset payload output dirs derive from each asset root's last segment.
        // Colliding segments would overwrite one another: refuse.
        let mut payload_outputs = Vec::new();
        let mut seen_segments = BTreeSet::new();
        for root in &profile.asset_roots {
            let segment = last_segment(root);
            if !seen_segments.insert(segment.to_string()) {
                return Err(anyhow!(
                    "export plan refuses colliding asset output segment `{segment}` from root `{root}`"
                ));
            }
            let package_path = format!("assets/{segment}");
            payload_outputs.push(PlannedOutput {
                kind: PlannedOutputKind::AssetPayload,
                staged_path: join_output(&output_dir, &package_path),
                package_path,
            });
        }

        let mut generated_outputs = vec![
            output(&output_dir, PlannedOutputKind::HtmlEntry, "index.html"),
            output(
                &output_dir,
                PlannedOutputKind::RuntimeBootstrap,
                "runtime/bootstrap.js",
            ),
        ];
        generated_outputs.extend(payload_outputs);
        generated_outputs.push(output(
            &output_dir,
            PlannedOutputKind::AssetManifest,
            "export-manifest.json",
        ));
        generated_outputs.push(output(
            &output_dir,
            PlannedOutputKind::Checksums,
            "export-checksums.txt",
        ));

        let expected_artifacts = generated_outputs
            .iter()
            .map(|o| ExpectedArtifact {
                package_path: o.package_path.clone(),
                kind: o.kind,
            })
            .collect();

        // Verification steps: standard load/probe/manifest checks, then one
        // scenario smoke per declared scenario id. Deterministic order.
        let mut verification_steps = vec![
            VerificationStep {
                id: "load-without-console-errors".to_string(),
                description: "Exported bundle loads with no console or runtime errors".to_string(),
            },
            VerificationStep {
                id: probe_step_id(profile.runtime_probe_mode),
                description: probe_step_description(profile.runtime_probe_mode),
            },
            VerificationStep {
                id: "asset-manifest-integrity".to_string(),
                description: "Every manifest entry resolves to a staged artifact".to_string(),
            },
        ];
        for scenario in &profile.verification_scenario_ids {
            verification_steps.push(VerificationStep {
                id: format!("scenario-smoke:{scenario}"),
                description: format!("Run scenario `{scenario}` as a load-time smoke check"),
            });
        }

        Ok(ExportPlan {
            schema_version: EXPORT_PLAN_SCHEMA_VERSION.to_string(),
            plan_id: format!("plan_{}", profile.profile_id),
            profile_id: profile.profile_id.clone(),
            project_id: profile.project_id.clone(),
            export_target: profile.export_target.clone(),
            output_dir,
            source_inputs,
            generated_outputs,
            blocked_files: BLOCKED_SOURCE_PREFIXES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            verification_steps,
            expected_artifacts,
        })
    }

    /// Render the plan as deterministic pretty JSON for dry-run inspection.
    pub fn to_dry_run_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize export plan JSON")
    }
}

fn output(output_dir: &str, kind: PlannedOutputKind, package_path: &str) -> PlannedOutput {
    PlannedOutput {
        kind,
        staged_path: join_output(output_dir, package_path),
        package_path: package_path.to_string(),
    }
}

fn join_output(output_dir: &str, package_path: &str) -> String {
    if output_dir.is_empty() {
        package_path.to_string()
    } else {
        format!("{output_dir}/{package_path}")
    }
}

fn last_segment(path: &str) -> &str {
    path.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(path)
}

fn probe_step_id(mode: RuntimeProbeMode) -> String {
    match mode {
        RuntimeProbeMode::Preserve => "runtime-probe-compatibility".to_string(),
        RuntimeProbeMode::Report => "runtime-probe-compatibility-report".to_string(),
    }
}

fn probe_step_description(mode: RuntimeProbeMode) -> String {
    match mode {
        RuntimeProbeMode::Preserve => {
            "Exported runtime preserves probe hooks for evidence-native QA".to_string()
        }
        RuntimeProbeMode::Report => {
            "Exported runtime preserves probe hooks and emits a probe-compatibility report"
                .to_string()
        }
    }
}

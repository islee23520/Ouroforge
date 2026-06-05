//! Export Verification Runner v1 (#728).
//!
//! Runs a bounded set of local, static verification checks over an assembled
//! export bundle and produces a structured pass/fail report. It fails closed on
//! a missing bundle, a blocked target, a missing probe, an unloadable entry
//! scene, or an unsafe runtime surface.
//!
//! Command safety: this runner executes no commands. It performs static checks
//! only. The verification command allowlist below is inert policy data
//! describing the local commands a future harness may run — there is no
//! arbitrary command runner, browser command bridge, or network/install command.

use crate::export_plan::ExportPlan;
use crate::export_probe_check::{check_bundle_probe, ExportProbeMode};
use crate::export_profile::ALLOWED_EXPORT_TARGETS;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const EXPORT_VERIFICATION_SCHEMA_VERSION: &str = "export-verification-v1";

/// Inert allowlist of local verification commands a future harness may run.
/// This module never executes them.
pub const EXPORT_VERIFICATION_ALLOWED_COMMANDS: &[&[&str]] = &[&["node", "--check"]];

/// Boundary statement: the runner does not execute commands.
pub const EXPORT_VERIFICATION_BOUNDARY: &str =
    "Static local verification only; does not execute commands, start servers, open browsers, run \
     network/install commands, or perform any publish/deploy/sign/upload operation.";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CheckStatus {
    Pass,
    Fail,
    Skipped,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VerificationCheck {
    pub id: String,
    pub status: CheckStatus,
    pub detail: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExportVerificationReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub verdict: CheckStatus,
    pub checks: Vec<VerificationCheck>,
}

impl ExportVerificationReport {
    pub fn passed(&self) -> bool {
        self.verdict == CheckStatus::Pass
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize verification report")
    }
}

/// True if `argv` matches an allowlisted local verification command (prefix
/// match). Pure policy check; executes nothing.
pub fn command_is_allowlisted(argv: &[&str]) -> bool {
    EXPORT_VERIFICATION_ALLOWED_COMMANDS
        .iter()
        .any(|allowed| argv.len() >= allowed.len() && &argv[..allowed.len()] == *allowed)
}

/// Run the static verification checks against an assembled bundle.
pub fn verify_export_bundle(
    plan: &ExportPlan,
    bundle_root: &Path,
    probe_mode: ExportProbeMode,
) -> ExportVerificationReport {
    let mut checks = Vec::new();

    // 1. Target must be an allowed v1 target (fail closed on blocked targets).
    let target_ok = ALLOWED_EXPORT_TARGETS.contains(&plan.export_target.as_str());
    checks.push(check(
        "target-allowed",
        target_ok,
        if target_ok {
            format!("target `{}` is allowed", plan.export_target)
        } else {
            format!(
                "target `{}` is not an allowed v1 target",
                plan.export_target
            )
        },
    ));

    // 2. Required bundle artifacts present (load surface).
    let index = bundle_root.join("index.html");
    let bootstrap = bundle_root.join("runtime/bootstrap.js");
    let bundle_present = index.is_file() && bootstrap.is_file();
    checks.push(check(
        "bundle-present",
        bundle_present,
        if bundle_present {
            "index.html and runtime/bootstrap.js are present".to_string()
        } else {
            "exported bundle is missing index.html or runtime/bootstrap.js".to_string()
        },
    ));

    // 3. Entry scene loads (parses as JSON).
    match load_entry_scene(bundle_root) {
        Ok(Some(scene_path)) => checks.push(check(
            "scene-loads",
            true,
            format!("entry scene `{scene_path}` parses as JSON"),
        )),
        Ok(None) => checks.push(check(
            "scene-loads",
            false,
            "no entry scene found under scene/".to_string(),
        )),
        Err(err) => checks.push(check("scene-loads", false, err.to_string())),
    }

    // 4. Runtime probe compatibility (fail closed on missing probe).
    match check_bundle_probe(bundle_root, probe_mode) {
        Ok(report) if report.passed => checks.push(check(
            "runtime-probe-compatibility",
            true,
            "exported runtime preserves the required probe surface".to_string(),
        )),
        Ok(report) => checks.push(check(
            "runtime-probe-compatibility",
            false,
            format!(
                "probe surface incomplete: missing {:?}",
                report.missing_methods
            ),
        )),
        Err(err) => checks.push(check("runtime-probe-compatibility", false, err.to_string())),
    }

    // 5. No unsafe runtime surface (no network/eval/command bridges leaked).
    checks.push(no_unsafe_runtime_surface(&bootstrap));

    // 6. Scenario smoke: one declared smoke per plan scenario-smoke step.
    let scenario_steps: Vec<&str> = plan
        .verification_steps
        .iter()
        .filter(|s| s.id.starts_with("scenario-smoke:"))
        .map(|s| s.id.as_str())
        .collect();
    if scenario_steps.is_empty() {
        checks.push(VerificationCheck {
            id: "scenario-smoke".to_string(),
            status: CheckStatus::Skipped,
            detail: "no verification scenarios declared".to_string(),
        });
    } else {
        let declared = scenario_steps.join(", ");
        checks.push(check(
            "scenario-smoke",
            bundle_present,
            format!("declared scenario smokes: {declared}"),
        ));
    }

    let verdict = if checks.iter().any(|c| c.status == CheckStatus::Fail) {
        CheckStatus::Fail
    } else {
        CheckStatus::Pass
    };
    ExportVerificationReport {
        schema_version: EXPORT_VERIFICATION_SCHEMA_VERSION.to_string(),
        verdict,
        checks,
    }
}

/// Fail closed unless the bundle verifies.
pub fn ensure_export_verified(
    plan: &ExportPlan,
    bundle_root: &Path,
    probe_mode: ExportProbeMode,
) -> Result<ExportVerificationReport> {
    let report = verify_export_bundle(plan, bundle_root, probe_mode);
    if report.passed() {
        return Ok(report);
    }
    let failures: Vec<&str> = report
        .checks
        .iter()
        .filter(|c| c.status == CheckStatus::Fail)
        .map(|c| c.id.as_str())
        .collect();
    Err(anyhow!(
        "export verification failed: {}",
        failures.join(", ")
    ))
}

fn check(id: &str, ok: bool, detail: String) -> VerificationCheck {
    VerificationCheck {
        id: id.to_string(),
        status: if ok {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        },
        detail,
    }
}

fn load_entry_scene(bundle_root: &Path) -> Result<Option<String>> {
    let scene_dir = bundle_root.join("scene");
    if !scene_dir.is_dir() {
        return Ok(None);
    }
    let mut found: Option<String> = None;
    let mut entries: Vec<_> = std::fs::read_dir(&scene_dir)
        .with_context(|| format!("failed to read {}", scene_dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let bytes = std::fs::read(&path)
                .with_context(|| format!("failed to read scene {}", path.display()))?;
            serde_json::from_slice::<serde_json::Value>(&bytes)
                .with_context(|| format!("entry scene {} is not valid JSON", path.display()))?;
            found = Some(format!("scene/{}", file_name(&path)));
            break;
        }
    }
    Ok(found)
}

fn no_unsafe_runtime_surface(bootstrap: &Path) -> VerificationCheck {
    let source = match std::fs::read_to_string(bootstrap) {
        Ok(s) => s,
        Err(err) => {
            return VerificationCheck {
                id: "no-unsafe-runtime-surface".to_string(),
                status: CheckStatus::Fail,
                detail: format!("cannot read runtime bootstrap: {err}"),
            }
        }
    };
    let lower = source.to_lowercase();
    for forbidden in [
        "xmlhttprequest",
        "websocket",
        "eval(",
        "child_process",
        "import(",
    ] {
        if lower.contains(forbidden) {
            return VerificationCheck {
                id: "no-unsafe-runtime-surface".to_string(),
                status: CheckStatus::Fail,
                detail: format!("runtime bootstrap exposes forbidden surface `{forbidden}`"),
            };
        }
    }
    VerificationCheck {
        id: "no-unsafe-runtime-surface".to_string(),
        status: CheckStatus::Pass,
        detail: "no network/eval/command surface in runtime bootstrap".to_string(),
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default()
}

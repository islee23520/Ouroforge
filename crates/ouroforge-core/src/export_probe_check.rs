//! Runtime Probe Preservation for Export v1 (#725).
//!
//! After a bundle is assembled (#723), the exported runtime must still expose
//! the `window.__OUROFORGE__` probe so evidence-native QA keeps working. This
//! module checks an exported bundle's runtime bootstrap for the required probe
//! surface and fails closed when hooks are missing.
//!
//! Two probe modes are supported:
//! - `dev-probe-enabled`: the full probe surface, including stepping and input.
//! - `packaged-probe-limited`: a read-only inspection subset.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// The global the exported runtime must define.
pub const PROBE_GLOBAL: &str = "__OUROFORGE__";

/// Read-only probe methods required in every mode.
const REQUIRED_LIMITED: &[&str] = &["getWorldState", "getFrameStats", "getEvents", "snapshot"];

/// Additional interactive methods required for the full dev probe.
const REQUIRED_DEV_EXTRA: &[&str] = &["step", "pause", "resume", "setInput", "restore"];

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ExportProbeMode {
    DevProbeEnabled,
    PackagedProbeLimited,
}

impl ExportProbeMode {
    pub fn required_methods(self) -> Vec<&'static str> {
        let mut methods: Vec<&'static str> = REQUIRED_LIMITED.to_vec();
        if self == ExportProbeMode::DevProbeEnabled {
            methods.extend_from_slice(REQUIRED_DEV_EXTRA);
        }
        methods
    }
}

/// Result of checking an exported bundle's probe surface. Serializable so it can
/// be recorded as export evidence.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProbeCheckReport {
    pub mode: ExportProbeMode,
    #[serde(rename = "globalPresent")]
    pub global_present: bool,
    #[serde(rename = "presentMethods")]
    pub present_methods: Vec<String>,
    #[serde(rename = "missingMethods")]
    pub missing_methods: Vec<String>,
    pub passed: bool,
}

impl ProbeCheckReport {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize probe check report")
    }
}

/// Check a runtime bootstrap source for the probe surface required by `mode`.
pub fn check_probe_source(bootstrap_js: &str, mode: ExportProbeMode) -> ProbeCheckReport {
    let global_present = bootstrap_js.contains(PROBE_GLOBAL);
    let mut present_methods = Vec::new();
    let mut missing_methods = Vec::new();
    for method in mode.required_methods() {
        if method_is_exposed(bootstrap_js, method) {
            present_methods.push(method.to_string());
        } else {
            missing_methods.push(method.to_string());
        }
    }
    let passed = global_present && missing_methods.is_empty();
    ProbeCheckReport {
        mode,
        global_present,
        present_methods,
        missing_methods,
        passed,
    }
}

/// Check an assembled bundle's `runtime/bootstrap.js`.
pub fn check_bundle_probe(bundle_root: &Path, mode: ExportProbeMode) -> Result<ProbeCheckReport> {
    let bootstrap = bundle_root.join("runtime/bootstrap.js");
    let source = fs::read_to_string(&bootstrap).with_context(|| {
        format!(
            "exported bundle is missing runtime bootstrap {}",
            bootstrap.display()
        )
    })?;
    Ok(check_probe_source(&source, mode))
}

/// Fail closed unless the exported bundle preserves the required probe surface.
pub fn ensure_bundle_probe_compatible(bundle_root: &Path, mode: ExportProbeMode) -> Result<()> {
    let report = check_bundle_probe(bundle_root, mode)?;
    if report.passed {
        return Ok(());
    }
    if !report.global_present {
        return Err(anyhow!(
            "exported runtime does not expose the {PROBE_GLOBAL} probe global"
        ));
    }
    Err(anyhow!(
        "exported runtime is missing required probe hooks: {}",
        report.missing_methods.join(", ")
    ))
}

/// A method is exposed if its name appears as an object-literal member: a
/// shorthand method `name(`, a property `name:`, or a value shorthand `name,`.
fn method_is_exposed(source: &str, method: &str) -> bool {
    source.contains(&format!("{method}("))
        || source.contains(&format!("{method}:"))
        || source.contains(&format!("{method},"))
}

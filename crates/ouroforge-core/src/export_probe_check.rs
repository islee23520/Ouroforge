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
    // Constrain detection to the object literal actually installed as the probe
    // global. A bare `bootstrap_js.contains(PROBE_GLOBAL)` or whole-file method
    // substring search would accept a stripped export that merely mentions
    // `__OUROFORGE__` or `getEvents` in a comment, string, or unrelated object
    // while the real probe global lacks the hook (#725).
    let probe_object = probe_object_literal(bootstrap_js);
    let global_present = probe_object.is_some();
    let scope = probe_object.unwrap_or("");
    let mut present_methods = Vec::new();
    let mut missing_methods = Vec::new();
    for method in mode.required_methods() {
        if method_is_exposed(scope, method) {
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

/// A method is exposed if its name appears as an object-literal member within
/// the resolved probe object: a shorthand method `name(`, a property `name:`,
/// or a value shorthand `name,`.
fn method_is_exposed(source: &str, method: &str) -> bool {
    source.contains(&format!("{method}("))
        || source.contains(&format!("{method}:"))
        || source.contains(&format!("{method},"))
}

/// Locate the source of the object literal installed as the runtime probe
/// global, resolving one level of `const NAME = ...` and `Object.freeze(...)`
/// indirection. Returns `None` when the global is never assigned or its value
/// cannot be resolved to an object literal, so probe detection can no longer be
/// satisfied by a stray `__OUROFORGE__`/method mention in a comment, string, or
/// unrelated object elsewhere in the bundle.
fn probe_object_literal(source: &str) -> Option<&str> {
    let rhs = global_assignment_rhs(source)?;
    resolve_object_literal(source, rhs)
}

/// Return the right-hand side of a real `<...>.__OUROFORGE__ = <rhs>` assignment
/// (a single `=`, not `==`/`===`/`=>`), as the remainder of the source after the
/// `=`. e.g. `probe;\n  ...` for `globalScope.__OUROFORGE__ = probe;`.
fn global_assignment_rhs(source: &str) -> Option<&str> {
    let mut from = 0;
    while let Some(rel) = source[from..].find(PROBE_GLOBAL) {
        let idx = from + rel;
        let after = &source[idx + PROBE_GLOBAL.len()..];
        let trimmed = after.trim_start();
        if let Some(rest) = trimmed.strip_prefix('=') {
            if !rest.starts_with('=') && !rest.starts_with('>') {
                return Some(rest.trim_start());
            }
        }
        from = idx + PROBE_GLOBAL.len();
    }
    None
}

/// Resolve an expression to the source of an object literal: an inline `{...}`,
/// an `Object.freeze({...})` wrapper, or a single identifier bound to one via a
/// `const`/`let`/`var` declaration. Brace matching is string- and
/// comment-aware. Only the value's leading token is inspected, so trailing
/// source after the expression is ignored.
fn resolve_object_literal<'a>(source: &'a str, expr: &'a str) -> Option<&'a str> {
    let expr = expr.trim_start();
    if let Some(inner) = expr.strip_prefix("Object.freeze(") {
        let inner = inner.trim_start();
        if inner.starts_with('{') {
            return balanced_object(inner);
        }
        // Object.freeze(identifier)
        return resolve_object_literal(source, inner);
    }
    if expr.starts_with('{') {
        return balanced_object(expr);
    }
    // Treat as an identifier referencing a binding elsewhere in the source.
    let ident: String = expr
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '$')
        .collect();
    if ident.is_empty() {
        return None;
    }
    for kw in ["const ", "let ", "var "] {
        let needle = format!("{kw}{ident}");
        let mut from = 0;
        while let Some(rel) = source[from..].find(&needle) {
            let idx = from + rel;
            let after = &source[idx + needle.len()..];
            // Reject a longer identifier match (e.g. `const probeVersion`).
            let boundary_ok = after
                .chars()
                .next()
                .map(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '$'))
                .unwrap_or(true);
            if boundary_ok {
                let trimmed = after.trim_start();
                if let Some(rest) = trimmed.strip_prefix('=') {
                    if !rest.starts_with('=') && !rest.starts_with('>') {
                        let rhs = rest.trim_start();
                        if rhs.starts_with('{') {
                            return balanced_object(rhs);
                        }
                        if let Some(inner) = rhs.strip_prefix("Object.freeze(") {
                            let inner = inner.trim_start();
                            if inner.starts_with('{') {
                                return balanced_object(inner);
                            }
                        }
                    }
                }
            }
            from = idx + needle.len();
        }
    }
    None
}

/// Return the `{...}` slice beginning at the first `{` in `s`, matching braces
/// while skipping string literals (`'`, `"`, `` ` ``) and `//`/`/* */` comments
/// so braces inside strings or comments do not unbalance the scan.
fn balanced_object(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    let start = s.find('{')?;
    let mut depth = 0usize;
    let mut i = start;
    let mut in_str: Option<u8> = None;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    while i < bytes.len() {
        let b = bytes[i];
        if in_line_comment {
            if b == b'\n' {
                in_line_comment = false;
            }
            i += 1;
            continue;
        }
        if in_block_comment {
            if b == b'*' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                in_block_comment = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if let Some(quote) = in_str {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == quote {
                in_str = None;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' | b'\'' | b'`' => in_str = Some(b),
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'/' => {
                in_line_comment = true;
                i += 2;
                continue;
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'*' => {
                in_block_comment = true;
                i += 2;
                continue;
            }
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[start..=i]);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

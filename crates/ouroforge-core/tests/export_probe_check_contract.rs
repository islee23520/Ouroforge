//! Runtime Probe Preservation for Export v1 contract (#725).

use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_plan::ExportPlan;
use ouroforge_core::export_probe_check::{
    check_bundle_probe, check_probe_source, ensure_bundle_probe_compatible, ExportProbeMode,
    PROBE_GLOBAL,
};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn assembled_bundle(name: &str) -> PathBuf {
    let staging =
        std::env::temp_dir().join(format!("ouroforge-probe-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&staging);
    let profile = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .unwrap();
    let plan = ExportPlan::from_profile_json(&profile).unwrap();
    assemble_web_bundle(&plan, &repo_root(), &staging).unwrap();
    staging
}

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join("examples/export-probe-v1").join(rel))
        .unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn assembled_bundle_passes_both_probe_modes() {
    let bundle = assembled_bundle("ok");
    for mode in [
        ExportProbeMode::DevProbeEnabled,
        ExportProbeMode::PackagedProbeLimited,
    ] {
        let report = check_bundle_probe(&bundle, mode).expect("probe check runs");
        assert!(report.global_present);
        assert!(report.passed, "missing: {:?}", report.missing_methods);
        assert!(report.missing_methods.is_empty());
    }
    ensure_bundle_probe_compatible(&bundle, ExportProbeMode::DevProbeEnabled)
        .expect("dev probe compatible");
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn limited_mode_requires_fewer_methods_than_dev() {
    let limited = ExportProbeMode::PackagedProbeLimited.required_methods();
    let dev = ExportProbeMode::DevProbeEnabled.required_methods();
    assert!(limited.len() < dev.len());
    for m in &limited {
        assert!(dev.contains(m));
    }
    assert!(dev.contains(&"step"));
    assert!(!limited.contains(&"step"));
}

#[test]
fn missing_probe_hook_fails_closed() {
    let source = fixture("invalid/missing-getevents-bootstrap.js");
    let report = check_probe_source(&source, ExportProbeMode::DevProbeEnabled);
    assert!(report.global_present);
    assert!(!report.passed);
    assert!(report.missing_methods.contains(&"getEvents".to_string()));
}

#[test]
fn absent_probe_global_fails_closed() {
    let source = fixture("invalid/no-probe-global-bootstrap.js");
    let report = check_probe_source(&source, ExportProbeMode::PackagedProbeLimited);
    assert!(!report.global_present);
    assert!(!report.passed);
    assert!(!source.contains(PROBE_GLOBAL));
}

#[test]
fn comment_mention_does_not_satisfy_missing_hook() {
    // `getEvents` appears only in a comment; the real probe object lacks it, so
    // detection must scope to the installed probe object and fail closed (#725).
    let source = "\
'use strict';
(function () {
  // The probe global window.__OUROFORGE__ should expose getEvents() per #725.
  const probe = Object.freeze({
    getWorldState() { return {}; },
    getFrameStats() { return {}; },
    snapshot() { return {}; },
    step() {},
    pause() {},
    resume() {},
    setInput() {},
    restore() {},
  });
  const globalScope = typeof window !== 'undefined' ? window : globalThis;
  globalScope.__OUROFORGE__ = probe;
})();
";
    let report = check_probe_source(source, ExportProbeMode::DevProbeEnabled);
    assert!(report.global_present, "global is genuinely installed");
    assert!(!report.passed, "getEvents only in a comment must not pass");
    assert!(report.missing_methods.contains(&"getEvents".to_string()));
}

#[test]
fn global_mentioned_only_in_comment_is_absent() {
    // A stray `__OUROFORGE__` mention with no real assignment must report the
    // probe global as absent rather than present (#725).
    let source = "\
'use strict';
// Note: window.__OUROFORGE__ is intentionally NOT installed in this bundle.
(function () { let tick = 0; tick += 1; })();
";
    let report = check_probe_source(source, ExportProbeMode::PackagedProbeLimited);
    assert!(
        !report.global_present,
        "comment mention is not an installation"
    );
    assert!(!report.passed);
}

#[test]
fn ensure_returns_actionable_error_for_missing_bundle() {
    let missing =
        std::env::temp_dir().join(format!("ouroforge-probe-missing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&missing);
    let err = ensure_bundle_probe_compatible(&missing, ExportProbeMode::DevProbeEnabled)
        .expect_err("missing bundle fails");
    assert!(err.to_string().contains("missing runtime bootstrap"));
}

#[test]
fn report_serializes_as_evidence() {
    let bundle = assembled_bundle("evidence");
    let report = check_bundle_probe(&bundle, ExportProbeMode::PackagedProbeLimited).unwrap();
    let json = report.to_json().unwrap();
    assert!(json.contains("packaged-probe-limited"));
    assert!(json.contains("\"passed\": true"));
    std::fs::remove_dir_all(&bundle).ok();
}

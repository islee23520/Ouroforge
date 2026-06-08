use std::path::{Path, PathBuf};

use ouroforge_core::{
    validate_locale_catalog, LocaleCatalog, LocalizationValidationStatus, StringCatalog,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read(relative)).expect(relative)
}

#[test]
fn localization_demo_validates_complete_title_and_rejects_incomplete_locale() {
    let manifest = read_json("examples/localization-v1/demo/demo-manifest.json");
    assert_eq!(manifest["schemaVersion"], "ouroforge.localization-demo.v1");
    assert_eq!(manifest["issue"], 1834);
    assert_eq!(manifest["determinism"]["network"], "disabled");
    assert_eq!(manifest["determinism"]["liveBrowser"], "not required");

    let catalog_ref = manifest["catalogRef"].as_str().unwrap();
    let complete_ref = manifest["completeLocaleRef"].as_str().unwrap();
    let rejected_ref = manifest["rejectedLocaleRef"].as_str().unwrap();

    let catalog = StringCatalog::from_json_str(&read(catalog_ref)).expect("catalog validates");
    let source_title = catalog
        .entries
        .iter()
        .find(|entry| entry.id == manifest["expected"]["sourceTitleId"])
        .expect("source title entry");
    assert_eq!(source_title.text, manifest["expected"]["sourceTitle"]);

    let complete =
        LocaleCatalog::from_json_str(&read(complete_ref)).expect("complete locale parses");
    let complete_report = validate_locale_catalog(&catalog, &complete).expect("complete report");
    assert_eq!(complete.locale, manifest["expected"]["completeLocale"]);
    assert_eq!(complete_report.status, LocalizationValidationStatus::Pass);
    assert_eq!(
        complete.translations[&source_title.id],
        manifest["expected"]["localizedTitle"]
    );

    let rejected =
        LocaleCatalog::from_json_str(&read(rejected_ref)).expect("rejected locale parses");
    let rejected_report = validate_locale_catalog(&catalog, &rejected).expect("rejected report");
    assert_eq!(rejected.locale, manifest["expected"]["rejectedLocale"]);
    assert_eq!(rejected_report.status, LocalizationValidationStatus::Fail);
    let reason = manifest["expected"]["rejectedReasonContains"]
        .as_str()
        .unwrap();
    assert!(rejected_report
        .issues
        .iter()
        .any(|issue| issue.contains(reason)));
}

#[test]
fn localization_demo_docs_preserve_boundaries() {
    let docs = read("docs/localization-v1-demo.md");
    let manifest = read("examples/localization-v1/demo/demo-manifest.json");
    let combined = format!("{docs}\n{manifest}");
    for required in [
        "Issue: #1834",
        "no network",
        "no live browser",
        "proposal-only",
        "review/apply/trust-gradient",
        "generated localization text remains proposal-only",
        "Godot replacement/parity claim",
        "#1 and #23 remain open",
    ] {
        assert!(
            combined.contains(required),
            "missing demo boundary: {required}"
        );
    }
}

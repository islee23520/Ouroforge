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

fn catalog() -> StringCatalog {
    StringCatalog::from_json_str(&read(
        "examples/localization-v1/string-catalog.complete.fixture.json",
    ))
    .expect("catalog validates")
}

#[test]
fn v48_matrix_enumerates_required_rows_and_boundaries() {
    let matrix = read_json("examples/localization-v1/scenario-coverage-v48/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "ouroforge.scenario-coverage.v48.localization.v1"
    );
    assert_eq!(matrix["issue"], "1835");
    let boundary = matrix["boundary"].as_str().unwrap();
    for required in [
        "Rust/local",
        "browser/Studio read-only",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "no timing flakes",
        "no auto-merge",
        "no self-approval",
        "no creative/tone automation",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "missing boundary {required}");
    }
    let ids = matrix["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "V48.externalization.catalog",
            "V48.locale.generation.valid",
            "V48.locale.missing.reject",
            "V48.locale.placeholder.reject",
            "V48.demo.smoke",
            "V48.default_locale.backcompat",
        ]
    );
}

#[test]
fn v48_externalization_and_locale_validation_states_are_locked() {
    let catalog = catalog();
    assert_eq!(catalog.source_locale, "en-US");
    assert_eq!(catalog.entries.len(), 4);
    assert!(catalog
        .entries
        .iter()
        .all(|entry| !entry.source_ref.is_empty()));

    let complete =
        LocaleCatalog::from_json_str(&read("examples/localization-v1/locale.es.fixture.json"))
            .expect("complete locale parses");
    let complete_report = validate_locale_catalog(&catalog, &complete).unwrap();
    assert_eq!(complete_report.status, LocalizationValidationStatus::Pass);
    assert!(complete_report.issues.is_empty());

    let missing = LocaleCatalog::from_json_str(&read(
        "examples/localization-v1/invalid/locale.missing.fixture.json",
    ))
    .unwrap();
    let missing_report = validate_locale_catalog(&catalog, &missing).unwrap();
    assert_eq!(missing_report.status, LocalizationValidationStatus::Fail);
    assert!(missing_report
        .issues
        .iter()
        .any(|issue| issue.contains("missing translation for deckbuilder.runmap.blocked")));

    let mismatch = LocaleCatalog::from_json_str(&read(
        "examples/localization-v1/invalid/locale.placeholder-mismatch.fixture.json",
    ))
    .unwrap();
    let mismatch_report = validate_locale_catalog(&catalog, &mismatch).unwrap();
    assert_eq!(mismatch_report.status, LocalizationValidationStatus::Fail);
    assert!(mismatch_report
        .issues
        .iter()
        .any(|issue| issue.contains("placeholder mismatch")));
}

#[test]
fn v48_demo_and_default_locale_backcompat_golden_are_locked() {
    let catalog = catalog();
    let demo = read_json("examples/localization-v1/demo/demo-manifest.json");
    assert_eq!(demo["schemaVersion"], "ouroforge.localization-demo.v1");
    assert_eq!(demo["determinism"]["network"], "disabled");
    assert_eq!(demo["determinism"]["liveBrowser"], "not required");
    assert_eq!(
        demo["expected"]["localizedTitle"],
        "IU de cartas / mano / canalización"
    );
    assert_eq!(demo["governance"]["anchors"], "#1 and #23 remain open");

    let golden = read_json(
        "examples/localization-v1/scenario-coverage-v48/default-locale-backcompat-golden.json",
    );
    assert_eq!(golden["expected"]["sourceLocale"], catalog.source_locale);
    let title = catalog
        .entries
        .iter()
        .find(|entry| entry.id == golden["expected"]["titleId"])
        .expect("title entry");
    assert_eq!(title.text, golden["expected"]["titleText"]);
    let balance = catalog
        .entries
        .iter()
        .find(|entry| entry.id == golden["expected"]["shopBalanceId"])
        .expect("balance entry");
    assert_eq!(
        balance.placeholders,
        vec!["amount".to_string(), "currency".to_string()]
    );
    assert_eq!(golden["expected"]["proposalRequiredForSourceLocale"], false);
}

#[test]
fn v48_doc_records_state_shape_scope() {
    let doc = read("docs/scenario-coverage-v48.md");
    for required in [
        "state/shape checks only",
        "default-locale",
        "backward-compatibility golden",
        "Generated runs/artifacts remain",
        "Issues #1 and #23 remain open",
        "cargo test -p ouroforge-core --test scenario_coverage_v48_localization",
        "translation quality",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

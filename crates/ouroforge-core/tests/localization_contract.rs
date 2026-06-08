use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    generate_locale_catalog, validate_locale_catalog, LocaleCatalog, LocalizationValidationStatus,
    StringCatalog, LOCALIZATION_BOUNDARY, LOCALIZATION_GENERATOR,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn catalog() -> StringCatalog {
    StringCatalog::from_json_str(&read(
        "examples/localization-v1/string-catalog.complete.fixture.json",
    ))
    .expect("catalog parses and validates")
}

#[test]
fn externalization_catalog_is_complete_and_conservative() {
    let catalog = catalog();
    assert_eq!(catalog.catalog_id, "deckbuilder-ui-localization-v1");
    assert_eq!(catalog.source_locale, "en-US");
    assert!(catalog.boundary.contains("Rust/local"));
    assert!(catalog.boundary.contains("proposal-only"));
    assert!(catalog.boundary.contains("read-only"));
    assert!(catalog.boundary.contains("#1 and #23 remain open"));
    assert!(catalog.generated_state_policy.contains("fixture-scoped"));

    let ids = catalog
        .entries
        .iter()
        .map(|entry| entry.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "deckbuilder.ui.title",
            "deckbuilder.shop.balance",
            "deckbuilder.runmap.blocked",
            "deckbuilder.score.authority",
        ]
    );
    assert!(catalog
        .entries
        .iter()
        .all(|entry| !entry.source_ref.is_empty() && !entry.context.is_empty()));
}

#[test]
fn translated_locale_validates_completeness_and_placeholder_integrity() {
    let catalog = catalog();
    let locale =
        LocaleCatalog::from_json_str(&read("examples/localization-v1/locale.es.fixture.json"))
            .expect("locale parses");
    let report = validate_locale_catalog(&catalog, &locale).expect("validation report");
    assert_eq!(report.status, LocalizationValidationStatus::Pass);
    assert_eq!(report.checked_entry_count, catalog.entries.len());
    assert!(report.issues.is_empty());
    assert_eq!(locale.generated_by, LOCALIZATION_GENERATOR);
    assert!(locale.proposal_only);
    assert_eq!(locale.boundary, LOCALIZATION_BOUNDARY);
}

#[test]
fn generation_produces_proposal_only_validated_locale() {
    let catalog = catalog();
    let mut translations = BTreeMap::new();
    translations.insert(
        "deckbuilder.ui.title".to_string(),
        "Interface cartes / main / pipeline".to_string(),
    );
    translations.insert(
        "deckbuilder.shop.balance".to_string(),
        "Solde : {amount} {currency}".to_string(),
    );
    translations.insert(
        "deckbuilder.runmap.blocked".to_string(),
        "Nécessite une rencontre terminée".to_string(),
    );
    translations.insert(
        "deckbuilder.score.authority".to_string(),
        "Score Rust/local faisant autorité {score}".to_string(),
    );

    let locale = generate_locale_catalog(&catalog, "fr-FR", translations)
        .expect("complete placeholder-safe locale generates");
    assert_eq!(locale.schema_version, "ouroforge.localization.locale.v1");
    assert_eq!(locale.locale, "fr-FR");
    assert!(locale.proposal_only);
    assert_eq!(locale.boundary, LOCALIZATION_BOUNDARY);
}

#[test]
fn missing_translation_rejects_fail_closed() {
    let catalog = catalog();
    let locale = LocaleCatalog::from_json_str(&read(
        "examples/localization-v1/invalid/locale.missing.fixture.json",
    ))
    .expect("invalid locale still parses");
    let report = validate_locale_catalog(&catalog, &locale).expect("validation report");
    assert_eq!(report.status, LocalizationValidationStatus::Fail);
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.contains("missing translation for deckbuilder.runmap.blocked")));
}

#[test]
fn placeholder_mismatch_rejects_fail_closed() {
    let catalog = catalog();
    let locale = LocaleCatalog::from_json_str(&read(
        "examples/localization-v1/invalid/locale.placeholder-mismatch.fixture.json",
    ))
    .expect("invalid locale still parses");
    let report = validate_locale_catalog(&catalog, &locale).expect("validation report");
    assert_eq!(report.status, LocalizationValidationStatus::Fail);
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.contains("deckbuilder.shop.balance placeholder mismatch")));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.contains("deckbuilder.score.authority placeholder mismatch")));
}

#[test]
fn docs_record_generated_state_wording_and_governance() {
    let docs = read("docs/localization-v1.md");
    for required in [
        "Issue: #1833",
        "proposal-only",
        "review/apply/trust-gradient",
        "Generated runs/artifacts remain untracked unless fixture-scoped",
        "no new engine",
        "auto-merge",
        "quality/fun claim",
        "Godot replacement/parity claim",
        "#1 and #23 remain open",
    ] {
        assert!(docs.contains(required), "missing doc text {required}");
    }
}

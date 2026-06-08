use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const LOCALIZATION_CATALOG_SCHEMA_VERSION: &str = "ouroforge.localization.string-catalog.v1";
pub const LOCALIZATION_LOCALE_SCHEMA_VERSION: &str = "ouroforge.localization.locale.v1";
pub const LOCALIZATION_GENERATOR: &str = "ouroforge.localization.generator.v1";
pub const LOCALIZATION_BOUNDARY: &str = "Rust/local localization validation; generation is proposal-only; browser/Studio surfaces remain read-only or draft-only; trusted writes route through the existing review/apply/trust-gradient path; no automated fun, quality, production, release, or Godot replacement claim; generated runs/artifacts remain untracked unless fixture-scoped; #1 and #23 remain open";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StringCatalog {
    pub schema_version: String,
    pub catalog_id: String,
    pub source_locale: String,
    pub boundary: String,
    pub generated_state_policy: String,
    pub entries: Vec<StringCatalogEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StringCatalogEntry {
    pub id: String,
    pub source_ref: String,
    pub context: String,
    pub text: String,
    #[serde(default)]
    pub placeholders: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocaleCatalog {
    pub schema_version: String,
    pub catalog_id: String,
    pub locale: String,
    pub generated_by: String,
    pub proposal_only: bool,
    pub boundary: String,
    pub translations: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalizationValidationReport {
    pub schema_version: String,
    pub catalog_id: String,
    pub locale: String,
    pub status: LocalizationValidationStatus,
    pub checked_entry_count: usize,
    pub issues: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalizationValidationStatus {
    Pass,
    Fail,
}

impl StringCatalog {
    pub fn from_json_str(text: &str) -> Result<Self> {
        let catalog: Self = serde_json::from_str(text)?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn validate(&self) -> Result<()> {
        let mut issues = Vec::new();
        if self.schema_version != LOCALIZATION_CATALOG_SCHEMA_VERSION {
            issues.push(format!(
                "schemaVersion must be {LOCALIZATION_CATALOG_SCHEMA_VERSION}"
            ));
        }
        if self.catalog_id.trim().is_empty() {
            issues.push("catalogId must not be empty".to_string());
        }
        if self.source_locale.trim().is_empty() {
            issues.push("sourceLocale must not be empty".to_string());
        }
        for required in [
            "Rust/local",
            "proposal-only",
            "read-only",
            "review/apply/trust-gradient",
            "#1 and #23 remain open",
        ] {
            if !self.boundary.contains(required) {
                issues.push(format!("boundary missing {required}"));
            }
        }
        if !self.generated_state_policy.contains("fixture-scoped") {
            issues.push(
                "generatedStatePolicy must keep generated artifacts fixture-scoped".to_string(),
            );
        }
        if self.entries.is_empty() {
            issues.push("entries must not be empty".to_string());
        }
        let mut ids = BTreeSet::new();
        for entry in &self.entries {
            if !ids.insert(entry.id.as_str()) {
                issues.push(format!("duplicate entry id {}", entry.id));
            }
            if entry.id.trim().is_empty() {
                issues.push("entry id must not be empty".to_string());
            }
            if entry.source_ref.trim().is_empty() {
                issues.push(format!("{} sourceRef must not be empty", entry.id));
            }
            if entry.context.trim().is_empty() {
                issues.push(format!("{} context must not be empty", entry.id));
            }
            if entry.text.trim().is_empty() {
                issues.push(format!("{} text must not be empty", entry.id));
            }
            let declared = normalize_placeholders(&entry.placeholders);
            let discovered = extract_placeholders(&entry.text);
            if declared != discovered {
                issues.push(format!(
                    "{} placeholders must match source text: declared {:?}, found {:?}",
                    entry.id, declared, discovered
                ));
            }
        }
        if issues.is_empty() {
            Ok(())
        } else {
            Err(anyhow!(issues.join("; ")))
        }
    }
}

impl LocaleCatalog {
    pub fn from_json_str(text: &str) -> Result<Self> {
        let locale: Self = serde_json::from_str(text)?;
        Ok(locale)
    }
}

pub fn generate_locale_catalog(
    catalog: &StringCatalog,
    locale: &str,
    translated_text: BTreeMap<String, String>,
) -> Result<LocaleCatalog> {
    catalog.validate()?;
    let candidate = LocaleCatalog {
        schema_version: LOCALIZATION_LOCALE_SCHEMA_VERSION.to_string(),
        catalog_id: catalog.catalog_id.clone(),
        locale: locale.to_string(),
        generated_by: LOCALIZATION_GENERATOR.to_string(),
        proposal_only: true,
        boundary: LOCALIZATION_BOUNDARY.to_string(),
        translations: translated_text,
    };
    let report = validate_locale_catalog(catalog, &candidate)?;
    if report.status == LocalizationValidationStatus::Pass {
        Ok(candidate)
    } else {
        Err(anyhow!(report.issues.join("; ")))
    }
}

pub fn validate_locale_catalog(
    catalog: &StringCatalog,
    locale: &LocaleCatalog,
) -> Result<LocalizationValidationReport> {
    catalog.validate()?;
    let mut issues = Vec::new();
    if locale.schema_version != LOCALIZATION_LOCALE_SCHEMA_VERSION {
        issues.push(format!(
            "schemaVersion must be {LOCALIZATION_LOCALE_SCHEMA_VERSION}"
        ));
    }
    if locale.catalog_id != catalog.catalog_id {
        issues.push("locale catalogId must match source catalog".to_string());
    }
    if locale.locale.trim().is_empty() || locale.locale == catalog.source_locale {
        issues.push("locale must be non-empty and distinct from sourceLocale".to_string());
    }
    if locale.generated_by != LOCALIZATION_GENERATOR {
        issues.push(format!("generatedBy must be {LOCALIZATION_GENERATOR}"));
    }
    if !locale.proposal_only {
        issues.push("proposalOnly must be true".to_string());
    }
    for required in [
        "Rust/local",
        "proposal-only",
        "read-only",
        "review/apply/trust-gradient",
        "#1 and #23 remain open",
    ] {
        if !locale.boundary.contains(required) {
            issues.push(format!("locale boundary missing {required}"));
        }
    }

    let source_ids = catalog
        .entries
        .iter()
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    for entry in &catalog.entries {
        match locale.translations.get(&entry.id) {
            Some(text) if !text.trim().is_empty() => {
                let expected = normalize_placeholders(&entry.placeholders);
                let actual = extract_placeholders(text);
                if expected != actual {
                    issues.push(format!(
                        "{} placeholder mismatch: expected {:?}, found {:?}",
                        entry.id, expected, actual
                    ));
                }
            }
            Some(_) => issues.push(format!("{} translation must not be empty", entry.id)),
            None => issues.push(format!("missing translation for {}", entry.id)),
        }
    }
    for key in locale.translations.keys() {
        if !source_ids.contains(key.as_str()) {
            issues.push(format!("unknown translation id {key}"));
        }
    }

    Ok(LocalizationValidationReport {
        schema_version: "ouroforge.localization.validation-report.v1".to_string(),
        catalog_id: catalog.catalog_id.clone(),
        locale: locale.locale.clone(),
        status: if issues.is_empty() {
            LocalizationValidationStatus::Pass
        } else {
            LocalizationValidationStatus::Fail
        },
        checked_entry_count: catalog.entries.len(),
        issues,
        boundary: LOCALIZATION_BOUNDARY.to_string(),
    })
}

fn normalize_placeholders(placeholders: &[String]) -> BTreeSet<String> {
    placeholders
        .iter()
        .map(|placeholder| placeholder.trim().to_string())
        .filter(|placeholder| !placeholder.is_empty())
        .collect()
}

fn extract_placeholders(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'{' {
            if let Some(end) = text[index + 1..].find('}') {
                let candidate = &text[index + 1..index + 1 + end];
                if is_placeholder_name(candidate) {
                    out.insert(candidate.to_string());
                }
                index += end + 2;
                continue;
            }
        }
        index += 1;
    }
    out
}

fn is_placeholder_name(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

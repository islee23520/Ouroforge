//! Product gap taxonomy adapter for #2350 consumers.
//!
//! The enum source of truth is `docs/product-gap-taxonomy.json`; this module
//! reads that fixture at compile time and validates downstream category/severity
//! ids against it instead of redeclaring a parallel Rust enum.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

pub const PRODUCT_GAP_TAXONOMY_JSON: &str = include_str!("../../../docs/product-gap-taxonomy.json");

#[derive(Debug, Deserialize)]
struct ProductGapTaxonomy {
    #[serde(rename = "categoryEnum")]
    category_enum: Vec<ProductGapCategory>,
    #[serde(rename = "severityEnum")]
    severity_enum: Vec<ProductGapSeverity>,
}

#[derive(Debug, Deserialize)]
struct ProductGapCategory {
    id: String,
    #[serde(rename = "defaultOwner")]
    default_owner: String,
}

#[derive(Debug, Deserialize)]
struct ProductGapSeverity {
    id: String,
}

fn taxonomy() -> Result<ProductGapTaxonomy> {
    serde_json::from_str(PRODUCT_GAP_TAXONOMY_JSON)
        .context("failed to parse docs/product-gap-taxonomy.json")
}

pub fn product_gap_category_ids() -> Result<Vec<String>> {
    Ok(taxonomy()?
        .category_enum
        .into_iter()
        .map(|c| c.id)
        .collect())
}

pub fn product_gap_severity_ids() -> Result<Vec<String>> {
    Ok(taxonomy()?
        .severity_enum
        .into_iter()
        .map(|s| s.id)
        .collect())
}

pub fn validate_product_gap_category(field: &str, value: &str) -> Result<()> {
    let taxonomy = taxonomy()?;
    if taxonomy.category_enum.iter().any(|c| c.id == value) {
        Ok(())
    } else {
        Err(anyhow!(
            "{field} `{value}` is not in docs/product-gap-taxonomy.json categoryEnum"
        ))
    }
}

pub fn validate_product_gap_severity(field: &str, value: &str) -> Result<()> {
    let taxonomy = taxonomy()?;
    if taxonomy.severity_enum.iter().any(|s| s.id == value) {
        Ok(())
    } else {
        Err(anyhow!(
            "{field} `{value}` is not in docs/product-gap-taxonomy.json severityEnum"
        ))
    }
}

pub fn default_owner_for_category(category: &str) -> Result<String> {
    taxonomy()?
        .category_enum
        .into_iter()
        .find(|entry| entry.id == category)
        .map(|entry| entry.default_owner)
        .ok_or_else(|| anyhow!("category `{category}` is not in docs/product-gap-taxonomy.json"))
}

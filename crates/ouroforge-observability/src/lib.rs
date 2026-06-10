use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

pub const SCHEMA_VERSION: &str = "live-observability-v1";

const REQUIRED_ARTIFACTS: &[&str] = &[
    "manifest.json",
    "console.jsonl",
    "frame-stats.jsonl",
    "world-samples.jsonl",
    "events.json",
    "input-replay.json",
    "screenshots/",
    "verdict.md",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub bundle_root: PathBuf,
    pub run_id: String,
    pub run_kind: String,
    pub artifact_count: usize,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: String,
    run_id: String,
    created_at: String,
    target_url: String,
    run_kind: String,
    tool_versions: Value,
    browser: Value,
    retry_attempts: u64,
    artifact_inventory: Vec<ArtifactInventoryEntry>,
}

#[derive(Debug, Deserialize)]
struct ArtifactInventoryEntry {
    path: String,
    kind: String,
    #[serde(default)]
    sha256: Option<String>,
    required: bool,
}

pub fn required_artifacts() -> &'static [&'static str] {
    REQUIRED_ARTIFACTS
}

pub fn validate_bundle(bundle_root: impl AsRef<Path>) -> Result<ValidationReport> {
    let bundle_root = bundle_root.as_ref();
    let manifest_path = bundle_root.join("manifest.json");
    if !manifest_path.is_file() {
        bail!("missing required artifact: manifest.json");
    }

    for required in REQUIRED_ARTIFACTS {
        let path = bundle_root.join(required.trim_end_matches('/'));
        if required.ends_with('/') {
            if !path.is_dir() {
                bail!("missing required directory artifact: {required}");
            }
        } else if !path.is_file() {
            bail!("missing required file artifact: {required}");
        }
    }

    let manifest: Manifest = serde_json::from_slice(&fs::read(&manifest_path)?)
        .context("manifest.json is not valid JSON")?;
    validate_manifest(bundle_root, &manifest)?;
    validate_jsonl(bundle_root.join("console.jsonl"), "console.jsonl")?;
    validate_jsonl(bundle_root.join("frame-stats.jsonl"), "frame-stats.jsonl")?;
    validate_jsonl(
        bundle_root.join("world-samples.jsonl"),
        "world-samples.jsonl",
    )?;
    validate_versioned_object(
        bundle_root.join("events.json"),
        "events.json",
        Some("events"),
    )?;
    validate_versioned_object(
        bundle_root.join("input-replay.json"),
        "input-replay.json",
        Some("steps"),
    )?;
    let verdict = fs::read_to_string(bundle_root.join("verdict.md"))?;
    if verdict.trim().is_empty() {
        bail!("verdict.md must not be empty");
    }

    Ok(ValidationReport {
        bundle_root: bundle_root.to_path_buf(),
        run_id: manifest.run_id,
        run_kind: manifest.run_kind,
        artifact_count: manifest.artifact_inventory.len(),
    })
}

fn validate_manifest(bundle_root: &Path, manifest: &Manifest) -> Result<()> {
    if manifest.schema_version != SCHEMA_VERSION {
        bail!("manifest schema_version must be {SCHEMA_VERSION}");
    }
    if manifest.run_id.trim().is_empty() {
        bail!("manifest run_id must not be empty");
    }
    if let Some(dir_name) = bundle_root.file_name().and_then(|name| name.to_str()) {
        if dir_name != manifest.run_id {
            bail!(
                "manifest run_id '{}' does not match bundle directory '{dir_name}'",
                manifest.run_id
            );
        }
    }
    if !is_timestamp_like(&manifest.created_at) {
        bail!("manifest created_at must be an ISO/RFC3339-like UTC timestamp ending in Z");
    }
    if !is_local_http_url(&manifest.target_url) {
        bail!("manifest target_url must be local-only http://127.0.0.1:<port>/... or http://localhost:<port>/...");
    }
    if !matches!(
        manifest.run_kind.as_str(),
        "runtime" | "studio" | "authoring"
    ) {
        bail!("manifest run_kind must be runtime, studio, or authoring");
    }
    if !manifest.tool_versions.is_object() {
        bail!("manifest tool_versions must be an object");
    }
    if !manifest.browser.is_object() {
        bail!("manifest browser must be an object");
    }
    let _attempts = manifest.retry_attempts;

    let mut inventory_by_path: HashMap<&str, &ArtifactInventoryEntry> = HashMap::new();
    for entry in &manifest.artifact_inventory {
        if entry.path.starts_with('/') || entry.path.contains("..") || entry.path.contains('\\') {
            bail!(
                "artifact inventory path must be safe and relative: {}",
                entry.path
            );
        }
        if !matches!(
            entry.kind.as_str(),
            "json" | "jsonl" | "markdown" | "directory" | "png" | "other"
        ) {
            bail!(
                "artifact inventory entry has unsupported kind: {}",
                entry.kind
            );
        }
        inventory_by_path.insert(entry.path.as_str(), entry);
    }

    for required in REQUIRED_ARTIFACTS {
        let entry = inventory_by_path
            .get(required)
            .ok_or_else(|| anyhow!("artifact inventory missing required path: {required}"))?;
        if !entry.required {
            bail!("required artifact inventory entry is not marked required: {required}");
        }
    }

    for entry in &manifest.artifact_inventory {
        let path = bundle_root.join(entry.path.trim_end_matches('/'));
        if entry.kind == "directory" || entry.path.ends_with('/') {
            if !path.is_dir() {
                bail!("inventory directory does not exist: {}", entry.path);
            }
            continue;
        }
        if !path.is_file() {
            bail!("inventory file does not exist: {}", entry.path);
        }
        if entry.path == "manifest.json" {
            if let Some(expected) = entry.sha256.as_deref() {
                if !is_lower_hex_sha256(expected) {
                    bail!("inventory manifest.json has invalid sha256 shape");
                }
            }
            continue;
        }
        let expected = entry
            .sha256
            .as_deref()
            .ok_or_else(|| anyhow!("inventory file missing sha256: {}", entry.path))?;
        if !is_lower_hex_sha256(expected) {
            bail!("inventory file has invalid sha256 shape: {}", entry.path);
        }
        let actual = sha256_file(&path)?;
        if actual != expected {
            bail!(
                "inventory sha256 mismatch for {}: expected {expected}, got {actual}",
                entry.path
            );
        }
    }

    Ok(())
}

fn validate_jsonl(path: PathBuf, label: &str) -> Result<()> {
    let content = fs::read_to_string(&path).with_context(|| format!("reading {label}"))?;
    for (index, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(line)
            .with_context(|| format!("{label} line {} is not valid JSON", index + 1))?;
        let object = value
            .as_object()
            .ok_or_else(|| anyhow!("{label} line {} must be a JSON object", index + 1))?;
        require_schema_and_timestamp(object, label, index + 1)?;
    }
    Ok(())
}

fn validate_versioned_object(path: PathBuf, label: &str, array_field: Option<&str>) -> Result<()> {
    let value: Value =
        serde_json::from_slice(&fs::read(&path).with_context(|| format!("reading {label}"))?)
            .with_context(|| format!("{label} is not valid JSON"))?;
    let object = value
        .as_object()
        .ok_or_else(|| anyhow!("{label} must be a JSON object"))?;
    if object.get("schema_version").and_then(Value::as_str) != Some(SCHEMA_VERSION) {
        bail!("{label} schema_version must be {SCHEMA_VERSION}");
    }
    if let Some(field) = array_field {
        if !object.get(field).is_some_and(Value::is_array) {
            bail!("{label} must contain array field {field}");
        }
    }
    Ok(())
}

fn require_schema_and_timestamp(
    object: &serde_json::Map<String, Value>,
    label: &str,
    line: usize,
) -> Result<()> {
    if object.get("schema_version").and_then(Value::as_str) != Some(SCHEMA_VERSION) {
        bail!("{label} line {line} schema_version must be {SCHEMA_VERSION}");
    }
    match object.get("timestamp").and_then(Value::as_str) {
        Some(timestamp) if is_timestamp_like(timestamp) => Ok(()),
        _ => bail!("{label} line {line} timestamp must be ISO/RFC3339-like UTC ending in Z"),
    }
}

fn is_timestamp_like(value: &str) -> bool {
    value.len() >= 20 && value.contains('T') && value.ends_with('Z')
}

fn is_local_http_url(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("http://") else {
        return false;
    };
    let host_port_path = if let Some(rest) = rest.strip_prefix("127.0.0.1:") {
        rest
    } else if let Some(rest) = rest.strip_prefix("localhost:") {
        rest
    } else {
        return false;
    };
    let Some((port, _path)) = host_port_path.split_once('/') else {
        return false;
    };
    !port.is_empty() && port.chars().all(|ch| ch.is_ascii_digit())
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .chars()
            .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase())
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(fs::read(path)?);
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn artifact_inventory_paths(bundle_root: impl AsRef<Path>) -> Result<BTreeSet<String>> {
    let manifest: Manifest =
        serde_json::from_slice(&fs::read(bundle_root.as_ref().join("manifest.json"))?)?;
    Ok(manifest
        .artifact_inventory
        .into_iter()
        .map(|entry| entry.path)
        .collect())
}

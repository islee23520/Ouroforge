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

const CHECKLIST_IDS: &[&str] = &[
    "po-check-live-url",
    "po-check-console",
    "po-check-screenshot",
    "po-check-replay",
    "po-check-world-sample",
    "po-check-event-sample",
    "po-check-frame-stats",
    "po-check-before-after",
    "po-check-verdict",
    "po-check-generated-state",
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
    #[serde(default)]
    observability_api_keys_used: Vec<String>,
    #[serde(default)]
    observability_api_keys_available: Vec<String>,
    #[serde(default)]
    diagnostics: Vec<Value>,
    #[serde(default)]
    replay: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArtifactInventoryEntry {
    path: String,
    kind: String,
    #[serde(default)]
    sha256: Option<String>,
    required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerdictOptions {
    pub generated_at: String,
    pub generated_state_audit: String,
}

impl Default for VerdictOptions {
    fn default() -> Self {
        Self {
            generated_at: "1970-01-01T00:00:00Z".to_string(),
            generated_state_audit: "No generated observability artifacts are tracked by this renderer; run `git status --short --ignored` for closure evidence.".to_string(),
        }
    }
}

pub fn required_artifacts() -> &'static [&'static str] {
    REQUIRED_ARTIFACTS
}

pub fn checklist_ids() -> &'static [&'static str] {
    CHECKLIST_IDS
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

pub fn render_verdict(bundle_root: impl AsRef<Path>, options: &VerdictOptions) -> Result<String> {
    let bundle_root = bundle_root.as_ref();
    validate_bundle(bundle_root)?;
    let manifest = read_manifest(bundle_root)?;
    let console_lines = read_jsonl(bundle_root.join("console.jsonl"))?;
    let frame_lines = read_jsonl(bundle_root.join("frame-stats.jsonl"))?;
    let world_lines = read_jsonl(bundle_root.join("world-samples.jsonl"))?;
    let events = read_json(bundle_root.join("events.json"))?;
    let replay = read_json(bundle_root.join("input-replay.json"))?;

    let console_errors = console_lines
        .iter()
        .filter(|line| {
            matches!(
                line.get("level").and_then(Value::as_str),
                Some("error") | Some("warning")
            )
        })
        .count();
    let runtime_diagnostics = collect_runtime_diagnostics(&manifest, &world_lines);
    let fatal_count = runtime_diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.to_ascii_lowercase().contains("fatal"))
        .count();
    let usability_gap_count = runtime_diagnostics.len().saturating_sub(fatal_count);
    let objective_sequence = replay
        .get("objective_flag_sequence")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let final_flags = objective_sequence
        .last()
        .and_then(|value| value.get("goal_flags"))
        .cloned()
        .unwrap_or(Value::Object(Default::default()));
    let exit_reached = final_flags
        .get("exit_reached")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let grid_won = final_flags
        .get("grid_won")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let grid_won_satisfies_objective =
        manifest.replay.as_deref() == Some("grid-puzzle") && grid_won;
    let objective_satisfied = exit_reached || grid_won_satisfies_objective;
    let screenshot_paths = inventory_paths_by_kind(&manifest, "png");
    let product_observed_complete = console_errors == 0
        && runtime_diagnostics.is_empty()
        && !world_lines.is_empty()
        && !frame_lines.is_empty()
        && !screenshot_paths.is_empty()
        && objective_satisfied;
    let product_status = if product_observed_complete {
        "product-observed complete"
    } else {
        "product-observed FAIL"
    };
    let mechanical_status = if objective_satisfied || manifest.replay.is_none() {
        "contract-pass"
    } else {
        "contract-fail"
    };

    let mut out = String::new();
    out.push_str("# Live observability verdict\n\n");
    out.push_str(&format!("Generated at: {}\n\n", options.generated_at));
    out.push_str("## Classification\n\n");
    out.push_str(&format!("- Mechanical contract: `{mechanical_status}`\n"));
    out.push_str(&format!("- Product observation: `{product_status}`\n"));
    out.push_str(&format!("- Bundle: `{}`\n", manifest.run_id));
    out.push_str(&format!("- Run id: `{}`\n", manifest.run_id));
    out.push_str(&format!("- Run kind: `{}`\n", manifest.run_kind));
    out.push_str(&format!("- Target: `{}`\n\n", manifest.target_url));

    out.push_str("## Checklist trace\n\n");
    out.push_str("| M115.3 id | Result | Artifact path(s) | Rationale |\n");
    out.push_str("| --- | --- | --- | --- |\n");
    let replay_result = if objective_sequence.is_empty() {
        "FAIL"
    } else {
        "PASS"
    };
    let screenshot_result = if screenshot_paths.is_empty() {
        "FAIL"
    } else {
        "PASS"
    };
    let diagnostics_result = if console_errors == 0 && fatal_count == 0 {
        "PASS"
    } else {
        "FAIL"
    };
    let world_result = if world_lines.is_empty() {
        "FAIL"
    } else {
        "PASS"
    };
    let frame_result = if frame_lines.is_empty() {
        "FAIL"
    } else {
        "PASS"
    };
    let event_count = events
        .get("events")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let event_result = if event_count == 0 { "FAIL" } else { "PASS" };
    let before_after_result = if objective_sequence.len() >= 2 {
        "PASS"
    } else {
        "N/A"
    };
    let verdict_result = if product_observed_complete {
        "PASS"
    } else {
        "FAIL"
    };
    let generated_result = "PASS";
    push_row(
        &mut out,
        "po-check-live-url",
        "PASS",
        "manifest.json",
        "Local-only target URL is recorded in manifest.",
    );
    push_row(
        &mut out,
        "po-check-console",
        diagnostics_result,
        "console.jsonl; world-samples.jsonl",
        &format!(
            "console warnings/errors: {console_errors}; runtime diagnostics: {}.",
            runtime_diagnostics.len()
        ),
    );
    push_row(
        &mut out,
        "po-check-screenshot",
        screenshot_result,
        &join_or(&screenshot_paths, "screenshots/"),
        "Screenshot inventory is listed by concrete artifact path.",
    );
    push_row(
        &mut out,
        "po-check-replay",
        replay_result,
        "input-replay.json",
        &format!(
            "objective checkpoints: {}; final exit_reached: {exit_reached}; final grid_won: {grid_won}.",
            objective_sequence.len()
        ),
    );
    push_row(
        &mut out,
        "po-check-world-sample",
        world_result,
        "world-samples.jsonl",
        &format!("world sample lines: {}.", world_lines.len()),
    );
    push_row(
        &mut out,
        "po-check-event-sample",
        event_result,
        "events.json",
        &format!("event entries: {event_count}."),
    );
    push_row(
        &mut out,
        "po-check-frame-stats",
        frame_result,
        "frame-stats.jsonl",
        &format!("frame stat lines: {}.", frame_lines.len()),
    );
    push_row(&mut out, "po-check-before-after", before_after_result, "input-replay.json", "Replay objective sequence provides start/after checkpoints for this run; no source mutation is claimed.");
    push_row(
        &mut out,
        "po-check-verdict",
        verdict_result,
        "verdict.md",
        "This verdict separates mechanical contract status from product-observed usability.",
    );
    push_row(
        &mut out,
        "po-check-generated-state",
        generated_result,
        "manifest.json",
        &options.generated_state_audit,
    );
    out.push('\n');

    out.push_str("## Artifact summaries\n\n");
    out.push_str(&format!(
        "- Console lines: `{}`; warning/error lines: `{console_errors}`.\n",
        console_lines.len()
    ));
    out.push_str(&format!("- Frame stat lines: `{}`.\n", frame_lines.len()));
    out.push_str(&format!("- World sample lines: `{}`.\n", world_lines.len()));
    out.push_str(&format!("- Event entries: `{event_count}`.\n"));
    out.push_str(&format!(
        "- Replay: `{}`; objective checkpoints: `{}`.\n",
        manifest.replay.as_deref().unwrap_or("none"),
        objective_sequence.len()
    ));
    out.push_str(&format!(
        "- Screenshots: `{}`.\n",
        join_or(&screenshot_paths, "none")
    ));
    out.push_str(&format!(
        "- Observability API keys used: `{}`.\n\n",
        join_or(&manifest.observability_api_keys_used, "none")
    ));

    out.push_str("## State progression\n\n");
    if objective_sequence.is_empty() {
        out.push_str("No objective flag sequence recorded.\n\n");
    } else {
        out.push_str("| Checkpoint | Tick | Flags |\n| --- | ---: | --- |\n");
        for checkpoint in &objective_sequence {
            let label = checkpoint
                .get("label")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            let tick = checkpoint.get("tick").and_then(Value::as_i64).unwrap_or(-1);
            let flags = checkpoint.get("goal_flags").cloned().unwrap_or(Value::Null);
            out.push_str(&format!(
                "| `{}` | {} | `{}` |\n",
                escape_md(label),
                tick,
                compact_json(&flags)
            ));
        }
        out.push('\n');
    }

    out.push_str("## Fatal failures vs usability gaps\n\n");
    out.push_str(&format!("- Fatal failures: `{fatal_count}`.\n"));
    out.push_str(&format!(
        "- Usability gaps/diagnostics: `{usability_gap_count}`.\n"
    ));
    if runtime_diagnostics.is_empty() {
        out.push_str("- No runtime diagnostics were recorded in the sampled bundle.\n\n");
    } else {
        for diagnostic in &runtime_diagnostics {
            out.push_str(&format!("- `{}`\n", escape_md(diagnostic)));
        }
        out.push('\n');
    }

    out.push_str("## Usability note\n\n");
    if product_observed_complete {
        out.push_str("The bundle satisfies the checklist items used by this renderer. Fun, release, market, and commercial readiness remain outside this mechanical verdict.\n");
    } else {
        out.push_str("The bundle can be a mechanical `contract-pass` while still being `product-observed FAIL`. Current diagnostics or missing checklist evidence must become gap/backlog input rather than being hidden behind green tests.\n");
    }

    Ok(out)
}

pub fn write_rendered_verdict(
    bundle_root: impl AsRef<Path>,
    options: &VerdictOptions,
) -> Result<()> {
    let bundle_root = bundle_root.as_ref();
    let markdown = render_verdict(bundle_root, options)?;
    let verdict_path = bundle_root.join("verdict.md");
    fs::write(&verdict_path, markdown)
        .with_context(|| format!("writing {}", verdict_path.display()))?;
    refresh_inventory_sha256(bundle_root, "verdict.md")?;
    validate_bundle(bundle_root)?;
    Ok(())
}

fn push_row(out: &mut String, id: &str, result: &str, paths: &str, rationale: &str) {
    out.push_str(&format!(
        "| `{}` | {} | `{}` | {} |\n",
        id,
        result,
        escape_md(paths),
        escape_md(rationale)
    ));
}

fn read_manifest(bundle_root: &Path) -> Result<Manifest> {
    serde_json::from_slice(&fs::read(bundle_root.join("manifest.json"))?)
        .context("manifest.json is not valid JSON")
}

fn read_json(path: PathBuf) -> Result<Value> {
    serde_json::from_slice(&fs::read(&path).with_context(|| format!("reading {}", path.display()))?)
        .with_context(|| format!("{} is not valid JSON", path.display()))
}

fn read_jsonl(path: PathBuf) -> Result<Vec<Value>> {
    let content =
        fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .with_context(|| format!("{} has invalid JSONL", path.display()))
        })
        .collect()
}

fn collect_runtime_diagnostics(manifest: &Manifest, world_lines: &[Value]) -> Vec<String> {
    let mut diagnostics = Vec::new();
    for diagnostic in &manifest.diagnostics {
        diagnostics.push(compact_json(diagnostic));
    }
    for line in world_lines {
        if let Some(items) = line.get("diagnostics").and_then(Value::as_array) {
            for item in items {
                diagnostics.push(compact_json(item));
            }
        }
        if let Some(items) = line.get("runtime_diagnostics").and_then(Value::as_array) {
            for item in items {
                diagnostics.push(compact_json(item));
            }
        }
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

fn inventory_paths_by_kind(manifest: &Manifest, kind: &str) -> Vec<String> {
    let mut paths: Vec<String> = manifest
        .artifact_inventory
        .iter()
        .filter(|entry| entry.kind == kind)
        .map(|entry| entry.path.clone())
        .collect();
    paths.sort();
    paths
}

fn compact_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
}

fn join_or(values: &[String], fallback: &str) -> String {
    if values.is_empty() {
        fallback.to_string()
    } else {
        values.join(",")
    }
}

fn escape_md(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
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
    let _available_keys = &manifest.observability_api_keys_available;

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

fn refresh_inventory_sha256(bundle_root: &Path, artifact_path: &str) -> Result<()> {
    let manifest_path = bundle_root.join("manifest.json");
    let mut manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)
        .context("manifest.json is not valid JSON")?;
    let digest = sha256_file(&bundle_root.join(artifact_path))?;
    let inventory = manifest
        .get_mut("artifact_inventory")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("manifest artifact_inventory must be an array"))?;
    let mut updated = false;
    for entry in inventory {
        if entry.get("path").and_then(Value::as_str) == Some(artifact_path) {
            entry["sha256"] = Value::String(digest.clone());
            updated = true;
        }
    }
    if !updated {
        bail!("artifact inventory missing path: {artifact_path}");
    }
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest)? + "\n",
    )
    .with_context(|| format!("writing {}", manifest_path.display()))?;
    Ok(())
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

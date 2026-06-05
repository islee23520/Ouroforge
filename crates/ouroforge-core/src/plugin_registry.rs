//! Local plugin registry and discovery (#740).
//!
//! Discovers declarative plugin manifests from explicitly allowlisted local
//! directories and builds a deterministic, validated registry. Discovery is a
//! read-only filesystem scan with fail-closed path validation: it never executes
//! plugin code, never follows symlinks, never descends hidden directories, never
//! traverses outside the scan root, and never installs plugins from the network.
//!
//! The registry consumes the manifest schema (#739) for validation; deeper
//! version-compatibility resolution (#743) and the CLI surface (#752) build on
//! the read model exposed here.

use crate::plugin_manifest::{PluginManifest, SUPPORTED_MANIFEST_SCHEMA_VERSIONS};
use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Component, Path};

/// Directories that may contain local plugins, relative to the repository root.
/// `plugins/` is the canonical author directory; the manifest example tree is
/// fixture-scoped. No generated/evidence/dot directory is allowlisted.
pub const ALLOWED_PLUGIN_DIRS: &[&str] = &["plugins", "examples/plugin-discovery-v1"];

/// Generated/evidence roots that must never host a discovered plugin manifest.
const GENERATED_ROOTS: &[&str] = &["runs/", "evidence/", "dashboard-data/", ".omx/"];

/// Generated/evidence root directory names (without trailing slash) that must
/// never be used as, or contained in, a plugin discovery base. Mirrors
/// [`GENERATED_ROOTS`] for callers that classify a user-supplied scan directory.
pub const GENERATED_ROOT_NAMES: &[&str] = &["runs", "evidence", "dashboard-data", ".omx"];

/// True if `path` is, or lies within, a generated/evidence root that must never
/// seed plugin discovery. Used to reject user-supplied scan directories (e.g.
/// `ouroforge plugin validate evidence`) before discovery treats the generated
/// root as a clean base whose relative paths bypass the generated-root guard.
pub fn is_generated_discovery_root(path: impl AsRef<Path>) -> bool {
    use std::path::Component;
    path.as_ref().components().any(|component| {
        matches!(component, Component::Normal(name)
            if GENERATED_ROOT_NAMES
                .iter()
                .any(|root| name.eq_ignore_ascii_case(root)))
    })
}

/// Maximum directory depth the scan descends, as a runaway guard.
const MAX_SCAN_DEPTH: usize = 8;

const MANIFEST_SUFFIX: &str = ".plugin.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginRegistryStatus {
    Valid,
    Invalid,
    Blocked,
    Incompatible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginRegistryCompatibility {
    Compatible,
    Incompatible,
    Unknown,
    FutureVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginRegistryDescriptor {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginRegistryEntry {
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    #[serde(rename = "manifestPath")]
    pub manifest_path: String,
    #[serde(rename = "validationStatus")]
    pub validation_status: PluginRegistryStatus,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Vec<String>,
    #[serde(rename = "declaredCapabilities")]
    pub declared_capabilities: Vec<String>,
    #[serde(rename = "extensionPoints")]
    pub extension_points: Vec<String>,
    pub permissions: Vec<String>,
    #[serde(rename = "assetMetadataDescriptors")]
    pub asset_metadata_descriptors: Vec<String>,
    /// Contributed extension descriptors (kind + id), used for cross-plugin
    /// conflict detection (#751).
    pub descriptors: Vec<PluginRegistryDescriptor>,
    #[serde(rename = "compatibilityStatus")]
    pub compatibility_status: PluginRegistryCompatibility,
    #[serde(rename = "manifestHash")]
    pub manifest_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginRegistry {
    pub root: String,
    pub entries: Vec<PluginRegistryEntry>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginRegistryReadModel {
    pub root: String,
    #[serde(rename = "pluginCount")]
    pub plugin_count: usize,
    #[serde(rename = "validCount")]
    pub valid_count: usize,
    #[serde(rename = "invalidCount")]
    pub invalid_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "incompatibleCount")]
    pub incompatible_count: usize,
    #[serde(rename = "capabilitySummary")]
    pub capability_summary: Vec<String>,
    #[serde(rename = "extensionPointSummary")]
    pub extension_point_summary: Vec<String>,
    #[serde(rename = "permissionSummary")]
    pub permission_summary: Vec<String>,
    #[serde(rename = "assetMetadataSummary")]
    pub asset_metadata_summary: Vec<String>,
    pub boundary: String,
}

impl PluginRegistry {
    pub fn read_model(&self) -> PluginRegistryReadModel {
        let mut valid_count = 0;
        let mut invalid_count = 0;
        let mut blocked_count = 0;
        let mut incompatible_count = 0;
        let mut capability_summary = Vec::new();
        let mut extension_point_summary = Vec::new();
        let mut permission_summary = Vec::new();
        let mut asset_metadata_summary = Vec::new();
        for entry in &self.entries {
            match entry.validation_status {
                PluginRegistryStatus::Valid => valid_count += 1,
                PluginRegistryStatus::Invalid => invalid_count += 1,
                PluginRegistryStatus::Blocked => blocked_count += 1,
                PluginRegistryStatus::Incompatible => incompatible_count += 1,
            }
            capability_summary.extend(
                entry
                    .declared_capabilities
                    .iter()
                    .map(|capability| format!("{}:{capability}", entry.plugin_id)),
            );
            extension_point_summary.extend(
                entry
                    .extension_points
                    .iter()
                    .map(|point| format!("{}:{point}", entry.plugin_id)),
            );
            permission_summary.extend(
                entry
                    .permissions
                    .iter()
                    .map(|permission| format!("{}:{permission}", entry.plugin_id)),
            );
            asset_metadata_summary.extend(
                entry
                    .asset_metadata_descriptors
                    .iter()
                    .map(|descriptor| format!("{}:{descriptor}", entry.plugin_id)),
            );
        }
        capability_summary.sort();
        capability_summary.dedup();
        extension_point_summary.sort();
        extension_point_summary.dedup();
        permission_summary.sort();
        permission_summary.dedup();
        asset_metadata_summary.sort();
        asset_metadata_summary.dedup();
        PluginRegistryReadModel {
            root: self.root.clone(),
            plugin_count: self.entries.len(),
            valid_count,
            invalid_count,
            blocked_count,
            incompatible_count,
            capability_summary,
            extension_point_summary,
            permission_summary,
            asset_metadata_summary,
            boundary: "Read-only plugin registry summary; declarative discovery only, with no plugin execution, no command execution, no network install, and no trusted writes.".to_string(),
        }
    }
}

/// Discover plugins under every allowlisted directory relative to `repo_root`,
/// returning a deterministic registry. Missing directories are skipped. Paths
/// are reported relative to `repo_root`.
pub fn discover_plugin_registry(repo_root: impl AsRef<Path>) -> Result<PluginRegistry> {
    let repo_root = repo_root.as_ref();
    let mut entries = Vec::new();
    for dir in ALLOWED_PLUGIN_DIRS {
        let scan_dir = repo_root.join(dir);
        if !scan_dir.exists() {
            continue;
        }
        scan_directory(&scan_dir, repo_root, 0, &mut entries)?;
    }
    finalize(repo_root, entries)
}

/// Discover plugins under a single directory, with paths reported relative to
/// that directory. Used for fixture trees and tests.
pub fn discover_plugins_in_dir(root: impl AsRef<Path>) -> Result<PluginRegistry> {
    let root = root.as_ref();
    // A generated/evidence root used directly as the scan base would strip its own
    // name from every reported manifest path, defeating the generated-root
    // classification in `manifest_entry` (e.g. `plugin validate evidence` would
    // report `child/x.plugin.json` instead of `evidence/child/x.plugin.json`). Refuse
    // to discover plugins when the base is, or descends from, a generated root.
    if let Some(generated) = generated_root_in_path(root) {
        return Err(anyhow!(
            "plugin discovery refuses generated/evidence root `{generated}`: generated state never hosts discovered plugin manifests"
        ));
    }
    let mut entries = Vec::new();
    if root.exists() {
        scan_directory(root, root, 0, &mut entries)?;
    }
    finalize(root, entries)
}

/// Returns the generated/evidence root component if `dir` is, or repo-relatively
/// descends from, one of the [`GENERATED_ROOTS`]. Used to refuse plugin discovery over
/// generated state where manifests are never authored.
///
/// Two cases are classified as generated state:
/// - The scan base *is* a generated root (its final component matches), e.g. `evidence`
///   or `.../evidence`. Pointing discovery directly at generated state would strip the
///   root name from every reported manifest path.
/// - The scan base is a *relative* path whose first repo-relative component is a
///   generated root, e.g. `evidence/nested`. A relative base is interpreted relative to
///   the repository root, so a leading generated component means discovery is descending
///   into generated state.
///
/// Generated roots are defined relative to the repository root, so an unrelated
/// *absolute* ancestor such as `/tmp/evidence/work` must not disqualify a legitimate
/// plugins directory nested beneath it.
fn generated_root_in_path(dir: &Path) -> Option<&'static str> {
    fn match_generated(name: &std::ffi::OsStr) -> Option<&'static str> {
        let name = name.to_str()?;
        GENERATED_ROOTS
            .iter()
            .map(|root| root.trim_end_matches('/'))
            .find(|root| *root == name)
    }

    // The scan base is itself a generated root (e.g. `.../evidence`).
    if let Some(Component::Normal(name)) = dir.components().next_back() {
        if let Some(root) = match_generated(name) {
            return Some(root);
        }
    }

    // A relative scan base descends from a generated root (e.g. `evidence/nested`).
    // Absolute paths carry filesystem ancestors that are not repo-relative and are
    // intentionally not classified here.
    if dir.is_relative() {
        if let Some(name) = dir.components().find_map(|component| match component {
            Component::Normal(name) => Some(name),
            _ => None,
        }) {
            if let Some(root) = match_generated(name) {
                return Some(root);
            }
        }
    }

    None
}

fn finalize(root: &Path, mut entries: Vec<PluginRegistryEntry>) -> Result<PluginRegistry> {
    entries.sort_by(|left, right| left.manifest_path.cmp(&right.manifest_path));
    Ok(PluginRegistry {
        root: root.display().to_string(),
        entries,
        boundary: "Declarative read-only plugin registry; discovery never executes plugin code, follows symlinks, descends hidden directories, traverses outside the scan root, or installs plugins from the network.".to_string(),
    })
}

fn scan_directory(
    dir: &Path,
    base: &Path,
    depth: usize,
    entries: &mut Vec<PluginRegistryEntry>,
) -> Result<()> {
    if depth > MAX_SCAN_DEPTH {
        return Ok(());
    }
    // Fail closed on a symlinked discovery root: child-level symlinks are
    // rejected below, but the scan root itself is never followed, so a root
    // such as `plugins -> /tmp/outside` cannot leak external manifests.
    if depth == 0 {
        let root_metadata = fs::symlink_metadata(dir)
            .with_context(|| format!("failed to stat {}", dir.display()))?;
        if root_metadata.file_type().is_symlink() {
            return Ok(());
        }
    }
    let read_dir = fs::read_dir(dir)
        .with_context(|| format!("failed to read plugin directory {}", dir.display()))?;
    let mut children: Vec<_> = read_dir.collect::<std::io::Result<Vec<_>>>()?;
    children.sort_by_key(|entry| entry.file_name());
    for child in children {
        let path = child.path();
        let file_name = child.file_name();
        let name = file_name.to_string_lossy();
        // Fail closed on symlinks: never follow them, regardless of target.
        let metadata = fs::symlink_metadata(&path)
            .with_context(|| format!("failed to stat {}", path.display()))?;
        if metadata.file_type().is_symlink() {
            if name.ends_with(MANIFEST_SUFFIX) {
                entries.push(blocked_entry(
                    base,
                    &path,
                    "symlinked plugin manifests are not followed",
                ));
            }
            continue;
        }
        // Skip hidden directories and files (names beginning with a dot).
        if name.starts_with('.') {
            continue;
        }
        if metadata.is_dir() {
            scan_directory(&path, base, depth + 1, entries)?;
        } else if metadata.is_file() && name.ends_with(MANIFEST_SUFFIX) {
            entries.push(manifest_entry(base, &path)?);
        }
    }
    Ok(())
}

fn relative_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn fallback_plugin_id(base: &Path, path: &Path) -> String {
    path.file_stem()
        .map(|stem| stem.to_string_lossy().replace(".plugin", ""))
        .filter(|stem| !stem.is_empty())
        .unwrap_or_else(|| relative_path(base, path))
}

fn blocked_entry(base: &Path, path: &Path, reason: &str) -> PluginRegistryEntry {
    PluginRegistryEntry {
        plugin_id: fallback_plugin_id(base, path),
        manifest_path: relative_path(base, path),
        validation_status: PluginRegistryStatus::Blocked,
        validation_errors: vec![reason.to_string()],
        declared_capabilities: Vec::new(),
        extension_points: Vec::new(),
        permissions: Vec::new(),
        asset_metadata_descriptors: Vec::new(),
        descriptors: Vec::new(),
        compatibility_status: PluginRegistryCompatibility::Unknown,
        manifest_hash: String::new(),
    }
}

/// Collect contributed extension descriptors (kind + id) from a validated
/// manifest: descriptor references plus inline asset metadata descriptors.
fn descriptors_from_manifest(manifest: &PluginManifest) -> Vec<PluginRegistryDescriptor> {
    let mut descriptors: Vec<PluginRegistryDescriptor> = manifest
        .descriptor_refs
        .iter()
        .map(|reference| PluginRegistryDescriptor {
            kind: reference.kind.clone(),
            id: reference.id.clone(),
        })
        .collect();
    descriptors.extend(
        manifest
            .asset_metadata
            .iter()
            .map(|descriptor| PluginRegistryDescriptor {
                kind: "assetMetadataProvider".to_string(),
                id: descriptor.descriptor_id.clone(),
            }),
    );
    descriptors
}

fn manifest_entry(base: &Path, path: &Path) -> Result<PluginRegistryEntry> {
    let manifest_path = relative_path(base, path);
    // Defense in depth: reject any path that escaped into a generated root or
    // contains traversal, even though the scan never produces such paths.
    if manifest_path.contains("..")
        || GENERATED_ROOTS
            .iter()
            .any(|root| manifest_path.starts_with(root))
    {
        return Ok(blocked_entry(
            base,
            path,
            "plugin manifest path is outside the allowed plugin tree",
        ));
    }

    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read plugin manifest {}", path.display()))?;
    let value: Option<serde_json::Value> = serde_json::from_str(&contents).ok();
    let manifest_hash = match &value {
        Some(value) => format!(
            "fnv1a64-canonical-json-v1:{}",
            crate::canonical_json_digest(value.clone())?
        ),
        None => format!(
            "fnv1a64-canonical-json-v1:{:016x}",
            crate::fnv1a64(contents.as_bytes())
        ),
    };
    let declared_id = value
        .as_ref()
        .and_then(|value| value.get("pluginId"))
        .and_then(|id| id.as_str())
        .map(|id| id.to_string());
    let plugin_id = declared_id.unwrap_or_else(|| fallback_plugin_id(base, path));

    let schema_version = value
        .as_ref()
        .and_then(|value| value.get("schemaVersion"))
        .and_then(|version| version.as_str());

    if let Some(version) = schema_version {
        if !SUPPORTED_MANIFEST_SCHEMA_VERSIONS.contains(&version) {
            return Ok(PluginRegistryEntry {
                plugin_id,
                manifest_path,
                validation_status: PluginRegistryStatus::Incompatible,
                validation_errors: vec![format!(
                    "manifest declares unsupported schemaVersion `{version}`"
                )],
                declared_capabilities: Vec::new(),
                extension_points: Vec::new(),
                permissions: Vec::new(),
                asset_metadata_descriptors: Vec::new(),
                descriptors: Vec::new(),
                compatibility_status: PluginRegistryCompatibility::Incompatible,
                manifest_hash,
            });
        }
    }

    match PluginManifest::from_json_str(&contents) {
        Ok(manifest) => {
            // A structurally valid manifest may still target an incompatible or
            // future engine version (#743). Such plugins are reported but blocked
            // from extension contribution.
            let report = crate::plugin_compatibility::evaluate(
                &manifest.schema_version,
                &manifest.compatibility.min_ouroforge_version,
                &manifest.compatibility.max_ouroforge_version,
            );
            let (validation_status, compatibility_status) = match report.status {
                crate::plugin_compatibility::PluginCompatibilityStatus::Compatible => (
                    PluginRegistryStatus::Valid,
                    PluginRegistryCompatibility::Compatible,
                ),
                crate::plugin_compatibility::PluginCompatibilityStatus::Incompatible => (
                    PluginRegistryStatus::Incompatible,
                    PluginRegistryCompatibility::Incompatible,
                ),
                crate::plugin_compatibility::PluginCompatibilityStatus::FutureVersion => (
                    PluginRegistryStatus::Incompatible,
                    PluginRegistryCompatibility::FutureVersion,
                ),
                crate::plugin_compatibility::PluginCompatibilityStatus::Unknown => (
                    PluginRegistryStatus::Incompatible,
                    PluginRegistryCompatibility::Unknown,
                ),
            };
            let contributes = report.may_contribute();
            Ok(PluginRegistryEntry {
                plugin_id: manifest.plugin_id.clone(),
                manifest_path,
                validation_status,
                validation_errors: report.diagnostics,
                declared_capabilities: if contributes {
                    manifest.declared_capabilities.clone()
                } else {
                    Vec::new()
                },
                extension_points: if contributes {
                    manifest.extension_points.clone()
                } else {
                    Vec::new()
                },
                permissions: manifest.permissions.clone(),
                asset_metadata_descriptors: if contributes {
                    manifest
                        .asset_metadata
                        .iter()
                        .map(|descriptor| descriptor.descriptor_id.clone())
                        .collect()
                } else {
                    Vec::new()
                },
                descriptors: if contributes {
                    descriptors_from_manifest(&manifest)
                } else {
                    Vec::new()
                },
                compatibility_status,
                manifest_hash,
            })
        }
        Err(error) => Ok(PluginRegistryEntry {
            plugin_id,
            manifest_path,
            validation_status: PluginRegistryStatus::Invalid,
            validation_errors: vec![format!("{error:#}")],
            declared_capabilities: Vec::new(),
            extension_points: Vec::new(),
            permissions: Vec::new(),
            asset_metadata_descriptors: Vec::new(),
            descriptors: Vec::new(),
            compatibility_status: PluginRegistryCompatibility::Unknown,
            manifest_hash,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn discovery_fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-discovery-v1")
    }

    #[test]
    fn discovers_fixture_tree_with_all_states() {
        let registry =
            discover_plugins_in_dir(discovery_fixture_root()).expect("fixture discovery succeeds");
        let model = registry.read_model();
        assert!(model.valid_count >= 1, "expected a valid plugin");
        assert!(model.invalid_count >= 1, "expected an invalid plugin");
        assert!(
            model.incompatible_count >= 1,
            "expected an incompatible plugin"
        );
        // Deterministic ordering by manifest path.
        let mut sorted = registry
            .entries
            .iter()
            .map(|entry| entry.manifest_path.clone())
            .collect::<Vec<_>>();
        let original = sorted.clone();
        sorted.sort();
        assert_eq!(sorted, original);
        // Every valid entry exposes a canonical manifest hash.
        for entry in &registry.entries {
            if entry.validation_status == PluginRegistryStatus::Valid {
                assert!(entry
                    .manifest_hash
                    .starts_with("fnv1a64-canonical-json-v1:"));
                assert!(!entry.declared_capabilities.is_empty());
            }
        }
    }

    fn write_manifest(path: &Path, schema_version: &str, plugin_id: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let manifest = serde_json::json!({
            "schemaVersion": schema_version,
            "pluginId": plugin_id,
            "name": "Temp Plugin",
            "version": "1.0.0",
            "description": "A declarative temp plugin.",
            "metadata": { "author": "Author", "project": "Project" },
            "compatibility": { "minOuroforgeVersion": "0.1.0" },
            "declaredCapabilities": ["dashboardPanel"],
            "extensionPoints": ["dashboard.panels.readOnly"],
            "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
        });
        fs::write(path, serde_json::to_string_pretty(&manifest).unwrap()).unwrap();
    }

    #[test]
    fn skips_hidden_directories() {
        let temp = std::env::temp_dir().join(format!(
            "ouro-plugin-registry-hidden-{}",
            crate::fnv1a64(b"hidden-test")
        ));
        let _ = fs::remove_dir_all(&temp);
        write_manifest(
            &temp.join("plugins/visible/ouroforge.plugin.json"),
            "ouroforge.plugin-manifest.v1",
            "visible-plugin",
        );
        write_manifest(
            &temp.join("plugins/.hidden/ouroforge.plugin.json"),
            "ouroforge.plugin-manifest.v1",
            "hidden-plugin",
        );
        let registry = discover_plugins_in_dir(&temp).expect("discovery succeeds");
        let ids: Vec<_> = registry
            .entries
            .iter()
            .map(|entry| entry.plugin_id.as_str())
            .collect();
        assert!(ids.contains(&"visible-plugin"));
        assert!(
            !ids.contains(&"hidden-plugin"),
            "hidden directories must not be discovered"
        );
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    #[cfg(unix)]
    fn blocks_symlinked_manifests() {
        use std::os::unix::fs::symlink;
        let temp = std::env::temp_dir().join(format!(
            "ouro-plugin-registry-symlink-{}",
            crate::fnv1a64(b"symlink-test")
        ));
        let _ = fs::remove_dir_all(&temp);
        let real = temp.join("outside/real.plugin.json");
        write_manifest(&real, "ouroforge.plugin-manifest.v1", "outside-plugin");
        let link_dir = temp.join("plugins");
        fs::create_dir_all(&link_dir).unwrap();
        symlink(&real, link_dir.join("linked.plugin.json")).unwrap();
        let registry = discover_plugins_in_dir(&link_dir).expect("discovery succeeds");
        assert_eq!(registry.entries.len(), 1);
        assert_eq!(
            registry.entries[0].validation_status,
            PluginRegistryStatus::Blocked
        );
        assert!(registry.entries[0].validation_errors[0].contains("symlink"));
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    #[cfg(unix)]
    fn does_not_follow_symlinked_scan_root() {
        // The threat model claims discovery never follows symlinks, but a scan
        // root that is itself a symlink must not leak external manifests (#750).
        use std::os::unix::fs::symlink;
        let temp = std::env::temp_dir().join(format!(
            "ouro-plugin-registry-rootlink-{}",
            crate::fnv1a64(b"root-symlink-test")
        ));
        let _ = fs::remove_dir_all(&temp);
        let outside = temp.join("outside");
        write_manifest(
            &outside.join("evil.plugin.json"),
            "ouroforge.plugin-manifest.v1",
            "outside-plugin",
        );
        let link_root = temp.join("plugins");
        symlink(&outside, &link_root).unwrap();
        let registry = discover_plugins_in_dir(&link_root).expect("discovery succeeds");
        assert!(
            registry.entries.is_empty(),
            "a symlinked scan root must not be followed: {:?}",
            registry.entries
        );
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn flags_generated_discovery_roots() {
        assert!(is_generated_discovery_root("evidence"));
        assert!(is_generated_discovery_root("./runs"));
        assert!(is_generated_discovery_root("some/path/dashboard-data"));
        assert!(is_generated_discovery_root(".omx/sub"));
        assert!(!is_generated_discovery_root("plugins"));
        assert!(!is_generated_discovery_root("examples/plugin-discovery-v1"));
    }

    #[test]
    fn reports_invalid_and_incompatible_states() {
        let temp = std::env::temp_dir().join(format!(
            "ouro-plugin-registry-states-{}",
            crate::fnv1a64(b"states-test")
        ));
        let _ = fs::remove_dir_all(&temp);
        write_manifest(
            &temp.join("good/ouroforge.plugin.json"),
            "ouroforge.plugin-manifest.v1",
            "good-plugin",
        );
        // Incompatible: unsupported schema version.
        write_manifest(
            &temp.join("legacy/ouroforge.plugin.json"),
            "ouroforge.plugin-manifest.v2",
            "legacy-plugin",
        );
        // Invalid: well-formed JSON but fails manifest validation.
        fs::create_dir_all(temp.join("broken")).unwrap();
        fs::write(
            temp.join("broken/ouroforge.plugin.json"),
            serde_json::json!({
                "schemaVersion": "ouroforge.plugin-manifest.v1",
                "pluginId": "broken-plugin",
                "name": "Broken",
                "version": "1.0.0",
                "description": "Bad capability.",
                "metadata": { "author": "A", "project": "P" },
                "compatibility": { "minOuroforgeVersion": "0.1.0" },
                "declaredCapabilities": ["executeScript"],
                "extensionPoints": ["dashboard.panels.readOnly"],
                "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
            })
            .to_string(),
        )
        .unwrap();
        let registry = discover_plugins_in_dir(&temp).expect("discovery succeeds");
        let model = registry.read_model();
        assert_eq!(model.valid_count, 1);
        assert_eq!(model.invalid_count, 1);
        assert_eq!(model.incompatible_count, 1);
        assert_eq!(model.blocked_count, 0);
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn missing_directory_yields_empty_registry() {
        let registry = discover_plugins_in_dir(
            std::env::temp_dir().join("ouro-plugin-registry-does-not-exist-xyz"),
        )
        .expect("discovery succeeds");
        assert!(registry.entries.is_empty());
        assert_eq!(registry.read_model().plugin_count, 0);
    }
}

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const UNITY_2D_ADAPTER_IR_SCHEMA_VERSION: &str = "unity-2d-adapter-ir-v1";
pub const UNITY_2D_ADAPTER_BOUNDARY: &str = "one-way Unity Force-Text/.meta source-project import; clean-room re-derivation; no Unity runtime bridge";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UnitySourceEngine {
    Unity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UnityFidelityGrade {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityMigrationIr {
    pub schema_version: String,
    pub boundary: String,
    pub source: UnitySourceProjectRef,
    pub scenes: Vec<UnityIrScene>,
    pub prefabs: Vec<UnityIrPrefab>,
    pub assets: Vec<UnityIrAssetRef>,
    pub inputs: Vec<UnityIrInputAction>,
    pub logic_touchpoints: Vec<UnityIrLogicTouchpoint>,
    pub unsupported: Vec<UnityIrUnsupportedFeature>,
    pub fidelity_report: UnityFidelityReport,
    pub oracle_records: Vec<UnityOracleRecord>,
    pub claimed_ported_units: Vec<String>,
    pub state_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnitySourceProjectRef {
    pub engine: UnitySourceEngine,
    pub root_label: String,
    pub accepted_formats: Vec<String>,
    pub source_files: Vec<String>,
    pub rejected_binary_patterns: Vec<String>,
    pub contract_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityIrScene {
    pub id: String,
    pub source_path: String,
    pub nodes: Vec<UnityIrNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityIrPrefab {
    pub id: String,
    pub source_path: String,
    pub nodes: Vec<UnityIrNode>,
    pub prefab_overrides_flattened: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityIrNode {
    pub id: String,
    pub source_path: String,
    pub file_id: String,
    pub name: String,
    pub active: Option<bool>,
    pub layer: Option<i64>,
    pub tag: Option<String>,
    pub parent_id: Option<String>,
    pub order: usize,
    pub fidelity_grade: UnityFidelityGrade,
    pub transform2d: UnityTransform2d,
    pub presentation: Option<UnityPresentation>,
    pub camera: Option<UnityCamera2d>,
    pub collider: Option<UnityCollider2d>,
    pub components: Vec<UnityComponentRecord>,
    pub metadata: BTreeMap<String, String>,
    pub provenance: UnityProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityComponentRecord {
    pub id: String,
    pub file_id: String,
    pub owner_file_id: Option<String>,
    pub unity_type: String,
    pub support_status: String,
    pub fidelity_grade: UnityFidelityGrade,
    pub serialized_fields: BTreeMap<String, String>,
    pub asset_refs: Vec<UnityAssetLink>,
    pub provenance: UnityProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityAssetLink {
    pub guid: Option<String>,
    pub file_id: Option<String>,
    pub path: Option<String>,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityTransform2d {
    pub position: Option<[String; 3]>,
    pub rotation: Option<[String; 4]>,
    pub scale: Option<[String; 3]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum UnityPresentation {
    SpriteRenderer {
        sprite: UnityAssetLink,
        color: Option<String>,
        sorting_layer: Option<String>,
        sorting_order: Option<i64>,
    },
    Tilemap {
        tile_asset_refs: Vec<UnityAssetLink>,
    },
    UiGraphic {
        source_component: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityCamera2d {
    pub orthographic: bool,
    pub size: Option<String>,
    pub clear_flags: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityCollider2d {
    pub collider_type: String,
    pub is_trigger: Option<bool>,
    pub used_by_composite: Option<bool>,
    pub physics_re_simulated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityIrAssetRef {
    pub id: String,
    pub guid: String,
    pub source_path: String,
    pub importer_kind: Option<String>,
    pub fidelity_grade: UnityFidelityGrade,
    pub provenance: UnityProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityIrInputAction {
    pub name: String,
    pub bindings: Vec<String>,
    pub backend: String,
    pub fidelity_grade: UnityFidelityGrade,
    pub provenance: UnityProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityIrLogicTouchpoint {
    pub id: String,
    pub source_path: String,
    pub node_id: Option<String>,
    pub component_file_id: String,
    pub trigger_kind: String,
    pub symbol: Option<String>,
    pub coupling_kind: String,
    pub era_r_status: String,
    pub fidelity_grade: UnityFidelityGrade,
    pub provenance: UnityProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityIrUnsupportedFeature {
    pub source_path: String,
    pub node_id: Option<String>,
    pub feature_kind: String,
    pub severity: String,
    pub reason: String,
    pub suggested_hand_off: String,
    pub fidelity_grade: UnityFidelityGrade,
    pub provenance: UnityProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityOracleRecord {
    pub id: String,
    pub source_ref: String,
    pub status: String,
    pub ported_claim_allowed: bool,
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityProvenanceRef {
    pub source_path: String,
    pub line: usize,
    pub section: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityFidelityReport {
    pub summary: UnityFidelitySummary,
    pub records: Vec<UnityFidelityRecord>,
    pub oracle_rule: String,
    pub clean_room_notice: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityFidelitySummary {
    pub green: usize,
    pub yellow: usize,
    pub red: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityFidelityRecord {
    pub unit_id: String,
    pub grade: UnityFidelityGrade,
    pub reason: String,
    pub source_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityAdapterDemoReport {
    pub schema_version: String,
    pub source_engine: String,
    pub source_project: String,
    pub ir_state_hash: String,
    pub fidelity_summary: UnityFidelitySummary,
    pub unsupported_count: usize,
    pub logic_touchpoint_count: usize,
    pub oracle_record_count: usize,
    pub claimed_ported_units: Vec<String>,
    pub oracle_gate: String,
    pub determinism: String,
    pub provenance: UnityDemoProvenance,
    pub data_shapes: UnityDemoDataShapes,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityDemoProvenance {
    pub origin: String,
    pub clean_room_source_only: bool,
    pub decompiled_source_copied: bool,
    pub accepted_formats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnityDemoDataShapes {
    pub ir_nodes: String,
    pub mapping_records: String,
    pub behavioral_unit_records: String,
    pub oracle_records: String,
    pub no_elixir_artifact_semantics: bool,
}

#[derive(Debug, Clone)]
pub struct UnitySourceFile {
    pub path: String,
    pub contents: String,
}

#[derive(Debug, Clone)]
struct UnityYamlObject {
    unity_type: String,
    _class_id: String,
    file_id: String,
    path: String,
    line: usize,
    lines: Vec<String>,
}

pub fn parse_unity_2d_project(root: impl AsRef<Path>) -> Result<UnityMigrationIr> {
    let root = root.as_ref();
    let mut files = Vec::new();
    collect_unity_source_files(root, root, &mut files)?;
    parse_unity_2d_source_files(root.display().to_string(), files)
}

pub fn parse_unity_2d_source_files(
    root_label: impl Into<String>,
    mut files: Vec<UnitySourceFile>,
) -> Result<UnityMigrationIr> {
    files.sort_by(|a, b| a.path.cmp(&b.path));
    reject_forbidden_files(&files)?;

    let meta = collect_meta_guid_map(&files);
    let mut assets = collect_assets_from_meta(&meta);
    let mut scenes = Vec::new();
    let mut prefabs = Vec::new();
    let mut inputs = BTreeMap::new();
    let mut logic_touchpoints = BTreeMap::new();
    let mut unsupported = Vec::new();
    let mut fidelity_records = BTreeMap::new();

    for file in &files {
        if file.path.ends_with(".unity") || file.path.ends_with(".prefab") {
            let objects = parse_unity_yaml_objects(file)?;
            let parsed = parse_scene_like_file(
                file,
                &objects,
                &meta,
                &mut assets,
                &mut logic_touchpoints,
                &mut unsupported,
                &mut fidelity_records,
            );
            if file.path.ends_with(".unity") {
                scenes.push(UnityIrScene {
                    id: format!("unity-scene:{}", file.path),
                    source_path: file.path.clone(),
                    nodes: parsed.nodes,
                });
            } else {
                prefabs.push(UnityIrPrefab {
                    id: format!("unity-prefab:{}", file.path),
                    source_path: file.path.clone(),
                    nodes: parsed.nodes,
                    prefab_overrides_flattened: parsed.prefab_overrides_flattened,
                });
            }
        } else if file.path.ends_with(".asset") {
            if file.path.ends_with("ProjectSettings/InputManager.asset")
                || file.path.contains("InputManager")
            {
                parse_input_manager(file, &mut inputs, &mut fidelity_records);
            } else {
                record_asset_yaml(file, &mut fidelity_records);
            }
        }
    }

    let oracle_records: Vec<_> = logic_touchpoints
        .values()
        .map(|touchpoint| UnityOracleRecord {
            id: format!("unity-oracle:{}", touchpoint.id),
            source_ref: touchpoint.id.clone(),
            status: "missing".to_string(),
            ported_claim_allowed: false,
            required_evidence: vec![
                "captured acceptance oracle".to_string(),
                "Ouroforge-native clean-room re-expression".to_string(),
                "2d bit-exact deterministic state hash".to_string(),
            ],
        })
        .collect();

    let mut fidelity_records: Vec<_> = fidelity_records.into_values().collect();
    fidelity_records.sort_by(|a, b| a.unit_id.cmp(&b.unit_id));
    let summary = UnityFidelitySummary {
        green: fidelity_records
            .iter()
            .filter(|record| record.grade == UnityFidelityGrade::Green)
            .count(),
        yellow: fidelity_records
            .iter()
            .filter(|record| record.grade == UnityFidelityGrade::Yellow)
            .count(),
        red: fidelity_records
            .iter()
            .filter(|record| record.grade == UnityFidelityGrade::Red)
            .count(),
    };

    assets.sort_by(|a, b| a.guid.cmp(&b.guid).then(a.source_path.cmp(&b.source_path)));
    assets.dedup_by(|a, b| a.guid == b.guid && a.source_path == b.source_path);

    let mut ir = UnityMigrationIr {
        schema_version: UNITY_2D_ADAPTER_IR_SCHEMA_VERSION.to_string(),
        boundary: UNITY_2D_ADAPTER_BOUNDARY.to_string(),
        source: UnitySourceProjectRef {
            engine: UnitySourceEngine::Unity,
            root_label: root_label.into(),
            accepted_formats: vec![
                ".unity".to_string(),
                ".prefab".to_string(),
                ".asset".to_string(),
                ".meta".to_string(),
                "Unity Force Text YAML".to_string(),
            ],
            source_files: files.iter().map(|file| file.path.clone()).collect(),
            rejected_binary_patterns: rejected_binary_patterns(),
            contract_ref: "docs/unity-2d-adapter-ir-contract-v1.md".to_string(),
        },
        scenes,
        prefabs,
        assets,
        inputs: inputs.into_values().collect(),
        logic_touchpoints: logic_touchpoints.into_values().collect(),
        unsupported,
        fidelity_report: UnityFidelityReport {
            summary,
            records: fidelity_records,
            oracle_rule: "No imported Unity unit is ported/equivalent until a later Ouroforge-native oracle passes; 2D requires bit-exact deterministic state hashes.".to_string(),
            clean_room_notice: "MonoBehaviour, callback, animation event, input reaction, and physics behavior are Era R clean-room re-derivation touchpoints, never copied or translated source code.".to_string(),
        },
        oracle_records,
        claimed_ported_units: Vec::new(),
        state_hash: String::new(),
    };
    ir.state_hash = unity_ir_state_hash(&ir)?;
    validate_unity_2d_ir(&ir)?;
    Ok(ir)
}

pub fn unity_2d_adapter_demo_report(
    project_root: impl AsRef<Path>,
) -> Result<UnityAdapterDemoReport> {
    let project_root = project_root.as_ref();
    let ir = parse_unity_2d_project(project_root)?;
    let canonical =
        serde_json::to_vec(&ir).context("serializing Unity migration IR for deterministic hash")?;
    Ok(UnityAdapterDemoReport {
        schema_version: "unity-2d-adapter-demo-report-v1".to_string(),
        source_engine: "unity".to_string(),
        source_project: project_root.display().to_string(),
        ir_state_hash: crate::export_hash::sha256_prefixed(&canonical),
        fidelity_summary: ir.fidelity_report.summary.clone(),
        unsupported_count: ir.unsupported.len(),
        logic_touchpoint_count: ir.logic_touchpoints.len(),
        oracle_record_count: ir.oracle_records.len(),
        claimed_ported_units: Vec::new(),
        oracle_gate: "No Unity unit is claimed ported; later Ouroforge-native acceptance evidence and a passing oracle are required before equivalence wording is allowed.".to_string(),
        determinism: "The demo hashes canonical Unity migration IR bytes with sha256; repeated runs over the same Force-Text source project produce the same state hash.".to_string(),
        provenance: UnityDemoProvenance {
            origin: "unity".to_string(),
            clean_room_source_only: true,
            decompiled_source_copied: false,
            accepted_formats: ir.source.accepted_formats.clone(),
        },
        data_shapes: UnityDemoDataShapes {
            ir_nodes: "UnityIrNode records in crates/ouroforge-core/src/unity_2d_adapter_ir.rs"
                .to_string(),
            mapping_records: "UnityComponentRecord and UnityAssetLink records preserve resolved source references without creating a write path.".to_string(),
            behavioral_unit_records: "UnityIrLogicTouchpoint records are Era R clean-room re-derivation tasks, not translated source.".to_string(),
            oracle_records: "UnityOracleRecord entries are missing until captured acceptance evidence passes.".to_string(),
            no_elixir_artifact_semantics: true,
        },
        boundary: UNITY_2D_ADAPTER_BOUNDARY.to_string(),
    })
}

pub fn write_unity_2d_adapter_demo_report(
    project_root: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<UnityAdapterDemoReport> {
    let report = unity_2d_adapter_demo_report(project_root)?;
    validate_unity_2d_adapter_demo_report(&report)?;
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(output_path, serde_json::to_vec_pretty(&report)?)
        .with_context(|| format!("writing {}", output_path.display()))?;
    Ok(report)
}

pub fn validate_unity_2d_adapter_demo_report(report: &UnityAdapterDemoReport) -> Result<()> {
    if report.schema_version != "unity-2d-adapter-demo-report-v1" {
        return Err(anyhow!(
            "unsupported Unity adapter demo report schema {}",
            report.schema_version
        ));
    }
    if report.source_engine != "unity" || report.provenance.origin != "unity" {
        return Err(anyhow!("Unity adapter demo report source engine drifted"));
    }
    if !report.claimed_ported_units.is_empty() {
        return Err(anyhow!(
            "Unity adapter report cannot claim ported units without a later passing oracle"
        ));
    }
    if !report.ir_state_hash.starts_with("sha256:") || report.ir_state_hash.len() != 71 {
        return Err(anyhow!(
            "Unity adapter report requires a sha256-prefixed deterministic state hash"
        ));
    }
    if report.boundary != UNITY_2D_ADAPTER_BOUNDARY {
        return Err(anyhow!("Unity adapter report boundary drifted"));
    }
    if !report.provenance.clean_room_source_only || report.provenance.decompiled_source_copied {
        return Err(anyhow!(
            "Unity adapter report must remain clean-room source-project/open-text only"
        ));
    }
    if !report.data_shapes.no_elixir_artifact_semantics {
        return Err(anyhow!(
            "Unity adapter report cannot grant Elixir/Phoenix artifact semantics"
        ));
    }
    if !report
        .oracle_gate
        .to_ascii_lowercase()
        .contains("no unity unit is claimed ported")
    {
        return Err(anyhow!(
            "Unity adapter report must state that no Unity unit is claimed ported"
        ));
    }
    if (report.unsupported_count > 0 || report.logic_touchpoint_count > 0)
        && report.fidelity_summary.red == 0
    {
        return Err(anyhow!(
            "Unity adapter report must keep unsupported/logic gaps Red"
        ));
    }
    if report.logic_touchpoint_count > 0 && report.oracle_record_count == 0 {
        return Err(anyhow!(
            "Unity adapter report must expose oracle records for logic touchpoints"
        ));
    }
    Ok(())
}

pub fn validate_unity_2d_ir(ir: &UnityMigrationIr) -> Result<()> {
    if ir.schema_version != UNITY_2D_ADAPTER_IR_SCHEMA_VERSION {
        return Err(anyhow!(
            "unsupported Unity adapter IR schema {}",
            ir.schema_version
        ));
    }
    if ir.boundary != UNITY_2D_ADAPTER_BOUNDARY {
        return Err(anyhow!("Unity adapter boundary drifted"));
    }
    if !ir.claimed_ported_units.is_empty() {
        return Err(anyhow!(
            "Unity adapter IR cannot claim ported units without a later passing oracle"
        ));
    }
    if ir
        .oracle_records
        .iter()
        .any(|oracle| oracle.status != "passed" && oracle.ported_claim_allowed)
    {
        return Err(anyhow!(
            "oracle-missing Unity unit cannot allow a ported claim"
        ));
    }
    if (ir.fidelity_report.summary.red > 0 || !ir.logic_touchpoints.is_empty())
        && ir.oracle_records.is_empty()
    {
        return Err(anyhow!(
            "Unity logic or Red fidelity gaps require explicit oracle records"
        ));
    }
    if ir
        .unsupported
        .iter()
        .any(|feature| feature.fidelity_grade != UnityFidelityGrade::Red)
    {
        return Err(anyhow!("unsupported Unity features must remain Red"));
    }
    if !ir.state_hash.starts_with("sha256:") || ir.state_hash.len() != 71 {
        return Err(anyhow!(
            "Unity adapter IR requires a sha256-prefixed deterministic state hash"
        ));
    }
    let expected = unity_ir_state_hash(ir)?;
    if ir.state_hash != expected {
        return Err(anyhow!(
            "Unity adapter state hash does not match canonical IR"
        ));
    }
    Ok(())
}

fn unity_ir_state_hash(ir: &UnityMigrationIr) -> Result<String> {
    let mut canonical = ir.clone();
    canonical.state_hash.clear();
    Ok(crate::export_hash::sha256_prefixed(&serde_json::to_vec(
        &canonical,
    )?))
}

fn collect_unity_source_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<UnitySourceFile>,
) -> Result<()> {
    for entry in fs::read_dir(current).with_context(|| format!("reading {}", current.display()))? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            if matches!(
                name.as_str(),
                "library" | "temp" | "obj" | "build" | "builds" | "logs"
            ) {
                continue;
            }
            collect_unity_source_files(root, &path, files)?;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if is_unity_source_text_path(&relative) {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("reading Unity source text {}", path.display()))?;
            files.push(UnitySourceFile {
                path: relative,
                contents,
            });
        }
    }
    Ok(())
}

fn is_unity_source_text_path(path: &str) -> bool {
    path.ends_with(".unity")
        || path.ends_with(".prefab")
        || path.ends_with(".asset")
        || path.ends_with(".meta")
}

fn reject_forbidden_files(files: &[UnitySourceFile]) -> Result<()> {
    for file in files {
        let lower = file.path.to_ascii_lowercase();
        if rejected_binary_patterns()
            .iter()
            .any(|pattern| lower.contains(pattern))
            || lower.ends_with(".dll")
            || lower.ends_with(".exe")
            || lower.ends_with(".apk")
            || lower.ends_with(".ipa")
            || lower.ends_with(".bundle")
            || lower.ends_with(".assets")
        {
            return Err(anyhow!(
                "Unity adapter accepts source-project Force-Text/.meta only; rejected {}",
                file.path
            ));
        }
    }
    Ok(())
}

fn rejected_binary_patterns() -> Vec<String> {
    vec![
        "library/".to_string(),
        "temp/".to_string(),
        "obj/".to_string(),
        "build/".to_string(),
        "builds/".to_string(),
        "il2cpp".to_string(),
        "monobleedingedge".to_string(),
        "globalgamemanagers".to_string(),
        "resources.assets".to_string(),
        "assetbundle".to_string(),
        "decompiled".to_string(),
    ]
}

fn collect_meta_guid_map(files: &[UnitySourceFile]) -> BTreeMap<String, MetaRecord> {
    let mut map = BTreeMap::new();
    for file in files.iter().filter(|file| file.path.ends_with(".meta")) {
        let guid = file
            .contents
            .lines()
            .find_map(|line| line.trim().strip_prefix("guid:"))
            .map(str::trim)
            .filter(|guid| !guid.is_empty())
            .unwrap_or("missing-guid");
        let importer_kind = file
            .contents
            .lines()
            .find_map(|line| line.trim().strip_suffix(':'))
            .filter(|key| key.ends_with("Importer"))
            .map(str::to_string);
        let asset_path = file.path.trim_end_matches(".meta").to_string();
        map.insert(
            guid.to_string(),
            MetaRecord {
                guid: guid.to_string(),
                asset_path,
                meta_path: file.path.clone(),
                importer_kind,
            },
        );
    }
    map
}

#[derive(Debug, Clone)]
struct MetaRecord {
    guid: String,
    asset_path: String,
    meta_path: String,
    importer_kind: Option<String>,
}

fn collect_assets_from_meta(meta: &BTreeMap<String, MetaRecord>) -> Vec<UnityIrAssetRef> {
    meta.values()
        .map(|record| UnityIrAssetRef {
            id: format!("unity-asset:{}", record.guid),
            guid: record.guid.clone(),
            source_path: record.asset_path.clone(),
            importer_kind: record.importer_kind.clone(),
            fidelity_grade: UnityFidelityGrade::Green,
            provenance: UnityProvenanceRef {
                source_path: record.meta_path.clone(),
                line: 1,
                section: "meta-guid".to_string(),
            },
        })
        .collect()
}

fn parse_unity_yaml_objects(file: &UnitySourceFile) -> Result<Vec<UnityYamlObject>> {
    let mut objects = Vec::new();
    let mut current: Option<UnityYamlObject> = None;
    for (index, raw) in file.contents.lines().enumerate() {
        let line_no = index + 1;
        let line = raw.trim_end();
        if line.starts_with("--- !u!") {
            if let Some(object) = current.take() {
                objects.push(object);
            }
            let (class_id, file_id) = parse_unity_header(line).ok_or_else(|| {
                anyhow!(
                    "malformed Unity Force-Text object header in {}:{line_no}",
                    file.path
                )
            })?;
            current = Some(UnityYamlObject {
                unity_type: String::new(),
                _class_id: class_id,
                file_id,
                path: file.path.clone(),
                line: line_no,
                lines: Vec::new(),
            });
            continue;
        }
        if let Some(object) = current.as_mut() {
            let trimmed = line.trim();
            if object.unity_type.is_empty()
                && !trimmed.is_empty()
                && !trimmed.starts_with('%')
                && trimmed.ends_with(':')
            {
                object.unity_type = trimmed.trim_end_matches(':').to_string();
            } else if !trimmed.is_empty() && !trimmed.starts_with('%') {
                object.lines.push(trimmed.to_string());
            }
        }
    }
    if let Some(object) = current.take() {
        objects.push(object);
    }
    Ok(objects)
}

fn parse_unity_header(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix("--- !u!")?;
    let (class_id, file_id) = rest.split_once(" &")?;
    Some((class_id.trim().to_string(), file_id.trim().to_string()))
}

struct ParsedSceneLike {
    nodes: Vec<UnityIrNode>,
    prefab_overrides_flattened: bool,
}

fn parse_scene_like_file(
    file: &UnitySourceFile,
    objects: &[UnityYamlObject],
    meta: &BTreeMap<String, MetaRecord>,
    assets: &mut Vec<UnityIrAssetRef>,
    logic_touchpoints: &mut BTreeMap<String, UnityIrLogicTouchpoint>,
    unsupported: &mut Vec<UnityIrUnsupportedFeature>,
    fidelity_records: &mut BTreeMap<String, UnityFidelityRecord>,
) -> ParsedSceneLike {
    let by_file_id: BTreeMap<_, _> = objects
        .iter()
        .map(|object| (object.file_id.clone(), object))
        .collect();
    let component_owner: BTreeMap<String, String> = objects
        .iter()
        .filter_map(|object| owner_file_id(object).map(|owner| (object.file_id.clone(), owner)))
        .collect();
    let components_by_owner = components_by_owner(objects);
    let parent_by_owner = parent_by_game_object(objects, &component_owner);
    let prefab_overrides_flattened = objects.iter().any(|object| {
        object
            .lines
            .iter()
            .any(|line| line.contains("m_Modifications"))
    });

    let mut nodes = Vec::new();
    for (order, object) in objects
        .iter()
        .filter(|object| object.unity_type == "GameObject")
        .enumerate()
    {
        let id = unity_node_id(&file.path, &object.file_id);
        let component_ids = gameobject_component_ids(object);
        let mut components = Vec::new();
        for component_id in component_ids.iter().chain(
            components_by_owner
                .get(&object.file_id)
                .into_iter()
                .flatten(),
        ) {
            if let Some(component_object) = by_file_id.get(component_id) {
                let component = component_record(component_object, meta);
                record_component_fidelity(&component, fidelity_records);
                if component.fidelity_grade == UnityFidelityGrade::Red {
                    let touchpoint_id = format!("unity-logic:{}:{}", id, component.file_id);
                    logic_touchpoints.insert(
                        touchpoint_id.clone(),
                        UnityIrLogicTouchpoint {
                            id: touchpoint_id.clone(),
                            source_path: file.path.clone(),
                            node_id: Some(id.clone()),
                            component_file_id: component.file_id.clone(),
                            trigger_kind: trigger_kind_for(&component.unity_type).to_string(),
                            symbol: script_symbol(&component),
                            coupling_kind: coupling_kind_for(&component.unity_type).to_string(),
                            era_r_status: "requires-clean-room-re-derivation".to_string(),
                            fidelity_grade: UnityFidelityGrade::Red,
                            provenance: component.provenance.clone(),
                        },
                    );
                    fidelity_records.insert(
                        touchpoint_id.clone(),
                        UnityFidelityRecord {
                            unit_id: touchpoint_id,
                            grade: UnityFidelityGrade::Red,
                            reason: "Unity behavior-bearing component is inventoried only; logic must be clean-room re-derived in Era R.".to_string(),
                            source_path: file.path.clone(),
                        },
                    );
                }
                if component.support_status == "unsupported" {
                    unsupported.push(UnityIrUnsupportedFeature {
                        source_path: file.path.clone(),
                        node_id: Some(id.clone()),
                        feature_kind: component.unity_type.clone(),
                        severity: "red".to_string(),
                        reason: "Unity component is outside the bounded 2D Force-Text skeleton subset.".to_string(),
                        suggested_hand_off: "Record as an Era R re-derivation task or defer; do not translate or silently drop.".to_string(),
                        fidelity_grade: UnityFidelityGrade::Red,
                        provenance: component.provenance.clone(),
                    });
                }
                for link in &component.asset_refs {
                    if let (Some(guid), Some(path)) = (&link.guid, &link.path) {
                        assets.push(UnityIrAssetRef {
                            id: format!("unity-asset:{guid}"),
                            guid: guid.clone(),
                            source_path: path.clone(),
                            importer_kind: meta
                                .get(guid)
                                .and_then(|record| record.importer_kind.clone()),
                            fidelity_grade: if link.resolved {
                                UnityFidelityGrade::Green
                            } else {
                                UnityFidelityGrade::Yellow
                            },
                            provenance: component.provenance.clone(),
                        });
                    }
                }
                components.push(component);
            }
        }
        components.sort_by(|a, b| a.file_id.cmp(&b.file_id));
        let grade = node_grade(&components);
        let name = field_value(object, "m_Name")
            .unwrap_or_else(|| format!("GameObject-{}", object.file_id));
        let provenance = UnityProvenanceRef {
            source_path: file.path.clone(),
            line: object.line,
            section: format!("GameObject:{}", object.file_id),
        };
        fidelity_records.insert(
            format!("unity-node:{id}"),
            UnityFidelityRecord {
                unit_id: format!("unity-node:{id}"),
                grade: grade.clone(),
                reason: match grade {
                    UnityFidelityGrade::Green => "Unity declarative GameObject skeleton parsed with supported 2D components.".to_string(),
                    UnityFidelityGrade::Yellow => "Unity GameObject has partial skeleton semantics that require an honest fidelity caveat.".to_string(),
                    UnityFidelityGrade::Red => "Unity GameObject contains behavior or unsupported components and is not a clean import.".to_string(),
                },
                source_path: file.path.clone(),
            },
        );
        nodes.push(UnityIrNode {
            id: id.clone(),
            source_path: file.path.clone(),
            file_id: object.file_id.clone(),
            name,
            active: field_value(object, "m_IsActive").and_then(|value| parse_unity_bool(&value)),
            layer: field_value(object, "m_Layer").and_then(|value| value.parse::<i64>().ok()),
            tag: field_value(object, "m_TagString"),
            parent_id: parent_by_owner
                .get(&object.file_id)
                .map(|parent| unity_node_id(&file.path, parent)),
            order,
            fidelity_grade: grade,
            transform2d: transform_for(&components),
            presentation: presentation_for(&components),
            camera: camera_for(&components),
            collider: collider_for(&components),
            components,
            metadata: BTreeMap::new(),
            provenance,
        });
    }
    nodes.sort_by(|a, b| a.order.cmp(&b.order).then(a.file_id.cmp(&b.file_id)));
    if prefab_overrides_flattened {
        fidelity_records.insert(
            format!("unity-prefab-overrides:{}", file.path),
            UnityFidelityRecord {
                unit_id: format!("unity-prefab-overrides:{}", file.path),
                grade: UnityFidelityGrade::Yellow,
                reason: "Prefab overrides were flattened as source-text skeleton deltas; nested behavior semantics remain gated.".to_string(),
                source_path: file.path.clone(),
            },
        );
    }
    ParsedSceneLike {
        nodes,
        prefab_overrides_flattened,
    }
}

fn gameobject_component_ids(object: &UnityYamlObject) -> Vec<String> {
    object
        .lines
        .iter()
        .filter(|line| line.contains("component:") && line.contains("fileID:"))
        .filter_map(|line| extract_braced_value(line, "fileID"))
        .collect()
}

fn components_by_owner(objects: &[UnityYamlObject]) -> BTreeMap<String, Vec<String>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for object in objects
        .iter()
        .filter(|object| object.unity_type != "GameObject")
    {
        if let Some(owner) = owner_file_id(object) {
            map.entry(owner).or_default().push(object.file_id.clone());
        }
    }
    map
}

fn owner_file_id(object: &UnityYamlObject) -> Option<String> {
    object
        .lines
        .iter()
        .find(|line| line.starts_with("m_GameObject:"))
        .and_then(|line| extract_braced_value(line, "fileID"))
}

fn parent_by_game_object(
    objects: &[UnityYamlObject],
    component_owner: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for object in objects
        .iter()
        .filter(|object| object.unity_type == "Transform" || object.unity_type == "RectTransform")
    {
        let Some(owner) = component_owner.get(&object.file_id) else {
            continue;
        };
        let Some(parent_transform) = object
            .lines
            .iter()
            .find(|line| line.starts_with("m_Father:"))
            .and_then(|line| extract_braced_value(line, "fileID"))
        else {
            continue;
        };
        if parent_transform == "0" {
            continue;
        }
        if let Some(parent_owner) = component_owner.get(&parent_transform) {
            map.insert(owner.clone(), parent_owner.clone());
        }
    }
    map
}

fn component_record(
    object: &UnityYamlObject,
    meta: &BTreeMap<String, MetaRecord>,
) -> UnityComponentRecord {
    let fields = serialized_fields(object);
    let asset_refs = asset_refs_for(object, meta);
    let (support_status, fidelity_grade) = component_support(&object.unity_type);
    UnityComponentRecord {
        id: format!("unity-component:{}:{}", object.path, object.file_id),
        file_id: object.file_id.clone(),
        owner_file_id: owner_file_id(object),
        unity_type: object.unity_type.clone(),
        support_status: support_status.to_string(),
        fidelity_grade,
        serialized_fields: fields,
        asset_refs,
        provenance: UnityProvenanceRef {
            source_path: object.path.clone(),
            line: object.line,
            section: format!("{}:{}", object.unity_type, object.file_id),
        },
    }
}

fn serialized_fields(object: &UnityYamlObject) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in &object.lines {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            if key.starts_with("m_") {
                map.insert(key.to_string(), value.trim().to_string());
            }
        }
    }
    map
}

fn asset_refs_for(
    object: &UnityYamlObject,
    meta: &BTreeMap<String, MetaRecord>,
) -> Vec<UnityAssetLink> {
    let mut refs = Vec::new();
    for line in &object.lines {
        if !line.contains("guid:") {
            continue;
        }
        let guid = extract_braced_value(line, "guid");
        let file_id = extract_braced_value(line, "fileID");
        let path = guid
            .as_ref()
            .and_then(|guid| meta.get(guid))
            .map(|record| record.asset_path.clone());
        refs.push(UnityAssetLink {
            resolved: path.is_some(),
            guid,
            file_id,
            path,
        });
    }
    refs.sort_by(|a, b| a.guid.cmp(&b.guid).then(a.file_id.cmp(&b.file_id)));
    refs.dedup();
    refs
}

fn component_support(unity_type: &str) -> (&'static str, UnityFidelityGrade) {
    match unity_type {
        "Transform" | "RectTransform" | "SpriteRenderer" | "Camera" | "GameObject" => {
            ("supported", UnityFidelityGrade::Green)
        }
        "Rigidbody2D" | "BoxCollider2D" | "CircleCollider2D" | "PolygonCollider2D"
        | "EdgeCollider2D" | "Tilemap" | "Grid" | "Animator" | "AudioSource" | "Canvas"
        | "CanvasRenderer" | "Text" | "Image" => ("partial", UnityFidelityGrade::Yellow),
        "MonoBehaviour" => ("behavior-touchpoint", UnityFidelityGrade::Red),
        _ => ("unsupported", UnityFidelityGrade::Red),
    }
}

fn record_component_fidelity(
    component: &UnityComponentRecord,
    fidelity_records: &mut BTreeMap<String, UnityFidelityRecord>,
) {
    fidelity_records.insert(
        component.id.clone(),
        UnityFidelityRecord {
            unit_id: component.id.clone(),
            grade: component.fidelity_grade.clone(),
            reason: match component.fidelity_grade {
                UnityFidelityGrade::Green => {
                    "Supported Unity declarative 2D component parsed into neutral IR.".to_string()
                }
                UnityFidelityGrade::Yellow => {
                    "Unity component has partial/best-effort skeleton semantics; caveat remains explicit.".to_string()
                }
                UnityFidelityGrade::Red => {
                    "Unity component is behavior-bearing or unsupported and must be clean-room re-derived/deferred.".to_string()
                }
            },
            source_path: component.provenance.source_path.clone(),
        },
    );
}

fn node_grade(components: &[UnityComponentRecord]) -> UnityFidelityGrade {
    if components
        .iter()
        .any(|component| component.fidelity_grade == UnityFidelityGrade::Red)
    {
        UnityFidelityGrade::Red
    } else if components
        .iter()
        .any(|component| component.fidelity_grade == UnityFidelityGrade::Yellow)
    {
        UnityFidelityGrade::Yellow
    } else {
        UnityFidelityGrade::Green
    }
}

fn transform_for(components: &[UnityComponentRecord]) -> UnityTransform2d {
    let transform = components.iter().find(|component| {
        component.unity_type == "Transform" || component.unity_type == "RectTransform"
    });
    UnityTransform2d {
        position: transform
            .and_then(|component| component.serialized_fields.get("m_LocalPosition"))
            .and_then(|value| parse_xyz(value)),
        rotation: transform
            .and_then(|component| component.serialized_fields.get("m_LocalRotation"))
            .and_then(|value| parse_xyzw(value)),
        scale: transform
            .and_then(|component| component.serialized_fields.get("m_LocalScale"))
            .and_then(|value| parse_xyz(value)),
    }
}

fn presentation_for(components: &[UnityComponentRecord]) -> Option<UnityPresentation> {
    let sprite = components
        .iter()
        .find(|component| component.unity_type == "SpriteRenderer")?;
    let sprite_ref = sprite
        .asset_refs
        .first()
        .cloned()
        .unwrap_or(UnityAssetLink {
            guid: None,
            file_id: None,
            path: None,
            resolved: false,
        });
    Some(UnityPresentation::SpriteRenderer {
        sprite: sprite_ref,
        color: sprite.serialized_fields.get("m_Color").cloned(),
        sorting_layer: sprite
            .serialized_fields
            .get("m_SortingLayerID")
            .or_else(|| sprite.serialized_fields.get("m_SortingLayer"))
            .cloned(),
        sorting_order: sprite
            .serialized_fields
            .get("m_SortingOrder")
            .and_then(|value| value.parse::<i64>().ok()),
    })
}

fn camera_for(components: &[UnityComponentRecord]) -> Option<UnityCamera2d> {
    let camera = components
        .iter()
        .find(|component| component.unity_type == "Camera")?;
    Some(UnityCamera2d {
        orthographic: camera
            .serialized_fields
            .get("orthographic")
            .and_then(|value| parse_unity_bool(value))
            .or_else(|| {
                camera
                    .serialized_fields
                    .get("m_Orthographic")
                    .and_then(|value| parse_unity_bool(value))
            })
            .unwrap_or(true),
        size: camera
            .serialized_fields
            .get("orthographic size")
            .or_else(|| camera.serialized_fields.get("m_OrthographicSize"))
            .cloned(),
        clear_flags: camera.serialized_fields.get("m_ClearFlags").cloned(),
    })
}

fn collider_for(components: &[UnityComponentRecord]) -> Option<UnityCollider2d> {
    let collider = components.iter().find(|component| {
        matches!(
            component.unity_type.as_str(),
            "BoxCollider2D" | "CircleCollider2D" | "PolygonCollider2D" | "EdgeCollider2D"
        )
    })?;
    Some(UnityCollider2d {
        collider_type: collider.unity_type.clone(),
        is_trigger: collider
            .serialized_fields
            .get("m_IsTrigger")
            .and_then(|value| parse_unity_bool(value)),
        used_by_composite: collider
            .serialized_fields
            .get("m_UsedByComposite")
            .and_then(|value| parse_unity_bool(value)),
        physics_re_simulated: true,
    })
}

fn parse_input_manager(
    file: &UnitySourceFile,
    inputs: &mut BTreeMap<String, UnityIrInputAction>,
    fidelity_records: &mut BTreeMap<String, UnityFidelityRecord>,
) {
    let mut current_name: Option<(String, usize)> = None;
    let mut bindings = BTreeSet::new();
    for (index, raw) in file.contents.lines().enumerate() {
        let line = raw.trim();
        if let Some(name) = line
            .strip_prefix("m_Name:")
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            if let Some((name, line_no)) = current_name.take() {
                insert_input(file, inputs, fidelity_records, name, line_no, &bindings);
                bindings.clear();
            }
            current_name = Some((name.to_string(), index + 1));
        } else if let Some(binding) = line
            .strip_prefix("positiveButton:")
            .or_else(|| line.strip_prefix("negativeButton:"))
            .or_else(|| line.strip_prefix("altPositiveButton:"))
            .or_else(|| line.strip_prefix("altNegativeButton:"))
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            bindings.insert(binding.to_string());
        }
    }
    if let Some((name, line_no)) = current_name.take() {
        insert_input(file, inputs, fidelity_records, name, line_no, &bindings);
    }
}

fn insert_input(
    file: &UnitySourceFile,
    inputs: &mut BTreeMap<String, UnityIrInputAction>,
    fidelity_records: &mut BTreeMap<String, UnityFidelityRecord>,
    name: String,
    line: usize,
    bindings: &BTreeSet<String>,
) {
    let action = UnityIrInputAction {
        name: name.clone(),
        bindings: bindings.iter().cloned().collect(),
        backend: "legacy-input-manager".to_string(),
        fidelity_grade: UnityFidelityGrade::Green,
        provenance: UnityProvenanceRef {
            source_path: file.path.clone(),
            line,
            section: "InputManager".to_string(),
        },
    };
    fidelity_records.insert(
        format!("unity-input:{name}"),
        UnityFidelityRecord {
            unit_id: format!("unity-input:{name}"),
            grade: UnityFidelityGrade::Green,
            reason: "Unity legacy InputManager binding is source text and declarative.".to_string(),
            source_path: file.path.clone(),
        },
    );
    inputs.insert(name, action);
}

fn record_asset_yaml(
    file: &UnitySourceFile,
    fidelity_records: &mut BTreeMap<String, UnityFidelityRecord>,
) {
    fidelity_records.insert(
        format!("unity-asset-yaml:{}", file.path),
        UnityFidelityRecord {
            unit_id: format!("unity-asset-yaml:{}", file.path),
            grade: UnityFidelityGrade::Yellow,
            reason: "Unity .asset source text is inventoried as declarative data; custom behavior implied by ScriptableObjects remains gated.".to_string(),
            source_path: file.path.clone(),
        },
    );
}

fn field_value(object: &UnityYamlObject, key: &str) -> Option<String> {
    object.lines.iter().find_map(|line| {
        line.strip_prefix(key)
            .and_then(|rest| rest.strip_prefix(':'))
            .map(str::trim)
            .map(str::to_string)
    })
}

fn extract_braced_value(line: &str, key: &str) -> Option<String> {
    let needle = format!("{key}:");
    let start = line.find(&needle)? + needle.len();
    let rest = &line[start..];
    let value = rest
        .split([',', '}'])
        .next()
        .unwrap_or_default()
        .trim()
        .trim_matches('"');
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn parse_xyz(value: &str) -> Option<[String; 3]> {
    Some([
        extract_braced_value(value, "x")?,
        extract_braced_value(value, "y")?,
        extract_braced_value(value, "z")?,
    ])
}

fn parse_xyzw(value: &str) -> Option<[String; 4]> {
    Some([
        extract_braced_value(value, "x")?,
        extract_braced_value(value, "y")?,
        extract_braced_value(value, "z")?,
        extract_braced_value(value, "w")?,
    ])
}

fn parse_unity_bool(value: &str) -> Option<bool> {
    match value.trim() {
        "1" | "true" | "True" => Some(true),
        "0" | "false" | "False" => Some(false),
        _ => None,
    }
}

fn unity_node_id(path: &str, file_id: &str) -> String {
    format!("unity-node:{path}:{file_id}")
}

fn trigger_kind_for(unity_type: &str) -> &str {
    match unity_type {
        "MonoBehaviour" => "script-ref",
        "Animator" => "animation-state-ref",
        _ => "unsupported-component",
    }
}

fn coupling_kind_for(unity_type: &str) -> &str {
    match unity_type {
        "MonoBehaviour" => "script",
        "Animator" => "animation",
        "Rigidbody2D" | "BoxCollider2D" | "CircleCollider2D" | "PolygonCollider2D"
        | "EdgeCollider2D" => "physics",
        _ => "unknown",
    }
}

fn script_symbol(component: &UnityComponentRecord) -> Option<String> {
    component
        .asset_refs
        .first()
        .and_then(|link| link.guid.clone().or_else(|| link.file_id.clone()))
        .or_else(|| component.serialized_fields.get("m_Script").cloned())
}

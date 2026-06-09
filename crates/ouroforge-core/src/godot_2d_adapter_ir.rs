use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const GODOT_2D_ADAPTER_IR_SCHEMA_VERSION: &str = "godot-2d-adapter-ir-v1";
pub const GODOT_2D_ADAPTER_BOUNDARY: &str =
    "one-way source-project text import; clean-room re-derivation; no Godot runtime bridge";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MigrationSourceEngine {
    Godot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FidelityGrade {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GodotMigrationIr {
    pub schema_version: String,
    pub boundary: String,
    pub source: GodotSourceProjectRef,
    pub scenes: Vec<GodotIrScene>,
    pub assets: Vec<GodotIrAssetRef>,
    pub inputs: Vec<GodotIrInputAction>,
    pub logic_touchpoints: Vec<GodotIrLogicTouchpoint>,
    pub unsupported: Vec<GodotIrUnsupportedFeature>,
    pub fidelity_report: GodotFidelityReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotSourceProjectRef {
    pub engine: MigrationSourceEngine,
    pub root_label: String,
    pub accepted_formats: Vec<String>,
    pub source_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GodotIrScene {
    pub id: String,
    pub source_path: String,
    pub nodes: Vec<GodotIrNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GodotIrNode {
    pub id: String,
    pub source_path: String,
    pub name: String,
    pub godot_type: String,
    pub parent_id: Option<String>,
    pub order: usize,
    pub fidelity_grade: FidelityGrade,
    pub transform2d: GodotTransform2d,
    pub visibility: Option<bool>,
    pub presentation: Option<GodotPresentation>,
    pub camera: Option<GodotCamera2d>,
    pub collider: Option<GodotCollider2d>,
    pub metadata: BTreeMap<String, String>,
    pub provenance: GodotProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GodotTransform2d {
    pub position: Option<[f64; 2]>,
    pub rotation: Option<f64>,
    pub scale: Option<[f64; 2]>,
    pub z_index: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum GodotPresentation {
    Sprite {
        texture_ref: Option<String>,
        region: Option<GodotRect2>,
        modulate: Option<String>,
    },
    Tilemap {
        tile_set_ref: Option<String>,
        cell_count_hint: usize,
    },
    Label {
        text: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GodotCamera2d {
    pub enabled: bool,
    pub zoom: Option<[f64; 2]>,
    pub metadata_only_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GodotCollider2d {
    pub shape_ref: Option<String>,
    pub sensor: bool,
    pub collision_layer: Option<i64>,
    pub collision_mask: Option<i64>,
    pub physics_re_simulated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GodotRect2 {
    pub origin: [f64; 2],
    pub size: [f64; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotIrAssetRef {
    pub id: String,
    pub source_path: String,
    pub kind: String,
    pub fidelity_grade: FidelityGrade,
    pub provenance: GodotProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotIrInputAction {
    pub name: String,
    pub bindings: Vec<String>,
    pub fidelity_grade: FidelityGrade,
    pub provenance: GodotProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotIrLogicTouchpoint {
    pub id: String,
    pub source_path: String,
    pub node_id: Option<String>,
    pub trigger_kind: String,
    pub symbol: Option<String>,
    pub exported_variables: Vec<String>,
    pub era_r_status: String,
    pub fidelity_grade: FidelityGrade,
    pub provenance: GodotProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotIrUnsupportedFeature {
    pub source_path: String,
    pub node_id: Option<String>,
    pub feature_kind: String,
    pub severity: String,
    pub reason: String,
    pub suggested_hand_off: String,
    pub fidelity_grade: FidelityGrade,
    pub provenance: GodotProvenanceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotProvenanceRef {
    pub source_path: String,
    pub line: usize,
    pub section: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotFidelityReport {
    pub summary: GodotFidelitySummary,
    pub records: Vec<GodotFidelityRecord>,
    pub oracle_rule: String,
    pub clean_room_notice: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotFidelitySummary {
    pub green: usize,
    pub yellow: usize,
    pub red: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotFidelityRecord {
    pub unit_id: String,
    pub grade: FidelityGrade,
    pub reason: String,
    pub source_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GodotAdapterDemoReport {
    pub schema_version: String,
    pub source_project: String,
    pub ir_state_hash: String,
    pub fidelity_summary: GodotFidelitySummary,
    pub unsupported_count: usize,
    pub logic_touchpoint_count: usize,
    pub claimed_ported_units: Vec<String>,
    pub oracle_gate: String,
    pub determinism: String,
    pub boundary: String,
}

#[derive(Debug, Clone)]
pub struct GodotSourceFile {
    pub path: String,
    pub contents: String,
}

#[derive(Debug, Clone)]
struct Section {
    kind: String,
    attrs: BTreeMap<String, String>,
    props: BTreeMap<String, String>,
    line: usize,
}

pub fn godot_2d_adapter_demo_report(
    project_root: impl AsRef<Path>,
) -> Result<GodotAdapterDemoReport> {
    let project_root = project_root.as_ref();
    let ir = parse_godot_2d_project(project_root)?;
    let canonical =
        serde_json::to_vec(&ir).context("serializing Godot migration IR for deterministic hash")?;
    Ok(GodotAdapterDemoReport {
        schema_version: "godot-2d-adapter-demo-report-v1".to_string(),
        source_project: project_root.display().to_string(),
        ir_state_hash: crate::export_hash::sha256_prefixed(&canonical),
        fidelity_summary: ir.fidelity_report.summary.clone(),
        unsupported_count: ir.unsupported.len(),
        logic_touchpoint_count: ir.logic_touchpoints.len(),
        claimed_ported_units: Vec::new(),
        oracle_gate: "No unit is claimed ported; later Ouroforge-native acceptance evidence must pass before equivalence wording is allowed.".to_string(),
        determinism: "The demo hashes canonical Godot migration IR bytes with sha256; repeated runs over the same source text produce the same state hash.".to_string(),
        boundary: GODOT_2D_ADAPTER_BOUNDARY.to_string(),
    })
}

pub fn write_godot_2d_adapter_demo_report(
    project_root: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<GodotAdapterDemoReport> {
    let report = godot_2d_adapter_demo_report(project_root)?;
    validate_godot_2d_adapter_demo_report(&report)?;
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(output_path, serde_json::to_vec_pretty(&report)?)
        .with_context(|| format!("writing {}", output_path.display()))?;
    Ok(report)
}

pub fn validate_godot_2d_adapter_demo_report(report: &GodotAdapterDemoReport) -> Result<()> {
    if report.schema_version != "godot-2d-adapter-demo-report-v1" {
        return Err(anyhow!(
            "unsupported Godot adapter demo report schema {}",
            report.schema_version
        ));
    }
    if !report.claimed_ported_units.is_empty() {
        return Err(anyhow!(
            "Godot adapter report cannot claim ported units without a later passing oracle"
        ));
    }
    if !report.ir_state_hash.starts_with("sha256:") || report.ir_state_hash.len() != 71 {
        return Err(anyhow!(
            "Godot adapter report requires a sha256-prefixed deterministic state hash"
        ));
    }
    if report.boundary != GODOT_2D_ADAPTER_BOUNDARY {
        return Err(anyhow!("Godot adapter report boundary drifted"));
    }
    if !report
        .oracle_gate
        .to_ascii_lowercase()
        .contains("no unit is claimed ported")
    {
        return Err(anyhow!(
            "Godot adapter report must state that no unit is claimed ported"
        ));
    }
    if (report.unsupported_count > 0 || report.logic_touchpoint_count > 0)
        && report.fidelity_summary.red == 0
    {
        return Err(anyhow!(
            "Godot adapter report cannot grade lossy or logic-touchpoint imports clean"
        ));
    }
    Ok(())
}

pub fn parse_godot_2d_project(root: impl AsRef<Path>) -> Result<GodotMigrationIr> {
    let root = root.as_ref();
    let mut files = Vec::new();
    collect_godot_source_files(root, root, &mut files)?;
    parse_godot_2d_source_files(root.display().to_string(), files)
}

pub fn parse_godot_2d_source_files(
    root_label: impl Into<String>,
    mut files: Vec<GodotSourceFile>,
) -> Result<GodotMigrationIr> {
    files.sort_by(|a, b| a.path.cmp(&b.path));
    reject_forbidden_files(&files)?;

    let mut scenes = Vec::new();
    let mut asset_map: BTreeMap<String, GodotIrAssetRef> = BTreeMap::new();
    let mut inputs = BTreeMap::new();
    let mut logic_touchpoints = BTreeMap::new();
    let mut unsupported = Vec::new();
    let mut fidelity_records = BTreeMap::new();

    for file in &files {
        if file.path.ends_with(".tscn") || file.path.ends_with(".tres") {
            let sections = parse_sections(file)?;
            collect_external_assets(file, &sections, &mut asset_map, &mut fidelity_records);
            if file.path.ends_with(".tscn") {
                let scene = parse_scene_nodes(
                    file,
                    &sections,
                    &mut logic_touchpoints,
                    &mut unsupported,
                    &mut fidelity_records,
                );
                scenes.push(scene);
            }
        } else if file.path.ends_with("project.godot") {
            parse_project_inputs(file, &mut inputs, &mut fidelity_records)?;
        }
    }

    let mut fidelity_records: Vec<_> = fidelity_records.into_values().collect();
    fidelity_records.sort_by(|a, b| a.unit_id.cmp(&b.unit_id));
    let summary = GodotFidelitySummary {
        green: fidelity_records
            .iter()
            .filter(|record| record.grade == FidelityGrade::Green)
            .count(),
        yellow: fidelity_records
            .iter()
            .filter(|record| record.grade == FidelityGrade::Yellow)
            .count(),
        red: fidelity_records
            .iter()
            .filter(|record| record.grade == FidelityGrade::Red)
            .count(),
    };

    Ok(GodotMigrationIr {
        schema_version: GODOT_2D_ADAPTER_IR_SCHEMA_VERSION.to_string(),
        boundary: GODOT_2D_ADAPTER_BOUNDARY.to_string(),
        source: GodotSourceProjectRef {
            engine: MigrationSourceEngine::Godot,
            root_label: root_label.into(),
            accepted_formats: vec![
                ".tscn".to_string(),
                ".tres".to_string(),
                "project.godot".to_string(),
            ],
            source_files: files.iter().map(|file| file.path.clone()).collect(),
        },
        scenes,
        assets: asset_map.into_values().collect(),
        inputs: inputs.into_values().collect(),
        logic_touchpoints: logic_touchpoints.into_values().collect(),
        unsupported,
        fidelity_report: GodotFidelityReport {
            summary,
            records: fidelity_records,
            oracle_rule: "No imported Godot unit is ported/equivalent until a later Ouroforge-native oracle passes; 2D requires bit-exact deterministic state hashes.".to_string(),
            clean_room_notice: "Script, signal, callback, input reaction, and physics behavior are Era R clean-room re-derivation touchpoints, never copied or translated source code.".to_string(),
        },
    })
}

fn collect_godot_source_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<GodotSourceFile>,
) -> Result<()> {
    for entry in fs::read_dir(current).with_context(|| format!("reading {}", current.display()))? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_godot_source_files(root, &path, files)?;
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
        if relative.ends_with(".tscn") || relative.ends_with(".tres") || relative == "project.godot"
        {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("reading Godot source text {}", path.display()))?;
            files.push(GodotSourceFile {
                path: relative,
                contents,
            });
        }
    }
    Ok(())
}

fn reject_forbidden_files(files: &[GodotSourceFile]) -> Result<()> {
    for file in files {
        let lower = file.path.to_ascii_lowercase();
        if lower.ends_with(".pck")
            || lower.ends_with(".exe")
            || lower.ends_with(".dll")
            || lower.ends_with(".so")
            || lower.ends_with(".dylib")
        {
            return Err(anyhow!(
                "Godot adapter accepts source-project text only; rejected {}",
                file.path
            ));
        }
    }
    Ok(())
}

fn parse_sections(file: &GodotSourceFile) -> Result<Vec<Section>> {
    let mut sections = Vec::new();
    let mut current: Option<Section> = None;
    for (index, raw_line) in file.contents.lines().enumerate() {
        let line_no = index + 1;
        let line = strip_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            if let Some(section) = current.take() {
                sections.push(section);
            }
            let inner = &line[1..line.len() - 1];
            let (kind, attrs) = parse_section_header(inner);
            current = Some(Section {
                kind,
                attrs,
                props: BTreeMap::new(),
                line: line_no,
            });
        } else if let Some((key, value)) = line.split_once('=') {
            if let Some(section) = current.as_mut() {
                section
                    .props
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    if let Some(section) = current.take() {
        sections.push(section);
    }
    Ok(sections)
}

fn strip_comment(line: &str) -> &str {
    let hash = line.find('#');
    let semi = line.find(';');
    match (hash, semi) {
        (Some(a), Some(b)) => &line[..a.min(b)],
        (Some(a), None) => &line[..a],
        (None, Some(b)) => &line[..b],
        (None, None) => line,
    }
}

fn parse_section_header(inner: &str) -> (String, BTreeMap<String, String>) {
    let mut tokens = shell_like_tokens(inner);
    if tokens.is_empty() {
        return (String::new(), BTreeMap::new());
    }
    let kind = tokens.remove(0);
    let mut attrs = BTreeMap::new();
    for token in tokens {
        if let Some((key, value)) = token.split_once('=') {
            attrs.insert(key.to_string(), trim_quotes(value).to_string());
        }
    }
    (kind, attrs)
}

fn shell_like_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn trim_quotes(value: &str) -> &str {
    value.trim().trim_matches('"')
}

fn collect_external_assets(
    file: &GodotSourceFile,
    sections: &[Section],
    asset_map: &mut BTreeMap<String, GodotIrAssetRef>,
    fidelity_records: &mut BTreeMap<String, GodotFidelityRecord>,
) {
    for section in sections
        .iter()
        .filter(|section| section.kind == "ext_resource")
    {
        let path = section.attrs.get("path").cloned().unwrap_or_default();
        if path.is_empty() {
            continue;
        }
        let kind = section
            .attrs
            .get("type")
            .cloned()
            .unwrap_or_else(|| "Resource".to_string());
        let id =
            normalize_resource_id(section.attrs.get("id").map(String::as_str).unwrap_or(&path));
        let provenance = GodotProvenanceRef {
            source_path: file.path.clone(),
            line: section.line,
            section: "ext_resource".to_string(),
        };
        asset_map.insert(
            id.clone(),
            GodotIrAssetRef {
                id: id.clone(),
                source_path: path.clone(),
                kind,
                fidelity_grade: FidelityGrade::Green,
                provenance,
            },
        );
        fidelity_records.insert(
            format!("asset:{id}"),
            GodotFidelityRecord {
                unit_id: format!("asset:{id}"),
                grade: FidelityGrade::Green,
                reason: "Godot external resource reference is source text and project-local by declaration.".to_string(),
                source_path: file.path.clone(),
            },
        );
    }
}

fn parse_scene_nodes(
    file: &GodotSourceFile,
    sections: &[Section],
    logic_touchpoints: &mut BTreeMap<String, GodotIrLogicTouchpoint>,
    unsupported: &mut Vec<GodotIrUnsupportedFeature>,
    fidelity_records: &mut BTreeMap<String, GodotFidelityRecord>,
) -> GodotIrScene {
    let node_sections: Vec<_> = sections
        .iter()
        .filter(|section| section.kind == "node")
        .collect();
    let mut parent_by_name = BTreeMap::new();
    for section in &node_sections {
        if let Some(name) = section.attrs.get("name") {
            let parent = section.attrs.get("parent").cloned();
            let id = node_id(&file.path, name);
            parent_by_name.insert(name.clone(), (id, parent));
        }
    }

    let mut nodes = Vec::new();
    for (order, section) in node_sections.iter().enumerate() {
        let name = section
            .attrs
            .get("name")
            .cloned()
            .unwrap_or_else(|| format!("unnamed-{order}"));
        let godot_type = section
            .attrs
            .get("type")
            .cloned()
            .unwrap_or_else(|| "Node".to_string());
        let id = node_id(&file.path, &name);
        let parent_id = section
            .attrs
            .get("parent")
            .and_then(|parent| resolve_parent_id(&file.path, parent, &parent_by_name));
        let provenance = GodotProvenanceRef {
            source_path: file.path.clone(),
            line: section.line,
            section: format!("node:{name}"),
        };
        let supported = supported_node_type(&godot_type);
        let grade = grade_for_node_type(&godot_type);
        let mut metadata = BTreeMap::new();
        for key in [
            "texture",
            "tile_set",
            "shape",
            "text",
            "collision_layer",
            "collision_mask",
        ] {
            if let Some(value) = section.props.get(key) {
                metadata.insert(key.to_string(), value.clone());
            }
        }
        let node = GodotIrNode {
            id: id.clone(),
            source_path: file.path.clone(),
            name: name.clone(),
            godot_type: godot_type.clone(),
            parent_id,
            order,
            fidelity_grade: grade.clone(),
            transform2d: GodotTransform2d {
                position: section
                    .props
                    .get("position")
                    .and_then(|value| parse_vector2(value)),
                rotation: section
                    .props
                    .get("rotation")
                    .and_then(|value| parse_f64(value)),
                scale: section
                    .props
                    .get("scale")
                    .and_then(|value| parse_vector2(value)),
                z_index: section
                    .props
                    .get("z_index")
                    .and_then(|value| parse_i64(value)),
            },
            visibility: section
                .props
                .get("visible")
                .and_then(|value| parse_bool(value)),
            presentation: presentation_for(&godot_type, section),
            camera: camera_for(&godot_type, section),
            collider: collider_for(&godot_type, section),
            metadata,
            provenance: provenance.clone(),
        };
        if !supported {
            unsupported.push(GodotIrUnsupportedFeature {
                source_path: file.path.clone(),
                node_id: Some(id.clone()),
                feature_kind: godot_type.clone(),
                severity: "red".to_string(),
                reason: "Godot node type is outside the bounded 2D source-text subset.".to_string(),
                suggested_hand_off:
                    "Record as unsupported or redesign/re-derive in Era R if behavior matters."
                        .to_string(),
                fidelity_grade: FidelityGrade::Red,
                provenance: provenance.clone(),
            });
        }
        if let Some(script) = section.props.get("script") {
            let touchpoint_id = format!("logic:{id}:script");
            logic_touchpoints.insert(
                touchpoint_id.clone(),
                GodotIrLogicTouchpoint {
                    id: touchpoint_id.clone(),
                    source_path: file.path.clone(),
                    node_id: Some(id.clone()),
                    trigger_kind: "script-ref".to_string(),
                    symbol: Some(script.clone()),
                    exported_variables: exported_variables(section),
                    era_r_status: "requires-clean-room-re-derivation".to_string(),
                    fidelity_grade: FidelityGrade::Red,
                    provenance: provenance.clone(),
                },
            );
            fidelity_records.insert(
                touchpoint_id.clone(),
                GodotFidelityRecord {
                    unit_id: touchpoint_id,
                    grade: FidelityGrade::Red,
                    reason: "Script reference is inventoried only; gameplay logic must be clean-room re-derived in Era R.".to_string(),
                    source_path: file.path.clone(),
                },
            );
        }
        fidelity_records.insert(
            format!("node:{id}"),
            GodotFidelityRecord {
                unit_id: format!("node:{id}"),
                grade,
                reason: if supported {
                    "Supported Godot declarative node skeleton parsed into neutral IR.".to_string()
                } else {
                    "Unsupported Godot node is explicitly flagged instead of dropped.".to_string()
                },
                source_path: file.path.clone(),
            },
        );
        nodes.push(node);
    }

    for section in sections
        .iter()
        .filter(|section| section.kind == "connection")
    {
        let signal = section.attrs.get("signal").cloned();
        let from = section.attrs.get("from").cloned();
        let method = section.attrs.get("method").cloned();
        let touchpoint_id = format!(
            "logic:{}:connection:{}:{}",
            file.path,
            from.clone().unwrap_or_default(),
            signal.clone().unwrap_or_default()
        );
        logic_touchpoints.insert(
            touchpoint_id.clone(),
            GodotIrLogicTouchpoint {
                id: touchpoint_id.clone(),
                source_path: file.path.clone(),
                node_id: from.as_ref().map(|name| node_id(&file.path, name)),
                trigger_kind: "signal-connection".to_string(),
                symbol: method.or(signal),
                exported_variables: Vec::new(),
                era_r_status: "requires-clean-room-re-derivation".to_string(),
                fidelity_grade: FidelityGrade::Red,
                provenance: GodotProvenanceRef {
                    source_path: file.path.clone(),
                    line: section.line,
                    section: "connection".to_string(),
                },
            },
        );
        fidelity_records.insert(
            touchpoint_id.clone(),
            GodotFidelityRecord {
                unit_id: touchpoint_id,
                grade: FidelityGrade::Red,
                reason: "Signal connection is a behavior touchpoint, not translated logic."
                    .to_string(),
                source_path: file.path.clone(),
            },
        );
    }

    GodotIrScene {
        id: format!("scene:{}", file.path),
        source_path: file.path.clone(),
        nodes,
    }
}

fn parse_project_inputs(
    file: &GodotSourceFile,
    inputs: &mut BTreeMap<String, GodotIrInputAction>,
    fidelity_records: &mut BTreeMap<String, GodotFidelityRecord>,
) -> Result<()> {
    let sections = parse_sections(file)?;
    for section in sections.iter().filter(|section| section.kind == "input") {
        for (name, value) in &section.props {
            let bindings = extract_input_bindings(value);
            inputs.insert(
                name.clone(),
                GodotIrInputAction {
                    name: name.clone(),
                    bindings,
                    fidelity_grade: FidelityGrade::Green,
                    provenance: GodotProvenanceRef {
                        source_path: file.path.clone(),
                        line: section.line,
                        section: "input".to_string(),
                    },
                },
            );
            fidelity_records.insert(
                format!("input:{name}"),
                GodotFidelityRecord {
                    unit_id: format!("input:{name}"),
                    grade: FidelityGrade::Green,
                    reason:
                        "Godot InputMap action declaration parsed as declarative input skeleton."
                            .to_string(),
                    source_path: file.path.clone(),
                },
            );
        }
    }
    Ok(())
}

fn supported_node_type(godot_type: &str) -> bool {
    matches!(
        godot_type,
        "Node"
            | "Node2D"
            | "CanvasItem"
            | "Sprite2D"
            | "TileMap"
            | "TileSet"
            | "Camera2D"
            | "Area2D"
            | "CollisionShape2D"
            | "CollisionPolygon2D"
            | "StaticBody2D"
            | "CharacterBody2D"
            | "RigidBody2D"
            | "Label"
    )
}

fn grade_for_node_type(godot_type: &str) -> FidelityGrade {
    match godot_type {
        "TileMap" | "TileSet" | "Area2D" | "CollisionShape2D" | "CollisionPolygon2D"
        | "StaticBody2D" | "CharacterBody2D" | "RigidBody2D" => FidelityGrade::Yellow,
        _ if supported_node_type(godot_type) => FidelityGrade::Green,
        _ => FidelityGrade::Red,
    }
}

fn node_id(scene_path: &str, name: &str) -> String {
    format!("{scene_path}::{name}")
}

fn resolve_parent_id(
    scene_path: &str,
    parent: &str,
    parent_by_name: &BTreeMap<String, (String, Option<String>)>,
) -> Option<String> {
    if parent == "." || parent.is_empty() {
        return None;
    }
    let name = parent
        .rsplit('/')
        .next()
        .unwrap_or(parent)
        .trim_matches('"');
    parent_by_name
        .get(name)
        .map(|(id, _)| id.clone())
        .or_else(|| Some(node_id(scene_path, name)))
}

fn presentation_for(godot_type: &str, section: &Section) -> Option<GodotPresentation> {
    match godot_type {
        "Sprite2D" => Some(GodotPresentation::Sprite {
            texture_ref: section
                .props
                .get("texture")
                .map(|value| normalize_resource_id(value)),
            region: section
                .props
                .get("region_rect")
                .and_then(|value| parse_rect2(value)),
            modulate: section.props.get("modulate").cloned(),
        }),
        "TileMap" => Some(GodotPresentation::Tilemap {
            tile_set_ref: section
                .props
                .get("tile_set")
                .map(|value| normalize_resource_id(value)),
            cell_count_hint: section
                .props
                .iter()
                .filter(|(key, _)| key.contains("cell") || key.contains("tile_data"))
                .count(),
        }),
        "Label" => Some(GodotPresentation::Label {
            text: section
                .props
                .get("text")
                .map(|value| trim_quotes(value).to_string()),
        }),
        _ => None,
    }
}

fn camera_for(godot_type: &str, section: &Section) -> Option<GodotCamera2d> {
    if godot_type != "Camera2D" {
        return None;
    }
    let mut metadata_only_fields = Vec::new();
    for key in [
        "position_smoothing_enabled",
        "limit_left",
        "limit_right",
        "limit_top",
        "limit_bottom",
    ] {
        if section.props.contains_key(key) {
            metadata_only_fields.push(key.to_string());
        }
    }
    Some(GodotCamera2d {
        enabled: section
            .props
            .get("enabled")
            .and_then(|value| parse_bool(value))
            .unwrap_or(true),
        zoom: section
            .props
            .get("zoom")
            .and_then(|value| parse_vector2(value)),
        metadata_only_fields,
    })
}

fn collider_for(godot_type: &str, section: &Section) -> Option<GodotCollider2d> {
    if !matches!(
        godot_type,
        "Area2D"
            | "CollisionShape2D"
            | "CollisionPolygon2D"
            | "StaticBody2D"
            | "CharacterBody2D"
            | "RigidBody2D"
    ) {
        return None;
    }
    Some(GodotCollider2d {
        shape_ref: section
            .props
            .get("shape")
            .map(|value| normalize_resource_id(value)),
        sensor: godot_type == "Area2D"
            || section
                .props
                .get("disabled")
                .and_then(|value| parse_bool(value))
                .unwrap_or(false),
        collision_layer: section
            .props
            .get("collision_layer")
            .and_then(|value| parse_i64(value)),
        collision_mask: section
            .props
            .get("collision_mask")
            .and_then(|value| parse_i64(value)),
        physics_re_simulated: true,
    })
}

fn exported_variables(section: &Section) -> Vec<String> {
    section
        .props
        .keys()
        .filter(|key| {
            key.starts_with("export/") || key.starts_with("metadata/") || key.starts_with("script_")
        })
        .cloned()
        .collect()
}

fn normalize_resource_id(value: &str) -> String {
    let trimmed = trim_quotes(value.trim());
    if let Some(inner) = trimmed
        .strip_prefix("ExtResource(")
        .and_then(|v| v.strip_suffix(')'))
    {
        return trim_quotes(inner.trim()).to_string();
    }
    if let Some(inner) = trimmed
        .strip_prefix("SubResource(")
        .and_then(|v| v.strip_suffix(')'))
    {
        return trim_quotes(inner.trim()).to_string();
    }
    trimmed.to_string()
}

fn parse_vector2(value: &str) -> Option<[f64; 2]> {
    let inner = value.trim().strip_prefix("Vector2(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(|part| part.trim().parse::<f64>().ok());
    Some([parts.next()??, parts.next()??])
}

fn parse_rect2(value: &str) -> Option<GodotRect2> {
    let inner = value.trim().strip_prefix("Rect2(")?.strip_suffix(')')?;
    let nums: Vec<f64> = inner
        .split(',')
        .filter_map(|part| part.trim().parse::<f64>().ok())
        .collect();
    if nums.len() == 4 {
        Some(GodotRect2 {
            origin: [nums[0], nums[1]],
            size: [nums[2], nums[3]],
        })
    } else {
        None
    }
}

fn parse_f64(value: &str) -> Option<f64> {
    value.trim().parse::<f64>().ok()
}

fn parse_i64(value: &str) -> Option<i64> {
    value.trim().parse::<i64>().ok()
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn extract_input_bindings(value: &str) -> Vec<String> {
    let mut bindings = BTreeSet::new();
    for marker in ["physical_keycode", "keycode", "button_index", "axis"] {
        let mut rest = value;
        while let Some(index) = rest.find(marker) {
            rest = &rest[index + marker.len()..];
            if let Some(eq_index) = rest.find(':').or_else(|| rest.find('=')) {
                let tail = &rest[eq_index + 1..];
                let token: String = tail
                    .chars()
                    .skip_while(|ch| ch.is_whitespace())
                    .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '-')
                    .collect();
                if !token.is_empty() {
                    bindings.insert(format!("{marker}:{token}"));
                }
            }
        }
    }
    if bindings.is_empty() {
        bindings.insert("raw-input-event".to_string());
    }
    bindings.into_iter().collect()
}

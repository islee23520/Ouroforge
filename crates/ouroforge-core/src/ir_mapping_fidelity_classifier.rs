use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::godot_2d_adapter_ir::{
    FidelityGrade as AdapterFidelityGrade, GodotMigrationIr, GodotPresentation,
};

pub const IR_MAPPING_SCHEMA_VERSION: &str = "ir-to-ouroforge-mapping-v1";
pub const IR_MAPPING_BOUNDARY: &str =
    "one-way IR to Ouroforge-native candidate skeleton; oracle-gated; clean-room";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MappingFidelityGrade {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OuroforgeMappingArtifact {
    pub schema_version: String,
    pub boundary: String,
    pub source_project: String,
    pub coordinate_space: CoordinateSpaceNormalization,
    pub scenes: Vec<NativeSceneCandidate>,
    pub asset_mappings: Vec<NativeAssetMapping>,
    pub input_mappings: Vec<NativeInputMapping>,
    pub mapping_records: Vec<MappingRecord>,
    pub behavioral_units: Vec<BehavioralUnitMappingRecord>,
    pub oracle_records: Vec<OracleMappingRecord>,
    pub fidelity_report: MappingFidelityReport,
    pub state_hash: String,
    pub claimed_ported_units: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinateSpaceNormalization {
    pub source_units: String,
    pub target_units: String,
    pub pixels_per_unit: u32,
    pub origin: String,
    pub rotation_unit: String,
    pub color_space: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeSceneCandidate {
    pub id: String,
    pub source_scene: String,
    pub entities: Vec<NativeEntityCandidate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeEntityCandidate {
    pub id: String,
    pub source_node_id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub transform: NativeTransform2d,
    pub presentation: Option<NativePresentationCandidate>,
    pub collider: Option<NativeColliderCandidate>,
    pub fidelity_grade: MappingFidelityGrade,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeTransform2d {
    pub position_units: Option<[f64; 2]>,
    pub rotation_radians: Option<f64>,
    pub scale: Option<[f64; 2]>,
    pub z_index: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum NativePresentationCandidate {
    Sprite {
        asset_ref: Option<String>,
        color_space: String,
    },
    Tilemap {
        tile_set_ref: Option<String>,
        cell_count_hint: usize,
    },
    Label {
        text: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeColliderCandidate {
    pub shape_ref: Option<String>,
    pub sensor: bool,
    pub collision_layer: Option<i64>,
    pub collision_mask: Option<i64>,
    pub physics_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeAssetMapping {
    pub source_asset_id: String,
    pub source_path: String,
    pub native_asset_id: String,
    pub kind: String,
    pub fidelity_grade: MappingFidelityGrade,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeInputMapping {
    pub action: String,
    pub bindings: Vec<String>,
    pub fidelity_grade: MappingFidelityGrade,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingRecord {
    pub source_id: String,
    pub native_id: String,
    pub element_kind: String,
    pub fidelity_grade: MappingFidelityGrade,
    pub reason: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehavioralUnitMappingRecord {
    pub id: String,
    pub source_id: String,
    pub trigger_kind: String,
    pub era_r_status: String,
    pub fidelity_grade: MappingFidelityGrade,
    pub clean_room_instruction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleMappingRecord {
    pub unit_id: String,
    pub status: String,
    pub required_evidence: Vec<String>,
    pub ported_claim_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingFidelityReport {
    pub green: usize,
    pub yellow: usize,
    pub red: usize,
    pub oracle_rule: String,
    pub gap_summary: Vec<String>,
}

pub fn map_godot_ir_to_ouroforge(ir: &GodotMigrationIr) -> Result<OuroforgeMappingArtifact> {
    let coordinate_space = CoordinateSpaceNormalization {
        source_units: "godot-canvas-pixels".to_string(),
        target_units: "ouroforge-world-units".to_string(),
        pixels_per_unit: 1,
        origin: "2d-top-left-preserved-for-skeleton".to_string(),
        rotation_unit: "radians".to_string(),
        color_space: "srgb".to_string(),
    };

    let mut mapping_records = Vec::new();
    let mut scenes = Vec::new();
    for scene in &ir.scenes {
        let mut entities = Vec::new();
        for node in &scene.nodes {
            let native_id = format!("entity:{}", node.id);
            let grade = grade_from_adapter(&node.fidelity_grade);
            entities.push(NativeEntityCandidate {
                id: native_id.clone(),
                source_node_id: node.id.clone(),
                name: node.name.clone(),
                parent_id: node.parent_id.clone().map(|id| format!("entity:{id}")),
                transform: NativeTransform2d {
                    position_units: node.transform2d.position,
                    rotation_radians: node.transform2d.rotation,
                    scale: node.transform2d.scale,
                    z_index: node.transform2d.z_index,
                },
                presentation: node.presentation.as_ref().map(presentation_from_adapter),
                collider: node
                    .collider
                    .as_ref()
                    .map(|collider| NativeColliderCandidate {
                        shape_ref: collider.shape_ref.clone(),
                        sensor: collider.sensor,
                        collision_layer: collider.collision_layer,
                        collision_mask: collider.collision_mask,
                        physics_policy: "re-simulated-in-ouroforge-not-reproduced".to_string(),
                    }),
                fidelity_grade: grade.clone(),
            });
            mapping_records.push(MappingRecord {
                source_id: node.id.clone(),
                native_id,
                element_kind: format!("node:{}", node.godot_type),
                fidelity_grade: grade,
                reason: reason_for_adapter_grade(&node.fidelity_grade).to_string(),
                provenance: format!(
                    "{}:{}:{}",
                    node.provenance.source_path, node.provenance.line, node.provenance.section
                ),
            });
        }
        scenes.push(NativeSceneCandidate {
            id: format!("native:{}", scene.id),
            source_scene: scene.source_path.clone(),
            entities,
        });
    }

    let asset_mappings: Vec<_> = ir
        .assets
        .iter()
        .map(|asset| NativeAssetMapping {
            source_asset_id: asset.id.clone(),
            source_path: asset.source_path.clone(),
            native_asset_id: format!("asset:{}", asset.id),
            kind: asset.kind.clone(),
            fidelity_grade: grade_from_adapter(&asset.fidelity_grade),
        })
        .collect();
    for asset in &asset_mappings {
        mapping_records.push(MappingRecord {
            source_id: asset.source_asset_id.clone(),
            native_id: asset.native_asset_id.clone(),
            element_kind: format!("asset:{}", asset.kind),
            fidelity_grade: asset.fidelity_grade.clone(),
            reason: "Project-local asset reference mapped to native asset candidate.".to_string(),
            provenance: asset.source_path.clone(),
        });
    }

    let input_mappings: Vec<_> = ir
        .inputs
        .iter()
        .map(|input| NativeInputMapping {
            action: input.name.clone(),
            bindings: input.bindings.clone(),
            fidelity_grade: grade_from_adapter(&input.fidelity_grade),
        })
        .collect();
    for input in &input_mappings {
        mapping_records.push(MappingRecord {
            source_id: input.action.clone(),
            native_id: format!("input:{}", input.action),
            element_kind: "input-action".to_string(),
            fidelity_grade: input.fidelity_grade.clone(),
            reason: "Declarative input binding mapped; gameplay reaction remains separate logic."
                .to_string(),
            provenance: "project.godot".to_string(),
        });
    }

    let mut behavioral_units = Vec::new();
    let mut oracle_records = Vec::new();
    for touchpoint in &ir.logic_touchpoints {
        let id = format!("behavioral-unit:{}", touchpoint.id);
        behavioral_units.push(BehavioralUnitMappingRecord {
            id: id.clone(),
            source_id: touchpoint.id.clone(),
            trigger_kind: touchpoint.trigger_kind.clone(),
            era_r_status: "requires-clean-room-re-derivation".to_string(),
            fidelity_grade: MappingFidelityGrade::Red,
            clean_room_instruction:
                "Re-implement from observed behavior and interrogated intent; never copy or translate source code."
                    .to_string(),
        });
        oracle_records.push(OracleMappingRecord {
            unit_id: id,
            status: "missing".to_string(),
            required_evidence: vec![
                "captured acceptance oracle".to_string(),
                "2d bit-exact deterministic state hash".to_string(),
            ],
            ported_claim_allowed: false,
        });
    }
    for unsupported in &ir.unsupported {
        let id = format!(
            "behavioral-unit:unsupported:{}:{}",
            unsupported.source_path, unsupported.feature_kind
        );
        behavioral_units.push(BehavioralUnitMappingRecord {
            id: id.clone(),
            source_id: unsupported
                .node_id
                .clone()
                .unwrap_or_else(|| unsupported.source_path.clone()),
            trigger_kind: unsupported.feature_kind.clone(),
            era_r_status: "unsupported-or-human-redesign".to_string(),
            fidelity_grade: MappingFidelityGrade::Red,
            clean_room_instruction: unsupported.suggested_hand_off.clone(),
        });
        oracle_records.push(OracleMappingRecord {
            unit_id: id,
            status: "missing".to_string(),
            required_evidence: vec![
                "human redesign or clean-room re-derivation decision".to_string()
            ],
            ported_claim_allowed: false,
        });
    }

    let mut gap_summary: Vec<String> = ir
        .fidelity_report
        .records
        .iter()
        .filter(|record| record.grade != AdapterFidelityGrade::Green)
        .map(|record| format!("{}: {}", record.unit_id, record.reason))
        .collect();
    gap_summary.sort();
    gap_summary.dedup();
    if gap_summary.is_empty() {
        gap_summary.push("no known gaps in declarative skeleton mapping".to_string());
    }

    let mut artifact = OuroforgeMappingArtifact {
        schema_version: IR_MAPPING_SCHEMA_VERSION.to_string(),
        boundary: IR_MAPPING_BOUNDARY.to_string(),
        source_project: ir.source.root_label.clone(),
        coordinate_space,
        scenes,
        asset_mappings,
        input_mappings,
        mapping_records,
        behavioral_units,
        oracle_records,
        fidelity_report: MappingFidelityReport {
            green: 0,
            yellow: 0,
            red: 0,
            oracle_rule: "No mapped unit is ported/equivalent until a later Ouroforge-native oracle passes; 2D requires bit-exact deterministic state hashes.".to_string(),
            gap_summary,
        },
        state_hash: String::new(),
        claimed_ported_units: Vec::new(),
    };
    classify_counts(&mut artifact);
    artifact.state_hash = mapping_state_hash(&artifact)?;
    validate_mapping_artifact(&artifact)?;
    Ok(artifact)
}

pub fn validate_mapping_artifact(artifact: &OuroforgeMappingArtifact) -> Result<()> {
    if artifact.schema_version != IR_MAPPING_SCHEMA_VERSION {
        return Err(anyhow!(
            "unsupported mapping schema {}",
            artifact.schema_version
        ));
    }
    if artifact.boundary != IR_MAPPING_BOUNDARY {
        return Err(anyhow!("IR mapping boundary drifted"));
    }
    if !artifact.claimed_ported_units.is_empty() {
        return Err(anyhow!(
            "IR mapping cannot claim ported units without passing oracle evidence"
        ));
    }
    if !artifact.state_hash.starts_with("sha256:") || artifact.state_hash.len() != 71 {
        return Err(anyhow!(
            "IR mapping requires a sha256 deterministic state hash"
        ));
    }
    if artifact
        .oracle_records
        .iter()
        .any(|oracle| oracle.status != "passed" && oracle.ported_claim_allowed)
    {
        return Err(anyhow!("oracle-missing unit cannot allow a ported claim"));
    }
    if !artifact.behavioral_units.is_empty() && artifact.fidelity_report.red == 0 {
        return Err(anyhow!("behavioral or unsupported units must be red"));
    }
    let expected_hash = mapping_state_hash(artifact)?;
    if artifact.state_hash != expected_hash {
        return Err(anyhow!(
            "IR mapping state hash does not match canonical mapping artifact"
        ));
    }
    Ok(())
}

fn classify_counts(artifact: &mut OuroforgeMappingArtifact) {
    let mut green = 0;
    let mut yellow = 0;
    let mut red = 0;
    for grade in artifact
        .mapping_records
        .iter()
        .map(|record| &record.fidelity_grade)
        .chain(
            artifact
                .behavioral_units
                .iter()
                .map(|unit| &unit.fidelity_grade),
        )
    {
        match grade {
            MappingFidelityGrade::Green => green += 1,
            MappingFidelityGrade::Yellow => yellow += 1,
            MappingFidelityGrade::Red => red += 1,
        }
    }
    artifact.fidelity_report.green = green;
    artifact.fidelity_report.yellow = yellow;
    artifact.fidelity_report.red = red;
}

fn mapping_state_hash(artifact: &OuroforgeMappingArtifact) -> Result<String> {
    let mut canonical = artifact.clone();
    canonical.state_hash.clear();
    let bytes = serde_json::to_vec(&canonical)?;
    Ok(crate::export_hash::sha256_prefixed(&bytes))
}

fn grade_from_adapter(grade: &AdapterFidelityGrade) -> MappingFidelityGrade {
    match grade {
        AdapterFidelityGrade::Green => MappingFidelityGrade::Green,
        AdapterFidelityGrade::Yellow => MappingFidelityGrade::Yellow,
        AdapterFidelityGrade::Red => MappingFidelityGrade::Red,
    }
}

fn presentation_from_adapter(presentation: &GodotPresentation) -> NativePresentationCandidate {
    match presentation {
        GodotPresentation::Sprite { texture_ref, .. } => NativePresentationCandidate::Sprite {
            asset_ref: texture_ref.clone(),
            color_space: "srgb".to_string(),
        },
        GodotPresentation::Tilemap {
            tile_set_ref,
            cell_count_hint,
        } => NativePresentationCandidate::Tilemap {
            tile_set_ref: tile_set_ref.clone(),
            cell_count_hint: *cell_count_hint,
        },
        GodotPresentation::Label { text } => {
            NativePresentationCandidate::Label { text: text.clone() }
        }
    }
}

fn reason_for_adapter_grade(grade: &AdapterFidelityGrade) -> &'static str {
    match grade {
        AdapterFidelityGrade::Green => {
            "Declarative skeleton fact mapped to Ouroforge-native candidate."
        }
        AdapterFidelityGrade::Yellow => {
            "Partial or metadata-only mapping; gap remains visible in fidelity report."
        }
        AdapterFidelityGrade::Red => "Unsupported or behavioral unit routed to Era R; not ported.",
    }
}

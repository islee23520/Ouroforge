use crate::export_hash::sha256_hex;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const GLTF_25D_IMPORT_REPORT_SCHEMA_VERSION: &str = "ouroforge.gltf-25d-import-report.v1";
pub const GLTF_25D_BOUNDARY: &str = "one-way source-project glTF presentation import only; no live engine bridge, no embedded runtime, no decompiled source, no gameplay logic translation, no trusted Studio write path";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dImportOptions {
    #[serde(rename = "sourceProjectRef")]
    pub source_project_ref: String,
    #[serde(rename = "sourcePath")]
    pub source_path: String,
    #[serde(default = "default_unit_scale", rename = "unitScale")]
    pub unit_scale: f64,
    #[serde(default = "default_axis", rename = "axisConvention")]
    pub axis_convention: String,
    #[serde(default = "default_color_space", rename = "colorSpace")]
    pub color_space: String,
    #[serde(default = "default_viewport_width", rename = "viewportWidth")]
    pub viewport_width: i64,
    #[serde(default = "default_viewport_height", rename = "viewportHeight")]
    pub viewport_height: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dImportReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "sourceProjectRef")]
    pub source_project_ref: String,
    #[serde(rename = "sourcePath")]
    pub source_path: String,
    pub boundary: String,
    #[serde(rename = "normalization")]
    pub normalization: Gltf25dNormalization,
    #[serde(rename = "nativeScene")]
    pub native_scene: Gltf25dNativeScene,
    #[serde(rename = "fidelityRows")]
    pub fidelity_rows: Vec<Gltf25dFidelityRow>,
    #[serde(rename = "reDerivationTasks")]
    pub re_derivation_tasks: Vec<Gltf25dReDerivationTask>,
    #[serde(rename = "stateHashPrimary")]
    pub state_hash_primary: String,
    #[serde(rename = "perceptualRenderSecondary")]
    pub perceptual_render_secondary: Gltf25dPerceptualRenderSecondary,
    #[serde(rename = "oracleRule")]
    pub oracle_rule: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dNormalization {
    #[serde(rename = "axisConvention")]
    pub axis_convention: String,
    #[serde(rename = "unitScale")]
    pub unit_scale: f64,
    #[serde(rename = "colorSpace")]
    pub color_space: String,
    #[serde(rename = "cameraProjection")]
    pub camera_projection: String,
    #[serde(rename = "deterministicStateOwner")]
    pub deterministic_state_owner: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dNativeScene {
    pub id: String,
    #[serde(rename = "sceneKind")]
    pub scene_kind: String,
    #[serde(rename = "logicAuthority")]
    pub logic_authority: String,
    pub nodes: Vec<Gltf25dNativeNode>,
    pub meshes: Vec<Gltf25dNativeMesh>,
    pub materials: Vec<Gltf25dNativeMaterial>,
    pub cameras: Vec<Gltf25dNativeCamera>,
    #[serde(rename = "activeCameraId")]
    pub active_camera_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dNativeNode {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "meshRef", default, skip_serializing_if = "Option::is_none")]
    pub mesh_ref: Option<String>,
    #[serde(rename = "cameraRef", default, skip_serializing_if = "Option::is_none")]
    pub camera_ref: Option<String>,
    #[serde(rename = "localTransform")]
    pub local_transform: Gltf25dTransform,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dNativeMesh {
    pub id: String,
    #[serde(rename = "sourceMeshIndex")]
    pub source_mesh_index: usize,
    #[serde(rename = "primitiveCount")]
    pub primitive_count: usize,
    #[serde(rename = "materialRefs")]
    pub material_refs: Vec<String>,
    #[serde(rename = "fidelityGrade")]
    pub fidelity_grade: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dNativeMaterial {
    pub id: String,
    pub kind: String,
    #[serde(rename = "baseColor")]
    pub base_color: String,
    #[serde(rename = "fidelityGrade")]
    pub fidelity_grade: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dNativeCamera {
    pub id: String,
    pub projection: String,
    #[serde(rename = "orthographicHeight")]
    pub orthographic_height: f64,
    pub near: f64,
    pub far: f64,
    pub viewport: Gltf25dViewport,
    #[serde(rename = "fidelityGrade")]
    pub fidelity_grade: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dTransform {
    pub translation: Gltf25dVec3,
    pub rotation: Gltf25dVec3,
    pub scale: Gltf25dVec3,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dViewport {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dFidelityRow {
    pub unit: String,
    pub grade: String,
    pub reason: String,
    #[serde(rename = "oracleRequired")]
    pub oracle_required: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dReDerivationTask {
    #[serde(rename = "taskId")]
    pub task_id: String,
    pub unit: String,
    pub reason: String,
    #[serde(rename = "eraRInput")]
    pub era_r_input: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Gltf25dPerceptualRenderSecondary {
    pub method: String,
    pub threshold: String,
    pub role: String,
}

pub fn normalize_gltf_25d_import_from_str(
    input: &str,
    options: Gltf25dImportOptions,
) -> Result<Gltf25dImportReport> {
    validate_options(&options)?;
    let document: Value = serde_json::from_str(input).context("failed to parse glTF JSON")?;
    reject_blocked_source_markers(&document)?;
    let asset_version = document
        .get("asset")
        .and_then(|asset| asset.get("version"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !asset_version.starts_with('2') {
        return Err(anyhow!("glTF asset.version must be 2.x"));
    }

    let materials = normalize_materials(&document);
    let meshes = normalize_meshes(&document, &materials)?;
    let cameras = normalize_cameras(&document, &options)?;
    if cameras
        .iter()
        .all(|camera| camera.projection != "orthographic")
    {
        return Err(anyhow!(
            "M97 requires at least one orthographic camera for the 2.5D presentation import"
        ));
    }
    let nodes = normalize_nodes(&document, options.unit_scale)?;
    if nodes.is_empty() {
        return Err(anyhow!("glTF nodes must not be empty"));
    }

    let active_camera_id = cameras
        .iter()
        .find(|camera| camera.projection == "orthographic")
        .map(|camera| camera.id.clone())
        .unwrap_or_else(|| cameras[0].id.clone());

    let scene = Gltf25dNativeScene {
        id: safe_id_from_path(&options.source_path),
        scene_kind: "2.5d-presentation".to_string(),
        logic_authority:
            "2D deterministic Ouroforge state; glTF presentation cannot mutate gameplay truth"
                .to_string(),
        nodes,
        meshes,
        materials: materials.into_values().collect(),
        cameras,
        active_camera_id,
    };

    let mut fidelity_rows = build_fidelity_rows(&document, &scene);
    let mut re_derivation_tasks = build_re_derivation_tasks(&document);
    if re_derivation_tasks.is_empty() {
        fidelity_rows.push(Gltf25dFidelityRow {
            unit: "logic".to_string(),
            grade: "red".to_string(),
            reason: "No gameplay logic is imported in M97; any logic-bearing source unit must go to Era R with oracle evidence.".to_string(),
            oracle_required: true,
        });
        re_derivation_tasks.push(Gltf25dReDerivationTask {
            task_id: "era-r-logic-handoff".to_string(),
            unit: "gameplay-logic".to_string(),
            reason: "M97 imports presentation skeletons only; logic is re-derived clean-room, never translated.".to_string(),
            era_r_input: "capture observed behavior + interrogated intent + deterministic state oracle before re-expression".to_string(),
        });
    }

    let normalization = Gltf25dNormalization {
        axis_convention: options.axis_convention.clone(),
        unit_scale: options.unit_scale,
        color_space: options.color_space.clone(),
        camera_projection: "orthographic/isometric presentation only".to_string(),
        deterministic_state_owner:
            "Rust data plane state-hash primary; perceptual render secondary".to_string(),
    };
    let canonical_scene = serde_json::to_vec(&scene).context("canonical scene serializes")?;
    let state_hash_primary = format!("sha256:{}", sha256_hex(&canonical_scene));

    Ok(Gltf25dImportReport {
        schema_version: GLTF_25D_IMPORT_REPORT_SCHEMA_VERSION.to_string(),
        source_project_ref: options.source_project_ref,
        source_path: options.source_path,
        boundary: GLTF_25D_BOUNDARY.to_string(),
        normalization,
        native_scene: scene,
        fidelity_rows,
        re_derivation_tasks,
        state_hash_primary,
        perceptual_render_secondary: Gltf25dPerceptualRenderSecondary {
            method: "SSIM/pixel-diff tolerance over normalized presentation render".to_string(),
            threshold: "fixture-defined tolerance; never cross-machine bit hash".to_string(),
            role: "secondary corroboration only".to_string(),
        },
        oracle_rule: "Nothing is claimed ported without captured acceptance evidence and a passing oracle; presentation imports are best-effort with honest fidelity rows.".to_string(),
    })
}

impl Gltf25dImportReport {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GLTF_25D_IMPORT_REPORT_SCHEMA_VERSION {
            return Err(anyhow!(
                "gltf 2.5d import report schemaVersion must be {GLTF_25D_IMPORT_REPORT_SCHEMA_VERSION}"
            ));
        }
        if !self.boundary.contains("one-way")
            || !self.boundary.contains("no live engine bridge")
            || !self.boundary.contains("no decompiled source")
        {
            return Err(anyhow!(
                "gltf 2.5d boundary must preserve one-way clean-room rules"
            ));
        }
        if self
            .native_scene
            .logic_authority
            .to_lowercase()
            .contains("gltf")
            && !self.native_scene.logic_authority.contains("cannot mutate")
        {
            return Err(anyhow!("glTF import cannot own gameplay logic authority"));
        }
        if !self.state_hash_primary.starts_with("sha256:") {
            return Err(anyhow!(
                "stateHashPrimary must be a sha256-prefixed deterministic digest"
            ));
        }
        let expected_state_hash = format!(
            "sha256:{}",
            sha256_hex(&serde_json::to_vec(&self.native_scene)?)
        );
        if self.state_hash_primary != expected_state_hash {
            return Err(anyhow!(
                "stateHashPrimary must match the deterministic nativeScene digest"
            ));
        }
        if !self
            .perceptual_render_secondary
            .role
            .contains("secondary corroboration")
        {
            return Err(anyhow!("perceptual render evidence must remain secondary"));
        }
        if self.fidelity_rows.is_empty() {
            return Err(anyhow!("fidelityRows must not be empty"));
        }
        if self.fidelity_rows.iter().any(|row| {
            row.reason
                .to_lowercase()
                .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-')
                .any(|token| {
                    matches!(
                        token,
                        "ported" | "auto-port" | "auto-ported" | "auto-translated"
                    )
                })
        }) {
            return Err(anyhow!("fidelity rows must not claim units were ported"));
        }
        if self.re_derivation_tasks.is_empty() {
            return Err(anyhow!(
                "reDerivationTasks must record logic/Era R hand-off"
            ));
        }
        Ok(())
    }
}

fn normalize_materials(document: &Value) -> BTreeMap<usize, Gltf25dNativeMaterial> {
    let mut map = BTreeMap::new();
    for (index, material) in document
        .get("materials")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        let id = named_id(material, "material", index);
        let has_unlit = material
            .get("extensions")
            .and_then(|extensions| extensions.get("KHR_materials_unlit"))
            .is_some();
        let base_color = material
            .get("pbrMetallicRoughness")
            .and_then(|pbr| pbr.get("baseColorFactor"))
            .and_then(Value::as_array)
            .map(|values| color_hex(values))
            .unwrap_or_else(|| "#ffffff".to_string());
        let unsupported_extension = material
            .get("extensions")
            .and_then(Value::as_object)
            .map(|extensions| {
                extensions
                    .keys()
                    .any(|key| key != "KHR_materials_unlit" && key != "KHR_texture_transform")
            })
            .unwrap_or(false);
        map.insert(
            index,
            Gltf25dNativeMaterial {
                id,
                kind: if has_unlit {
                    "unlit"
                } else {
                    "metallic-roughness"
                }
                .to_string(),
                base_color,
                fidelity_grade: if unsupported_extension {
                    "yellow"
                } else {
                    "green"
                }
                .to_string(),
            },
        );
    }
    if map.is_empty() {
        map.insert(
            0,
            Gltf25dNativeMaterial {
                id: "default-material".to_string(),
                kind: "metallic-roughness".to_string(),
                base_color: "#ffffff".to_string(),
                fidelity_grade: "yellow".to_string(),
            },
        );
    }
    map
}

fn normalize_meshes(
    document: &Value,
    materials: &BTreeMap<usize, Gltf25dNativeMaterial>,
) -> Result<Vec<Gltf25dNativeMesh>> {
    let meshes = document
        .get("meshes")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("glTF meshes must not be empty"))?;
    let mut output = Vec::new();
    for (index, mesh) in meshes.iter().enumerate() {
        let primitives = mesh
            .get("primitives")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("glTF mesh {index} primitives must be an array"))?;
        if primitives.is_empty() {
            return Err(anyhow!("glTF mesh {index} primitives must not be empty"));
        }
        let mut material_refs = BTreeSet::new();
        let mut yellow = false;
        for primitive in primitives {
            let mode = primitive.get("mode").and_then(Value::as_i64).unwrap_or(4);
            if mode != 4 {
                yellow = true;
            }
            if primitive.get("extensions").is_some() {
                yellow = true;
            }
            let material_index = primitive
                .get("material")
                .and_then(Value::as_u64)
                .map(|value| value as usize)
                .unwrap_or(0);
            let Some(material) = materials.get(&material_index) else {
                return Err(anyhow!(
                    "glTF mesh {index} references missing material {material_index}"
                ));
            };
            material_refs.insert(material.id.clone());
        }
        output.push(Gltf25dNativeMesh {
            id: named_id(mesh, "mesh", index),
            source_mesh_index: index,
            primitive_count: primitives.len(),
            material_refs: material_refs.into_iter().collect(),
            fidelity_grade: if yellow { "yellow" } else { "green" }.to_string(),
        });
    }
    Ok(output)
}

fn normalize_cameras(
    document: &Value,
    options: &Gltf25dImportOptions,
) -> Result<Vec<Gltf25dNativeCamera>> {
    let cameras = document
        .get("cameras")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("glTF cameras must include an orthographic camera"))?;
    let mut output = Vec::new();
    for (index, camera) in cameras.iter().enumerate() {
        let camera_type = camera
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        match camera_type {
            "orthographic" => {
                let ortho = camera.get("orthographic").ok_or_else(|| {
                    anyhow!("orthographic camera {index} requires orthographic settings")
                })?;
                let ymag = number_at(ortho, "ymag").unwrap_or(10.0);
                output.push(Gltf25dNativeCamera {
                    id: named_id(camera, "camera", index),
                    projection: "orthographic".to_string(),
                    orthographic_height: round3(ymag * options.unit_scale),
                    near: number_at(ortho, "znear").unwrap_or(0.1),
                    far: number_at(ortho, "zfar").unwrap_or(1000.0),
                    viewport: Gltf25dViewport {
                        width: options.viewport_width,
                        height: options.viewport_height,
                    },
                    fidelity_grade: "green".to_string(),
                });
            }
            "perspective" => {
                let perspective = camera.get("perspective").unwrap_or(&Value::Null);
                output.push(Gltf25dNativeCamera {
                    id: named_id(camera, "camera", index),
                    projection: "perspective-presentation-warning".to_string(),
                    orthographic_height: 0.0,
                    near: number_at(perspective, "znear").unwrap_or(0.1),
                    far: number_at(perspective, "zfar").unwrap_or(1000.0),
                    viewport: Gltf25dViewport {
                        width: options.viewport_width,
                        height: options.viewport_height,
                    },
                    fidelity_grade: "yellow".to_string(),
                });
            }
            other => return Err(anyhow!("unsupported glTF camera type for M97: {other}")),
        }
    }
    Ok(output)
}

fn normalize_nodes(document: &Value, unit_scale: f64) -> Result<Vec<Gltf25dNativeNode>> {
    let nodes = document
        .get("nodes")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("glTF nodes must not be empty"))?;
    let mut parents = BTreeMap::new();
    for (parent_index, node) in nodes.iter().enumerate() {
        if let Some(children) = node.get("children").and_then(Value::as_array) {
            for child in children {
                if let Some(child_index) = child.as_u64() {
                    parents.insert(child_index as usize, parent_index);
                }
            }
        }
    }
    let mut output = Vec::new();
    for (index, node) in nodes.iter().enumerate() {
        let id = named_id(node, "node", index);
        let parent = parents
            .get(&index)
            .map(|idx| named_id(&nodes[*idx], "node", *idx));
        output.push(Gltf25dNativeNode {
            id,
            parent,
            mesh_ref: node.get("mesh").and_then(Value::as_u64).map(|mesh_index| {
                named_id(
                    &document["meshes"][mesh_index as usize],
                    "mesh",
                    mesh_index as usize,
                )
            }),
            camera_ref: node
                .get("camera")
                .and_then(Value::as_u64)
                .map(|camera_index| {
                    named_id(
                        &document["cameras"][camera_index as usize],
                        "camera",
                        camera_index as usize,
                    )
                }),
            local_transform: transform_from_node(node, unit_scale),
            role: if node.get("camera").is_some() {
                "presentation-camera"
            } else if node.get("mesh").is_some() {
                "presentation-geometry"
            } else {
                "presentation-group"
            }
            .to_string(),
        });
    }
    Ok(output)
}

fn build_fidelity_rows(document: &Value, scene: &Gltf25dNativeScene) -> Vec<Gltf25dFidelityRow> {
    let mut rows = Vec::new();
    for mesh in &scene.meshes {
        rows.push(Gltf25dFidelityRow {
            unit: format!("mesh:{}", mesh.id),
            grade: mesh.fidelity_grade.clone(),
            reason: if mesh.fidelity_grade == "green" {
                "geometry primitive data imported as presentation skeleton"
            } else {
                "mesh has unsupported primitive mode or extension; preserved as presentation gap"
            }
            .to_string(),
            oracle_required: false,
        });
    }
    for material in &scene.materials {
        rows.push(Gltf25dFidelityRow {
            unit: format!("material:{}", material.id),
            grade: material.fidelity_grade.clone(),
            reason: if material.fidelity_grade == "green" {
                "standard metallic-roughness/unlit material mapped for presentation"
            } else {
                "custom shader/extension must be baked or re-authored; not translated"
            }
            .to_string(),
            oracle_required: false,
        });
    }
    for camera in &scene.cameras {
        rows.push(Gltf25dFidelityRow {
            unit: format!("camera:{}", camera.id),
            grade: camera.fidelity_grade.clone(),
            reason: if camera.projection == "orthographic" {
                "orthographic/isometric camera imported for presentation"
            } else {
                "perspective camera is presentation-only warning in M97"
            }
            .to_string(),
            oracle_required: false,
        });
    }
    for extension in document
        .get("extensionsUsed")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        if !matches!(extension, "KHR_materials_unlit" | "KHR_texture_transform") {
            rows.push(Gltf25dFidelityRow {
                unit: format!("extension:{extension}"),
                grade: "yellow".to_string(),
                reason:
                    "unsupported glTF extension is recorded as a fidelity gap, not silently dropped"
                        .to_string(),
                oracle_required: false,
            });
        }
    }
    rows
}

fn build_re_derivation_tasks(document: &Value) -> Vec<Gltf25dReDerivationTask> {
    let mut tasks = Vec::new();
    let serialized = document.to_string().to_lowercase();
    for token in ["script", "logic", "physics", "shader", "vfx"] {
        if serialized.contains(token) {
            tasks.push(Gltf25dReDerivationTask {
                task_id: format!("era-r-{token}-handoff"),
                unit: token.to_string(),
                reason: format!("{token} is behavior-bearing or source-engine-specific; M97 does not translate it"),
                era_r_input: "observed behavior + interrogated intent + captured oracle + deterministic state expectations".to_string(),
            });
        }
    }
    tasks
}

fn validate_options(options: &Gltf25dImportOptions) -> Result<()> {
    if options.source_project_ref.trim().is_empty() || options.source_path.trim().is_empty() {
        return Err(anyhow!("sourceProjectRef and sourcePath are required"));
    }
    let lowered = format!("{} {}", options.source_project_ref, options.source_path).to_lowercase();
    if [
        "decompiled",
        "shipped-build",
        "live-bridge",
        "runtime-embed",
        "../",
        "\\",
    ]
    .iter()
    .any(|token| lowered.contains(token))
    {
        return Err(anyhow!(
            "glTF import source must be source-project/open-format and one-way only"
        ));
    }
    if options.unit_scale <= 0.0 || !options.unit_scale.is_finite() {
        return Err(anyhow!("unitScale must be finite and positive"));
    }
    if options.viewport_width <= 0 || options.viewport_height <= 0 {
        return Err(anyhow!("viewport dimensions must be positive"));
    }
    Ok(())
}

fn reject_blocked_source_markers(document: &Value) -> Result<()> {
    let serialized = document.to_string().to_lowercase();
    for token in [
        "decompiled",
        "shipped-build",
        "live bridge",
        "embedded runtime",
        "auto-port",
    ] {
        if serialized.contains(token) {
            return Err(anyhow!("blocked glTF source marker found: {token}"));
        }
    }
    Ok(())
}

fn transform_from_node(node: &Value, unit_scale: f64) -> Gltf25dTransform {
    let translation = vec3_from_array(node.get("translation"), [0.0, 0.0, 0.0], unit_scale);
    let scale = vec3_from_array(node.get("scale"), [1.0, 1.0, 1.0], 1.0);
    let rotation = node
        .get("rotation")
        .and_then(Value::as_array)
        .and_then(|values| {
            if values.len() == 4 {
                Some(Gltf25dVec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                })
            } else {
                None
            }
        })
        .unwrap_or(Gltf25dVec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
    Gltf25dTransform {
        translation,
        rotation,
        scale,
    }
}

fn vec3_from_array(value: Option<&Value>, default: [f64; 3], scale: f64) -> Gltf25dVec3 {
    let values = value.and_then(Value::as_array);
    let at = |index: usize| {
        values
            .and_then(|items| items.get(index))
            .and_then(Value::as_f64)
            .unwrap_or(default[index])
    };
    Gltf25dVec3 {
        x: round3(at(0) * scale),
        y: round3(at(1) * scale),
        z: round3(at(2) * scale),
    }
}

fn named_id(value: &Value, fallback: &str, index: usize) -> String {
    value
        .get("name")
        .and_then(Value::as_str)
        .map(safe_id)
        .filter(|id| !id.is_empty())
        .unwrap_or_else(|| format!("{fallback}-{index}"))
}

fn safe_id_from_path(path: &str) -> String {
    let stem = path
        .rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".gltf")
        .trim_end_matches(".glb");
    let id = safe_id(stem);
    if id.is_empty() {
        "gltf-25d-import".to_string()
    } else {
        id
    }
}

fn safe_id(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn number_at(value: &Value, field: &str) -> Option<f64> {
    value.get(field).and_then(Value::as_f64).map(round3)
}

fn color_hex(values: &[Value]) -> String {
    let to_u8 = |index: usize| -> u8 {
        let value = values.get(index).and_then(Value::as_f64).unwrap_or(1.0);
        (value.clamp(0.0, 1.0) * 255.0).round() as u8
    };
    format!("#{:02x}{:02x}{:02x}", to_u8(0), to_u8(1), to_u8(2))
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn default_unit_scale() -> f64 {
    1.0
}
fn default_axis() -> String {
    "gltf-y-up-right-handed-to-ouroforge-presentation".to_string()
}
fn default_color_space() -> String {
    "srgb-textures-linear-lighting".to_string()
}
fn default_viewport_width() -> i64 {
    640
}
fn default_viewport_height() -> i64 {
    360
}

pub fn write_report_json(report: &Gltf25dImportReport) -> Result<String> {
    report.validate()?;
    serde_json::to_string_pretty(report).context("gltf 2.5d report serializes")
}

pub fn example_report_from_fixture() -> Result<Gltf25dImportReport> {
    normalize_gltf_25d_import_from_str(
        include_str!("../../../examples/2-5d-gltf-import-v1/source/ortho-demo.gltf"),
        Gltf25dImportOptions {
            source_project_ref: "examples/2-5d-gltf-import-v1/source-project".to_string(),
            source_path: "examples/2-5d-gltf-import-v1/source/ortho-demo.gltf".to_string(),
            unit_scale: 1.0,
            axis_convention: default_axis(),
            color_space: default_color_space(),
            viewport_width: 640,
            viewport_height: 360,
        },
    )
}

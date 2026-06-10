use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const TILEMAP_SOURCE_SCHEMA_VERSION: &str = "ouroforge.tilemap-source.v1";
pub const TILEMAP_DRAFT_SCHEMA_VERSION: &str = "ouroforge.tilemap-draft.v1";
pub const TILEMAP_REACHABILITY_SCHEMA_VERSION: &str = "ouroforge.tilemap-reachability.v1";
pub const TILEMAP_LIVE_REPLAY_SCHEMA_VERSION: &str = "ouroforge.tilemap-live-replay.v1";
pub const TILEMAP_SOURCE_PATH_PREFIX: &str = "examples/tilemap-authoring-v1/maps/";
pub const TILEMAP_SOURCE_PATH_SUFFIX: &str = ".tilemap.json";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapSourceArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "mapId")]
    pub map_id: String,
    #[serde(rename = "sourcePath")]
    pub source_path: String,
    #[serde(rename = "pathConvention")]
    pub path_convention: String,
    #[serde(rename = "tilesetRef")]
    pub tileset_ref: String,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "tileSize")]
    pub tile_size: u32,
    pub palette: Vec<TilemapPaletteEntry>,
    pub layers: Vec<TilemapLayer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub markers: Vec<TilemapMarker>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapPaletteEntry {
    #[serde(rename = "tileId")]
    pub tile_id: String,
    pub label: String,
    #[serde(default)]
    pub solid: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TilemapLayerKind {
    Ground,
    Collision,
    Trigger,
    Objective,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapLayer {
    #[serde(rename = "layerId")]
    pub layer_id: String,
    pub kind: TilemapLayerKind,
    pub cells: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TilemapMarkerKind {
    Spawn,
    Objective,
    Trigger,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapMarker {
    #[serde(rename = "markerId")]
    pub marker_id: String,
    pub kind: TilemapMarkerKind,
    pub x: u32,
    pub y: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl TilemapSourceArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse tilemap source")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != TILEMAP_SOURCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "tilemap source schemaVersion must be {TILEMAP_SOURCE_SCHEMA_VERSION}"
            ));
        }
        validate_identifier("tilemap source mapId", &self.map_id)?;
        validate_source_path(&self.source_path)?;
        if self.path_convention
            != format!("{TILEMAP_SOURCE_PATH_PREFIX}<map-id>{TILEMAP_SOURCE_PATH_SUFFIX}")
        {
            return Err(anyhow!("tilemap source pathConvention must name the shared dogfood tilemap path convention"));
        }
        validate_repo_relative_ref("tilemap source tilesetRef", &self.tileset_ref)?;
        if self.width == 0 || self.height == 0 || self.width > 128 || self.height > 128 {
            return Err(anyhow!("tilemap source width/height must be 1..=128"));
        }
        if self.tile_size == 0 || self.tile_size > 256 {
            return Err(anyhow!("tilemap source tileSize must be 1..=256"));
        }
        let palette = validate_palette(&self.palette)?;
        let layer_ids = validate_layers(self.width, self.height, &palette, &self.layers)?;
        let _ = layer_ids;
        validate_markers(self.width, self.height, &self.markers)?;
        Ok(())
    }

    pub fn is_blocked(&self, x: u32, y: u32) -> bool {
        self.layers.iter().any(|layer| {
            layer.kind == TilemapLayerKind::Collision && layer.cells[y as usize][x as usize] != "."
        }) || self.layers.iter().any(|layer| {
            layer.cells[y as usize][x as usize] != "."
                && self.palette.iter().any(|entry| {
                    entry.tile_id == layer.cells[y as usize][x as usize] && entry.solid
                })
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapDraftArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    pub target: TilemapDraftTarget,
    #[serde(rename = "proposedOperations")]
    pub proposed_operations: Vec<TilemapDraftOperation>,
    #[serde(rename = "validationStatus")]
    pub validation_status: TilemapDraftValidationStatus,
    #[serde(rename = "previewSummary")]
    pub preview_summary: TilemapDraftPreviewSummary,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapDraftTarget {
    pub path: String,
    #[serde(rename = "baseDigest")]
    pub base_digest: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "camelCase", deny_unknown_fields)]
pub enum TilemapDraftOperation {
    Paint {
        #[serde(rename = "operationId")]
        operation_id: String,
        #[serde(rename = "layerId")]
        layer_id: String,
        x: u32,
        y: u32,
        #[serde(rename = "tileId")]
        tile_id: String,
    },
    Erase {
        #[serde(rename = "operationId")]
        operation_id: String,
        #[serde(rename = "layerId")]
        layer_id: String,
        x: u32,
        y: u32,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TilemapDraftValidationStatus {
    Drafted,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapDraftPreviewSummary {
    #[serde(rename = "changedCells")]
    pub changed_cells: Vec<TilemapChangedCell>,
    #[serde(rename = "affectedCollisionCells")]
    pub affected_collision_cells: Vec<TilemapGridPoint>,
    #[serde(rename = "affectedTriggerMarkers")]
    pub affected_trigger_markers: Vec<String>,
    #[serde(rename = "generatedPreviewOnly")]
    pub generated_preview_only: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct TilemapGridPoint {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapChangedCell {
    #[serde(rename = "layerId")]
    pub layer_id: String,
    pub x: u32,
    pub y: u32,
    pub before: String,
    pub after: String,
}

impl TilemapDraftArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        serde_json::from_str(input).context("failed to parse tilemap draft")
    }
}

pub fn tilemap_base_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv64:{hash:016x}")
}

pub fn validate_tilemap_draft_against_base(
    draft: &TilemapDraftArtifact,
    base_json: &str,
) -> Result<TilemapDraftPreviewSummary> {
    if draft.schema_version != TILEMAP_DRAFT_SCHEMA_VERSION {
        return Err(anyhow!(
            "tilemap draft schemaVersion must be {TILEMAP_DRAFT_SCHEMA_VERSION}"
        ));
    }
    validate_identifier("tilemap draft draftId", &draft.draft_id)?;
    validate_source_path(&draft.target.path)?;
    let base = TilemapSourceArtifact::from_json_str(base_json)?;
    if draft.target.path != base.source_path {
        return Err(anyhow!(
            "tilemap draft target path must match base tilemap sourcePath"
        ));
    }
    let digest = tilemap_base_digest(base_json);
    if draft.target.base_digest != digest {
        return Err(anyhow!("tilemap draft baseDigest is stale"));
    }
    if draft.proposed_operations.is_empty() {
        return Err(anyhow!("tilemap draft requires proposedOperations"));
    }
    let mut operation_ids = BTreeSet::new();
    let mut by_layer = base
        .layers
        .iter()
        .map(|layer| (layer.layer_id.clone(), layer.clone()))
        .collect::<BTreeMap<_, _>>();
    let palette = base
        .palette
        .iter()
        .map(|entry| entry.tile_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut changed = Vec::new();
    let mut collision_points = BTreeSet::new();
    let mut trigger_markers = BTreeSet::new();
    for op in &draft.proposed_operations {
        let (operation_id, layer_id, x, y, after) = match op {
            TilemapDraftOperation::Paint {
                operation_id,
                layer_id,
                x,
                y,
                tile_id,
            } => {
                if !palette.contains(tile_id.as_str()) {
                    return Err(anyhow!(
                        "tilemap draft paint tileId is not in palette: {tile_id}"
                    ));
                }
                (operation_id, layer_id, *x, *y, tile_id.clone())
            }
            TilemapDraftOperation::Erase {
                operation_id,
                layer_id,
                x,
                y,
            } => (operation_id, layer_id, *x, *y, ".".to_string()),
        };
        validate_identifier("tilemap draft operationId", operation_id)?;
        if !operation_ids.insert(operation_id.as_str()) {
            return Err(anyhow!(
                "duplicate tilemap draft operationId: {operation_id}"
            ));
        }
        if x >= base.width || y >= base.height {
            return Err(anyhow!(
                "tilemap draft operation {operation_id} is out of bounds"
            ));
        }
        let layer = by_layer
            .get_mut(layer_id)
            .ok_or_else(|| anyhow!("tilemap draft layerId not found: {layer_id}"))?;
        let before = layer.cells[y as usize][x as usize].clone();
        if before != after {
            layer.cells[y as usize][x as usize] = after.clone();
            changed.push(TilemapChangedCell {
                layer_id: layer_id.clone(),
                x,
                y,
                before,
                after,
            });
            if layer.kind == TilemapLayerKind::Collision {
                collision_points.insert(TilemapGridPoint { x, y });
            }
            for marker in base
                .markers
                .iter()
                .filter(|m| m.x == x && m.y == y && m.kind == TilemapMarkerKind::Trigger)
            {
                trigger_markers.insert(marker.marker_id.clone());
            }
        }
    }
    let preview = TilemapDraftPreviewSummary {
        changed_cells: changed,
        affected_collision_cells: collision_points.into_iter().collect(),
        affected_trigger_markers: trigger_markers.into_iter().collect(),
        generated_preview_only: true,
    };
    if draft.preview_summary != preview {
        return Err(anyhow!("tilemap draft previewSummary drift"));
    }
    match draft.validation_status {
        TilemapDraftValidationStatus::Drafted if !draft.blocked_reasons.is_empty() => Err(anyhow!(
            "drafted tilemap draft must not include blockedReasons"
        )),
        TilemapDraftValidationStatus::Blocked if draft.blocked_reasons.is_empty() => {
            Err(anyhow!("blocked tilemap draft requires blockedReasons"))
        }
        _ => Ok(preview),
    }
}
fn validate_palette(values: &[TilemapPaletteEntry]) -> Result<BTreeSet<&str>> {
    if values.is_empty() {
        return Err(anyhow!("tilemap source palette must not be empty"));
    }
    let mut ids = BTreeSet::new();
    for entry in values {
        validate_identifier("tilemap source palette.tileId", &entry.tile_id)?;
        if !ids.insert(entry.tile_id.as_str()) {
            return Err(anyhow!(
                "duplicate tilemap source palette tileId: {}",
                entry.tile_id
            ));
        }
        if entry.label.trim().is_empty() {
            return Err(anyhow!("tilemap source palette label must not be empty"));
        }
    }
    Ok(ids)
}

fn validate_layers(
    width: u32,
    height: u32,
    palette: &BTreeSet<&str>,
    layers: &[TilemapLayer],
) -> Result<BTreeSet<String>> {
    if layers.is_empty() {
        return Err(anyhow!("tilemap source layers must not be empty"));
    }
    let mut ids = BTreeSet::new();
    for layer in layers {
        validate_identifier("tilemap source layerId", &layer.layer_id)?;
        if !ids.insert(layer.layer_id.clone()) {
            return Err(anyhow!(
                "duplicate tilemap source layerId: {}",
                layer.layer_id
            ));
        }
        if layer.cells.len() != height as usize {
            return Err(anyhow!(
                "tilemap source layer {} height must match map height",
                layer.layer_id
            ));
        }
        for (y, row) in layer.cells.iter().enumerate() {
            if row.len() != width as usize {
                return Err(anyhow!(
                    "tilemap source layer {} row {y} width must match map width",
                    layer.layer_id
                ));
            }
            for tile in row {
                if tile != "." && !palette.contains(tile.as_str()) {
                    return Err(anyhow!(
                        "tilemap source layer {} references unknown tileId {tile}",
                        layer.layer_id
                    ));
                }
            }
        }
    }
    Ok(ids)
}

fn validate_markers(width: u32, height: u32, markers: &[TilemapMarker]) -> Result<()> {
    let mut ids = BTreeSet::new();
    for marker in markers {
        validate_identifier("tilemap markerId", &marker.marker_id)?;
        if !ids.insert(marker.marker_id.as_str()) {
            return Err(anyhow!("duplicate tilemap markerId: {}", marker.marker_id));
        }
        if marker.x >= width || marker.y >= height {
            return Err(anyhow!(
                "tilemap marker {} is out of bounds",
                marker.marker_id
            ));
        }
    }
    Ok(())
}

fn validate_source_path(path: &str) -> Result<()> {
    validate_repo_relative_ref("tilemap source path", path)?;
    if !path.starts_with(TILEMAP_SOURCE_PATH_PREFIX) || !path.ends_with(TILEMAP_SOURCE_PATH_SUFFIX)
    {
        return Err(anyhow!(
            "tilemap source path must follow {TILEMAP_SOURCE_PATH_PREFIX}<map-id>{TILEMAP_SOURCE_PATH_SUFFIX}"
        ));
    }
    Ok(())
}

fn validate_repo_relative_ref(label: &str, value: &str) -> Result<()> {
    if value.is_empty() || value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{label} must be a safe repo-relative path"));
    }
    Ok(())
}

fn validate_identifier(label: &str, value: &str) -> Result<()> {
    let valid = !value.is_empty()
        && value.len() <= 96
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
    if !valid {
        return Err(anyhow!("{label} must be a bounded identifier"));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum TilemapReachabilityDiagnostic {
    MissingSpawn,
    MissingObjective,
    SpawnBlocked,
    ObjectiveBlocked,
    ObjectiveUnreachable,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TilemapReachabilityStatus {
    Passed,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapReachabilityReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "mapId")]
    pub map_id: String,
    #[serde(rename = "targetTilemapRef")]
    pub target_tilemap_ref: String,
    pub status: TilemapReachabilityStatus,
    pub diagnostics: Vec<TilemapReachabilityDiagnostic>,
    #[serde(rename = "objectivePath")]
    pub objective_path: Vec<TilemapGridPoint>,
    pub boundary: String,
}

pub fn evaluate_tilemap_reachability(map: &TilemapSourceArtifact) -> TilemapReachabilityReport {
    let mut diagnostics = BTreeSet::new();
    let spawn = map
        .markers
        .iter()
        .find(|m| m.kind == TilemapMarkerKind::Spawn);
    let objective = map
        .markers
        .iter()
        .find(|m| m.kind == TilemapMarkerKind::Objective);
    let mut path = Vec::new();
    match (spawn, objective) {
        (None, _) => {
            diagnostics.insert(TilemapReachabilityDiagnostic::MissingSpawn);
        }
        (_, None) => {
            diagnostics.insert(TilemapReachabilityDiagnostic::MissingObjective);
        }
        (Some(spawn), Some(objective)) => {
            if map.is_blocked(spawn.x, spawn.y) {
                diagnostics.insert(TilemapReachabilityDiagnostic::SpawnBlocked);
            }
            if map.is_blocked(objective.x, objective.y) {
                diagnostics.insert(TilemapReachabilityDiagnostic::ObjectiveBlocked);
            }
            if diagnostics.is_empty() {
                path = bfs_path(map, (spawn.x, spawn.y), (objective.x, objective.y));
                if path.is_empty() {
                    diagnostics.insert(TilemapReachabilityDiagnostic::ObjectiveUnreachable);
                }
            }
        }
    }
    let diagnostics: Vec<_> = diagnostics.into_iter().collect();
    TilemapReachabilityReport {
        schema_version: TILEMAP_REACHABILITY_SCHEMA_VERSION.to_string(),
        map_id: map.map_id.clone(),
        target_tilemap_ref: map.source_path.clone(),
        status: if diagnostics.is_empty() {
            TilemapReachabilityStatus::Passed
        } else {
            TilemapReachabilityStatus::Blocked
        },
        diagnostics,
        objective_path: path,
        boundary: "Rust/local reachability evidence only; no browser trusted writes, no auto-apply, no gameplay quality claim.".to_string(),
    }
}

pub fn tilemap_reachability_report_from_json_str(
    map_json: &str,
) -> Result<TilemapReachabilityReport> {
    let map = TilemapSourceArtifact::from_json_str(map_json)?;
    Ok(evaluate_tilemap_reachability(&map))
}

fn bfs_path(
    map: &TilemapSourceArtifact,
    start: (u32, u32),
    goal: (u32, u32),
) -> Vec<TilemapGridPoint> {
    let mut queue = VecDeque::new();
    let mut seen = BTreeSet::new();
    let mut prev = BTreeMap::new();
    queue.push_back(start);
    seen.insert(start);
    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == goal {
            break;
        }
        for (nx, ny) in neighbors(map.width, map.height, x, y) {
            if map.is_blocked(nx, ny) || !seen.insert((nx, ny)) {
                continue;
            }
            prev.insert((nx, ny), (x, y));
            queue.push_back((nx, ny));
        }
    }
    if !seen.contains(&goal) {
        return Vec::new();
    }
    let mut cursor = goal;
    let mut out = vec![TilemapGridPoint {
        x: cursor.0,
        y: cursor.1,
    }];
    while cursor != start {
        cursor = prev[&cursor];
        out.push(TilemapGridPoint {
            x: cursor.0,
            y: cursor.1,
        });
    }
    out.reverse();
    out
}

fn neighbors(width: u32, height: u32, x: u32, y: u32) -> impl Iterator<Item = (u32, u32)> {
    let mut values = Vec::new();
    if x > 0 {
        values.push((x - 1, y));
    }
    if y > 0 {
        values.push((x, y - 1));
    }
    if x + 1 < width {
        values.push((x + 1, y));
    }
    if y + 1 < height {
        values.push((x, y + 1));
    }
    values.into_iter()
}

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const TILEMAP_SOURCE_SCHEMA_VERSION: &str = "ouroforge.tilemap-source.v1";
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
            return Err(anyhow!(
                "tilemap source pathConvention must name the shared dogfood tilemap path convention"
            ));
        }
        validate_repo_relative_ref("tilemap source tilesetRef", &self.tileset_ref)?;
        if self.width == 0 || self.height == 0 || self.width > 128 || self.height > 128 {
            return Err(anyhow!("tilemap source width/height must be 1..=128"));
        }
        if self.tile_size == 0 || self.tile_size > 256 {
            return Err(anyhow!("tilemap source tileSize must be 1..=256"));
        }
        let palette = validate_palette(&self.palette)?;
        validate_layers(self.width, self.height, &palette, &self.layers)?;
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
) -> Result<()> {
    if layers.is_empty() {
        return Err(anyhow!("tilemap source layers must not be empty"));
    }
    let mut ids = BTreeSet::new();
    for layer in layers {
        validate_identifier("tilemap source layerId", &layer.layer_id)?;
        if !ids.insert(layer.layer_id.as_str()) {
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
    Ok(())
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

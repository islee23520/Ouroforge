use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::{Component, Path};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static EVIDENCE_INDEX_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceArtifact {
    pub id: String,
    pub kind: String,
    pub path: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub added_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceIndex {
    pub artifacts: Vec<EvidenceArtifact>,
}

pub fn add_evidence_artifact(
    run_dir: impl AsRef<Path>,
    id: &str,
    kind: &str,
    path: &str,
    metadata: serde_json::Value,
) -> Result<EvidenceArtifact> {
    require_text("evidence artifact id", id)?;
    require_text("evidence artifact kind", kind)?;
    require_text("evidence artifact path", path)?;
    validate_evidence_artifact_path(path)?;

    let _guard = EVIDENCE_INDEX_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(|_| anyhow!("evidence index lock poisoned"))?;
    let mut index = read_evidence_index(&run_dir)?;
    if index.artifacts.iter().any(|artifact| artifact.id == id) {
        return Err(anyhow!("evidence artifact id already exists: {id}"));
    }

    let artifact = EvidenceArtifact {
        id: id.to_string(),
        kind: kind.to_string(),
        path: path.to_string(),
        metadata,
        added_at_unix_ms: unix_millis()?,
    };
    index.artifacts.push(artifact.clone());
    write_evidence_index(run_dir, &index)?;
    Ok(artifact)
}

pub fn list_evidence_artifacts(run_dir: impl AsRef<Path>) -> Result<Vec<EvidenceArtifact>> {
    Ok(read_evidence_index(run_dir)?.artifacts)
}

pub fn validate_evidence_artifact_path(path: &str) -> Result<()> {
    let evidence_path = Path::new(path);
    if evidence_path.is_absolute() {
        return Err(anyhow!("evidence artifact path must be relative"));
    }
    if !path.starts_with("evidence/") {
        return Err(anyhow!("evidence artifact path must start with evidence/"));
    }
    for component in evidence_path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => {
                return Err(anyhow!(
                    "evidence artifact path must stay inside the run evidence tree"
                ));
            }
        }
    }
    Ok(())
}

pub fn read_evidence_index(run_dir: impl AsRef<Path>) -> Result<EvidenceIndex> {
    let index_path = run_dir.as_ref().join("evidence/index.json");
    let input = fs::read_to_string(&index_path)
        .with_context(|| format!("failed to read evidence index {}", index_path.display()))?;
    let index: EvidenceIndex = serde_json::from_str(&input)
        .with_context(|| format!("failed to parse evidence index {}", index_path.display()))?;
    Ok(index)
}

pub fn write_evidence_index(run_dir: impl AsRef<Path>, index: &EvidenceIndex) -> Result<()> {
    write_json_atomic(&run_dir.as_ref().join("evidence/index.json"), &json!(index))
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}

fn write_json_atomic(path: &Path, value: &serde_json::Value) -> Result<()> {
    let body = serde_json::to_string_pretty(value).context("failed to serialize JSON")?;
    let temp_path = path.with_extension(format!(
        "json.tmp-{}-{}",
        std::process::id(),
        unix_millis()?
    ));
    fs::write(&temp_path, format!("{body}\n"))
        .with_context(|| format!("failed to write {}", temp_path.display()))?;
    fs::rename(&temp_path, path).with_context(|| {
        format!(
            "failed to replace {} with {}",
            path.display(),
            temp_path.display()
        )
    })
}

fn unix_millis() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before UNIX_EPOCH")?
        .as_millis())
}

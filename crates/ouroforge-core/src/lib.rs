use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub id: String,
    pub title: String,
    pub goal: String,
    pub constraints: Constraints,
    pub acceptance: Vec<String>,
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Constraints {
    pub target: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub id: String,
    pub description: String,
}

impl Seed {
    pub fn from_yaml_str(input: &str) -> Result<Self> {
        let seed: Seed = serde_yaml::from_str(input).context("failed to parse Seed YAML")?;
        seed.validate()?;
        Ok(seed)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let input = fs::read_to_string(path)
            .with_context(|| format!("failed to read Seed file {}", path.display()))?;
        Self::from_yaml_str(&input)
    }

    pub fn validate(&self) -> Result<()> {
        require_text("id", &self.id)?;
        require_text("title", &self.title)?;
        require_text("goal", &self.goal)?;
        require_text("constraints.target", &self.constraints.target)?;

        if self.acceptance.is_empty() {
            return Err(anyhow!("acceptance must contain at least one item"));
        }
        for (index, item) in self.acceptance.iter().enumerate() {
            require_text(&format!("acceptance[{index}]"), item)?;
        }

        if self.scenarios.is_empty() {
            return Err(anyhow!("scenarios must contain at least one item"));
        }
        for (index, scenario) in self.scenarios.iter().enumerate() {
            require_text(&format!("scenarios[{index}].id"), &scenario.id)?;
            require_text(
                &format!("scenarios[{index}].description"),
                &scenario.description,
            )?;
        }

        Ok(())
    }
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}

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

pub fn append_ledger_event(
    run_dir: impl AsRef<Path>,
    kind: &str,
    actor: &str,
    payload: serde_json::Value,
) -> Result<serde_json::Value> {
    require_text("ledger event kind", kind)?;
    require_text("ledger event actor", actor)?;

    let event = json!({
        "event": kind,
        "actor": actor,
        "payload": payload,
        "created_at_unix_ms": unix_millis()?,
    });
    let ledger_path = run_dir.as_ref().join("ledger.jsonl");
    let mut file = OpenOptions::new()
        .create(false)
        .append(true)
        .open(&ledger_path)
        .with_context(|| format!("failed to open ledger for append {}", ledger_path.display()))?;
    let line = serde_json::to_string(&event).context("failed to serialize ledger event")?;
    writeln!(file, "{line}").context("failed to append ledger event")?;
    Ok(event)
}

pub fn read_ledger_events(run_dir: impl AsRef<Path>) -> Result<Vec<serde_json::Value>> {
    let ledger_path = run_dir.as_ref().join("ledger.jsonl");
    let file = File::open(&ledger_path)
        .with_context(|| format!("failed to read ledger {}", ledger_path.display()))?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.with_context(|| {
            format!(
                "failed to read ledger line {} from {}",
                line_number + 1,
                ledger_path.display()
            )
        })?;
        if line.trim().is_empty() {
            continue;
        }
        let event: serde_json::Value = serde_json::from_str(&line).with_context(|| {
            format!(
                "failed to parse ledger JSON on line {} from {}",
                line_number + 1,
                ledger_path.display()
            )
        })?;
        events.push(event);
    }

    Ok(events)
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

fn read_evidence_index(run_dir: impl AsRef<Path>) -> Result<EvidenceIndex> {
    let index_path = run_dir.as_ref().join("evidence/index.json");
    let input = fs::read_to_string(&index_path)
        .with_context(|| format!("failed to read evidence index {}", index_path.display()))?;
    let index: EvidenceIndex = serde_json::from_str(&input)
        .with_context(|| format!("failed to parse evidence index {}", index_path.display()))?;
    Ok(index)
}

fn write_evidence_index(run_dir: impl AsRef<Path>, index: &EvidenceIndex) -> Result<()> {
    write_json_atomic(&run_dir.as_ref().join("evidence/index.json"), &json!(index))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunArtifacts {
    pub run_dir: PathBuf,
}

pub fn create_run(
    seed_path: impl AsRef<Path>,
    runs_root: impl AsRef<Path>,
) -> Result<RunArtifacts> {
    let seed_path = seed_path.as_ref();
    let runs_root = runs_root.as_ref();
    let seed_yaml = fs::read_to_string(seed_path)
        .with_context(|| format!("failed to read Seed file {}", seed_path.display()))?;
    let seed = Seed::from_yaml_str(&seed_yaml)?;

    fs::create_dir_all(runs_root)
        .with_context(|| format!("failed to create runs root {}", runs_root.display()))?;

    let created_at_unix_ms = unix_millis()?;
    let run_id = format!("run-{created_at_unix_ms}-{}", std::process::id());
    let run_dir = runs_root.join(&run_id);
    fs::create_dir(&run_dir)
        .with_context(|| format!("failed to create run directory {}", run_dir.display()))?;
    fs::create_dir(run_dir.join("evidence")).context("failed to create evidence directory")?;

    write_json(
        &run_dir.join("run.json"),
        &json!({
            "id": run_id,
            "seed_id": seed.id,
            "seed_title": seed.title,
            "status": "created",
            "created_at_unix_ms": created_at_unix_ms,
        }),
    )?;
    fs::write(run_dir.join("seed.snapshot.yaml"), seed_yaml)
        .context("failed to write seed snapshot")?;
    write_ledger_created(&run_dir.join("ledger.jsonl"), created_at_unix_ms)?;
    fs::write(run_dir.join("journal.md"), initial_journal()).context("failed to write journal")?;
    write_json(
        &run_dir.join("verdict.json"),
        &json!({ "status": "pending" }),
    )?;
    write_evidence_index(
        &run_dir,
        &EvidenceIndex {
            artifacts: Vec::new(),
        },
    )?;

    Ok(RunArtifacts { run_dir })
}

fn unix_millis() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before UNIX_EPOCH")?
        .as_millis())
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<()> {
    let body = serde_json::to_string_pretty(value).context("failed to serialize JSON")?;
    fs::write(path, format!("{body}\n"))
        .with_context(|| format!("failed to write {}", path.display()))
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

fn write_ledger_created(path: &Path, created_at_unix_ms: u128) -> Result<()> {
    let mut file =
        File::create(path).with_context(|| format!("failed to write {}", path.display()))?;
    let line = serde_json::to_string(&json!({
        "event": "run.created",
        "created_at_unix_ms": created_at_unix_ms,
    }))
    .context("failed to serialize ledger event")?;
    writeln!(file, "{line}").context("failed to write ledger event")
}

fn initial_journal() -> &'static str {
    "# Ouroforge Run Journal\n\n## Seed\n\n## Hypothesis\n\n## Observations\n\n## Evidence\n\n## Verdict\n\n## Next Mutation\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_SEED: &str = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

    #[test]
    fn parses_valid_seed() {
        let seed = Seed::from_yaml_str(VALID_SEED).expect("valid seed parses");
        assert_eq!(seed.id, "platformer.v0");
        assert_eq!(seed.constraints.target, "file-harness");
    }

    #[test]
    fn rejects_seed_missing_required_target() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints: {}
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("missing target fails");
        assert!(error.to_string().contains("failed to parse Seed YAML"));
    }

    #[test]
    fn rejects_seed_with_unknown_fields() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
future_scope: should-not-be-accepted
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("unknown fields fail");
        assert!(error.to_string().contains("failed to parse Seed YAML"));
    }

    #[test]
    fn creates_required_run_artifacts() {
        let root = unique_temp_dir("ouroforge-core-test");
        fs::create_dir_all(&root).expect("temp root exists");
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, VALID_SEED).expect("seed written");
        let runs_root = root.join("runs");

        let artifacts = create_run(&seed_path, &runs_root).expect("run artifacts created");

        assert!(artifacts.run_dir.join("run.json").is_file());
        assert!(artifacts.run_dir.join("seed.snapshot.yaml").is_file());
        assert!(artifacts.run_dir.join("ledger.jsonl").is_file());
        assert!(artifacts.run_dir.join("journal.md").is_file());
        assert!(artifacts.run_dir.join("verdict.json").is_file());
        assert!(artifacts.run_dir.join("evidence/index.json").is_file());

        let ledger = fs::read_to_string(artifacts.run_dir.join("ledger.jsonl")).unwrap();
        let first_event: serde_json::Value =
            serde_json::from_str(ledger.lines().next().unwrap()).unwrap();
        assert_eq!(first_event["event"], "run.created");

        let evidence = fs::read_to_string(artifacts.run_dir.join("evidence/index.json")).unwrap();
        let evidence_index: serde_json::Value = serde_json::from_str(&evidence).unwrap();
        assert_eq!(evidence_index["artifacts"].as_array().unwrap().len(), 0);

        let journal = fs::read_to_string(artifacts.run_dir.join("journal.md")).unwrap();
        for heading in [
            "## Seed",
            "## Hypothesis",
            "## Observations",
            "## Evidence",
            "## Verdict",
            "## Next Mutation",
        ] {
            assert!(journal.contains(heading), "journal missing {heading}");
        }

        let verdict = fs::read_to_string(artifacts.run_dir.join("verdict.json")).unwrap();
        let verdict_json: serde_json::Value = serde_json::from_str(&verdict).unwrap();
        assert_eq!(verdict_json["status"], "pending");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn appends_and_reads_ledger_events() {
        let (root, artifacts) = create_test_run("ouroforge-ledger-test");

        append_ledger_event(
            &artifacts.run_dir,
            "test.event",
            "test",
            json!({ "ok": true }),
        )
        .expect("first event appended");
        append_ledger_event(&artifacts.run_dir, "test.second", "test", json!({}))
            .expect("second event appended");

        let events = read_ledger_events(&artifacts.run_dir).expect("ledger reads");
        assert_eq!(events.len(), 3);
        assert_eq!(events[0]["event"], "run.created");
        assert_eq!(events[1]["event"], "test.event");
        assert_eq!(events[1]["actor"], "test");
        assert_eq!(events[1]["payload"], json!({ "ok": true }));
        assert_eq!(events[2]["event"], "test.second");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_malformed_ledger_events() {
        let (root, artifacts) = create_test_run("ouroforge-bad-ledger-test");
        fs::write(artifacts.run_dir.join("ledger.jsonl"), "not-json\n")
            .expect("bad ledger written");

        let error = read_ledger_events(&artifacts.run_dir).expect_err("bad ledger fails");
        assert!(error.to_string().contains("failed to parse ledger JSON"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn adds_and_lists_evidence_artifacts() {
        let (root, artifacts) = create_test_run("ouroforge-evidence-test");

        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/artifact-1.txt",
            json!({ "source": "unit-test" }),
        )
        .expect("first evidence added");
        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-2",
            "application/json",
            "evidence/artifact-2.json",
            json!({}),
        )
        .expect("second evidence added");

        let artifacts_list = list_evidence_artifacts(&artifacts.run_dir).expect("evidence lists");
        assert_eq!(artifacts_list.len(), 2);
        assert_eq!(artifacts_list[0].id, "artifact-1");
        assert_eq!(artifacts_list[0].metadata, json!({ "source": "unit-test" }));
        assert_eq!(artifacts_list[1].kind, "application/json");

        let index = fs::read_to_string(artifacts.run_dir.join("evidence/index.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&index).unwrap();
        assert_eq!(parsed["artifacts"].as_array().unwrap().len(), 2);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_malformed_evidence_index_and_duplicate_ids() {
        let (root, artifacts) = create_test_run("ouroforge-bad-evidence-test");

        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/artifact-1.txt",
            json!({}),
        )
        .expect("evidence added");
        let duplicate = add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/duplicate.txt",
            json!({}),
        )
        .expect_err("duplicate id fails");
        assert!(duplicate.to_string().contains("already exists"));

        fs::write(
            artifacts.run_dir.join("evidence/index.json"),
            r#"{"artifacts":"not-an-array"}"#,
        )
        .expect("bad evidence index written");
        let error = list_evidence_artifacts(&artifacts.run_dir).expect_err("bad index fails");
        assert!(error.to_string().contains("failed to parse evidence index"));

        fs::remove_dir_all(root).ok();
    }

    fn create_test_run(prefix: &str) -> (PathBuf, RunArtifacts) {
        let root = unique_temp_dir(prefix);
        fs::create_dir_all(&root).expect("temp root exists");
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, VALID_SEED).expect("seed written");
        let runs_root = root.join("runs");
        let artifacts = create_run(&seed_path, &runs_root).expect("run artifacts created");
        (root, artifacts)
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{}",
            std::process::id(),
            unix_millis().expect("time works")
        ))
    }
}

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File};
use std::io::Write;
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
    write_json(
        &run_dir.join("evidence/index.json"),
        &json!({ "artifacts": [] }),
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

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{}",
            std::process::id(),
            unix_millis().expect("time works")
        ))
    }
}

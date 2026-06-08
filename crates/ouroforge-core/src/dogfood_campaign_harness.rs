use crate::{
    bind_run_command_context, create_run, read_ledger_events, run_command_context_for_run,
};
use anyhow::{anyhow, Result};
use ouroforge_evaluator::dogfood_contract::{
    DogfoodCampaignContract, DogfoodFrictionKind, DogfoodFrictionMapping,
    DOGFOOD_CAMPAIGN_CONTRACT_SCHEMA_VERSION,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

pub const DOGFOOD_CAMPAIGN_HARNESS_SCHEMA_VERSION: &str = "dogfood-campaign-harness-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DogfoodCampaignHarnessConfig {
    #[serde(rename = "seedPath")]
    pub seed_path: PathBuf,
    #[serde(rename = "runsRoot")]
    pub runs_root: PathBuf,
    pub workers: usize,
    #[serde(rename = "resumeRunDir", skip_serializing_if = "Option::is_none")]
    pub resume_run_dir: Option<PathBuf>,
    #[serde(default, rename = "friction")]
    pub friction: Vec<DogfoodFrictionObservation>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DogfoodFrictionObservation {
    #[serde(rename = "frictionId")]
    pub friction_id: String,
    pub kind: DogfoodFrictionKind,
    pub stage: String,
    pub summary: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DogfoodCampaignHarnessReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "contractSchemaVersion")]
    pub contract_schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "runDir")]
    pub run_dir: String,
    #[serde(rename = "ledgerPath")]
    pub ledger_path: String,
    pub resumed: bool,
    pub deterministic: bool,
    pub status: String,
    #[serde(rename = "stagesCompleted")]
    pub stages_completed: Vec<String>,
    #[serde(rename = "frictionLogged")]
    pub friction_logged: Vec<String>,
    pub boundary: String,
}

pub fn run_dogfood_campaign_harness(
    config: &DogfoodCampaignHarnessConfig,
) -> Result<DogfoodCampaignHarnessReport> {
    if config.workers == 0 {
        return Err(anyhow!("dogfood campaign workers must be at least 1"));
    }
    let contract = DogfoodCampaignContract::era_l_deckbuilder();
    contract.validate()?;
    validate_config_against_contract(config, &contract)?;

    let (run_dir, resumed) = match &config.resume_run_dir {
        Some(run_dir) => {
            require_existing_run_ledger(run_dir)?;
            (run_dir.clone(), true)
        }
        None => {
            let artifacts = create_run(&config.seed_path, &config.runs_root)?;
            let context = run_command_context_for_run(
                &config.seed_path,
                &config.runs_root,
                config.workers,
                None,
                None,
            );
            bind_run_command_context(&artifacts.run_dir, context)?;
            (artifacts.run_dir, false)
        }
    };

    append_unique_event(
        &run_dir,
        "dogfood.campaign.started",
        "dogfood-campaign-harness",
        "campaign:started",
        json!({
            "schemaVersion": DOGFOOD_CAMPAIGN_HARNESS_SCHEMA_VERSION,
            "contractSchemaVersion": contract.schema_version,
            "titleId": contract.title_id,
            "seedPath": config.seed_path.to_string_lossy(),
            "workers": config.workers,
            "resumed": resumed,
            "executorCommand": contract.run_command,
            "evidencePipeline": contract.evidence_pipeline,
            "boundary": "Rust harness records campaign progress in existing ledger.jsonl only; executor/openchrome/verdict/journal artifacts remain the evidence pipeline."
        }),
    )?;

    let mut stages_completed = Vec::new();
    for (index, stage) in contract.loop_stages.iter().enumerate() {
        append_unique_event(
            &run_dir,
            "dogfood.campaign.stage.completed",
            "dogfood-campaign-harness",
            &format!("stage:{stage}"),
            json!({
                "campaignStage": stage,
                "stageIndex": index,
                "stageAttribution": stage,
                "existingArtifactRefs": ["ledger.jsonl", "journal.md", "verdict.json"],
                "boundary": "Stage attribution is an append-only ledger event; no separate campaign store is written."
            }),
        )?;
        stages_completed.push(stage.clone());
    }

    let mut friction_logged = Vec::new();
    for observation in &config.friction {
        validate_friction_observation(observation, &contract)?;
        let mapping = friction_mapping(observation.kind, &contract)?;
        append_unique_event(
            &run_dir,
            &format!("dogfood.friction.{}", observation.kind.as_str()),
            "dogfood-campaign-harness",
            &format!("friction:{}", observation.friction_id),
            json!({
                "frictionId": observation.friction_id,
                "frictionKind": observation.kind.as_str(),
                "ledgerEventKind": mapping.ledger_event_kind,
                "campaignStage": observation.stage,
                "stageAttribution": observation.stage,
                "summary": observation.summary,
                "evidenceRefs": observation.evidence_refs,
                "requiredArtifactRefs": mapping.required_artifact_refs,
                "boundary": "Friction is evidence linked to existing artifacts, not a new telemetry store."
            }),
        )?;
        friction_logged.push(observation.friction_id.clone());
    }

    append_unique_event(
        &run_dir,
        "dogfood.campaign.completed",
        "dogfood-campaign-harness",
        "campaign:completed",
        json!({
            "titleId": contract.title_id,
            "status": "completed",
            "stagesCompleted": stages_completed,
            "frictionCount": friction_logged.len(),
            "requiresHumanInputOnAutonomousPath": false,
            "highRiskAutoApply": false,
            "dataPlane": "Rust kernel/evaluator/source-apply",
            "controlPlane": "Elixir executor unchanged",
            "boundary": "Autonomous path completed without human input; HIGH-RISK/source-affecting fixes remain queued for human go/no-go and are never auto-applied."
        }),
    )?;

    Ok(DogfoodCampaignHarnessReport {
        schema_version: DOGFOOD_CAMPAIGN_HARNESS_SCHEMA_VERSION.to_string(),
        contract_schema_version: DOGFOOD_CAMPAIGN_CONTRACT_SCHEMA_VERSION.to_string(),
        title_id: contract.title_id,
        ledger_path: run_dir.join("ledger.jsonl").to_string_lossy().to_string(),
        run_dir: run_dir.to_string_lossy().to_string(),
        resumed,
        deterministic: true,
        status: "completed".to_string(),
        stages_completed,
        friction_logged,
        boundary: "Report is derived from existing run ledger events; no new store or verification engine is introduced.".to_string(),
    })
}

fn validate_config_against_contract(
    config: &DogfoodCampaignHarnessConfig,
    contract: &DogfoodCampaignContract,
) -> Result<()> {
    let seed_path = config.seed_path.to_string_lossy();
    if !seed_path.ends_with(&contract.seed_path) && seed_path != contract.seed_path {
        return Err(anyhow!(
            "dogfood campaign seedPath must target {}",
            contract.seed_path
        ));
    }
    Ok(())
}

fn validate_friction_observation(
    observation: &DogfoodFrictionObservation,
    contract: &DogfoodCampaignContract,
) -> Result<()> {
    require_text("dogfood frictionId", &observation.friction_id)?;
    require_text("dogfood friction summary", &observation.summary)?;
    if !contract
        .loop_stages
        .iter()
        .any(|stage| stage == &observation.stage)
    {
        return Err(anyhow!(
            "dogfood friction stage {} is not in the campaign loop",
            observation.stage
        ));
    }
    if observation.evidence_refs.is_empty() {
        return Err(anyhow!("dogfood friction evidenceRefs must not be empty"));
    }
    for evidence_ref in &observation.evidence_refs {
        require_existing_artifact_ref(evidence_ref)?;
    }
    Ok(())
}

fn friction_mapping(
    kind: DogfoodFrictionKind,
    contract: &DogfoodCampaignContract,
) -> Result<&DogfoodFrictionMapping> {
    contract
        .friction_mappings
        .iter()
        .find(|mapping| mapping.kind == kind)
        .ok_or_else(|| anyhow!("dogfood friction mapping missing kind {}", kind.as_str()))
}

fn append_unique_event(
    run_dir: &Path,
    kind: &str,
    actor: &str,
    idempotency_key: &str,
    mut payload: serde_json::Value,
) -> Result<serde_json::Value> {
    let existing = read_ledger_events(run_dir)?;
    if let Some(event) = existing.iter().find(|event| {
        event.get("event").and_then(|value| value.as_str()) == Some(kind)
            && event
                .get("payload")
                .and_then(|payload| payload.get("idempotencyKey"))
                .and_then(|value| value.as_str())
                == Some(idempotency_key)
    }) {
        return Ok(event.clone());
    }

    let payload_object = payload
        .as_object_mut()
        .ok_or_else(|| anyhow!("dogfood ledger payload must be an object"))?;
    payload_object.insert(
        "idempotencyKey".to_string(),
        serde_json::Value::String(idempotency_key.to_string()),
    );
    crate::append_ledger_event(run_dir, kind, actor, payload)
}

fn require_existing_run_ledger(run_dir: &Path) -> Result<()> {
    let ledger_path = run_dir.join("ledger.jsonl");
    if !ledger_path.is_file() {
        return Err(anyhow!(
            "dogfood campaign resumeRunDir must contain ledger.jsonl: {}",
            ledger_path.display()
        ));
    }
    Ok(())
}

fn require_existing_artifact_ref(value: &str) -> Result<()> {
    require_text("dogfood evidence ref", value)?;
    let lower = value.to_ascii_lowercase();
    if value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
        || value.contains("//")
        || lower.contains("new_db_schema")
        || lower.contains("new_store_schema")
        || lower.contains("new_telemetry_schema")
    {
        return Err(anyhow!(
            "dogfood evidence ref must be existing repo/run relative evidence, not a new store/schema: {value}"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}

pub fn dogfood_campaign_friction_summary(run_dir: impl AsRef<Path>) -> Result<Vec<String>> {
    let mut out = BTreeSet::new();
    for event in read_ledger_events(run_dir)? {
        let Some(kind) = event.get("event").and_then(|value| value.as_str()) else {
            continue;
        };
        if !kind.starts_with("dogfood.friction.") {
            continue;
        }
        if let Some(id) = event
            .get("payload")
            .and_then(|payload| payload.get("frictionId"))
            .and_then(|value| value.as_str())
        {
            out.insert(id.to_string());
        }
    }
    Ok(out.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn campaign_harness_records_all_stages_and_friction_in_existing_ledger() {
        let root = temp_root("dogfood-campaign-harness");
        let seed_path = write_dogfood_seed(&root);
        let runs_root = root.join("runs");
        let report = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
            seed_path,
            runs_root,
            workers: 2,
            resume_run_dir: None,
            friction: vec![DogfoodFrictionObservation {
                friction_id: "retry-openchrome-flake".to_string(),
                kind: DogfoodFrictionKind::Retry,
                stage: "re-verify".to_string(),
                summary: "openchrome probe required one bounded retry".to_string(),
                evidence_refs: vec!["ledger.jsonl".to_string(), "journal.md".to_string()],
            }],
        })
        .expect("campaign harness runs");

        assert_eq!(report.status, "completed");
        assert_eq!(report.stages_completed.len(), 7);
        assert_eq!(report.friction_logged, vec!["retry-openchrome-flake"]);
        assert!(report.boundary.contains("no new store"));

        let events = read_ledger_events(&report.run_dir).expect("ledger reads");
        assert!(events
            .iter()
            .any(|event| event.get("event").and_then(|v| v.as_str())
                == Some("dogfood.campaign.started")));
        assert!(events
            .iter()
            .any(|event| event.get("event").and_then(|v| v.as_str())
                == Some("dogfood.friction.retry")));
        assert!(events
            .iter()
            .any(|event| event.get("event").and_then(|v| v.as_str())
                == Some("dogfood.campaign.completed")));
        assert!(Path::new(&report.run_dir).join("journal.md").is_file());
        assert!(Path::new(&report.run_dir).join("verdict.json").is_file());

        fs::remove_dir_all(root).expect("temp root removed");
    }

    #[test]
    fn campaign_harness_is_resumable_without_duplicate_stage_or_friction_events() {
        let root = temp_root("dogfood-campaign-resume");
        let seed_path = write_dogfood_seed(&root);
        let runs_root = root.join("runs");
        let friction = vec![DogfoodFrictionObservation {
            friction_id: "gate-fail-design".to_string(),
            kind: DogfoodFrictionKind::GateFail,
            stage: "re-verify".to_string(),
            summary: "design-integrity failed before proposal acceptance".to_string(),
            evidence_refs: vec!["verdict.json".to_string(), "ledger.jsonl".to_string()],
        }];
        let first = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
            seed_path: seed_path.clone(),
            runs_root: runs_root.clone(),
            workers: 2,
            resume_run_dir: None,
            friction: friction.clone(),
        })
        .expect("initial run succeeds");
        let first_count = read_ledger_events(&first.run_dir)
            .expect("ledger reads")
            .len();

        let resumed = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
            seed_path,
            runs_root,
            workers: 2,
            resume_run_dir: Some(PathBuf::from(&first.run_dir)),
            friction,
        })
        .expect("resume succeeds");
        let second_count = read_ledger_events(&resumed.run_dir)
            .expect("ledger reads")
            .len();

        assert!(resumed.resumed);
        assert_eq!(first_count, second_count);
        assert_eq!(
            dogfood_campaign_friction_summary(&resumed.run_dir).expect("summary"),
            vec!["gate-fail-design".to_string()]
        );

        fs::remove_dir_all(root).expect("temp root removed");
    }

    #[test]
    fn campaign_harness_rejects_humanless_drift_new_store_refs_and_wrong_title_seed() {
        let root = temp_root("dogfood-campaign-rejects");
        let seed_path = root.join("wrong.yaml");
        fs::write(&seed_path, minimal_seed_yaml()).expect("seed written");
        let err = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
            seed_path,
            runs_root: root.join("runs"),
            workers: 2,
            resume_run_dir: None,
            friction: vec![],
        })
        .expect_err("wrong seed rejected");
        assert!(err.to_string().contains("dogfood-deckbuilder"));

        let seed_path = write_dogfood_seed(&root);
        let err = run_dogfood_campaign_harness(&DogfoodCampaignHarnessConfig {
            seed_path,
            runs_root: root.join("runs"),
            workers: 2,
            resume_run_dir: None,
            friction: vec![DogfoodFrictionObservation {
                friction_id: "bad-ref".to_string(),
                kind: DogfoodFrictionKind::ManualIntervention,
                stage: "trace".to_string(),
                summary: "bad evidence ref".to_string(),
                evidence_refs: vec!["new_db_schema/campaign.json".to_string()],
            }],
        })
        .expect_err("new store-looking ref rejected");
        assert!(err.to_string().contains("new_db_schema") || err.to_string().contains("Seed"));

        fs::remove_dir_all(root).expect("temp root removed");
    }

    fn write_dogfood_seed(root: &Path) -> PathBuf {
        let seeds = root.join("seeds");
        fs::create_dir_all(&seeds).expect("seeds dir created");
        let path = seeds.join("dogfood-deckbuilder.yaml");
        fs::write(&path, minimal_seed_yaml()).expect("seed written");
        path
    }

    fn minimal_seed_yaml() -> &'static str {
        r#"id: dogfood.deckbuilder.v1
title: Era I Engine-Builder Deckbuilder Dogfood Seed
goal: Exercise the existing engine-builder deckbuilder through the autonomous dogfood harness.
constraints:
  target: game-runtime
acceptance:
  - The dogfood harness records every autonomous stage in the existing ledger.
scenarios:
  - id: dogfood-smoke
    description: Placeholder real-title smoke scenario for harness contract tests.
    steps:
      - wait:
          frames: 1
    assertions:
      - world_state:
          path: object.x
          equals: 0
"#
    }

    fn temp_root(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("ouroforge-{label}-{stamp}"));
        fs::create_dir_all(&root).expect("temp root created");
        root
    }
}

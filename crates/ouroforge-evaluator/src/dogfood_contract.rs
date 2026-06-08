use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const DOGFOOD_CAMPAIGN_CONTRACT_SCHEMA_VERSION: &str = "real-title-dogfooding-contract-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum DogfoodFrictionKind {
    Stall,
    Retry,
    ManualIntervention,
    BudgetHalt,
    GateFail,
}

impl DogfoodFrictionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stall => "stall",
            Self::Retry => "retry",
            Self::ManualIntervention => "manual-intervention",
            Self::BudgetHalt => "budget-halt",
            Self::GateFail => "gate-fail",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DogfoodFrictionMapping {
    pub kind: DogfoodFrictionKind,
    #[serde(rename = "ledgerEventKind")]
    pub ledger_event_kind: String,
    #[serde(rename = "requiredArtifactRefs")]
    pub required_artifact_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DogfoodCampaignContract {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "seedPath")]
    pub seed_path: String,
    #[serde(rename = "runCommand")]
    pub run_command: Vec<String>,
    #[serde(rename = "loopStages")]
    pub loop_stages: Vec<String>,
    #[serde(rename = "evidencePipeline")]
    pub evidence_pipeline: Vec<String>,
    #[serde(rename = "frictionMappings")]
    pub friction_mappings: Vec<DogfoodFrictionMapping>,
    #[serde(rename = "highRiskAutoApply")]
    pub high_risk_auto_apply: bool,
    #[serde(rename = "requiresHumanInputOnAutonomousPath")]
    pub requires_human_input_on_autonomous_path: bool,
    #[serde(rename = "dataPlane")]
    pub data_plane: String,
    #[serde(rename = "controlPlane")]
    pub control_plane: String,
    pub boundary: String,
}

impl DogfoodCampaignContract {
    pub fn era_l_deckbuilder() -> Self {
        Self {
            schema_version: DOGFOOD_CAMPAIGN_CONTRACT_SCHEMA_VERSION.to_string(),
            title_id: "era-i-engine-builder-deckbuilder".to_string(),
            seed_path: "seeds/dogfood-deckbuilder.yaml".to_string(),
            run_command: vec![
                "cargo".to_string(),
                "run".to_string(),
                "-p".to_string(),
                "ouroforge-cli".to_string(),
                "--".to_string(),
                "run".to_string(),
                "seeds/dogfood-deckbuilder.yaml".to_string(),
                "--workers".to_string(),
                "2".to_string(),
            ],
            loop_stages: [
                "detect",
                "explain",
                "trace",
                "attribute",
                "propose",
                "re-verify",
                "apply-or-queue",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            evidence_pipeline: [
                "openchrome",
                "scenario-verdicts",
                "four-gates",
                "design-integrity",
                "journal.md",
                "ledger.jsonl",
                "loop-coverage-attribution",
                "evolve",
                "source-apply",
                "trust-gradient",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            friction_mappings: vec![
                mapping(DogfoodFrictionKind::Stall, &["ledger.jsonl", "journal.md", "verdict.json"]),
                mapping(
                    DogfoodFrictionKind::Retry,
                    &["ledger.jsonl", "journal.md", "scenario-verdict", "openchrome"],
                ),
                mapping(
                    DogfoodFrictionKind::ManualIntervention,
                    &["ledger.jsonl", "journal.md", "trust-gradient"],
                ),
                mapping(
                    DogfoodFrictionKind::BudgetHalt,
                    &["ledger.jsonl", "journal.md", "loop-coverage-attribution", "verdict.json"],
                ),
                mapping(
                    DogfoodFrictionKind::GateFail,
                    &["ledger.jsonl", "journal.md", "scenario-verdict", "openchrome", "gate-verdict"],
                ),
            ],
            high_risk_auto_apply: false,
            requires_human_input_on_autonomous_path: false,
            data_plane: "Rust kernel/evaluator/source-apply".to_string(),
            control_plane: "Elixir executor unchanged".to_string(),
            boundary: "Autonomous self-validation reuses openchrome, scenario verdicts, the four gates plus design-integrity, journal.md, ledger.jsonl, loop-coverage attribution, evolve, source-apply, and trust-gradient; it adds no new verification engine or data plane, never auto-applies HIGH-RISK/source-affecting fixes, and leaves fun/taste plus release go/no-go human.".to_string(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != DOGFOOD_CAMPAIGN_CONTRACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "dogfood campaign contract schemaVersion must be {DOGFOOD_CAMPAIGN_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        require_exact(
            "dogfood titleId",
            &self.title_id,
            "era-i-engine-builder-deckbuilder",
        )?;
        require_exact(
            "dogfood seedPath",
            &self.seed_path,
            "seeds/dogfood-deckbuilder.yaml",
        )?;
        require_contains_all(
            "dogfood runCommand",
            &self.run_command,
            &[
                "cargo",
                "run",
                "ouroforge-cli",
                "seeds/dogfood-deckbuilder.yaml",
                "--workers",
                "2",
            ],
        )?;
        require_contains_all(
            "dogfood loopStages",
            &self.loop_stages,
            &[
                "detect",
                "explain",
                "trace",
                "attribute",
                "propose",
                "re-verify",
            ],
        )?;
        require_contains_all(
            "dogfood evidencePipeline",
            &self.evidence_pipeline,
            &[
                "openchrome",
                "scenario-verdicts",
                "four-gates",
                "design-integrity",
                "journal.md",
                "ledger.jsonl",
                "loop-coverage-attribution",
                "evolve",
                "source-apply",
                "trust-gradient",
            ],
        )?;
        if self.high_risk_auto_apply {
            return Err(anyhow!(
                "dogfood campaign must never auto-apply HIGH-RISK/source-affecting fixes"
            ));
        }
        if self.requires_human_input_on_autonomous_path {
            return Err(anyhow!(
                "dogfood autonomous path must not require human input"
            ));
        }
        if !self.data_plane.to_ascii_lowercase().contains("rust") {
            return Err(anyhow!("dogfood dataPlane must remain Rust/local"));
        }
        if !self.control_plane.to_ascii_lowercase().contains("elixir")
            || !self
                .control_plane
                .to_ascii_lowercase()
                .contains("unchanged")
        {
            return Err(anyhow!(
                "dogfood controlPlane must state the Elixir executor is unchanged"
            ));
        }
        for phrase in [
            "no new verification engine",
            "data plane",
            "never auto-applies",
            "human",
        ] {
            if !self.boundary.to_ascii_lowercase().contains(phrase) {
                return Err(anyhow!("dogfood boundary must mention {phrase}"));
            }
        }
        validate_friction_mappings(&self.friction_mappings)
    }
}

fn mapping(kind: DogfoodFrictionKind, refs: &[&str]) -> DogfoodFrictionMapping {
    DogfoodFrictionMapping {
        kind,
        ledger_event_kind: kind.as_str().to_string(),
        required_artifact_refs: refs.iter().map(|value| (*value).to_string()).collect(),
    }
}

fn validate_friction_mappings(mappings: &[DogfoodFrictionMapping]) -> Result<()> {
    let required = [
        DogfoodFrictionKind::Stall,
        DogfoodFrictionKind::Retry,
        DogfoodFrictionKind::ManualIntervention,
        DogfoodFrictionKind::BudgetHalt,
        DogfoodFrictionKind::GateFail,
    ];
    let seen = mappings
        .iter()
        .map(|mapping| mapping.kind)
        .collect::<BTreeSet<_>>();
    for kind in required {
        if !seen.contains(&kind) {
            return Err(anyhow!(
                "dogfood friction mapping missing kind {}",
                kind.as_str()
            ));
        }
    }
    for mapping in mappings {
        require_exact(
            "dogfood friction ledgerEventKind",
            &mapping.ledger_event_kind,
            mapping.kind.as_str(),
        )?;
        require_contains_all(
            "dogfood friction requiredArtifactRefs",
            &mapping.required_artifact_refs,
            &["ledger.jsonl", "journal.md"],
        )?;
    }
    Ok(())
}

fn require_exact(field: &str, actual: &str, expected: &str) -> Result<()> {
    if actual != expected {
        return Err(anyhow!("{field} must be {expected}"));
    }
    Ok(())
}

fn require_contains_all(field: &str, actual: &[String], expected: &[&str]) -> Result<()> {
    for expected_value in expected {
        if !actual.iter().any(|value| value == expected_value) {
            return Err(anyhow!("{field} missing {expected_value}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn era_l_deckbuilder_contract_validates_autonomy_invariants() {
        let contract = DogfoodCampaignContract::era_l_deckbuilder();

        contract.validate().expect("default contract is valid");
        assert_eq!(contract.title_id, "era-i-engine-builder-deckbuilder");
        assert!(!contract.requires_human_input_on_autonomous_path);
        assert!(!contract.high_risk_auto_apply);
        assert!(contract
            .evidence_pipeline
            .contains(&"ledger.jsonl".to_string()));
        assert!(contract
            .evidence_pipeline
            .contains(&"loop-coverage-attribution".to_string()));
    }

    #[test]
    fn rejects_new_data_plane_or_high_risk_auto_apply() {
        let mut contract = DogfoodCampaignContract::era_l_deckbuilder();
        contract.high_risk_auto_apply = true;
        assert!(contract.validate().is_err());

        let mut contract = DogfoodCampaignContract::era_l_deckbuilder();
        contract.data_plane = "new database verifier".to_string();
        assert!(contract.validate().is_err());
    }

    #[test]
    fn rejects_missing_friction_evidence_mapping() {
        let mut contract = DogfoodCampaignContract::era_l_deckbuilder();
        contract
            .friction_mappings
            .retain(|mapping| mapping.kind != DogfoodFrictionKind::GateFail);

        assert!(contract.validate().is_err());
    }
}

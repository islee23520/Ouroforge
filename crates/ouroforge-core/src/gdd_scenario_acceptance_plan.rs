use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_SCENARIO_ACCEPTANCE_PLAN_SCHEMA_VERSION: &str = "gdd-scenario-acceptance-plan-v1";
const SUPPORTED_ASSERTION_KINDS: &[&str] = &["flag", "event", "state", "objective", "evidence-ref"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddScenarioAcceptancePlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "requirementIds")]
    pub requirement_ids: Vec<String>,
    #[serde(rename = "mechanicsMappingIds")]
    pub mechanics_mapping_ids: Vec<String>,
    #[serde(rename = "feasibilityOutcomeRefs")]
    pub feasibility_outcome_refs: Vec<String>,
    #[serde(rename = "scenarioDrafts")]
    pub scenario_drafts: Vec<Value>,
    #[serde(rename = "requirementCoverage")]
    pub requirement_coverage: Vec<Value>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<Value>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddScenarioAcceptancePlanReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "requirementCount")]
    pub requirement_count: usize,
    #[serde(rename = "mechanicsMappingCount")]
    pub mechanics_mapping_count: usize,
    #[serde(rename = "scenarioDraftCount")]
    pub scenario_draft_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "assertionKindCounts")]
    pub assertion_kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddScenarioAcceptancePlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse GDD Scenario Acceptance Plan JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddScenarioAcceptancePlanReadModel {
        let mut assertion_kind_counts = BTreeMap::new();
        for draft in &self.scenario_drafts {
            if let Some(assertions) = draft
                .get("expectedFlagsEventsStates")
                .and_then(Value::as_array)
            {
                for assertion in assertions {
                    let kind = str_field(assertion, "kind").unwrap_or("unknown");
                    *assertion_kind_counts.entry(kind.to_string()).or_insert(0) += 1;
                }
            }
        }
        GddScenarioAcceptancePlanReadModel {
            schema_version: self.schema_version.clone(),
            plan_id: self.plan_id.clone(),
            status: self.status.clone(),
            requirement_count: self.requirement_ids.len(),
            mechanics_mapping_count: self.mechanics_mapping_ids.len(),
            scenario_draft_count: self.scenario_drafts.len(),
            blocked_count: self.blocked_count(),
            assertion_kind_counts,
            validation_summary: vec![
                "scenario drafts link acceptance ids, requirement ids, scenario ids, mechanics mapping ids, and feasibility outcomes".to_string(),
                "required setup, input/action steps, expected flags/events/states, and evidence expectations are draft data, not trusted tests".to_string(),
                "unsupported mechanics, unsupported assertions, contradictory acceptance criteria, unsafe refs, missing evidence, and stale targets fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no trusted test creation or apply authority".to_string(),
                "existing scenario packs, behavior contracts, evidence, dashboard, and Studio read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD scenario acceptance plan read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_SCENARIO_ACCEPTANCE_PLAN_SCHEMA_VERSION {
            return Err(anyhow!("GDD scenario acceptance plan schemaVersion must be {GDD_SCENARIO_ACCEPTANCE_PLAN_SCHEMA_VERSION}"));
        }
        require_id("GDD scenario acceptance plan planId", &self.plan_id)?;
        require_ref(
            "GDD scenario acceptance plan feasibilityGateRef",
            &self.feasibility_gate_ref,
        )?;
        validate_id_list(
            "GDD scenario acceptance plan requirementIds",
            &self.requirement_ids,
            true,
        )?;
        validate_id_list(
            "GDD scenario acceptance plan mechanicsMappingIds",
            &self.mechanics_mapping_ids,
            true,
        )?;
        validate_ref_list(
            "GDD scenario acceptance plan feasibilityOutcomeRefs",
            &self.feasibility_outcome_refs,
            true,
        )?;
        require_nonempty(
            "GDD scenario acceptance plan scenarioDrafts",
            self.scenario_drafts.len(),
        )?;
        require_nonempty(
            "GDD scenario acceptance plan requirementCoverage",
            self.requirement_coverage.len(),
        )?;
        require_nonempty(
            "GDD scenario acceptance plan expectedEvidence",
            self.expected_evidence.len(),
        )?;
        if self.scenario_drafts.len() > 24 {
            return Err(anyhow!("GDD scenario acceptance plan is overbroad for v1"));
        }
        let reqs: BTreeSet<&str> = self.requirement_ids.iter().map(String::as_str).collect();
        let maps: BTreeSet<&str> = self
            .mechanics_mapping_ids
            .iter()
            .map(String::as_str)
            .collect();
        let outcomes: BTreeSet<&str> = self
            .feasibility_outcome_refs
            .iter()
            .map(String::as_str)
            .collect();
        validate_scenario_drafts(&self.scenario_drafts, &reqs, &maps, &outcomes)?;
        validate_requirement_coverage(&self.requirement_coverage, &reqs)?;
        validate_evidence(&self.expected_evidence)?;
        validate_text_list(
            "GDD scenario acceptance plan blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let stale = self
            .scenario_drafts
            .iter()
            .any(|draft| bool_field(draft, "staleTarget"));
        let blocked = self.blocked_count() > 0;
        match self.status.as_str() {
            "ready" if stale || blocked => Err(anyhow!("ready GDD scenario acceptance plan must not include stale targets, unsupported checks, or blockers"))?,
            "partial" if !blocked => Err(anyhow!("partial GDD scenario acceptance plan requires missing coverage or blocked reasons"))?,
            "blocked" if !blocked => Err(anyhow!("blocked GDD scenario acceptance plan requires visible blocked reasons"))?,
            "stale" if !stale => Err(anyhow!("stale GDD scenario acceptance plan requires at least one staleTarget"))?,
            "ready" | "partial" | "blocked" | "stale" => {}
            _ => return Err(anyhow!("GDD scenario acceptance plan status must be ready, partial, blocked, or stale")),
        }
        require_text("GDD scenario acceptance plan boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "scenario drafts",
            "not trusted tests",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no autonomous unrestricted game creation",
            "no hidden implementation of unsupported checks",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD scenario acceptance plan boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .scenario_drafts
                .iter()
                .filter(|draft| {
                    bool_field(draft, "staleTarget")
                        || !array_empty(draft, "unsupportedChecks")
                        || !array_empty(draft, "blockedReasons")
                })
                .count()
            + self
                .requirement_coverage
                .iter()
                .filter(|coverage| str_field(coverage, "coverageStatus") != Some("covered"))
                .count()
    }
}

fn validate_scenario_drafts(
    values: &[Value],
    reqs: &BTreeSet<&str>,
    maps: &BTreeSet<&str>,
    outcomes: &BTreeSet<&str>,
) -> Result<()> {
    let mut acceptance_ids = BTreeSet::new();
    let mut scenario_ids = BTreeSet::new();
    let mut assertion_names = BTreeMap::new();
    for draft in values {
        let acceptance_id = required_id(draft, "acceptanceId")?;
        if !acceptance_ids.insert(acceptance_id.to_string()) {
            return Err(anyhow!(
                "GDD scenario acceptance plan acceptanceId `{acceptance_id}` is duplicated"
            ));
        }
        let requirement_id = required_str(draft, "requirementId")?;
        require_member(
            "GDD scenario acceptance plan scenarioDrafts.requirementId",
            requirement_id,
            reqs,
        )?;
        require_member(
            "GDD scenario acceptance plan scenarioDrafts.mechanicsMappingId",
            required_str(draft, "mechanicsMappingId")?,
            maps,
        )?;
        require_member_ref(
            "GDD scenario acceptance plan scenarioDrafts.feasibilityOutcomeRef",
            required_str(draft, "feasibilityOutcomeRef")?,
            outcomes,
        )?;
        let scenario_id = required_id(draft, "scenarioId")?;
        if !scenario_ids.insert(scenario_id.to_string()) {
            return Err(anyhow!(
                "GDD scenario acceptance plan scenarioId `{scenario_id}` is duplicated"
            ));
        }
        require_ref(
            "GDD scenario acceptance plan scenarioDrafts.scenarioRef",
            required_str(draft, "scenarioRef")?,
        )?;
        validate_text_values(
            "GDD scenario acceptance plan requiredSetup",
            draft.get("requiredSetup"),
            true,
        )?;
        validate_steps(draft.get("inputActionSteps"))?;
        validate_assertions(
            draft.get("expectedFlagsEventsStates"),
            &mut assertion_names,
            acceptance_id,
        )?;
        validate_evidence_refs(
            "GDD scenario acceptance plan scenarioDrafts.evidenceNeeded",
            draft.get("evidenceNeeded"),
        )?;
        let unsupported = string_array(
            draft.get("unsupportedChecks"),
            "GDD scenario acceptance plan scenarioDrafts.unsupportedChecks",
            false,
        )?;
        validate_text_list(
            "GDD scenario acceptance plan scenarioDrafts.unsupportedChecks",
            &unsupported,
            false,
        )?;
        let blockers = string_array(
            draft.get("blockedReasons"),
            "GDD scenario acceptance plan scenarioDrafts.blockedReasons",
            false,
        )?;
        validate_text_list(
            "GDD scenario acceptance plan scenarioDrafts.blockedReasons",
            &blockers,
            false,
        )?;
        if (!unsupported.is_empty() || bool_field(draft, "staleTarget")) && blockers.is_empty() {
            return Err(anyhow!("GDD scenario acceptance plan unsupported checks or stale targets require blockedReasons"));
        }
    }
    Ok(())
}
fn validate_requirement_coverage(values: &[Value], reqs: &BTreeSet<&str>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for coverage in values {
        let requirement_id = required_str(coverage, "requirementId")?;
        require_member(
            "GDD scenario acceptance plan requirementCoverage.requirementId",
            requirement_id,
            reqs,
        )?;
        if !seen.insert(requirement_id.to_string()) {
            return Err(anyhow!("GDD scenario acceptance plan duplicate coverage for requirement `{requirement_id}`"));
        }
        let status = required_str(coverage, "coverageStatus")?;
        if !["covered", "partial", "blocked", "unsupported"].contains(&status) {
            return Err(anyhow!(
                "GDD scenario acceptance plan unsupported coverageStatus `{status}`"
            ));
        }
        let blockers = string_array(
            coverage.get("blockedReasons"),
            "GDD scenario acceptance plan requirementCoverage.blockedReasons",
            false,
        )?;
        validate_text_list(
            "GDD scenario acceptance plan requirementCoverage.blockedReasons",
            &blockers,
            false,
        )?;
        if status != "covered" && blockers.is_empty() {
            return Err(anyhow!(
                "GDD scenario acceptance plan non-covered requirements require blockedReasons"
            ));
        }
    }
    Ok(())
}
fn validate_steps(value: Option<&Value>) -> Result<()> {
    let steps = value.and_then(Value::as_array).ok_or_else(|| {
        anyhow!("GDD scenario acceptance plan inputActionSteps must not be empty")
    })?;
    require_nonempty("GDD scenario acceptance plan inputActionSteps", steps.len())?;
    for step in steps {
        required_id(step, "stepId")?;
        require_text(
            "GDD scenario acceptance plan inputActionSteps.action",
            required_str(step, "action")?,
        )?;
        require_text(
            "GDD scenario acceptance plan inputActionSteps.rationale",
            required_str(step, "rationale")?,
        )?;
    }
    Ok(())
}
fn validate_assertions(
    value: Option<&Value>,
    assertion_names: &mut BTreeMap<String, String>,
    acceptance_id: &str,
) -> Result<()> {
    let assertions = value.and_then(Value::as_array).ok_or_else(|| {
        anyhow!("GDD scenario acceptance plan expectedFlagsEventsStates must not be empty")
    })?;
    require_nonempty(
        "GDD scenario acceptance plan expectedFlagsEventsStates",
        assertions.len(),
    )?;
    for assertion in assertions {
        let assertion_id = required_id(assertion, "assertionId")?;
        let kind = required_str(assertion, "kind")?;
        if !SUPPORTED_ASSERTION_KINDS.contains(&kind) {
            return Err(anyhow!(
                "GDD scenario acceptance plan unsupported assertion `{kind}`"
            ));
        }
        let name = required_str(assertion, "name")?;
        require_text(
            "GDD scenario acceptance plan expectedFlagsEventsStates.name",
            name,
        )?;
        let expected = required_str(assertion, "expected")?;
        require_text(
            "GDD scenario acceptance plan expectedFlagsEventsStates.expected",
            expected,
        )?;
        let key = format!("{kind}:{name}");
        if let Some(previous) = assertion_names.insert(key, expected.to_string()) {
            if previous != expected {
                return Err(anyhow!("GDD scenario acceptance plan has contradictory acceptance criteria for `{name}`"));
            }
        }
        if assertion_id == acceptance_id {
            return Err(anyhow!(
                "GDD scenario acceptance plan assertionId must differ from acceptanceId"
            ));
        }
    }
    Ok(())
}
fn validate_evidence(values: &[Value]) -> Result<()> {
    let mut ids = BTreeSet::new();
    for item in values {
        let id = required_id(item, "evidenceId")?;
        if !ids.insert(id.to_string()) {
            return Err(anyhow!(
                "GDD scenario acceptance plan evidenceId `{id}` is duplicated"
            ));
        }
        let path = required_str(item, "pathHint")?;
        require_ref(
            "GDD scenario acceptance plan expectedEvidence.pathHint",
            path,
        )?;
        if !path.contains("evidence") && !path.contains("scenario") {
            return Err(anyhow!("GDD scenario acceptance plan expectedEvidence must point to scenario/evidence refs"));
        }
        require_text(
            "GDD scenario acceptance plan expectedEvidence.description",
            required_str(item, "description")?,
        )?;
    }
    Ok(())
}
fn validate_evidence_refs(field: &str, value: Option<&Value>) -> Result<()> {
    let refs = string_array(value, field, true)?;
    for item in &refs {
        require_ref(field, item)?;
        if !item.contains("evidence") && !item.contains("scenario") {
            return Err(anyhow!(
                "{field} must reference scenario/evidence expectations"
            ));
        }
    }
    Ok(())
}
fn validate_text_values(field: &str, value: Option<&Value>, required: bool) -> Result<()> {
    let values = string_array(value, field, required)?;
    validate_text_list(field, &values, required)
}
fn array_empty(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|v| v.is_empty())
        .unwrap_or(true)
}
fn bool_field(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}
fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}
fn required_str<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    str_field(value, key)
        .ok_or_else(|| anyhow!("GDD scenario acceptance plan missing string field `{key}`"))
}
fn required_id<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    let id = required_str(value, key)?;
    require_id(&format!("GDD scenario acceptance plan {key}"), id)?;
    Ok(id)
}
fn string_array(value: Option<&Value>, field: &str, required: bool) -> Result<Vec<String>> {
    let Some(value) = value else {
        if required {
            return Err(anyhow!("{field} must not be empty"));
        } else {
            return Ok(vec![]);
        }
    };
    let array = value
        .as_array()
        .ok_or_else(|| anyhow!("{field} must be an array"))?;
    if required {
        require_nonempty(field, array.len())?;
    }
    array
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_string)
                .ok_or_else(|| anyhow!("{field} must contain strings"))
        })
        .collect()
}
fn validate_id_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    let mut seen = BTreeSet::new();
    for value in values {
        require_id(field, value)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate id `{value}`"));
        }
    }
    Ok(())
}
fn validate_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    let mut seen = BTreeSet::new();
    for value in values {
        require_ref(field, value)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate ref `{value}`"));
        }
    }
    Ok(())
}
fn validate_text_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}
fn require_member(field: &str, value: &str, allowed: &BTreeSet<&str>) -> Result<()> {
    require_id(field, value)?;
    if !allowed.contains(value) {
        return Err(anyhow!(
            "{field} `{value}` is missing from declared GDD requirements or mechanics mapping ids"
        ));
    }
    Ok(())
}
fn require_member_ref(field: &str, value: &str, allowed: &BTreeSet<&str>) -> Result<()> {
    require_ref(field, value)?;
    if !allowed.contains(value) {
        return Err(anyhow!(
            "{field} `{value}` is missing from declared feasibility gate outcomes"
        ));
    }
    Ok(())
}
fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
fn require_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!("{field} must be a bounded local id"));
    }
    Ok(())
}
fn require_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("data:") {
        return Err(anyhow!("{field} remote refs are not allowed"));
    }
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} contains forbidden traversal and must stay inside local fixture/reference roots"));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/")
        || value.starts_with("evidence/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, runs/, or evidence/ refs"
        ));
    }
    Ok(())
}
fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "dynamic code loading",
        "dynamic import",
        "command bridge",
        "local server bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "commercial readiness",
        "hosted/cloud",
        "native export",
        "plugin runtime",
        "autonomous unrestricted game creation",
        "trusted tests",
        "hidden implementation of unsupported checks",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD scenario authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}
fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    value.contains(phrase)
        && ![
            "no ",
            "not ",
            "without ",
            "avoid ",
            "forbid ",
            "forbidden ",
            "not yet ",
        ]
        .iter()
        .any(|prefix| value.contains(&format!("{prefix}{phrase}")))
}

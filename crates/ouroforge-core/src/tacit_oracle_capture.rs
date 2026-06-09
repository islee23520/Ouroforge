use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::legacy_logic_ingestion::{BehavioralUnitRecord, EraRHandoffState, FidelityGrade};

pub const TACIT_ORACLE_CAPTURE_SCHEMA_VERSION: &str = "tacit.oracle.capture.v1";
pub const TACIT_ORACLE_CAPTURE_BOUNDARY: &str = "source-independent tacit intent and oracle capture; one-way on-ramp; clean-room re-derivation; no source translation; no decompiled source copying; no engine runtime bridge; Rust-owned artifact truth; Studio presentation/gated-input only; no ported claim without passing oracle";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterrogationQuestion {
    pub id: String,
    pub unit_id: String,
    pub prompt: String,
    pub resolves: String,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TacitAnswerRecord {
    pub question_id: String,
    pub author: String,
    pub answer: String,
    pub confidence: AnswerConfidence,
    pub provenance_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservedBehaviorTrace {
    pub id: String,
    pub unit_id: String,
    pub stimulus: String,
    pub observed_events: Vec<String>,
    pub state_hash: String,
    pub source_provenance: String,
    #[serde(default)]
    pub secondary_render_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleCaptureReport {
    pub schema_version: String,
    pub boundary: String,
    pub deterministic_digest: String,
    pub sessions: Vec<InterrogationSessionReport>,
    pub oracle_specs: Vec<OracleSpec>,
    pub fidelity_report: OracleFidelityReport,
    pub re_derivation_tasks: Vec<OracleReDerivationTask>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterrogationSessionReport {
    pub unit_id: String,
    pub unit_name: String,
    pub question_ids: Vec<String>,
    pub answered_question_ids: Vec<String>,
    pub unresolved_questions: Vec<String>,
    pub intent_record: Option<TacitIntentRecord>,
    pub oracle_status: CapturedOracleStatus,
    pub fidelity_grade: FidelityGrade,
    pub handoff_state: EraRHandoffState,
    pub ported_claim_allowed: bool,
    pub gaps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TacitIntentRecord {
    pub unit_id: String,
    pub intent_statement: String,
    pub invariants: Vec<String>,
    pub edge_cases: Vec<String>,
    pub timing_notes: Vec<String>,
    pub provenance_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleSpec {
    pub id: String,
    pub unit_id: String,
    pub stimulus: String,
    pub expected_events: Vec<String>,
    pub primary_state_hash: String,
    pub secondary_render_digest: Option<String>,
    pub tolerance: String,
    pub provenance_refs: Vec<String>,
    pub status: CapturedOracleStatus,
    pub ported_claim_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturedOracleStatus {
    Missing,
    Captured,
    Passing,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleFidelityReport {
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub no_oracle_not_ported: bool,
    pub clean_room_source_only: bool,
    pub deterministic_capture: bool,
    pub studio_trusted_write_authority: bool,
    pub blocked_oracle_refs: Vec<String>,
    pub gap_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleReDerivationTask {
    pub unit_id: String,
    pub task: String,
    pub reason: String,
    pub handoff_state: EraRHandoffState,
}

pub fn capture_tacit_oracles(
    units: &[BehavioralUnitRecord],
    questions: &[InterrogationQuestion],
    answers: &[TacitAnswerRecord],
    traces: &[ObservedBehaviorTrace],
) -> Result<OracleCaptureReport> {
    if units.is_empty() {
        return Err(anyhow!(
            "tacit oracle capture requires at least one M108 behavioral unit"
        ));
    }

    let mut units = units.to_vec();
    units.sort_by(|a, b| a.id.cmp(&b.id));
    let mut questions = questions.to_vec();
    questions.sort_by(|a, b| a.unit_id.cmp(&b.unit_id).then(a.id.cmp(&b.id)));
    let mut answers = answers.to_vec();
    answers.sort_by(|a, b| {
        a.question_id
            .cmp(&b.question_id)
            .then(a.author.cmp(&b.author))
    });
    let mut traces = traces.to_vec();
    traces.sort_by(|a, b| a.unit_id.cmp(&b.unit_id).then(a.id.cmp(&b.id)));

    validate_questions(&units, &questions)?;
    validate_answers(&questions, &answers)?;
    validate_traces(&units, &traces)?;

    let answers_by_question = answers.iter().fold(
        BTreeMap::<String, Vec<&TacitAnswerRecord>>::new(),
        |mut acc, answer| {
            acc.entry(answer.question_id.clone())
                .or_default()
                .push(answer);
            acc
        },
    );
    let traces_by_unit = traces.iter().fold(
        BTreeMap::<String, Vec<&ObservedBehaviorTrace>>::new(),
        |mut acc, trace| {
            acc.entry(trace.unit_id.clone()).or_default().push(trace);
            acc
        },
    );

    let mut sessions = Vec::new();
    let mut oracle_specs = Vec::new();
    let mut blocked_refs = BTreeSet::new();
    let mut gap_summary = BTreeSet::new();

    for unit in &units {
        let unit_questions = questions
            .iter()
            .filter(|question| question.unit_id == unit.id)
            .collect::<Vec<_>>();
        let mut answered_question_ids = BTreeSet::new();
        let mut unresolved_questions = Vec::new();
        let mut answer_records = Vec::new();

        for question in &unit_questions {
            if let Some(records) = answers_by_question.get(&question.id) {
                answered_question_ids.insert(question.id.clone());
                answer_records.extend(records.iter().copied());
            } else if question.required {
                unresolved_questions
                    .push(format!("{} unresolved: {}", question.id, question.resolves));
            }
        }

        let unit_traces = traces_by_unit.get(&unit.id).cloned().unwrap_or_default();
        let mut gaps = unit.gaps.clone();
        let has_blocked_ref = answer_records
            .iter()
            .flat_map(|answer| answer.provenance_refs.iter())
            .chain(unit_traces.iter().map(|trace| &trace.source_provenance))
            .any(|reference| is_blocked_reference(reference));
        if has_blocked_ref {
            blocked_refs.insert(format!(
                "{} includes decompiled/ripped/foreign-runtime provenance; oracle blocked",
                unit.id
            ));
            gaps.push("blocked provenance is not clean-room/source-only".to_string());
        }

        let has_required_answers = !unit_questions.is_empty()
            && unit_questions
                .iter()
                .filter(|question| question.required)
                .all(|question| answered_question_ids.contains(&question.id));
        let has_replayable_trace = unit_traces
            .iter()
            .any(|trace| is_state_hash(&trace.state_hash) && !trace.observed_events.is_empty());

        let (oracle_status, grade, handoff_state) = if has_blocked_ref {
            (
                CapturedOracleStatus::Blocked,
                FidelityGrade::Red,
                EraRHandoffState::RejectOrDefer,
            )
        } else if has_required_answers && has_replayable_trace {
            (
                CapturedOracleStatus::Captured,
                FidelityGrade::Green,
                EraRHandoffState::Reexpress,
            )
        } else if answer_records.is_empty() {
            gaps.push("oracle missing; interrogation answers required".to_string());
            (
                CapturedOracleStatus::Missing,
                FidelityGrade::Yellow,
                EraRHandoffState::Interrogate,
            )
        } else {
            gaps.push("oracle incomplete; replayable deterministic trace required".to_string());
            (
                CapturedOracleStatus::Missing,
                FidelityGrade::Yellow,
                EraRHandoffState::CaptureOracle,
            )
        };

        if oracle_status == CapturedOracleStatus::Captured {
            for trace in unit_traces {
                let mut provenance_refs = BTreeSet::new();
                provenance_refs.insert(trace.source_provenance.clone());
                for answer in &answer_records {
                    for reference in &answer.provenance_refs {
                        provenance_refs.insert(reference.clone());
                    }
                }
                oracle_specs.push(OracleSpec {
                    id: format!("oracle:{}:{}", sanitize(&unit.id), sanitize(&trace.id)),
                    unit_id: unit.id.clone(),
                    stimulus: trace.stimulus.clone(),
                    expected_events: trace.observed_events.clone(),
                    primary_state_hash: trace.state_hash.clone(),
                    secondary_render_digest: trace.secondary_render_digest.clone(),
                    tolerance: if trace.secondary_render_digest.is_some() {
                        "state-hash primary; perceptual render secondary".to_string()
                    } else {
                        "2D bit-exact deterministic state hash".to_string()
                    },
                    provenance_refs: provenance_refs.into_iter().collect(),
                    status: CapturedOracleStatus::Captured,
                    ported_claim_allowed: false,
                });
            }
        }

        let intent_record = if answer_records.is_empty() || has_blocked_ref {
            None
        } else {
            Some(build_intent_record(unit, &answer_records))
        };

        for gap in &gaps {
            gap_summary.insert(format!("{}: {}", unit.id, gap));
        }

        sessions.push(InterrogationSessionReport {
            unit_id: unit.id.clone(),
            unit_name: unit.name.clone(),
            question_ids: unit_questions.iter().map(|q| q.id.clone()).collect(),
            answered_question_ids: answered_question_ids.into_iter().collect(),
            unresolved_questions,
            intent_record,
            oracle_status,
            fidelity_grade: grade,
            handoff_state,
            ported_claim_allowed: false,
            gaps,
        });
    }

    let re_derivation_tasks = sessions
        .iter()
        .filter(|session| session.oracle_status != CapturedOracleStatus::Captured)
        .map(|session| OracleReDerivationTask {
            unit_id: session.unit_id.clone(),
            task: match session.handoff_state {
                EraRHandoffState::Interrogate => "ask_more".to_string(),
                EraRHandoffState::CaptureOracle => "capture_or_repair_oracle".to_string(),
                EraRHandoffState::Reexpress => "reexpress".to_string(),
                EraRHandoffState::Verify => "verify".to_string(),
                EraRHandoffState::RejectOrDefer => "reject_or_defer".to_string(),
            },
            reason: session.gaps.join("; "),
            handoff_state: session.handoff_state,
        })
        .collect::<Vec<_>>();

    let fidelity_report = build_fidelity_report(&sessions, &blocked_refs, gap_summary);
    let deterministic_digest = report_digest(&sessions, &oracle_specs, &fidelity_report);

    Ok(OracleCaptureReport {
        schema_version: TACIT_ORACLE_CAPTURE_SCHEMA_VERSION.to_string(),
        boundary: TACIT_ORACLE_CAPTURE_BOUNDARY.to_string(),
        deterministic_digest,
        sessions,
        oracle_specs,
        fidelity_report,
        re_derivation_tasks,
    })
}

fn validate_questions(
    units: &[BehavioralUnitRecord],
    questions: &[InterrogationQuestion],
) -> Result<()> {
    let unit_ids = units
        .iter()
        .map(|unit| unit.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut question_ids = BTreeSet::new();
    for question in questions {
        validate_id("question", &question.id)?;
        if !unit_ids.contains(question.unit_id.as_str()) {
            return Err(anyhow!(
                "question {} references unknown unit {}",
                question.id,
                question.unit_id
            ));
        }
        if !question_ids.insert(question.id.as_str()) {
            return Err(anyhow!("duplicate question id {}", question.id));
        }
        if question.prompt.trim().is_empty() || question.resolves.trim().is_empty() {
            return Err(anyhow!(
                "question {} must include prompt and ambiguity",
                question.id
            ));
        }
    }
    Ok(())
}

fn validate_answers(
    questions: &[InterrogationQuestion],
    answers: &[TacitAnswerRecord],
) -> Result<()> {
    let question_ids = questions
        .iter()
        .map(|question| question.id.as_str())
        .collect::<BTreeSet<_>>();
    for answer in answers {
        if !question_ids.contains(answer.question_id.as_str()) {
            return Err(anyhow!(
                "answer references unknown question {}",
                answer.question_id
            ));
        }
        if answer.author.trim().is_empty() || answer.answer.trim().is_empty() {
            return Err(anyhow!(
                "answer for {} must include author and answer",
                answer.question_id
            ));
        }
        if answer.provenance_refs.is_empty() {
            return Err(anyhow!(
                "answer for {} must include provenance",
                answer.question_id
            ));
        }
    }
    Ok(())
}

fn validate_traces(units: &[BehavioralUnitRecord], traces: &[ObservedBehaviorTrace]) -> Result<()> {
    let unit_ids = units
        .iter()
        .map(|unit| unit.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut trace_ids = BTreeSet::new();
    for trace in traces {
        validate_id("trace", &trace.id)?;
        if !trace_ids.insert(trace.id.as_str()) {
            return Err(anyhow!("duplicate trace id {}", trace.id));
        }
        if !unit_ids.contains(trace.unit_id.as_str()) {
            return Err(anyhow!(
                "trace {} references unknown unit {}",
                trace.id,
                trace.unit_id
            ));
        }
        if trace.stimulus.trim().is_empty() || trace.source_provenance.trim().is_empty() {
            return Err(anyhow!(
                "trace {} must include stimulus and provenance",
                trace.id
            ));
        }
    }
    Ok(())
}

fn build_intent_record(
    unit: &BehavioralUnitRecord,
    answers: &[&TacitAnswerRecord],
) -> TacitIntentRecord {
    let mut invariants = BTreeSet::new();
    let mut edge_cases = BTreeSet::new();
    let mut timing_notes = BTreeSet::new();
    let mut provenance_refs = BTreeSet::new();
    let mut statements = Vec::new();

    for answer in answers {
        statements.push(answer.answer.trim().to_string());
        for reference in &answer.provenance_refs {
            provenance_refs.insert(reference.clone());
        }
        let lower = answer.answer.to_ascii_lowercase();
        if lower.contains("must") || lower.contains("always") || lower.contains("never") {
            invariants.insert(answer.answer.trim().to_string());
        }
        if lower.contains("edge") || lower.contains("if") || lower.contains("when") {
            edge_cases.insert(answer.answer.trim().to_string());
        }
        if lower.contains("frame")
            || lower.contains("tick")
            || lower.contains("second")
            || lower.contains("timing")
        {
            timing_notes.insert(answer.answer.trim().to_string());
        }
    }

    if invariants.is_empty() {
        invariants
            .insert("No implicit invariant captured beyond the answered intent text".to_string());
    }
    if edge_cases.is_empty() {
        edge_cases
            .insert("No edge-case answer captured; keep follow-up open if needed".to_string());
    }
    if timing_notes.is_empty() {
        timing_notes.insert(
            "No timing tolerance captured; downstream oracle must fail closed if timing matters"
                .to_string(),
        );
    }

    TacitIntentRecord {
        unit_id: unit.id.clone(),
        intent_statement: statements.join(" "),
        invariants: invariants.into_iter().collect(),
        edge_cases: edge_cases.into_iter().collect(),
        timing_notes: timing_notes.into_iter().collect(),
        provenance_refs: provenance_refs.into_iter().collect(),
    }
}

fn build_fidelity_report(
    sessions: &[InterrogationSessionReport],
    blocked_refs: &BTreeSet<String>,
    mut gap_summary: BTreeSet<String>,
) -> OracleFidelityReport {
    let green_count = sessions
        .iter()
        .filter(|session| session.fidelity_grade == FidelityGrade::Green)
        .count();
    let yellow_count = sessions
        .iter()
        .filter(|session| session.fidelity_grade == FidelityGrade::Yellow)
        .count();
    let red_count = sessions
        .iter()
        .filter(|session| session.fidelity_grade == FidelityGrade::Red)
        .count();
    if sessions
        .iter()
        .any(|session| session.oracle_status == CapturedOracleStatus::Missing)
    {
        gap_summary
            .insert("oracle-less units explicitly flagged; no ported claim allowed".to_string());
    }
    OracleFidelityReport {
        green_count,
        yellow_count,
        red_count,
        no_oracle_not_ported: true,
        clean_room_source_only: true,
        deterministic_capture: true,
        studio_trusted_write_authority: false,
        blocked_oracle_refs: blocked_refs.iter().cloned().collect(),
        gap_summary: gap_summary.into_iter().collect(),
    }
}

fn report_digest(
    sessions: &[InterrogationSessionReport],
    oracle_specs: &[OracleSpec],
    fidelity_report: &OracleFidelityReport,
) -> String {
    stable_digest(
        &serde_json::json!({
            "schemaVersion": TACIT_ORACLE_CAPTURE_SCHEMA_VERSION,
            "boundary": TACIT_ORACLE_CAPTURE_BOUNDARY,
            "sessions": sessions,
            "oracleSpecs": oracle_specs,
            "fidelityReport": fidelity_report,
        })
        .to_string(),
    )
}

fn validate_id(kind: &str, id: &str) -> Result<()> {
    if id.trim().is_empty()
        || id.contains("..")
        || id.contains('\\')
        || id.starts_with('/')
        || id.chars().any(|c| c.is_control())
    {
        return Err(anyhow!("unsafe {kind} id {id:?}"));
    }
    Ok(())
}

fn is_state_hash(value: &str) -> bool {
    value.starts_with("fnv64:") || value.starts_with("sha256:") || value.starts_with("state:")
}

fn is_blocked_reference(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    lower.contains("decompiled")
        || lower.contains("ilspy")
        || lower.contains("dnspy")
        || lower.contains("shipped-build")
        || lower.contains("foreign-runtime")
        || lower.contains("live-bridge")
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn stable_digest(text: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in text.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::legacy_logic_ingestion::{
        analyze_legacy_logic, LegacyLogicSource, LegacyLogicSourceKind,
    };

    fn units() -> Vec<BehavioralUnitRecord> {
        analyze_legacy_logic(&[LegacyLogicSource {
            path: "Assets/Scripts/PlayerController.cs".to_string(),
            kind: LegacyLogicSourceKind::CSharpSource,
            source_only_attestation: true,
            text: r#"
using UnityEngine;
public class PlayerController : MonoBehaviour {
  void Update() { if (Input.GetButtonDown("Jump")) { body.AddForce(Vector2.up); } }
}
"#
            .to_string(),
        }])
        .unwrap()
        .behavioral_units
    }

    fn question(unit_id: &str) -> InterrogationQuestion {
        InterrogationQuestion {
            id: "q.jump.intent".to_string(),
            unit_id: unit_id.to_string(),
            prompt: "What player-visible result should Jump produce?".to_string(),
            resolves: "jump outcome and timing".to_string(),
            required: true,
        }
    }

    fn answer() -> TacitAnswerRecord {
        TacitAnswerRecord {
            question_id: "q.jump.intent".to_string(),
            author: "designer".to_string(),
            answer: "Jump must raise vertical velocity on the same fixed tick; when already grounded it never double-applies in one frame.".to_string(),
            confidence: AnswerConfidence::High,
            provenance_refs: vec!["source-notes/player-controller.md".to_string()],
        }
    }

    fn trace(unit_id: &str, state_hash: &str) -> ObservedBehaviorTrace {
        ObservedBehaviorTrace {
            id: "trace.jump.001".to_string(),
            unit_id: unit_id.to_string(),
            stimulus: "frame 10: press Jump".to_string(),
            observed_events: vec![
                "vertical_velocity_positive".to_string(),
                "jump_count=1".to_string(),
            ],
            state_hash: state_hash.to_string(),
            source_provenance: "source-run/open-text-trace.json".to_string(),
            secondary_render_digest: None,
        }
    }

    #[test]
    fn captured_answers_and_traces_synthesize_oracle_without_port_claim() {
        let units = units();
        let unit_id = units[0].id.clone();
        let report = capture_tacit_oracles(
            &units,
            &[question(&unit_id)],
            &[answer()],
            &[trace(&unit_id, "fnv64:0123456789abcdef")],
        )
        .unwrap();

        assert_eq!(report.schema_version, TACIT_ORACLE_CAPTURE_SCHEMA_VERSION);
        assert!(report.boundary.contains("source-independent"));
        assert!(report.boundary.contains("Rust-owned artifact truth"));
        assert_eq!(
            report.sessions[0].oracle_status,
            CapturedOracleStatus::Captured
        );
        assert_eq!(report.sessions[0].fidelity_grade, FidelityGrade::Green);
        assert_eq!(
            report.sessions[0].handoff_state,
            EraRHandoffState::Reexpress
        );
        assert!(!report.sessions[0].ported_claim_allowed);
        assert_eq!(report.oracle_specs.len(), 1);
        assert_eq!(
            report.oracle_specs[0].primary_state_hash,
            "fnv64:0123456789abcdef"
        );
        assert_eq!(
            report.oracle_specs[0].tolerance,
            "2D bit-exact deterministic state hash"
        );
        assert!(!report.oracle_specs[0].ported_claim_allowed);
        assert_eq!(report.fidelity_report.green_count, 1);
        assert!(report.fidelity_report.no_oracle_not_ported);
        assert!(!report.fidelity_report.studio_trusted_write_authority);
    }

    #[test]
    fn oracle_less_units_are_flagged_and_not_green() {
        let units = units();
        let unit_id = units[0].id.clone();
        let report = capture_tacit_oracles(&units, &[question(&unit_id)], &[], &[]).unwrap();

        assert_eq!(report.oracle_specs.len(), 0);
        assert_eq!(
            report.sessions[0].oracle_status,
            CapturedOracleStatus::Missing
        );
        assert_eq!(report.sessions[0].fidelity_grade, FidelityGrade::Yellow);
        assert_eq!(
            report.sessions[0].handoff_state,
            EraRHandoffState::Interrogate
        );
        assert!(!report.sessions[0].ported_claim_allowed);
        assert!(report.sessions[0]
            .gaps
            .iter()
            .any(|gap| gap.contains("oracle missing")));
        assert_eq!(report.re_derivation_tasks[0].task, "ask_more");
        assert_eq!(report.fidelity_report.green_count, 0);
        assert!(report
            .fidelity_report
            .gap_summary
            .iter()
            .any(|gap| gap.contains("oracle-less units explicitly flagged")));
    }

    #[test]
    fn deterministic_digest_is_stable_and_changes_with_oracle_hash() {
        let units = units();
        let unit_id = units[0].id.clone();
        let first = capture_tacit_oracles(
            &units,
            &[question(&unit_id)],
            &[answer()],
            &[trace(&unit_id, "fnv64:aaaaaaaaaaaaaaaa")],
        )
        .unwrap();
        let reordered = capture_tacit_oracles(
            &units,
            &[question(&unit_id)],
            &[answer()],
            &[trace(&unit_id, "fnv64:aaaaaaaaaaaaaaaa")],
        )
        .unwrap();
        let changed = capture_tacit_oracles(
            &units,
            &[question(&unit_id)],
            &[answer()],
            &[trace(&unit_id, "fnv64:bbbbbbbbbbbbbbbb")],
        )
        .unwrap();

        assert_eq!(first.deterministic_digest, reordered.deterministic_digest);
        assert_ne!(first.deterministic_digest, changed.deterministic_digest);
    }

    #[test]
    fn blocked_provenance_fails_closed_red() {
        let units = units();
        let unit_id = units[0].id.clone();
        let mut answer = answer();
        answer.provenance_refs = vec!["decompiled/ilspy-dump.cs".to_string()];
        let report = capture_tacit_oracles(
            &units,
            &[question(&unit_id)],
            &[answer],
            &[trace(&unit_id, "fnv64:0123456789abcdef")],
        )
        .unwrap();

        assert_eq!(
            report.sessions[0].oracle_status,
            CapturedOracleStatus::Blocked
        );
        assert_eq!(report.sessions[0].fidelity_grade, FidelityGrade::Red);
        assert_eq!(
            report.sessions[0].handoff_state,
            EraRHandoffState::RejectOrDefer
        );
        assert!(report.oracle_specs.is_empty());
        assert!(!report.fidelity_report.blocked_oracle_refs.is_empty());
        assert!(!report.sessions[0].ported_claim_allowed);
    }
}

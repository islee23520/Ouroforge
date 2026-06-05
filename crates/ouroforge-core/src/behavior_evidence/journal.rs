use super::{BehaviorEvidenceBundleArtifact, BehaviorEvidenceBundleStatus};
use crate::EvidenceIndex;
use std::path::Path;

pub fn render_behavior_evidence_journal_section(
    run_dir: &Path,
    evidence: &EvidenceIndex,
) -> String {
    let bundle_artifacts = evidence
        .artifacts
        .iter()
        .filter(|artifact| {
            artifact
                .metadata
                .get("artifact")
                .and_then(|value| value.as_str())
                == Some("behavior_evidence_bundle")
                || artifact.path.contains("behavior-evidence-bundle")
        })
        .collect::<Vec<_>>();
    if bundle_artifacts.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push_str("## Behavior Evidence Lifecycle\n\n");
    out.push_str("- Boundary: read-only structured behavior lifecycle evidence; no arbitrary script execution, eval, dynamic import, plugin loader, command bridge, local server bridge, browser trusted write, or auto-apply.\n");
    for artifact in bundle_artifacts {
        out.push_str(&format!("- Bundle artifact: `{}`\n", artifact.path));
        let path = run_dir.join(&artifact.path);
        match std::fs::read_to_string(&path)
            .map_err(|error| error.to_string())
            .and_then(|input| {
                BehaviorEvidenceBundleArtifact::from_json_str(&input)
                    .map_err(|error| error.to_string())
            }) {
            Ok(bundle) => render_bundle(&mut out, &bundle),
            Err(error) => {
                out.push_str(&format!(
                    "  - Malformed behavior evidence bundle: {error}\n"
                ));
            }
        }
    }
    out.push('\n');
    out
}

fn render_bundle(out: &mut String, bundle: &BehaviorEvidenceBundleArtifact) {
    out.push_str(&format!(
        "  - Bundle: `{}` status `{:?}`\n",
        bundle.bundle_id, bundle.status
    ));
    out.push_str(&format!(
        "  - Lifecycle refs: definitions `{}`, runtime events `{}`, scenario outcomes `{}`, drafts `{}`, reviews `{}`, applies `{}`, rollback `{}`, rerun comparisons `{}`\n",
        bundle.behavior_definition_refs.len(),
        bundle.runtime_event_refs.len(),
        bundle.scenario_outcome_refs.len(),
        bundle.draft_refs.len(),
        bundle.review_decision_refs.len(),
        bundle.apply_transaction_refs.len(),
        bundle.rollback_metadata_refs.len(),
        bundle.rerun_comparison_refs.len()
    ));
    if matches!(
        bundle.status,
        BehaviorEvidenceBundleStatus::Blocked | BehaviorEvidenceBundleStatus::Stale
    ) {
        out.push_str(&format!(
            "  - Blocked/stale reasons: {}\n",
            join_or_none(&bundle.blocked_reasons)
        ));
    }
    if bundle.observed_failures.is_empty() {
        out.push_str("  - Observed failures: none recorded.\n");
    } else {
        for failure in &bundle.observed_failures {
            out.push_str(&format!(
                "  - Observed failure `{}`: {} (`{}`)\n",
                failure.scenario_id, failure.summary, failure.evidence_ref.path
            ));
        }
    }
    if bundle.next_step_hypotheses.is_empty() {
        out.push_str("  - Next-step hypotheses: none recorded.\n");
    } else {
        for hypothesis in &bundle.next_step_hypotheses {
            out.push_str(&format!(
                "  - Next-step hypothesis `{}`: {}\n",
                hypothesis.id, hypothesis.summary
            ));
        }
    }
}

fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

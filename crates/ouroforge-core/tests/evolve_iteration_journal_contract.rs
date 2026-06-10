use ouroforge_core::evolve_iteration_journal::{
    EvolveIterationEntry, EvolveIterationJournal, EvolveIterationOutcome,
    EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION,
};
use std::collections::BTreeSet;

fn entry(id: &str, applied: bool, outcome: EvolveIterationOutcome) -> EvolveIterationEntry {
    EvolveIterationEntry {
        iteration_id: id.to_string(),
        hypothesis: "fix observed failure".to_string(),
        before_bundle_ref: format!("runs/journal/{id}/before.json"),
        proposal_ref: format!("runs/journal/{id}/proposal.json"),
        review_ref: applied.then(|| format!("runs/journal/{id}/review.json")),
        apply_ref: applied.then(|| format!("runs/journal/{id}/apply.json")),
        after_bundle_ref: applied.then(|| format!("runs/journal/{id}/after.json")),
        comparison_ref: applied.then(|| format!("runs/journal/{id}/comparison.json")),
        evidence_refs: vec![format!("runs/journal/{id}/evidence.json")],
        outcome,
        decision: "record result".to_string(),
        next_action: "route next action".to_string(),
        applied_change: applied,
    }
}

fn journal() -> EvolveIterationJournal {
    EvolveIterationJournal {
        schema_version: EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION.to_string(),
        journal_id: "journal-2382".to_string(),
        entries: vec![entry(
            "iter-accepted",
            true,
            EvolveIterationOutcome::Accepted,
        )],
        guardrails: vec![
            "append-only; applied changes require review/apply/evidence links".to_string(),
        ],
    }
}

#[test]
fn journal_is_append_only_and_keeps_inconclusive_runs_visible() {
    let journal = journal();
    journal.validate().expect("valid journal");
    let next = journal
        .append_entry(entry(
            "iter-inconclusive",
            false,
            EvolveIterationOutcome::Inconclusive,
        ))
        .expect("append inconclusive");
    next.validate_is_append_of(&journal).expect("append-only");
    let model = next.read_model();
    assert_eq!(model.entry_count, 2);
    assert_eq!(model.inconclusive_count, 1);
    assert!(model.append_only);
}

#[test]
fn applied_iteration_cannot_omit_review_apply_or_comparison_links() {
    let mut bad = entry("iter-bad", true, EvolveIterationOutcome::Accepted);
    bad.review_ref = None;
    let error = bad
        .validate()
        .expect_err("missing review is rejected")
        .to_string();
    assert!(error.contains("reviewRef"), "{error}");
}

#[test]
fn journal_rejects_history_rewrite_and_missing_evidence_refs() {
    let journal = journal();
    let next = journal
        .append_entry(entry(
            "iter-regressed",
            true,
            EvolveIterationOutcome::Regressed,
        ))
        .expect("append regressed");
    let mut rewritten = next.clone();
    rewritten.entries[0].decision = "rewritten".to_string();
    let error = rewritten
        .validate_is_append_of(&journal)
        .expect_err("rewrite rejected")
        .to_string();
    assert!(error.contains("rewrote entry"), "{error}");

    let mut refs = BTreeSet::new();
    for entry in &next.entries {
        refs.insert(entry.before_bundle_ref.clone());
        refs.insert(entry.proposal_ref.clone());
        refs.extend(entry.evidence_refs.iter().cloned());
        for reference in [
            &entry.review_ref,
            &entry.apply_ref,
            &entry.after_bundle_ref,
            &entry.comparison_ref,
        ]
        .into_iter()
        .flatten()
        {
            refs.insert(reference.clone());
        }
    }
    refs.remove("runs/journal/iter-regressed/comparison.json");
    let error = next
        .validate_references_exist(&refs)
        .expect_err("missing ref rejected")
        .to_string();
    assert!(error.contains("missing from known evidence set"), "{error}");
}

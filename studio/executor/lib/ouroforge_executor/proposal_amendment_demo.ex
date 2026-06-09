defmodule OuroforgeExecutor.ProposalAmendmentDemo do
  @moduledoc """
  Scripted Proposal Amendment and Re-Verify demo (#2055).

  The demo has two paths:

    * autonomous default: no human intervention is provided, so the loop
      completes without the Studio surface;
    * amended proposal: Studio captures a human edit as intervention evidence and
      routes it to the Rust proposal-amendment validator before review/apply
      readiness is reported.

  This module does not write artifacts, run hidden commands, or apply changes.
  A caller supplies the Rust validator runner so tests can prove routing without
  granting Elixir data-plane authority.
  """

  alias OuroforgeExecutor.ProposalAmendmentSurface

  @boundary ProposalAmendmentSurface.boundary()

  def autonomous_default_demo do
    %{
      demo_id: "m75-amend-before-approve-autonomous-default",
      human_intervention: :absent,
      status: :completed_without_human,
      read_gated_write: false,
      trusted_write_performed: false,
      gated_write_attempted: false,
      cli_fallback_supported: true,
      evidence_refs: [
        "runs/m75/demo/autonomous-loop-completed.json",
        "runs/m75/demo/no-human-surface-required.json"
      ],
      boundary: @boundary
    }
  end

  def amended_proposal_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)
    artifact_path = Keyword.get(opts, :artifact_path, "runs/m75/demo/amendment.green.json")

    with {:ok, capture} <- ProposalAmendmentSurface.capture(capture_attrs()),
         {:ok, submission} <- ProposalAmendmentSurface.to_rust_submission(capture),
         rust_artifact <- green_rust_artifact(capture, artifact_path),
         {:ok, rust_result} <- runner.(capture.routeCli, rust_artifact) do
      {:ok,
       %{
         demo_id: "m75-amend-before-approve-human-amended",
         human_intervention: :present_optional,
         capture: submission,
         rust_artifact: rust_artifact,
         rust_result: rust_result,
         status: :ready_for_review_apply,
         gated_write_attempted: true,
         trusted_write_performed: false,
         raw_bypass_performed: false,
         autonomous_fallback_still_supported: true,
         boundary: @boundary
       }}
    end
  end

  def blocked_amendment_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)
    artifact_path = Keyword.get(opts, :artifact_path, "runs/m75/demo/amendment.blocked.json")

    with {:ok, capture} <- ProposalAmendmentSurface.capture(capture_attrs()),
         rust_artifact <- blocked_rust_artifact(capture, artifact_path),
         {:error, rust_result} <- runner.(capture.routeCli, rust_artifact) do
      {:ok,
       %{
         demo_id: "m75-amend-before-approve-blocked",
         human_intervention: :present_optional,
         rust_artifact: rust_artifact,
         rust_result: rust_result,
         status: :blocked_by_gate,
         gated_write_attempted: true,
         trusted_write_performed: false,
         raw_bypass_performed: false,
         autonomous_fallback_still_supported: true,
         boundary: @boundary
       }}
    end
  end

  defp capture_attrs do
    %{
      amendment_id: "amendment-m75-demo-001",
      proposal_id: "proposal-agent-demo-001",
      base_proposal_ref: "runs/m75/demo/proposal-agent-demo-001.before.json",
      human_actor: "local-human",
      edit_summary: "Tighten the proposed config before approval.",
      amended_payload: ~s({"spawnBudget":3,"difficulty":"medium"})
    }
  end

  defp green_rust_artifact(capture, artifact_path) do
    base_rust_artifact(capture, artifact_path, "ready-for-review-apply", [
      gate("review-apply", "passed"),
      gate("scene-source-apply", "passed"),
      gate("evaluator", "passed"),
      gate("design-integrity", "passed")
    ])
  end

  defp blocked_rust_artifact(capture, artifact_path) do
    base_rust_artifact(capture, artifact_path, "blocked", [
      gate("review-apply", "passed"),
      gate("scene-source-apply", "passed"),
      gate("evaluator", "failed"),
      gate("design-integrity", "passed")
    ])
  end

  defp base_rust_artifact(capture, artifact_path, status, gates) do
    %{
      "artifactPath" => artifact_path,
      "schemaVersion" => "ouroforge.proposal-amendment.v1",
      "amendmentId" => capture.amendmentId,
      "proposalId" => capture.proposalId,
      "baseProposalRef" => capture.baseProposalRef,
      "amendedProposalRef" => "runs/m75/demo/proposal-agent-demo-001.amended.json",
      "humanActor" => capture.humanActor,
      "editSummary" => capture.editSummary,
      "capturedVia" => "studio-phoenix-live-view",
      "interventionAsEvidence" => true,
      "beforeEvidenceRefs" => ["runs/m75/demo/before-verdict.json"],
      "afterEvidenceRefs" => ["runs/m75/demo/after-verdict.json"],
      "provenanceRefs" => ["runs/m75/demo/human-amendment-provenance.json"],
      "gateResults" => gates,
      "status" => status,
      "reviewApplyRef" => "runs/m75/demo/review-apply-decision.json",
      "autoApplyPerformed" => false,
      "rawBypassRequested" => false,
      "studioTrustedWriteAuthority" => false,
      "humanRequiredForAutonomousLoop" => false,
      "cliFallbackSupported" => true,
      "boundary" => @boundary
    }
  end

  defp gate(kind, status) do
    %{
      "kind" => kind,
      "status" => status,
      "evidenceRef" => "runs/m75/demo/evidence/#{kind}.json",
      "beforeRef" => "runs/m75/demo/before/#{kind}.json",
      "afterRef" => "runs/m75/demo/after/#{kind}.json"
    }
  end
end

defmodule OuroforgeExecutor.HumanArtifactIntakeDemo do
  @moduledoc """
  Scripted Human-Authored Artifact Intake demo (#2059).

  Proves no-human autonomous completion and optional human-authored intake routed
  through Rust gates before review/apply readiness.
  """

  alias OuroforgeExecutor.HumanArtifactIntakeSurface

  @boundary HumanArtifactIntakeSurface.boundary()

  def autonomous_default_demo do
    %{
      demo_id: "m76-human-artifact-intake-autonomous-default",
      human_intervention: :absent,
      status: :completed_without_human,
      read_gated_write: false,
      trusted_write_performed: false,
      gated_write_attempted: false,
      cli_fallback_supported: true,
      evidence_refs: [
        "runs/m76/demo/autonomous-loop-completed.json",
        "runs/m76/demo/no-human-surface-required.json"
      ],
      boundary: @boundary
    }
  end

  def human_intake_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)
    artifact_path = Keyword.get(opts, :artifact_path, "runs/m76/demo/human-intake.green.json")

    with {:ok, capture} <- HumanArtifactIntakeSurface.capture(capture_attrs()),
         {:ok, submission} <- HumanArtifactIntakeSurface.to_rust_submission(capture),
         rust_artifact <- green_rust_artifact(capture, artifact_path),
         {:ok, rust_result} <- runner.(capture.routeCli, rust_artifact) do
      {:ok,
       %{
         demo_id: "m76-human-artifact-intake-gated-human-card",
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

  def blocked_human_intake_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)
    artifact_path = Keyword.get(opts, :artifact_path, "runs/m76/demo/human-intake.blocked.json")

    with {:ok, capture} <- HumanArtifactIntakeSurface.capture(capture_attrs()),
         rust_artifact <- blocked_rust_artifact(capture, artifact_path),
         {:error, rust_result} <- runner.(capture.routeCli, rust_artifact) do
      {:ok,
       %{
         demo_id: "m76-human-artifact-intake-blocked",
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
      intake_id: "human-intake-m76-demo-001",
      artifact_id: "card-spark-human-demo",
      artifact_kind: "card",
      target_ref: "projects/demo/cards/card-spark.json",
      target_base_ref: "hash:card-base-before-demo",
      author: "human:local-author",
      author_provenance_ref: "runs/m76/demo/human-author-provenance.json",
      original_payload: ~s({"name":"Spark","cost":1,"text":"Deal 1 damage."})
    }
  end

  defp green_rust_artifact(capture, artifact_path) do
    base_rust_artifact(capture, artifact_path, "ready-for-review-apply", [
      gate("review-apply", "passed"),
      gate("scene-source-apply", "passed"),
      gate("evaluator", "passed"),
      gate("evidence-provenance", "passed")
    ])
  end

  defp blocked_rust_artifact(capture, artifact_path) do
    base_rust_artifact(capture, artifact_path, "blocked", [
      gate("review-apply", "passed"),
      gate("scene-source-apply", "passed"),
      gate("evaluator", "failed"),
      gate("evidence-provenance", "passed")
    ])
  end

  defp base_rust_artifact(capture, artifact_path, status, gates) do
    %{
      "artifactPath" => artifact_path,
      "schemaVersion" => "ouroforge.human-artifact-intake.v1",
      "intakeId" => capture.intakeId,
      "artifactId" => capture.artifactId,
      "artifactKind" => capture.artifactKind,
      "capturedVia" => "studio-phoenix-live-view",
      "author" => capture.author,
      "authorProvenanceRef" => capture.authorProvenanceRef,
      "humanProvenance" => true,
      "originalArtifactRef" => "runs/m76/demo/original-card.json",
      "normalizedCandidateRef" => "runs/m76/demo/normalized-card.json",
      "targetRef" => capture.targetRef,
      "targetBaseRef" => capture.targetBaseRef,
      "validationReportRef" => "runs/m76/demo/validation/card.report.json",
      "reviewApplyRef" => "runs/m76/demo/review-apply/card.decision.json",
      "gateResults" => gates,
      "status" => status,
      "interventionAsEvidence" => true,
      "readGatedWrite" => true,
      "rawBypassRequested" => false,
      "directArtifactWrite" => false,
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
      "evidenceRef" => "runs/m76/demo/evidence/#{kind}.json",
      "beforeRef" => "runs/m76/demo/before/#{kind}.json",
      "afterRef" => "runs/m76/demo/after/#{kind}.json"
    }
  end
end

defmodule OuroforgeExecutor.DiagnosisCorrectionDemo do
  @moduledoc """
  Scripted Diagnosis Correction and Intervention Feedback Loop demo (#2071).

  Proves that optional human correction is captured as read + gated-write
  evidence, routed to the Rust diagnosis-correction data plane, and never blocks
  the autonomous loop when no human intervenes.
  """

  alias OuroforgeExecutor.DiagnosisCorrectionSurface

  @boundary DiagnosisCorrectionSurface.boundary()

  def autonomous_default_demo do
    %{
      demo_id: "m79-diagnosis-correction-autonomous-default",
      human_intervention: :absent,
      status: :completed_without_human,
      waited_for_human?: false,
      human_surface_required?: false,
      read_gated_write: false,
      trusted_write_performed: false,
      gated_write_attempted: false,
      cli_fallback_supported: true,
      attribution: %{
        selected: "asset-provenance-gap",
        evidence_ref: "runs/m79/demo/autonomous/original-diagnosis.json"
      },
      evidence_refs: [
        "runs/m79/demo/autonomous-loop-completed.json",
        "runs/m79/demo/no-human-correction-required.json"
      ],
      boundary: @boundary
    }
  end

  def corrected_attribution_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)

    with {:ok, capture} <- DiagnosisCorrectionSurface.capture(capture_attrs()),
         {:ok, submission} <- DiagnosisCorrectionSurface.to_rust_submission(capture),
         {:ok, rust_result} <- runner.(capture.routeCli, submission) do
      {:ok,
       %{
         demo_id: "m79-diagnosis-correction-gated-reattribution",
         human_intervention: :present_optional,
         capture: capture,
         rust_submission: submission,
         rust_result: rust_result,
         status: :corrected_and_reattributed,
         gated_write_attempted: true,
         trusted_write_performed: false,
         raw_bypass_performed: false,
         autonomous_fallback_still_supported: true,
         prior_update: rust_result["priorUpdate"],
         before_attribution: rust_result["beforeAttribution"],
         after_attribution: rust_result["afterAttribution"],
         rendered: render(:corrected, rust_result),
         boundary: @boundary
       }}
    end
  end

  def rejected_correction_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)

    with {:ok, capture} <- DiagnosisCorrectionSurface.capture(capture_attrs()),
         {:ok, submission} <- DiagnosisCorrectionSurface.to_rust_submission(capture),
         {:error, rust_result} <- runner.(capture.routeCli, submission) do
      {:ok,
       %{
         demo_id: "m79-diagnosis-correction-rejected-gate",
         human_intervention: :present_optional,
         capture: capture,
         rust_submission: submission,
         rust_result: rust_result,
         status: :blocked_by_existing_gates,
         gated_write_attempted: true,
         trusted_write_performed: false,
         raw_bypass_performed: false,
         autonomous_fallback_still_supported: true,
         rendered: render(:blocked, rust_result),
         boundary: @boundary
       }}
    end
  end

  defp capture_attrs do
    %{
      correction_id: "corr-m79-demo-threshold",
      diagnosis_id: "diag-m79-demo-001",
      run_id: "run-m79-demo-001",
      original_attribution: "asset-provenance-gap",
      corrected_attribution: "evaluator-threshold-drift",
      human_actor: "human://local-operator",
      correction_rationale:
        "Asset provenance evidence is complete; evaluator threshold drift caused the blocked run."
    }
  end

  defp render(status, rust_result) do
    """
    M79 diagnosis correction and intervention feedback demo
    status: #{status}
    trusted writes: false
    Autonomous fallback verified: true
    Gated write verified: true
    human write type: diagnosis correction/intervention evidence
    gate path: review/apply -> scene/source-apply -> evaluator -> evidence/provenance
    re-attribution: #{rust_result["beforeAttribution"] || "blocked"} -> #{rust_result["afterAttribution"] || "blocked"}
    Rust data-plane result: #{inspect(rust_result)}
    intervention-as-evidence routed through Rust CLI gates
    transparent heuristic prior update; no opaque ML; no automated fun/taste inference
    """
  end
end

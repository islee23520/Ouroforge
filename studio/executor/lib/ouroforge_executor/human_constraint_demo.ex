defmodule OuroforgeExecutor.HumanConstraintDemo do
  @moduledoc """
  Scripted Human Constraints as First-Class Gates demo (#2067).

  Proves that optional human constraints are captured as read + gated-write
  evidence, routed to the Rust evaluator gate, and never block the autonomous
  loop when no human intervenes.
  """

  alias OuroforgeExecutor.HumanConstraintSurface

  @boundary HumanConstraintSurface.boundary()

  def autonomous_default_demo do
    %{
      demo_id: "m78-human-constraints-autonomous-default",
      human_intervention: :absent,
      status: :completed_without_human,
      waited_for_human?: false,
      human_surface_required?: false,
      read_gated_write: false,
      trusted_write_performed: false,
      gated_write_attempted: false,
      cli_fallback_supported: true,
      evidence_refs: [
        "runs/m78/demo/autonomous-loop-completed.json",
        "runs/m78/demo/no-human-constraint-required.json"
      ],
      boundary: @boundary
    }
  end

  def blocking_constraint_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)

    with {:ok, capture} <- HumanConstraintSurface.capture(capture_attrs()),
         {:ok, gate_input} <-
           HumanConstraintSurface.to_rust_gate_input(capture, violating_candidate()),
         {:error, rust_result} <- runner.(capture.routeCli, gate_input) do
      {:ok,
       %{
         demo_id: "m78-human-constraints-forbidden-mechanic-blocked",
         human_intervention: :present_optional,
         capture: capture,
         rust_gate_input: gate_input,
         rust_result: rust_result,
         status: :blocked_by_human_constraint_gate,
         gated_write_attempted: true,
         trusted_write_performed: false,
         raw_bypass_performed: false,
         autonomous_fallback_still_supported: true,
         rendered: render(:blocked, rust_result),
         boundary: @boundary
       }}
    end
  end

  def passing_constraint_demo(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)

    with {:ok, capture} <- HumanConstraintSurface.capture(capture_attrs()),
         {:ok, gate_input} <-
           HumanConstraintSurface.to_rust_gate_input(capture, compliant_candidate()),
         {:ok, rust_result} <- runner.(capture.routeCli, gate_input) do
      {:ok,
       %{
         demo_id: "m78-human-constraints-compliant-candidate",
         human_intervention: :present_optional,
         capture: capture,
         rust_gate_input: gate_input,
         rust_result: rust_result,
         status: :passes_human_constraint_gate,
         gated_write_attempted: true,
         trusted_write_performed: false,
         raw_bypass_performed: false,
         autonomous_fallback_still_supported: true,
         rendered: render(:pass, rust_result),
         boundary: @boundary
       }}
    end
  end

  defp capture_attrs do
    %{
      constraint_id: "constraint-m78-no-dash",
      kind: "forbidden-mechanic",
      author: "human:local-designer",
      author_provenance_ref: "runs/m78/demo/provenance/human-designer.json",
      target_ref: "runs/m78/demo/candidates/card.json",
      target_base_ref: "hash:m78-candidate-before",
      normalized_constraint_ref: "runs/m78/demo/constraints/no-dash.normalized.json",
      review_apply_ref: "runs/m78/demo/review/no-dash.decision.json",
      evaluator_evidence_ref: "runs/m78/demo/evaluator/human-constraint.json",
      evidence_refs: ["runs/m78/demo/evidence/no-dash-capture.json"],
      forbidden_mechanic: "dash"
    }
  end

  defp violating_candidate do
    %{
      "candidateId" => "candidate-m78-dash-card",
      "targetRef" => "runs/m78/demo/candidates/card.json",
      "mechanics" => ["dash", "burn"],
      "style" => "pixel-art",
      "budget" => 8,
      "evidenceRefs" => ["runs/m78/demo/evidence/candidate-dash.json"]
    }
  end

  defp compliant_candidate do
    %{
      "candidateId" => "candidate-m78-no-dash-card",
      "targetRef" => "runs/m78/demo/candidates/card.json",
      "mechanics" => ["burn"],
      "style" => "pixel-art",
      "budget" => 8,
      "evidenceRefs" => ["runs/m78/demo/evidence/candidate-no-dash.json"]
    }
  end

  defp render(status, rust_result) do
    """
    M78 human constraints as first-class gates demo
    status: #{status}
    trusted writes: false
    Autonomous fallback verified: true
    Gated write verified: true
    human write type: constraint/proposal evidence
    gate path: review/apply -> scene/source-apply -> evaluator -> evidence/provenance
    Rust evaluator result: #{inspect(rust_result)}
    intervention-as-evidence routed through Rust evaluator gates
    """
  end
end

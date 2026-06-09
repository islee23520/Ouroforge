defmodule OuroforgeExecutor.HumanConstraintSurfaceTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.HumanConstraintSurface

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        constraint_id: "constraint-m78-budget",
        kind: "budget-cap",
        author: "human:local-producer",
        author_provenance_ref: "runs/m78/provenance/human-producer.json",
        target_ref: "runs/m78/candidates/card.json",
        target_base_ref: "hash:m78-base",
        normalized_constraint_ref: "runs/m78/constraints/budget.normalized.json",
        review_apply_ref: "runs/m78/review/budget.decision.json",
        evaluator_evidence_ref: "runs/m78/evaluator/budget.json",
        evidence_refs: ["runs/m78/evidence/budget-capture.json"],
        budget_cap: 10
      },
      overrides
    )
  end

  test "captures a human constraint as read + gated-write evaluator evidence" do
    assert {:ok, capture} = HumanConstraintSurface.capture(attrs())
    assert capture.interventionAsEvidence
    assert capture.readGatedWrite
    assert capture.routeCli == ["evaluate"]
    refute capture.directArtifactWrite
    refute capture.rawBypassRequested
    refute capture.studioTrustedWriteAuthority
    refute capture.humanRequiredForAutonomousLoop
    assert capture.cliFallbackSupported
    assert capture.boundary =~ "Rust = data plane"
    assert capture.boundary =~ "Elixir/OTP + Phoenix LiveView = control + presentation"
    assert capture.boundary =~ "#1 and #23 remain open"

    assert {:ok, record} = HumanConstraintSurface.to_rust_constraint_record(capture)
    assert record["kind"] == "budget-cap"
    assert record["budgetCap"] == 10
    assert record["directArtifactWrite"] == false
    assert record["studioTrustedWriteAuthority"] == false
  end

  test "rejects raw bypass, trusted write, payload mismatch, and mandatory human drift" do
    assert {:error, :raw_bypass_forbidden} =
             HumanConstraintSurface.capture(attrs(%{target_ref: "raw_apply_bypass"}))

    assert {:error, :constraint_payload_mismatch} =
             HumanConstraintSurface.capture(attrs(%{kind: "required-style"}))

    assert {:ok, capture} = HumanConstraintSurface.capture(attrs())

    assert {:error, :trusted_write_forbidden} =
             %{capture | directArtifactWrite: true} |> HumanConstraintSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | studioTrustedWriteAuthority: true} |> HumanConstraintSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{capture | humanRequiredForAutonomousLoop: true}
             |> HumanConstraintSurface.validate()
  end
end

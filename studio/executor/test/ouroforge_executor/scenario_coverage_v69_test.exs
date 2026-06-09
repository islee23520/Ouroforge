defmodule OuroforgeExecutor.ScenarioCoverageV69Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{HumanConstraintDemo, HumanConstraintSurface}

  @repo_root Path.expand("../../../..", __DIR__)
  @matrix_path Path.join(
                 @repo_root,
                 "examples/human-constraints-first-class-gates-v1/scenario-coverage-v69/matrix.fixture.json"
               )

  defp decode_json!(path) do
    path
    |> File.read!()
    |> OuroforgeExecutor.JSON.decode!()
  end

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        constraint_id: "scenario-v69-no-dash",
        kind: "forbidden-mechanic",
        author: "human:local-designer",
        author_provenance_ref: "runs/v69/provenance/human-designer.json",
        target_ref: "runs/v69/candidates/card.json",
        target_base_ref: "hash:v69-card-before",
        normalized_constraint_ref: "runs/v69/constraints/no-dash.normalized.json",
        review_apply_ref: "runs/v69/review/no-dash.decision.json",
        evaluator_evidence_ref: "runs/v69/evaluator/human-constraint.json",
        evidence_refs: ["runs/v69/evidence/no-dash-capture.json"],
        forbidden_mechanic: "dash"
      },
      overrides
    )
  end

  test "coverage v69 matrix records human constraint no-bypass and zero-human rows" do
    matrix = decode_json!(@matrix_path)

    assert matrix["schemaVersion"] ==
             "scenario-coverage-v69-human-constraints-first-class-gates-v1"

    assert matrix["coverageVersion"] == 69
    assert matrix["autonomyInvariants"]["humanInputRequired"] == false
    assert matrix["autonomyInvariants"]["loopCompletesWithoutHuman"] == true
    assert matrix["autonomyInvariants"]["studioTrustedWriteAuthority"] == false
    assert matrix["autonomyInvariants"]["elixirOwnsArtifactSemantics"] == false
    assert matrix["autonomyInvariants"]["rustDataPlaneOwnsValidation"] == true

    row_ids = Enum.map(matrix["rows"], & &1["id"])

    for required <- [
          "constraints-recorded-through-existing-gates",
          "violating-output-blocked-with-evidence",
          "no-raw-bypass-from-elixir-human-constraint-surface",
          "loop-completes-without-human-input",
          "mandatory-human-regression-fails-closed",
          "coverage-v69-boundaries"
        ] do
      assert required in row_ids
    end

    assert Enum.all?(matrix["rows"], &(&1["status"] == "pass"))
  end

  test "Studio capture routes constraints as gated Rust evaluator evidence only" do
    assert {:ok, capture} = HumanConstraintSurface.capture(attrs())
    assert capture.routeCli == ["evaluate"]
    assert capture.interventionAsEvidence
    assert capture.readGatedWrite
    assert capture.cliFallbackSupported
    refute capture.directArtifactWrite
    refute capture.rawBypassRequested
    refute capture.studioTrustedWriteAuthority
    refute capture.humanRequiredForAutonomousLoop

    candidate = %{
      "candidateId" => "scenario-v69-dash-card",
      "targetRef" => "runs/v69/candidates/card.json",
      "mechanics" => ["dash"],
      "style" => "pixel-art",
      "budget" => 4,
      "evidenceRefs" => ["runs/v69/evidence/candidate.json"]
    }

    assert {:ok, gate_input} = HumanConstraintSurface.to_rust_gate_input(capture, candidate)
    [constraint] = gate_input["constraints"]
    assert constraint["reviewApplyRef"] =~ "review"
    assert constraint["evaluatorEvidenceRef"] =~ "evaluator"
    assert constraint["directArtifactWrite"] == false
    assert gate_input["schemaVersion"] == "ouroforge.human-constraint-gate.v1"
  end

  test "demo blocks violations and keeps autonomous fallback available" do
    runner = fn argv, gate_input ->
      assert argv == ["evaluate"]
      assert gate_input["candidate"]["mechanics"] == ["dash", "burn"]

      {:error,
       %{
         "readyForReviewApply" => false,
         "humanConstraints" => %{"status" => "fail", "failureCount" => 1},
         "blockedReasons" => ["HumanConstraint: forbidden mechanic dash"]
       }}
    end

    assert {:ok, demo} = HumanConstraintDemo.blocking_constraint_demo(runner: runner)
    assert demo.status == :blocked_by_human_constraint_gate
    assert demo.autonomous_fallback_still_supported
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.rendered =~ "Gated write verified: true"

    fallback = HumanConstraintDemo.autonomous_default_demo()
    assert fallback.status == :completed_without_human
    refute fallback.waited_for_human?
    refute fallback.human_surface_required?
  end

  test "mandatory human, raw bypass, and trusted write regressions fail closed" do
    assert {:error, :raw_bypass_forbidden} =
             HumanConstraintSurface.capture(attrs(%{forbidden_mechanic: "raw_apply_bypass"}))

    assert {:ok, capture} = HumanConstraintSurface.capture(attrs())

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{capture | humanRequiredForAutonomousLoop: true}
             |> HumanConstraintSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{capture | cliFallbackSupported: false}
             |> HumanConstraintSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | studioTrustedWriteAuthority: true}
             |> HumanConstraintSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | directArtifactWrite: true}
             |> HumanConstraintSurface.validate()
  end
end

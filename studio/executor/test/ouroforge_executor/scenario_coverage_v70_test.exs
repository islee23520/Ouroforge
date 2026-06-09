defmodule OuroforgeExecutor.ScenarioCoverageV70Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{DiagnosisCorrectionDemo, DiagnosisCorrectionSurface}

  @repo_root Path.expand("../../../..", __DIR__)
  @matrix_path Path.join(
                 @repo_root,
                 "examples/diagnosis-correction-intervention-feedback-v1/scenario-coverage-v70/matrix.fixture.json"
               )

  defp decode_json!(path) do
    path
    |> File.read!()
    |> OuroforgeExecutor.JSON.decode!()
  end

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        correction_id: "scenario-v70-threshold",
        diagnosis_id: "diag-v70-001",
        run_id: "run-v70-001",
        original_attribution: "asset-provenance-gap",
        corrected_attribution: "evaluator-threshold-drift",
        human_actor: "human://local-operator",
        correction_rationale: "Evidence shows the evaluator threshold drifted."
      },
      overrides
    )
  end

  test "coverage v70 matrix records diagnosis correction no-bypass and zero-human rows" do
    matrix = decode_json!(@matrix_path)

    assert matrix["schemaVersion"] ==
             "scenario-coverage-v70-diagnosis-correction-intervention-feedback-v1"

    assert matrix["coverageVersion"] == 70
    assert matrix["autonomyInvariants"]["humanInputRequired"] == false
    assert matrix["autonomyInvariants"]["loopCompletesWithoutHuman"] == true
    assert matrix["autonomyInvariants"]["studioTrustedWriteAuthority"] == false
    assert matrix["autonomyInvariants"]["elixirOwnsArtifactSemantics"] == false
    assert matrix["autonomyInvariants"]["rustDataPlaneOwnsValidation"] == true
    assert matrix["autonomyInvariants"]["opaqueMlUpdate"] == false
    assert matrix["autonomyInvariants"]["automatedFunTasteInference"] == false

    row_ids = Enum.map(matrix["rows"], & &1["id"])

    for required <- [
          "diagnosis-correction-recorded-through-existing-gates",
          "corrected-attribution-improves-subsequent-run",
          "no-raw-bypass-from-elixir-diagnosis-correction-surface",
          "loop-completes-without-human-input",
          "mandatory-human-and-opaque-inference-regressions-fail-closed",
          "coverage-v70-boundaries"
        ] do
      assert required in row_ids
    end

    assert Enum.all?(matrix["rows"], &(&1["status"] == "pass"))
  end

  test "Studio capture routes diagnosis correction as gated Rust evidence only" do
    assert {:ok, capture} = DiagnosisCorrectionSurface.capture(attrs())
    assert capture.routeCli == ["diagnosis-correction", "validate"]
    assert capture.interventionAsEvidence
    assert capture.readGatedWrite
    assert capture.cliFallbackSupported
    refute capture.directArtifactWrite
    refute capture.rawBypassRequested
    refute capture.studioTrustedWriteAuthority
    refute capture.elixirOwnsDiagnosisSemantics
    refute capture.opaqueMlUpdate
    refute capture.automatedFunTasteInference
    refute capture.humanRequiredForAutonomousLoop

    assert {:ok, submission} = DiagnosisCorrectionSurface.to_rust_submission(capture)
    assert submission["routeCli"] == ["diagnosis-correction", "validate"]
    assert submission["interventionAsEvidence"] == true
    assert submission["readGatedWrite"] == true
    assert submission["directArtifactWrite"] == false
    assert submission["elixirOwnsDiagnosisSemantics"] == false
  end

  test "demo records correction and keeps autonomous fallback available" do
    runner = fn argv, submission ->
      assert argv == ["diagnosis-correction", "validate"]
      assert submission["originalAttribution"] == "asset-provenance-gap"
      assert submission["correctedAttribution"] == "evaluator-threshold-drift"

      {:ok,
       %{
         "recorded" => true,
         "beforeAttribution" => "asset-provenance-gap",
         "afterAttribution" => "evaluator-threshold-drift",
         "priorUpdate" => %{"kind" => "transparent-heuristic-prior", "delta" => 4},
         "gatePath" => ["review/apply", "scene/source-apply", "evaluator", "evidence/provenance"]
       }}
    end

    assert {:ok, demo} = DiagnosisCorrectionDemo.corrected_attribution_demo(runner: runner)
    assert demo.status == :corrected_and_reattributed
    assert demo.after_attribution == "evaluator-threshold-drift"
    assert demo.autonomous_fallback_still_supported
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.rendered =~ "Gated write verified: true"

    fallback = DiagnosisCorrectionDemo.autonomous_default_demo()
    assert fallback.status == :completed_without_human
    refute fallback.waited_for_human?
    refute fallback.human_surface_required?
  end

  test "mandatory human, raw bypass, trusted write, and opaque inference regressions fail closed" do
    assert {:error, :raw_bypass_forbidden} =
             DiagnosisCorrectionSurface.capture(
               attrs(%{corrected_attribution: "raw_apply_bypass"})
             )

    assert {:ok, capture} = DiagnosisCorrectionSurface.capture(attrs())

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{capture | humanRequiredForAutonomousLoop: true}
             |> DiagnosisCorrectionSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{capture | cliFallbackSupported: false} |> DiagnosisCorrectionSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | studioTrustedWriteAuthority: true}
             |> DiagnosisCorrectionSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | directArtifactWrite: true} |> DiagnosisCorrectionSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | elixirOwnsDiagnosisSemantics: true}
             |> DiagnosisCorrectionSurface.validate()

    assert {:error, :opaque_or_fun_taste_inference_forbidden} =
             %{capture | opaqueMlUpdate: true} |> DiagnosisCorrectionSurface.validate()

    assert {:error, :opaque_or_fun_taste_inference_forbidden} =
             %{capture | automatedFunTasteInference: true}
             |> DiagnosisCorrectionSurface.validate()
  end
end

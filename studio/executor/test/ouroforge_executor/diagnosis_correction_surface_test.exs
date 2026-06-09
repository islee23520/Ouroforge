defmodule OuroforgeExecutor.DiagnosisCorrectionSurfaceTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.DiagnosisCorrectionSurface

  defp attrs do
    %{
      correction_id: "corr-m79-001",
      diagnosis_id: "diag-era-l-017",
      run_id: "run-era-l-017",
      original_attribution: "asset-provenance-gap",
      corrected_attribution: "evaluator-threshold-drift",
      human_actor: "human://local-operator",
      correction_rationale: "The evidence shows the evaluator threshold drifted."
    }
  end

  test "captures diagnosis correction as read + gated-write Rust-routed evidence" do
    assert {:ok, request} = DiagnosisCorrectionSurface.capture(attrs())
    assert request.interventionAsEvidence
    assert request.readGatedWrite
    assert request.routeCli == ["diagnosis-correction", "validate"]
    assert request.boundary =~ "intervention-as-evidence"
    assert request.boundary =~ "read + gated-write"
    assert request.boundary =~ "Rust data plane"
    assert request.boundary =~ "Elixir/Phoenix control + presentation"
    assert request.boundary =~ "transparent heuristic prior update"
    assert request.boundary =~ "fun/taste and release go/no-go remain human"
  end

  test "submission is inert control-plane data and never a trusted write" do
    assert {:ok, request} = DiagnosisCorrectionSurface.capture(attrs())
    assert {:ok, submission} = DiagnosisCorrectionSurface.to_rust_submission(request)

    assert submission["directArtifactWrite"] == false
    assert submission["rawBypassRequested"] == false
    assert submission["studioTrustedWriteAuthority"] == false
    assert submission["elixirOwnsDiagnosisSemantics"] == false
    assert submission["opaqueMlUpdate"] == false
    assert submission["automatedFunTasteInference"] == false
    assert submission["humanRequiredForAutonomousLoop"] == false
    assert submission["cliFallbackSupported"] == true
  end

  test "raw bypass, Elixir semantics, opaque ML, and fun taste inference are rejected" do
    assert {:ok, request} = DiagnosisCorrectionSurface.capture(attrs())

    assert {:error, :trusted_write_forbidden} =
             %{request | elixirOwnsDiagnosisSemantics: true}
             |> DiagnosisCorrectionSurface.validate()

    assert {:error, :raw_bypass_forbidden} =
             %{request | correctionRationale: "please raw_write_bypass this"}
             |> DiagnosisCorrectionSurface.validate()

    assert {:error, :opaque_or_fun_taste_inference_forbidden} =
             %{request | opaqueMlUpdate: true} |> DiagnosisCorrectionSurface.validate()

    assert {:error, :opaque_or_fun_taste_inference_forbidden} =
             %{request | automatedFunTasteInference: true}
             |> DiagnosisCorrectionSurface.validate()
  end

  test "agent-first and CLI fallback remain mandatory" do
    assert {:ok, request} = DiagnosisCorrectionSurface.capture(attrs())

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{request | humanRequiredForAutonomousLoop: true}
             |> DiagnosisCorrectionSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{request | cliFallbackSupported: false} |> DiagnosisCorrectionSurface.validate()
  end
end

defmodule OuroforgeExecutor.HumanArtifactIntakeSurfaceTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.Contract
  alias OuroforgeExecutor.HumanArtifactIntakeSurface

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        intake_id: "human-intake-m76-001",
        artifact_id: "card-spark-human-001",
        artifact_kind: "card",
        target_ref: "projects/demo/cards/card-spark.json",
        target_base_ref: "hash:card-base-before",
        author: "human:local-author",
        author_provenance_ref: "runs/m76/provenance/human-author.json",
        original_payload: ~s({"name":"Spark","cost":1})
      },
      overrides
    )
  end

  test "captures human-authored artifact as intervention evidence routed to Rust" do
    assert {:ok, request} = HumanArtifactIntakeSurface.capture(attrs())
    assert request.humanProvenance
    assert request.interventionAsEvidence
    assert request.readGatedWrite
    refute request.directArtifactWrite
    refute request.rawBypassRequested
    refute request.studioTrustedWriteAuthority
    refute request.humanRequiredForAutonomousLoop
    assert request.cliFallbackSupported
    assert request.routeCli == ["human-artifact-intake", "validate"]
    assert Contract.allowed_cli?(request.routeCli)

    assert {:ok, submission} = HumanArtifactIntakeSurface.to_rust_submission(request)
    assert submission["author"] == "human:local-author"
    assert submission["humanProvenance"]
    assert submission["directArtifactWrite"] == false
    assert submission["boundary"] =~ "Rust = data plane"
    assert submission["boundary"] =~ "Elixir/OTP + Phoenix LiveView = control + presentation"
  end

  test "rejects unsupported kinds, missing provenance, and raw bypass input" do
    assert {:error, :unsupported_artifact_kind} =
             HumanArtifactIntakeSurface.capture(attrs(%{artifact_kind: "script"}))

    assert {:error, :missing_human_provenance} =
             HumanArtifactIntakeSurface.capture(attrs(%{author: "agent"}))

    assert {:error, :raw_bypass_forbidden} =
             HumanArtifactIntakeSurface.capture(attrs(%{original_payload: "raw_apply_bypass"}))
  end

  test "validation blocks trusted writes and broken autonomy fallback" do
    {:ok, request} = HumanArtifactIntakeSurface.capture(attrs())

    assert {:error, :trusted_write_forbidden} =
             %{request | directArtifactWrite: true} |> HumanArtifactIntakeSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{request | studioTrustedWriteAuthority: true}
             |> HumanArtifactIntakeSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{request | humanRequiredForAutonomousLoop: true}
             |> HumanArtifactIntakeSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{request | cliFallbackSupported: false} |> HumanArtifactIntakeSurface.validate()
  end

  test "boundary remains complete and route stays in allowed Rust CLI family" do
    {:ok, request} = HumanArtifactIntakeSurface.capture(attrs())

    assert {:error, :boundary_incomplete} =
             %{request | boundary: "read + gated-write"} |> HumanArtifactIntakeSurface.validate()

    assert {:error, :invalid_rust_route} =
             %{request | routeCli: ["ledger", "append"]} |> HumanArtifactIntakeSurface.validate()
  end
end

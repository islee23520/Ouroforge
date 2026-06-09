defmodule OuroforgeExecutor.ProposalAmendmentSurfaceTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.Contract
  alias OuroforgeExecutor.ProposalAmendmentSurface

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        amendment_id: "amendment-m75-001",
        proposal_id: "proposal-agent-001",
        base_proposal_ref: "runs/m75/proposals/proposal-agent-001.before.json",
        human_actor: "local-human",
        edit_summary: "Tune the proposed config before approval.",
        amended_payload: ~s({"difficulty":"medium","budget":3})
      },
      overrides
    )
  end

  test "captures a Studio amendment as intervention evidence routed to Rust" do
    assert {:ok, request} = ProposalAmendmentSurface.capture(attrs())
    assert request.interventionAsEvidence
    assert request.readGatedWrite
    refute request.directArtifactWrite
    refute request.rawBypassRequested
    refute request.studioTrustedWriteAuthority
    refute request.humanRequiredForAutonomousLoop
    assert request.cliFallbackSupported
    assert request.routeCli == ["proposal-amendment", "validate"]
    assert Contract.allowed_cli?(request.routeCli)

    assert {:ok, submission} = ProposalAmendmentSurface.to_rust_submission(request)
    assert submission["interventionAsEvidence"]
    assert submission["directArtifactWrite"] == false
    assert submission["boundary"] =~ "Rust data plane"
    assert submission["boundary"] =~ "Elixir/Phoenix control + presentation"
  end

  test "rejects blank or raw bypass amendment input" do
    assert {:error, :missing_human_edit} =
             ProposalAmendmentSurface.capture(attrs(%{amended_payload: "  "}))

    assert {:error, :raw_bypass_forbidden} =
             ProposalAmendmentSurface.capture(attrs(%{edit_summary: "use raw_write_bypass"}))
  end

  test "validation blocks trusted writes and broken fallback" do
    {:ok, request} = ProposalAmendmentSurface.capture(attrs())

    assert {:error, :trusted_write_forbidden} =
             %{request | directArtifactWrite: true} |> ProposalAmendmentSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{request | studioTrustedWriteAuthority: true} |> ProposalAmendmentSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{request | humanRequiredForAutonomousLoop: true}
             |> ProposalAmendmentSurface.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{request | cliFallbackSupported: false} |> ProposalAmendmentSurface.validate()
  end

  test "boundary remains complete and route stays in allowed Rust CLI family" do
    {:ok, request} = ProposalAmendmentSurface.capture(attrs())

    assert {:error, :boundary_incomplete} =
             %{request | boundary: "read + gated-write"} |> ProposalAmendmentSurface.validate()

    assert {:error, :invalid_rust_route} =
             %{request | routeCli: ["ledger", "append"]} |> ProposalAmendmentSurface.validate()
  end
end

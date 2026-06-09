defmodule OuroforgeExecutor.ScenarioCoverageV66Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.ProposalAmendmentDemo
  alias OuroforgeExecutor.ProposalAmendmentSurface

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        amendment_id: "scenario-v66-amendment-001",
        proposal_id: "scenario-v66-proposal-001",
        base_proposal_ref: "runs/v66/proposals/proposal.before.json",
        human_actor: "local-human",
        edit_summary: "Tune the proposal before approval.",
        amended_payload: ~s({"difficulty":"medium","budget":3})
      },
      overrides
    )
  end

  test "v66 Studio captures read + gated-write and never a raw bypass or trusted write" do
    assert {:ok, capture} = ProposalAmendmentSurface.capture(attrs())
    assert capture.interventionAsEvidence
    assert capture.readGatedWrite
    assert capture.routeCli == ["proposal-amendment", "validate"]
    refute capture.directArtifactWrite
    refute capture.rawBypassRequested
    refute capture.studioTrustedWriteAuthority
    refute capture.humanRequiredForAutonomousLoop
    assert capture.cliFallbackSupported
    assert capture.boundary =~ "intervention-as-evidence"
    assert capture.boundary =~ "read + gated-write"
    assert capture.boundary =~ "#1 and #23 remain open"

    assert {:ok, submission} = ProposalAmendmentSurface.to_rust_submission(capture)
    assert submission["directArtifactWrite"] == false
    assert submission["rawBypassRequested"] == false
    assert submission["studioTrustedWriteAuthority"] == false
  end

  test "v66 Studio rejects raw bypass language and mandatory human dependencies" do
    assert {:error, :raw_bypass_forbidden} =
             ProposalAmendmentSurface.capture(attrs(%{amended_payload: "raw_apply_bypass"}))

    assert {:ok, capture} = ProposalAmendmentSurface.capture(attrs())

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{capture | humanRequiredForAutonomousLoop: true}
             |> ProposalAmendmentSurface.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | studioTrustedWriteAuthority: true}
             |> ProposalAmendmentSurface.validate()
  end

  test "v66 autonomous default completes without human input" do
    demo = ProposalAmendmentDemo.autonomous_default_demo()

    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    assert demo.cli_fallback_supported
    refute demo.trusted_write_performed
    refute demo.gated_write_attempted
    assert demo.boundary =~ "local-first CLI fallback"
  end

  test "v66 failed Rust reverify blocks review/apply readiness" do
    runner = fn argv, artifact ->
      assert argv == ["proposal-amendment", "validate"]
      assert artifact["status"] == "blocked"
      assert Enum.any?(artifact["gateResults"], &(&1["status"] == "failed"))
      assert artifact["studioTrustedWriteAuthority"] == false
      assert artifact["rawBypassRequested"] == false
      {:error, %{"readyForReviewApply" => false, "blockedReasons" => ["Evaluator:Failed"]}}
    end

    assert {:ok, demo} = ProposalAmendmentDemo.blocked_amendment_demo(runner: runner)
    assert demo.status == :blocked_by_gate
    assert demo.rust_result["readyForReviewApply"] == false
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
  end
end

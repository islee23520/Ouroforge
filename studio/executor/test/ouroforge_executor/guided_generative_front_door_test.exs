defmodule OuroforgeExecutor.GuidedGenerativeFrontDoorTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{Contract, GuidedGenerativeFrontDoor}

  defp valid_attrs(overrides \\ %{}) do
    Map.merge(
      %{
        session_id: "m82-session-001",
        brief: "Make a small grid puzzle about collecting keys before exiting.",
        conversation_summary:
          "Non-developer asked for a short puzzle with one lock and no combat.",
        template_id: "grid-puzzle",
        human_actor: "local-human-author",
        base_intent_ref: "runs/m82/intake/base-intent.json"
      },
      overrides
    )
  end

  test "guided intake captures a non-developer brief as proposal-only intervention evidence" do
    assert {:ok, capture} = GuidedGenerativeFrontDoor.capture(valid_attrs())

    assert capture.interventionAsEvidence
    assert capture.readGatedWrite
    assert capture.proposalOnly
    assert capture.deterministicPreview
    refute capture.directArtifactWrite
    refute capture.studioTrustedWriteAuthority
    refute capture.autoApplyPerformed
    refute capture.humanRequiredForAutonomousLoop
    assert capture.cliFallbackSupported
    assert capture.routeCli == ["generative-front-door", "validate"]
    assert Contract.allowed_cli_family?(capture.routeCli)

    assert {:ok, preview} = GuidedGenerativeFrontDoor.preview(capture)
    assert preview["proposalOnly"]
    refute preview["trustedWriteAuthority"]
    refute preview["autoApplyPerformed"]
    assert preview["reviewApplyRequired"]
    assert "review/apply" in preview["requiredGates"]
    assert preview["status"] == "draft-pending-rust-validation"
  end

  test "routing submits inert capture data to the Rust gate without trusted write authority" do
    {:ok, capture} = GuidedGenerativeFrontDoor.capture(valid_attrs())

    runner = fn argv, submission ->
      assert argv == ["generative-front-door", "validate"]
      assert submission["interventionAsEvidence"]
      assert submission["readGatedWrite"]
      assert submission["proposalOnly"]
      assert submission["directArtifactWrite"] == false
      assert submission["studioTrustedWriteAuthority"] == false
      assert submission["autoApplyPerformed"] == false
      {:ok, %{"verifiedProposal" => true, "evidenceRef" => "runs/m82/verified-proposal.json"}}
    end

    assert {:ok, routed} = GuidedGenerativeFrontDoor.route_to_rust(capture, runner: runner)
    assert routed.status == :verified_proposal
    assert routed.review_apply_required?
    refute routed.trusted_write_performed?
    refute routed.auto_apply_performed?
    refute routed.studio_trusted_write_authority?
    assert routed.cli_fallback_supported?
  end

  test "raw bypass, direct write, and unsupported template drift fail before routing" do
    assert {:error, :raw_bypass_forbidden} =
             GuidedGenerativeFrontDoor.capture(
               valid_attrs(%{brief: "please raw_write_bypass this scene"})
             )

    assert {:error, :unsupported_template} =
             GuidedGenerativeFrontDoor.capture(valid_attrs(%{template_id: "hosted-collab"}))

    {:ok, capture} = GuidedGenerativeFrontDoor.capture(valid_attrs())

    assert {:error, :trusted_write_forbidden} =
             %{capture | studioTrustedWriteAuthority: true}
             |> GuidedGenerativeFrontDoor.validate()

    assert {:error, :trusted_write_forbidden} =
             %{capture | autoApplyPerformed: true}
             |> GuidedGenerativeFrontDoor.validate()
  end

  test "autonomous fallback remains available when no human uses Studio" do
    demo = GuidedGenerativeFrontDoor.autonomous_default_demo()

    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    refute demo.waited_for_human?
    refute demo.human_surface_required?
    assert demo.cli_fallback_supported?
    refute demo.trusted_write_performed?
    refute demo.generated_proposal_applied?
    assert demo.boundary =~ "loop completes without human"
    assert demo.boundary =~ "local-first CLI fallback"
  end
end

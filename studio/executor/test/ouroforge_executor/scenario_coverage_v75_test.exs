defmodule OuroforgeExecutor.ScenarioCoverageV75Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{StudioHumanGradeDemo, StudioInterventionPanels, StudioLiveShell}

  @coverage_doc Path.expand(
                  "../../../../docs/scenario-coverage-v75-human-grade-studio.md",
                  __DIR__
                )

  test "v75 coverage document records Human-Grade Studio boundaries" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v75"
    assert doc =~ "Human-Grade Studio"
    assert doc =~ "read + gated-write"
    assert doc =~ "intervention-as-evidence"
    assert doc =~ "Rust remains the data plane"
    assert doc =~ "Elixir/OTP + Phoenix LiveView is control + presentation only"
    assert doc =~ "No trusted Elixir write authority"
    assert doc =~ "#1 and #23 remain open"
  end

  test "v75 live shell renders Rust-owned read models and rejects authority drift" do
    assert {:ok, shell} = StudioLiveShell.new(%{active: :evidence})
    rendered = StudioLiveShell.render(shell)

    assert rendered.readOnlyRendering
    assert rendered.readGatedWrite
    assert rendered.interventionAsEvidence
    assert rendered.rustDataPlaneOwnsTruth
    assert rendered.localFirst
    assert rendered.cliFallbackSupported
    refute rendered.trustedWriteAuthority
    refute rendered.directArtifactWrite
    refute rendered.commandBridge

    for kind <- [:evidence, :diagnosis, :journal, :verdict] do
      assert {:ok, event} =
               StudioLiveShell.rust_owned_event(kind, %{
                 id: "v75-#{kind}",
                 status: :fresh,
                 evidence_refs: ["runs/v75/#{kind}.json"]
               })

      assert event.rustOwned
      assert event.readOnly
      refute event.trustedWriteAuthority
      refute event.directArtifactWrite
      refute event.commandBridge
    end

    assert {:error, :trusted_write_or_scope_drift} =
             StudioLiveShell.new(%{trusted_write_authority: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioLiveShell.new(%{command_bridge: true})

    assert {:error, :autonomy_or_cli_fallback_broken} =
             StudioLiveShell.new(%{autonomous_loop_requires_human: true})
  end

  test "v75 integrated panels route every write through existing gates" do
    assert {:ok, surface} = StudioInterventionPanels.new()
    rendered = StudioInterventionPanels.render(surface)

    assert rendered.readGatedWrite
    assert rendered.interventionAsEvidence
    assert rendered.rustDataPlaneRequired
    refute rendered.trustedWriteAuthority
    refute rendered.directArtifactWrite
    refute rendered.commandBridge

    submissions = [
      submit(:steering, steering_attrs()),
      submit(:amendment, amendment_attrs()),
      submit(:constraint, constraint_attrs()),
      submit(:correction, correction_attrs()),
      submit(:takeover, stage_attrs()),
      submit(:handback, stage_attrs()),
      submit(:authoring, authoring_attrs())
    ]

    assert Enum.all?(submissions, &(&1.status == :queued_for_rust_gate))
    assert Enum.all?(submissions, & &1.interventionAsEvidence)
    assert Enum.all?(submissions, & &1.readGatedWrite)
    assert Enum.all?(submissions, & &1.rustDataPlaneRequired)
    assert Enum.all?(submissions, &(not &1.directArtifactWrite))
    assert Enum.all?(submissions, &(not &1.trustedWriteAuthority))
    assert Enum.all?(submissions, &(not &1.commandBridge))
    assert Enum.all?(submissions, &(not &1.elixirOwnsArtifactSemantics))

    assert Enum.any?(submissions, &(&1.gateFamily =~ "review/apply"))
    assert Enum.any?(submissions, &(&1.gateFamily =~ "human constraint"))
    assert Enum.any?(submissions, &(&1.gateFamily =~ "diagnosis correction"))
    assert Enum.any?(submissions, &(&1.gateFamily =~ "steering directive"))
  end

  test "v75 authoring remains review apply and scene source apply only" do
    submission = submit(:authoring, authoring_attrs())

    assert submission.kind == :authoring
    assert submission.gateFamily =~ "review/apply"
    assert submission.gateFamily =~ "scene/source-apply"

    assert submission.payload.existingGates == [
             "review/apply",
             "scene/source-apply",
             "evaluator",
             "evidence/provenance"
           ]

    assert submission.payload.directArtifactWrite == false
    assert submission.payload.autoApplyPerformed == false
    assert submission.payload.rustRoute == ["source-apply", "review", "draft-v75-1"]

    assert {:error, :trusted_write_forbidden} =
             StudioInterventionPanels.submit(
               :authoring,
               Map.put(authoring_attrs(), :direct_artifact_write, true)
             )

    assert {:error, :command_bridge_forbidden} =
             StudioInterventionPanels.submit(
               :authoring,
               Map.put(authoring_attrs(), :command_bridge, true)
             )
  end

  test "v75 demo observes intervenes and preserves no-human fallback" do
    demo = StudioHumanGradeDemo.run()

    assert StudioHumanGradeDemo.read_gated_write?(demo)
    assert StudioHumanGradeDemo.autonomous_first?(demo)
    assert demo.live_feedback?
    assert demo.gated_write_verified?
    assert demo.autonomous_fallback_verified?
    assert demo.autonomous.status == :completed_without_human
    refute demo.autonomous.human_surface_required?
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.command_bridge?

    rendered = StudioHumanGradeDemo.render(demo)
    assert rendered =~ "M85 Human-Grade Studio demo"
    assert rendered =~ "Gated intervention verified: true"
    assert rendered =~ "Autonomous fallback verified: true"
    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
    refute rendered =~ "hosted"
  end

  defp submit(kind, attrs) do
    assert {:ok, submission} = StudioInterventionPanels.submit(kind, attrs)
    submission
  end

  defp steering_attrs do
    %{
      id: "steer-v75-1",
      campaign_id: "v75-studio",
      action: :reprioritize,
      target: "diagnosis-refresh",
      actor_id: "human-operator",
      reason: "Prioritize diagnosis evidence refresh",
      issued_at: "2026-06-09T00:00:00Z",
      base_refs: ["runs/v75/live-state.json"]
    }
  end

  defp amendment_attrs do
    %{
      amendment_id: "amend-v75-1",
      proposal_id: "proposal-v75-1",
      base_proposal_ref: "runs/v75/proposals/proposal.json",
      human_actor: "human-operator",
      edit_summary: "Clarify a gated proposal",
      amended_payload: "{\"copy\":\"gated copy\"}"
    }
  end

  defp constraint_attrs do
    %{
      constraint_id: "constraint-v75-1",
      kind: "required-style",
      author: "human-operator",
      author_provenance_ref: "runs/v75/human/style-author.json",
      target_ref: "runs/v75/candidate.json",
      target_base_ref: "runs/v75/base.json",
      normalized_constraint_ref: "runs/v75/constraints/style.json",
      review_apply_ref: "runs/v75/review/style.json",
      evaluator_evidence_ref: "runs/v75/evaluator/style.json",
      evidence_refs: ["runs/v75/evidence/style.json"],
      required_style: "Use concise accessible Studio labels"
    }
  end

  defp correction_attrs do
    %{
      correction_id: "correction-v75-1",
      diagnosis_id: "diagnosis-v75-1",
      run_id: "run-v75",
      original_attribution: "theme panel",
      corrected_attribution: "stale evidence ref",
      human_actor: "human-operator",
      correction_rationale: "Rust evidence points at a stale ref"
    }
  end

  defp stage_attrs do
    %{
      stage_id: "stage-v75-authoring",
      campaign_id: "v75-studio",
      actor_id: "human-operator",
      reason: "Inspect authoring draft before handback"
    }
  end

  defp authoring_attrs do
    %{
      draft_id: "draft-v75-1",
      target_ref: "runs/v75/drafts/scene-draft.json",
      target_base_ref: "runs/v75/base/scene.hash",
      review_apply_ref: "runs/v75/review/draft-v75-1.json",
      scene_or_source_apply_ref: "runs/v75/source-apply/draft-v75-1.json",
      summary: "Preview a scene/source authoring draft through existing gates"
    }
  end
end

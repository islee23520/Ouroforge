defmodule OuroforgeExecutor.StudioInterventionPanelsTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{LocalPubSub, StudioInterventionPanels}

  test "integrated panels enumerate Era M interventions and authoring as gated writes" do
    assert {:ok, surface} = StudioInterventionPanels.new()
    rendered = StudioInterventionPanels.render(surface)

    assert Map.keys(rendered.panels) |> Enum.sort() ==
             [:amendment, :authoring, :constraint, :correction, :handback, :steering, :takeover]

    assert rendered.boundary =~ "intervention-as-evidence"
    assert rendered.boundary =~ "read + gated-write"
    assert rendered.boundary =~ "two-plane"
    assert rendered.boundary =~ "local-first"
    assert rendered.readGatedWrite
    assert rendered.interventionAsEvidence
    assert rendered.rustDataPlaneRequired
    refute rendered.trustedWriteAuthority
    refute rendered.directArtifactWrite
    refute rendered.commandBridge
    assert rendered.cliFallbackSupported
  end

  test "steering amendment constraint correction takeover handback and authoring route to gates" do
    registry = Module.concat(__MODULE__, "Registry#{System.unique_integer([:positive])}")
    start_supervised!({Registry, keys: :duplicate, name: registry})
    assert {:ok, _} = LocalPubSub.subscribe(StudioInterventionPanels.topic(), registry)

    submissions = [
      submit(:steering, steering_attrs(), registry),
      submit(:amendment, amendment_attrs(), registry),
      submit(:constraint, constraint_attrs(), registry),
      submit(:correction, correction_attrs(), registry),
      submit(:takeover, stage_attrs(), registry),
      submit(:handback, stage_attrs(), registry),
      submit(:authoring, authoring_attrs(), registry)
    ]

    assert Enum.map(submissions, & &1.kind) ==
             [:steering, :amendment, :constraint, :correction, :takeover, :handback, :authoring]

    assert Enum.all?(submissions, &(&1.status == :queued_for_rust_gate))
    assert Enum.all?(submissions, & &1.interventionAsEvidence)
    assert Enum.all?(submissions, & &1.readGatedWrite)
    assert Enum.all?(submissions, & &1.rustDataPlaneRequired)
    assert Enum.all?(submissions, &(not &1.directArtifactWrite))
    assert Enum.all?(submissions, &(not &1.trustedWriteAuthority))
    assert Enum.all?(submissions, &(not &1.commandBridge))
    assert Enum.all?(submissions, &(not &1.elixirOwnsArtifactSemantics))

    assert Enum.any?(submissions, &(&1.gateFamily =~ "scene/source-apply"))
    assert Enum.any?(submissions, &(&1.gateFamily =~ "human constraint"))
    assert Enum.any?(submissions, &(&1.gateFamily =~ "diagnosis correction"))

    for submission <- submissions do
      assert_receive {:ouroforge_executor_pubsub, "studio:interventions",
                      {:studio_intervention_panel, ^submission}}
    end
  end

  test "bypass command bridge direct-write hosted-store and mandatory-human drift fail closed" do
    assert {:error, :trusted_write_or_scope_drift} =
             StudioInterventionPanels.new(%{trusted_write_authority: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioInterventionPanels.new(%{direct_artifact_write: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioInterventionPanels.new(%{command_bridge: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioInterventionPanels.new(%{new_data_store: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioInterventionPanels.new(%{hosted_collaborative: true})

    assert {:error, :autonomy_or_cli_fallback_broken} =
             StudioInterventionPanels.new(%{human_required_for_autonomous_loop: true})

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

    assert {:error, :raw_bypass_forbidden} =
             StudioInterventionPanels.submit(
               :takeover,
               Map.put(stage_attrs(), :reason, "raw_write_bypass")
             )
  end

  defp submit(kind, attrs, registry) do
    assert {:ok, submission} = StudioInterventionPanels.submit(kind, attrs, pubsub: registry)
    submission
  end

  defp steering_attrs do
    %{
      id: "steer-m85-1",
      campaign_id: "m85-studio",
      action: :reprioritize,
      target: "diagnosis-refresh",
      actor_id: "human-operator",
      reason: "Surface latest diagnosis after evidence refresh",
      issued_at: "2026-06-09T00:00:00Z",
      base_refs: ["runs/m85/live-state.json"]
    }
  end

  defp amendment_attrs do
    %{
      amendment_id: "amend-m85-1",
      proposal_id: "proposal-m85-1",
      base_proposal_ref: "runs/m85/proposals/proposal.json",
      human_actor: "human-operator",
      edit_summary: "Clarify the copy while preserving gate evidence",
      amended_payload: "{\"copy\":\"clear gated copy\"}",
      provenance_refs: ["runs/m85/provenance/amend.json"]
    }
  end

  defp constraint_attrs do
    %{
      constraint_id: "constraint-m85-1",
      kind: "required-style",
      author: "human-operator",
      author_provenance_ref: "runs/m85/human/style-author.json",
      target_ref: "runs/m85/candidate.json",
      target_base_ref: "runs/m85/base.json",
      normalized_constraint_ref: "runs/m85/constraints/style.json",
      review_apply_ref: "runs/m85/review/style.json",
      evaluator_evidence_ref: "runs/m85/evaluator/style.json",
      evidence_refs: ["runs/m85/evidence/style.json"],
      required_style: "Use concise accessible Studio labels"
    }
  end

  defp correction_attrs do
    %{
      correction_id: "correction-m85-1",
      diagnosis_id: "diagnosis-m85-1",
      run_id: "run-m85",
      original_attribution: "theme panel",
      corrected_attribution: "stale evidence ref",
      human_actor: "human-operator",
      correction_rationale: "The latest Rust evidence points at a stale ref, not theme rendering"
    }
  end

  defp stage_attrs do
    %{
      stage_id: "stage-m85-authoring",
      campaign_id: "m85-studio",
      actor_id: "human-operator",
      reason: "Temporarily inspect authoring draft before handback",
      provenance_refs: ["runs/m85/stage/takeover.json"]
    }
  end

  defp authoring_attrs do
    %{
      draft_id: "draft-m85-1",
      target_ref: "runs/m85/drafts/scene-draft.json",
      target_base_ref: "runs/m85/base/scene.hash",
      review_apply_ref: "runs/m85/review/draft-m85-1.json",
      scene_or_source_apply_ref: "runs/m85/source-apply/draft-m85-1.json",
      summary: "Preview a scene/source authoring draft through existing gates",
      provenance_refs: ["runs/m85/provenance/draft-m85-1.json"]
    }
  end
end

defmodule OuroforgeExecutor.ReDerivationUXTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{ReDerivationSession, ReDerivationUX}

  defp evidence_fixture do
    %{
      project_id: "era-r-m113-demo",
      source_project_ref: "source-projects/m113/sample-project/project.gltf",
      target_dimensionality: :three_d,
      claimed_ported_units: [],
      units: [
        %{
          unit_id: "unit.door-open",
          behavioral_unit_ref: "ir/m108/unit.door-open.json",
          source_ref: "source-projects/m113/sample-project/door.tscn",
          oracle_ref: "oracles/m109/unit.door-open.json",
          reexpression_ref: "drafts/m110/unit.door-open.json",
          ab_evidence_ref: "evidence/m111/unit.door-open.json",
          coverage_ref: "coverage/m112/unit.door-open.json",
          primary_state_hash: "fnv64:aaaaaaaaaaaaaaaa",
          secondary_render_digest: "ssim:0.996;pixel-diff:0.001",
          grade: :green,
          oracle_status: :captured,
          ab_status: :passed,
          coverage_status: :verified,
          pipeline_stage: :semantic_coverage,
          gap_summary: []
        },
        %{
          unit_id: "unit.jump-feel",
          behavioral_unit_ref: "ir/m108/unit.jump-feel.json",
          source_ref: "source-projects/m113/sample-project/player.tscn",
          oracle_ref: "oracles/m109/unit.jump-feel.json",
          ab_evidence_ref: "evidence/m111/unit.jump-feel.json",
          coverage_ref: "coverage/m112/unit.jump-feel.json",
          primary_state_hash: "fnv64:bbbbbbbbbbbbbbbb",
          secondary_render_digest: "ssim:0.982;pixel-diff:0.010",
          grade: :yellow,
          oracle_status: :captured,
          ab_status: :needs_repair,
          coverage_status: :human_escalated,
          pipeline_stage: :verify,
          question_prompt: "Does the jump apex feel intentional?",
          intent_feel_escalation: true,
          gap_summary: ["jump feel needs Ring 2 human intent review"],
          re_derivation_tasks: ["retune native jump arc after human clarification"]
        },
        %{
          unit_id: "unit.shader-vfx",
          behavioral_unit_ref: "ir/m108/unit.shader-vfx.json",
          source_ref: "source-projects/m113/sample-project/vfx.tres",
          grade: :red,
          oracle_status: :missing,
          ab_status: :not_run,
          coverage_status: :blocked,
          pipeline_stage: :interrogate,
          question_prompt: "Should this VFX be approximated or deferred?",
          gap_summary: ["source shader behavior is unsupported and must be re-derived"],
          re_derivation_tasks: ["capture human intent and reject/defer unsupported shader feel"]
        }
      ]
    }
  end

  test "surface renders re-derivation pipeline evidence without port claims" do
    assert {:ok, surface} = ReDerivationUX.surface(evidence_fixture())

    assert surface.summary == %{green: 1, yellow: 1, red: 1, escalated: 2}
    assert surface.liveViewPresentation
    assert surface.readGatedWrite
    assert surface.rustDataPlaneOwnsTruth
    refute surface.elixirOwnsArtifactSemantics
    refute surface.directArtifactWrite
    refute surface.studioTrustedWriteAuthority
    refute surface.finishedGameAutoPort
    assert surface.claimedPortedUnits == []

    assert Enum.any?(
             surface.rustShapes,
             &(&1.path == "crates/ouroforge-core/src/semantic_port_coverage.rs")
           )

    assert {:ok, rendered} = ReDerivationUX.render(surface)
    assert rendered =~ "Re-derivation UX"
    assert rendered =~ "Fidelity: 🟢 1 / 🟡 1 / 🔴 1"
    assert rendered =~ "Intent/feel escalations: 2"
    assert rendered =~ "no auto-port without oracle"
  end

  test "intent and feel queue is human owned and gated through allowed CLI preview" do
    assert {:ok, surface} = ReDerivationUX.surface(evidence_fixture())
    assert {:ok, [jump, shader]} = ReDerivationUX.escalation_queue(surface)

    assert jump.unitId == "unit.jump-feel"
    assert jump.targetRing == "Ring 2"
    assert jump.humanOwned
    assert jump.readGatedWrite
    refute jump.trustedWriteAuthority
    refute jump.directArtifactWrite
    refute jump.portClaimAllowed
    assert Enum.take(jump.routeCli, 3) == ["behavior", "draft", "preview"]

    runner = fn executable, argv, _opts ->
      assert executable == "ouroforge"
      assert argv == jump.routeCli ++ ["--human-note-preview"]
      {"preview queued", 0}
    end

    assert {:ok, result} =
             ReDerivationUX.submit_intent_feel(jump, "Apex should feel floaty", runner: runner)

    assert result.status == 0
    assert shader.reason == "blocked re-derivation task"
  end

  test "surface fails closed on port claims trusted writes unsafe refs and missing oracle" do
    port_claim = put_in(evidence_fixture()[:claimed_ported_units], ["unit.door-open"])
    assert {:error, :ported_claim_forbidden} = ReDerivationUX.surface(port_claim)

    trusted = Map.put(evidence_fixture(), :studio_trusted_write_authority, true)
    assert {:error, :trusted_write_forbidden} = ReDerivationUX.surface(trusted)

    unsafe =
      put_in(evidence_fixture()[:units], [
        Map.put(hd(evidence_fixture()[:units]), :source_ref, "decompiled/Assembly-CSharp/Door.cs")
      ])

    assert {:error, :invalid_unit_card} = ReDerivationUX.surface(unsafe)

    missing_oracle =
      update_in(evidence_fixture()[:units], fn [green | rest] ->
        [Map.delete(green, :oracle_ref) | rest]
      end)

    assert {:error, :invalid_unit_card} = ReDerivationUX.surface(missing_oracle)
  end

  test "3d green rows require state hash primary and render secondary" do
    no_hash =
      update_in(evidence_fixture()[:units], fn [green | rest] ->
        [Map.put(green, :primary_state_hash, "sha256:not-state-hash") | rest]
      end)

    assert {:error, :determinism_evidence_required} = ReDerivationUX.surface(no_hash)

    no_render =
      update_in(evidence_fixture()[:units], fn [green | rest] ->
        [Map.delete(green, :secondary_render_digest) | rest]
      end)

    assert {:error, :determinism_evidence_required} = ReDerivationUX.surface(no_render)
  end

  test "OTP session stores presentation state and broadcasts PubSub events" do
    assert {:ok, _} = OuroforgeExecutor.LocalPubSub.subscribe(ReDerivationUX.pubsub_topic())
    {:ok, pid} = start_supervised({ReDerivationSession, name: nil, session_id: "m113-test"})

    assert {:ok, surface, escalations} =
             ReDerivationSession.attach_evidence(pid, evidence_fixture())

    assert_receive {:ouroforge_executor_pubsub, "studio:rederivation-ux",
                    %{event: :evidence_ready}}

    assert length(escalations) == 2

    runner = fn _exe, _argv, _opts -> {"preview queued", 0} end

    assert {:ok, result} =
             ReDerivationSession.submit_intent_feel(pid, "unit.jump-feel", "Keep floaty",
               runner: runner
             )

    assert_receive {:ouroforge_executor_pubsub, "studio:rederivation-ux",
                    %{event: :intent_feel_routed}}

    assert result.status == 0

    state = ReDerivationSession.state(pid)
    assert state.status == :intent_feel_routed
    assert state.surface == surface
    assert Enum.all?(state.escalations, &(&1.trustedWriteAuthority == false))
  end
end

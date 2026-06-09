defmodule OuroforgeExecutor.StudioHumanGradeDemoTest do
  use ExUnit.Case, async: false
  @moduletag :demo

  alias OuroforgeExecutor.{
    LocalPubSub,
    StudioHumanGradeDemo,
    StudioInterventionPanels,
    StudioLiveShell
  }

  test "M85 demo observes live campaign, submits gated intervention, and keeps fallback autonomous" do
    registry = Module.concat(__MODULE__, "Registry#{System.unique_integer([:positive])}")
    start_supervised!({Registry, keys: :duplicate, name: registry})
    assert {:ok, _} = LocalPubSub.subscribe(StudioLiveShell.topics().diagnosis, registry)
    assert {:ok, _} = LocalPubSub.subscribe(StudioInterventionPanels.topic(), registry)

    demo = StudioHumanGradeDemo.run(pubsub: registry)

    assert demo.version == "m85-human-grade-studio-demo-v1"
    assert demo.boundary == :read_gated_write_demo_no_elixir_artifact_writes
    assert StudioHumanGradeDemo.read_gated_write?(demo)
    assert StudioHumanGradeDemo.autonomous_first?(demo)
    assert demo.live_feedback?
    assert demo.observed_shell.active == :diagnosis
    assert demo.intervention.kind == :steering
    assert demo.intervention.status == :queued_for_rust_gate
    assert demo.intervention.gateFamily =~ "steering directive gate"
    assert demo.intervention.routeCli == ["loop", "step"]
    assert demo.autonomous.evidence_ref == "runs/m85/demo/no-human-loop.json"

    assert_receive {:ouroforge_executor_pubsub, "studio:diagnosis",
                    {:studio_live_shell, %{kind: :diagnosis, rustOwned: true, readOnly: true}}}

    assert_receive {:ouroforge_executor_pubsub, "studio:interventions",
                    {:studio_intervention_panel, submitted}}

    assert submitted == demo.intervention
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.command_bridge?
  end

  test "M85 rendered demo is conservative and names the gate boundary" do
    rendered =
      StudioHumanGradeDemo.run()
      |> StudioHumanGradeDemo.render()

    assert rendered =~ "M85 Human-Grade Studio demo"
    assert rendered =~ "trusted writes: false"
    assert rendered =~ "Direct artifact write: false"
    assert rendered =~ "command bridge: false"
    assert rendered =~ "Gated intervention verified: true"
    assert rendered =~ "Autonomous fallback verified: true"
    assert rendered =~ "Intervention route: loop step"

    assert rendered =~
             "operator steering is intervention-as-evidence queued for existing Rust gates"

    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
    refute rendered =~ "hosted"
  end
end

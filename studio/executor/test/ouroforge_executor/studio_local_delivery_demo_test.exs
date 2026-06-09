defmodule OuroforgeExecutor.StudioLocalDeliveryDemoTest do
  use ExUnit.Case, async: false
  @moduletag :demo

  alias OuroforgeExecutor.{
    LocalPubSub,
    StudioInterventionPanels,
    StudioLiveShell,
    StudioLocalDeliveryDemo
  }

  test "M86 demo shows local package install UX, gated human write, and autonomous fallback" do
    registry = Module.concat(__MODULE__, "Registry#{System.unique_integer([:positive])}")
    start_supervised!({Registry, keys: :duplicate, name: registry})
    assert {:ok, _} = LocalPubSub.subscribe(StudioLiveShell.topics().evidence, registry)
    assert {:ok, _} = LocalPubSub.subscribe(StudioInterventionPanels.topic(), registry)

    demo = StudioLocalDeliveryDemo.run(pubsub: registry)

    assert demo.version == "m86-studio-local-delivery-demo-v1"
    assert demo.boundary == :local_package_read_gated_write_demo_no_elixir_artifact_writes
    assert demo.install_ux_visible?
    assert StudioLocalDeliveryDemo.read_gated_write?(demo)
    assert StudioLocalDeliveryDemo.autonomous_first?(demo)
    assert StudioLocalDeliveryDemo.smoke_verified?(demo)
    assert demo.observed_shell.active == :evidence
    assert demo.intervention.kind == :constraint
    assert demo.intervention.status == :queued_for_rust_gate
    assert demo.intervention.gateFamily =~ "human constraint"
    assert demo.intervention.routeCli == ["evaluate"]

    assert demo.autonomous.commands == [
             "cargo build --workspace --jobs 2",
             "cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2"
           ]

    assert_receive {:ouroforge_executor_pubsub, "studio:evidence",
                    {:studio_live_shell, %{kind: :evidence, rustOwned: true, readOnly: true}}}

    assert_receive {:ouroforge_executor_pubsub, "studio:interventions",
                    {:studio_intervention_panel, submitted}}

    assert submitted == demo.intervention
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.command_bridge?
    refute demo.hosted_collaborative?
  end

  test "M86 demo render is conservative and names generated smoke evidence" do
    rendered =
      StudioLocalDeliveryDemo.run()
      |> StudioLocalDeliveryDemo.render()

    assert rendered =~ "M86 Studio local delivery demo"
    assert rendered =~ "trusted writes: false"
    assert rendered =~ "Direct artifact write: false"
    assert rendered =~ "command bridge: false"
    assert rendered =~ "Hosted collaborative: false"
    assert rendered =~ "Install UX visible: true"
    assert rendered =~ "cargo build --workspace --jobs 2"
    assert rendered =~ "mix run --no-halt"
    assert rendered =~ "runs/studio-local-package-smoke-v1/smoke.json"
    assert rendered =~ "Gated intervention verified: true"
    assert rendered =~ "Autonomous fallback verified: true"

    assert rendered =~
             "human package write is intervention-as-evidence queued for existing Rust gates"

    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
    refute rendered =~ "release channel"
  end
end

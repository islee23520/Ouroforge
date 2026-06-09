defmodule OuroforgeExecutor.StudioLiveShellTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.StudioLiveShell

  test "shell renders navigation and read-only Rust-owned view contracts" do
    assert {:ok, shell} = StudioLiveShell.new(%{active: :diagnosis})
    rendered = StudioLiveShell.render(shell)

    assert rendered.shell == "Phoenix LiveView Studio Shell"
    assert rendered.active == :diagnosis
    assert Enum.map(rendered.nav, & &1.kind) == [:diagnosis, :evidence, :journal, :verdict]
    assert rendered.readOnlyRendering
    assert rendered.readGatedWrite
    assert rendered.interventionAsEvidence
    assert rendered.rustDataPlaneOwnsTruth
    refute rendered.trustedWriteAuthority
    refute rendered.directArtifactWrite
    refute rendered.commandBridge
    assert rendered.localFirst
    assert rendered.cliFallbackSupported
    refute rendered.autonomousLoopRequiresHuman

    assert rendered.boundary =~ "two-plane"
    assert rendered.boundary =~ "local-first"
    assert rendered.boundary =~ "read + gated-write"
    assert rendered.boundary =~ "intervention-as-evidence"
  end

  test "local PubSub delivers live evidence diagnosis journal and verdict updates" do
    registry = Module.concat(__MODULE__, "Registry#{System.unique_integer([:positive])}")
    start_supervised!({Registry, keys: :duplicate, name: registry})

    assert {:ok, shell} = StudioLiveShell.new()
    assert {:ok, ^shell} = StudioLiveShell.subscribe(shell, registry)

    for kind <- [:evidence, :diagnosis, :journal, :verdict] do
      assert {:ok, event} =
               StudioLiveShell.broadcast(
                 kind,
                 %{
                   id: "#{kind}-1",
                   title: "#{kind} read model",
                   status: :fresh,
                   evidence_refs: ["runs/m85/#{kind}.json"]
                 },
                 registry
               )

      topic = StudioLiveShell.topics()[kind]
      assert_receive {:ouroforge_executor_pubsub, ^topic, {:studio_live_shell, ^event}}
      assert event.rustOwned
      assert event.readOnly
      refute event.trustedWriteAuthority
      refute event.directArtifactWrite
      refute event.commandBridge
    end
  end

  test "events update shell views without granting write authority" do
    assert {:ok, shell} = StudioLiveShell.new()

    assert {:ok, event} =
             StudioLiveShell.rust_owned_event(:evidence, %{
               id: "evidence-run-1",
               status: :fresh,
               evidence_refs: ["runs/m85/evidence.json"],
               run_ref: "runs/m85"
             })

    assert {:ok, updated} = StudioLiveShell.apply_event(shell, event)
    view = updated.views.evidence

    assert view.status == :fresh
    refute view.empty?
    assert [^event] = view.entries
    assert view.read_only?
    assert view.rust_owned?
    refute view.trusted_write_authority?
    refute updated.directArtifactWrite
    refute updated.commandBridge
  end

  test "raw writes command bridges new stores hosted collab and mandatory-human drift fail closed" do
    assert {:error, :trusted_write_or_scope_drift} =
             StudioLiveShell.new(%{trusted_write_authority: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioLiveShell.new(%{direct_artifact_write: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioLiveShell.new(%{command_bridge: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioLiveShell.new(%{new_data_store: true})

    assert {:error, :trusted_write_or_scope_drift} =
             StudioLiveShell.new(%{hosted_collaborative: true})

    assert {:error, :autonomy_or_cli_fallback_broken} =
             StudioLiveShell.new(%{autonomous_loop_requires_human: true})

    assert {:error, :two_plane_boundary_broken} =
             StudioLiveShell.new(%{elixir_owns_artifact_semantics: true})

    assert {:error, :trusted_write_forbidden} =
             StudioLiveShell.rust_owned_event(:evidence, %{
               trusted_write_authority: true,
               evidence_refs: ["runs/m85/evidence.json"]
             })

    assert {:error, :command_bridge_forbidden} =
             StudioLiveShell.rust_owned_event(:verdict, %{
               command_bridge: true,
               evidence_refs: ["runs/m85/verdict.json"]
             })
  end
end

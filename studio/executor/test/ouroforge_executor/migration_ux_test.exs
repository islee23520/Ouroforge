defmodule OuroforgeExecutor.MigrationUXTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{Contract, MigrationImportSession, MigrationUX}

  defp report_fixture do
    %{
      source_engine: "godot-2d",
      source_project_path: "examples/godot-2d-adapter-v1/sample-project",
      ir_state_hash: "sha256:ir",
      verification_state_hash: "sha256:verification",
      claimed_ported_units: [],
      oracle_rule: "ported_claim_allowed=false until captured oracle evidence passes",
      rows: [
        %{
          id: "scene/root",
          source_ref: "res://main.tscn:Node2D",
          target_ref: "ouroforge://scene/root",
          grade: "green",
          rationale: "scene hierarchy and transform are declarative skeleton",
          evidence_ref: "runs/m94/evidence/scene-root.json",
          oracle_status: "not_required_for_skeleton"
        },
        %{
          id: "tilemap/cave",
          source_ref: "res://main.tscn:TileMap",
          target_ref: "ouroforge://tilemap/cave",
          grade: "yellow",
          rationale: "tile metadata imported with renderer normalization caveat",
          evidence_ref: "runs/m94/evidence/tilemap-cave.json",
          oracle_status: "missing"
        },
        %{
          id: "script/player-controller",
          source_ref: "res://player.gd",
          target_ref: nil,
          grade: "red",
          rationale: "behavior-bearing movement logic must be re-derived clean-room",
          evidence_ref: "runs/m94/evidence/player-controller.json",
          era_r_task_ref: "generated/era-r/player-controller.behavior-draft.json",
          oracle_status: "missing",
          port_claim_allowed: false
        }
      ]
    }
  end

  test "wizard routes source-project imports through Rust migration CLI only" do
    assert {:ok, wizard} =
             MigrationUX.new_wizard(%{
               source_engine: :godot,
               source_project_path: "examples/godot-2d-adapter-v1/sample-project",
               operator_label: "sample Godot import"
             })

    assert wizard.routeCli == [
             "migration",
             "verify-demo",
             "--project",
             "examples/godot-2d-adapter-v1/sample-project",
             "--output",
             "examples/godot-2d-adapter-v1/generated/import-verification-report.json"
           ]

    assert Contract.allowed_cli_family?(wizard.routeCli)
    assert wizard.liveViewPresentation
    assert wizard.rustDataPlaneOwnsTruth
    refute wizard.elixirOwnsArtifactSemantics
    refute wizard.directArtifactWrite
    refute wizard.studioTrustedWriteAuthority
    refute wizard.newDataStore
    refute wizard.liveBridge
    refute wizard.embeddedEngineRuntime
    refute wizard.decompiledSourceAccepted
    refute wizard.finishedGameAutoPort

    assert Enum.any?(
             wizard.rustShapes,
             &(&1.path == "crates/ouroforge-core/src/godot_2d_adapter_ir.rs")
           )
  end

  test "wizard rejects shipped builds and unsupported engines before CLI routing" do
    assert {:error, :source_project_only} =
             MigrationUX.new_wizard(%{
               source_engine: :unity,
               source_project_path: "tmp/Builds/game.exe"
             })

    assert {:error, :unsupported_source_engine} =
             MigrationUX.new_wizard(%{
               source_engine: :unreal,
               source_project_path: "examples/project"
             })
  end

  test "import invocation captures CLI result as control-plane state, not artifact truth" do
    assert {:ok, wizard} =
             MigrationUX.new_wizard(%{
               source_engine: :unity,
               source_project_path: "examples/unity-2d-adapter-v1/sample-project"
             })

    runner = fn executable, argv, opts ->
      assert executable == "ouroforge"
      assert argv == wizard.routeCli
      assert opts[:stderr_to_stdout] == false

      {"Unity migration demo report: examples/unity-2d-adapter-v1/generated/fidelity-report.json",
       0}
    end

    assert {:ok, invoked} = MigrationUX.import(wizard, runner: runner)
    assert invoked.status == :import_invoked
    assert invoked.routeCli == wizard.routeCli
    refute invoked.trustedWriteAuthority
    refute invoked.directArtifactWrite
    assert invoked.rustDataPlaneOwnsTruth
  end

  test "fidelity report view renders honest grades and fix-forward links" do
    assert {:ok, view} = MigrationUX.report_view(report_fixture())
    assert view.summary == %{green: 1, yellow: 1, red: 1}
    assert view.claimedPortedUnits == []
    assert view.rustOwnedEvidence
    refute view.elixirOwnsArtifactSemantics
    refute view.directArtifactWrite

    assert {:ok, text} = MigrationUX.render_report(view)
    assert text =~ "🟢 1 / 🟡 1 / 🔴 1"
    assert text =~ "no port claim without oracle"

    assert {:ok, links} = MigrationUX.fix_forward_links(view)
    assert Enum.map(links, & &1.targetEra) == ["Era M", "Era R"]

    red = Enum.find(links, &(&1.targetEra == "Era R"))

    assert red.routeCli == [
             "behavior",
             "draft",
             "preview",
             "generated/era-r/player-controller.behavior-draft.json"
           ]

    refute red.trustedWriteAuthority
    refute red.directArtifactWrite
    refute red.portClaimAllowed
  end

  test "report blocks port claims, red rows without Era R handoff, and trusted write drift" do
    port_claim = put_in(report_fixture()[:claimed_ported_units], ["script/player-controller"])
    assert {:error, :ported_claim_forbidden} = MigrationUX.report_view(port_claim)

    red_without_task =
      update_in(report_fixture()[:rows], fn rows ->
        Enum.map(rows, fn
          %{id: "script/player-controller"} = row -> Map.delete(row, :era_r_task_ref)
          row -> row
        end)
      end)

    assert {:error, :red_without_era_r_task} = MigrationUX.report_view(red_without_task)

    trusted_row =
      update_in(report_fixture()[:rows], fn rows ->
        Enum.map(rows, fn
          %{id: "scene/root"} = row -> Map.put(row, :port_claim_allowed, true)
          row -> row
        end)
      end)

    assert {:error, :invalid_fidelity_row} = MigrationUX.report_view(trusted_row)
  end

  test "OTP session stores only ephemeral wizard/report state and broadcasts presentation events" do
    assert {:ok, _} = OuroforgeExecutor.LocalPubSub.subscribe(MigrationUX.pubsub_topic())

    {:ok, pid} = start_supervised({MigrationImportSession, name: nil, session_id: "m94-test"})

    assert {:ok, wizard} =
             MigrationImportSession.configure(pid, %{
               source_engine: :godot,
               source_project_path: "examples/godot-2d-adapter-v1/sample-project"
             })

    assert_receive {:ouroforge_executor_pubsub, "studio:migration-ux", %{event: :configured}}

    runner = fn _exe, _argv, _opts -> {"ok", 0} end
    assert {:ok, _invoked} = MigrationImportSession.import(pid, runner: runner)
    assert_receive {:ouroforge_executor_pubsub, "studio:migration-ux", %{event: :import_invoked}}

    assert {:ok, view, links} = MigrationImportSession.attach_report(pid, report_fixture())
    assert_receive {:ouroforge_executor_pubsub, "studio:migration-ux", %{event: :report_ready}}

    state = MigrationImportSession.state(pid)
    assert state.status == :report_ready
    assert state.wizard == wizard
    assert state.report == view
    assert state.fixForwardLinks == links
    assert Enum.all?(links, &(&1.trustedWriteAuthority == false))
  end

  test "fix-forward route is gated through allowed preview CLI and cannot self-authorize" do
    assert {:ok, view} = MigrationUX.report_view(report_fixture())
    assert {:ok, links} = MigrationUX.fix_forward_links(view)
    red = Enum.find(links, &(&1.targetEra == "Era R"))

    runner = fn executable, argv, _opts ->
      assert executable == "ouroforge"
      assert argv == red.routeCli
      {"preview ok", 0}
    end

    assert {:ok, result} = MigrationUX.route_fix_forward(red, runner: runner)
    assert result.argv == red.routeCli
  end
end

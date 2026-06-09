defmodule OuroforgeExecutor.MigrationUXDemo do
  @moduledoc """
  Scripted Studio Migration UX demo for Era O/M94 (#2188).

  The demo composes existing Rust-owned migration reports with the Studio
  control/presentation model from #2187. It demonstrates import invocation,
  report rendering, and fix-forward routing without claiming any behavior is
  ported and without Elixir trusted writes.
  """

  alias OuroforgeExecutor.MigrationUX

  defstruct version: "migration-ux-studio-demo-v1",
            boundary:
              "one-way source-project on-ramp; source-only; clean-room re-derivation; Rust-owned fidelity/oracle evidence; Studio read + gated-write only; no trusted Elixir writes; no auto-port claim",
            godot: nil,
            unity: nil,
            report: nil,
            fixForwardLinks: [],
            scriptedCommands: [],
            evidenceRefs: [],
            deterministicHashes: [],
            claimedPortedUnits: [],
            trustedWriteAuthority: false,
            directArtifactWrite: false,
            noAutoPortClaim: true,
            oracleGated: true,
            cleanRoom: true

  def run(opts \\ []) do
    runner = Keyword.get(opts, :runner, &demo_runner/3)

    {:ok, godot} =
      MigrationUX.new_wizard(%{
        source_engine: :godot,
        source_project_path: "examples/godot-2d-adapter-v1/sample-project",
        output_ref: "examples/godot-2d-adapter-v1/generated/import-verification-report.json"
      })

    {:ok, unity} =
      MigrationUX.new_wizard(%{
        source_engine: :unity,
        source_project_path: "examples/unity-2d-adapter-v1/sample-project",
        output_ref: "examples/unity-2d-adapter-v1/generated/fidelity-report.json"
      })

    {:ok, godot_invocation} = MigrationUX.import(godot, runner: runner)
    {:ok, unity_invocation} = MigrationUX.import(unity, runner: runner)
    {:ok, report} = MigrationUX.report_view(report_fixture())
    {:ok, links} = MigrationUX.fix_forward_links(report)

    %__MODULE__{
      godot: godot_invocation,
      unity: unity_invocation,
      report: report,
      fixForwardLinks: links,
      scriptedCommands: [
        godot.routeCli,
        unity.routeCli,
        ["run", "seeds/migration-demo.yaml", "--workers", "2"]
      ],
      evidenceRefs: Enum.map(report.rows, & &1.evidenceRef),
      deterministicHashes:
        Enum.reject(
          [report.irStateHash, report.mappingStateHash, report.verificationStateHash],
          &blank?/1
        ),
      claimedPortedUnits: report.claimedPortedUnits
    }
  end

  def render(%__MODULE__{} = demo) do
    [
      "Migration UX Studio demo #{demo.version}",
      "Boundary: #{demo.boundary}",
      "Fidelity: 🟢 #{demo.report.summary.green} / 🟡 #{demo.report.summary.yellow} / 🔴 #{demo.report.summary.red}",
      "Determinism: #{Enum.join(demo.deterministicHashes, ", ")}",
      "Claimed ported units: #{length(demo.claimedPortedUnits)}",
      "Fix-forward routes: #{Enum.map(demo.fixForwardLinks, & &1.targetEra) |> Enum.join(", ")}",
      "Evidence refs: #{Enum.join(demo.evidenceRefs, ", ")}",
      "No auto-port claim: #{demo.noAutoPortClaim and demo.claimedPortedUnits == []}",
      "Trusted writes by Studio: #{demo.trustedWriteAuthority or demo.directArtifactWrite}"
    ]
    |> Enum.join("\n")
  end

  def validate(%__MODULE__{} = demo) do
    cond do
      demo.claimedPortedUnits != [] ->
        {:error, :ported_claim_forbidden}

      not demo.noAutoPortClaim or not demo.oracleGated or not demo.cleanRoom ->
        {:error, :migration_boundary_broken}

      demo.trustedWriteAuthority or demo.directArtifactWrite ->
        {:error, :trusted_write_forbidden}

      demo.deterministicHashes == [] ->
        {:error, :missing_deterministic_hash}

      not Enum.any?(
        demo.fixForwardLinks,
        &(&1.targetEra == "Era R" and &1.portClaimAllowed == false)
      ) ->
        {:error, :missing_era_r_fix_forward}

      true ->
        :ok
    end
  end

  def to_summary(%__MODULE__{} = demo) do
    %{
      "version" => demo.version,
      "boundary" => demo.boundary,
      "scriptedCommands" => demo.scriptedCommands,
      "evidenceRefs" => demo.evidenceRefs,
      "deterministicHashes" => demo.deterministicHashes,
      "fidelitySummary" => %{
        "green" => demo.report.summary.green,
        "yellow" => demo.report.summary.yellow,
        "red" => demo.report.summary.red
      },
      "claimedPortedUnits" => demo.claimedPortedUnits,
      "noAutoPortClaim" => demo.noAutoPortClaim,
      "oracleGated" => demo.oracleGated,
      "cleanRoom" => demo.cleanRoom,
      "studioTrustedWriteAuthority" => demo.trustedWriteAuthority,
      "directArtifactWrite" => demo.directArtifactWrite,
      "fixForwardTargets" => Enum.map(demo.fixForwardLinks, & &1.targetEra)
    }
  end

  def report_fixture do
    %{
      source_engine: "godot-2d",
      source_project_path: "examples/godot-2d-adapter-v1/sample-project",
      ir_state_hash: "sha256:m94-studio-demo-ir",
      mapping_state_hash: "sha256:m94-studio-demo-mapping",
      verification_state_hash: "sha256:m94-studio-demo-verification",
      claimed_ported_units: [],
      oracle_rule: "ported_claim_allowed=false until captured oracle evidence passes",
      rows: [
        %{
          id: "demo.scene.skeleton",
          source_ref: "examples/godot-2d-adapter-v1/sample-project/scenes/main.tscn",
          target_ref: "ouroforge://demo/scene/skeleton",
          grade: "green",
          rationale: "declarative scene hierarchy imports as skeleton evidence",
          evidence_ref:
            "examples/godot-2d-adapter-v1/import-verification-demo/demo-summary.fixture.json",
          oracle_status: "not_required_for_skeleton"
        },
        %{
          id: "demo.tilemap.presentation",
          source_ref: "examples/godot-2d-adapter-v1/sample-project/scenes/main.tscn#TileMap",
          target_ref: "ouroforge://demo/tilemap/presentation",
          grade: "yellow",
          rationale: "tile presentation imports best-effort with fidelity caveat",
          evidence_ref: "examples/godot-2d-adapter-v1/mapping-fidelity-report.sample.json",
          oracle_status: "missing"
        },
        %{
          id: "demo.player.logic",
          source_ref: "examples/godot-2d-adapter-v1/sample-project/scenes/player.gd",
          target_ref: nil,
          grade: "red",
          rationale: "behavior is an Era R clean-room re-derivation task, not translated source",
          evidence_ref: "examples/godot-2d-adapter-v1/scenario-coverage-v80/matrix.fixture.json",
          era_r_task_ref: "generated/era-r/demo-player-logic.behavior-draft.json",
          oracle_status: "missing",
          port_claim_allowed: false
        }
      ]
    }
  end

  defp demo_runner(_executable, argv, _opts) do
    {"demo invoked #{Enum.join(argv, " ")}", 0}
  end

  defp blank?(value), do: value in [nil, "", []]
end

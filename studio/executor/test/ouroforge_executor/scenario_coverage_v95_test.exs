defmodule OuroforgeExecutor.ScenarioCoverageV95Test do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.{ReDerivationUX, ReDerivationUXDemo}

  defp repo_root do
    Path.expand("../../../..", __DIR__)
  end

  defp read_text(path) do
    repo_root() |> Path.join(path) |> File.read!()
  end

  defp read_json(path) do
    path |> read_text() |> OuroforgeExecutor.JSON.decode!()
  end

  defp fixture do
    %{
      project_id: "scenario-coverage-v95",
      source_project_ref: "source-projects/m113/sample-project/project.gltf",
      target_dimensionality: :three_d,
      claimed_ported_units: [],
      units: [
        %{
          unit_id: "unit.v95-door",
          behavioral_unit_ref: "ir/m108/unit.v95-door.json",
          source_ref: "source-projects/m113/sample-project/door.tscn",
          oracle_ref: "oracles/m109/unit.v95-door.json",
          ab_evidence_ref: "evidence/m111/unit.v95-door.json",
          coverage_ref: "coverage/m112/unit.v95-door.json",
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
          unit_id: "unit.v95-feel",
          behavioral_unit_ref: "ir/m108/unit.v95-feel.json",
          source_ref: "source-projects/m113/sample-project/player.tscn",
          primary_state_hash: "fnv64:bbbbbbbbbbbbbbbb",
          secondary_render_digest: "ssim:0.982;pixel-diff:0.010",
          grade: :yellow,
          oracle_status: :captured,
          ab_status: :needs_repair,
          coverage_status: :human_escalated,
          pipeline_stage: :verify,
          intent_feel_escalation: true,
          question_prompt: "Should the jump feel floaty?",
          gap_summary: ["lossy feel mismatch remains"],
          re_derivation_tasks: ["capture Ring 2 intent before retuning"]
        },
        %{
          unit_id: "unit.v95-vfx",
          behavioral_unit_ref: "ir/m108/unit.v95-vfx.json",
          source_ref: "source-projects/m113/sample-project/vfx.tres",
          grade: :red,
          oracle_status: :missing,
          ab_status: :not_run,
          coverage_status: :blocked,
          pipeline_stage: :interrogate,
          gap_summary: ["unsupported shader behavior must be re-derived"],
          re_derivation_tasks: ["reject or defer unsafe/lossy shader translation"]
        }
      ]
    }
  end

  test "v95 matrix records rows and boundaries" do
    matrix =
      read_json("examples/rederivation-ux-demo-v1/scenario-coverage-v95/matrix.fixture.json")

    assert matrix["schemaVersion"] == "scenario-coverage-v95-rederivation-ux-v1"
    assert matrix["coverageVersion"] == 95
    assert matrix["issueRef"] == "#2240"

    rows = matrix["rows"]

    for required <- [
          "v95.demo-renders-honest-fidelity-summary",
          "v95.no-auto-port-without-oracle-fails",
          "v95.lossy-import-not-graded-clean",
          "v95.ungated-auto-translated-port-fails",
          "v95.deterministic-state-hash-break-fails",
          "v95.human-intent-feel-routes-through-gated-cli",
          "v95.clean-room-source-only-and-no-studio-trusted-write"
        ] do
      assert Enum.any?(rows, &(&1["id"] == required)), "missing #{required}"
    end

    assert Enum.all?(rows, &(&1["status"] == "pass"))
    assert matrix["invariants"]["autoPortWithoutOracleAllowed"] == false
    assert matrix["invariants"]["lossyImportMayGradeClean"] == false
    assert matrix["invariants"]["ungatedAutoTranslatedPortAllowed"] == false
    assert matrix["invariants"]["deterministicStateHashRequired"] == true
    assert matrix["invariants"]["studioTrustedWriteAuthority"] == false
    assert matrix["invariants"]["humanFeelIsRing2"] == true
  end

  test "demo renders honest fidelity summary without auto-port claim" do
    demo = ReDerivationUXDemo.run()
    assert :ok = ReDerivationUXDemo.validate(demo)
    assert demo.fidelitySummary == %{green: 1, yellow: 1, red: 1, escalated: 2}
    assert demo.claimedPortedUnits == []
    assert demo.noAutoPortClaim
    assert demo.oracleGated
    refute demo.trustedWriteAuthority
  end

  test "no auto-port without oracle and lossy imports are not clean" do
    missing_oracle =
      update_in(fixture()[:units], fn [green | rest] ->
        [Map.delete(green, :oracle_ref) | rest]
      end)

    assert {:error, :invalid_unit_card} = ReDerivationUX.surface(missing_oracle)

    assert {:ok, surface} = ReDerivationUX.surface(fixture())
    lossy = Enum.find(surface.units, &(&1.unitId == "unit.v95-feel"))
    assert lossy.grade == :yellow
    assert lossy.gapSummary != []
    assert lossy.reDerivationTasks != []
    refute lossy.portClaimAllowed
  end

  test "ungated auto-translated port and unsafe refs fail closed" do
    trusted = Map.put(fixture(), :direct_artifact_write, true)
    assert {:error, :trusted_write_forbidden} = ReDerivationUX.surface(trusted)

    unsafe =
      update_in(fixture()[:units], fn [green | rest] ->
        [Map.put(green, :source_ref, "decompiled/Assembly-CSharp/Door.cs") | rest]
      end)

    assert {:error, :invalid_unit_card} = ReDerivationUX.surface(unsafe)

    bridge = Map.put(fixture(), :live_bridge, true)
    assert {:error, :rederivation_boundary_broken} = ReDerivationUX.surface(bridge)
  end

  test "deterministic state-hash break fails and 3d render remains secondary" do
    bad_hash =
      update_in(fixture()[:units], fn [green | rest] ->
        [Map.put(green, :primary_state_hash, "sha256:not-state-hash") | rest]
      end)

    assert {:error, :determinism_evidence_required} = ReDerivationUX.surface(bad_hash)

    missing_render =
      update_in(fixture()[:units], fn [green | rest] ->
        [Map.delete(green, :secondary_render_digest) | rest]
      end)

    assert {:error, :determinism_evidence_required} = ReDerivationUX.surface(missing_render)
  end

  test "human intent feel routes through gated cli and no Studio trusted write" do
    assert {:ok, surface} = ReDerivationUX.surface(fixture())
    assert {:ok, escalations} = ReDerivationUX.escalation_queue(surface)
    escalation = Enum.find(escalations, &(&1.unitId == "unit.v95-feel"))

    refute escalation.trustedWriteAuthority
    refute escalation.directArtifactWrite
    refute escalation.portClaimAllowed
    assert escalation.targetRing == "Ring 2"
    assert Enum.take(escalation.routeCli, 3) == ["behavior", "draft", "preview"]

    runner = fn executable, argv, _opts ->
      assert executable == "ouroforge"
      assert argv == escalation.routeCli ++ ["--human-note-preview"]
      {"queued", 0}
    end

    assert {:ok, result} =
             ReDerivationUX.submit_intent_feel(escalation, "Keep it floaty", runner: runner)

    assert result.status == 0
  end

  test "v95 docs preserve clean-room and two-plane anchors" do
    doc = read_text("docs/scenario-coverage-v95-rederivation-ux.md") |> String.downcase()

    for required <- [
          "scenario coverage v95",
          "one-way on-ramp",
          "source-project/open-text",
          "no auto-port without oracle",
          "yellow/red",
          "state-hash primary",
          "studio is elixir/phoenix control + presentation only",
          "rust remains the data plane",
          "#1 and #23 remain open"
        ] do
      assert doc =~ required, "missing #{required}"
    end
  end
end

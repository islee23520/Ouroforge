defmodule OuroforgeExecutor.ReDerivationUXDemo do
  @moduledoc """
  Scripted M113 re-derivation UX demo.

  The demo loads a small fixture-shaped source-project skeleton, renders the
  Rust-owned re-derivation evidence through `ReDerivationUX`, and proves the
  boundary: content import is best-effort, logic is re-derived/verified through
  oracle-gated evidence, intent/feel is routed to humans, and no unit is claimed
  ported without an oracle.
  """

  alias OuroforgeExecutor.ReDerivationUX

  @version "rederivation-ux-demo-v1"

  defstruct version: @version,
            fixtureRef: "examples/rederivation-ux-demo-v1/fidelity-report.fixture.json",
            surface: nil,
            escalations: [],
            rendered: nil,
            scriptedCommands: [],
            fidelitySummary: %{},
            claimedPortedUnits: [],
            noAutoPortClaim: true,
            oracleGated: true,
            deterministicStateHashPrimary: true,
            perceptualRenderSecondaryOnly: true,
            trustedWriteAuthority: false,
            directArtifactWrite: false,
            cleanRoom: true

  def run(attrs \\ %{}) do
    evidence = Map.merge(fixture(), attrs)
    {:ok, surface} = ReDerivationUX.surface(evidence)
    {:ok, escalations} = ReDerivationUX.escalation_queue(surface)
    {:ok, rendered} = ReDerivationUX.render(surface)

    %__MODULE__{
      surface: surface,
      escalations: escalations,
      rendered: rendered,
      scriptedCommands: scripted_commands(escalations),
      fidelitySummary: surface.summary,
      claimedPortedUnits: surface.claimedPortedUnits,
      noAutoPortClaim:
        surface.claimedPortedUnits == [] and
          Enum.all?(surface.units, &(&1.portClaimAllowed == false)),
      oracleGated:
        Enum.all?(
          Enum.filter(surface.units, &(&1.grade == :green)),
          &(&1.oracleStatus == :captured)
        ),
      deterministicStateHashPrimary: surface.summary.green > 0,
      perceptualRenderSecondaryOnly:
        surface.targetDimensionality in [:two_point_five_d, :three_d],
      trustedWriteAuthority: surface.studioTrustedWriteAuthority,
      directArtifactWrite: surface.directArtifactWrite,
      cleanRoom:
        not surface.decompiledSourceAccepted and not surface.liveBridge and
          not surface.embeddedEngineRuntime
    }
  end

  def validate(%__MODULE__{} = demo) do
    cond do
      demo.claimedPortedUnits != [] or not demo.noAutoPortClaim ->
        {:error, :ported_claim_forbidden}

      not demo.oracleGated ->
        {:error, :oracle_gate_missing}

      not demo.deterministicStateHashPrimary ->
        {:error, :determinism_missing}

      demo.trustedWriteAuthority or demo.directArtifactWrite ->
        {:error, :trusted_write_forbidden}

      not demo.cleanRoom ->
        {:error, :clean_room_boundary_broken}

      true ->
        :ok
    end
  end

  def to_summary(%__MODULE__{} = demo) do
    %{
      "version" => demo.version,
      "fixtureRef" => demo.fixtureRef,
      "fidelitySummary" => %{
        "green" => demo.fidelitySummary.green,
        "yellow" => demo.fidelitySummary.yellow,
        "red" => demo.fidelitySummary.red,
        "escalated" => demo.fidelitySummary.escalated
      },
      "claimedPortedUnits" => demo.claimedPortedUnits,
      "noAutoPortClaim" => demo.noAutoPortClaim,
      "oracleGated" => demo.oracleGated,
      "deterministicStateHashPrimary" => demo.deterministicStateHashPrimary,
      "perceptualRenderSecondaryOnly" => demo.perceptualRenderSecondaryOnly,
      "trustedWriteAuthority" => demo.trustedWriteAuthority,
      "directArtifactWrite" => demo.directArtifactWrite,
      "cleanRoom" => demo.cleanRoom,
      "scriptedCommands" => demo.scriptedCommands,
      "escalationTargets" => Enum.map(demo.escalations, & &1.unitId)
    }
  end

  def render(%__MODULE__{} = demo) do
    [
      "Re-derivation UX demo: #{demo.version}",
      demo.rendered,
      "Claimed ported units: #{length(demo.claimedPortedUnits)}",
      "No auto-port claim: #{demo.noAutoPortClaim}",
      "Oracle gated: #{demo.oracleGated}",
      "State-hash primary: #{demo.deterministicStateHashPrimary}",
      "Render secondary only: #{demo.perceptualRenderSecondaryOnly}",
      "Trusted writes by Studio: #{demo.trustedWriteAuthority}"
    ]
    |> Enum.join("\n")
  end

  defp fixture do
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

  defp scripted_commands(escalations) do
    [
      ["run", "seeds/migration-demo.yaml", "--workers", "2"],
      [
        "migration",
        "verify-demo",
        "--project",
        "source-projects/m113/sample-project/project.gltf"
      ],
      ["behavior", "draft", "preview", "generated/era-r/unit.jump-feel.intent-feel-review.json"]
    ] ++ Enum.map(escalations, & &1.routeCli)
  end
end

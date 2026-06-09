defmodule OuroforgeExecutor.ReDerivationUX do
  @moduledoc """
  Studio-facing re-derivation UX read model for Era R/M113 (#2238).

  This module is deliberately control/presentation plane only. It prepares the
  data that a Phoenix LiveView renders for interrogation, behavioral-unit
  decomposition, oracle capture, deterministic re-expression, differential A/B,
  semantic-port coverage, and Ring 2 intent/feel escalation. It never parses
  source projects, grades fidelity, mutates artifacts, or owns artifact
  semantics; Rust remains the data plane and every write-affecting human note is
  routed through existing `ouroforge` CLI/review gates.
  """

  alias OuroforgeExecutor.{CLI, Contract}

  @schema "ouroforge.rederivation-ux.v1"
  @boundary "Era R re-derivation UX; one-way on-ramp; source-project/open-text clean-room inputs only; interrogation decomposition oracle capture deterministic re-expression differential verification semantic-port coverage rendered from Rust-owned evidence; Phoenix LiveView control + presentation only; read + gated-write; no trusted Elixir writes; no artifact semantics; no new data store; no live bridge; no embedded engine runtime; no decompiled source; no finished-game auto-port; intent feel fun release remain Ring 2 human escalation; #1 and #23 remain open"
  @pubsub_topic "studio:rederivation-ux"

  @rust_shapes [
    %{
      name: "BehavioralUnitRecord",
      path: "crates/ouroforge-core/src/legacy_logic_ingestion.rs",
      role: "M108 clean-room behavioral-unit extraction and Era R hand-off state"
    },
    %{
      name: "OracleSpec",
      path: "crates/ouroforge-core/src/tacit_oracle_capture.rs",
      role: "M109 captured oracle evidence and no-port-without-oracle rule"
    },
    %{
      name: "ReExpressionPlan",
      path: "crates/ouroforge-core/src/deterministic_reexpression.rs",
      role: "M110 deterministic native re-expression target and gated behavior draft"
    },
    %{
      name: "DifferentialVerificationReport",
      path: "crates/ouroforge-core/src/differential_verification.rs",
      role: "M111 behavioral A/B status, fidelity report, rollback evidence, and state hash"
    },
    %{
      name: "SemanticPortCoverageReport",
      path: "crates/ouroforge-core/src/semantic_port_coverage.rs",
      role: "M112 coverage/convergence ledger, residual backlog, and Ring 2 escalation"
    }
  ]

  defmodule UnitCard do
    @moduledoc false
    defstruct [
      :unitId,
      :behavioralUnitRef,
      :oracleRef,
      :reexpressionRef,
      :abEvidenceRef,
      :coverageRef,
      :sourceRef,
      :primaryStateHash,
      :secondaryRenderDigest,
      :questionPrompt,
      :humanAnswerRef,
      grade: :yellow,
      oracleStatus: :missing,
      abStatus: :not_run,
      coverageStatus: :pending,
      pipelineStage: :interrogate,
      reDerivationTasks: [],
      gapSummary: [],
      portClaimAllowed: false,
      fullyPortedClaimAllowed: false,
      intentFeelEscalation: false,
      releaseGoNoGo: false
    ]
  end

  defmodule Surface do
    @moduledoc false
    @schema "ouroforge.rederivation-ux.v1"
    @boundary "Era R re-derivation UX; one-way on-ramp; source-project/open-text clean-room inputs only; interrogation decomposition oracle capture deterministic re-expression differential verification semantic-port coverage rendered from Rust-owned evidence; Phoenix LiveView control + presentation only; read + gated-write; no trusted Elixir writes; no artifact semantics; no new data store; no live bridge; no embedded engine runtime; no decompiled source; no finished-game auto-port; intent feel fun release remain Ring 2 human escalation; #1 and #23 remain open"
    @pubsub_topic "studio:rederivation-ux"
    defstruct schemaVersion: @schema,
              projectId: nil,
              sourceProjectRef: nil,
              targetDimensionality: :two_d,
              units: [],
              summary: %{green: 0, yellow: 0, red: 0, escalated: 0},
              rustShapes: [],
              boundary: @boundary,
              pubsubTopic: @pubsub_topic,
              liveViewPresentation: true,
              phoenixPubSubTopic: @pubsub_topic,
              readGatedWrite: true,
              rustDataPlaneOwnsTruth: true,
              elixirOwnsArtifactSemantics: false,
              directArtifactWrite: false,
              studioTrustedWriteAuthority: false,
              newDataStore: false,
              liveBridge: false,
              embeddedEngineRuntime: false,
              decompiledSourceAccepted: false,
              finishedGameAutoPort: false,
              claimedPortedUnits: []
  end

  defmodule Escalation do
    @moduledoc false
    defstruct [
      :unitId,
      :reason,
      :questionPrompt,
      :reviewDraftRef,
      :routeCli,
      :sourceRef,
      targetRing: "Ring 2",
      humanOwned: true,
      readGatedWrite: true,
      trustedWriteAuthority: false,
      directArtifactWrite: false,
      portClaimAllowed: false,
      releaseGoNoGo: false
    ]
  end

  def schema, do: @schema
  def boundary, do: @boundary
  def pubsub_topic, do: @pubsub_topic
  def rust_shapes, do: @rust_shapes

  def surface(evidence) when is_map(evidence) do
    units = evidence |> list_value(:units, :unitCards) |> Enum.map(&unit_from_map/1)

    surface = %Surface{
      projectId:
        value(evidence, :project_id, value(evidence, :projectId, "rederivation-project")),
      sourceProjectRef: value(evidence, :source_project_ref, value(evidence, :sourceProjectRef)),
      targetDimensionality:
        normalize_dimensionality(
          value(evidence, :target_dimensionality, value(evidence, :targetDimensionality, :two_d))
        ),
      units: units,
      summary: summarize(units),
      rustShapes: @rust_shapes,
      claimedPortedUnits: list_value(evidence, :claimed_ported_units, :claimedPortedUnits),
      liveViewPresentation:
        bool_value(
          evidence,
          :live_view_presentation,
          bool_value(evidence, :liveViewPresentation, true)
        ),
      readGatedWrite:
        bool_value(evidence, :read_gated_write, bool_value(evidence, :readGatedWrite, true)),
      rustDataPlaneOwnsTruth:
        bool_value(
          evidence,
          :rust_data_plane_owns_truth,
          bool_value(evidence, :rustDataPlaneOwnsTruth, true)
        ),
      elixirOwnsArtifactSemantics:
        bool_value(
          evidence,
          :elixir_owns_artifact_semantics,
          bool_value(evidence, :elixirOwnsArtifactSemantics, false)
        ),
      directArtifactWrite:
        bool_value(
          evidence,
          :direct_artifact_write,
          bool_value(evidence, :directArtifactWrite, false)
        ),
      studioTrustedWriteAuthority:
        bool_value(
          evidence,
          :studio_trusted_write_authority,
          bool_value(evidence, :studioTrustedWriteAuthority, false)
        ),
      newDataStore:
        bool_value(evidence, :new_data_store, bool_value(evidence, :newDataStore, false)),
      liveBridge: bool_value(evidence, :live_bridge, bool_value(evidence, :liveBridge, false)),
      embeddedEngineRuntime:
        bool_value(
          evidence,
          :embedded_engine_runtime,
          bool_value(evidence, :embeddedEngineRuntime, false)
        ),
      decompiledSourceAccepted:
        bool_value(
          evidence,
          :decompiled_source_accepted,
          bool_value(evidence, :decompiledSourceAccepted, false)
        ),
      finishedGameAutoPort:
        bool_value(
          evidence,
          :finished_game_auto_port,
          bool_value(evidence, :finishedGameAutoPort, false)
        )
    }

    with :ok <- validate_surface(surface) do
      {:ok, surface}
    end
  end

  def validate_surface(%Surface{} = surface) do
    cond do
      surface.schemaVersion != @schema ->
        {:error, :unsupported_schema}

      blank?(surface.projectId) or blank?(surface.sourceProjectRef) or surface.units == [] ->
        {:error, :missing_rederivation_evidence}

      unsafe_ref?(surface.sourceProjectRef) or not source_project_ref?(surface.sourceProjectRef) ->
        {:error, :source_project_only}

      not surface.liveViewPresentation or not surface.readGatedWrite ->
        {:error, :not_liveview_read_gated_write}

      not surface.rustDataPlaneOwnsTruth or surface.elixirOwnsArtifactSemantics ->
        {:error, :two_plane_boundary_broken}

      surface.directArtifactWrite or surface.studioTrustedWriteAuthority or surface.newDataStore ->
        {:error, :trusted_write_forbidden}

      surface.liveBridge or surface.embeddedEngineRuntime or surface.decompiledSourceAccepted or
          surface.finishedGameAutoPort ->
        {:error, :rederivation_boundary_broken}

      surface.claimedPortedUnits != [] or Enum.any?(surface.units, &ported_claim?/1) ->
        {:error, :ported_claim_forbidden}

      Enum.any?(surface.units, &invalid_unit?/1) ->
        {:error, :invalid_unit_card}

      Enum.any?(surface.units, &determinism_broken?(surface.targetDimensionality, &1)) ->
        {:error, :determinism_evidence_required}

      true ->
        :ok
    end
  end

  def escalation_queue(%Surface{} = surface) do
    with :ok <- validate_surface(surface) do
      {:ok,
       surface.units
       |> Enum.filter(&escalation_required?/1)
       |> Enum.map(&escalation_for_unit/1)}
    end
  end

  def submit_intent_feel(%Escalation{} = escalation, note, opts \\ []) when is_binary(note) do
    cond do
      blank?(note) ->
        {:error, :empty_human_note}

      escalation.trustedWriteAuthority or escalation.directArtifactWrite or
        escalation.portClaimAllowed or escalation.releaseGoNoGo ->
        {:error, :trusted_write_forbidden}

      not Contract.allowed_cli_family?(escalation.routeCli) ->
        {:error, {:outside_frozen_cli_surface, escalation.routeCli}}

      true ->
        CLI.run(escalation.routeCli ++ ["--human-note-preview"], opts)
    end
  end

  def render(%Surface{} = surface) do
    with :ok <- validate_surface(surface),
         {:ok, escalations} <- escalation_queue(surface) do
      lines = [
        "Re-derivation UX: #{surface.projectId}",
        "Source: #{surface.sourceProjectRef}",
        "Fidelity: 🟢 #{surface.summary.green} / 🟡 #{surface.summary.yellow} / 🔴 #{surface.summary.red}",
        "Intent/feel escalations: #{length(escalations)}",
        "Boundary: one-way source-project on-ramp; clean-room re-derivation; no auto-port without oracle; two-plane read + gated-write",
        "Trusted writes by Studio: #{surface.studioTrustedWriteAuthority}"
      ]

      {:ok, Enum.join(lines, "\n")}
    end
  end

  defp unit_from_map(%UnitCard{} = unit), do: unit

  defp unit_from_map(unit) when is_map(unit) do
    grade = normalize_grade(value(unit, :grade, :yellow))

    %UnitCard{
      unitId:
        value(unit, :unit_id, value(unit, :unitId, "unit-#{System.unique_integer([:positive])}")),
      behavioralUnitRef: value(unit, :behavioral_unit_ref, value(unit, :behavioralUnitRef)),
      oracleRef: value(unit, :oracle_ref, value(unit, :oracleRef)),
      reexpressionRef: value(unit, :reexpression_ref, value(unit, :reexpressionRef)),
      abEvidenceRef: value(unit, :ab_evidence_ref, value(unit, :abEvidenceRef)),
      coverageRef: value(unit, :coverage_ref, value(unit, :coverageRef)),
      sourceRef: value(unit, :source_ref, value(unit, :sourceRef)),
      primaryStateHash: value(unit, :primary_state_hash, value(unit, :primaryStateHash)),
      secondaryRenderDigest:
        value(unit, :secondary_render_digest, value(unit, :secondaryRenderDigest)),
      questionPrompt: value(unit, :question_prompt, value(unit, :questionPrompt)),
      humanAnswerRef: value(unit, :human_answer_ref, value(unit, :humanAnswerRef)),
      grade: grade,
      oracleStatus:
        normalize_status(value(unit, :oracle_status, value(unit, :oracleStatus, :missing))),
      abStatus: normalize_status(value(unit, :ab_status, value(unit, :abStatus, :not_run))),
      coverageStatus:
        normalize_status(value(unit, :coverage_status, value(unit, :coverageStatus, :pending))),
      pipelineStage:
        normalize_stage(value(unit, :pipeline_stage, value(unit, :pipelineStage, :interrogate))),
      reDerivationTasks: list_value(unit, :re_derivation_tasks, :reDerivationTasks),
      gapSummary: list_value(unit, :gap_summary, :gapSummary),
      portClaimAllowed:
        bool_value(unit, :port_claim_allowed, bool_value(unit, :portClaimAllowed, false)),
      fullyPortedClaimAllowed:
        bool_value(
          unit,
          :fully_ported_claim_allowed,
          bool_value(unit, :fullyPortedClaimAllowed, false)
        ),
      intentFeelEscalation:
        bool_value(unit, :intent_feel_escalation, bool_value(unit, :intentFeelEscalation, false)),
      releaseGoNoGo: bool_value(unit, :release_go_no_go, bool_value(unit, :releaseGoNoGo, false))
    }
  end

  defp invalid_unit?(%UnitCard{} = unit) do
    blank?(unit.unitId) or blank?(unit.behavioralUnitRef) or blank?(unit.sourceRef) or
      unit.grade not in [:green, :yellow, :red] or unsafe_ref?(unit.behavioralUnitRef) or
      unsafe_ref?(unit.sourceRef) or green_without_oracle?(unit) or red_without_task?(unit)
  end

  defp green_without_oracle?(%UnitCard{} = unit) do
    unit.grade == :green and
      (unit.oracleStatus != :captured or blank?(unit.oracleRef) or unit.abStatus != :passed or
         unit.coverageStatus != :verified or unit.gapSummary != [])
  end

  defp red_without_task?(%UnitCard{} = unit),
    do: unit.grade == :red and unit.reDerivationTasks == []

  defp determinism_broken?(dimensionality, %UnitCard{} = unit) do
    unit.grade == :green and
      (not state_hash?(unit.primaryStateHash) or
         (dimensionality in [:two_point_five_d, :three_d] and blank?(unit.secondaryRenderDigest)))
  end

  defp ported_claim?(%UnitCard{} = unit),
    do: unit.portClaimAllowed or unit.fullyPortedClaimAllowed

  defp escalation_required?(%UnitCard{} = unit) do
    unit.intentFeelEscalation or unit.grade == :red or unit.releaseGoNoGo or
      unit.pipelineStage in [:interrogate, :oracle_capture]
  end

  defp escalation_for_unit(%UnitCard{} = unit) do
    %Escalation{
      unitId: unit.unitId,
      reason: escalation_reason(unit),
      questionPrompt: unit.questionPrompt || "Clarify human intent/feel for #{unit.unitId}",
      reviewDraftRef: review_ref(unit),
      routeCli: ["behavior", "draft", "preview", review_ref(unit)],
      sourceRef: unit.sourceRef,
      releaseGoNoGo: unit.releaseGoNoGo
    }
  end

  defp escalation_reason(%UnitCard{releaseGoNoGo: true}), do: "release go/no-go remains human"
  defp escalation_reason(%UnitCard{intentFeelEscalation: true}), do: "Ring 2 intent/feel review"
  defp escalation_reason(%UnitCard{grade: :red}), do: "blocked re-derivation task"
  defp escalation_reason(%UnitCard{pipelineStage: stage}), do: "human clarification for #{stage}"

  defp review_ref(%UnitCard{} = unit) do
    unit.humanAnswerRef || "generated/era-r/#{unit.unitId}.intent-feel-review.json"
  end

  defp summarize(units) do
    %{
      green: Enum.count(units, &(&1.grade == :green)),
      yellow: Enum.count(units, &(&1.grade == :yellow)),
      red: Enum.count(units, &(&1.grade == :red)),
      escalated: Enum.count(units, &escalation_required?/1)
    }
  end

  defp normalize_dimensionality(value) when value in [:two_d, "two_d", "2d"], do: :two_d

  defp normalize_dimensionality(value)
       when value in [:two_point_five_d, "two_point_five_d", "2.5d"],
       do: :two_point_five_d

  defp normalize_dimensionality(value) when value in [:three_d, "three_d", "3d"], do: :three_d
  defp normalize_dimensionality(value), do: value

  defp normalize_grade(value) when value in [:green, "green", "🟢"], do: :green
  defp normalize_grade(value) when value in [:yellow, "yellow", "🟡"], do: :yellow
  defp normalize_grade(value) when value in [:red, "red", "🔴"], do: :red
  defp normalize_grade(value), do: value

  defp normalize_status(value) when is_binary(value), do: String.to_atom(value)
  defp normalize_status(value), do: value

  defp normalize_stage(value) when is_binary(value), do: String.to_atom(value)
  defp normalize_stage(value), do: value

  defp source_project_ref?(ref) when is_binary(ref) do
    String.contains?(ref, ["source-project", ".tscn", ".tres", ".meta", ".gltf", ".glb"])
  end

  defp source_project_ref?(_), do: false

  defp unsafe_ref?(value) when is_binary(value) do
    lower = String.downcase(value)

    String.contains?(value, "..") or String.starts_with?(value, "/") or
      String.contains?(value, "\\") or
      String.contains?(lower, [
        "decompiled",
        "ilspy",
        "dnspy",
        "ripped",
        "shipped-build",
        "foreign-runtime",
        "live-bridge",
        "vendored_unity_runtime",
        "vendored_unreal_runtime"
      ])
  end

  defp unsafe_ref?(_), do: true

  defp state_hash?(value) when is_binary(value),
    do: String.match?(value, ~r/^fnv64:[0-9a-f]{16}$/)

  defp state_hash?(_), do: false

  defp list_value(map, key, alt_key) do
    case value(map, key, value(map, alt_key, [])) do
      list when is_list(list) -> list
      _ -> []
    end
  end

  defp value(map, key, default \\ nil)

  defp value(map, key, default) when is_map(map),
    do: Map.get(map, key, Map.get(map, to_string(key), default))

  defp value(_, _, default), do: default

  defp bool_value(map, key, default) do
    case value(map, key, default) do
      value when is_boolean(value) -> value
      _ -> default
    end
  end

  defp blank?(value), do: value in [nil, "", []]
end

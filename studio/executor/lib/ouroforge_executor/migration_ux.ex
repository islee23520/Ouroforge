defmodule OuroforgeExecutor.MigrationUX do
  @moduledoc """
  Local Studio Migration UX model for Era O/M94 (#2187).

  This is the Phoenix LiveView-facing control/presentation model used by the
  local Studio app. It prepares wizard state, renders Rust-owned fidelity
  reports, and creates fix-forward links to Era R/Era M routes. It does not
  parse source projects, grade fidelity, mutate artifacts, or own any import
  semantics; those remain in the Rust `ouroforge` data plane.
  """

  alias OuroforgeExecutor.{CLI, Contract}

  @schema "ouroforge.migration-ux.v1"
  @boundary "Era O migration on-ramp; one-way source-project import; source-only open/text inputs; clean-room re-derivation; Rust data plane owns adapter IR mapping fidelity oracle determinism evidence provenance and gates; Phoenix LiveView control + presentation only; read + gated-write; no trusted Elixir writes; no new data store; no live bridge; no embedded engine runtime; no decompiled source; no finished-game auto-port; Era R re-derives logic; Era M may route human-facing fix-forward; #1 and #23 remain open"

  @pubsub_topic "studio:migration-ux"

  @rust_shapes [
    %{
      name: "GodotMigrationIr",
      path: "crates/ouroforge-core/src/godot_2d_adapter_ir.rs",
      role: "Godot .tscn/.tres source-project skeleton IR and fidelity rows"
    },
    %{
      name: "UnityMigrationIr",
      path: "crates/ouroforge-core/src/unity_2d_adapter_ir.rs",
      role: "Unity Force-Text/.meta source-project skeleton IR and fidelity rows"
    },
    %{
      name: "MappingArtifact",
      path: "crates/ouroforge-core/src/ir_mapping_fidelity_classifier.rs",
      role: "IR-to-Ouroforge native mapping records and claimed-ported-unit guard"
    },
    %{
      name: "ImportVerificationReport",
      path: "crates/ouroforge-core/src/import_verification_report.rs",
      role: "composed import verification, deterministic state hash, and oracle/fidelity report"
    },
    %{
      name: "LogicTouchpointHandoff",
      path: "crates/ouroforge-core/src/logic_touchpoint_handoff.rs",
      role: "Era R clean-room re-derivation task records for behavior-bearing units"
    }
  ]

  defmodule Wizard do
    @moduledoc false
    @schema "ouroforge.migration-ux.v1"
    @boundary "Era O migration on-ramp; one-way source-project import; source-only open/text inputs; clean-room re-derivation; Rust data plane owns adapter IR mapping fidelity oracle determinism evidence provenance and gates; Phoenix LiveView control + presentation only; read + gated-write; no trusted Elixir writes; no new data store; no live bridge; no embedded engine runtime; no decompiled source; no finished-game auto-port; Era R re-derives logic; Era M may route human-facing fix-forward; #1 and #23 remain open"
    @pubsub_topic "studio:migration-ux"
    defstruct schemaVersion: @schema,
              sourceEngine: nil,
              sourceProjectPath: nil,
              operatorLabel: nil,
              licenseNotes: nil,
              outputRef: nil,
              steps: [],
              routeCli: [],
              rustShapes: [],
              pubsubTopic: @pubsub_topic,
              boundary: @boundary,
              readGatedWrite: true,
              liveViewPresentation: true,
              phoenixPubSubTopic: @pubsub_topic,
              otpSessionStateOnly: true,
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

  defmodule FidelityRow do
    @moduledoc false
    defstruct [
      :id,
      :sourceRef,
      :targetRef,
      :grade,
      :label,
      :rationale,
      :evidenceRef,
      :oracleStatus,
      :eraRTaskRef,
      portClaimAllowed: false
    ]
  end

  defmodule ReportView do
    @moduledoc false
    @schema "ouroforge.migration-ux.v1"
    @boundary "Era O migration on-ramp; one-way source-project import; source-only open/text inputs; clean-room re-derivation; Rust data plane owns adapter IR mapping fidelity oracle determinism evidence provenance and gates; Phoenix LiveView control + presentation only; read + gated-write; no trusted Elixir writes; no new data store; no live bridge; no embedded engine runtime; no decompiled source; no finished-game auto-port; Era R re-derives logic; Era M may route human-facing fix-forward; #1 and #23 remain open"
    defstruct schemaVersion: @schema,
              reportId: nil,
              sourceEngine: nil,
              sourceProjectPath: nil,
              irStateHash: nil,
              verificationStateHash: nil,
              mappingStateHash: nil,
              rows: [],
              summary: %{green: 0, yellow: 0, red: 0},
              claimedPortedUnits: [],
              oracleRule: nil,
              rustShapes: [],
              boundary: @boundary,
              readOnlyRendering: true,
              rustOwnedEvidence: true,
              elixirOwnsArtifactSemantics: false,
              directArtifactWrite: false,
              studioTrustedWriteAuthority: false
  end

  defmodule FixForwardLink do
    @moduledoc false
    defstruct [
      :rowId,
      :label,
      :targetEra,
      :routeCli,
      :reason,
      :sourceRef,
      :eraRTaskRef,
      readGatedWrite: true,
      trustedWriteAuthority: false,
      directArtifactWrite: false,
      portClaimAllowed: false
    ]
  end

  def schema, do: @schema
  def boundary, do: @boundary
  def pubsub_topic, do: @pubsub_topic
  def rust_shapes, do: @rust_shapes

  def new_wizard(attrs) when is_map(attrs) do
    engine = attrs |> value(:source_engine) |> normalize_engine()
    source_path = value(attrs, :source_project_path)
    output_ref = value(attrs, :output_ref, default_output(engine))

    wizard = %Wizard{
      sourceEngine: engine,
      sourceProjectPath: source_path,
      operatorLabel: value(attrs, :operator_label),
      licenseNotes: value(attrs, :license_notes),
      outputRef: output_ref,
      steps: [
        "select source-project root",
        "Rust preflight rejects non-source/build/decompiled inputs",
        "Rust adapter imports declarative skeleton to deterministic IR",
        "Rust classifier emits 🟢/🟡/🔴 fidelity rows and oracle requirements",
        "Studio renders report and routes 🔴 gaps to Era R clean-room tasks"
      ],
      routeCli: import_route(engine, source_path, output_ref),
      rustShapes: @rust_shapes
    }

    with :ok <- validate_wizard(wizard) do
      {:ok, wizard}
    end
  end

  def validate_wizard(%Wizard{} = wizard) do
    cond do
      wizard.schemaVersion != @schema ->
        {:error, :unsupported_schema}

      wizard.sourceEngine not in ["godot-2d", "unity-2d"] ->
        {:error, :unsupported_source_engine}

      blank?(wizard.sourceProjectPath) or unsafe_source_path?(wizard.sourceProjectPath) ->
        {:error, :invalid_source_project_path}

      not source_only?(wizard.sourceProjectPath) ->
        {:error, :source_project_only}

      not wizard.readGatedWrite or not wizard.liveViewPresentation or
          not wizard.otpSessionStateOnly ->
        {:error, :not_control_presentation}

      not wizard.rustDataPlaneOwnsTruth or wizard.elixirOwnsArtifactSemantics ->
        {:error, :two_plane_boundary_broken}

      wizard.directArtifactWrite or wizard.studioTrustedWriteAuthority or wizard.newDataStore ->
        {:error, :trusted_write_forbidden}

      wizard.liveBridge or wizard.embeddedEngineRuntime or wizard.decompiledSourceAccepted or
          wizard.finishedGameAutoPort ->
        {:error, :migration_boundary_broken}

      wizard.claimedPortedUnits != [] ->
        {:error, :ported_claim_forbidden}

      not Contract.allowed_cli_family?(wizard.routeCli) ->
        {:error, {:outside_frozen_cli_surface, wizard.routeCli}}

      true ->
        :ok
    end
  end

  def import(%Wizard{} = wizard, opts \\ []) do
    with :ok <- validate_wizard(wizard),
         {:ok, result} <- CLI.run(wizard.routeCli, opts) do
      {:ok,
       %{
         status: :import_invoked,
         routeCli: wizard.routeCli,
         cliResult: result,
         trustedWriteAuthority: false,
         directArtifactWrite: false,
         rustDataPlaneOwnsTruth: true,
         pubsubTopic: wizard.pubsubTopic
       }}
    end
  end

  def report_view(report) when is_map(report) do
    rows = report |> list_value(:rows, :fidelityRows) |> Enum.map(&row_from_map/1)

    view = %ReportView{
      reportId: value(report, :report_id, value(report, :reportId, "migration-report")),
      sourceEngine: normalize_engine(value(report, :source_engine, value(report, :sourceEngine))),
      sourceProjectPath: value(report, :source_project_path, value(report, :sourceProjectPath)),
      irStateHash: value(report, :ir_state_hash, value(report, :irStateHash)),
      verificationStateHash:
        value(report, :verification_state_hash, value(report, :verificationStateHash)),
      mappingStateHash: value(report, :mapping_state_hash, value(report, :mappingStateHash)),
      rows: rows,
      summary: summarize(rows),
      claimedPortedUnits: list_value(report, :claimed_ported_units, :claimedPortedUnits),
      oracleRule:
        value(
          report,
          :oracle_rule,
          value(report, :oracleRule, "ported_claim_allowed=false until oracle passes")
        ),
      rustShapes: @rust_shapes
    }

    with :ok <- validate_report(view) do
      {:ok, view}
    end
  end

  def validate_report(%ReportView{} = view) do
    cond do
      view.schemaVersion != @schema ->
        {:error, :unsupported_schema}

      view.sourceEngine not in ["godot-2d", "unity-2d"] ->
        {:error, :unsupported_source_engine}

      blank?(view.sourceProjectPath) ->
        {:error, :missing_source_project}

      blank?(view.irStateHash) and blank?(view.verificationStateHash) and
          blank?(view.mappingStateHash) ->
        {:error, :missing_deterministic_hash}

      view.claimedPortedUnits != [] ->
        {:error, :ported_claim_forbidden}

      Enum.any?(view.rows, &(&1.portClaimAllowed or invalid_row?(&1))) ->
        {:error, :invalid_fidelity_row}

      Enum.any?(view.rows, &(&1.grade == :red and blank?(&1.eraRTaskRef))) ->
        {:error, :red_without_era_r_task}

      view.directArtifactWrite or view.studioTrustedWriteAuthority or
          view.elixirOwnsArtifactSemantics ->
        {:error, :trusted_write_forbidden}

      true ->
        :ok
    end
  end

  def fix_forward_links(%ReportView{} = view) do
    with :ok <- validate_report(view) do
      {:ok,
       view.rows
       |> Enum.reject(&(&1.grade == :green))
       |> Enum.map(&fix_forward_link/1)}
    end
  end

  def route_fix_forward(%FixForwardLink{} = link, opts \\ []) do
    cond do
      link.trustedWriteAuthority or link.directArtifactWrite or link.portClaimAllowed ->
        {:error, :trusted_write_forbidden}

      not Contract.allowed_cli_family?(link.routeCli) ->
        {:error, {:outside_frozen_cli_surface, link.routeCli}}

      true ->
        CLI.run(link.routeCli, opts)
    end
  end

  def render_report(%ReportView{} = view) do
    with :ok <- validate_report(view) do
      lines = [
        "Migration UX fidelity report: #{view.reportId}",
        "Source: #{view.sourceEngine} #{view.sourceProjectPath}",
        "Hashes: #{Enum.reject([view.irStateHash, view.mappingStateHash, view.verificationStateHash], &blank?/1) |> Enum.join(" | ")}",
        "Summary: 🟢 #{view.summary.green} / 🟡 #{view.summary.yellow} / 🔴 #{view.summary.red}",
        "Oracle: #{view.oracleRule}",
        "Boundary: one-way source-project on-ramp; clean-room re-derivation; no port claim without oracle"
      ]

      {:ok, Enum.join(lines, "\n")}
    end
  end

  defp fix_forward_link(%FidelityRow{grade: :red} = row) do
    %FixForwardLink{
      rowId: row.id,
      label: "Route #{row.id} to Era R clean-room re-derivation",
      targetEra: "Era R",
      routeCli: ["behavior", "draft", "preview", row.eraRTaskRef || "missing-era-r-task"],
      reason: row.rationale,
      sourceRef: row.sourceRef,
      eraRTaskRef: row.eraRTaskRef
    }
  end

  defp fix_forward_link(%FidelityRow{} = row) do
    %FixForwardLink{
      rowId: row.id,
      label: "Open review/fix-forward context for #{row.id}",
      targetEra: "Era M",
      routeCli: ["patch-preview", "show", row.evidenceRef || row.id],
      reason: row.rationale,
      sourceRef: row.sourceRef,
      eraRTaskRef: row.eraRTaskRef
    }
  end

  defp row_from_map(%FidelityRow{} = row), do: row

  defp row_from_map(row) when is_map(row) do
    grade = normalize_grade(value(row, :grade))

    %FidelityRow{
      id: value(row, :id, value(row, :rowId, "row-#{System.unique_integer([:positive])}")),
      sourceRef: value(row, :source_ref, value(row, :sourceRef)),
      targetRef: value(row, :target_ref, value(row, :targetRef)),
      grade: grade,
      label: value(row, :label, label_for_grade(grade)),
      rationale: value(row, :rationale),
      evidenceRef: value(row, :evidence_ref, value(row, :evidenceRef)),
      oracleStatus: value(row, :oracle_status, value(row, :oracleStatus, "missing")),
      eraRTaskRef: value(row, :era_r_task_ref, value(row, :eraRTaskRef)),
      portClaimAllowed:
        bool_value(row, :port_claim_allowed, bool_value(row, :portClaimAllowed, false))
    }
  end

  defp invalid_row?(%FidelityRow{} = row) do
    row.grade not in [:green, :yellow, :red] or blank?(row.id) or blank?(row.sourceRef) or
      blank?(row.rationale)
  end

  defp summarize(rows) do
    %{
      green: Enum.count(rows, &(&1.grade == :green)),
      yellow: Enum.count(rows, &(&1.grade == :yellow)),
      red: Enum.count(rows, &(&1.grade == :red))
    }
  end

  defp import_route("godot-2d", project, output),
    do: ["migration", "verify-demo", "--project", project, "--output", output]

  defp import_route("unity-2d", project, output),
    do: ["migration", "unity-demo", "--project", project, "--output", output]

  defp import_route(_, project, output),
    do: ["migration", "unsupported", project || "", output || ""]

  defp default_output("godot-2d"),
    do: "examples/godot-2d-adapter-v1/generated/import-verification-report.json"

  defp default_output("unity-2d"),
    do: "examples/unity-2d-adapter-v1/generated/fidelity-report.json"

  defp default_output(_), do: "generated/migration-fidelity-report.json"

  defp normalize_engine(value) when value in [:godot, :godot_2d, "godot", "godot-2d"],
    do: "godot-2d"

  defp normalize_engine(value) when value in [:unity, :unity_2d, "unity", "unity-2d"],
    do: "unity-2d"

  defp normalize_engine(value), do: value

  defp normalize_grade(value) when value in [:green, "green", "🟢", "clean"], do: :green
  defp normalize_grade(value) when value in [:yellow, "yellow", "🟡", "flagged"], do: :yellow
  defp normalize_grade(value) when value in [:red, "red", "🔴", "rederive", "re-derive"], do: :red
  defp normalize_grade(value), do: value

  defp label_for_grade(:green), do: "Clean skeleton import"
  defp label_for_grade(:yellow), do: "Needs review / best-effort"
  defp label_for_grade(:red), do: "Re-derive / unsupported"
  defp label_for_grade(_), do: "Unknown"

  defp source_only?(path) when is_binary(path) do
    blocked = [
      "/Build/",
      "/Builds/",
      "/Library/",
      "/Temp/",
      ".apk",
      ".ipa",
      ".exe",
      ".app",
      ".dll",
      "AssetBundle",
      "globalgamemanagers",
      "resources.assets",
      "decompiled"
    ]

    not Enum.any?(blocked, &String.contains?(path, &1))
  end

  defp source_only?(_), do: false

  defp unsafe_source_path?(path) when is_binary(path),
    do: String.contains?(path, "..") or String.trim(path) == ""

  defp unsafe_source_path?(_), do: true

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

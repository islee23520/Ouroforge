defmodule OuroforgeExecutor.StudioLiveShell do
  @moduledoc """
  Minimal Phoenix LiveView Studio shell model for M85 (#2091).

  This module is intentionally dependency-free so the local executor can keep
  its existing small OTP footprint while presenting the state shape a Phoenix
  LiveView renders. The shell subscribes to `OuroforgeExecutor.LocalPubSub`
  topics, renders Rust-owned evidence/diagnosis/journal/verdict read models,
  and never writes trusted artifacts, ledgers, evidence, source, scenes, or
  release state. Any future write-affecting action must become a Rust-validated
  intervention-as-evidence request through the existing gates.
  """

  alias OuroforgeExecutor.LocalPubSub

  @schema "ouroforge.studio-live-shell.v1"
  @boundary "local-first Phoenix LiveView control + presentation plane; read + gated-write; intervention-as-evidence; two-plane Rust data plane owns artifact truth validation determinism evidence provenance review/apply scene/source-apply evaluator gates; Elixir renders routes and refreshes only; no trusted writes; no raw bypass; no command bridge; no new store; CLI fallback completes without Studio or human input; hosted multi-user collaborative Studio deferred; fun/taste and release go/no-go remain human; #1 and #23 remain open"

  @view_kinds [:diagnosis, :evidence, :journal, :verdict]

  @topics %{
    evidence: "studio:evidence",
    diagnosis: "studio:diagnosis",
    journal: "studio:journal",
    verdict: "studio:verdict"
  }

  defmodule View do
    @moduledoc false
    defstruct [
      :id,
      :title,
      :topic,
      :kind,
      :status,
      entries: [],
      empty?: true,
      read_only?: true,
      rust_owned?: true,
      trusted_write_authority?: false,
      direct_artifact_write?: false,
      command_bridge?: false
    ]
  end

  defstruct schemaVersion: @schema,
            shell: "Phoenix LiveView Studio Shell",
            nav: [],
            active: :evidence,
            views: %{},
            topics: @topics,
            boundary: @boundary,
            live?: true,
            localFirst: true,
            cliFallbackSupported: true,
            autonomousLoopRequiresHuman: false,
            readOnlyRendering: true,
            readGatedWrite: true,
            interventionAsEvidence: true,
            rustDataPlaneOwnsTruth: true,
            elixirOwnsArtifactSemantics: false,
            trustedWriteAuthority: false,
            directArtifactWrite: false,
            commandBridge: false,
            newDataStore: false,
            hostedCollaborative: false,
            notes: []

  def schema, do: @schema
  def boundary, do: @boundary
  def topics, do: @topics

  def new(attrs \\ %{}) when is_map(attrs) do
    active = attrs |> value(:active, :evidence) |> normalize_active()

    shell = %__MODULE__{
      active: active,
      nav: nav(active),
      views: initial_views(),
      localFirst: bool_value(attrs, :local_first, true),
      cliFallbackSupported: bool_value(attrs, :cli_fallback_supported, true),
      autonomousLoopRequiresHuman: bool_value(attrs, :autonomous_loop_requires_human, false),
      readOnlyRendering: bool_value(attrs, :read_only_rendering, true),
      readGatedWrite: bool_value(attrs, :read_gated_write, true),
      interventionAsEvidence: bool_value(attrs, :intervention_as_evidence, true),
      rustDataPlaneOwnsTruth: bool_value(attrs, :rust_data_plane_owns_truth, true),
      elixirOwnsArtifactSemantics: bool_value(attrs, :elixir_owns_artifact_semantics, false),
      trustedWriteAuthority: bool_value(attrs, :trusted_write_authority, false),
      directArtifactWrite: bool_value(attrs, :direct_artifact_write, false),
      commandBridge: bool_value(attrs, :command_bridge, false),
      newDataStore: bool_value(attrs, :new_data_store, false),
      hostedCollaborative: bool_value(attrs, :hosted_collaborative, false),
      notes: [
        "renders Rust-owned evidence/diagnosis/journal/verdict read models",
        "PubSub refresh is local presentation/control only",
        "CLI fallback remains sufficient for zero-human autonomous runs"
      ]
    }

    with :ok <- validate(shell) do
      {:ok, shell}
    end
  end

  def validate(%__MODULE__{} = shell) do
    cond do
      shell.schemaVersion != @schema ->
        {:error, :unsupported_schema}

      contains_raw_bypass?(shell.boundary) ->
        {:error, :raw_bypass_forbidden}

      not shell.localFirst or not shell.cliFallbackSupported or shell.autonomousLoopRequiresHuman ->
        {:error, :autonomy_or_cli_fallback_broken}

      not shell.readOnlyRendering or not shell.readGatedWrite or not shell.interventionAsEvidence ->
        {:error, :not_read_gated_write}

      not shell.rustDataPlaneOwnsTruth or shell.elixirOwnsArtifactSemantics ->
        {:error, :two_plane_boundary_broken}

      shell.trustedWriteAuthority or shell.directArtifactWrite or shell.commandBridge or
        shell.newDataStore or shell.hostedCollaborative ->
        {:error, :trusted_write_or_scope_drift}

      not complete_views?(shell.views) ->
        {:error, :incomplete_views}

      true ->
        :ok
    end
  end

  def subscribe(%__MODULE__{} = shell, registry \\ OuroforgeExecutor.PubSub) do
    shell.topics
    |> Map.values()
    |> Enum.each(&LocalPubSub.subscribe(&1, registry))

    {:ok, shell}
  end

  def broadcast(kind, payload, registry \\ OuroforgeExecutor.PubSub) when is_atom(kind) do
    with {:ok, event} <- rust_owned_event(kind, payload),
         {:ok, topic} <- topic_for(kind) do
      :ok = LocalPubSub.broadcast(topic, {:studio_live_shell, event}, registry)
      {:ok, event}
    end
  end

  def apply_event(%__MODULE__{} = shell, {:studio_live_shell, event}) do
    apply_event(shell, event)
  end

  def apply_event(%__MODULE__{} = shell, %{kind: kind} = event) do
    kind = normalize_active(kind)
    view = Map.fetch!(shell.views, kind)
    entries = [event | view.entries] |> Enum.take(50)
    view = %{view | entries: entries, empty?: false, status: event.status}

    updated = %{shell | views: Map.put(shell.views, kind, view)}

    with :ok <- validate(updated) do
      {:ok, updated}
    end
  end

  def render(%__MODULE__{} = shell) do
    %{
      schemaVersion: shell.schemaVersion,
      shell: shell.shell,
      nav: shell.nav,
      active: shell.active,
      views: shell.views,
      boundary: shell.boundary,
      readOnlyRendering: shell.readOnlyRendering,
      readGatedWrite: shell.readGatedWrite,
      interventionAsEvidence: shell.interventionAsEvidence,
      rustDataPlaneOwnsTruth: shell.rustDataPlaneOwnsTruth,
      trustedWriteAuthority: shell.trustedWriteAuthority,
      directArtifactWrite: shell.directArtifactWrite,
      commandBridge: shell.commandBridge,
      localFirst: shell.localFirst,
      cliFallbackSupported: shell.cliFallbackSupported,
      autonomousLoopRequiresHuman: shell.autonomousLoopRequiresHuman,
      notes: shell.notes
    }
  end

  def rust_owned_event(kind, payload) when is_map(payload) do
    kind = normalize_active(kind)

    with {:ok, _topic} <- topic_for(kind),
         :ok <- validate_payload(payload) do
      {:ok,
       %{
         schemaVersion: "ouroforge.studio-live-event.v1",
         kind: kind,
         id: payload |> value(:id, "#{kind}-#{System.unique_integer([:positive])}"),
         title: payload |> value(:title, title_for(kind)),
         status: payload |> value(:status, :present),
         evidenceRefs: payload |> value(:evidence_refs, []),
         runRef: payload |> value(:run_ref, nil),
         diagnosisId: payload |> value(:diagnosis_id, nil),
         verdict: payload |> value(:verdict, nil),
         journalRef: payload |> value(:journal_ref, nil),
         rustOwned: true,
         readOnly: true,
         trustedWriteAuthority: false,
         directArtifactWrite: false,
         commandBridge: false,
         receivedAt: payload |> value(:received_at, "local-pubsub")
       }}
    end
  end

  def rust_owned_event(_kind, _payload), do: {:error, :malformed_payload}

  defp validate_payload(payload) do
    cond do
      bool_value(payload, :trusted_write_authority, false) ->
        {:error, :trusted_write_forbidden}

      bool_value(payload, :direct_artifact_write, false) ->
        {:error, :trusted_write_forbidden}

      bool_value(payload, :command_bridge, false) ->
        {:error, :command_bridge_forbidden}

      not valid_refs?(value(payload, :evidence_refs, [])) ->
        {:error, :invalid_evidence_refs}

      true ->
        :ok
    end
  end

  defp initial_views do
    @topics
    |> Enum.map(fn {kind, topic} ->
      {kind,
       %View{
         id: "studio-#{kind}",
         title: title_for(kind),
         topic: topic,
         kind: kind,
         status: :waiting_for_rust_read_model
       }}
    end)
    |> Map.new()
  end

  defp nav(active) do
    @view_kinds
    |> Enum.map(fn kind ->
      %{id: "nav-#{kind}", kind: kind, label: title_for(kind), active?: kind == active}
    end)
  end

  defp complete_views?(views) when is_map(views) do
    Enum.all?(@view_kinds, fn kind ->
      case Map.get(views, kind) do
        %View{} = view ->
          view.read_only? and view.rust_owned? and not view.trusted_write_authority? and
            not view.direct_artifact_write? and not view.command_bridge?

        _ ->
          false
      end
    end)
  end

  defp complete_views?(_), do: false

  defp topic_for(kind) do
    case Map.fetch(@topics, kind) do
      {:ok, topic} -> {:ok, topic}
      :error -> {:error, :unsupported_view}
    end
  end

  defp title_for(:evidence), do: "Evidence"
  defp title_for(:diagnosis), do: "Diagnosis"
  defp title_for(:journal), do: "Journal"
  defp title_for(:verdict), do: "Verdict"
  defp title_for(kind), do: kind |> to_string() |> String.capitalize()

  defp normalize_active(value) when is_binary(value) do
    case value do
      "diagnosis" -> :diagnosis
      "evidence" -> :evidence
      "journal" -> :journal
      "verdict" -> :verdict
      _ -> :evidence
    end
  end

  defp normalize_active(value) when value in @view_kinds, do: value
  defp normalize_active(_), do: :evidence

  defp valid_refs?(refs) when is_list(refs) do
    Enum.all?(refs, &(is_binary(&1) and &1 != "" and not String.contains?(&1, "..")))
  end

  defp valid_refs?(_), do: false

  defp value(map, key, default) do
    Map.get(map, key) || Map.get(map, Atom.to_string(key), default)
  end

  defp bool_value(map, key, default) do
    case value(map, key, default) do
      value when is_boolean(value) -> value
      _ -> default
    end
  end

  defp contains_raw_bypass?(value) when is_binary(value) do
    String.contains?(String.downcase(value), ["raw_write_bypass", "raw apply bypass"])
  end

  defp contains_raw_bypass?(_), do: false
end

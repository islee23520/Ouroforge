defmodule OuroforgeExecutor.StudioInterventionPanels do
  @moduledoc """
  Integrated Studio intervention and authoring panel model for M85 (#2092).

  The module composes Era M steering, proposal amendment, human constraint,
  diagnosis correction, takeover/handback, and authoring controls behind a
  single local Phoenix LiveView presentation/control-plane contract. Each action
  is captured as intervention-as-evidence and routed to an existing Rust-owned
  gate family. This module never writes artifacts, ledgers, evidence, source,
  scenes, or release state; it only renders/captures/routes and broadcasts local
  feedback for the Studio shell.
  """

  alias OuroforgeExecutor.{
    DiagnosisCorrectionSurface,
    HumanConstraintSurface,
    LocalPubSub,
    ProposalAmendmentSurface,
    SteeringDirective,
    StudioLiveShell
  }

  @schema "ouroforge.studio-intervention-panels.v1"
  @topic "studio:interventions"
  @boundary "integrated intervention and authoring Studio panels; intervention-as-evidence; read + gated-write; two-plane; local-first; Rust data plane owns artifact truth validation determinism evidence provenance review/apply scene/source-apply evaluator gates; Elixir/OTP + Phoenix LiveView control + presentation only; no trusted writes; no raw bypass; no command bridge; no new store; CLI fallback completes without Studio or human input; hosted multi-user collaborative Studio deferred; fun/taste and release go/no-go remain human; #1 and #23 remain open"

  @panels [:steering, :amendment, :constraint, :correction, :takeover, :handback, :authoring]

  defmodule Panel do
    @moduledoc false
    defstruct [
      :id,
      :kind,
      :title,
      :gate_family,
      route_cli: [],
      read_only?: false,
      gated_write?: true,
      intervention_as_evidence?: true,
      rust_owned_gate?: true,
      trusted_write_authority?: false,
      direct_artifact_write?: false,
      command_bridge?: false
    ]
  end

  defmodule Submission do
    @moduledoc false
    defstruct [
      :schemaVersion,
      :submissionId,
      :panel,
      :kind,
      :status,
      :gateFamily,
      :routeCli,
      :payload,
      :provenanceRefs,
      :feedbackTopic,
      :boundary,
      interventionAsEvidence: true,
      readGatedWrite: true,
      rustDataPlaneRequired: true,
      directArtifactWrite: false,
      trustedWriteAuthority: false,
      commandBridge: false,
      elixirOwnsArtifactSemantics: false,
      humanRequiredForAutonomousLoop: false,
      cliFallbackSupported: true,
      localFirst: true,
      autoApplyPerformed: false
    ]
  end

  defstruct schemaVersion: @schema,
            panels: %{},
            feedbackTopic: @topic,
            boundary: @boundary,
            readGatedWrite: true,
            interventionAsEvidence: true,
            rustDataPlaneRequired: true,
            directArtifactWrite: false,
            trustedWriteAuthority: false,
            commandBridge: false,
            newDataStore: false,
            hostedCollaborative: false,
            humanRequiredForAutonomousLoop: false,
            cliFallbackSupported: true,
            localFirst: true

  def schema, do: @schema
  def topic, do: @topic
  def boundary, do: @boundary
  def panel_kinds, do: @panels

  def new(attrs \\ %{}) when is_map(attrs) do
    surface = %__MODULE__{
      panels: panel_catalog(),
      directArtifactWrite: bool_value(attrs, :direct_artifact_write, false),
      trustedWriteAuthority: bool_value(attrs, :trusted_write_authority, false),
      commandBridge: bool_value(attrs, :command_bridge, false),
      newDataStore: bool_value(attrs, :new_data_store, false),
      hostedCollaborative: bool_value(attrs, :hosted_collaborative, false),
      humanRequiredForAutonomousLoop:
        bool_value(attrs, :human_required_for_autonomous_loop, false),
      cliFallbackSupported: bool_value(attrs, :cli_fallback_supported, true),
      localFirst: bool_value(attrs, :local_first, true)
    }

    with :ok <- validate(surface) do
      {:ok, surface}
    end
  end

  def validate(%__MODULE__{} = surface) do
    cond do
      surface.schemaVersion != @schema ->
        {:error, :unsupported_schema}

      contains_raw_bypass?(surface.boundary) ->
        {:error, :raw_bypass_forbidden}

      not surface.readGatedWrite or not surface.interventionAsEvidence or
          not surface.rustDataPlaneRequired ->
        {:error, :not_intervention_evidence}

      surface.directArtifactWrite or surface.trustedWriteAuthority or surface.commandBridge or
        surface.newDataStore or surface.hostedCollaborative ->
        {:error, :trusted_write_or_scope_drift}

      surface.humanRequiredForAutonomousLoop or not surface.cliFallbackSupported or
          not surface.localFirst ->
        {:error, :autonomy_or_cli_fallback_broken}

      not complete_panels?(surface.panels) ->
        {:error, :incomplete_panels}

      true ->
        :ok
    end
  end

  def subscribe(registry \\ OuroforgeExecutor.PubSub) do
    LocalPubSub.subscribe(@topic, registry)
  end

  def submit(kind, attrs, opts \\ []) when is_atom(kind) and is_map(attrs) do
    registry = Keyword.get(opts, :pubsub, OuroforgeExecutor.PubSub)

    with {:ok, surface} <- new(),
         {:ok, submission} <- build_submission(surface, kind, attrs),
         :ok <- validate_submission(submission) do
      :ok = LocalPubSub.broadcast(@topic, {:studio_intervention_panel, submission}, registry)
      {:ok, submission}
    end
  end

  def render(%__MODULE__{} = surface) do
    %{
      schemaVersion: surface.schemaVersion,
      panels: surface.panels,
      feedbackTopic: surface.feedbackTopic,
      boundary: surface.boundary,
      readGatedWrite: surface.readGatedWrite,
      interventionAsEvidence: surface.interventionAsEvidence,
      rustDataPlaneRequired: surface.rustDataPlaneRequired,
      trustedWriteAuthority: surface.trustedWriteAuthority,
      directArtifactWrite: surface.directArtifactWrite,
      commandBridge: surface.commandBridge,
      localFirst: surface.localFirst,
      cliFallbackSupported: surface.cliFallbackSupported,
      shellTopics: StudioLiveShell.topics()
    }
  end

  defp build_submission(surface, kind, attrs) when kind in @panels do
    panel = Map.fetch!(surface.panels, kind)

    with {:ok, payload} <- payload_for(kind, attrs) do
      {:ok,
       %Submission{
         schemaVersion: "ouroforge.studio-gated-submission.v1",
         submissionId: submission_id(kind, attrs),
         panel: panel.id,
         kind: kind,
         status: :queued_for_rust_gate,
         gateFamily: panel.gate_family,
         routeCli: panel.route_cli,
         payload: payload,
         provenanceRefs: provenance_refs(attrs),
         feedbackTopic: @topic,
         boundary: @boundary
       }}
    end
  end

  defp build_submission(_surface, _kind, _attrs), do: {:error, :unsupported_panel}

  defp payload_for(:steering, attrs) do
    directive = SteeringDirective.new(attrs)

    with {:ok, directive} <- SteeringDirective.validate(directive) do
      {:ok,
       %{
         type: :steering_directive,
         directive: directive,
         rustRoute: SteeringDirective.to_cli_args(directive, :validate),
         existingGate: OuroforgeExecutor.LiveSteering
       }}
    end
  end

  defp payload_for(:amendment, attrs) do
    with {:ok, request} <- ProposalAmendmentSurface.capture(attrs),
         {:ok, submission} <- ProposalAmendmentSurface.to_rust_submission(request) do
      {:ok, %{type: :proposal_amendment, request: request, rustSubmission: submission}}
    end
  end

  defp payload_for(:constraint, attrs) do
    with {:ok, request} <- HumanConstraintSurface.capture(attrs),
         {:ok, record} <- HumanConstraintSurface.to_rust_constraint_record(request) do
      {:ok, %{type: :human_constraint, request: request, rustSubmission: record}}
    end
  end

  defp payload_for(:correction, attrs) do
    with {:ok, request} <- DiagnosisCorrectionSurface.capture(attrs),
         {:ok, submission} <- DiagnosisCorrectionSurface.to_rust_submission(request) do
      {:ok, %{type: :diagnosis_correction, request: request, rustSubmission: submission}}
    end
  end

  defp payload_for(kind, attrs) when kind in [:takeover, :handback] do
    with :ok <- require_text(attrs, :stage_id),
         :ok <- require_text(attrs, :campaign_id),
         :ok <- require_text(attrs, :actor_id),
         :ok <- require_text(attrs, :reason),
         :ok <- reject_raw_bypass(value(attrs, :reason)) do
      {:ok,
       %{
         type: kind,
         stageId: value(attrs, :stage_id),
         campaignId: value(attrs, :campaign_id),
         actorId: value(attrs, :actor_id),
         reason: value(attrs, :reason),
         rustRoute: ["stage", Atom.to_string(kind), value(attrs, :stage_id)],
         existingGate: OuroforgeExecutor.StageTakeover
       }}
    end
  end

  defp payload_for(:authoring, attrs) do
    with :ok <- require_text(attrs, :draft_id),
         :ok <- require_text(attrs, :target_ref),
         :ok <- require_text(attrs, :target_base_ref),
         :ok <- require_text(attrs, :review_apply_ref),
         :ok <- require_text(attrs, :scene_or_source_apply_ref),
         :ok <- require_text(attrs, :summary),
         :ok <- reject_raw_bypass(value(attrs, :summary)),
         :ok <- validate_authoring_flags(attrs) do
      {:ok,
       %{
         type: :interactive_authoring_draft,
         draftId: value(attrs, :draft_id),
         targetRef: value(attrs, :target_ref),
         targetBaseRef: value(attrs, :target_base_ref),
         summary: value(attrs, :summary),
         reviewApplyRef: value(attrs, :review_apply_ref),
         sceneOrSourceApplyRef: value(attrs, :scene_or_source_apply_ref),
         rustRoute: ["source-apply", "review", value(attrs, :draft_id)],
         existingGates: ["review/apply", "scene/source-apply", "evaluator", "evidence/provenance"],
         directArtifactWrite: false,
         autoApplyPerformed: false
       }}
    end
  end

  defp validate_submission(%Submission{} = submission) do
    cond do
      submission.schemaVersion != "ouroforge.studio-gated-submission.v1" ->
        {:error, :unsupported_submission_schema}

      not submission.interventionAsEvidence or not submission.readGatedWrite or
          not submission.rustDataPlaneRequired ->
        {:error, :not_intervention_evidence}

      submission.directArtifactWrite or submission.trustedWriteAuthority or
        submission.commandBridge or
        submission.elixirOwnsArtifactSemantics or submission.autoApplyPerformed ->
        {:error, :trusted_write_forbidden}

      submission.humanRequiredForAutonomousLoop or not submission.cliFallbackSupported or
          not submission.localFirst ->
        {:error, :autonomy_or_cli_fallback_broken}

      contains_raw_bypass?(inspect(submission.payload)) ->
        {:error, :raw_bypass_forbidden}

      true ->
        :ok
    end
  end

  defp panel_catalog do
    %{
      steering:
        panel(:steering, "Steering Console", "live campaign steering directive gate", [
          "loop",
          "step"
        ]),
      amendment:
        panel(:amendment, "Amendment Panel", "proposal amendment reverify gate", [
          "proposal-amendment",
          "validate"
        ]),
      constraint:
        panel(:constraint, "Constraint Panel", "human constraint evaluator gate", ["evaluate"]),
      correction:
        panel(:correction, "Correction Panel", "diagnosis correction evaluator gate", [
          "diagnosis-correction",
          "validate"
        ]),
      takeover:
        panel(:takeover, "Takeover Control", "stage takeover validation and record gate", [
          "stage",
          "takeover"
        ]),
      handback:
        panel(:handback, "Handback Control", "stage handback reverify and record gate", [
          "stage",
          "handback"
        ]),
      authoring:
        panel(:authoring, "Interactive Authoring", "review/apply and scene/source-apply gates", [
          "source-apply",
          "review"
        ])
    }
  end

  defp panel(kind, title, gate_family, route_cli) do
    %Panel{
      id: "studio-panel-#{kind}",
      kind: kind,
      title: title,
      gate_family: gate_family,
      route_cli: route_cli
    }
  end

  defp complete_panels?(panels) when is_map(panels) do
    Enum.all?(@panels, fn kind ->
      case Map.get(panels, kind) do
        %Panel{} = panel ->
          panel.gated_write? and panel.intervention_as_evidence? and panel.rust_owned_gate? and
            not panel.trusted_write_authority? and not panel.direct_artifact_write? and
            not panel.command_bridge?

        _ ->
          false
      end
    end)
  end

  defp complete_panels?(_), do: false

  defp submission_id(kind, attrs) do
    value(attrs, :submission_id) || value(attrs, :id) || value(attrs, :draft_id) ||
      "#{kind}-#{System.unique_integer([:positive])}"
  end

  defp provenance_refs(attrs) do
    attrs
    |> value(:provenance_refs, [])
    |> List.wrap()
    |> Enum.concat(List.wrap(value(attrs, :base_refs, [])))
    |> Enum.reject(&is_nil/1)
    |> Enum.uniq()
  end

  defp validate_authoring_flags(attrs) do
    cond do
      bool_value(attrs, :direct_artifact_write, false) -> {:error, :trusted_write_forbidden}
      bool_value(attrs, :auto_apply_performed, false) -> {:error, :trusted_write_forbidden}
      bool_value(attrs, :command_bridge, false) -> {:error, :command_bridge_forbidden}
      true -> :ok
    end
  end

  defp require_text(attrs, key) do
    case value(attrs, key) do
      value when is_binary(value) and value != "" -> :ok
      _ -> {:error, {:missing_text, key}}
    end
  end

  defp reject_raw_bypass(value) do
    if contains_raw_bypass?(value), do: {:error, :raw_bypass_forbidden}, else: :ok
  end

  defp value(map, key, default \\ nil) do
    Map.get(map, key) || Map.get(map, Atom.to_string(key), default)
  end

  defp bool_value(map, key, default) do
    case value(map, key, default) do
      value when is_boolean(value) -> value
      _ -> default
    end
  end

  defp contains_raw_bypass?(value) when is_binary(value) do
    String.contains?(String.downcase(value), [
      "raw_write_bypass",
      "raw_apply_bypass",
      "trusted studio write"
    ])
  end

  defp contains_raw_bypass?(_), do: false
end

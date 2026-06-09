defmodule OuroforgeExecutor.GuidedGenerativeFrontDoor do
  @moduledoc """
  Local Phoenix LiveView control/presentation shell for the Era N guided
  generative front door (#2079).

  The shell lets a non-developer describe intent and inspect a proposal preview,
  but it remains inert Studio state until routed to the Rust data plane. It does
  not generate trusted artifacts, apply proposals, write ledgers/evidence, own
  proposal semantics, or certify gate results.
  """

  alias OuroforgeExecutor.Contract

  @boundary "guided generative front door; intervention-as-evidence; read + gated-write; Rust data plane validates and records; Elixir/OTP + Phoenix LiveView control + presentation only; Milestone 30 generative intake; review/apply, scene/source-apply, evaluator, evidence/provenance gates required; generation proposal-only; no raw bypass; deterministic preview; local-first CLI fallback; loop completes without human; fun/taste and release go/no-go remain human; #1 and #23 remain open"
  @template_ids ~w(grid-puzzle deck-roguelike engine-builder-deckbuilder)
  @route_cli ["generative-front-door", "validate"]

  defstruct [
    :schemaVersion,
    :sessionId,
    :brief,
    :conversationSummary,
    :templateId,
    :humanActor,
    :baseIntentRef,
    :preview,
    :routeCli,
    :boundary,
    interventionAsEvidence: true,
    readGatedWrite: true,
    proposalOnly: true,
    deterministicPreview: true,
    directArtifactWrite: false,
    rawBypassRequested: false,
    studioTrustedWriteAuthority: false,
    elixirOwnsProposalSemantics: false,
    autoApplyPerformed: false,
    reviewerBypass: false,
    humanRequiredForAutonomousLoop: false,
    cliFallbackSupported: true
  ]

  @type t :: %__MODULE__{}

  def boundary, do: @boundary
  def route_cli, do: @route_cli
  def template_ids, do: @template_ids

  def autonomous_default_demo do
    %{
      demo_id: "m82-guided-front-door-autonomous-default",
      status: :completed_without_human,
      human_intervention: :absent,
      waited_for_human?: false,
      human_surface_required?: false,
      cli_fallback_supported?: true,
      trusted_write_performed?: false,
      generated_proposal_applied?: false,
      evidence_refs: [
        "runs/m82/guided-front-door/no-human-required.json",
        "runs/m82/guided-front-door/cli-fallback-supported.json"
      ],
      boundary: @boundary
    }
  end

  def capture(attrs) when is_map(attrs) do
    attrs = normalize_attrs(attrs)

    request = %__MODULE__{
      schemaVersion: "ouroforge.guided-generative-front-door-capture.v1",
      sessionId: value(attrs, :session_id),
      brief: value(attrs, :brief),
      conversationSummary: value(attrs, :conversation_summary),
      templateId: value(attrs, :template_id),
      humanActor: value(attrs, :human_actor),
      baseIntentRef: value(attrs, :base_intent_ref),
      routeCli: @route_cli,
      boundary: @boundary
    }

    with :ok <- validate_capture(request) do
      {:ok, %__MODULE__{request | preview: build_preview(request)}}
    end
  end

  def preview(%__MODULE__{} = request) do
    with :ok <- validate(request) do
      {:ok, request.preview || build_preview(request)}
    end
  end

  def to_rust_submission(%__MODULE__{} = request) do
    with :ok <- validate(request),
         {:ok, preview} <- preview(request) do
      {:ok,
       %{
         "schemaVersion" => request.schemaVersion,
         "sessionId" => request.sessionId,
         "brief" => request.brief,
         "conversationSummary" => request.conversationSummary,
         "templateId" => request.templateId,
         "humanActor" => request.humanActor,
         "baseIntentRef" => request.baseIntentRef,
         "preview" => preview,
         "routeCli" => request.routeCli,
         "interventionAsEvidence" => true,
         "readGatedWrite" => true,
         "proposalOnly" => true,
         "deterministicPreview" => true,
         "directArtifactWrite" => false,
         "rawBypassRequested" => false,
         "studioTrustedWriteAuthority" => false,
         "elixirOwnsProposalSemantics" => false,
         "autoApplyPerformed" => false,
         "reviewerBypass" => false,
         "humanRequiredForAutonomousLoop" => false,
         "cliFallbackSupported" => true,
         "boundary" => request.boundary
       }}
    end
  end

  def route_to_rust(%__MODULE__{} = request, opts) when is_list(opts) do
    runner = Keyword.fetch!(opts, :runner)

    with {:ok, submission} <- to_rust_submission(request),
         {:ok, result} <- runner.(request.routeCli, submission) do
      {:ok,
       %{
         status: :verified_proposal,
         route_cli: request.routeCli,
         submission: submission,
         rust_result: result,
         trusted_write_performed?: false,
         auto_apply_performed?: false,
         review_apply_required?: true,
         studio_trusted_write_authority?: false,
         cli_fallback_supported?: true
       }}
    else
      {:error, reason} ->
        {:error,
         %{
           status: :blocked_by_gate,
           reason: reason,
           trusted_write_performed?: false,
           auto_apply_performed?: false,
           studio_trusted_write_authority?: false
         }}
    end
  end

  def validate(%__MODULE__{} = request) do
    with :ok <- validate_capture(request),
         :ok <- validate_preview(request.preview) do
      :ok
    end
  end

  defp validate_capture(%__MODULE__{} = request) do
    cond do
      request.schemaVersion != "ouroforge.guided-generative-front-door-capture.v1" ->
        {:error, :unsupported_schema}

      blank?(request.sessionId) or blank?(request.brief) or blank?(request.conversationSummary) ->
        {:error, :missing_intake}

      request.templateId not in @template_ids ->
        {:error, :unsupported_template}

      blank?(request.humanActor) or blank?(request.baseIntentRef) ->
        {:error, :missing_provenance}

      contains_raw_bypass?(request.brief) or contains_raw_bypass?(request.conversationSummary) or
          contains_raw_bypass?(request.baseIntentRef) ->
        {:error, :raw_bypass_forbidden}

      not request.interventionAsEvidence or not request.readGatedWrite or
        not request.proposalOnly or not request.deterministicPreview ->
        {:error, :not_gated_proposal_evidence}

      request.directArtifactWrite or request.rawBypassRequested or
        request.studioTrustedWriteAuthority or request.elixirOwnsProposalSemantics or
        request.autoApplyPerformed or request.reviewerBypass ->
        {:error, :trusted_write_forbidden}

      request.humanRequiredForAutonomousLoop or not request.cliFallbackSupported ->
        {:error, :autonomy_or_cli_fallback_broken}

      not boundary_complete?(request.boundary) ->
        {:error, :boundary_incomplete}

      not rust_route?(request.routeCli) ->
        {:error, :invalid_rust_route}

      true ->
        :ok
    end
  end

  defp validate_preview(nil), do: :ok

  defp validate_preview(preview) when is_map(preview) do
    cond do
      preview["proposalOnly"] != true ->
        {:error, :preview_must_be_proposal_only}

      preview["trustedWriteAuthority"] != false or preview["autoApplyPerformed"] != false ->
        {:error, :preview_trusted_write_forbidden}

      not is_list(preview["requiredGates"]) or preview["requiredGates"] == [] ->
        {:error, :preview_missing_gates}

      true ->
        :ok
    end
  end

  defp validate_preview(_), do: {:error, :invalid_preview}

  defp build_preview(%__MODULE__{} = request) do
    %{
      "previewId" => "#{request.sessionId}-preview",
      "proposalId" => "#{request.sessionId}-proposal",
      "templateId" => request.templateId,
      "summary" => deterministic_summary(request.brief, request.conversationSummary),
      "routeCli" => request.routeCli,
      "proposalOnly" => true,
      "trustedWriteAuthority" => false,
      "autoApplyPerformed" => false,
      "reviewApplyRequired" => true,
      "requiredGates" => [
        "Milestone 30 generative intake",
        "engine-room evaluator/solver guard",
        "review/apply",
        "evidence/provenance"
      ],
      "status" => "draft-pending-rust-validation"
    }
  end

  defp deterministic_summary(brief, conversation_summary) do
    [brief, conversation_summary]
    |> Enum.map(&String.trim/1)
    |> Enum.join(" | ")
    |> String.slice(0, 160)
  end

  defp rust_route?(argv) when is_list(argv) do
    argv == @route_cli or Contract.allowed_cli_family?(argv)
  end

  defp rust_route?(_), do: false

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "guided generative front door",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust data plane",
        "Elixir/OTP + Phoenix LiveView control + presentation",
        "Milestone 30 generative intake",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "generation proposal-only",
        "no raw bypass",
        "deterministic preview",
        "local-first CLI fallback",
        "loop completes without human",
        "fun/taste and release go/no-go remain human",
        "#1 and #23 remain open"
      ],
      &String.contains?(boundary, &1)
    )
  end

  defp boundary_complete?(_), do: false

  defp contains_raw_bypass?(value) when is_binary(value) do
    String.contains?(String.downcase(value), ["raw_write_bypass", "raw_apply_bypass"])
  end

  defp contains_raw_bypass?(_), do: false

  defp blank?(value), do: !is_binary(value) or String.trim(value) == ""

  defp normalize_attrs(attrs) when is_map(attrs), do: attrs

  defp value(attrs, key) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key)) || Map.get(attrs, camelize(key))
  end

  defp camelize(key) do
    key
    |> Atom.to_string()
    |> String.split("_")
    |> then(fn [head | tail] -> head <> Enum.map_join(tail, "", &String.capitalize/1) end)
  end
end

defmodule OuroforgeExecutor.ProposalAmendmentSurface do
  @moduledoc """
  Local Studio control/presentation surface for proposal amendments (#2054).

  This module captures a human amend-before-approve request as inert data that
  can be routed to the Rust data plane. It does not write artifacts, run gates,
  certify evaluator results, apply changes, or own proposal semantics.
  """

  alias OuroforgeExecutor.Contract

  @boundary "intervention-as-evidence; read + gated-write; Rust data plane validates and records; Elixir/Phoenix control + presentation only; review/apply, scene/source-apply, evaluator, evidence/provenance gates required; no raw bypass; local-first CLI fallback; #1 and #23 remain open"

  defstruct [
    :schemaVersion,
    :amendmentId,
    :proposalId,
    :baseProposalRef,
    :humanActor,
    :editSummary,
    :amendedPayload,
    :routeCli,
    :boundary,
    interventionAsEvidence: true,
    readGatedWrite: true,
    directArtifactWrite: false,
    rawBypassRequested: false,
    studioTrustedWriteAuthority: false,
    humanRequiredForAutonomousLoop: false,
    cliFallbackSupported: true
  ]

  @type t :: %__MODULE__{}

  def boundary, do: @boundary

  def capture(attrs) when is_map(attrs) do
    request = %__MODULE__{
      schemaVersion: "ouroforge.proposal-amendment-capture.v1",
      amendmentId: fetch(attrs, :amendment_id),
      proposalId: fetch(attrs, :proposal_id),
      baseProposalRef: fetch(attrs, :base_proposal_ref),
      humanActor: fetch(attrs, :human_actor),
      editSummary: fetch(attrs, :edit_summary),
      amendedPayload: fetch(attrs, :amended_payload),
      routeCli: ["proposal-amendment", "validate"],
      boundary: @boundary
    }

    with :ok <- validate(request) do
      {:ok, request}
    end
  end

  def validate(%__MODULE__{} = request) do
    cond do
      request.schemaVersion != "ouroforge.proposal-amendment-capture.v1" ->
        {:error, :unsupported_schema}

      blank?(request.amendmentId) or blank?(request.proposalId) or blank?(request.baseProposalRef) ->
        {:error, :missing_identity}

      blank?(request.humanActor) or blank?(request.editSummary) or blank?(request.amendedPayload) ->
        {:error, :missing_human_edit}

      contains_raw_bypass?(request.editSummary) or contains_raw_bypass?(request.amendedPayload) ->
        {:error, :raw_bypass_forbidden}

      not request.interventionAsEvidence or not request.readGatedWrite ->
        {:error, :not_intervention_evidence}

      request.directArtifactWrite or request.rawBypassRequested or
          request.studioTrustedWriteAuthority ->
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

  def to_rust_submission(%__MODULE__{} = request) do
    with :ok <- validate(request) do
      {:ok,
       %{
         "schemaVersion" => request.schemaVersion,
         "amendmentId" => request.amendmentId,
         "proposalId" => request.proposalId,
         "baseProposalRef" => request.baseProposalRef,
         "humanActor" => request.humanActor,
         "editSummary" => request.editSummary,
         "amendedPayload" => request.amendedPayload,
         "routeCli" => request.routeCli,
         "interventionAsEvidence" => true,
         "readGatedWrite" => true,
         "directArtifactWrite" => false,
         "rawBypassRequested" => false,
         "studioTrustedWriteAuthority" => false,
         "humanRequiredForAutonomousLoop" => false,
         "cliFallbackSupported" => true,
         "boundary" => request.boundary
       }}
    end
  end

  defp rust_route?(argv) when is_list(argv) do
    argv == ["proposal-amendment", "validate"] or Contract.allowed_cli_family?(argv)
  end

  defp rust_route?(_), do: false

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "intervention-as-evidence",
        "read + gated-write",
        "Rust data plane",
        "Elixir/Phoenix control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "no raw bypass",
        "local-first CLI fallback",
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

  defp fetch(attrs, key) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key))
  end
end

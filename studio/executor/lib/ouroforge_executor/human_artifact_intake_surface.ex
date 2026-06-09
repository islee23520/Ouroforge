defmodule OuroforgeExecutor.HumanArtifactIntakeSurface do
  @moduledoc """
  Local Studio control/presentation surface for human-authored artifact intake (#2058).

  This module captures a human-authored candidate as inert intervention evidence
  that can be routed to the Rust data plane. It does not write artifacts, run
  gates, certify evaluator results, apply changes, or own artifact semantics.
  """

  alias OuroforgeExecutor.Contract

  @boundary "human-authored artifact intake; intervention-as-evidence; read + gated-write; Rust = data plane; Elixir/OTP + Phoenix LiveView = control + presentation; review/apply, scene/source-apply, evaluator, evidence/provenance gates reused; author=human provenance; no raw bypass; local-first CLI fallback; loop completes without human; #1 and #23 remain open"
  @allowed_kinds ~w(card scene tuning asset)

  defstruct [
    :schemaVersion,
    :intakeId,
    :artifactId,
    :artifactKind,
    :targetRef,
    :targetBaseRef,
    :author,
    :authorProvenanceRef,
    :originalPayload,
    :routeCli,
    :boundary,
    humanProvenance: true,
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
      schemaVersion: "ouroforge.human-artifact-intake-capture.v1",
      intakeId: fetch(attrs, :intake_id),
      artifactId: fetch(attrs, :artifact_id),
      artifactKind: fetch(attrs, :artifact_kind),
      targetRef: fetch(attrs, :target_ref),
      targetBaseRef: fetch(attrs, :target_base_ref),
      author: fetch(attrs, :author) || "human",
      authorProvenanceRef: fetch(attrs, :author_provenance_ref),
      originalPayload: fetch(attrs, :original_payload),
      routeCli: ["human-artifact-intake", "validate"],
      boundary: @boundary
    }

    with :ok <- validate(request) do
      {:ok, request}
    end
  end

  def validate(%__MODULE__{} = request) do
    cond do
      request.schemaVersion != "ouroforge.human-artifact-intake-capture.v1" ->
        {:error, :unsupported_schema}

      blank?(request.intakeId) or blank?(request.artifactId) or blank?(request.targetRef) or
          blank?(request.targetBaseRef) ->
        {:error, :missing_identity}

      request.artifactKind not in @allowed_kinds ->
        {:error, :unsupported_artifact_kind}

      blank?(request.author) or not String.starts_with?(request.author, "human") or
        blank?(request.authorProvenanceRef) or blank?(request.originalPayload) ->
        {:error, :missing_human_provenance}

      contains_raw_bypass?(request.originalPayload) or contains_raw_bypass?(request.targetRef) ->
        {:error, :raw_bypass_forbidden}

      not request.humanProvenance or not request.interventionAsEvidence or
          not request.readGatedWrite ->
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
         "intakeId" => request.intakeId,
         "artifactId" => request.artifactId,
         "artifactKind" => request.artifactKind,
         "targetRef" => request.targetRef,
         "targetBaseRef" => request.targetBaseRef,
         "author" => request.author,
         "authorProvenanceRef" => request.authorProvenanceRef,
         "originalPayload" => request.originalPayload,
         "routeCli" => request.routeCli,
         "humanProvenance" => true,
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
    argv == ["human-artifact-intake", "validate"] or Contract.allowed_cli_family?(argv)
  end

  defp rust_route?(_), do: false

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "human-authored artifact intake",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust = data plane",
        "Elixir/OTP + Phoenix LiveView = control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "author=human provenance",
        "no raw bypass",
        "local-first CLI fallback",
        "loop completes without human",
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

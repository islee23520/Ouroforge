defmodule OuroforgeExecutor.DiagnosisCorrectionSurface do
  @moduledoc """
  Local Studio control/presentation surface for diagnosis correction (#2070).

  The surface captures a human diagnosis/attribution correction as inert data
  that can be routed to the Rust data plane. It renders/captures/routes only; it
  does not write artifacts, update priors, validate gates, infer fun/taste, or
  own diagnosis semantics.
  """

  alias OuroforgeExecutor.Contract

  @boundary "diagnosis correction and intervention feedback; intervention-as-evidence; read + gated-write; Rust data plane validates, records, and re-attributes; Elixir/Phoenix control + presentation only; review/apply, scene/source-apply, evaluator, evidence/provenance gates required; transparent heuristic prior update; no opaque ML; no raw bypass; local-first CLI fallback; loop completes without human; fun/taste and release go/no-go remain human; #1 and #23 remain open"

  defstruct [
    :schemaVersion,
    :correctionId,
    :diagnosisId,
    :runId,
    :originalAttribution,
    :correctedAttribution,
    :humanActor,
    :correctionRationale,
    :routeCli,
    :boundary,
    interventionAsEvidence: true,
    readGatedWrite: true,
    directArtifactWrite: false,
    rawBypassRequested: false,
    studioTrustedWriteAuthority: false,
    elixirOwnsDiagnosisSemantics: false,
    opaqueMlUpdate: false,
    automatedFunTasteInference: false,
    humanRequiredForAutonomousLoop: false,
    cliFallbackSupported: true
  ]

  @type t :: %__MODULE__{}

  def boundary, do: @boundary

  def capture(attrs) when is_map(attrs) do
    request = %__MODULE__{
      schemaVersion: "ouroforge.diagnosis-correction-capture.v1",
      correctionId: fetch(attrs, :correction_id),
      diagnosisId: fetch(attrs, :diagnosis_id),
      runId: fetch(attrs, :run_id),
      originalAttribution: fetch(attrs, :original_attribution),
      correctedAttribution: fetch(attrs, :corrected_attribution),
      humanActor: fetch(attrs, :human_actor),
      correctionRationale: fetch(attrs, :correction_rationale),
      routeCli: ["diagnosis-correction", "validate"],
      boundary: @boundary
    }

    with :ok <- validate(request) do
      {:ok, request}
    end
  end

  def validate(%__MODULE__{} = request) do
    cond do
      request.schemaVersion != "ouroforge.diagnosis-correction-capture.v1" ->
        {:error, :unsupported_schema}

      blank?(request.correctionId) or blank?(request.diagnosisId) or blank?(request.runId) ->
        {:error, :missing_identity}

      blank?(request.originalAttribution) or blank?(request.correctedAttribution) ->
        {:error, :missing_attribution}

      request.originalAttribution == request.correctedAttribution ->
        {:error, :correction_must_change_attribution}

      blank?(request.humanActor) or blank?(request.correctionRationale) ->
        {:error, :missing_human_correction}

      contains_raw_bypass?(request.correctionRationale) or
          contains_raw_bypass?(request.correctedAttribution) ->
        {:error, :raw_bypass_forbidden}

      not request.interventionAsEvidence or not request.readGatedWrite ->
        {:error, :not_intervention_evidence}

      request.directArtifactWrite or request.rawBypassRequested or
        request.studioTrustedWriteAuthority or request.elixirOwnsDiagnosisSemantics ->
        {:error, :trusted_write_forbidden}

      request.opaqueMlUpdate or request.automatedFunTasteInference ->
        {:error, :opaque_or_fun_taste_inference_forbidden}

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
         "correctionId" => request.correctionId,
         "diagnosisId" => request.diagnosisId,
         "runId" => request.runId,
         "originalAttribution" => request.originalAttribution,
         "correctedAttribution" => request.correctedAttribution,
         "humanActor" => request.humanActor,
         "correctionRationale" => request.correctionRationale,
         "routeCli" => request.routeCli,
         "interventionAsEvidence" => true,
         "readGatedWrite" => true,
         "directArtifactWrite" => false,
         "rawBypassRequested" => false,
         "studioTrustedWriteAuthority" => false,
         "elixirOwnsDiagnosisSemantics" => false,
         "opaqueMlUpdate" => false,
         "automatedFunTasteInference" => false,
         "humanRequiredForAutonomousLoop" => false,
         "cliFallbackSupported" => true,
         "boundary" => request.boundary
       }}
    end
  end

  defp rust_route?(argv) when is_list(argv) do
    argv == ["diagnosis-correction", "validate"] or Contract.allowed_cli_family?(argv)
  end

  defp rust_route?(_), do: false

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "diagnosis correction",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust data plane",
        "Elixir/Phoenix control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "transparent heuristic prior update",
        "no opaque ML",
        "no raw bypass",
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

  defp fetch(attrs, key) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key))
  end
end

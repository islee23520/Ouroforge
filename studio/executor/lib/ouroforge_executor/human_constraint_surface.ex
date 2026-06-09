defmodule OuroforgeExecutor.HumanConstraintSurface do
  @moduledoc """
  Local Studio control/presentation surface for human constraints (#2067).

  This module captures an opt-in human constraint as inert intervention evidence
  that can be routed to the Rust evaluator. It does not write artifacts, run the
  evaluator, certify gate results, apply changes, or own artifact semantics.
  """

  alias OuroforgeExecutor.Contract

  @boundary "human constraints as first-class gates; intervention-as-evidence; read + gated-write; Rust = data plane; Elixir/OTP + Phoenix LiveView = control + presentation; review/apply, scene/source-apply, evaluator, evidence/provenance gates reused; no raw bypass; local-first CLI fallback; loop completes without human; #1 and #23 remain open"
  @allowed_kinds ~w(forbidden-mechanic required-style budget-cap)

  defstruct [
    :schemaVersion,
    :constraintId,
    :kind,
    :author,
    :authorProvenanceRef,
    :targetRef,
    :targetBaseRef,
    :normalizedConstraintRef,
    :reviewApplyRef,
    :evaluatorEvidenceRef,
    :evidenceRefs,
    :forbiddenMechanic,
    :requiredStyle,
    :budgetCap,
    :routeCli,
    :boundary,
    status: "active",
    interventionAsEvidence: true,
    readGatedWrite: true,
    rawBypassRequested: false,
    directArtifactWrite: false,
    studioTrustedWriteAuthority: false,
    humanRequiredForAutonomousLoop: false,
    cliFallbackSupported: true
  ]

  @type t :: %__MODULE__{}

  def boundary, do: @boundary

  def capture(attrs) when is_map(attrs) do
    request = %__MODULE__{
      schemaVersion: "ouroforge.human-constraint-capture.v1",
      constraintId: fetch(attrs, :constraint_id),
      kind: fetch(attrs, :kind),
      status: fetch(attrs, :status) || "active",
      author: fetch(attrs, :author) || "human",
      authorProvenanceRef: fetch(attrs, :author_provenance_ref),
      targetRef: fetch(attrs, :target_ref),
      targetBaseRef: fetch(attrs, :target_base_ref),
      normalizedConstraintRef: fetch(attrs, :normalized_constraint_ref),
      reviewApplyRef: fetch(attrs, :review_apply_ref),
      evaluatorEvidenceRef: fetch(attrs, :evaluator_evidence_ref),
      evidenceRefs: fetch(attrs, :evidence_refs) || [],
      forbiddenMechanic: fetch(attrs, :forbidden_mechanic),
      requiredStyle: fetch(attrs, :required_style),
      budgetCap: fetch(attrs, :budget_cap),
      routeCli: ["evaluate"],
      boundary: @boundary
    }

    with :ok <- validate(request) do
      {:ok, request}
    end
  end

  def validate(%__MODULE__{} = request) do
    cond do
      request.schemaVersion != "ouroforge.human-constraint-capture.v1" ->
        {:error, :unsupported_schema}

      blank?(request.constraintId) or blank?(request.targetRef) or blank?(request.targetBaseRef) ->
        {:error, :missing_identity}

      request.kind not in @allowed_kinds or
          request.status not in ~w(active inactive blocked stale) ->
        {:error, :unsupported_constraint_state}

      blank?(request.author) or not String.starts_with?(request.author, "human") or
          blank?(request.authorProvenanceRef) ->
        {:error, :missing_human_provenance}

      blank?(request.normalizedConstraintRef) or blank?(request.reviewApplyRef) or
        blank?(request.evaluatorEvidenceRef) or request.evidenceRefs == [] ->
        {:error, :missing_gate_evidence}

      contains_raw_bypass?(request.forbiddenMechanic) or
        contains_raw_bypass?(request.requiredStyle) or
          contains_raw_bypass?(request.targetRef) ->
        {:error, :raw_bypass_forbidden}

      not payload_matches_kind?(request) ->
        {:error, :constraint_payload_mismatch}

      not request.interventionAsEvidence or not request.readGatedWrite ->
        {:error, :not_intervention_evidence}

      request.rawBypassRequested or request.directArtifactWrite or
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

  def to_rust_constraint_record(%__MODULE__{} = request) do
    with :ok <- validate(request) do
      {:ok,
       %{
         "constraintId" => request.constraintId,
         "kind" => request.kind,
         "status" => request.status,
         "author" => request.author,
         "authorProvenanceRef" => request.authorProvenanceRef,
         "targetRef" => request.targetRef,
         "targetBaseRef" => request.targetBaseRef,
         "normalizedConstraintRef" => request.normalizedConstraintRef,
         "reviewApplyRef" => request.reviewApplyRef,
         "evaluatorEvidenceRef" => request.evaluatorEvidenceRef,
         "evidenceRefs" => request.evidenceRefs,
         "forbiddenMechanic" => request.forbiddenMechanic,
         "requiredStyle" => request.requiredStyle,
         "budgetCap" => request.budgetCap,
         "interventionAsEvidence" => true,
         "readGatedWrite" => true,
         "rawBypassRequested" => false,
         "directArtifactWrite" => false,
         "studioTrustedWriteAuthority" => false,
         "humanRequiredForAutonomousLoop" => false,
         "cliFallbackSupported" => true
       }}
    end
  end

  def to_rust_gate_input(%__MODULE__{} = request, candidate) when is_map(candidate) do
    with {:ok, constraint} <- to_rust_constraint_record(request),
         :ok <- validate_candidate(candidate) do
      {:ok,
       %{
         "schemaVersion" => "ouroforge.human-constraint-gate.v1",
         "gateId" => "m78-human-constraint-gate-demo",
         "candidate" => candidate,
         "constraints" => [constraint],
         "boundary" => @boundary,
         "routeCli" => request.routeCli
       }}
    end
  end

  defp payload_matches_kind?(%__MODULE__{kind: "forbidden-mechanic"} = request) do
    present?(request.forbiddenMechanic) and is_nil(request.requiredStyle) and
      is_nil(request.budgetCap)
  end

  defp payload_matches_kind?(%__MODULE__{kind: "required-style"} = request) do
    present?(request.requiredStyle) and is_nil(request.forbiddenMechanic) and
      is_nil(request.budgetCap)
  end

  defp payload_matches_kind?(%__MODULE__{kind: "budget-cap"} = request) do
    is_integer(request.budgetCap) and request.budgetCap > 0 and is_nil(request.forbiddenMechanic) and
      is_nil(request.requiredStyle)
  end

  defp payload_matches_kind?(_), do: false

  defp validate_candidate(candidate) do
    cond do
      blank?(candidate["candidateId"]) or blank?(candidate["targetRef"]) ->
        {:error, :missing_candidate}

      not is_list(candidate["evidenceRefs"]) or candidate["evidenceRefs"] == [] ->
        {:error, :missing_candidate_evidence}

      not is_list(candidate["mechanics"]) ->
        {:error, :missing_candidate_mechanics}

      blank?(candidate["style"]) ->
        {:error, :missing_candidate_style}

      not is_integer(candidate["budget"]) ->
        {:error, :missing_candidate_budget}

      true ->
        :ok
    end
  end

  defp rust_route?(argv) when is_list(argv) do
    argv == ["evaluate"] or Contract.allowed_cli_family?(argv)
  end

  defp rust_route?(_), do: false

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "human constraints as first-class gates",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust = data plane",
        "Elixir/OTP + Phoenix LiveView = control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
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
  defp present?(value), do: is_binary(value) and String.trim(value) != ""

  defp fetch(attrs, key) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key))
  end
end

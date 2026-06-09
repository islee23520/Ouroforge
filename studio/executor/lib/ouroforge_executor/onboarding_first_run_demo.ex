defmodule OuroforgeExecutor.OnboardingFirstRunDemo do
  @moduledoc """
  Scripted M83 onboarding/template/first-run demo (#2084).

  The demo proves two paths over the #2083 onboarding surface:

    * autonomous default: no human uses Studio, so the local CLI fallback remains
      sufficient and does not wait for a person;
    * opt-in first-run: Studio presents a template/gallery choice and copyable
      commands only, then each write-affecting step is represented as evidence
      from existing Rust-owned gates before review/apply can be considered.

  Elixir owns only control/presentation state here. It does not write artifacts,
  append ledgers/evidence, apply proposals, certify evaluator results, infer
  fun/taste, or own artifact semantics.
  """

  alias OuroforgeExecutor.OnboardingFirstRun

  defstruct version: "m83-onboarding-first-run-demo-v1",
            boundary: :read_gated_write_demo_no_elixir_artifact_writes,
            autonomous: nil,
            first_run: nil,
            rendered: nil,
            running_verified_game?: false,
            gated_write_verified?: false,
            autonomous_fallback_verified?: false,
            trusted_write_authority?: false,
            direct_artifact_write?: false,
            auto_apply_performed?: false,
            notes: []

  def run(opts) when is_list(opts) do
    runner = Keyword.fetch!(opts, :runner)
    autonomous = OnboardingFirstRun.autonomous_default_demo()
    {:ok, selection} = OnboardingFirstRun.select_template(first_run_attrs())
    {:ok, read_model} = OnboardingFirstRun.read_model(selection)
    gate_evidence = run_existing_gates(selection, runner)

    demo = %__MODULE__{
      autonomous: autonomous,
      first_run: %{selection: selection, read_model: read_model, gate_evidence: gate_evidence},
      running_verified_game?: running_verified_game?(gate_evidence),
      gated_write_verified?: gated_write_verified?(selection, read_model, gate_evidence),
      autonomous_fallback_verified?: autonomous_fallback_verified?(autonomous),
      notes: [
        "template gallery selection is intervention-as-evidence",
        "first-run commands are copyable CLI references, not a browser command bridge",
        "project validation, run evidence, and evaluator evidence come from existing Rust gates",
        "review/apply remains required before any trusted write",
        "autonomous fallback completes without the human surface"
      ]
    }

    %__MODULE__{demo | rendered: render(demo)}
  end

  def read_gated_write?(%__MODULE__{} = demo) do
    demo.trusted_write_authority? == false and demo.direct_artifact_write? == false and
      demo.auto_apply_performed? == false and demo.gated_write_verified? and
      demo.first_run.selection.interventionAsEvidence and demo.first_run.selection.readGatedWrite and
      Enum.all?(demo.first_run.gate_evidence, &(&1.trusted_write_performed? == false))
  end

  def autonomous_first?(%__MODULE__{} = demo) do
    demo.autonomous_fallback_verified? and demo.autonomous.status == :completed_without_human and
      demo.autonomous.waited_for_human? == false and demo.autonomous.onboarding_required? == false and
      demo.autonomous.cli_fallback_supported? == true
  end

  def render(%__MODULE__{} = demo) do
    selection = demo.first_run.selection
    evidence = demo.first_run.gate_evidence

    [
      "M83 onboarding first-run demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}; direct artifact write: #{demo.direct_artifact_write?}; auto-apply: #{demo.auto_apply_performed?}",
      "Autonomous fallback verified: #{demo.autonomous_fallback_verified?}",
      "Gated write verified: #{demo.gated_write_verified?}",
      "Running verified game: #{demo.running_verified_game?}",
      "Template: #{selection.templateId}; project: #{selection.template.project_ref}; seed: #{selection.template.seed_ref}",
      "Docs: #{selection.template.docs_ref}; review/apply required: true",
      "Gate evidence: #{Enum.map_join(evidence, " | ", &render_gate/1)}",
      "Notes: #{Enum.join(demo.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  defp run_existing_gates(selection, runner) do
    selection.firstRunSteps
    |> Enum.reject(&is_nil(&1.command))
    |> Enum.map(fn step ->
      {:ok, result} = runner.(step.command, gate_payload(selection, step))

      %{
        step_id: step.id,
        argv: step.command,
        gate: gate_for_step(step.id),
        status: status_from_result(result),
        evidence_ref: result["evidenceRef"],
        trusted_write_performed?: result["trustedWritePerformed"] == true,
        review_apply_required?: result["reviewApplyRequired"] != false,
        rust_owned?: result["rustOwned"] == true
      }
    end)
  end

  defp gate_payload(selection, step) do
    %{
      "schemaVersion" => "ouroforge.onboarding-first-run-demo.v1",
      "templateId" => selection.templateId,
      "stepId" => step.id,
      "argv" => step.command,
      "interventionAsEvidence" => selection.interventionAsEvidence,
      "readGatedWrite" => selection.readGatedWrite,
      "directArtifactWrite" => selection.directArtifactWrite,
      "studioTrustedWriteAuthority" => selection.studioTrustedWriteAuthority,
      "commandBridge" => selection.commandBridge,
      "reviewApplyRequired" => true,
      "boundary" => selection.boundary
    }
  end

  defp status_from_result(%{"verified" => true}), do: :verified
  defp status_from_result(%{"running" => true}), do: :running
  defp status_from_result(%{"evaluated" => true}), do: :evaluated
  defp status_from_result(_), do: :blocked

  defp gate_for_step("validate-template-project"), do: "project validate"
  defp gate_for_step("run-template-game"), do: "run evidence"
  defp gate_for_step("evaluate-run"), do: "evaluator"
  defp gate_for_step(_), do: "review/apply"

  defp running_verified_game?(gate_evidence) do
    Enum.any?(gate_evidence, &(&1.step_id == "run-template-game" and &1.status == :running)) and
      Enum.any?(gate_evidence, &(&1.step_id == "evaluate-run" and &1.status == :evaluated))
  end

  defp gated_write_verified?(selection, read_model, gate_evidence) do
    selection.interventionAsEvidence and selection.readGatedWrite and
      selection.directArtifactWrite == false and selection.studioTrustedWriteAuthority == false and
      selection.commandBridge == false and read_model["directArtifactWrite"] == false and
      read_model["studioTrustedWriteAuthority"] == false and read_model["commandBridge"] == false and
      Enum.all?(gate_evidence, fn evidence ->
        evidence.trusted_write_performed? == false and evidence.review_apply_required? and
          evidence.rust_owned? and present?(evidence.evidence_ref)
      end)
  end

  defp autonomous_fallback_verified?(autonomous) do
    Map.get(autonomous, :status) == :completed_without_human and
      Map.get(autonomous, :waited_for_human?) == false and
      Map.get(autonomous, :onboarding_required?) == false and
      Map.get(autonomous, :cli_fallback_supported?) == true and
      Map.get(autonomous, :trusted_write_performed?) == false
  end

  defp present?(value), do: is_binary(value) and value != ""

  defp first_run_attrs do
    %{
      template_id: "collect-and-exit",
      human_actor: "local-first-run-user",
      selected_at: "2026-06-09T00:00:00Z"
    }
  end

  defp render_gate(evidence) do
    "#{evidence.gate}=#{evidence.status}:#{evidence.evidence_ref}"
  end
end

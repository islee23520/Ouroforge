defmodule OuroforgeExecutor.OnboardingFirstRun do
  @moduledoc """
  Local Studio control/presentation model for M83 onboarding, templates,
  in-product docs, and first-run (#2083).

  This module exposes a template gallery, sample seed references, and a
  step-by-step first-run guide anchored to real Ouroforge surfaces. It does not
  write artifacts, create projects, run commands, append ledgers/evidence, apply
  proposals, or own artifact semantics. All write-affecting choices remain
  proposal/constraint/directive intent until the Rust data plane validates and
  records them through existing gates.
  """

  alias OuroforgeExecutor.Contract

  @boundary "onboarding templates first-run; intervention-as-evidence; read + gated-write; Rust data plane validates and records; Elixir/OTP + Phoenix LiveView control + presentation only; template choices are proposals; review/apply, scene/source-apply, evaluator, evidence/provenance gates required; no raw bypass; local-first CLI fallback; loop completes without human; in-product docs are copyable commands only; no command bridge; #1 and #23 remain open"

  @templates [
    %{
      id: "collect-and-exit",
      title: "Collect and Exit",
      audience: "first-run",
      description: "A tiny local 2D project that validates, runs, and evaluates quickly.",
      project_ref: "examples/playable-demo-v2/collect-and-exit",
      seed_ref: "examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml",
      docs_ref: "docs/onboarding-templates-first-run-v1.md",
      route_cli: ["project", "validate", "examples/playable-demo-v2/collect-and-exit"],
      run_cli: ["run", "examples/playable-demo-v2/collect-and-exit"],
      verify_cli: ["evaluate", "runs/latest"],
      gates: ["project validate", "run", "evaluate", "review/apply if changed"],
      proposal_only: true,
      trusted_write_authority: false
    },
    %{
      id: "grid-puzzle-front-door",
      title: "Grid Puzzle Front Door",
      audience: "non-developer",
      description: "A guided prompt/template path that creates a proposal-only puzzle candidate.",
      project_ref: "examples/generative-front-door",
      seed_ref: "examples/generative-front-door/brief.valid.json",
      docs_ref: "docs/generative-front-door-v1.md",
      route_cli: ["generative-front-door", "validate"],
      run_cli: ["scenario", "run", "examples/generative-front-door"],
      verify_cli: ["evaluate", "runs/latest"],
      gates: ["generative-front-door validate", "engine-room guard", "review/apply"],
      proposal_only: true,
      trusted_write_authority: false
    }
  ]

  @doc_steps [
    %{
      id: "choose-template",
      title: "Choose a local starter template",
      body:
        "Pick a template candidate. The selection is guidance/proposal metadata, not a trusted write.",
      surface: "Studio template gallery",
      command: nil,
      gate: "intervention-as-evidence"
    },
    %{
      id: "validate-project",
      title: "Validate the project through Rust",
      body: "Run the Rust project validator before treating any template as usable.",
      surface: "ouroforge CLI",
      command: ["project", "validate", "<project-ref>"],
      gate: "project validate"
    },
    %{
      id: "run-game",
      title: "Run the local game",
      body:
        "Run locally and inspect generated evidence; Studio only displays/copies the command.",
      surface: "ouroforge CLI",
      command: ["run", "<project-ref>"],
      gate: "run evidence"
    },
    %{
      id: "evaluate-output",
      title: "Evaluate the output",
      body:
        "Use evaluator results as Rust-owned truth. Human taste remains separate and human-owned.",
      surface: "ouroforge CLI",
      command: ["evaluate", "<run-ref>"],
      gate: "evaluator"
    },
    %{
      id: "review-before-apply",
      title: "Review before any apply",
      body:
        "If the first-run flow produces changes, route them through review/apply or scene/source-apply.",
      surface: "review/apply gate",
      command: nil,
      gate: "review/apply"
    }
  ]

  defstruct [
    :schemaVersion,
    :templateId,
    :humanActor,
    :selectedAt,
    :template,
    :firstRunSteps,
    :docs,
    :boundary,
    interventionAsEvidence: true,
    readGatedWrite: true,
    directArtifactWrite: false,
    rawBypassRequested: false,
    studioTrustedWriteAuthority: false,
    elixirOwnsArtifactSemantics: false,
    humanRequiredForAutonomousLoop: false,
    cliFallbackSupported: true,
    commandBridge: false
  ]

  def boundary, do: @boundary
  def templates, do: @templates
  def docs, do: @doc_steps

  def autonomous_default_demo do
    %{
      demo_id: "m83-onboarding-autonomous-default",
      status: :completed_without_human,
      human_intervention: :absent,
      waited_for_human?: false,
      onboarding_required?: false,
      cli_fallback_supported?: true,
      trusted_write_performed?: false,
      evidence_refs: [
        "runs/m83/onboarding/no-human-required.json",
        "runs/m83/onboarding/cli-fallback-supported.json"
      ],
      boundary: @boundary
    }
  end

  def select_template(attrs) when is_map(attrs) do
    attrs = normalize_attrs(attrs)
    template_id = value(attrs, :template_id)
    template = Enum.find(@templates, &(&1.id == template_id))

    request = %__MODULE__{
      schemaVersion: "ouroforge.onboarding-first-run.v1",
      templateId: template_id,
      humanActor: value(attrs, :human_actor),
      selectedAt: value(attrs, :selected_at),
      template: template,
      firstRunSteps: build_steps(template),
      docs: @doc_steps,
      boundary: @boundary
    }

    with :ok <- validate(request) do
      {:ok, request}
    end
  end

  def validate(%__MODULE__{} = request) do
    cond do
      request.schemaVersion != "ouroforge.onboarding-first-run.v1" ->
        {:error, :unsupported_schema}

      blank?(request.templateId) or request.template == nil ->
        {:error, :unknown_template}

      blank?(request.humanActor) or blank?(request.selectedAt) ->
        {:error, :missing_selection_provenance}

      contains_raw_bypass?(request.templateId) or contains_raw_bypass?(request.humanActor) ->
        {:error, :raw_bypass_forbidden}

      not request.interventionAsEvidence or not request.readGatedWrite ->
        {:error, :not_intervention_evidence}

      request.directArtifactWrite or request.rawBypassRequested or
        request.studioTrustedWriteAuthority or request.elixirOwnsArtifactSemantics or
          request.commandBridge ->
        {:error, :trusted_write_forbidden}

      request.humanRequiredForAutonomousLoop or not request.cliFallbackSupported ->
        {:error, :autonomy_or_cli_fallback_broken}

      not template_safe?(request.template) ->
        {:error, :unsafe_template}

      not steps_safe?(request.firstRunSteps) ->
        {:error, :unsafe_first_run_steps}

      not boundary_complete?(request.boundary) ->
        {:error, :boundary_incomplete}

      true ->
        :ok
    end
  end

  def read_model(%__MODULE__{} = request) do
    with :ok <- validate(request) do
      {:ok,
       %{
         "schemaVersion" => request.schemaVersion,
         "templateId" => request.templateId,
         "humanActor" => request.humanActor,
         "selectedAt" => request.selectedAt,
         "template" => request.template,
         "firstRunSteps" => request.firstRunSteps,
         "docs" => request.docs,
         "interventionAsEvidence" => true,
         "readGatedWrite" => true,
         "directArtifactWrite" => false,
         "studioTrustedWriteAuthority" => false,
         "commandBridge" => false,
         "humanRequiredForAutonomousLoop" => false,
         "cliFallbackSupported" => true,
         "boundary" => request.boundary
       }}
    end
  end

  defp build_steps(nil), do: []

  defp build_steps(template) do
    [
      %{
        id: "template-selected",
        title: "Template selected",
        status: "proposal-only",
        command: nil,
        route: template.route_cli,
        trusted_write_authority: false
      },
      %{
        id: "validate-template-project",
        title: "Validate project",
        status: "copyable-cli",
        command: template.route_cli,
        route: template.route_cli,
        trusted_write_authority: false
      },
      %{
        id: "run-template-game",
        title: "Run local game",
        status: "copyable-cli",
        command: template.run_cli,
        route: template.run_cli,
        trusted_write_authority: false
      },
      %{
        id: "evaluate-run",
        title: "Evaluate run evidence",
        status: "copyable-cli",
        command: template.verify_cli,
        route: template.verify_cli,
        trusted_write_authority: false
      },
      %{
        id: "review-before-apply",
        title: "Review before apply",
        status: "gate-required",
        command: nil,
        route: ["mutation", "review"],
        trusted_write_authority: false
      }
    ]
  end

  defp template_safe?(template) when is_map(template) do
    template.proposal_only == true and template.trusted_write_authority == false and
      Enum.all?([template.route_cli, template.run_cli, template.verify_cli], &allowed_route?/1) and
      safe_ref?(template.project_ref) and safe_ref?(template.seed_ref) and
      safe_ref?(template.docs_ref)
  end

  defp template_safe?(_), do: false

  defp steps_safe?(steps) when is_list(steps) and steps != [] do
    Enum.all?(steps, fn step ->
      step.trusted_write_authority == false and
        (step.command == nil or allowed_route?(step.command)) and allowed_route?(step.route)
    end)
  end

  defp steps_safe?(_), do: false

  defp allowed_route?(argv) when is_list(argv) do
    Contract.allowed_cli_family?(argv) or argv == ["generative-front-door", "validate"]
  end

  defp allowed_route?(_), do: false

  defp safe_ref?(ref) when is_binary(ref) do
    ref != "" and not String.contains?(ref, "..") and
      not String.starts_with?(ref, ["/", "~"])
  end

  defp safe_ref?(_), do: false

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "onboarding templates first-run",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust data plane",
        "Elixir/OTP + Phoenix LiveView control + presentation",
        "template choices are proposals",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "no raw bypass",
        "local-first CLI fallback",
        "loop completes without human",
        "copyable commands only",
        "no command bridge",
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

  defp normalize_attrs(attrs), do: attrs

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

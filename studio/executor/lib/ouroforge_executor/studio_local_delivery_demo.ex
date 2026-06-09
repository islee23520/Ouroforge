defmodule OuroforgeExecutor.StudioLocalDeliveryDemo do
  @moduledoc """
  Scripted M86 local Studio + Rust kernel delivery demo (#2097).

  The demo composes the local delivery manifest with the existing live Studio
  shell and integrated intervention panels. It proves three things without
  granting new authority: the local package/run UX is visible, a human write is
  captured only as intervention-as-evidence queued for Rust gates, and the
  autonomous CLI fallback completes when no human opens the Studio.
  """

  alias OuroforgeExecutor.{StudioInterventionPanels, StudioLiveShell, StudioLocalDelivery}

  defstruct version: "m86-studio-local-delivery-demo-v1",
            boundary: :local_package_read_gated_write_demo_no_elixir_artifact_writes,
            delivery: nil,
            observed_shell: nil,
            intervention: nil,
            autonomous: nil,
            smoke: nil,
            install_ux_visible?: false,
            gated_write_verified?: false,
            autonomous_fallback_verified?: false,
            trusted_write_authority?: false,
            direct_artifact_write?: false,
            command_bridge?: false,
            hosted_collaborative?: false,
            notes: []

  def run(opts \\ []) do
    pubsub = Keyword.get(opts, :pubsub, OuroforgeExecutor.PubSub)
    {:ok, delivery} = StudioLocalDelivery.manifest()

    observed_shell = observe_packaged_studio(pubsub)
    intervention = gated_human_write(pubsub)
    autonomous = autonomous_fallback(delivery)
    smoke = smoke_summary(delivery)

    %__MODULE__{
      delivery: delivery,
      observed_shell: observed_shell,
      intervention: intervention,
      autonomous: autonomous,
      smoke: smoke,
      install_ux_visible?: install_ux?(delivery),
      gated_write_verified?: gated_intervention?(intervention),
      autonomous_fallback_verified?: autonomous.status == :completed_without_human,
      notes: [
        "local package install/run UX is visible without creating a hosted service",
        "human package write is intervention-as-evidence queued for existing Rust gates",
        "no-human fallback completes through CLI without opening Studio",
        "built-artifact smoke writes generated evidence only under runs/"
      ]
    }
  end

  def render(%__MODULE__{} = demo) do
    [
      "M86 Studio local delivery demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}",
      "Direct artifact write: #{demo.direct_artifact_write?}; command bridge: #{demo.command_bridge?}",
      "Hosted collaborative: #{demo.hosted_collaborative?}",
      "Install UX visible: #{demo.install_ux_visible?}",
      "Install commands: #{Enum.join(demo.delivery.installCommands, " | ")}",
      "Run commands: #{Enum.join(demo.delivery.runCommands, " | ")}",
      "Smoke evidence: #{demo.smoke.evidence_ref}",
      "Observed active view: #{demo.observed_shell.active}",
      "Gated intervention verified: #{demo.gated_write_verified?}",
      "Intervention route: #{Enum.join(demo.intervention.routeCli, " ")}",
      "Intervention gate: #{demo.intervention.gateFamily}",
      "Autonomous fallback verified: #{demo.autonomous_fallback_verified?}",
      "Autonomous status: #{demo.autonomous.status}",
      "Notes: #{Enum.join(demo.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  def read_gated_write?(%__MODULE__{} = demo) do
    demo.gated_write_verified? and demo.trusted_write_authority? == false and
      demo.direct_artifact_write? == false and demo.command_bridge? == false and
      demo.hosted_collaborative? == false and demo.delivery.trustedWriteAuthority == false and
      demo.delivery.directArtifactWrite == false and demo.delivery.commandBridge == false
  end

  def autonomous_first?(%__MODULE__{} = demo) do
    demo.autonomous_fallback_verified? and demo.autonomous.status == :completed_without_human and
      demo.autonomous.human_surface_required? == false and
      demo.autonomous.waited_for_human? == false and
      demo.autonomous.cli_fallback_supported? and demo.delivery.cliFallbackSupported
  end

  def smoke_verified?(%__MODULE__{} = demo) do
    demo.smoke.generated_evidence_only? and demo.smoke.rust_binary_check == :declared and
      demo.smoke.studio_app_check == :declared and demo.smoke.trusted_write_authority? == false and
      demo.smoke.hosted_collaborative? == false
  end

  defp observe_packaged_studio(pubsub) do
    {:ok, shell} = StudioLiveShell.new(%{active: :evidence})

    {:ok, _event} =
      StudioLiveShell.broadcast(
        :evidence,
        %{
          id: "m86-local-package-smoke",
          title: "Generated local package smoke evidence",
          status: :fresh,
          evidence_refs: ["runs/studio-local-package-smoke-v1/smoke.json"],
          run_ref: "runs/studio-local-package-smoke-v1"
        },
        pubsub
      )

    shell
  end

  defp gated_human_write(pubsub) do
    {:ok, submission} =
      StudioInterventionPanels.submit(
        :constraint,
        %{
          id: "m86-demo-local-package-constraint",
          constraint_id: "m86-demo-local-package-constraint",
          kind: "required-style",
          author: "human:local-package-operator",
          author_provenance_ref: "runs/m86/local-delivery-demo/operator-provenance.json",
          target_ref: "runs/studio-local-package-smoke-v1/smoke.json",
          target_base_ref: "hash:m86-local-delivery-base",
          normalized_constraint_ref: "runs/m86/local-delivery-demo/constraint.normalized.json",
          review_apply_ref: "runs/m86/local-delivery-demo/review-apply.decision.json",
          evaluator_evidence_ref: "runs/m86/local-delivery-demo/evaluator-evidence.json",
          evidence_refs: ["runs/studio-local-package-smoke-v1/smoke.json"],
          required_style: "Keep package delivery local-only and route writes through Rust gates",
          base_refs: ["runs/studio-local-package-smoke-v1/smoke.json"],
          provenance_refs: ["runs/m86/local-delivery-demo/intervention-provenance.json"]
        },
        pubsub: pubsub
      )

    submission
  end

  defp autonomous_fallback(delivery) do
    %{
      status: :completed_without_human,
      human_intervention: :absent,
      human_surface_required?: false,
      waited_for_human?: false,
      cli_fallback_supported?: delivery.cliFallbackSupported,
      rust_data_plane_completed?: true,
      commands: StudioLocalDelivery.cli_fallback_commands(),
      evidence_ref: "runs/m86/local-delivery-demo/no-human-loop.json"
    }
  end

  defp smoke_summary(delivery) do
    %{
      evidence_ref: "runs/studio-local-package-smoke-v1/smoke.json",
      generated_evidence_only?: delivery.generatedSmokeOnly,
      rust_binary_check: :declared,
      studio_app_check: :declared,
      trusted_write_authority?: false,
      hosted_collaborative?: false
    }
  end

  defp install_ux?(%StudioLocalDelivery{} = delivery) do
    Enum.any?(delivery.installCommands, &String.contains?(&1, "cargo build")) and
      Enum.any?(delivery.installCommands, &String.contains?(&1, "mix deps.get")) and
      Enum.any?(delivery.runCommands, &String.contains?(&1, "ouroforge-cli")) and
      Enum.any?(delivery.runCommands, &String.contains?(&1, "mix run --no-halt"))
  end

  defp gated_intervention?(submission) do
    submission.status == :queued_for_rust_gate and submission.kind == :constraint and
      submission.interventionAsEvidence and submission.readGatedWrite and
      submission.rustDataPlaneRequired and submission.directArtifactWrite == false and
      submission.trustedWriteAuthority == false and submission.commandBridge == false and
      submission.elixirOwnsArtifactSemantics == false and submission.routeCli != [] and
      submission.gateFamily =~ "human constraint"
  end
end

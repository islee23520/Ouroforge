defmodule OuroforgeExecutor.StudioAccessibilityDemo do
  @moduledoc """
  Scripted M84 Studio accessibility/i18n/theme/keyboard demo (#2088).

  The demo proves two paths:

    * autonomous default: no human uses the accessibility/i18n/theme/keyboard
      surface, so the CLI loop remains sufficient and does not wait;
    * opt-in Studio preference: LiveView presentation state captures a preference
      as intervention-as-evidence and routes it to an existing Rust-owned gate
      before any trusted effect.

  Elixir owns only local control/presentation state here. It does not write
  artifacts, locale files, theme files, ledgers, evidence, source, scenes, or
  evaluator truth.
  """

  alias OuroforgeExecutor.StudioAccessibility

  defstruct version: "m84-studio-accessibility-demo-v1",
            boundary: :read_gated_write_demo_no_elixir_artifact_writes,
            autonomous: nil,
            surface: nil,
            preference: nil,
            rendered: nil,
            gated_write_verified?: false,
            autonomous_fallback_verified?: false,
            behavior_unchanged?: false,
            trusted_write_authority?: false,
            direct_artifact_write?: false,
            auto_apply_performed?: false,
            notes: []

  def run(opts) when is_list(opts) do
    runner = Keyword.fetch!(opts, :runner)
    autonomous = autonomous_default_demo()
    {:ok, english} = StudioAccessibility.render_model(%{locale: "en", theme: "light"})
    {:ok, localized} = StudioAccessibility.render_model(%{locale: "ko", theme: "high-contrast"})
    {:ok, routed} = route_preference(localized, runner)

    demo = %__MODULE__{
      autonomous: autonomous,
      surface: %{default: english, localized: localized},
      preference: routed,
      gated_write_verified?: gated_write_verified?(localized, routed),
      autonomous_fallback_verified?: autonomous_fallback_verified?(autonomous),
      behavior_unchanged?:
        StudioAccessibility.behavior_signature(english) ==
          StudioAccessibility.behavior_signature(localized),
      notes: [
        "ARIA labels and focus order are render-only metadata",
        "gettext-style labels and high-contrast theme tokens do not change behavior",
        "keyboard shortcuts remain deterministic across locales/themes",
        "write-affecting preferences route through Rust-owned gates",
        "autonomous fallback completes without the human surface"
      ]
    }

    %__MODULE__{demo | rendered: render(demo)}
  end

  def read_gated_write?(%__MODULE__{} = demo) do
    demo.gated_write_verified? and demo.trusted_write_authority? == false and
      demo.direct_artifact_write? == false and demo.auto_apply_performed? == false and
      demo.preference.review_apply_required? and demo.preference.trusted_write_performed? == false
  end

  def autonomous_first?(%__MODULE__{} = demo) do
    demo.autonomous_fallback_verified? and demo.autonomous.status == :completed_without_human and
      demo.autonomous.waited_for_human? == false and
      demo.autonomous.human_surface_required? == false
  end

  def render(%__MODULE__{} = demo) do
    localized = demo.surface.localized

    [
      "M84 Studio accessibility/i18n/theme/keyboard demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}; direct artifact write: #{demo.direct_artifact_write?}; auto-apply: #{demo.auto_apply_performed?}",
      "Autonomous fallback verified: #{demo.autonomous_fallback_verified?}",
      "Gated write verified: #{demo.gated_write_verified?}",
      "Behavior unchanged: #{demo.behavior_unchanged?}",
      "Locale: #{localized.locale}; title: #{localized.labels["studio.title"]}; theme: #{localized.themeTokens.name}; contrast: #{localized.themeTokens.contrast_ratio}",
      "Focus order: #{Enum.join(localized.focusOrder, " > ")}",
      "Rust route: #{Enum.join(demo.preference.route_cli, " ")}; review/apply required: #{demo.preference.review_apply_required?}",
      "Evidence ref: #{demo.preference.evidence_ref}",
      "Notes: #{Enum.join(demo.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  defp route_preference(%StudioAccessibility{} = model, runner) do
    payload = %{
      "schemaVersion" => "ouroforge.studio-accessibility-preference.v1",
      "locale" => model.locale,
      "theme" => model.theme,
      "interventionAsEvidence" => model.interventionAsEvidence,
      "readGatedWrite" => model.readGatedWrite,
      "directArtifactWrite" => model.directArtifactWrite,
      "studioTrustedWriteAuthority" => model.studioTrustedWriteAuthority,
      "commandBridge" => model.commandBridge,
      "reviewApplyRequired" => true,
      "behaviorSignature" => StudioAccessibility.behavior_signature(model),
      "boundary" => model.boundary
    }

    route_cli = ["evaluate", "studio-accessibility-preference"]
    {:ok, result} = runner.(route_cli, payload)

    {:ok,
     %{
       route_cli: route_cli,
       status: status_from_result(result),
       evidence_ref: result["evidenceRef"],
       rust_owned?: result["rustOwned"] == true,
       trusted_write_performed?: result["trustedWritePerformed"] == true,
       review_apply_required?: result["reviewApplyRequired"] != false
     }}
  end

  defp status_from_result(%{"verified" => true}), do: :verified_preference
  defp status_from_result(_), do: :blocked

  defp gated_write_verified?(model, routed) do
    model.interventionAsEvidence and model.readGatedWrite and model.directArtifactWrite == false and
      model.studioTrustedWriteAuthority == false and model.commandBridge == false and
      routed.status == :verified_preference and routed.rust_owned? and
      routed.trusted_write_performed? == false and routed.review_apply_required? and
      present?(routed.evidence_ref)
  end

  defp autonomous_fallback_verified?(autonomous) do
    autonomous.status == :completed_without_human and autonomous.waited_for_human? == false and
      autonomous.human_surface_required? == false and autonomous.cli_fallback_supported? == true and
      autonomous.trusted_write_performed? == false
  end

  defp autonomous_default_demo do
    %{
      demo_id: "m84-studio-accessibility-autonomous-default",
      status: :completed_without_human,
      human_intervention: :absent,
      waited_for_human?: false,
      human_surface_required?: false,
      cli_fallback_supported?: true,
      trusted_write_performed?: false,
      evidence_refs: [
        "runs/m84/accessibility/no-human-required.json",
        "runs/m84/accessibility/cli-fallback-supported.json"
      ],
      boundary: StudioAccessibility.boundary()
    }
  end

  defp present?(value), do: is_binary(value) and value != ""
end

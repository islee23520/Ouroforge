defmodule OuroforgeExecutor.ScenarioCoverageV74Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{StudioAccessibility, StudioAccessibilityDemo}

  @coverage_doc Path.expand(
                  "../../../../docs/scenario-coverage-v74-studio-accessibility-i18n-themes-keyboard.md",
                  __DIR__
                )

  def runner(["evaluate", "studio-accessibility-preference"], payload) do
    assert payload["interventionAsEvidence"]
    assert payload["readGatedWrite"]
    assert payload["directArtifactWrite"] == false
    assert payload["studioTrustedWriteAuthority"] == false
    assert payload["commandBridge"] == false
    assert payload["reviewApplyRequired"] == true
    assert payload["behaviorSignature"].rust_data_plane_required

    {:ok,
     %{
       "verified" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/v74/studio-accessibility/preference-gate.json"
     }}
  end

  test "v74 coverage document records M84 boundaries" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v74"
    assert doc =~ "Tool Accessibility, Internationalization, Themes, and Keyboard"
    assert doc =~ "intervention-as-evidence"
    assert doc =~ "read + gated-write"
    assert doc =~ "Rust remains the data plane"
    assert doc =~ "CLI fallback remains sufficient"
    assert doc =~ "#1 and #23 remain open"
  end

  test "v74 a11y labels focus order themes and shortcuts are complete" do
    assert {:ok, model} =
             StudioAccessibility.render_model(%{locale: "ko", theme: "high-contrast"})

    assert model.aria["skip-link"] == "본문으로 건너뛰기"
    assert model.aria["campaign-progress"] == "캠페인 진행"
    assert "skip-link" == hd(model.focusOrder)
    assert "cli-fallback" == List.last(model.focusOrder)
    assert model.themeTokens.contrast_ratio >= 7.0
    assert model.themeTokens.reduced_motion

    shortcut_pairs = Enum.map(model.shortcuts, &{&1.scope, &1.key})
    assert shortcut_pairs == Enum.uniq(shortcut_pairs)
    assert Enum.any?(model.shortcuts, &(&1.action == "copy-cli-reference"))
    assert Enum.any?(model.shortcuts, &(&1.action == "open-review-apply-status"))
  end

  test "v74 localization and themes preserve behavior signature" do
    assert {:ok, english} = StudioAccessibility.render_model(%{locale: "en", theme: "light"})

    assert {:ok, korean} =
             StudioAccessibility.render_model(%{locale: "ko", theme: "high-contrast"})

    assert english.labels["studio.title"] != korean.labels["studio.title"]

    assert StudioAccessibility.behavior_signature(english) ==
             StudioAccessibility.behavior_signature(korean)
  end

  test "v74 no raw bypass trusted write command bridge or mandatory-human drift" do
    assert {:error, :raw_bypass_forbidden} =
             StudioAccessibility.render_model(%{locale: "en", theme: "raw_write_bypass"})

    assert {:ok, model} = StudioAccessibility.render_model(%{locale: "en", theme: "light"})

    assert {:error, :trusted_write_forbidden} =
             %{model | directArtifactWrite: true} |> StudioAccessibility.validate()

    assert {:error, :trusted_write_forbidden} =
             %{model | studioTrustedWriteAuthority: true} |> StudioAccessibility.validate()

    assert {:error, :trusted_write_forbidden} =
             %{model | elixirOwnsArtifactSemantics: true} |> StudioAccessibility.validate()

    assert {:error, :trusted_write_forbidden} =
             %{model | commandBridge: true} |> StudioAccessibility.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{model | humanRequiredForAutonomousLoop: true} |> StudioAccessibility.validate()
  end

  test "v74 demo proves gated preference routing and no-human fallback" do
    demo = StudioAccessibilityDemo.run(runner: &__MODULE__.runner/2)

    assert StudioAccessibilityDemo.read_gated_write?(demo)
    assert StudioAccessibilityDemo.autonomous_first?(demo)
    assert demo.behavior_unchanged?
    assert demo.gated_write_verified?
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.auto_apply_performed?
    assert demo.preference.status == :verified_preference
    assert demo.preference.rust_owned?
  end
end

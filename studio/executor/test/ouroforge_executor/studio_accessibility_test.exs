defmodule OuroforgeExecutor.StudioAccessibilityTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.StudioAccessibility

  test "renders keyboard and screen-reader navigable Studio metadata" do
    assert {:ok, model} = StudioAccessibility.render_model(%{locale: "en", theme: "light"})

    assert model.schemaVersion == "ouroforge.studio-accessibility.v1"
    assert model.boundary =~ "tool accessibility i18n themes keyboard"

    assert model.focusOrder == [
             "skip-link",
             "command-palette",
             "campaign-progress",
             "evidence-provenance",
             "gate-status",
             "cli-fallback"
           ]

    assert model.aria["skip-link"] == "Skip to main content"
    assert model.aria["gate-status"] == "Gate status"
    assert Enum.any?(model.shortcuts, &(&1.action == "open-command-palette"))
    assert Enum.any?(model.shortcuts, &(&1.action == "copy-cli-reference"))
    assert model.themeTokens.contrast_ratio >= 7.0
  end

  test "gettext-style catalog localizes labels without changing behavior" do
    assert {:ok, english} = StudioAccessibility.render_model(%{locale: "en", theme: "light"})

    assert {:ok, korean} =
             StudioAccessibility.render_model(%{locale: "ko", theme: "high-contrast"})

    assert english.labels["studio.title"] == "Ouroforge Studio"
    assert korean.labels["studio.title"] == "오로포지 스튜디오"
    assert korean.labels["action.copy_cli"] == "CLI 대체 명령 복사"
    assert StudioAccessibility.gettext("panel.gates", "ko") == "게이트 상태"

    assert StudioAccessibility.behavior_signature(english) ==
             StudioAccessibility.behavior_signature(korean)
  end

  test "themes expose contrast tokens while preserving read + gated-write authority" do
    assert {:ok, model} =
             StudioAccessibility.render_model(%{locale: "en", theme: "high-contrast"})

    assert model.themeTokens.name == "High Contrast"
    assert model.themeTokens.reduced_motion
    assert model.interventionAsEvidence
    assert model.readGatedWrite
    refute model.directArtifactWrite
    refute model.studioTrustedWriteAuthority
    refute model.elixirOwnsArtifactSemantics
    refute model.commandBridge
    refute model.humanRequiredForAutonomousLoop
    assert model.cliFallbackSupported
  end

  test "raw bypass, trusted writes, command bridge, and mandatory human drift fail closed" do
    assert {:error, :raw_bypass_forbidden} =
             StudioAccessibility.render_model(%{locale: "raw_apply_bypass", theme: "light"})

    assert {:error, :unsupported_locale} =
             StudioAccessibility.render_model(%{locale: "fr", theme: "light"})

    assert {:error, :unsupported_theme} =
             StudioAccessibility.render_model(%{locale: "en", theme: "low-contrast"})

    assert {:ok, model} = StudioAccessibility.render_model(%{locale: "en", theme: "light"})

    assert {:error, :trusted_write_forbidden} =
             %{model | directArtifactWrite: true} |> StudioAccessibility.validate()

    assert {:error, :trusted_write_forbidden} =
             %{model | studioTrustedWriteAuthority: true} |> StudioAccessibility.validate()

    assert {:error, :trusted_write_forbidden} =
             %{model | commandBridge: true} |> StudioAccessibility.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{model | humanRequiredForAutonomousLoop: true} |> StudioAccessibility.validate()
  end
end

defmodule OuroforgeExecutor.OnboardingFirstRunTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OnboardingFirstRun

  defp attrs(overrides \\ %{}) do
    Map.merge(
      %{
        template_id: "collect-and-exit",
        human_actor: "local-first-run-user",
        selected_at: "2026-06-09T00:00:00Z"
      },
      overrides
    )
  end

  test "template gallery exposes local starter templates with sample seeds and docs" do
    templates = OnboardingFirstRun.templates()
    assert Enum.any?(templates, &(&1.id == "collect-and-exit"))
    assert Enum.any?(templates, &(&1.id == "grid-puzzle-front-door"))

    for template <- templates do
      assert template.proposal_only
      refute template.trusted_write_authority
      assert is_binary(template.seed_ref) and template.seed_ref != ""
      assert is_binary(template.docs_ref) and template.docs_ref != ""

      assert is_list(template.gates) and
               String.contains?(Enum.join(template.gates, " "), "review/apply")
    end
  end

  test "first-run selection becomes read + gated-write intervention evidence" do
    assert {:ok, selection} = OnboardingFirstRun.select_template(attrs())
    assert selection.interventionAsEvidence
    assert selection.readGatedWrite
    refute selection.directArtifactWrite
    refute selection.studioTrustedWriteAuthority
    refute selection.commandBridge
    refute selection.humanRequiredForAutonomousLoop
    assert selection.cliFallbackSupported

    assert {:ok, read_model} = OnboardingFirstRun.read_model(selection)
    assert read_model["templateId"] == "collect-and-exit"
    assert read_model["directArtifactWrite"] == false
    assert read_model["commandBridge"] == false
    assert Enum.any?(read_model["firstRunSteps"], &(&1.id == "run-template-game"))
  end

  test "first-run flow reaches run and evaluate steps through copyable CLI only" do
    {:ok, selection} = OnboardingFirstRun.select_template(attrs())
    step_ids = Enum.map(selection.firstRunSteps, & &1.id)

    assert step_ids == [
             "template-selected",
             "validate-template-project",
             "run-template-game",
             "evaluate-run",
             "review-before-apply"
           ]

    assert Enum.all?(selection.firstRunSteps, &(&1.trusted_write_authority == false))

    assert Enum.any?(
             selection.firstRunSteps,
             &(&1.command == ["run", "examples/playable-demo-v2/collect-and-exit"])
           )

    assert Enum.any?(selection.firstRunSteps, &(&1.command == ["evaluate", "runs/latest"]))
  end

  test "raw bypass, unknown template, and command bridge drift fail closed" do
    assert {:error, :unknown_template} =
             OnboardingFirstRun.select_template(attrs(%{template_id: "hosted-template-store"}))

    assert {:error, :raw_bypass_forbidden} =
             OnboardingFirstRun.select_template(attrs(%{human_actor: "raw_write_bypass"}))

    {:ok, selection} = OnboardingFirstRun.select_template(attrs())

    assert {:error, :trusted_write_forbidden} =
             %{selection | commandBridge: true} |> OnboardingFirstRun.validate()
  end

  test "autonomous fallback remains available without first-run UI" do
    demo = OnboardingFirstRun.autonomous_default_demo()
    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    refute demo.waited_for_human?
    refute demo.onboarding_required?
    assert demo.cli_fallback_supported?
    refute demo.trusted_write_performed?
    assert demo.boundary =~ "loop completes without human"
  end
end

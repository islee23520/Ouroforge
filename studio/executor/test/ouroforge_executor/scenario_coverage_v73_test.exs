defmodule OuroforgeExecutor.ScenarioCoverageV73Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.{OnboardingFirstRun, OnboardingFirstRunDemo}

  @coverage_doc Path.expand(
                  "../../../../docs/scenario-coverage-v73-onboarding-first-run.md",
                  __DIR__
                )

  def runner(["project", "validate", "examples/playable-demo-v2/collect-and-exit"], payload) do
    assert payload["interventionAsEvidence"]
    assert payload["readGatedWrite"]
    assert payload["directArtifactWrite"] == false
    assert payload["studioTrustedWriteAuthority"] == false
    assert payload["commandBridge"] == false

    {:ok,
     %{
       "verified" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/v73/onboarding/project-validate.json"
     }}
  end

  def runner(["run", "examples/playable-demo-v2/collect-and-exit"], payload) do
    assert payload["interventionAsEvidence"]
    assert payload["readGatedWrite"]
    assert payload["directArtifactWrite"] == false
    assert payload["studioTrustedWriteAuthority"] == false
    assert payload["commandBridge"] == false

    {:ok,
     %{
       "running" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/v73/onboarding/run-evidence.json"
     }}
  end

  def runner(["evaluate", "runs/latest"], payload) do
    assert payload["interventionAsEvidence"]
    assert payload["readGatedWrite"]
    assert payload["directArtifactWrite"] == false
    assert payload["studioTrustedWriteAuthority"] == false
    assert payload["commandBridge"] == false

    {:ok,
     %{
       "evaluated" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/v73/onboarding/evaluator.json"
     }}
  end

  test "v73 coverage document records onboarding first-run boundaries" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v73"
    assert doc =~ "Onboarding, Templates, In-Product Docs, and First-Run"
    assert doc =~ "intervention-as-evidence"
    assert doc =~ "read + gated-write"
    assert doc =~ "Rust remains the data plane"
    assert doc =~ "copyable CLI commands only"
    assert doc =~ "CLI fallback remains sufficient"
    assert doc =~ "#1 and #23 remain open"
  end

  test "v73 template gallery and docs point to local first-run surfaces" do
    templates = OnboardingFirstRun.templates()
    docs = OnboardingFirstRun.docs()

    assert Enum.any?(templates, &(&1.id == "collect-and-exit"))
    assert Enum.any?(templates, &(&1.id == "grid-puzzle-front-door"))

    for template <- templates do
      assert template.proposal_only
      refute template.trusted_write_authority
      assert String.starts_with?(template.project_ref, "examples/")
      assert String.starts_with?(template.seed_ref, "examples/")
      assert String.starts_with?(template.docs_ref, "docs/")
      assert Enum.any?(template.gates, &String.contains?(&1, "review/apply"))
    end

    assert Enum.any?(docs, &(&1.id == "validate-project" and &1.gate == "project validate"))
    assert Enum.any?(docs, &(&1.id == "run-game" and &1.gate == "run evidence"))
    assert Enum.any?(docs, &(&1.id == "evaluate-output" and &1.gate == "evaluator"))
  end

  test "v73 no raw bypass and mandatory-human regressions fail closed" do
    assert {:error, :raw_bypass_forbidden} =
             OnboardingFirstRun.select_template(%{
               template_id: "collect-and-exit",
               human_actor: "raw_apply_bypass",
               selected_at: "2026-06-09T00:00:00Z"
             })

    {:ok, selection} =
      OnboardingFirstRun.select_template(%{
        template_id: "collect-and-exit",
        human_actor: "local-first-run-user",
        selected_at: "2026-06-09T00:00:00Z"
      })

    assert {:error, :trusted_write_forbidden} =
             %{selection | studioTrustedWriteAuthority: true} |> OnboardingFirstRun.validate()

    assert {:error, :trusted_write_forbidden} =
             %{selection | directArtifactWrite: true} |> OnboardingFirstRun.validate()

    assert {:error, :trusted_write_forbidden} =
             %{selection | commandBridge: true} |> OnboardingFirstRun.validate()

    assert {:error, :autonomy_or_cli_fallback_broken} =
             %{selection | humanRequiredForAutonomousLoop: true} |> OnboardingFirstRun.validate()
  end

  test "v73 demo proves gated first-run and no-human fallback" do
    demo = OnboardingFirstRunDemo.run(runner: &__MODULE__.runner/2)

    assert OnboardingFirstRunDemo.read_gated_write?(demo)
    assert OnboardingFirstRunDemo.autonomous_first?(demo)
    assert demo.running_verified_game?
    assert demo.gated_write_verified?
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.auto_apply_performed?
    assert Enum.all?(demo.first_run.gate_evidence, & &1.rust_owned?)
    assert Enum.all?(demo.first_run.gate_evidence, &(&1.trusted_write_performed? == false))
  end
end

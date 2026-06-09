defmodule OuroforgeExecutor.OnboardingFirstRunDemoTest do
  use ExUnit.Case, async: true
  @moduletag :demo

  alias OuroforgeExecutor.OnboardingFirstRunDemo

  def runner(["project", "validate", "examples/playable-demo-v2/collect-and-exit"], payload) do
    assert_common_payload(payload, "validate-template-project")

    {:ok,
     %{
       "verified" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/m83/demo/collect-and-exit.project-validate.json"
     }}
  end

  def runner(["run", "examples/playable-demo-v2/collect-and-exit"], payload) do
    assert_common_payload(payload, "run-template-game")

    {:ok,
     %{
       "running" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/m83/demo/collect-and-exit.run-evidence.json"
     }}
  end

  def runner(["evaluate", "runs/latest"], payload) do
    assert_common_payload(payload, "evaluate-run")

    {:ok,
     %{
       "evaluated" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/m83/demo/collect-and-exit.evaluator.json"
     }}
  end

  test "M83 demo reaches a running verified game through existing gates" do
    demo = OnboardingFirstRunDemo.run(runner: &__MODULE__.runner/2)

    assert demo.version == "m83-onboarding-first-run-demo-v1"
    assert demo.boundary == :read_gated_write_demo_no_elixir_artifact_writes
    assert demo.running_verified_game?
    assert OnboardingFirstRunDemo.read_gated_write?(demo)
    assert demo.gated_write_verified?
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.auto_apply_performed?

    assert Enum.map(demo.first_run.gate_evidence, & &1.step_id) == [
             "validate-template-project",
             "run-template-game",
             "evaluate-run"
           ]
  end

  test "M83 demo proves autonomous fallback completes without a human surface" do
    demo = OnboardingFirstRunDemo.run(runner: &__MODULE__.runner/2)

    assert OnboardingFirstRunDemo.autonomous_first?(demo)
    assert demo.autonomous.status == :completed_without_human
    assert demo.autonomous.human_intervention == :absent
    refute demo.autonomous.waited_for_human?
    refute demo.autonomous.onboarding_required?
    assert demo.autonomous.cli_fallback_supported?
  end

  test "rendered demo evidence is conservative and names docs plus gates" do
    rendered =
      OnboardingFirstRunDemo.run(runner: &__MODULE__.runner/2)
      |> OnboardingFirstRunDemo.render()

    assert rendered =~ "M83 onboarding first-run demo"
    assert rendered =~ "trusted writes: false"
    assert rendered =~ "direct artifact write: false"
    assert rendered =~ "auto-apply: false"
    assert rendered =~ "Autonomous fallback verified: true"
    assert rendered =~ "Gated write verified: true"
    assert rendered =~ "Running verified game: true"
    assert rendered =~ "docs/onboarding-templates-first-run-v1.md"
    assert rendered =~ "project validate=verified"
    assert rendered =~ "run evidence=running"
    assert rendered =~ "evaluator=evaluated"
    assert rendered =~ "review/apply required: true"
    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
    refute rendered =~ "hosted"
  end

  defp assert_common_payload(payload, step_id) do
    assert payload["schemaVersion"] == "ouroforge.onboarding-first-run-demo.v1"
    assert payload["templateId"] == "collect-and-exit"
    assert payload["stepId"] == step_id
    assert payload["interventionAsEvidence"]
    assert payload["readGatedWrite"]
    assert payload["directArtifactWrite"] == false
    assert payload["studioTrustedWriteAuthority"] == false
    assert payload["commandBridge"] == false
    assert payload["reviewApplyRequired"] == true
    assert payload["boundary"] =~ "Rust data plane validates and records"
  end
end

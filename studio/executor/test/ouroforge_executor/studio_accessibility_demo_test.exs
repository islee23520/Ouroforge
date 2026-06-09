defmodule OuroforgeExecutor.StudioAccessibilityDemoTest do
  use ExUnit.Case, async: true
  @moduletag :demo

  alias OuroforgeExecutor.StudioAccessibilityDemo

  def runner(["evaluate", "studio-accessibility-preference"], payload) do
    assert payload["schemaVersion"] == "ouroforge.studio-accessibility-preference.v1"
    assert payload["locale"] == "ko"
    assert payload["theme"] == "high-contrast"
    assert payload["interventionAsEvidence"]
    assert payload["readGatedWrite"]
    assert payload["directArtifactWrite"] == false
    assert payload["studioTrustedWriteAuthority"] == false
    assert payload["commandBridge"] == false
    assert payload["reviewApplyRequired"] == true
    assert payload["behaviorSignature"].trusted_write_authority == false
    assert payload["boundary"] =~ "Rust data plane validates and records"

    {:ok,
     %{
       "verified" => true,
       "rustOwned" => true,
       "trustedWritePerformed" => false,
       "reviewApplyRequired" => true,
       "evidenceRef" => "runs/m84/demo/studio-accessibility-preference.json"
     }}
  end

  test "M84 demo proves a11y i18n theme and keyboard path stays gated" do
    demo = StudioAccessibilityDemo.run(runner: &__MODULE__.runner/2)

    assert demo.version == "m84-studio-accessibility-demo-v1"
    assert demo.boundary == :read_gated_write_demo_no_elixir_artifact_writes
    assert StudioAccessibilityDemo.read_gated_write?(demo)
    assert demo.gated_write_verified?
    assert demo.behavior_unchanged?
    refute demo.trusted_write_authority?
    refute demo.direct_artifact_write?
    refute demo.auto_apply_performed?

    assert demo.surface.localized.locale == "ko"
    assert demo.surface.localized.theme == "high-contrast"
    assert demo.preference.status == :verified_preference
    assert demo.preference.review_apply_required?
  end

  test "M84 demo proves autonomous fallback completes without human surface" do
    demo = StudioAccessibilityDemo.run(runner: &__MODULE__.runner/2)

    assert StudioAccessibilityDemo.autonomous_first?(demo)
    assert demo.autonomous.status == :completed_without_human
    assert demo.autonomous.human_intervention == :absent
    refute demo.autonomous.waited_for_human?
    refute demo.autonomous.human_surface_required?
    assert demo.autonomous.cli_fallback_supported?
  end

  test "rendered demo evidence is conservative and names the Rust gate" do
    rendered =
      StudioAccessibilityDemo.run(runner: &__MODULE__.runner/2)
      |> StudioAccessibilityDemo.render()

    assert rendered =~ "M84 Studio accessibility/i18n/theme/keyboard demo"
    assert rendered =~ "trusted writes: false"
    assert rendered =~ "direct artifact write: false"
    assert rendered =~ "auto-apply: false"
    assert rendered =~ "Autonomous fallback verified: true"
    assert rendered =~ "Gated write verified: true"
    assert rendered =~ "Behavior unchanged: true"
    assert rendered =~ "Rust route: evaluate studio-accessibility-preference"
    assert rendered =~ "review/apply required: true"
    assert rendered =~ "오로포지 스튜디오"
    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
    refute rendered =~ "hosted"
  end
end

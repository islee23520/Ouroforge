defmodule OuroforgeExecutor.GuidedGenerativeFrontDoorDemoTest do
  use ExUnit.Case, async: true
  @moduletag :demo

  alias OuroforgeExecutor.GuidedGenerativeFrontDoorDemo

  def runner(["generative-front-door", "validate"], submission) do
    assert submission["interventionAsEvidence"]
    assert submission["readGatedWrite"]
    assert submission["proposalOnly"]
    assert submission["directArtifactWrite"] == false
    assert submission["studioTrustedWriteAuthority"] == false
    assert submission["autoApplyPerformed"] == false
    assert submission["preview"]["proposalOnly"] == true
    assert submission["preview"]["reviewApplyRequired"] == true

    {:ok,
     %{
       "verifiedProposal" => true,
       "evidenceRef" => "runs/m82/demo/guided-front-door.verified-proposal.json",
       "reviewApplyRequired" => true
     }}
  end

  test "M82 demo proves guided non-developer intake stays read + gated-write" do
    demo = GuidedGenerativeFrontDoorDemo.run(runner: &__MODULE__.runner/2)

    assert demo.version == "m82-guided-generative-front-door-demo-v1"
    assert demo.boundary == :read_gated_write_demo_no_elixir_artifact_writes
    assert GuidedGenerativeFrontDoorDemo.read_gated_write?(demo)
    assert demo.gated_write_verified?
    refute demo.trusted_write_authority?
    refute demo.auto_apply_performed?

    assert demo.guided.capture.templateId == "grid-puzzle"
    assert demo.guided.capture.preview["status"] == "draft-pending-rust-validation"
    assert demo.guided.routed.status == :verified_proposal
    assert demo.guided.routed.review_apply_required?
  end

  test "M82 demo proves autonomous fallback completes without a human surface" do
    demo = GuidedGenerativeFrontDoorDemo.run(runner: &__MODULE__.runner/2)

    assert GuidedGenerativeFrontDoorDemo.autonomous_first?(demo)
    assert demo.autonomous.status == :completed_without_human
    assert demo.autonomous.human_intervention == :absent
    refute demo.autonomous.waited_for_human?
    refute demo.autonomous.human_surface_required?
    assert demo.autonomous.cli_fallback_supported?
  end

  test "rendered demo evidence is conservative and names the Rust gate" do
    rendered =
      GuidedGenerativeFrontDoorDemo.run(runner: &__MODULE__.runner/2)
      |> GuidedGenerativeFrontDoorDemo.render()

    assert rendered =~ "M82 guided generative front-door demo"
    assert rendered =~ "trusted writes: false"
    assert rendered =~ "auto-apply: false"
    assert rendered =~ "Autonomous fallback verified: true"
    assert rendered =~ "Gated write verified: true"
    assert rendered =~ "Rust route: generative-front-door validate"
    assert rendered =~ "review/apply required: true"
    assert rendered =~ "intervention-as-evidence"
    refute rendered =~ "raw_write_bypass"
    refute rendered =~ "no-code"
    refute rendered =~ "hosted"
  end
end

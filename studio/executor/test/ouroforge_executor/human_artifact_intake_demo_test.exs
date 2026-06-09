defmodule OuroforgeExecutor.HumanArtifactIntakeDemoTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.HumanArtifactIntakeDemo

  @moduletag :demo

  test "autonomous default completes without human artifact surface" do
    demo = HumanArtifactIntakeDemo.autonomous_default_demo()
    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    assert demo.cli_fallback_supported
    refute demo.trusted_write_performed
    refute demo.gated_write_attempted
    assert Enum.any?(demo.evidence_refs, &String.contains?(&1, "no-human"))
    assert demo.boundary =~ "local-first CLI fallback"
  end

  test "human-authored artifact is captured then routed to Rust validator before review/apply" do
    runner = fn argv, artifact ->
      assert argv == ["human-artifact-intake", "validate"]
      assert artifact["status"] == "ready-for-review-apply"
      assert artifact["author"] == "human:local-author"
      assert artifact["humanProvenance"]
      assert artifact["interventionAsEvidence"]
      assert artifact["directArtifactWrite"] == false
      assert artifact["studioTrustedWriteAuthority"] == false
      {:ok, %{"readyForReviewApply" => true, "passedGateCount" => 4}}
    end

    assert {:ok, demo} = HumanArtifactIntakeDemo.human_intake_demo(runner: runner)
    assert demo.status == :ready_for_review_apply
    assert demo.gated_write_attempted
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
    assert demo.capture["readGatedWrite"]
    assert demo.rust_result["readyForReviewApply"]
  end

  test "failed validation blocks human-authored artifact without raw apply" do
    runner = fn argv, artifact ->
      assert argv == ["human-artifact-intake", "validate"]
      assert artifact["status"] == "blocked"
      assert Enum.any?(artifact["gateResults"], &(&1["status"] == "failed"))
      assert artifact["directArtifactWrite"] == false
      assert artifact["rawBypassRequested"] == false
      {:error, %{"readyForReviewApply" => false, "blockedReasons" => ["Evaluator:Failed"]}}
    end

    assert {:ok, demo} = HumanArtifactIntakeDemo.blocked_human_intake_demo(runner: runner)
    assert demo.status == :blocked_by_gate
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
    assert demo.rust_result["readyForReviewApply"] == false
  end
end

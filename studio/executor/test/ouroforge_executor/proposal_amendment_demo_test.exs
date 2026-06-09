defmodule OuroforgeExecutor.ProposalAmendmentDemoTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.ProposalAmendmentDemo

  @moduletag :demo

  test "autonomous default completes without human surface" do
    demo = ProposalAmendmentDemo.autonomous_default_demo()

    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    assert demo.cli_fallback_supported
    refute demo.trusted_write_performed
    refute demo.gated_write_attempted
    assert Enum.any?(demo.evidence_refs, &String.contains?(&1, "no-human"))
    assert demo.boundary =~ "local-first CLI fallback"
  end

  test "human amendment is captured then routed to Rust validator before review/apply" do
    runner = fn argv, artifact ->
      assert argv == ["proposal-amendment", "validate"]
      assert artifact["status"] == "ready-for-review-apply"
      assert artifact["interventionAsEvidence"]
      assert artifact["autoApplyPerformed"] == false
      assert artifact["studioTrustedWriteAuthority"] == false
      {:ok, %{"readyForReviewApply" => true, "passedGateCount" => 4}}
    end

    assert {:ok, demo} = ProposalAmendmentDemo.amended_proposal_demo(runner: runner)
    assert demo.status == :ready_for_review_apply
    assert demo.gated_write_attempted
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
    assert demo.capture["readGatedWrite"]
    assert demo.rust_result["readyForReviewApply"]
  end

  test "failed re-verification blocks the amended proposal without raw apply" do
    runner = fn argv, artifact ->
      assert argv == ["proposal-amendment", "validate"]
      assert artifact["status"] == "blocked"
      assert Enum.any?(artifact["gateResults"], &(&1["status"] == "failed"))
      {:error, %{"readyForReviewApply" => false, "blockedReasons" => ["Evaluator:Failed"]}}
    end

    assert {:ok, demo} = ProposalAmendmentDemo.blocked_amendment_demo(runner: runner)
    assert demo.status == :blocked_by_gate
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
    assert demo.rust_result["readyForReviewApply"] == false
  end
end

defmodule OuroforgeExecutor.HumanConstraintDemoTest do
  use ExUnit.Case, async: true
  @moduletag :demo

  alias OuroforgeExecutor.HumanConstraintDemo

  test "M78 autonomous default completes without human constraint surface" do
    demo = HumanConstraintDemo.autonomous_default_demo()

    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    assert demo.cli_fallback_supported
    refute demo.waited_for_human?
    refute demo.human_surface_required?
    refute demo.trusted_write_performed
    refute demo.gated_write_attempted
    assert Enum.any?(demo.evidence_refs, &String.contains?(&1, "no-human-constraint"))
    assert demo.boundary =~ "loop completes without human"
  end

  test "M78 demo blocks violating candidate through Rust evaluator constraint gate" do
    runner = fn argv, gate_input ->
      assert argv == ["evaluate"]
      assert gate_input["schemaVersion"] == "ouroforge.human-constraint-gate.v1"
      assert gate_input["candidate"]["mechanics"] == ["dash", "burn"]
      [constraint] = gate_input["constraints"]
      assert constraint["kind"] == "forbidden-mechanic"
      assert constraint["forbiddenMechanic"] == "dash"
      assert constraint["interventionAsEvidence"]
      assert constraint["readGatedWrite"]
      assert constraint["directArtifactWrite"] == false
      assert constraint["rawBypassRequested"] == false
      assert constraint["studioTrustedWriteAuthority"] == false
      assert constraint["humanRequiredForAutonomousLoop"] == false

      {:error,
       %{
         "readyForReviewApply" => false,
         "humanConstraints" => %{"status" => "fail", "failureCount" => 1},
         "blockedReasons" => ["HumanConstraint: forbidden mechanic dash"]
       }}
    end

    assert {:ok, demo} = HumanConstraintDemo.blocking_constraint_demo(runner: runner)
    assert demo.status == :blocked_by_human_constraint_gate
    assert demo.gated_write_attempted
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
    assert demo.rust_result["readyForReviewApply"] == false
    assert demo.rendered =~ "Gated write verified: true"
    assert demo.rendered =~ "intervention-as-evidence routed through Rust evaluator gates"
    refute demo.rendered =~ "raw_write_bypass"
    refute demo.rendered =~ "no-code"
    refute demo.rendered =~ "hosted"
  end

  test "M78 demo allows compliant candidate while preserving gated-write evidence" do
    runner = fn argv, gate_input ->
      assert argv == ["evaluate"]
      assert gate_input["candidate"]["mechanics"] == ["burn"]
      [constraint] = gate_input["constraints"]
      assert constraint["evaluatorEvidenceRef"] == "runs/m78/demo/evaluator/human-constraint.json"

      {:ok,
       %{
         "readyForReviewApply" => true,
         "humanConstraints" => %{"status" => "pass", "failureCount" => 0}
       }}
    end

    assert {:ok, demo} = HumanConstraintDemo.passing_constraint_demo(runner: runner)
    assert demo.status == :passes_human_constraint_gate
    assert demo.rust_result["readyForReviewApply"]
    refute demo.trusted_write_performed
    assert demo.autonomous_fallback_still_supported
  end
end

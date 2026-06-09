defmodule OuroforgeExecutor.DiagnosisCorrectionDemoTest do
  use ExUnit.Case, async: true
  @moduletag :demo

  alias OuroforgeExecutor.DiagnosisCorrectionDemo

  test "M79 autonomous default completes without human correction surface" do
    demo = DiagnosisCorrectionDemo.autonomous_default_demo()

    assert demo.status == :completed_without_human
    assert demo.human_intervention == :absent
    assert demo.cli_fallback_supported
    refute demo.waited_for_human?
    refute demo.human_surface_required?
    refute demo.trusted_write_performed
    refute demo.gated_write_attempted
    assert demo.attribution.selected == "asset-provenance-gap"
    assert Enum.any?(demo.evidence_refs, &String.contains?(&1, "no-human-correction"))
    assert demo.boundary =~ "loop completes without human"
  end

  test "M79 demo records a correction and improves subsequent attribution through Rust gates" do
    runner = fn argv, submission ->
      assert argv == ["diagnosis-correction", "validate"]
      assert submission["schemaVersion"] == "ouroforge.diagnosis-correction-capture.v1"
      assert submission["originalAttribution"] == "asset-provenance-gap"
      assert submission["correctedAttribution"] == "evaluator-threshold-drift"
      assert submission["interventionAsEvidence"]
      assert submission["readGatedWrite"]
      assert submission["directArtifactWrite"] == false
      assert submission["rawBypassRequested"] == false
      assert submission["studioTrustedWriteAuthority"] == false
      assert submission["elixirOwnsDiagnosisSemantics"] == false
      assert submission["opaqueMlUpdate"] == false
      assert submission["automatedFunTasteInference"] == false
      assert submission["humanRequiredForAutonomousLoop"] == false

      {:ok,
       %{
         "recorded" => true,
         "beforeAttribution" => "asset-provenance-gap",
         "afterAttribution" => "evaluator-threshold-drift",
         "priorUpdate" => %{
           "kind" => "transparent-heuristic-prior",
           "delta" => 4,
           "correctionRef" => "corr-m79-demo-threshold"
         },
         "gatePath" => ["review/apply", "scene/source-apply", "evaluator", "evidence/provenance"],
         "provenanceRefs" => ["runs/m79/demo/provenance/corr-m79-demo-threshold.json"]
       }}
    end

    assert {:ok, demo} = DiagnosisCorrectionDemo.corrected_attribution_demo(runner: runner)
    assert demo.status == :corrected_and_reattributed
    assert demo.before_attribution == "asset-provenance-gap"
    assert demo.after_attribution == "evaluator-threshold-drift"
    assert demo.prior_update["kind"] == "transparent-heuristic-prior"
    assert demo.gated_write_attempted
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
    assert demo.rendered =~ "Gated write verified: true"
    assert demo.rendered =~ "intervention-as-evidence routed through Rust CLI gates"
    assert demo.rendered =~ "transparent heuristic prior update"
    refute demo.rendered =~ "raw_write_bypass"
    refute demo.rendered =~ "no-code"
    refute demo.rendered =~ "hosted"
  end

  test "M79 demo keeps rejected corrections visible without blocking autonomy" do
    runner = fn argv, submission ->
      assert argv == ["diagnosis-correction", "validate"]
      assert submission["routeCli"] == ["diagnosis-correction", "validate"]

      {:error,
       %{
         "recorded" => false,
         "blockedReasons" => ["Evaluator: failed correction evidence"],
         "gatePath" => ["review/apply", "scene/source-apply", "evaluator", "evidence/provenance"]
       }}
    end

    assert {:ok, demo} = DiagnosisCorrectionDemo.rejected_correction_demo(runner: runner)
    assert demo.status == :blocked_by_existing_gates
    assert demo.gated_write_attempted
    refute demo.trusted_write_performed
    refute demo.raw_bypass_performed
    assert demo.autonomous_fallback_still_supported
    assert demo.rendered =~ "Autonomous fallback verified: true"
  end
end

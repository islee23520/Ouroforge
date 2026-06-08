defmodule OuroforgeExecutor.ScenarioCoverageV59Test do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.{
    CampaignStatus,
    Contract,
    Demo,
    ParityPanel,
    Runbook,
    TaskDAG,
    TelemetryPanel
  }

  @coverage_doc Path.expand("../../../../docs/scenario-coverage-v59.md", __DIR__)
  @governance_doc Path.expand(
                    "../../../../docs/executor-operator-cockpit-v1-governance-handoff.md",
                    __DIR__
                  )

  def runner("ouroforge", argv, _opts), do: {"#{Enum.join(argv, " ")} valid\n", 0}

  test "v59 documents every M67 cockpit capability and governance guardrail" do
    doc = File.read!(@coverage_doc)

    assert doc =~ "Scenario Coverage v59"
    assert doc =~ "#2002"
    assert doc =~ "#2003"
    assert doc =~ "#2004"
    assert doc =~ "#2005"
    assert doc =~ "#2006"
    assert doc =~ "#2008"
    assert doc =~ "#2009"
    assert doc =~ "V59.contract.boundary"
    assert doc =~ "V59.status.campaign"
    assert doc =~ "V59.dag.frontier"
    assert doc =~ "V59.runbook.copy_only"
    assert doc =~ "V59.telemetry.local"
    assert doc =~ "V59.parity.manual_executor"
    assert doc =~ "V59.demo.composed"
    assert doc =~ "V59.boundary.no_trusted_writes"
    assert doc =~ "Elixir/OTP = local executor control plane only"
    assert doc =~ "Rust kernel = data plane and source of truth"
    assert doc =~ "frozen `ouroforge` CLI surface"
    assert doc =~ "#1 and #23 remain open"
  end

  test "v59 governance handoff records M67-9 evidence and next design gate" do
    doc = File.read!(@governance_doc)

    assert doc =~ "Executor Operator Cockpit v1 Governance Handoff"
    assert doc =~ "M67-9"
    assert doc =~ "#2002"
    assert doc =~ "#2011"
    assert doc =~ "PR #2013"
    assert doc =~ "PR #2021"
    assert doc =~ "Elixir/OTP remains the local executor control plane"
    assert doc =~ "Rust remains the data plane and source of truth"

    assert doc =~
             "No direct artifact, ledger, evidence, trust-gradient, apply, release, merge, or deploy writes"

    assert doc =~ "#1 and #23 remain open governance anchors"
    assert doc =~ "Next design-gate question"
  end

  test "v59 composed cockpit remains read-only across every panel" do
    contract = Contract.read_only_contract()
    status = CampaignStatus.fixture(:blocked)
    dag = TaskDAG.fixture(:blocked)
    runbook = Runbook.from_models(status, dag)
    telemetry = TelemetryPanel.fixture(:budget_limited)
    parity = ParityPanel.fixture(:matching)

    demo =
      Demo.run(
        runner: &__MODULE__.runner/3,
        state: :blocked,
        telemetry: %{stop_gate: :human_decision_required}
      )

    assert Contract.read_only?(contract)
    refute status.trusted_write_authority?
    refute dag.trusted_write_authority?
    assert Runbook.copy_only?(runbook)
    refute telemetry.remote_telemetry?
    refute telemetry.trusted_write_authority?
    assert parity.parity_status == :byte_identical
    refute parity.trusted_write_authority?
    assert Demo.read_only?(demo)
    assert Demo.golden_parity?(demo)
    assert runbook.human_judgment_required?
    assert telemetry.stop_gate.human_judgment_required?
  end

  test "v59 state fixtures cover normal, waiting, retrying, budget, backpressure, and blocked states" do
    status = CampaignStatus.fixtures()
    dag = TaskDAG.fixtures()
    runbook = Runbook.fixtures()
    telemetry = TelemetryPanel.fixtures()

    assert status.normal.status == :running
    assert status.waiting.status == :waiting
    assert status.retrying.status == :retrying
    assert status.budget_limited.status == :budget_limited_requires_human_judgment
    assert status.backpressured.status == :backpressured
    assert status.blocked.status == :blocked_requires_human_judgment

    assert dag.waiting.runnable_frontier == ["seed"]
    assert dag.skipped.skipped_tasks == ["project"]
    assert runbook.blocked.human_judgment_required?
    assert telemetry.backpressured.backpressure.state == :backpressured
  end
end

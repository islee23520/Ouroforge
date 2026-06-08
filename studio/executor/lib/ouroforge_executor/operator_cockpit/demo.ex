defmodule OuroforgeExecutor.OperatorCockpit.Demo do
  @moduledoc """
  M67-7 minimal local read-only executor cockpit demo.

  The demo composes the M67 cockpit contract, campaign status, task DAG,
  runbook, telemetry, and parity panels into a deterministic local snapshot. It
  accepts a runner for `ouroforge` CLI parity checks, renders text only, and
  provides no browser-executed commands or trusted-write authority.
  """

  alias OuroforgeExecutor.OperatorCockpit.{
    CampaignStatus,
    Contract,
    ParityPanel,
    Runbook,
    TaskDAG,
    TelemetryPanel
  }

  defstruct version: "m67-7",
            boundary: :minimal_read_only_executor_cockpit_demo,
            contract: nil,
            campaign_status: nil,
            task_dag: nil,
            runbook: nil,
            telemetry: nil,
            parity: nil,
            panels: [],
            trusted_write_authority?: false,
            executable_actions: []

  def run(opts) do
    runner = Keyword.fetch!(opts, :runner)
    state = Keyword.get(opts, :state, :blocked)
    telemetry = Keyword.get(opts, :telemetry, %{})

    contract = Contract.read_only_contract()
    status = CampaignStatus.fixture(state)
    dag = TaskDAG.fixture(if state == :normal, do: :waiting, else: state)
    runbook = Runbook.from_models(status, dag)
    telemetry_panel = TelemetryPanel.from_inputs(status, dag, telemetry)
    parity = ParityPanel.from_demo(runner: runner)

    %__MODULE__{
      contract: contract,
      campaign_status: status,
      task_dag: dag,
      runbook: runbook,
      telemetry: telemetry_panel,
      parity: parity,
      panels: [:contract, :campaign_status, :task_dag, :runbook, :telemetry, :parity]
    }
  end

  def render(%__MODULE__{} = demo) do
    [
      "M67 minimal read-only executor cockpit demo",
      "Boundary: #{demo.boundary}; trusted writes: #{demo.trusted_write_authority?}; executable actions: #{length(demo.executable_actions)}",
      "",
      Contract.render_summary(demo.contract),
      "",
      CampaignStatus.render(demo.campaign_status),
      "",
      TaskDAG.render(demo.task_dag),
      "",
      Runbook.render(demo.runbook),
      "",
      TelemetryPanel.render(demo.telemetry),
      "",
      ParityPanel.render(demo.parity)
    ]
    |> Enum.join("\n")
  end

  def golden_parity?(%__MODULE__{} = demo), do: demo.parity.parity_status == :byte_identical

  def read_only?(%__MODULE__{} = demo) do
    demo.trusted_write_authority? == false and demo.executable_actions == [] and
      Contract.read_only?(demo.contract) and
      demo.runbook.executable_actions == [] and
      demo.telemetry.remote_telemetry? == false and
      demo.parity.trusted_write_authority? == false
  end
end

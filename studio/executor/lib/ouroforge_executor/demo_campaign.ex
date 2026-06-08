defmodule OuroforgeExecutor.DemoCampaign do
  @moduledoc """
  Bounded M63 demo campaign with golden parity against the manual CLI path.

  The campaign is intentionally tiny and read/validate-only: it proves the
  executor can drive the frozen `ouroforge` CLI surface without changing kernel
  artifacts. The audit trail is operational evidence only; Rust CLI output is
  the data-plane result being compared.
  """

  alias OuroforgeExecutor.CLI

  @steps [
    %{
      step_id: "seed-validate",
      argv: [
        "seed",
        "validate",
        "examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml"
      ],
      evidence_ref: "examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml"
    },
    %{
      step_id: "project-validate",
      argv: ["project", "validate", "examples/playable-demo-v2/collect-and-exit"],
      evidence_ref: "examples/playable-demo-v2/collect-and-exit/ouroforge.project.json"
    }
  ]

  def steps, do: @steps

  def run_manual(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)
    command_opts = Keyword.get(opts, :command_opts, [])

    Enum.map(@steps, fn step ->
      {stdout, status} = runner.("ouroforge", step.argv, command_opts)
      transcript(step, status, stdout)
    end)
  end

  def run_executor(opts \\ []) do
    runner = Keyword.fetch!(opts, :runner)
    command_opts = Keyword.get(opts, :command_opts, [])
    executable = Keyword.get(opts, :executable, "ouroforge")

    Enum.map(@steps, fn step ->
      case CLI.run(step.argv, Keyword.merge(command_opts, executable: executable, runner: runner)) do
        {:ok, result} -> transcript(step, result.status, result.stdout)
        {:error, result} -> transcript(step, result.status, result.stdout)
      end
    end)
  end

  def golden_parity?(manual_transcript, executor_transcript) do
    :erlang.term_to_binary(manual_transcript) == :erlang.term_to_binary(executor_transcript)
  end

  def audit_trail(transcript) do
    Enum.map(transcript, fn entry ->
      %{
        step_id: entry.step_id,
        argv: entry.argv,
        status: entry.status,
        evidence_ref: entry.evidence_ref,
        kernel_evidence: %{stdout: entry.stdout}
      }
    end)
  end

  defp transcript(step, status, stdout) do
    %{
      step_id: step.step_id,
      argv: step.argv,
      status: status,
      stdout: stdout,
      evidence_ref: step.evidence_ref
    }
  end
end

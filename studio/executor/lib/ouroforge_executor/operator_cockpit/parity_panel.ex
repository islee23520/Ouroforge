defmodule OuroforgeExecutor.OperatorCockpit.ParityPanel do
  @moduledoc """
  M67-6 golden parity and manual fallback panel.

  The panel compares executor-driven `ouroforge` CLI output with the equivalent
  manual CLI path and renders copy-only fallback commands. It delegates artifact
  truth to Rust CLI bytes and never writes artifacts, ledgers, evidence, or trust
  state.
  """

  alias OuroforgeExecutor.DemoCampaign

  defstruct version: "m67-6",
            boundary: :read_only_golden_parity_panel,
            parity_status: :unknown,
            comparator: :byte_identical_output,
            manual_transcript: [],
            executor_transcript: [],
            manual_fallback_commands: [],
            mismatches: [],
            artifact_truth: :rust_ouroforge_cli,
            trusted_write_authority?: false,
            human_judgment_required?: false,
            notes: []

  def from_transcripts(manual, executor) when is_list(manual) and is_list(executor) do
    parity? = DemoCampaign.golden_parity?(manual, executor)
    mismatches = mismatches(manual, executor)

    %__MODULE__{
      parity_status: if(parity?, do: :byte_identical, else: :mismatch_requires_human_review),
      manual_transcript: manual,
      executor_transcript: executor,
      manual_fallback_commands: fallback_commands(manual ++ executor),
      mismatches: mismatches,
      human_judgment_required?: not parity?,
      notes: notes(parity?, mismatches)
    }
  end

  def from_demo(opts) do
    runner = Keyword.fetch!(opts, :runner)
    command_opts = Keyword.get(opts, :command_opts, [])
    manual = DemoCampaign.run_manual(runner: runner, command_opts: command_opts)
    executor = DemoCampaign.run_executor(runner: runner, command_opts: command_opts)
    from_transcripts(manual, executor)
  end

  def render(%__MODULE__{} = panel) do
    [
      "Parity panel: read-only golden/manual comparison",
      "Status: #{panel.parity_status}",
      "Comparator: #{panel.comparator}",
      "Artifact truth: #{panel.artifact_truth}",
      "Trusted writes: #{panel.trusted_write_authority?}",
      "Human judgment required: #{panel.human_judgment_required?}",
      "Manual fallback commands: #{Enum.join(panel.manual_fallback_commands, " | ")}",
      "Mismatches: #{render_mismatches(panel.mismatches)}",
      "Notes: #{Enum.join(panel.notes, " | ")}"
    ]
    |> Enum.join("\n")
  end

  def fixture(:matching) do
    runner = fn "ouroforge", argv, _opts -> {"#{Enum.join(argv, " ")} valid\n", 0} end
    from_demo(runner: runner)
  end

  def fixture(:mismatch) do
    manual = [
      %{
        step_id: "seed-validate",
        argv: ["seed", "validate", "seed.yaml"],
        status: 0,
        stdout: "manual",
        evidence_ref: "seed.yaml"
      }
    ]

    executor = [
      %{
        step_id: "seed-validate",
        argv: ["seed", "validate", "seed.yaml"],
        status: 1,
        stdout: "executor",
        evidence_ref: "seed.yaml"
      }
    ]

    from_transcripts(manual, executor)
  end

  defp fallback_commands(transcripts) do
    transcripts
    |> Enum.map(fn entry -> "ouroforge #{Enum.map_join(entry.argv, " ", &shell_quote/1)}" end)
    |> Enum.uniq()
  end

  defp shell_quote(value) do
    value = to_string(value)

    if String.match?(value, ~r/^[A-Za-z0-9_@%+=:,\.\/\-]+$/) do
      value
    else
      "'" <> String.replace(value, "'", "'\''") <> "'"
    end
  end

  defp mismatches(manual, executor) do
    manual_by_step = Map.new(manual, &{&1.step_id, &1})
    executor_by_step = Map.new(executor, &{&1.step_id, &1})
    steps = (Map.keys(manual_by_step) ++ Map.keys(executor_by_step)) |> Enum.uniq() |> Enum.sort()

    Enum.flat_map(steps, fn step ->
      m = Map.get(manual_by_step, step)
      e = Map.get(executor_by_step, step)

      cond do
        m == nil ->
          [%{step_id: step, reason: :missing_manual}]

        e == nil ->
          [%{step_id: step, reason: :missing_executor}]

        :erlang.term_to_binary(m) != :erlang.term_to_binary(e) ->
          [
            %{
              step_id: step,
              reason: :byte_mismatch,
              manual_status: m.status,
              executor_status: e.status
            }
          ]

        true ->
          []
      end
    end)
  end

  defp notes(true, _),
    do: [
      "executor-driven output is byte-identical to the equivalent manual ouroforge CLI transcript"
    ]

  defp notes(false, _),
    do: ["mismatch is informational only and requires manual review before any go/no-go decision"]

  defp render_mismatches([]), do: "none"

  defp render_mismatches(mismatches),
    do: Enum.map_join(mismatches, "; ", &"#{&1.step_id}=#{&1.reason}")
end

defmodule OuroforgeExecutor.DemoCampaignTest do
  use ExUnit.Case, async: false

  alias OuroforgeExecutor.DemoCampaign

  @moduletag :demo

  test "executor-driven campaign has golden parity with manual Rust CLI path" do
    repo_root = Path.expand("../../../..", __DIR__)

    runner = fn "ouroforge", argv, opts ->
      command_opts = Keyword.merge([cd: repo_root, stderr_to_stdout: true], opts)
      System.cmd("cargo", ["run", "-q", "-p", "ouroforge-cli", "--" | argv], command_opts)
    end

    manual = DemoCampaign.run_manual(runner: runner)
    executor = DemoCampaign.run_executor(runner: runner)

    assert Enum.all?(manual, &(&1.status == 0))
    assert Enum.all?(executor, &(&1.status == 0))
    assert DemoCampaign.golden_parity?(manual, executor)

    audit = DemoCampaign.audit_trail(executor)
    assert Enum.map(audit, & &1.step_id) == ["seed-validate", "project-validate"]
    assert Enum.all?(audit, &String.contains?(&1.kernel_evidence.stdout, "valid"))
  end
end

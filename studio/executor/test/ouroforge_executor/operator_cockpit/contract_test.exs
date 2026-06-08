defmodule OuroforgeExecutor.OperatorCockpit.ContractTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.Contract

  test "M67-1 contract preserves read-only two-plane boundary" do
    contract = Contract.read_only_contract()

    assert contract.version == "m67-1"
    assert contract.control_plane == :elixir_otp
    assert contract.data_plane == :rust_kernel
    assert contract.rust_boundary == :frozen_ouroforge_cli
    assert contract.local_only?
    refute contract.hosted_dashboard?
    refute contract.remote_workers?
    refute contract.remote_telemetry?
    assert Contract.read_only?(contract)

    refute Contract.read_only?(%{contract | remote_workers?: true})
    refute Contract.read_only?(%{contract | hosted_dashboard?: true})
    refute Contract.read_only?(%{contract | remote_telemetry?: true})
    refute Contract.read_only?(%{contract | local_only?: false})
  end

  test "M67-1 contract enumerates all required local executor states" do
    assert Contract.supported_states() == [
             :normal,
             :waiting,
             :retrying,
             :budget_limited,
             :backpressured,
             :blocked
           ]
  end

  test "M67-1 forbids trusted write authority and keeps source traceability explicit" do
    for surface <- [
          :artifact,
          :ledger,
          :evidence,
          :trust_gradient,
          :apply,
          :release,
          :merge,
          :deploy
        ] do
      assert Contract.trusted_write_surface?(surface)
    end

    for source <- [
          :executor_state,
          :executor_telemetry,
          :ouroforge_cli_output,
          :executor_evidence_path
        ] do
      assert Contract.traceable_source?(source)
    end

    refute Contract.traceable_source?(:browser_button)
    assert Contract.ambiguous_status(:blocked) == :requires_human_judgment
    assert Contract.ambiguous_status(:budget_limited) == :requires_human_judgment
    assert Contract.ambiguous_status(:normal) == :operator_information_only
  end

  test "M67-1 summary is operator-readable and non-executable" do
    summary = Contract.render_summary()

    assert summary =~ "read-only local Studio UX"
    assert summary =~ "frozen ouroforge CLI"

    assert summary =~
             "No direct artifact, ledger, evidence, trust-gradient, apply, release, merge, or deploy writes"

    assert summary =~ "human judgment"
    assert summary =~ "byte-identical parity"
    assert summary =~ "no hosted dashboard, remote workers, or remote telemetry"
    refute summary =~ "execute now"
    refute summary =~ "approve automatically"
  end
end

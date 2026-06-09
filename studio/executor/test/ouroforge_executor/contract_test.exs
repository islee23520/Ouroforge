defmodule OuroforgeExecutor.ContractTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.Contract

  test "declares Elixir as control plane and Rust as data plane" do
    assert :rust_kernel == Contract.data_plane_owner()

    assert Contract.control_plane_responsibilities() == [
             :schedule,
             :supervise,
             :budget,
             :retry,
             :backpressure,
             :telemetry,
             :live_steering
           ]
  end

  test "allows only the frozen CLI drive surface" do
    assert Contract.allowed_cli?(["run"])
    assert Contract.allowed_cli?(["evaluate"])
    assert Contract.allowed_cli?(["mutation", "review"])
    assert Contract.allowed_cli?(["mutation", "apply-scene"])
    assert Contract.allowed_cli?(["ledger", "list"])
    assert Contract.allowed_cli?(["evidence", "list"])
    assert Contract.allowed_cli?(["loop", "step"])

    refute Contract.allowed_cli?(["ledger", "append"])
    refute Contract.allowed_cli?(["evidence", "add"])
  end

  test "forbids direct trusted artifact writes through append/add commands" do
    assert Contract.forbidden_cli?(["ledger", "append"])
    assert Contract.forbidden_cli?(["evidence", "add"])
  end

  test "requires byte-identical golden parity against the manual CLI path" do
    assert %{
             required: true,
             comparator: :byte_identical_artifacts,
             manual_path: :ouroforge_cli,
             executor_path: :otp_control_plane_driving_ouroforge_cli
           } = Contract.golden_parity()
  end
end

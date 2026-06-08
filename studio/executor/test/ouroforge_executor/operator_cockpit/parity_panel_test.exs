defmodule OuroforgeExecutor.OperatorCockpit.ParityPanelTest do
  use ExUnit.Case, async: true

  alias OuroforgeExecutor.OperatorCockpit.ParityPanel

  test "M67-6 reports byte-identical manual/executor CLI parity" do
    panel = ParityPanel.fixture(:matching)

    assert panel.version == "m67-6"
    assert panel.boundary == :read_only_golden_parity_panel
    assert panel.parity_status == :byte_identical
    assert panel.comparator == :byte_identical_output
    assert panel.artifact_truth == :rust_ouroforge_cli
    refute panel.human_judgment_required?
    refute panel.trusted_write_authority?
    assert panel.mismatches == []

    assert Enum.any?(
             panel.manual_fallback_commands,
             &String.starts_with?(&1, "ouroforge seed validate")
           )
  end

  test "M67-6 mismatches require manual review instead of self-certification" do
    panel = ParityPanel.fixture(:mismatch)

    assert panel.parity_status == :mismatch_requires_human_review
    assert panel.human_judgment_required?
    assert [%{reason: :byte_mismatch, step_id: "seed-validate"}] = panel.mismatches
  end

  test "M67-6 shell-quotes copy-only fallback command arguments" do
    panel =
      ParityPanel.from_transcripts(
        [
          %{
            step_id: "space-path",
            argv: ["seed", "validate", "path with spaces/seed's.yaml"],
            status: 0,
            stdout: "ok",
            evidence_ref: "seed.yaml"
          }
        ],
        [
          %{
            step_id: "space-path",
            argv: ["seed", "validate", "path with spaces/seed's.yaml"],
            status: 0,
            stdout: "ok",
            evidence_ref: "seed.yaml"
          }
        ]
      )

    assert panel.manual_fallback_commands == [
             "ouroforge seed validate 'path with spaces/seed'\''s.yaml'"
           ]
  end

  test "M67-6 render exposes copy-only manual fallback commands" do
    rendered = :matching |> ParityPanel.fixture() |> ParityPanel.render()

    assert rendered =~ "read-only golden/manual comparison"
    assert rendered =~ "Status: byte_identical"
    assert rendered =~ "Artifact truth: rust_ouroforge_cli"
    assert rendered =~ "Trusted writes: false"
    assert rendered =~ "Manual fallback commands: ouroforge seed validate"
    assert rendered =~ "byte-identical to the equivalent manual ouroforge CLI"
    refute rendered =~ "click to apply"
    refute rendered =~ "write ledger"
  end
end

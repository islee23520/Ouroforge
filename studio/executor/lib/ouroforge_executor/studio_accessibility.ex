defmodule OuroforgeExecutor.StudioAccessibility do
  @moduledoc """
  Local Studio accessibility, i18n, theme, and keyboard presentation model (#2087).

  This module is a pure Elixir control/presentation surface that a local Phoenix
  LiveView can render. It provides gettext-style labels, screen-reader metadata,
  theme tokens, and keyboard shortcut descriptions without writing artifacts,
  ledgers, evidence, locale files, theme files, source, scenes, or evaluator
  truth. Any write-affecting user preference remains intervention-as-evidence
  until routed through existing Rust gates.
  """

  @boundary "tool accessibility i18n themes keyboard; intervention-as-evidence; read + gated-write; Rust data plane validates and records write-affecting preferences; Elixir/OTP + Phoenix LiveView control + presentation only; gettext-style local catalog; ARIA and keyboard navigation; review/apply, scene/source-apply, evaluator, evidence/provenance gates required; no raw bypass; no command bridge; local-first CLI fallback; loop completes without human; fun/taste and release go/no-go remain human; #1 and #23 remain open"

  @locales %{
    "en" => %{
      "studio.title" => "Ouroforge Studio",
      "nav.skip_to_main" => "Skip to main content",
      "nav.command_palette" => "Open command palette",
      "panel.progress" => "Campaign progress",
      "panel.evidence" => "Evidence and provenance",
      "panel.gates" => "Gate status",
      "action.copy_cli" => "Copy CLI fallback command",
      "action.review_apply" => "Review through existing gates",
      "status.read_only" => "Read-only Studio surface",
      "status.gated_write" => "Write-affecting changes require Rust gates"
    },
    "ko" => %{
      "studio.title" => "오로포지 스튜디오",
      "nav.skip_to_main" => "본문으로 건너뛰기",
      "nav.command_palette" => "명령 팔레트 열기",
      "panel.progress" => "캠페인 진행",
      "panel.evidence" => "증거 및 출처",
      "panel.gates" => "게이트 상태",
      "action.copy_cli" => "CLI 대체 명령 복사",
      "action.review_apply" => "기존 게이트로 검토",
      "status.read_only" => "읽기 전용 Studio 화면",
      "status.gated_write" => "쓰기 영향 변경은 Rust 게이트가 필요"
    }
  }

  @themes %{
    "light" => %{
      name: "Light",
      foreground: "#111827",
      background: "#ffffff",
      accent: "#1d4ed8",
      focus_ring: "#f59e0b",
      contrast_ratio: 12.6,
      reduced_motion: false
    },
    "high-contrast" => %{
      name: "High Contrast",
      foreground: "#ffffff",
      background: "#000000",
      accent: "#ffff00",
      focus_ring: "#00ffff",
      contrast_ratio: 21.0,
      reduced_motion: true
    }
  }

  @shortcuts [
    %{id: "skip-main", key: "Tab", scope: "global", action: "focus-main", order: 1},
    %{id: "command-palette", key: "?", scope: "global", action: "open-command-palette", order: 2},
    %{id: "next-panel", key: "j", scope: "panels", action: "focus-next-panel", order: 3},
    %{id: "previous-panel", key: "k", scope: "panels", action: "focus-previous-panel", order: 4},
    %{id: "copy-cli", key: "c", scope: "focused-command", action: "copy-cli-reference", order: 5},
    %{
      id: "review-gate",
      key: "r",
      scope: "focused-proposal",
      action: "open-review-apply-status",
      order: 6
    }
  ]

  @focus_order [
    "skip-link",
    "command-palette",
    "campaign-progress",
    "evidence-provenance",
    "gate-status",
    "cli-fallback"
  ]

  defstruct schemaVersion: "ouroforge.studio-accessibility.v1",
            locale: "en",
            theme: "light",
            labels: %{},
            aria: %{},
            focusOrder: [],
            shortcuts: [],
            themeTokens: %{},
            boundary: @boundary,
            interventionAsEvidence: true,
            readGatedWrite: true,
            directArtifactWrite: false,
            studioTrustedWriteAuthority: false,
            elixirOwnsArtifactSemantics: false,
            commandBridge: false,
            humanRequiredForAutonomousLoop: false,
            cliFallbackSupported: true

  def boundary, do: @boundary
  def supported_locales, do: @locales |> Map.keys() |> Enum.sort()
  def supported_themes, do: @themes |> Map.keys() |> Enum.sort()
  def shortcut_catalog, do: @shortcuts
  def focus_order, do: @focus_order

  def render_model(attrs \\ %{}) when is_map(attrs) do
    locale = value(attrs, :locale) || "en"
    theme = value(attrs, :theme) || "light"

    model = %__MODULE__{
      locale: locale,
      theme: theme,
      labels: labels_for(locale),
      aria: aria_for(locale),
      focusOrder: @focus_order,
      shortcuts: @shortcuts,
      themeTokens: Map.get(@themes, theme),
      interventionAsEvidence: bool_value(attrs, :intervention_as_evidence, true),
      readGatedWrite: bool_value(attrs, :read_gated_write, true),
      directArtifactWrite: bool_value(attrs, :direct_artifact_write, false),
      studioTrustedWriteAuthority: bool_value(attrs, :studio_trusted_write_authority, false),
      elixirOwnsArtifactSemantics: bool_value(attrs, :elixir_owns_artifact_semantics, false),
      commandBridge: bool_value(attrs, :command_bridge, false),
      humanRequiredForAutonomousLoop:
        bool_value(attrs, :human_required_for_autonomous_loop, false),
      cliFallbackSupported: bool_value(attrs, :cli_fallback_supported, true)
    }

    with :ok <- validate(model) do
      {:ok, model}
    end
  end

  def validate(%__MODULE__{} = model) do
    cond do
      model.schemaVersion != "ouroforge.studio-accessibility.v1" ->
        {:error, :unsupported_schema}

      contains_raw_bypass?(model.locale) or contains_raw_bypass?(model.theme) ->
        {:error, :raw_bypass_forbidden}

      not Map.has_key?(@locales, model.locale) ->
        {:error, :unsupported_locale}

      not Map.has_key?(@themes, model.theme) or model.themeTokens == nil ->
        {:error, :unsupported_theme}

      not model.interventionAsEvidence or not model.readGatedWrite ->
        {:error, :not_intervention_evidence}

      model.directArtifactWrite or model.studioTrustedWriteAuthority or
        model.elixirOwnsArtifactSemantics or model.commandBridge ->
        {:error, :trusted_write_forbidden}

      model.humanRequiredForAutonomousLoop or not model.cliFallbackSupported ->
        {:error, :autonomy_or_cli_fallback_broken}

      not complete_labels?(model.labels) or not complete_aria?(model.aria) ->
        {:error, :incomplete_accessibility_labels}

      not keyboard_safe?(model.shortcuts, model.focusOrder) ->
        {:error, :unsafe_keyboard_model}

      model.themeTokens.contrast_ratio < 7.0 ->
        {:error, :insufficient_contrast}

      not boundary_complete?(model.boundary) ->
        {:error, :boundary_incomplete}

      true ->
        :ok
    end
  end

  def gettext(key, locale \\ "en") when is_binary(key) and is_binary(locale) do
    @locales
    |> Map.get(locale, @locales["en"])
    |> Map.get(key, @locales["en"][key] || key)
  end

  def behavior_signature(%__MODULE__{} = model) do
    %{
      semantic_actions: Enum.map(model.shortcuts, & &1.action),
      focus_order: model.focusOrder,
      trusted_write_authority: false,
      rust_data_plane_required: true,
      cli_fallback_supported: true
    }
  end

  defp labels_for(locale) do
    @locales["en"]
    |> Map.keys()
    |> Map.new(fn key -> {key, gettext(key, locale)} end)
  end

  defp aria_for(locale) do
    %{
      "skip-link" => gettext("nav.skip_to_main", locale),
      "command-palette" => gettext("nav.command_palette", locale),
      "campaign-progress" => gettext("panel.progress", locale),
      "evidence-provenance" => gettext("panel.evidence", locale),
      "gate-status" => gettext("panel.gates", locale),
      "cli-fallback" => gettext("action.copy_cli", locale)
    }
  end

  defp complete_labels?(labels) when is_map(labels) do
    @locales["en"]
    |> Map.keys()
    |> Enum.all?(&(is_binary(labels[&1]) and labels[&1] != ""))
  end

  defp complete_labels?(_), do: false

  defp complete_aria?(aria) when is_map(aria) do
    Enum.all?(@focus_order, &(is_binary(aria[&1]) and aria[&1] != ""))
  end

  defp complete_aria?(_), do: false

  defp keyboard_safe?(shortcuts, focus_order) when is_list(shortcuts) and is_list(focus_order) do
    shortcut_ids = Enum.map(shortcuts, & &1.id)
    keys_by_scope = Enum.map(shortcuts, &{&1.scope, &1.key})

    shortcut_ids == Enum.uniq(shortcut_ids) and keys_by_scope == Enum.uniq(keys_by_scope) and
      focus_order == Enum.uniq(focus_order) and "skip-link" in focus_order and
      Enum.all?(shortcuts, &valid_shortcut?/1)
  end

  defp keyboard_safe?(_, _), do: false

  defp valid_shortcut?(shortcut) do
    Enum.all?([shortcut.id, shortcut.key, shortcut.scope, shortcut.action], &present?/1) and
      is_integer(shortcut.order)
  end

  defp boundary_complete?(boundary) when is_binary(boundary) do
    Enum.all?(
      [
        "tool accessibility i18n themes keyboard",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust data plane validates and records",
        "Elixir/OTP + Phoenix LiveView control + presentation",
        "gettext-style local catalog",
        "ARIA and keyboard navigation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "no raw bypass",
        "no command bridge",
        "local-first CLI fallback",
        "loop completes without human",
        "fun/taste and release go/no-go remain human",
        "#1 and #23 remain open"
      ],
      &String.contains?(boundary, &1)
    )
  end

  defp boundary_complete?(_), do: false

  defp contains_raw_bypass?(value) when is_binary(value) do
    String.contains?(String.downcase(value), ["raw_write_bypass", "raw_apply_bypass"])
  end

  defp contains_raw_bypass?(_), do: false

  defp present?(value), do: is_binary(value) and value != ""

  defp bool_value(attrs, key, default) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key)) || Map.get(attrs, camelize(key)) ||
      default
  end

  defp value(attrs, key) do
    Map.get(attrs, key) || Map.get(attrs, Atom.to_string(key)) || Map.get(attrs, camelize(key))
  end

  defp camelize(key) do
    key
    |> Atom.to_string()
    |> String.split("_")
    |> then(fn [head | tail] -> head <> Enum.map_join(tail, "", &String.capitalize/1) end)
  end
end

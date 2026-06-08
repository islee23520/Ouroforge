//! Scoring modifier/effect model contract (#1799).
//!
//! This locks readable one-line modifier effects over the existing
//! card-roguelite substrate. The model is Rust/local validation data only: it is
//! not a parallel engine, browser/Studio trusted write surface, or automated fun
//! verdict.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    resolve_card_roguelite_state, validate_card_roguelite_config, CardRogueliteConfig,
    CardRogueliteModifierEffectOperation, CardRogueliteModifierEffectScope,
};

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture_config(relative: &str) -> CardRogueliteConfig {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn modifier_effects_parse_as_readable_one_line_substrate_data() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/modifier-effects.valid.json",
    );

    validate_card_roguelite_config(&config).expect("readable effect config validates");
    let tuned = config
        .modifiers
        .get("tuned")
        .expect("tuned modifier exists");
    let effect = tuned.effect.as_ref().expect("tuned has readable effect");

    assert_eq!(effect.text, "add +3 before multipliers");
    assert_eq!(effect.scope, CardRogueliteModifierEffectScope::Card);
    assert_eq!(
        effect.operation,
        CardRogueliteModifierEffectOperation::Additive
    );
    assert!(!effect.text.contains('\n'), "effect is one readable line");
}

#[test]
fn readable_effects_do_not_change_ordered_substrate_scoring() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/modifier-effects.valid.json",
    );

    let first = resolve_card_roguelite_state(&config).expect("scoring resolves");
    let second = resolve_card_roguelite_state(&config).expect("scoring repeats");

    assert_eq!(first.score, 16, "(base 5 + tuned 3) * overdrive 2");
    assert_eq!(
        first.digest, second.digest,
        "effect metadata remains deterministic"
    );
    assert_eq!(
        first.read_only_inspection.trusted_emitter,
        "rust-card-roguelite-substrate"
    );
    assert!(first
        .read_only_inspection
        .browser_studio_mode
        .contains("read-only"));
}

#[test]
fn malformed_or_opaque_effect_text_is_rejected_fail_closed() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/modifier-effects.malformed.json",
    );

    let error = validate_card_roguelite_config(&config)
        .expect_err("multi-line opaque effect must fail closed")
        .to_string();

    assert!(
        error.contains("effect text") && error.contains("one readable ASCII line"),
        "unexpected error: {error}"
    );
    assert!(resolve_card_roguelite_state(&config).is_err());
}

#[test]
fn effect_text_must_match_declared_operation() {
    let mut config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/modifier-effects.valid.json",
    );
    config
        .modifiers
        .get_mut("overdrive")
        .expect("overdrive exists")
        .effect
        .as_mut()
        .expect("overdrive effect exists")
        .text = "add a small starter bonus".to_string();

    let error = validate_card_roguelite_config(&config)
        .expect_err("operation/text drift must fail closed")
        .to_string();

    assert!(
        error.contains("declared operation"),
        "unexpected error: {error}"
    );
}

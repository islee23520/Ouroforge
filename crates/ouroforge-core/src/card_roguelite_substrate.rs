//! Card-Roguelite Substrate v1 core model (#1792).
//!
//! The substrate is an additive Rust/local abstraction over the existing
//! deck-roguelike class (#1601) and seeded RNG contract (#1600). It models cards,
//! modifiers, deterministic resolution, run/ante, shop, seed, and meta
//! declarations as validated configuration. Variants are configs over this
//! substrate; this module is not a parallel browser/Studio engine and grants no
//! trusted write authority.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fnv1a64;
use crate::seeded_rng::{SeededRng, SeededRngState, SEEDED_RNG_ALGORITHM};

pub const CARD_ROGUELITE_SUBSTRATE_CONFIG_SCHEMA_VERSION: &str =
    "ouroforge.card-roguelite-substrate-config.v1";
pub const CARD_ROGUELITE_SUBSTRATE_STATE_SCHEMA_VERSION: &str =
    "ouroforge.card-roguelite-substrate-state.v1";
pub const CARD_ROGUELITE_SUBSTRATE_PROBE_SCHEMA_VERSION: &str =
    "ouroforge.card-roguelite-substrate-probe.v1";
pub const CARD_ROGUELITE_SUBSTRATE_DIGEST_ALGORITHM: &str = "fnv1a64-canonical-json-v1";

const MAX_CARDS: usize = 128;
const MAX_MODIFIERS: usize = 128;
const MAX_DECK: usize = 128;
const MAX_SHOP_OFFERS: usize = 16;
const MAX_META_UNLOCKS: usize = 64;
const MAX_MODIFIER_EFFECT_TEXT_CHARS: usize = 96;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteConfig {
    pub schema_version: String,
    pub config_id: String,
    pub variant: String,
    pub seed: u32,
    pub cards: BTreeMap<String, CardRogueliteCard>,
    pub starting_deck: Vec<String>,
    #[serde(default)]
    pub modifiers: BTreeMap<String, CardRogueliteModifier>,
    pub run: CardRogueliteRunConfig,
    pub shop: CardRogueliteShopConfig,
    #[serde(default)]
    pub meta: CardRogueliteMetaConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteCard {
    pub cost: u32,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub actions: Vec<CardRogueliteEffect>,
    #[serde(default)]
    pub modifier_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CardRogueliteEffect {
    Damage { amount: i32 },
    Block { amount: i32 },
    Score { amount: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteModifier {
    pub order: i32,
    #[serde(default)]
    pub add_score: i32,
    #[serde(default = "one")]
    pub multiply_score: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<CardRogueliteModifierEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteModifierEffect {
    pub text: String,
    pub scope: CardRogueliteModifierEffectScope,
    pub operation: CardRogueliteModifierEffectOperation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardRogueliteModifierEffectScope {
    Card,
    Tag,
    Hand,
    Deck,
    Run,
    Shop,
    ScoringPhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardRogueliteModifierEffectOperation {
    Additive,
    Multiplicative,
    Clamp,
    Selector,
    Gate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteRunConfig {
    pub starting_hp: i32,
    pub starting_quota: i32,
    pub ante_steps: Vec<CardRogueliteAnteStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteAnteStep {
    pub ante: u32,
    pub quota: i32,
    pub reward_gold: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteShopConfig {
    pub base_gold: i32,
    pub offer_count: usize,
    pub price_floor: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteMetaConfig {
    #[serde(default)]
    pub unlocks: Vec<CardRogueliteUnlock>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteUnlock {
    pub id: String,
    pub required_ante: u32,
    pub card_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteState {
    pub schema_version: String,
    pub config_id: String,
    pub variant: String,
    pub seed: u32,
    pub rng: SeededRngState,
    pub ante: u32,
    pub quota: i32,
    pub hp: i32,
    pub gold: i32,
    pub deck: Vec<String>,
    pub shop_offers: Vec<CardRogueliteShopOffer>,
    pub score: i32,
    pub status: CardRogueliteStatus,
    pub digest: CardRogueliteDigest,
    pub read_only_inspection: CardRogueliteReadOnlyInspection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardRogueliteStatus {
    Ready,
    Won,
    Lost,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteShopOffer {
    pub card_id: String,
    pub price: i32,
    pub roll: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardRogueliteDigest {
    pub algorithm: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteReadOnlyInspection {
    pub trusted_emitter: String,
    pub browser_studio_mode: String,
    pub disallowed_actions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRogueliteProbeState {
    pub schema_version: String,
    pub substrate_state: CardRogueliteState,
    pub digest: CardRogueliteDigest,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DigestInput<'a> {
    schema_version: &'a str,
    config_id: &'a str,
    variant: &'a str,
    seed: u32,
    rng: SeededRngState,
    ante: u32,
    quota: i32,
    hp: i32,
    gold: i32,
    deck: &'a [String],
    shop_offers: &'a [CardRogueliteShopOffer],
    score: i32,
    status: CardRogueliteStatus,
}

fn one() -> i32 {
    1
}

pub fn deck_roguelike_spec_to_substrate_config(spec: &Value) -> Result<CardRogueliteConfig> {
    let obj = spec
        .as_object()
        .ok_or_else(|| anyhow!("deck roguelike spec must be an object"))?;
    if spec.get("schemaVersion").and_then(Value::as_str) != Some("ouroforge.deck-roguelike.v1") {
        return Err(anyhow!("schemaVersion must be ouroforge.deck-roguelike.v1"));
    }
    let seed = spec.get("seed").and_then(Value::as_u64).unwrap_or(0) as u32;
    let config_id = spec
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("deck-roguelike-classic")
        .to_string();
    let cards_obj = spec
        .get("cards")
        .and_then(Value::as_object)
        .filter(|cards| !cards.is_empty())
        .ok_or_else(|| anyhow!("cards vocabulary must be a non-empty object"))?;
    let mut cards = BTreeMap::new();
    for (id, card) in cards_obj {
        let card_obj = card
            .as_object()
            .ok_or_else(|| anyhow!("card {id} must be an object"))?;
        let card_type = card_obj
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("card {id} must declare type"))?;
        let cost = card_obj
            .get("cost")
            .and_then(Value::as_u64)
            .ok_or_else(|| anyhow!("card {id} cost must be a non-negative integer"))?
            as u32;
        let (tag, effect) = match card_type {
            "attack" => (
                "attack".to_string(),
                CardRogueliteEffect::Damage {
                    amount: card_obj
                        .get("damage")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("attack card {id} must declare damage"))?
                        as i32,
                },
            ),
            "skill" => (
                "skill".to_string(),
                CardRogueliteEffect::Block {
                    amount: card_obj
                        .get("block")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("skill card {id} must declare block"))?
                        as i32,
                },
            ),
            other => return Err(anyhow!("card {id} has unknown type {other}")),
        };
        cards.insert(
            id.clone(),
            CardRogueliteCard {
                cost,
                tags: vec![tag, "deck-roguelike-classic".to_string()],
                actions: vec![effect],
                modifier_refs: Vec::new(),
            },
        );
    }
    let starting_deck = spec
        .get("deck")
        .and_then(Value::as_array)
        .filter(|deck| !deck.is_empty())
        .ok_or_else(|| anyhow!("deck must be a non-empty array of card ids"))?
        .iter()
        .map(|card_id| {
            card_id
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| anyhow!("deck card id must be a string"))
        })
        .collect::<Result<Vec<_>>>()?;
    let player_hp =
        spec.get("player")
            .and_then(|player| player.get("maxHp"))
            .and_then(Value::as_i64)
            .ok_or_else(|| anyhow!("player.maxHp must be a positive integer"))? as i32;
    let enemy_hp =
        spec.get("enemy")
            .and_then(|enemy| enemy.get("maxHp"))
            .and_then(Value::as_i64)
            .ok_or_else(|| anyhow!("enemy.maxHp must be a positive integer"))? as i32;
    let config = CardRogueliteConfig {
        schema_version: CARD_ROGUELITE_SUBSTRATE_CONFIG_SCHEMA_VERSION.to_string(),
        config_id,
        variant: "deck-roguelike-classic".to_string(),
        seed,
        cards,
        starting_deck,
        modifiers: BTreeMap::new(),
        run: CardRogueliteRunConfig {
            starting_hp: player_hp,
            starting_quota: enemy_hp,
            ante_steps: vec![CardRogueliteAnteStep {
                ante: 1,
                quota: enemy_hp,
                reward_gold: 0,
            }],
        },
        shop: CardRogueliteShopConfig {
            base_gold: 0,
            offer_count: 1,
            price_floor: 0,
        },
        meta: CardRogueliteMetaConfig::default(),
    };
    let _ = obj;
    validate_card_roguelite_config(&config)?;
    Ok(config)
}

pub fn validate_card_roguelite_config(config: &CardRogueliteConfig) -> Result<()> {
    if config.schema_version != CARD_ROGUELITE_SUBSTRATE_CONFIG_SCHEMA_VERSION {
        return Err(anyhow!(
            "schemaVersion must be {}",
            CARD_ROGUELITE_SUBSTRATE_CONFIG_SCHEMA_VERSION
        ));
    }
    validate_id("configId", &config.config_id)?;
    validate_id("variant", &config.variant)?;
    if config.cards.is_empty() || config.cards.len() > MAX_CARDS {
        return Err(anyhow!("cards must contain 1..={MAX_CARDS} entries"));
    }
    if config.modifiers.len() > MAX_MODIFIERS {
        return Err(anyhow!("modifiers must not exceed {MAX_MODIFIERS}"));
    }
    if config.starting_deck.is_empty() || config.starting_deck.len() > MAX_DECK {
        return Err(anyhow!(
            "startingDeck must contain 1..={MAX_DECK} card refs"
        ));
    }
    for (card_id, card) in &config.cards {
        validate_id("card id", card_id)?;
        for tag in &card.tags {
            validate_id("card tag", tag)?;
        }
        for modifier_ref in &card.modifier_refs {
            if !config.modifiers.contains_key(modifier_ref) {
                return Err(anyhow!(
                    "card {card_id} references undeclared modifier {modifier_ref}"
                ));
            }
        }
    }
    for card_id in &config.starting_deck {
        if !config.cards.contains_key(card_id) {
            return Err(anyhow!("startingDeck references undeclared card {card_id}"));
        }
    }
    for (modifier_id, modifier) in &config.modifiers {
        validate_id("modifier id", modifier_id)?;
        if modifier.multiply_score < 1 {
            return Err(anyhow!("modifier {modifier_id} multiplyScore must be >= 1"));
        }
        if let Some(effect) = &modifier.effect {
            validate_modifier_effect(modifier_id, effect)?;
        }
    }
    if config.run.starting_hp <= 0 {
        return Err(anyhow!("run.startingHp must be positive"));
    }
    if config.run.starting_quota <= 0 {
        return Err(anyhow!("run.startingQuota must be positive"));
    }
    if config.run.ante_steps.is_empty() {
        return Err(anyhow!("run.anteSteps must not be empty"));
    }
    let mut seen_antes = BTreeSet::new();
    for step in &config.run.ante_steps {
        if step.ante == 0 || !seen_antes.insert(step.ante) {
            return Err(anyhow!(
                "run.anteSteps ante values must be unique positive integers"
            ));
        }
        if step.quota <= 0 || step.reward_gold < 0 {
            return Err(anyhow!(
                "run.anteSteps must use positive quota and non-negative rewardGold"
            ));
        }
    }
    if config.shop.base_gold < 0 || config.shop.price_floor < 0 {
        return Err(anyhow!("shop gold and price floors must be non-negative"));
    }
    if config.shop.offer_count == 0 || config.shop.offer_count > MAX_SHOP_OFFERS {
        return Err(anyhow!("shop.offerCount must be 1..={MAX_SHOP_OFFERS}"));
    }
    if config.meta.unlocks.len() > MAX_META_UNLOCKS {
        return Err(anyhow!("meta.unlocks must not exceed {MAX_META_UNLOCKS}"));
    }
    let mut unlocks = BTreeSet::new();
    for unlock in &config.meta.unlocks {
        validate_id("unlock id", &unlock.id)?;
        if !unlocks.insert(&unlock.id) {
            return Err(anyhow!("duplicate meta unlock {}", unlock.id));
        }
        if unlock.required_ante == 0 || !config.cards.contains_key(&unlock.card_ref) {
            return Err(anyhow!(
                "meta unlock {} must use a positive requiredAnte and declared cardRef",
                unlock.id
            ));
        }
    }
    Ok(())
}

pub fn resolve_card_roguelite_state(config: &CardRogueliteConfig) -> Result<CardRogueliteState> {
    validate_card_roguelite_config(config)?;
    let mut rng = SeededRng::new(config.seed);
    let deck = shuffle_refs(&config.starting_deck, &mut rng);
    let score = deck
        .iter()
        .filter_map(|card_id| config.cards.get(card_id))
        .map(|card| resolve_card_score(card, &config.modifiers))
        .sum::<i32>();
    let ante_step = config
        .run
        .ante_steps
        .iter()
        .min_by_key(|step| step.ante)
        .expect("validated non-empty ante steps");
    let mut offers = Vec::with_capacity(config.shop.offer_count);
    let card_ids = config.cards.keys().cloned().collect::<Vec<_>>();
    for _ in 0..config.shop.offer_count {
        let roll = rng.next_raw();
        let card_id = card_ids[(roll as usize) % card_ids.len()].clone();
        let price = (config.shop.price_floor + config.cards[&card_id].cost as i32)
            .max(config.shop.price_floor);
        offers.push(CardRogueliteShopOffer {
            card_id,
            price,
            roll,
        });
    }
    let mut state = CardRogueliteState {
        schema_version: CARD_ROGUELITE_SUBSTRATE_STATE_SCHEMA_VERSION.to_string(),
        config_id: config.config_id.clone(),
        variant: config.variant.clone(),
        seed: config.seed,
        rng: rng.capture(),
        ante: ante_step.ante,
        quota: ante_step.quota.max(config.run.starting_quota),
        hp: config.run.starting_hp,
        gold: config.shop.base_gold + ante_step.reward_gold,
        deck,
        shop_offers: offers,
        score,
        status: if score >= ante_step.quota {
            CardRogueliteStatus::Won
        } else {
            CardRogueliteStatus::Ready
        },
        digest: CardRogueliteDigest {
            algorithm: CARD_ROGUELITE_SUBSTRATE_DIGEST_ALGORITHM.to_string(),
            value: String::new(),
        },
        read_only_inspection: CardRogueliteReadOnlyInspection {
            trusted_emitter: "rust-card-roguelite-substrate".to_string(),
            browser_studio_mode: "read-only card-roguelite substrate inspection".to_string(),
            disallowed_actions: vec![
                "trusted writes".to_string(),
                "command bridge".to_string(),
                "live mutation".to_string(),
                "automated fun verdict".to_string(),
            ],
        },
    };
    state.digest = digest_state(&state);
    Ok(state)
}

pub fn card_roguelite_probe_state(config: &CardRogueliteConfig) -> Result<CardRogueliteProbeState> {
    let substrate_state = resolve_card_roguelite_state(config)?;
    Ok(CardRogueliteProbeState {
        schema_version: CARD_ROGUELITE_SUBSTRATE_PROBE_SCHEMA_VERSION.to_string(),
        digest: substrate_state.digest.clone(),
        substrate_state,
    })
}

pub fn digest_card_roguelite_state(state: &CardRogueliteState) -> CardRogueliteDigest {
    digest_state(state)
}

pub fn default_deck_roguelike_substrate_config(seed: u32) -> CardRogueliteConfig {
    let mut cards = BTreeMap::new();
    cards.insert(
        "strike".to_string(),
        CardRogueliteCard {
            cost: 1,
            tags: vec!["attack".to_string(), "classic".to_string()],
            actions: vec![CardRogueliteEffect::Damage { amount: 6 }],
            modifier_refs: Vec::new(),
        },
    );
    cards.insert(
        "defend".to_string(),
        CardRogueliteCard {
            cost: 1,
            tags: vec!["skill".to_string(), "classic".to_string()],
            actions: vec![CardRogueliteEffect::Block { amount: 5 }],
            modifier_refs: Vec::new(),
        },
    );
    cards.insert(
        "bash".to_string(),
        CardRogueliteCard {
            cost: 2,
            tags: vec!["attack".to_string(), "classic".to_string()],
            actions: vec![CardRogueliteEffect::Damage { amount: 8 }],
            modifier_refs: Vec::new(),
        },
    );
    CardRogueliteConfig {
        schema_version: CARD_ROGUELITE_SUBSTRATE_CONFIG_SCHEMA_VERSION.to_string(),
        config_id: "deck-roguelike-classic".to_string(),
        variant: "deck-roguelike-classic".to_string(),
        seed,
        cards,
        starting_deck: vec![
            "strike".to_string(),
            "strike".to_string(),
            "strike".to_string(),
            "strike".to_string(),
            "strike".to_string(),
            "bash".to_string(),
            "bash".to_string(),
            "defend".to_string(),
            "defend".to_string(),
            "defend".to_string(),
        ],
        modifiers: BTreeMap::new(),
        run: CardRogueliteRunConfig {
            starting_hp: 30,
            starting_quota: 20,
            ante_steps: vec![CardRogueliteAnteStep {
                ante: 1,
                quota: 20,
                reward_gold: 10,
            }],
        },
        shop: CardRogueliteShopConfig {
            base_gold: 99,
            offer_count: 3,
            price_floor: 1,
        },
        meta: CardRogueliteMetaConfig::default(),
    }
}

pub fn default_engine_builder_deckbuilder_substrate_config(seed: u32) -> CardRogueliteConfig {
    let mut modifiers = BTreeMap::new();
    modifiers.insert(
        "tuned".to_string(),
        CardRogueliteModifier {
            order: 10,
            add_score: 2,
            multiply_score: 1,
            effect: Some(CardRogueliteModifierEffect {
                text: "add +2 before multipliers".to_string(),
                scope: CardRogueliteModifierEffectScope::Card,
                operation: CardRogueliteModifierEffectOperation::Additive,
            }),
        },
    );
    modifiers.insert(
        "overdrive".to_string(),
        CardRogueliteModifier {
            order: 20,
            add_score: 0,
            multiply_score: 2,
            effect: Some(CardRogueliteModifierEffect {
                text: "double score after additive tuning".to_string(),
                scope: CardRogueliteModifierEffectScope::ScoringPhase,
                operation: CardRogueliteModifierEffectOperation::Multiplicative,
            }),
        },
    );

    let mut cards = BTreeMap::new();
    cards.insert(
        "spark-plug".to_string(),
        CardRogueliteCard {
            cost: 1,
            tags: vec![
                "engine-builder".to_string(),
                "starter".to_string(),
                "score".to_string(),
            ],
            actions: vec![CardRogueliteEffect::Score { amount: 4 }],
            modifier_refs: vec!["tuned".to_string()],
        },
    );
    cards.insert(
        "coolant-loop".to_string(),
        CardRogueliteCard {
            cost: 1,
            tags: vec![
                "engine-builder".to_string(),
                "starter".to_string(),
                "stabilizer".to_string(),
            ],
            actions: vec![CardRogueliteEffect::Block { amount: 6 }],
            modifier_refs: Vec::new(),
        },
    );
    cards.insert(
        "gear-train".to_string(),
        CardRogueliteCard {
            cost: 2,
            tags: vec![
                "engine-builder".to_string(),
                "starter".to_string(),
                "scaler".to_string(),
            ],
            actions: vec![CardRogueliteEffect::Score { amount: 7 }],
            modifier_refs: vec!["overdrive".to_string()],
        },
    );
    cards.insert(
        "pressure-valve".to_string(),
        CardRogueliteCard {
            cost: 2,
            tags: vec![
                "engine-builder".to_string(),
                "shop".to_string(),
                "control".to_string(),
            ],
            actions: vec![CardRogueliteEffect::Damage { amount: 5 }],
            modifier_refs: vec!["tuned".to_string()],
        },
    );
    cards.insert(
        "blueprint-cache".to_string(),
        CardRogueliteCard {
            cost: 3,
            tags: vec![
                "engine-builder".to_string(),
                "unlock".to_string(),
                "planning".to_string(),
            ],
            actions: vec![CardRogueliteEffect::Score { amount: 9 }],
            modifier_refs: Vec::new(),
        },
    );

    CardRogueliteConfig {
        schema_version: CARD_ROGUELITE_SUBSTRATE_CONFIG_SCHEMA_VERSION.to_string(),
        config_id: "engine-builder-deckbuilder".to_string(),
        variant: "engine-builder-deckbuilder".to_string(),
        seed,
        cards,
        starting_deck: vec![
            "spark-plug".to_string(),
            "spark-plug".to_string(),
            "spark-plug".to_string(),
            "coolant-loop".to_string(),
            "coolant-loop".to_string(),
            "gear-train".to_string(),
            "gear-train".to_string(),
            "pressure-valve".to_string(),
        ],
        modifiers,
        run: CardRogueliteRunConfig {
            starting_hp: 24,
            starting_quota: 48,
            ante_steps: vec![
                CardRogueliteAnteStep {
                    ante: 1,
                    quota: 48,
                    reward_gold: 12,
                },
                CardRogueliteAnteStep {
                    ante: 2,
                    quota: 72,
                    reward_gold: 18,
                },
            ],
        },
        shop: CardRogueliteShopConfig {
            base_gold: 35,
            offer_count: 4,
            price_floor: 2,
        },
        meta: CardRogueliteMetaConfig {
            unlocks: vec![CardRogueliteUnlock {
                id: "blueprint-cache".to_string(),
                required_ante: 2,
                card_ref: "blueprint-cache".to_string(),
            }],
        },
    }
}

fn resolve_card_score(
    card: &CardRogueliteCard,
    modifiers: &BTreeMap<String, CardRogueliteModifier>,
) -> i32 {
    let base = card
        .actions
        .iter()
        .map(|effect| match effect {
            CardRogueliteEffect::Damage { amount }
            | CardRogueliteEffect::Block { amount }
            | CardRogueliteEffect::Score { amount } => *amount,
        })
        .sum::<i32>();
    let mut ordered = card
        .modifier_refs
        .iter()
        .filter_map(|modifier_ref| modifiers.get(modifier_ref))
        .collect::<Vec<_>>();
    ordered.sort_by_key(|modifier| modifier.order);
    ordered.into_iter().fold(base, |score, modifier| {
        (score + modifier.add_score) * modifier.multiply_score
    })
}

fn shuffle_refs(cards: &[String], rng: &mut SeededRng) -> Vec<String> {
    let mut result = cards.to_vec();
    for i in (1..result.len()).rev() {
        let j = (rng.next_raw() as usize) % (i + 1);
        result.swap(i, j);
    }
    result
}

fn digest_state(state: &CardRogueliteState) -> CardRogueliteDigest {
    let canonical = serde_json::to_vec(&DigestInput {
        schema_version: &state.schema_version,
        config_id: &state.config_id,
        variant: &state.variant,
        seed: state.seed,
        rng: state.rng,
        ante: state.ante,
        quota: state.quota,
        hp: state.hp,
        gold: state.gold,
        deck: &state.deck,
        shop_offers: &state.shop_offers,
        score: state.score,
        status: state.status,
    })
    .unwrap_or_default();
    CardRogueliteDigest {
        algorithm: CARD_ROGUELITE_SUBSTRATE_DIGEST_ALGORITHM.to_string(),
        value: format!("{:016x}", fnv1a64(&canonical)),
    }
}

fn validate_modifier_effect(modifier_id: &str, effect: &CardRogueliteModifierEffect) -> Result<()> {
    let text = effect.text.trim();
    if text != effect.text {
        return Err(anyhow!(
            "modifier {modifier_id} effect text must not have leading or trailing whitespace"
        ));
    }
    if text.is_empty()
        || text.chars().count() > MAX_MODIFIER_EFFECT_TEXT_CHARS
        || text.contains('\n')
        || text.contains('\r')
        || !text.chars().all(|ch| ch.is_ascii_graphic() || ch == ' ')
    {
        return Err(anyhow!(
            "modifier {modifier_id} effect text must be one readable ASCII line up to {MAX_MODIFIER_EFFECT_TEXT_CHARS} chars"
        ));
    }
    if text.split_whitespace().count() < 2 {
        return Err(anyhow!(
            "modifier {modifier_id} effect text must be individually legible"
        ));
    }
    let lower = text.to_ascii_lowercase();
    let blocked_tokens = [
        "http://",
        "https://",
        "javascript:",
        "<script",
        "shell",
        "exec",
        "write file",
        "auto-merge",
        "fun score",
        "production-ready",
    ];
    if blocked_tokens.iter().any(|token| lower.contains(token)) {
        return Err(anyhow!(
            "modifier {modifier_id} effect text crosses the readable mechanical boundary"
        ));
    }
    let operation_hint_present = match effect.operation {
        CardRogueliteModifierEffectOperation::Additive => {
            lower.contains("add") || lower.contains('+')
        }
        CardRogueliteModifierEffectOperation::Multiplicative => {
            lower.contains("double")
                || lower.contains("multiply")
                || lower.contains("multiplier")
                || lower.contains("times")
                || lower.contains('x')
        }
        CardRogueliteModifierEffectOperation::Clamp => {
            lower.contains("clamp")
                || lower.contains("cap")
                || lower.contains("floor")
                || lower.contains("limit")
        }
        CardRogueliteModifierEffectOperation::Selector => {
            lower.contains("select")
                || lower.contains("card")
                || lower.contains("tag")
                || lower.contains("matching")
        }
        CardRogueliteModifierEffectOperation::Gate => {
            lower.contains("if") || lower.contains("when") || lower.contains("unless")
        }
    };
    if !operation_hint_present {
        return Err(anyhow!(
            "modifier {modifier_id} effect text must describe its declared operation"
        ));
    }
    Ok(())
}

fn validate_id(label: &str, value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(anyhow!(
            "{label} must be a non-empty lower-case id using [a-z0-9_-]"
        ));
    }
    Ok(())
}

pub fn card_roguelite_seed_algorithm() -> &'static str {
    SEEDED_RNG_ALGORITHM
}

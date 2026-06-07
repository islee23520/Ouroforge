//! Contract test for the Deck-Roguelike Game Class v1 (#1601).
//!
//! Rust mirror of the runtime test
//! `examples/game-runtime/deck-roguelike.test.cjs`. It re-derives the
//! deterministic deck-roguelike semantics in trusted Rust over the shared
//! fixtures and machine-checks the genre's acceptance properties rather than
//! asserting them: a seeded run is reproducible (digest-stable for an identical
//! seed and action sequence), a different seed shuffles differently, the seeded
//! run is winnable and observable, and a malformed deck fails closed.
//!
//! Per the Era F language boundary, the trusted validation/solver logic is owned
//! by Rust/local; the JavaScript runtime reproduces the same observable behavior
//! for the browser-local probe. The shuffle reuses the seeded stochastic
//! determinism layer (#1600): the same mulberry32 stream, so runs are
//! seed-reproducible and replay-stable.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_scene(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

fn deck_spec(scene_fixture: &str) -> Value {
    read_scene(scene_fixture)["deckRoguelike"].clone()
}

// --- Seeded mulberry32 stream (identical to the runtime seeded-rng layer) ------

const RNG_INCREMENT: u32 = 0x6d2b79f5;

#[derive(Clone, Debug)]
struct Rng {
    seed: u32,
    state: u32,
    draw_count: u32,
}

impl Rng {
    fn new(seed: u32) -> Self {
        Rng {
            seed,
            state: seed,
            draw_count: 0,
        }
    }

    fn next_raw(&mut self) -> u32 {
        self.state = self.state.wrapping_add(RNG_INCREMENT);
        let mut t = self.state;
        t = (t ^ (t >> 15)).wrapping_mul(1 | t);
        t = (t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t))) ^ t;
        let raw = t ^ (t >> 14);
        self.draw_count += 1;
        raw
    }

    fn next_below(&mut self, bound: u32) -> u32 {
        if bound <= 1 {
            return 0;
        }
        self.next_raw() % bound
    }
}

fn shuffle(cards: &[String], rng: &mut Rng) -> Vec<String> {
    let mut result = cards.to_vec();
    let mut i = result.len();
    while i > 1 {
        i -= 1;
        let j = rng.next_below((i + 1) as u32) as usize;
        result.swap(i, j);
    }
    result
}

// --- Validated deck-roguelike state (mirror of the runtime module) -------------

#[derive(Clone, Debug)]
struct Card {
    kind: String,
    cost: u32,
    damage: u32,
    block: u32,
}

#[derive(Clone, Debug)]
struct Intent {
    value: u32,
}

#[derive(Clone, Debug)]
struct DeckState {
    rng: Rng,
    cards: HashMap<String, Card>,
    player_hp: u32,
    player_block: u32,
    player_energy: u32,
    energy_per_turn: u32,
    bonus_energy: u32,
    hand_size: usize,
    relics: Vec<String>,
    relic_vocabulary: HashMap<String, (String, u32, u32)>, // trigger, block, energy
    enemy_hp: u32,
    enemy_block: u32,
    intents: Vec<Intent>,
    intent_index: usize,
    draw_pile: Vec<String>,
    hand: Vec<String>,
    discard_pile: Vec<String>,
    turn: u32,
    status: String,
}

fn as_pos_u32(value: &Value) -> Option<u32> {
    value.as_u64().filter(|v| *v > 0).map(|v| v as u32)
}

fn as_nonneg_u32(value: &Value) -> Option<u32> {
    value.as_u64().map(|v| v as u32)
}

/// Validate-then-build, mirroring the runtime module. Returns the initial state
/// (after run-start relics and the turn-1 draw) or a clear diagnostic.
fn validate_spec(spec: &Value) -> Result<DeckState, String> {
    let obj = spec.as_object().ok_or("spec must be an object")?;
    if spec["schemaVersion"] != "ouroforge.deck-roguelike.v1" {
        return Err("schemaVersion must be ouroforge.deck-roguelike.v1".into());
    }
    let seed = spec["seed"].as_u64().unwrap_or(0) as u32;

    let player = spec["player"]
        .as_object()
        .ok_or("player must be an object")?;
    let max_hp = as_pos_u32(&player["maxHp"]).ok_or("player.maxHp must be a positive integer")?;
    let energy_per_turn = as_pos_u32(&player["energyPerTurn"])
        .ok_or("player.energyPerTurn must be a positive integer")?;
    let hand_size = as_pos_u32(&player["handSize"])
        .ok_or("player.handSize must be a positive integer")? as usize;

    let cards_obj = spec["cards"]
        .as_object()
        .filter(|c| !c.is_empty())
        .ok_or("cards vocabulary must be a non-empty object")?;
    let mut cards: HashMap<String, Card> = HashMap::new();
    for (id, def) in cards_obj {
        let kind = def["type"]
            .as_str()
            .filter(|t| *t == "attack" || *t == "skill")
            .ok_or_else(|| format!("card \"{id}\" has unknown type"))?;
        let cost = as_nonneg_u32(&def["cost"])
            .ok_or_else(|| format!("card \"{id}\" cost must be a non-negative integer"))?;
        if kind == "attack" && as_nonneg_u32(&def["damage"]).is_none() {
            return Err(format!("attack card \"{id}\" must declare damage"));
        }
        if kind == "skill" && as_nonneg_u32(&def["block"]).is_none() {
            return Err(format!("skill card \"{id}\" must declare block"));
        }
        cards.insert(
            id.clone(),
            Card {
                kind: kind.to_string(),
                cost,
                damage: as_nonneg_u32(&def["damage"]).unwrap_or(0),
                block: as_nonneg_u32(&def["block"]).unwrap_or(0),
            },
        );
    }

    let deck = spec["deck"]
        .as_array()
        .filter(|d| !d.is_empty())
        .ok_or("deck must be a non-empty array of card ids")?;
    let mut deck_ids = Vec::new();
    for card_id in deck {
        let card_id = card_id.as_str().ok_or("deck card id must be a string")?;
        if !cards.contains_key(card_id) {
            return Err(format!("deck references undeclared card \"{card_id}\""));
        }
        deck_ids.push(card_id.to_string());
    }

    let mut relic_vocabulary: HashMap<String, (String, u32, u32)> = HashMap::new();
    if let Some(relic_spec) = spec.get("relicVocabulary") {
        let relic_spec = relic_spec
            .as_object()
            .ok_or("relicVocabulary must be an object")?;
        for (id, def) in relic_spec {
            let trigger = def["trigger"]
                .as_str()
                .filter(|t| *t == "run-start" || *t == "turn-start")
                .ok_or_else(|| format!("relic \"{id}\" must declare a valid trigger"))?;
            let effect = def.get("effect").and_then(Value::as_object);
            let block = effect
                .and_then(|e| e.get("block"))
                .and_then(as_nonneg_u32)
                .unwrap_or(0);
            let energy = effect
                .and_then(|e| e.get("energy"))
                .and_then(as_nonneg_u32)
                .unwrap_or(0);
            relic_vocabulary.insert(id.clone(), (trigger.to_string(), block, energy));
        }
    }
    let mut relics = Vec::new();
    if let Some(relic_list) = spec.get("relics") {
        for relic_id in relic_list.as_array().ok_or("relics must be an array")? {
            let relic_id = relic_id.as_str().ok_or("relic id must be a string")?;
            if !relic_vocabulary.contains_key(relic_id) {
                return Err(format!("relics references undeclared relic \"{relic_id}\""));
            }
            relics.push(relic_id.to_string());
        }
    }

    let enemy = spec["enemy"].as_object().ok_or("enemy must be an object")?;
    let enemy_max_hp =
        as_pos_u32(&enemy["maxHp"]).ok_or("enemy.maxHp must be a positive integer")?;
    let intents_raw = enemy["intents"]
        .as_array()
        .filter(|i| !i.is_empty())
        .ok_or("enemy.intents must be a non-empty array")?;
    let mut intents = Vec::new();
    for intent in intents_raw {
        if intent["type"] != "attack" {
            return Err("each enemy intent must be an attack".into());
        }
        let value =
            as_nonneg_u32(&intent["value"]).ok_or("intent value must be a non-negative integer")?;
        intents.push(Intent { value });
    }

    let _ = obj;
    let mut rng = Rng::new(seed);
    let draw_pile = shuffle(&deck_ids, &mut rng);

    let mut state = DeckState {
        rng,
        cards,
        player_hp: max_hp,
        player_block: 0,
        player_energy: 0,
        energy_per_turn,
        bonus_energy: 0,
        hand_size,
        relics,
        relic_vocabulary,
        enemy_hp: enemy_max_hp,
        enemy_block: 0,
        intents,
        intent_index: 0,
        draw_pile,
        hand: Vec::new(),
        discard_pile: Vec::new(),
        turn: 0,
        status: "playing".into(),
    };

    apply_run_start_relics(&mut state);
    begin_player_turn(&mut state);
    Ok(state)
}

fn apply_run_start_relics(state: &mut DeckState) {
    for relic_id in &state.relics {
        if let Some((trigger, block, energy)) = state.relic_vocabulary.get(relic_id) {
            if trigger == "run-start" {
                state.player_block += block;
                state.bonus_energy += energy;
            }
        }
    }
}

fn draw_card(state: &mut DeckState) -> Option<String> {
    if state.draw_pile.is_empty() {
        if state.discard_pile.is_empty() {
            return None;
        }
        let reshuffled = shuffle(&state.discard_pile, &mut state.rng);
        state.draw_pile = reshuffled;
        state.discard_pile.clear();
    }
    Some(state.draw_pile.remove(0))
}

fn begin_player_turn(state: &mut DeckState) {
    state.turn += 1;
    let turn_start_block: u32 = state
        .relics
        .iter()
        .filter_map(|id| state.relic_vocabulary.get(id))
        .filter(|(trigger, _, _)| trigger == "turn-start")
        .map(|(_, block, _)| *block)
        .sum();
    state.player_block += turn_start_block;
    state.player_energy = state.energy_per_turn + state.bonus_energy;
    while state.hand.len() < state.hand_size {
        match draw_card(state) {
            Some(card) => state.hand.push(card),
            None => break,
        }
    }
}

fn deal_damage(amount: u32, block: &mut u32, hp: &mut u32) {
    let absorbed = (*block).min(amount);
    *block -= absorbed;
    let remaining = amount - absorbed;
    if remaining > 0 {
        *hp = hp.saturating_sub(remaining);
    }
}

fn play_card(state: &mut DeckState, hand_index: usize) -> bool {
    if hand_index >= state.hand.len() {
        return false;
    }
    let card_id = state.hand[hand_index].clone();
    let card = state.cards.get(&card_id).expect("card present").clone();
    if card.cost > state.player_energy {
        return false;
    }
    state.player_energy -= card.cost;
    if card.kind == "attack" {
        deal_damage(card.damage, &mut state.enemy_block, &mut state.enemy_hp);
    } else {
        state.player_block += card.block;
    }
    state.hand.remove(hand_index);
    state.discard_pile.push(card_id);
    if state.enemy_hp == 0 {
        state.status = "won".into();
    }
    true
}

fn end_turn(state: &mut DeckState) {
    let intent_value = state.intents[state.intent_index].value;
    deal_damage(intent_value, &mut state.player_block, &mut state.player_hp);
    state.intent_index = (state.intent_index + 1) % state.intents.len();
    while let Some(card) = state.hand.pop() {
        state.discard_pile.push(card);
    }
    if state.player_hp == 0 {
        state.status = "lost".into();
        return;
    }
    begin_player_turn(state);
}

/// Greedy deterministic driver mirroring the runtime test: each turn, play every
/// affordable attack card from the front of the hand, then end the turn.
fn drive_run(state: &mut DeckState) {
    let mut guard = 0;
    while state.status == "playing" && guard < 64 {
        guard += 1;
        loop {
            let mut played = false;
            for i in 0..state.hand.len() {
                let card = &state.cards[&state.hand[i]];
                if card.kind == "attack" && card.cost <= state.player_energy {
                    play_card(state, i);
                    played = true;
                    break;
                }
            }
            if !played || state.status != "playing" {
                break;
            }
        }
        if state.status != "playing" {
            break;
        }
        end_turn(state);
    }
}

/// Canonical digest subset, mirroring the runtime module's `digestState`.
fn digest(state: &DeckState) -> String {
    format!(
        "rng={}:{}:{}|turn={}|status={}|php={}|pbl={}|ehp={}|hand={}|draw={}|discard={}",
        state.rng.seed,
        state.rng.state,
        state.rng.draw_count,
        state.turn,
        state.status,
        state.player_hp,
        state.player_block,
        state.enemy_hp,
        state.hand.join(","),
        state.draw_pile.join(","),
        state.discard_pile.join(","),
    )
}

const VALID_FIXTURE: &str = "examples/game-runtime/deck-roguelike-scene-v1.json";

#[test]
fn valid_deck_spec_satisfies_the_game_class_contract() {
    let spec = deck_spec(VALID_FIXTURE);
    let state = validate_spec(&spec).expect("valid deck spec");
    assert_eq!(state.status, "playing");
    assert_eq!(state.turn, 1);
    assert_eq!(state.player_hp, 30);
    // The run-start relic grants 3 starting block.
    assert_eq!(state.player_block, 3);
    assert_eq!(state.player_energy, 3);
    assert_eq!(state.enemy_hp, 20);
    // Seeded opening hand and remaining draw pile are deterministic for seed 12345.
    assert_eq!(
        state.hand,
        vec!["strike", "strike", "bash", "defend", "defend"]
    );
    assert_eq!(
        state.draw_pile,
        vec!["strike", "strike", "strike", "bash", "defend"]
    );
}

#[test]
fn identical_seed_reproduces_a_digest_stable_run() {
    let spec = deck_spec(VALID_FIXTURE);
    let mut a = validate_spec(&spec).expect("valid deck spec");
    let mut b = validate_spec(&spec).expect("valid deck spec");
    drive_run(&mut a);
    drive_run(&mut b);
    assert_eq!(a.status, "won", "the seeded run reaches a win");
    assert_eq!(
        digest(&a),
        digest(&b),
        "identical seed reproduces a digest-stable run"
    );
}

#[test]
fn seeded_run_is_winnable_and_player_survives() {
    let spec = deck_spec(VALID_FIXTURE);
    let mut state = validate_spec(&spec).expect("valid deck spec");
    drive_run(&mut state);
    assert_eq!(state.status, "won");
    assert_eq!(state.enemy_hp, 0);
    assert!(state.player_hp > 0, "the player survives the encounter");
}

#[test]
fn different_seed_shuffles_differently() {
    let spec = deck_spec(VALID_FIXTURE);
    let baseline = validate_spec(&spec).expect("valid deck spec");

    let mut divergent_spec = spec.clone();
    divergent_spec["seed"] = Value::from(999u32);
    let divergent = validate_spec(&divergent_spec).expect("valid deck spec");

    assert_ne!(
        baseline.hand, divergent.hand,
        "a different seed shuffles to a different opening hand"
    );
}

#[test]
fn malformed_deck_fails_closed() {
    let spec = deck_spec("examples/game-runtime/deck-roguelike-invalid-malformed-deck.json");
    let error = validate_spec(&spec).expect_err("malformed deck must be rejected");
    assert!(
        error.contains("undeclared card \"phantom\""),
        "diagnostic names the offending card: {error}"
    );
}

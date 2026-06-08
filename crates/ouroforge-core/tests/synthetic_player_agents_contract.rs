//! Contract test for Synthetic Player Persona Agents v1 (#1606).
//!
//! Trusted Rust mirror of the runtime persona agents
//! `examples/game-runtime/synthetic-player.js` and its runtime test
//! `examples/game-runtime/synthetic-player.test.cjs`. Per the Era F language
//! boundary, the trusted persona/run logic is owned by Rust/local; the
//! JavaScript module reproduces the same observable behavior for the
//! browser-local probe. Both sides use integer-only mulberry32 decision streams
//! (identical algorithm to the deck shuffle / seeded-rng layer #1600), so a
//! persona's seeded trajectory is digest-identical across the two languages.
//!
//! The personas are human-like (skill governs misplay/fumble rate, aggression
//! governs attack-vs-block style), not win-maximizing solvers: there is no
//! lookahead or per-game tuning. They drive the existing deck-roguelike game
//! class (#1601) — re-derived here in trusted Rust over the shared fixtures —
//! and never perform a trusted write. The test machine-checks the genre's
//! acceptance properties: a persona run is reproducible on a fixed seed,
//! skill/style parameters vary behavior in a bounded way (including a
//! deterministic win/loss spread), the run budget bounds every run, and a
//! malformed persona spec fails closed.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_json(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

fn deck_spec(scene_fixture: &str) -> Value {
    read_json(scene_fixture)["deckRoguelike"].clone()
}

// --- Seeded mulberry32 stream (identical to the runtime seeded-rng layer) ------

const RNG_INCREMENT: u32 = 0x6d2b79f5;
// Narrow fumble band just past the skill threshold (mirrors the JS module).
const FUMBLE_BAND: u32 = 10;
const PARAM_MAX: u32 = 100;

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

fn validate_spec(spec: &Value) -> Result<DeckState, String> {
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
    // Discard the leftover hand front-to-back, matching the runtime module's
    // `hand.shift()` order. (Popping back-to-front would reverse the discard
    // pile and diverge on the next seeded reshuffle.)
    for card in state.hand.drain(..) {
        state.discard_pile.push(card);
    }
    if state.player_hp == 0 {
        state.status = "lost".into();
        return;
    }
    begin_player_turn(state);
}

/// Compact deck digest, identical format to the runtime module's `deckDigest`.
fn deck_digest(state: &DeckState) -> String {
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

// --- Persona agents (the new logic this issue owns) ----------------------------

#[derive(Clone, Debug)]
struct Persona {
    id: String,
    skill: u32,
    aggression: u32,
    seed: u32,
}

#[derive(Clone, Copy, Debug)]
struct Budget {
    max_turns: u32,
    max_actions: u32,
}

const DEFAULT_BUDGET: Budget = Budget {
    max_turns: 64,
    max_actions: 512,
};

#[derive(Debug)]
struct RunRecord {
    outcome: String,
    turns: u32,
    actions: u32,
    budget_exhausted: bool,
    digest: String,
}

fn as_param(value: &Value, field: &str, id: &str) -> Result<u32, String> {
    let n = value.as_i64().ok_or_else(|| {
        format!("persona \"{id}\" {field} must be an integer in [0, {PARAM_MAX}]")
    })?;
    if n < 0 || n > PARAM_MAX as i64 {
        return Err(format!(
            "persona \"{id}\" {field} must be an integer in [0, {PARAM_MAX}]"
        ));
    }
    Ok(n as u32)
}

/// Validate-then-build a personas roster plus its bounded budget. Fails closed.
fn normalize_personas(spec: &Value) -> Result<(Vec<Persona>, Budget), String> {
    if spec["schemaVersion"] != "ouroforge.synthetic-player-personas.v1" {
        return Err("schemaVersion must be ouroforge.synthetic-player-personas.v1".into());
    }
    let list = spec["personas"]
        .as_array()
        .filter(|p| !p.is_empty())
        .ok_or("personas must be a non-empty array")?;
    let mut personas = Vec::new();
    let mut ids = std::collections::HashSet::new();
    for raw in list {
        let id = raw["id"]
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("persona id must be a non-empty string")?
            .to_string();
        let skill = as_param(&raw["skill"], "skill", &id)?;
        let aggression = as_param(&raw["aggression"], "aggression", &id)?;
        let seed = raw["seed"].as_u64().unwrap_or(0) as u32;
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate persona id \"{id}\""));
        }
        personas.push(Persona {
            id,
            skill,
            aggression,
            seed,
        });
    }
    let budget = match spec.get("budget") {
        None => DEFAULT_BUDGET,
        Some(b) => {
            let max_turns = b["maxTurns"]
                .as_u64()
                .unwrap_or(DEFAULT_BUDGET.max_turns as u64);
            let max_actions = b["maxActions"]
                .as_u64()
                .unwrap_or(DEFAULT_BUDGET.max_actions as u64);
            if max_turns == 0 || max_actions == 0 {
                return Err("budget values must be positive integers".into());
            }
            Budget {
                max_turns: max_turns as u32,
                max_actions: max_actions as u32,
            }
        }
    };
    Ok((personas, budget))
}

/// Integer card value under a persona's style; identical to the JS `scoreCard`.
fn score_card(card: &Card, aggression: u32, threat: u32) -> u64 {
    if card.kind == "attack" {
        (card.damage as u64) * (100 + aggression as u64)
    } else {
        (card.block as u64) * (100 + (PARAM_MAX - aggression) as u64)
            + if threat > 0 { (threat as u64) * 5 } else { 0 }
    }
}

enum Action {
    PlayCard(usize),
    EndTurn,
}

/// Choose one existing probe action; identical decision logic to `chooseAction`.
fn choose_action(persona: &Persona, rng: &mut Rng, state: &DeckState) -> Action {
    if state.status != "playing" {
        return Action::EndTurn;
    }
    let energy = state.player_energy;
    let affordable: Vec<usize> = (0..state.hand.len())
        .filter(|&i| state.cards[&state.hand[i]].cost <= energy)
        .collect();
    if affordable.is_empty() {
        return Action::EndTurn;
    }
    let intent_value = state.intents[state.intent_index].value;
    let threat = intent_value.saturating_sub(state.player_block);
    let roll = rng.next_below(100);
    if roll >= persona.skill {
        let local_offset = roll - persona.skill;
        if local_offset < FUMBLE_BAND {
            return Action::EndTurn;
        }
        let pick = rng.next_below(affordable.len() as u32) as usize;
        return Action::PlayCard(affordable[pick]);
    }
    let mut best = affordable[0];
    let mut best_score = score_card(&state.cards[&state.hand[best]], persona.aggression, threat);
    for &i in &affordable[1..] {
        let candidate = score_card(&state.cards[&state.hand[i]], persona.aggression, threat);
        if candidate > best_score {
            best_score = candidate;
            best = i;
        }
    }
    Action::PlayCard(best)
}

/// Run one persona against one deck spec under a bounded budget. Deterministic:
/// the persona decision stream is seeded from (deck seed XOR persona seed) and
/// is independent of the deck shuffle stream.
fn play_run(spec: &Value, persona: &Persona, budget: Budget) -> RunRecord {
    let mut state = validate_spec(spec).expect("valid deck spec");
    let mut prng = Rng::new(state.rng.seed ^ persona.seed);
    let mut actions = 0u32;
    let mut budget_exhausted = false;
    while state.status == "playing" {
        if actions >= budget.max_actions {
            budget_exhausted = true;
            break;
        }
        match choose_action(persona, &mut prng, &state) {
            Action::PlayCard(index) => {
                play_card(&mut state, index);
            }
            Action::EndTurn => {
                // Ending a turn begins the next one; stop before starting a turn
                // beyond the budget so a run record never advances past max_turns.
                if state.turn >= budget.max_turns {
                    budget_exhausted = true;
                    break;
                }
                end_turn(&mut state);
            }
        }
        actions += 1;
    }
    let digest = format!(
        "persona={}|skill={}|aggro={}|outcome={}|turn={}|actions={}|budget={}|{}",
        persona.id,
        persona.skill,
        persona.aggression,
        state.status,
        state.turn,
        actions,
        if budget_exhausted { 1 } else { 0 },
        deck_digest(&state),
    );
    RunRecord {
        outcome: state.status.clone(),
        turns: state.turn,
        actions,
        budget_exhausted,
        digest,
    }
}

const BALANCE_SCENE: &str = "examples/game-runtime/deck-roguelike-balance-scene-v1.json";
const PERSONAS_FIXTURE: &str = "examples/game-runtime/synthetic-player-personas-v1.json";

fn load_roster() -> (Vec<Persona>, Budget) {
    normalize_personas(&read_json(PERSONAS_FIXTURE)).expect("valid persona roster")
}

fn persona_by_id<'a>(personas: &'a [Persona], id: &str) -> &'a Persona {
    personas
        .iter()
        .find(|p| p.id == id)
        .expect("persona exists")
}

// Pinned roster digests on the balance scene (seed 20). Identical to the
// strings the JS runtime test asserts, so the trusted Rust mirror and the
// browser-local probe agree byte-for-byte on every persona trajectory.
const EXPECTED_DIGESTS: &[(&str, &str)] = &[
    (
        "cautious-novice",
        "persona=cautious-novice|skill=20|aggro=25|outcome=won|turn=4|actions=11|budget=0|rng=20:440012099:19|turn=4|status=won|php=4|pbl=0|ehp=0|hand=strike,defend,strike,bash|draw=strike,defend,bash|discard=strike",
    ),
    (
        "reckless-novice",
        "persona=reckless-novice|skill=20|aggro=90|outcome=lost|turn=3|actions=7|budget=0|rng=20:1703683439:15|turn=3|status=lost|php=0|pbl=0|ehp=24|hand=|draw=strike,defend,bash|discard=strike,strike,bash,defend,strike",
    ),
    (
        "balanced-veteran",
        "persona=balanced-veteran|skill=70|aggro=50|outcome=won|turn=3|actions=7|budget=0|rng=20:1703683439:15|turn=3|status=won|php=5|pbl=0|ehp=0|hand=defend,strike,strike,bash|draw=strike,strike,defend|discard=bash",
    ),
    (
        "aggressive-expert",
        "persona=aggressive-expert|skill=95|aggro=95|outcome=won|turn=3|actions=7|budget=0|rng=20:1703683439:15|turn=3|status=won|php=5|pbl=0|ehp=0|hand=defend,strike,strike,bash|draw=strike,strike,defend|discard=bash",
    ),
    (
        "defensive-expert",
        "persona=defensive-expert|skill=95|aggro=5|outcome=won|turn=4|actions=12|budget=0|rng=20:440012099:19|turn=4|status=won|php=4|pbl=10|ehp=0|hand=strike,strike|draw=bash,bash,strike|discard=defend,defend,strike",
    ),
];

#[test]
fn persona_reproduces_an_identical_run_on_a_fixed_seed() {
    let spec = deck_spec(BALANCE_SCENE);
    let (personas, budget) = load_roster();
    let persona = persona_by_id(&personas, "balanced-veteran");
    let first = play_run(&spec, persona, budget);
    let second = play_run(&spec, persona, budget);
    assert_eq!(
        first.digest, second.digest,
        "a persona reproduces an identical run on a fixed seed"
    );
}

#[test]
fn roster_matches_pinned_cross_language_digests() {
    let spec = deck_spec(BALANCE_SCENE);
    let (personas, budget) = load_roster();
    for (id, expected) in EXPECTED_DIGESTS {
        let persona = persona_by_id(&personas, id);
        let record = play_run(&spec, persona, budget);
        assert_eq!(
            &record.digest, expected,
            "persona \"{id}\" matches its pinned cross-language digest"
        );
    }
}

#[test]
fn skill_and_style_vary_behavior_within_the_budget() {
    let spec = deck_spec(BALANCE_SCENE);
    let (personas, budget) = load_roster();
    let records: Vec<RunRecord> = personas
        .iter()
        .map(|p| play_run(&spec, p, budget))
        .collect();

    // Skill/style parameters vary the trajectory and observable behavior.
    let distinct_digests: std::collections::HashSet<&String> =
        records.iter().map(|r| &r.digest).collect();
    assert!(
        distinct_digests.len() > 1,
        "skill/style parameters vary the trajectory"
    );
    let behaviors: std::collections::HashSet<String> = records
        .iter()
        .map(|r| format!("{}:{}:{}", r.outcome, r.turns, r.actions))
        .collect();
    assert!(
        behaviors.len() > 1,
        "skill/style parameters vary observable behavior"
    );

    // The variation is bounded: every run stays within budget and ends legal.
    for record in &records {
        assert!(
            record.turns <= budget.max_turns,
            "a run stays within the turn budget"
        );
        assert!(
            record.actions <= budget.max_actions,
            "a run stays within the action budget"
        );
        assert!(
            matches!(record.outcome.as_str(), "playing" | "won" | "lost"),
            "a run ends in a legal status"
        );
    }
}

#[test]
fn personas_are_human_like_not_win_maximizers() {
    let spec = deck_spec(BALANCE_SCENE);
    let (personas, budget) = load_roster();
    let records: Vec<(String, RunRecord)> = personas
        .iter()
        .map(|p| (p.id.clone(), play_run(&spec, p, budget)))
        .collect();

    let wins = records.iter().filter(|(_, r)| r.outcome == "won").count();
    let losses = records.iter().filter(|(_, r)| r.outcome == "lost").count();
    assert!(wins >= 1, "at least one persona wins the seeded encounter");
    assert!(
        losses >= 1,
        "at least one persona loses — agents are human-like, not win-maximizers"
    );
    let reckless = &records
        .iter()
        .find(|(id, _)| id == "reckless-novice")
        .unwrap()
        .1;
    assert_eq!(
        reckless.outcome, "lost",
        "the reckless novice over-extends and dies"
    );
    let expert = &records
        .iter()
        .find(|(id, _)| id == "aggressive-expert")
        .unwrap()
        .1;
    assert_eq!(
        expert.outcome, "won",
        "the disciplined expert wins the same seeded fight"
    );
}

#[test]
fn run_budget_bounds_every_run() {
    let spec = deck_spec(BALANCE_SCENE);
    let (personas, _) = load_roster();
    let persona = persona_by_id(&personas, "aggressive-expert");

    // A tight action budget halts the run mid-encounter, fail-safe, not forever.
    let capped = play_run(
        &spec,
        persona,
        Budget {
            max_turns: 99,
            max_actions: 3,
        },
    );
    assert_eq!(capped.actions, 3, "the action budget caps the actions");
    assert!(
        capped.budget_exhausted,
        "the action budget flags exhaustion"
    );
    assert_eq!(capped.outcome, "playing", "the capped run is unfinished");

    // A tight turn budget likewise halts the run.
    let turn_capped = play_run(
        &spec,
        persona,
        Budget {
            max_turns: 1,
            max_actions: 999,
        },
    );
    assert!(
        turn_capped.budget_exhausted,
        "the turn budget flags exhaustion"
    );
    assert_eq!(
        turn_capped.turns, 1,
        "the turn budget bounds the run to exactly max_turns turns (no off-by-one)"
    );

    // A generous budget lets the same persona finish: the cap, not the policy,
    // stopped the bounded runs.
    let generous = play_run(&spec, persona, DEFAULT_BUDGET);
    assert_eq!(generous.outcome, "won", "a generous budget lets it finish");
    assert!(
        !generous.budget_exhausted,
        "a finished run is not exhausted"
    );
}

#[test]
fn malformed_persona_fails_closed() {
    let spec = read_json("examples/game-runtime/synthetic-player-invalid-persona.json");
    let error = normalize_personas(&spec).expect_err("malformed persona must be rejected");
    assert!(
        error.contains("skill must be an integer in [0, 100]"),
        "diagnostic names the offending field: {error}"
    );
}

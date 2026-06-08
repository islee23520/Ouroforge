//! Contract test for Balance Telemetry Aggregation v1 (#1607).
//!
//! Trusted Rust mirror of the runtime balance-telemetry aggregator
//! `examples/game-runtime/balance-telemetry.js` and its runtime test
//! `examples/game-runtime/balance-telemetry.test.cjs`. Per the Era F language
//! boundary, the trusted aggregation/detector logic is owned by Rust/local; the
//! JavaScript module reproduces the same observable report for the browser-local
//! dashboard. Both sides use integer-only thresholds and the same persona
//! decision stream (#1606) over the re-derived deck-roguelike class (#1601), so
//! the aggregated balance report is digest-identical across the two languages.
//!
//! This aggregates over EXISTING persona-run evidence — it is not a new
//! simulation, engine, or solver — and the metrics are descriptive, never an
//! auto-applied balance change. The test machine-checks the acceptance
//! properties: aggregation is deterministic, a planted degenerate combo
//! (smite+hex) is flagged with a replayable seed, a planted dead item (brick) is
//! flagged with a replayable seed, a normal card (strike) is left unflagged, and
//! malformed aggregation input fails closed.
//!
//! The deck-roguelike and persona machinery below mirrors
//! `synthetic_player_agents_contract.rs` (#1606); this test extends the run
//! record with a per-card play tally and adds the trusted aggregation.

use std::collections::{BTreeMap, HashMap};
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

// Not every field of the ported run record is read by the telemetry tests
// (e.g. the per-run persona digest); they are retained to mirror the #1606
// record shape faithfully.
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct RunRecord {
    persona_id: String,
    skill: u32,
    aggression: u32,
    seed: u32,
    deck_seed: u32,
    outcome: String,
    turns: u32,
    actions: u32,
    budget_exhausted: bool,
    card_plays: BTreeMap<String, u32>,
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
    let deck_seed = state.rng.seed;
    let mut prng = Rng::new(state.rng.seed ^ persona.seed);
    let mut actions = 0u32;
    let mut budget_exhausted = false;
    // Per-card play tally (observation only), consumed by balance telemetry
    // aggregation. Counts a card once per accepted play; mirrors the JS module.
    let mut card_plays: BTreeMap<String, u32> = BTreeMap::new();
    while state.status == "playing" {
        if actions >= budget.max_actions {
            budget_exhausted = true;
            break;
        }
        match choose_action(persona, &mut prng, &state) {
            Action::PlayCard(index) => {
                let card_id = state.hand.get(index).cloned();
                if play_card(&mut state, index) {
                    if let Some(cid) = card_id {
                        *card_plays.entry(cid).or_insert(0) += 1;
                    }
                }
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
        persona_id: persona.id.clone(),
        skill: persona.skill,
        aggression: persona.aggression,
        seed: persona.seed,
        deck_seed,
        outcome: state.status.clone(),
        turns: state.turn,
        actions,
        budget_exhausted,
        card_plays,
        digest,
    }
}

// --- Balance telemetry aggregation (the new logic this issue owns) --------------

// A card is flagged degenerate when it drives wins and dominates usage: its
// share of all card plays is at least DEGEN_SHARE_PCT percent. A degenerate pair
// is flagged when both members are degenerate and co-played in at least 90% of
// winning runs. Integer thresholds keep the report identical to the JS module.
const DEGEN_SHARE_PCT: u64 = 30;
const DEGEN_NUM: u64 = 9;
const DEGEN_DEN: u64 = 10;

#[derive(Clone, Debug, PartialEq)]
struct CardStat {
    card: String,
    plays: u32,
    runs_included: u32,
    wins_included: u32,
    losses_included: u32,
}

#[derive(Clone, Debug, PartialEq)]
struct ComboFlag {
    cards: Vec<String>,
    included: u32,
    total_wins: u32,
    replay_deck_seed: u32,
    replay_persona: String,
}

#[derive(Clone, Debug, PartialEq)]
struct DeadFlag {
    card: String,
    replay_deck_seed: u32,
    replay_persona: String,
}

#[derive(Clone, Debug)]
struct BalanceReport {
    cards: Vec<CardStat>,
    degenerate_combos: Vec<ComboFlag>,
    dead_items: Vec<DeadFlag>,
    digest: String,
}

fn plays_of(run: &RunRecord, card: &str) -> u32 {
    *run.card_plays.get(card).unwrap_or(&0)
}

/// The sorted card vocabulary declared by a deck spec. Fails closed.
fn vocabulary_of(spec: &Value) -> Result<Vec<String>, String> {
    let cards = spec["cards"]
        .as_object()
        .filter(|c| !c.is_empty())
        .ok_or("deck spec must declare a cards vocabulary")?;
    let mut vocab: Vec<String> = cards.keys().cloned().collect();
    vocab.sort();
    Ok(vocab)
}

/// Aggregate run records over a card vocabulary into a balance report. Pure and
/// deterministic; mirrors the JS `aggregate`. Fails closed on empty input.
fn aggregate(
    runs: &[RunRecord],
    vocab: &[String],
    scene_id: &str,
) -> Result<BalanceReport, String> {
    if runs.is_empty() {
        return Err("runs must be a non-empty array".into());
    }
    if vocab.is_empty() {
        return Err("vocabulary must be a non-empty array".into());
    }
    let total_runs = runs.len() as u32;
    let winning_runs: Vec<&RunRecord> = runs.iter().filter(|r| r.outcome == "won").collect();
    let total_wins = winning_runs.len() as u32;
    let losses = runs.iter().filter(|r| r.outcome == "lost").count() as u32;
    let playing = runs.iter().filter(|r| r.outcome == "playing").count() as u32;

    let mut total_plays: u64 = 0;
    for run in runs {
        for card in vocab {
            total_plays += plays_of(run, card) as u64;
        }
    }

    let cards: Vec<CardStat> = vocab
        .iter()
        .map(|card| {
            let mut plays = 0u32;
            let mut runs_included = 0u32;
            let mut wins_included = 0u32;
            let mut losses_included = 0u32;
            for run in runs {
                let count = plays_of(run, card);
                plays += count;
                if count > 0 {
                    runs_included += 1;
                    if run.outcome == "won" {
                        wins_included += 1;
                    } else if run.outcome == "lost" {
                        losses_included += 1;
                    }
                }
            }
            CardStat {
                card: card.clone(),
                plays,
                runs_included,
                wins_included,
                losses_included,
            }
        })
        .collect();

    // Dead items: declared but never played in any run.
    let dead_items: Vec<DeadFlag> = cards
        .iter()
        .filter(|c| c.plays == 0)
        .map(|c| DeadFlag {
            card: c.card.clone(),
            replay_deck_seed: runs[0].deck_seed,
            replay_persona: runs[0].persona_id.clone(),
        })
        .collect();

    // Degenerate cards dominate usage and drive wins.
    let degenerate_cards: Vec<&CardStat> = if total_wins > 0 {
        cards
            .iter()
            .filter(|c| {
                c.wins_included > 0 && (c.plays as u64) * 100 >= total_plays * DEGEN_SHARE_PCT
            })
            .collect()
    } else {
        Vec::new()
    };

    let mut degenerate_combos: Vec<ComboFlag> = Vec::new();
    for c in &degenerate_cards {
        let run = winning_runs
            .iter()
            .find(|r| plays_of(r, &c.card) > 0)
            .expect("a degenerate card appears in a winning run");
        degenerate_combos.push(ComboFlag {
            cards: vec![c.card.clone()],
            included: c.wins_included,
            total_wins,
            replay_deck_seed: run.deck_seed,
            replay_persona: run.persona_id.clone(),
        });
    }
    for i in 0..degenerate_cards.len() {
        for j in (i + 1)..degenerate_cards.len() {
            let a = &degenerate_cards[i].card;
            let b = &degenerate_cards[j].card;
            let co_runs: Vec<&&RunRecord> = winning_runs
                .iter()
                .filter(|r| plays_of(r, a) > 0 && plays_of(r, b) > 0)
                .collect();
            let count = co_runs.len() as u32;
            if count > 0 && (count as u64) * DEGEN_DEN >= (total_wins as u64) * DEGEN_NUM {
                let mut pair = vec![a.clone(), b.clone()];
                pair.sort();
                degenerate_combos.push(ComboFlag {
                    cards: pair,
                    included: count,
                    total_wins,
                    replay_deck_seed: co_runs[0].deck_seed,
                    replay_persona: co_runs[0].persona_id.clone(),
                });
            }
        }
    }
    degenerate_combos.sort_by(|x, y| x.cards.join("+").cmp(&y.cards.join("+")));

    // Difficulty curve, sorted by persona for a stable order.
    let mut curve: Vec<&RunRecord> = runs.iter().collect();
    curve.sort_by(|x, y| x.persona_id.cmp(&y.persona_id));

    let cards_str = cards
        .iter()
        .map(|c| {
            format!(
                "{}:{}:{}:{}:{}",
                c.card, c.plays, c.runs_included, c.wins_included, c.losses_included
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let degen_str = degenerate_combos
        .iter()
        .map(|g| {
            format!(
                "{{{}@{}/{}#{}:{}}}",
                g.cards.join("+"),
                g.included,
                g.total_wins,
                g.replay_deck_seed,
                g.replay_persona
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let dead_str = dead_items
        .iter()
        .map(|d| format!("{{{}#{}:{}}}", d.card, d.replay_deck_seed, d.replay_persona))
        .collect::<Vec<_>>()
        .join(",");
    let curve_str = curve
        .iter()
        .map(|p| format!("{}:{}:{}:{}", p.persona_id, p.outcome, p.turns, p.actions))
        .collect::<Vec<_>>()
        .join(",");
    let digest = format!(
        "report|scene={}|runs={}|won={}|lost={}|playing={}|cards={}|degen={}|dead={}|curve={}",
        scene_id,
        total_runs,
        total_wins,
        losses,
        playing,
        cards_str,
        degen_str,
        dead_str,
        curve_str
    );

    Ok(BalanceReport {
        cards,
        degenerate_combos,
        dead_items,
        digest,
    })
}

const TELEMETRY_SCENE: &str =
    "examples/game-runtime/deck-roguelike-balance-telemetry-scene-v1.json";
const SAMPLE_REPORT: &str = "examples/game-runtime/balance-telemetry-report-v1.json";

// Pinned report digest on the planted scene (seed 31). Identical to the string
// the JS runtime test asserts, so the trusted Rust mirror and the browser-local
// dashboard agree byte-for-byte on the aggregated balance report.
const EXPECTED_DIGEST: &str = "report|scene=deck-roguelike-balance-telemetry-trial-v1|runs=5|won=5|lost=0|playing=0|cards=brick:0:0:0:0,hex:12:5:5:0,smite:13:5:5:0,strike:6:5:5:0|degen={hex@5/5#31:cautious-novice},{hex+smite@5/5#31:cautious-novice},{smite@5/5#31:cautious-novice}|dead={brick#31:cautious-novice}|curve=aggressive-expert:won:2:7,balanced-veteran:won:4:9,cautious-novice:won:2:7,defensive-expert:won:2:7,reckless-novice:won:3:9";

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

fn run_roster_over(scene: &str) -> Vec<RunRecord> {
    let spec = deck_spec(scene);
    let (personas, budget) = load_roster();
    personas
        .iter()
        .map(|p| play_run(&spec, p, budget))
        .collect()
}

fn build_report() -> BalanceReport {
    let spec = deck_spec(TELEMETRY_SCENE);
    let runs = run_roster_over(TELEMETRY_SCENE);
    aggregate(
        &runs,
        &vocabulary_of(&spec).expect("vocabulary"),
        spec["id"].as_str().unwrap_or("scene"),
    )
    .expect("aggregates")
}

#[test]
fn aggregation_is_deterministic_and_matches_the_pinned_digest() {
    let a = build_report();
    let b = build_report();
    assert_eq!(a.digest, b.digest, "aggregation is deterministic");
    assert_eq!(
        a.digest, EXPECTED_DIGEST,
        "report matches the pinned cross-language digest"
    );
    // The committed sample balance report pins the same digest.
    let sample = read_json(SAMPLE_REPORT);
    assert_eq!(
        sample["digest"].as_str().unwrap(),
        EXPECTED_DIGEST,
        "the committed sample balance report matches the produced report"
    );
}

#[test]
fn flags_the_planted_degenerate_combo_with_a_replayable_seed() {
    let report = build_report();
    let combo = report
        .degenerate_combos
        .iter()
        .find(|g| g.cards == vec!["hex".to_string(), "smite".to_string()])
        .expect("the planted degenerate combo (hex+smite) is flagged");
    assert_eq!(
        combo.included, combo.total_wins,
        "the combo appears in every winning run"
    );
    assert_eq!(combo.replay_deck_seed, 31, "the flag carries the deck seed");

    // Replay the cited seed and confirm both cards are actually played.
    let spec = deck_spec(TELEMETRY_SCENE);
    let (personas, budget) = load_roster();
    let persona = persona_by_id(&personas, &combo.replay_persona);
    let replay = play_run(&spec, persona, budget);
    assert!(
        plays_of(&replay, "hex") > 0 && plays_of(&replay, "smite") > 0,
        "the replay seed reproduces the combo"
    );
}

#[test]
fn flags_the_planted_dead_item_and_leaves_normal_cards_unflagged() {
    let report = build_report();
    let dead: Vec<&str> = report.dead_items.iter().map(|d| d.card.as_str()).collect();
    assert_eq!(dead, vec!["brick"], "only the planted dead item is flagged");
    assert_eq!(report.dead_items[0].replay_deck_seed, 31);

    let strike = report
        .cards
        .iter()
        .find(|c| c.card == "strike")
        .expect("strike present");
    assert!(strike.plays > 0, "the normal card strike is played");
    assert!(
        !report
            .degenerate_combos
            .iter()
            .any(|g| g.cards.iter().any(|c| c == "strike")),
        "the normal card is not degenerate"
    );
    assert!(
        !report.dead_items.iter().any(|d| d.card == "strike"),
        "the normal card is not flagged dead"
    );
}

#[test]
fn malformed_aggregation_input_fails_closed() {
    let runs = run_roster_over(TELEMETRY_SCENE);
    let empty_runs: Vec<RunRecord> = Vec::new();
    let vocab = vec!["smite".to_string()];
    assert!(
        aggregate(&empty_runs, &vocab, "scene")
            .unwrap_err()
            .contains("runs must be a non-empty"),
        "empty runs fail closed"
    );
    assert!(
        aggregate(&runs, &[], "scene")
            .unwrap_err()
            .contains("vocabulary must be a non-empty"),
        "empty vocabulary fails closed"
    );
    assert!(
        vocabulary_of(&serde_json::json!({}))
            .unwrap_err()
            .contains("must declare a cards vocabulary"),
        "a deck without a vocabulary fails closed"
    );
}

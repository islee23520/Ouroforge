//! Brief/NL Intake and Proposal Model v1 (#1593).
//!
//! Part of Generative Front Door v1 (#1592) under #1 Era F Milestone 30. This
//! module turns a structured authoring brief — whose natural-language
//! description is preserved for provenance — into a grid-puzzle artifact
//! *proposal*, validated by trusted Rust/local logic, carrying generation
//! provenance that links the brief to the resulting proposal.
//!
//! Boundary: this is the generative FRONT DOOR. It emits a proposal only. It
//! reuses the existing [`crate::MutationProposal`] model (it is not a new
//! writer), it never performs a trusted write, auto-apply, self-approval, or
//! reviewer bypass, and it never promotes anything. Promotion past the ENGINE
//! ROOM (four gates + solver + over-solution, #1594) is out of scope here: a
//! freshly generated proposal is always unverified and pending review. The
//! grid-puzzle artifact shape mirrors the Grid-Puzzle Game Class v1 (#1574)
//! fixture; this module does not re-implement the runtime/solver, it only
//! validates that the assembled artifact is well-formed before proposing it.
//!
//! Campaign-Scale Generation v1 (#1649) extends this same front door to a second
//! genre — the deck-roguelike class ([`DeckRoguelikeBrief`] /
//! [`intake_deck_roguelike_brief`]) whose artifact shape mirrors the
//! Deck-Roguelike Game Class v1 (#1601) fixture. Both genres reuse the same
//! proposal-only model and provenance here; the campaign-scale set wrapper that
//! produces many such proposals lives in
//! [`crate::content_scale_generation`]. Adding the second genre does not relax
//! any boundary: deck-roguelike proposals are likewise unverified, pending, and
//! never promoted by this module.

use crate::export_hash::sha256_hex;
use crate::MutationProposal;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

/// Schema version for the generative intake brief and the generation provenance.
pub const GENERATIVE_INTAKE_SCHEMA_VERSION: &str = "ouroforge.generative-intake.v1";
/// Identifier recorded as the generator of every intake proposal.
pub const GENERATIVE_INTAKE_GENERATOR: &str = "generative-front-door-intake-v1";
/// The only game class supported by intake v1.
pub const GRID_PUZZLE_GAME_CLASS: &str = "grid-puzzle";
/// The grid-puzzle artifact schema version, matching the Grid-Puzzle Game Class
/// v1 fixture (#1574).
pub const GRID_PUZZLE_SCHEMA_VERSION: &str = "ouroforge.grid-puzzle.v1";
/// The deck-roguelike game class supported by the campaign-scale extension
/// (#1649).
pub const DECK_ROGUELIKE_GAME_CLASS: &str = "deck-roguelike";
/// The deck-roguelike artifact schema version, matching the Deck-Roguelike Game
/// Class v1 fixture (#1601).
pub const DECK_ROGUELIKE_SCHEMA_VERSION: &str = "ouroforge.deck-roguelike.v1";

/// The intake source recorded in provenance (the proposal originated from a
/// brief routed through the generative front door).
pub const GENERATIVE_INTAKE_SOURCE: &str = "brief";

/// Sentinel `from` value for a freshly generated proposal that has no prior
/// artifact. The existing proposal model requires non-empty text; generation
/// produces a new artifact rather than mutating an existing one.
const GENERATIVE_FROM_NONE: &str = "(no prior artifact)";

/// Canonical grid-puzzle object vocabulary used by intake v1. Mirrors the
/// Grid-Puzzle Game Class v1 fixture so the author only sketches the layout.
const CANONICAL_OBJECT_ROLES: &[(&str, &str)] = &[
    ("floor", "background"),
    ("wall", "solid"),
    ("player", "player"),
    ("crate", "pushable"),
    ("target", "target"),
];

/// Canonical grid-puzzle legend used by intake v1. Maps a single character to
/// the stacked object layers it represents.
const CANONICAL_LEGEND: &[(&str, &[&str])] = &[
    ("#", &["floor", "wall"]),
    (".", &["floor"]),
    ("@", &["floor", "target"]),
    ("*", &["floor", "crate"]),
    ("P", &["floor", "player"]),
];

const ALLOWED_DIRECTIONS: &[&str] = &["up", "down", "left", "right"];

/// A structured authoring brief: the front-door intake. The `description` is the
/// natural-language statement of intent (preserved for provenance); `rows` and
/// `intended_solution` are the author's grid-puzzle sketch using the canonical
/// legend.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GenerativeBrief {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    pub title: String,
    /// Natural-language description of the desired puzzle. Preserved verbatim as
    /// the provenance source; never executed.
    pub description: String,
    #[serde(rename = "gameClass")]
    pub game_class: String,
    #[serde(rename = "puzzleId")]
    pub puzzle_id: String,
    /// The grid layout sketched with the canonical legend characters.
    pub rows: Vec<String>,
    #[serde(rename = "intendedSolution")]
    pub intended_solution: Vec<String>,
}

/// Generation provenance attached to a proposal: it links the brief to the
/// resulting proposal and records how the proposal was produced. Read-only
/// audit metadata; it confers no apply or promotion authority.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GenerationProvenance {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    /// Deterministic digest of the canonical brief bytes; links proposal to the
    /// exact brief that produced it.
    #[serde(rename = "briefDigest")]
    pub brief_digest: String,
    pub generator: String,
    #[serde(rename = "gameClass")]
    pub game_class: String,
    pub source: String,
    /// Always true: generation emits proposals only, never a trusted write.
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
}

/// A generated proposal: the existing [`MutationProposal`] plus the generation
/// provenance that links it to its brief. This wraps — it does not modify — the
/// existing proposal model.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GenerativeProposal {
    pub proposal: MutationProposal,
    pub provenance: GenerationProvenance,
}

impl GenerativeBrief {
    /// Parse a brief from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> Result<Self> {
        let brief: GenerativeBrief = serde_json::from_str(text)
            .map_err(|err| anyhow!("generative brief is not valid JSON: {err}"))?;
        Ok(brief)
    }

    /// Validate the brief structurally, failing closed on any problem. Does not
    /// assemble or validate the resulting artifact (see [`intake_brief`]).
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GENERATIVE_INTAKE_SCHEMA_VERSION {
            return Err(anyhow!(
                "generative brief schemaVersion must be \"{GENERATIVE_INTAKE_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("generative brief briefId", &self.brief_id)?;
        crate::require_text("generative brief title", &self.title)?;
        crate::require_text("generative brief description", &self.description)?;
        crate::require_text("generative brief puzzleId", &self.puzzle_id)?;
        if self.game_class != GRID_PUZZLE_GAME_CLASS {
            return Err(anyhow!(
                "generative brief gameClass \"{}\" is unsupported; intake v1 only supports \"{GRID_PUZZLE_GAME_CLASS}\"",
                self.game_class
            ));
        }
        if self.rows.is_empty() {
            return Err(anyhow!("generative brief rows must be a non-empty array"));
        }
        if self.intended_solution.is_empty() {
            return Err(anyhow!(
                "generative brief intendedSolution must be a non-empty array"
            ));
        }
        for direction in &self.intended_solution {
            if !ALLOWED_DIRECTIONS.contains(&direction.as_str()) {
                return Err(anyhow!(
                    "generative brief intendedSolution contains unsupported direction \"{direction}\""
                ));
            }
        }
        Ok(())
    }

    /// Deterministic digest over the canonical serialization of the brief.
    pub fn digest(&self) -> Result<String> {
        let canonical = serde_json::to_vec(self)
            .map_err(|err| anyhow!("failed to serialize generative brief: {err}"))?;
        Ok(sha256_hex(&canonical))
    }
}

impl GenerationProvenance {
    /// Validate the provenance, failing closed on any problem.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GENERATIVE_INTAKE_SCHEMA_VERSION {
            return Err(anyhow!(
                "generation provenance schemaVersion must be \"{GENERATIVE_INTAKE_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("generation provenance briefId", &self.brief_id)?;
        crate::require_text("generation provenance briefDigest", &self.brief_digest)?;
        if self.brief_digest.len() != 64
            || !self.brief_digest.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err(anyhow!(
                "generation provenance briefDigest must be a 64-character hex digest"
            ));
        }
        if self.generator != GENERATIVE_INTAKE_GENERATOR {
            return Err(anyhow!(
                "generation provenance generator must be \"{GENERATIVE_INTAKE_GENERATOR}\""
            ));
        }
        if self.game_class != GRID_PUZZLE_GAME_CLASS && self.game_class != DECK_ROGUELIKE_GAME_CLASS
        {
            return Err(anyhow!(
                "generation provenance gameClass must be \"{GRID_PUZZLE_GAME_CLASS}\" or \"{DECK_ROGUELIKE_GAME_CLASS}\""
            ));
        }
        if self.source != GENERATIVE_INTAKE_SOURCE {
            return Err(anyhow!(
                "generation provenance source must be \"{GENERATIVE_INTAKE_SOURCE}\""
            ));
        }
        if !self.proposal_only {
            return Err(anyhow!(
                "generation provenance proposalOnly must be true: generation emits proposals only"
            ));
        }
        Ok(())
    }
}

impl GenerativeProposal {
    /// Validate the wrapped proposal and provenance together, failing closed.
    pub fn validate(&self) -> Result<()> {
        self.proposal.validate()?;
        self.provenance.validate()?;
        if !self
            .proposal
            .evidence_id
            .ends_with(self.provenance.brief_id.as_str())
        {
            return Err(anyhow!(
                "generative proposal evidence_id must reference provenance briefId"
            ));
        }
        Ok(())
    }

    /// True iff the provenance links this proposal to the given brief: the brief
    /// id matches and the recorded digest equals the brief's canonical digest.
    pub fn links_to(&self, brief: &GenerativeBrief) -> Result<bool> {
        Ok(self.provenance.brief_id == brief.brief_id
            && self.provenance.brief_digest == brief.digest()?)
    }
}

/// Assemble the canonical grid-puzzle artifact from the author's brief sketch.
fn assemble_grid_puzzle_artifact(brief: &GenerativeBrief) -> Value {
    let mut objects = Map::new();
    for (name, role) in CANONICAL_OBJECT_ROLES {
        objects.insert((*name).to_string(), json!({ "role": role }));
    }
    let mut legend = Map::new();
    for (symbol, layers) in CANONICAL_LEGEND {
        legend.insert(
            (*symbol).to_string(),
            Value::Array(layers.iter().map(|l| json!(l)).collect()),
        );
    }
    let width = brief.rows.first().map(|r| r.chars().count()).unwrap_or(0);
    json!({
        "schemaVersion": GRID_PUZZLE_SCHEMA_VERSION,
        "id": brief.puzzle_id,
        "width": width,
        "height": brief.rows.len(),
        "objects": Value::Object(objects),
        "legend": Value::Object(legend),
        "rows": brief.rows,
        "win": { "type": "all-targets-covered" },
        "lose": { "type": "none" },
        "intendedSolution": brief.intended_solution,
    })
}

/// Trusted structural validator for an assembled grid-puzzle artifact. This is a
/// fail-closed well-formedness check over the artifact shape; it is not the
/// engine room (four gates + solver + over-solution, #1594) and it does not
/// re-implement the runtime. It mirrors the structural acceptance rules of the
/// Grid-Puzzle Game Class v1 contract (#1574).
fn validate_grid_puzzle_artifact(artifact: &Value) -> Result<()> {
    if artifact["schemaVersion"] != GRID_PUZZLE_SCHEMA_VERSION {
        return Err(anyhow!(
            "grid-puzzle artifact schemaVersion must be \"{GRID_PUZZLE_SCHEMA_VERSION}\""
        ));
    }
    let width = artifact["width"]
        .as_u64()
        .filter(|w| *w > 0)
        .ok_or_else(|| anyhow!("grid-puzzle artifact width must be a positive integer"))?
        as usize;
    let height = artifact["height"]
        .as_u64()
        .filter(|h| *h > 0)
        .ok_or_else(|| anyhow!("grid-puzzle artifact height must be a positive integer"))?
        as usize;

    // Build the legend → roles map from the canonical vocabulary.
    let mut role_by_object: BTreeMap<&str, &str> = BTreeMap::new();
    for (name, role) in CANONICAL_OBJECT_ROLES {
        role_by_object.insert(name, role);
    }
    let mut roles_by_char: BTreeMap<char, Vec<&str>> = BTreeMap::new();
    for (symbol, layers) in CANONICAL_LEGEND {
        let symbol_char = symbol.chars().next().expect("legend symbol is one char");
        roles_by_char.insert(
            symbol_char,
            layers
                .iter()
                .map(|layer| *role_by_object.get(layer).expect("canonical layer has role"))
                .collect(),
        );
    }

    let rows = artifact["rows"]
        .as_array()
        .filter(|r| r.len() == height)
        .ok_or_else(|| anyhow!("grid-puzzle artifact rows must be an array of {height} strings"))?;
    let mut player_count = 0usize;
    let mut target_count = 0usize;
    let mut pushable_count = 0usize;
    for (y, row) in rows.iter().enumerate() {
        let row = row
            .as_str()
            .filter(|s| s.chars().count() == width)
            .ok_or_else(|| {
                anyhow!("grid-puzzle artifact row {y} must be a string of length {width}")
            })?;
        for (x, symbol) in row.chars().enumerate() {
            let roles = roles_by_char.get(&symbol).ok_or_else(|| {
                anyhow!(
                    "grid-puzzle artifact row {y} column {x} uses unknown character \"{symbol}\""
                )
            })?;
            for role in roles {
                match *role {
                    "player" => player_count += 1,
                    "target" => target_count += 1,
                    "pushable" => pushable_count += 1,
                    _ => {}
                }
            }
        }
    }

    if player_count != 1 {
        return Err(anyhow!(
            "grid-puzzle artifact must contain exactly one player cell, found {player_count}"
        ));
    }
    if target_count == 0 {
        return Err(anyhow!(
            "grid-puzzle artifact win type all-targets-covered requires at least one target"
        ));
    }
    if pushable_count < target_count {
        return Err(anyhow!(
            "grid-puzzle artifact requires at least as many pushables ({pushable_count}) as targets ({target_count})"
        ));
    }
    Ok(())
}

/// Front-door intake: turn a brief into a validated grid-puzzle proposal with
/// generation provenance. Fails closed on a malformed brief or a malformed
/// assembled artifact. `now_unix_ms` is supplied by the caller so the result is
/// deterministic and testable; this function never reads the clock, the
/// filesystem, the network, or performs any trusted write.
pub fn intake_brief(brief: &GenerativeBrief, now_unix_ms: u128) -> Result<GenerativeProposal> {
    brief.validate()?;

    let artifact = assemble_grid_puzzle_artifact(brief);
    validate_grid_puzzle_artifact(&artifact)?;
    let artifact_json = serde_json::to_string(&artifact)
        .map_err(|err| anyhow!("failed to serialize grid-puzzle artifact: {err}"))?;

    let brief_digest = brief.digest()?;
    let evidence_id = format!("generative-intake/{}", brief.brief_id);

    // Build the proposal directly via the existing model. Generation is
    // proposal-only: it does not bind to a run directory, read evidence from
    // disk, or perform any trusted write, so it does not use the run-bound
    // `create_mutation_proposal` path. A freshly generated proposal is
    // proposed/pending — it has not passed the engine room (#1594).
    let proposal = MutationProposal {
        id: format!("generative-{}", brief.puzzle_id),
        reason: format!("Generated grid-puzzle proposal from brief: {}", brief.title),
        evidence_id: evidence_id.clone(),
        target: GRID_PUZZLE_GAME_CLASS.to_string(),
        path: format!("grid-puzzle/{}.json", brief.puzzle_id),
        from: GENERATIVE_FROM_NONE.to_string(),
        to: artifact_json,
        confidence: "unverified".to_string(),
        status: "proposed".to_string(),
        verdict_status: "pending".to_string(),
        created_at_unix_ms: now_unix_ms,
        rationale: None,
    };

    let provenance = GenerationProvenance {
        schema_version: GENERATIVE_INTAKE_SCHEMA_VERSION.to_string(),
        brief_id: brief.brief_id.clone(),
        brief_digest,
        generator: GENERATIVE_INTAKE_GENERATOR.to_string(),
        game_class: GRID_PUZZLE_GAME_CLASS.to_string(),
        source: GENERATIVE_INTAKE_SOURCE.to_string(),
        proposal_only: true,
    };

    let generative = GenerativeProposal {
        proposal,
        provenance,
    };
    generative.validate()?;
    Ok(generative)
}

// --- Deck-roguelike genre (Campaign-Scale Generation v1, #1649) ---------------

/// A structured authoring brief for the deck-roguelike genre: the front-door
/// intake for a single deck-roguelike encounter. The `description` is the
/// natural-language statement of intent (preserved verbatim for provenance); the
/// remaining fields are the author's deck/relic/enemy sketch. The brief mirrors
/// the Deck-Roguelike Game Class v1 (#1601) artifact so the author sketches the
/// run directly; structural acceptance is enforced on the assembled artifact.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeckRoguelikeBrief {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    pub title: String,
    /// Natural-language description of the desired encounter. Preserved verbatim
    /// as the provenance source; never executed.
    pub description: String,
    #[serde(rename = "gameClass")]
    pub game_class: String,
    /// The id recorded on the assembled deck-roguelike artifact.
    #[serde(rename = "runId")]
    pub run_id: String,
    /// Seed for the runtime's seeded stochastic layer (#1600). Recorded on the
    /// artifact; this module never executes the run.
    pub seed: u64,
    /// Player configuration: `{ maxHp, energyPerTurn, handSize }`.
    pub player: Value,
    /// Card vocabulary keyed by card id.
    pub cards: Value,
    /// The draw pile as a list of declared card ids.
    pub deck: Vec<String>,
    /// Optional list of relics the run starts with (each must be declared in
    /// `relicVocabulary`).
    #[serde(default)]
    pub relics: Vec<String>,
    /// Optional relic vocabulary keyed by relic id.
    #[serde(rename = "relicVocabulary", default)]
    pub relic_vocabulary: Value,
    /// Enemy configuration: `{ maxHp, intents: [...] }`.
    pub enemy: Value,
}

impl DeckRoguelikeBrief {
    /// Parse a brief from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> Result<Self> {
        let brief: DeckRoguelikeBrief = serde_json::from_str(text)
            .map_err(|err| anyhow!("deck-roguelike brief is not valid JSON: {err}"))?;
        Ok(brief)
    }

    /// Validate the brief's envelope, failing closed on any problem. Deep
    /// structural acceptance is enforced on the assembled artifact (see
    /// [`intake_deck_roguelike_brief`]).
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GENERATIVE_INTAKE_SCHEMA_VERSION {
            return Err(anyhow!(
                "deck-roguelike brief schemaVersion must be \"{GENERATIVE_INTAKE_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("deck-roguelike brief briefId", &self.brief_id)?;
        crate::require_text("deck-roguelike brief title", &self.title)?;
        crate::require_text("deck-roguelike brief description", &self.description)?;
        crate::require_text("deck-roguelike brief runId", &self.run_id)?;
        if self.game_class != DECK_ROGUELIKE_GAME_CLASS {
            return Err(anyhow!(
                "deck-roguelike brief gameClass \"{}\" is unsupported; expected \"{DECK_ROGUELIKE_GAME_CLASS}\"",
                self.game_class
            ));
        }
        if self.deck.is_empty() {
            return Err(anyhow!(
                "deck-roguelike brief deck must be a non-empty array of card ids"
            ));
        }
        Ok(())
    }

    /// Deterministic digest over the canonical serialization of the brief.
    pub fn digest(&self) -> Result<String> {
        let canonical = serde_json::to_vec(self)
            .map_err(|err| anyhow!("failed to serialize deck-roguelike brief: {err}"))?;
        Ok(sha256_hex(&canonical))
    }
}

/// Assemble the canonical deck-roguelike artifact from the author's brief
/// sketch. Mirrors the Deck-Roguelike Game Class v1 (#1601) `deckRoguelike`
/// block; this module does not execute or balance the run.
fn assemble_deck_roguelike_artifact(brief: &DeckRoguelikeBrief) -> Value {
    let mut artifact = Map::new();
    artifact.insert(
        "schemaVersion".to_string(),
        json!(DECK_ROGUELIKE_SCHEMA_VERSION),
    );
    artifact.insert("id".to_string(), json!(brief.run_id));
    artifact.insert("seed".to_string(), json!(brief.seed));
    artifact.insert("player".to_string(), brief.player.clone());
    artifact.insert("cards".to_string(), brief.cards.clone());
    artifact.insert("deck".to_string(), json!(brief.deck));
    if !brief.relics.is_empty() {
        artifact.insert("relics".to_string(), json!(brief.relics));
    }
    if !brief.relic_vocabulary.is_null() {
        artifact.insert(
            "relicVocabulary".to_string(),
            brief.relic_vocabulary.clone(),
        );
    }
    artifact.insert("enemy".to_string(), brief.enemy.clone());
    Value::Object(artifact)
}

/// Trusted structural validator for an assembled deck-roguelike artifact. A
/// fail-closed well-formedness check over the artifact shape; it is not the
/// engine room and it does not re-implement the runtime. It mirrors the
/// structural acceptance rules of the Deck-Roguelike Game Class v1 contract
/// (#1601): a card vocabulary with valid types, a deck that references only
/// declared cards, relics that reference only the declared vocabulary, and an
/// enemy with at least one intent.
fn validate_deck_roguelike_artifact(artifact: &Value) -> Result<()> {
    if artifact["schemaVersion"] != DECK_ROGUELIKE_SCHEMA_VERSION {
        return Err(anyhow!(
            "deck-roguelike artifact schemaVersion must be \"{DECK_ROGUELIKE_SCHEMA_VERSION}\""
        ));
    }
    let player = artifact["player"]
        .as_object()
        .ok_or_else(|| anyhow!("deck-roguelike artifact player must be an object"))?;
    for field in ["maxHp", "energyPerTurn", "handSize"] {
        if player
            .get(field)
            .and_then(Value::as_u64)
            .filter(|v| *v > 0)
            .is_none()
        {
            return Err(anyhow!(
                "deck-roguelike artifact player.{field} must be a positive integer"
            ));
        }
    }

    let cards = artifact["cards"]
        .as_object()
        .filter(|c| !c.is_empty())
        .ok_or_else(|| anyhow!("deck-roguelike artifact cards must be a non-empty object"))?;
    for (id, def) in cards {
        let kind = def["type"]
            .as_str()
            .filter(|t| *t == "attack" || *t == "skill")
            .ok_or_else(|| anyhow!("deck-roguelike artifact card \"{id}\" has unknown type"))?;
        if def.get("cost").and_then(Value::as_u64).is_none() {
            return Err(anyhow!(
                "deck-roguelike artifact card \"{id}\" cost must be a non-negative integer"
            ));
        }
        if kind == "attack" && def.get("damage").and_then(Value::as_u64).is_none() {
            return Err(anyhow!(
                "deck-roguelike artifact attack card \"{id}\" must declare damage"
            ));
        }
        if kind == "skill" && def.get("block").and_then(Value::as_u64).is_none() {
            return Err(anyhow!(
                "deck-roguelike artifact skill card \"{id}\" must declare block"
            ));
        }
    }

    let deck = artifact["deck"]
        .as_array()
        .filter(|d| !d.is_empty())
        .ok_or_else(|| {
            anyhow!("deck-roguelike artifact deck must be a non-empty array of card ids")
        })?;
    for card_id in deck {
        let card_id = card_id
            .as_str()
            .ok_or_else(|| anyhow!("deck-roguelike artifact deck card id must be a string"))?;
        if !cards.contains_key(card_id) {
            return Err(anyhow!(
                "deck-roguelike artifact deck references undeclared card \"{card_id}\""
            ));
        }
    }

    let empty_vocab = Map::new();
    let relic_vocab = match artifact.get("relicVocabulary") {
        Some(Value::Object(map)) => map,
        Some(Value::Null) | None => &empty_vocab,
        Some(_) => {
            return Err(anyhow!(
                "deck-roguelike artifact relicVocabulary must be an object"
            ))
        }
    };
    for (id, def) in relic_vocab {
        def["trigger"]
            .as_str()
            .filter(|t| *t == "run-start" || *t == "turn-start")
            .ok_or_else(|| {
                anyhow!("deck-roguelike artifact relic \"{id}\" must declare a valid trigger")
            })?;
    }
    if let Some(relics) = artifact.get("relics") {
        let relics = relics
            .as_array()
            .ok_or_else(|| anyhow!("deck-roguelike artifact relics must be an array"))?;
        for relic_id in relics {
            let relic_id = relic_id
                .as_str()
                .ok_or_else(|| anyhow!("deck-roguelike artifact relic id must be a string"))?;
            if !relic_vocab.contains_key(relic_id) {
                return Err(anyhow!(
                    "deck-roguelike artifact relics references undeclared relic \"{relic_id}\""
                ));
            }
        }
    }

    let enemy = artifact["enemy"]
        .as_object()
        .ok_or_else(|| anyhow!("deck-roguelike artifact enemy must be an object"))?;
    if enemy
        .get("maxHp")
        .and_then(Value::as_u64)
        .filter(|v| *v > 0)
        .is_none()
    {
        return Err(anyhow!(
            "deck-roguelike artifact enemy.maxHp must be a positive integer"
        ));
    }
    let intents = enemy
        .get("intents")
        .and_then(Value::as_array)
        .filter(|i| !i.is_empty())
        .ok_or_else(|| {
            anyhow!("deck-roguelike artifact enemy.intents must be a non-empty array")
        })?;
    for intent in intents {
        if intent.get("type").and_then(Value::as_str).is_none() {
            return Err(anyhow!(
                "deck-roguelike artifact enemy intent must declare a string type"
            ));
        }
    }
    Ok(())
}

/// Front-door intake for the deck-roguelike genre: turn a brief into a validated
/// deck-roguelike proposal with generation provenance. Fails closed on a
/// malformed brief or a malformed assembled artifact. Like [`intake_brief`],
/// `now_unix_ms` is supplied by the caller, and this function never reads the
/// clock, the filesystem, the network, or performs any trusted write. The result
/// is a proposal only: unverified, pending, never promoted.
pub fn intake_deck_roguelike_brief(
    brief: &DeckRoguelikeBrief,
    now_unix_ms: u128,
) -> Result<GenerativeProposal> {
    brief.validate()?;

    let artifact = assemble_deck_roguelike_artifact(brief);
    validate_deck_roguelike_artifact(&artifact)?;
    let artifact_json = serde_json::to_string(&artifact)
        .map_err(|err| anyhow!("failed to serialize deck-roguelike artifact: {err}"))?;

    let brief_digest = brief.digest()?;
    let evidence_id = format!("generative-intake/{}", brief.brief_id);

    let proposal = MutationProposal {
        id: format!("generative-{}", brief.run_id),
        reason: format!(
            "Generated deck-roguelike proposal from brief: {}",
            brief.title
        ),
        evidence_id: evidence_id.clone(),
        target: DECK_ROGUELIKE_GAME_CLASS.to_string(),
        path: format!("deck-roguelike/{}.json", brief.run_id),
        from: GENERATIVE_FROM_NONE.to_string(),
        to: artifact_json,
        confidence: "unverified".to_string(),
        status: "proposed".to_string(),
        verdict_status: "pending".to_string(),
        created_at_unix_ms: now_unix_ms,
        rationale: None,
    };

    let provenance = GenerationProvenance {
        schema_version: GENERATIVE_INTAKE_SCHEMA_VERSION.to_string(),
        brief_id: brief.brief_id.clone(),
        brief_digest,
        generator: GENERATIVE_INTAKE_GENERATOR.to_string(),
        game_class: DECK_ROGUELIKE_GAME_CLASS.to_string(),
        source: GENERATIVE_INTAKE_SOURCE.to_string(),
        proposal_only: true,
    };

    let generative = GenerativeProposal {
        proposal,
        provenance,
    };
    generative.validate()?;
    Ok(generative)
}

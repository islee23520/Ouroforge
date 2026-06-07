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
        if self.game_class != GRID_PUZZLE_GAME_CLASS {
            return Err(anyhow!(
                "generation provenance gameClass must be \"{GRID_PUZZLE_GAME_CLASS}\""
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

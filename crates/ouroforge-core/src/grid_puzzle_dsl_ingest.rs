//! PuzzleScript-compatible DSL ingest for the Grid-Puzzle Game Class v1 (#1575).
//!
//! This is the trusted Rust/local owner of the **validate-then-load** ingest
//! path described in the Grid-Puzzle Game Class v1 design gate
//! (`docs/grid-puzzle-game-class-v1.md`, "PuzzleScript-compatible DSL ingest
//! contract"). It parses a PuzzleScript-compatible DSL document, validates it
//! against the supported compatible subset, and lowers it into the existing
//! `ouroforge.grid-puzzle.v1` game-class spec — the exact shape the runtime
//! (`examples/game-runtime/grid-puzzle.js`), the game-class contract test, and
//! the [`crate::puzzle_solver`] consume.
//!
//! It is not a new engine, runtime, or parser engine: it is a bounded
//! validate-then-lower front door onto the existing game class. The supported
//! subset is the canonical PuzzleScript block-pushing (Sokoban) shape:
//!
//! - an `OBJECTS` vocabulary;
//! - a `LEGEND` mapping single-character level glyphs to object stacks (`and`)
//!   and optional property synonyms (`or`);
//! - `COLLISIONLAYERS` declaring the bottom-to-top layer order;
//! - exactly the canonical push `RULES` of the form
//!   `[ > Player | Crate ] -> [ > Player | > Crate ]`;
//! - a `WINCONDITIONS` line of the form `All Target on Crate`;
//! - one or more `LEVELS` grids (the first grid is ingested).
//!
//! Validation **fails closed**. A missing, malformed, ambiguous, or
//! out-of-subset document is rejected with a structured reason and never
//! silently partially imported. Anything outside the supported subset (an
//! unrecognised rule form, an unsupported win condition, a directional/late/
//! random rule prefix) is rejected as an explicit unsupported-construct
//! diagnostic rather than guessed at.
//!
//! Roles are derived deterministically from PuzzleScript semantics, not declared
//! directly: the special `Player` object is the player, `Background` is the
//! grid background, objects pushed by a push rule are pushable, the win
//! condition's target operand is a target, and every other object used by the
//! level is solid. The lowered spec is re-validated through the existing
//! [`crate::puzzle_solver::validate_spec`] game-class validator before it is
//! returned, so a document that does not load into the game class is rejected.

use std::collections::BTreeMap;
use std::fmt;

use serde_json::{json, Map, Value};

use crate::puzzle_solver;

/// The PuzzleScript section headers the supported subset understands.
const KNOWN_SECTIONS: [&str; 7] = [
    "OBJECTS",
    "LEGEND",
    "SOUNDS",
    "COLLISIONLAYERS",
    "RULES",
    "WINCONDITIONS",
    "LEVELS",
];

/// The game-class roles the lowering may assign. Mirrors the runtime and
/// [`crate::puzzle_solver`] allowed-role vocabulary.
const PLAYER_OBJECT: &str = "player";
const BACKGROUND_OBJECT: &str = "background";

/// A structured, fail-closed rejection reason. The two kinds let callers (and
/// the contract test) distinguish a malformed document from a well-formed one
/// that uses a construct outside the supported compatible subset; both are
/// rejections and neither loads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IngestError {
    /// The document is missing required structure, references undeclared
    /// vocabulary, or is otherwise not well-formed PuzzleScript-compatible
    /// input.
    Malformed(String),
    /// The document is well-formed but uses a PuzzleScript construct this
    /// compatible subset does not support (so it cannot be faithfully lowered).
    UnsupportedConstruct(String),
}

impl IngestError {
    /// The human-readable diagnostic reason.
    pub fn message(&self) -> &str {
        match self {
            IngestError::Malformed(message) | IngestError::UnsupportedConstruct(message) => message,
        }
    }

    /// True when the rejection is an explicit unsupported-construct diagnostic.
    pub fn is_unsupported(&self) -> bool {
        matches!(self, IngestError::UnsupportedConstruct(_))
    }

    /// True when the rejection is a malformed-input diagnostic.
    pub fn is_malformed(&self) -> bool {
        matches!(self, IngestError::Malformed(_))
    }
}

impl fmt::Display for IngestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IngestError::Malformed(message) => write!(f, "malformed PuzzleScript DSL: {message}"),
            IngestError::UnsupportedConstruct(message) => {
                write!(f, "unsupported PuzzleScript construct: {message}")
            }
        }
    }
}

impl std::error::Error for IngestError {}

/// A deterministic summary of a successful ingest, suitable for evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IngestReport {
    /// The lowered game-class spec id (derived from the document title).
    pub id: String,
    /// The level width that was lowered.
    pub width: usize,
    /// The level height that was lowered.
    pub height: usize,
    /// The object roles assigned by the lowering, sorted by object name.
    pub roles: Vec<(String, String)>,
}

/// The result of a successful validate-then-load ingest.
#[derive(Clone, Debug)]
pub struct IngestOutcome {
    /// The lowered `ouroforge.grid-puzzle.v1` spec, re-validated against the
    /// existing game-class validator.
    pub spec: Value,
    /// A deterministic summary of what was ingested.
    pub report: IngestReport,
}

/// How a legend entry combines its referenced objects.
#[derive(Clone, Debug)]
enum Alias {
    /// `Symbol = A and B` (or a single object): a bottom-to-top cell stack.
    Stack(Vec<String>),
    /// `Symbol = A or B`: a property synonym used by rules/win conditions.
    Property(Vec<String>),
}

/// Parsed PuzzleScript sections, keyed by canonical (upper-case) header.
struct Sections {
    title: Option<String>,
    by_name: BTreeMap<String, Vec<String>>,
}

/// Validate-then-load entry point: parse, validate, and lower a
/// PuzzleScript-compatible DSL document into an `ouroforge.grid-puzzle.v1`
/// game-class spec. Fails closed with a structured [`IngestError`].
pub fn ingest_puzzlescript(source: &str) -> Result<IngestOutcome, IngestError> {
    let sections = split_sections(source);

    let object_lines = section(&sections, "OBJECTS")?;
    let objects = parse_objects(object_lines)?;

    let legend_lines = section(&sections, "LEGEND")?;
    let aliases = parse_legend(legend_lines, &objects)?;

    let collision_lines = section(&sections, "COLLISIONLAYERS")?;
    let layer_index = parse_collision_layers(collision_lines, &objects, &aliases)?;

    let rule_lines = section(&sections, "RULES")?;
    let pushables = parse_rules(rule_lines, &objects, &aliases)?;

    let win_lines = section(&sections, "WINCONDITIONS")?;
    let (targets, win_pushable) = parse_win_conditions(win_lines, &objects, &aliases, &pushables)?;

    let level_lines = section(&sections, "LEVELS")?;
    let rows = parse_first_level(level_lines, &aliases)?;

    let spec = lower(
        &sections,
        &objects,
        &aliases,
        &layer_index,
        &pushables,
        &targets,
        &win_pushable,
        &rows,
    )?;

    // Reuse the existing game-class validator so a document that does not load
    // into the runtime game class is rejected here, fail-closed.
    let state = puzzle_solver::validate_spec(&spec).map_err(|reason| {
        IngestError::Malformed(format!("lowered spec does not load: {reason}"))
    })?;

    let mut roles: Vec<(String, String)> = spec["objects"]
        .as_object()
        .expect("lowered spec has objects")
        .iter()
        .map(|(name, def)| {
            (
                name.clone(),
                def["role"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    roles.sort();

    let report = IngestReport {
        id: spec["id"].as_str().unwrap_or_default().to_string(),
        width: state.width(),
        height: state.height(),
        roles,
    };

    Ok(IngestOutcome { spec, report })
}

/// Split the document into sections. Lines before the first recognised header
/// are the prelude (only `title` is retained). `=`-only separator lines are
/// dropped. Header recognition is case-insensitive.
fn split_sections(source: &str) -> Sections {
    let mut by_name: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut title: Option<String> = None;
    let mut current: Option<String> = None;

    for raw in source.lines() {
        let line = strip_comment(raw);
        let trimmed = line.trim();

        if is_separator(trimmed) {
            continue;
        }

        let header = trimmed.to_ascii_uppercase();
        if KNOWN_SECTIONS.contains(&header.as_str()) {
            current = Some(header.clone());
            by_name.entry(header).or_default();
            continue;
        }

        match &current {
            None => {
                // Prelude: capture the title for the spec id; ignore the rest.
                if let Some(rest) = prelude_value(trimmed, "title") {
                    if !rest.is_empty() {
                        title = Some(rest.to_string());
                    }
                }
            }
            Some(name) => {
                by_name
                    .entry(name.clone())
                    .or_default()
                    .push(line.to_string());
            }
        }
    }

    Sections { title, by_name }
}

/// Strip a `(comment)` style PuzzleScript inline comment region. Conservative:
/// only drops a parenthesised suffix on a line; nested comments are uncommon in
/// the supported subset and a stray paren is preserved.
fn strip_comment(line: &str) -> &str {
    match line.find('(') {
        Some(idx) if line[idx..].contains(')') => &line[..idx],
        _ => line,
    }
}

fn is_separator(trimmed: &str) -> bool {
    !trimmed.is_empty() && trimmed.chars().all(|c| c == '=')
}

fn prelude_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let lower = line.to_ascii_lowercase();
    if lower.starts_with(key) {
        let rest = line[key.len()..].trim();
        return Some(rest);
    }
    None
}

fn section<'a>(sections: &'a Sections, name: &str) -> Result<&'a [String], IngestError> {
    sections
        .by_name
        .get(name)
        .map(Vec::as_slice)
        .ok_or_else(|| IngestError::Malformed(format!("required section {name} is missing")))
}

/// Parse the `OBJECTS` section into a declaration-ordered list of lower-cased
/// object ids. Color and sprite bodies are tolerated and ignored.
fn parse_objects(lines: &[String]) -> Result<Vec<String>, IngestError> {
    let mut objects: Vec<String> = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if lines[index].trim().is_empty() {
            index += 1;
            continue;
        }
        let name = lines[index]
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if name.is_empty() {
            return Err(IngestError::Malformed("object name is empty".into()));
        }
        if objects.contains(&name) {
            return Err(IngestError::Malformed(format!(
                "duplicate object declaration \"{name}\""
            )));
        }
        objects.push(name);
        index += 1;
        // Consume the (ignored) color/sprite body up to the next blank line.
        while index < lines.len() && !lines[index].trim().is_empty() {
            index += 1;
        }
    }
    if objects.is_empty() {
        return Err(IngestError::Malformed("OBJECTS section is empty".into()));
    }
    Ok(objects)
}

/// Parse the `LEGEND` section into ordered alias definitions.
fn parse_legend(lines: &[String], objects: &[String]) -> Result<Vec<(String, Alias)>, IngestError> {
    let mut aliases: Vec<(String, Alias)> = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (key, rhs) = trimmed.split_once('=').ok_or_else(|| {
            IngestError::Malformed(format!("legend line lacks '=': \"{trimmed}\""))
        })?;
        let key = key.trim().to_ascii_lowercase();
        if key.is_empty() {
            return Err(IngestError::Malformed("legend key is empty".into()));
        }
        let tokens: Vec<String> = rhs
            .split_whitespace()
            .map(|t| t.to_ascii_lowercase())
            .collect();
        if tokens.is_empty() {
            return Err(IngestError::Malformed(format!(
                "legend \"{key}\" has an empty definition"
            )));
        }

        let uses_and = tokens.iter().any(|t| t == "and");
        let uses_or = tokens.iter().any(|t| t == "or");
        if uses_and && uses_or {
            return Err(IngestError::Malformed(format!(
                "legend \"{key}\" mixes 'and' and 'or'"
            )));
        }

        let names: Vec<String> = tokens
            .iter()
            .filter(|t| *t != "and" && *t != "or")
            .cloned()
            .collect();
        if names.is_empty() {
            return Err(IngestError::Malformed(format!(
                "legend \"{key}\" lists no objects"
            )));
        }

        let alias = if uses_or {
            let mut members = Vec::new();
            for name in &names {
                members.extend(resolve_members(name, objects, &aliases)?);
            }
            Alias::Property(members)
        } else {
            let mut stack = Vec::new();
            for name in &names {
                stack.extend(resolve_stack(name, objects, &aliases)?);
            }
            Alias::Stack(stack)
        };
        aliases.push((key, alias));
    }
    if aliases.is_empty() {
        return Err(IngestError::Malformed("LEGEND section is empty".into()));
    }
    Ok(aliases)
}

/// Resolve a name to a concrete bottom-to-top object stack. A property used
/// where a stack is required is malformed.
fn resolve_stack(
    name: &str,
    objects: &[String],
    aliases: &[(String, Alias)],
) -> Result<Vec<String>, IngestError> {
    let name = name.to_ascii_lowercase();
    if objects.contains(&name) {
        return Ok(vec![name]);
    }
    match find_alias(&name, aliases) {
        Some(Alias::Stack(stack)) => Ok(stack.clone()),
        Some(Alias::Property(_)) => Err(IngestError::Malformed(format!(
            "property \"{name}\" cannot be used as a stacked level tile"
        ))),
        None => Err(IngestError::Malformed(format!(
            "legend references undeclared object \"{name}\""
        ))),
    }
}

/// Resolve a name to the set of concrete objects it denotes (an object, a
/// stack's objects, or a property's members).
fn resolve_members(
    name: &str,
    objects: &[String],
    aliases: &[(String, Alias)],
) -> Result<Vec<String>, IngestError> {
    let name = name.to_ascii_lowercase();
    if objects.contains(&name) {
        return Ok(vec![name]);
    }
    match find_alias(&name, aliases) {
        Some(Alias::Stack(stack)) => Ok(stack.clone()),
        Some(Alias::Property(members)) => Ok(members.clone()),
        None => Err(IngestError::Malformed(format!(
            "reference to undeclared object or property \"{name}\""
        ))),
    }
}

fn find_alias<'a>(name: &str, aliases: &'a [(String, Alias)]) -> Option<&'a Alias> {
    aliases
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, alias)| alias)
}

/// Parse `COLLISIONLAYERS` into an object -> bottom-to-top layer index map. Each
/// referenced object must be declared; every object used by the level must
/// appear in a layer (validated later via lookups).
fn parse_collision_layers(
    lines: &[String],
    objects: &[String],
    aliases: &[(String, Alias)],
) -> Result<BTreeMap<String, usize>, IngestError> {
    let mut layer_index: BTreeMap<String, usize> = BTreeMap::new();
    let mut index = 0usize;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let tokens = trimmed
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|t| !t.is_empty());
        let mut any = false;
        for token in tokens {
            for object in resolve_members(token, objects, aliases)? {
                layer_index.entry(object).or_insert(index);
                any = true;
            }
        }
        if any {
            index += 1;
        }
    }
    if layer_index.is_empty() {
        return Err(IngestError::Malformed(
            "COLLISIONLAYERS section is empty".into(),
        ));
    }
    Ok(layer_index)
}

/// Parse `RULES`, accepting only the canonical push rule and identifying the
/// pushable objects. Any other rule form is an explicit unsupported construct.
fn parse_rules(
    lines: &[String],
    objects: &[String],
    aliases: &[(String, Alias)],
) -> Result<Vec<String>, IngestError> {
    let mut pushables: Vec<String> = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let pushable_name = match_push_rule(trimmed)?;
        for object in resolve_members(&pushable_name, objects, aliases)? {
            if !pushables.contains(&object) {
                pushables.push(object);
            }
        }
    }
    if pushables.is_empty() {
        return Err(IngestError::Malformed(
            "RULES section declares no canonical push rule".into(),
        ));
    }
    Ok(pushables)
}

/// Match `[ > Player | X ] -> [ > Player | > X ]` exactly, returning `X`.
/// Returns an unsupported-construct error for any other rule shape.
fn match_push_rule(rule: &str) -> Result<String, IngestError> {
    let unsupported = || {
        IngestError::UnsupportedConstruct(format!("rule is outside the push subset: \"{rule}\""))
    };

    let (lhs, rhs) = rule.split_once("->").ok_or_else(unsupported)?;
    let lhs_cells = bracket_cells(lhs).ok_or_else(unsupported)?;
    let rhs_cells = bracket_cells(rhs).ok_or_else(unsupported)?;
    if lhs_cells.len() != 2 || rhs_cells.len() != 2 {
        return Err(unsupported());
    }

    let lhs0 = tokens(&lhs_cells[0]);
    let lhs1 = tokens(&lhs_cells[1]);
    let rhs0 = tokens(&rhs_cells[0]);
    let rhs1 = tokens(&rhs_cells[1]);

    // LHS: `> player | X`
    if lhs0.len() != 2 || lhs0[0] != ">" || lhs0[1] != PLAYER_OBJECT {
        return Err(unsupported());
    }
    if lhs1.len() != 1 {
        return Err(unsupported());
    }
    let pushable = lhs1[0].clone();
    // RHS: `> player | > X`
    if rhs0.len() != 2 || rhs0[0] != ">" || rhs0[1] != PLAYER_OBJECT {
        return Err(unsupported());
    }
    if rhs1.len() != 2 || rhs1[0] != ">" || rhs1[1] != pushable {
        return Err(unsupported());
    }
    Ok(pushable)
}

/// Split a `[ ... | ... ]` bracket body into its cells. Returns `None` if the
/// side is not a single well-formed bracket group.
fn bracket_cells(side: &str) -> Option<Vec<String>> {
    let trimmed = side.trim();
    let inner = trimmed.strip_prefix('[')?.strip_suffix(']')?;
    if inner.contains('[') || inner.contains(']') {
        return None;
    }
    Some(inner.split('|').map(|c| c.trim().to_string()).collect())
}

fn tokens(cell: &str) -> Vec<String> {
    cell.split_whitespace()
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

/// Parse `WINCONDITIONS`, accepting only `All <Target> on <Pushable>`. Returns
/// the target objects and the pushable operand. Other forms are unsupported.
fn parse_win_conditions(
    lines: &[String],
    objects: &[String],
    aliases: &[(String, Alias)],
    pushables: &[String],
) -> Result<(Vec<String>, String), IngestError> {
    let mut result: Option<(Vec<String>, String)> = None;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts: Vec<String> = tokens(trimmed);
        if parts.len() != 4 || parts[0] != "all" || parts[2] != "on" {
            return Err(IngestError::UnsupportedConstruct(format!(
                "win condition is outside the supported \"All <target> on <pushable>\" subset: \"{trimmed}\""
            )));
        }
        if result.is_some() {
            return Err(IngestError::UnsupportedConstruct(
                "multiple win conditions are not supported".into(),
            ));
        }
        let targets = resolve_members(&parts[1], objects, aliases)?;
        let win_pushables = resolve_members(&parts[3], objects, aliases)?;
        for object in &win_pushables {
            if !pushables.contains(object) {
                return Err(IngestError::Malformed(format!(
                    "win condition references \"{object}\", which no push rule makes pushable"
                )));
            }
        }
        if targets.is_empty() {
            return Err(IngestError::Malformed(
                "win condition declares no target object".into(),
            ));
        }
        let pushable = win_pushables
            .first()
            .cloned()
            .ok_or_else(|| IngestError::Malformed("win condition declares no pushable".into()))?;
        result = Some((targets, pushable));
    }
    result.ok_or_else(|| IngestError::Malformed("WINCONDITIONS section declares no win".into()))
}

/// Parse the first level grid out of `LEVELS`. `message` lines and blank lines
/// before the grid are skipped; ragged grids are malformed.
fn parse_first_level(
    lines: &[String],
    aliases: &[(String, Alias)],
) -> Result<Vec<String>, IngestError> {
    let mut rows: Vec<String> = Vec::new();
    for line in lines {
        let trimmed_end = line.trim_end();
        let is_blank = trimmed_end.trim().is_empty();
        let is_message = trimmed_end
            .trim_start()
            .to_ascii_lowercase()
            .starts_with("message");
        if rows.is_empty() {
            if is_blank || is_message {
                continue;
            }
            rows.push(trimmed_end.to_string());
        } else if is_blank || is_message {
            break;
        } else {
            rows.push(trimmed_end.to_string());
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Malformed("LEVELS section has no grid".into()));
    }
    let width = rows[0].chars().count();
    for (y, row) in rows.iter().enumerate() {
        if row.chars().count() != width {
            return Err(IngestError::Malformed(format!(
                "level row {y} has width {}, expected {width}",
                row.chars().count()
            )));
        }
        for symbol in row.chars() {
            let key = symbol.to_ascii_lowercase().to_string();
            match find_alias(&key, aliases) {
                Some(Alias::Stack(_)) => {}
                Some(Alias::Property(_)) => {
                    return Err(IngestError::Malformed(format!(
                        "level uses property glyph \"{symbol}\" as a tile"
                    )));
                }
                None => {
                    return Err(IngestError::Malformed(format!(
                        "level uses glyph \"{symbol}\" absent from the legend"
                    )));
                }
            }
        }
    }
    Ok(rows)
}

/// Lower the validated sections into an `ouroforge.grid-puzzle.v1` spec.
#[allow(clippy::too_many_arguments)]
fn lower(
    sections: &Sections,
    objects: &[String],
    aliases: &[(String, Alias)],
    layer_index: &BTreeMap<String, usize>,
    pushables: &[String],
    targets: &[String],
    win_pushable: &str,
    rows: &[String],
) -> Result<Value, IngestError> {
    let _ = win_pushable; // validated against pushables; the spec uses roles.

    // Collect the objects actually used by the level, and order each cell stack
    // by collision-layer index for deterministic, runtime-faithful layering.
    let mut used: Vec<String> = Vec::new();
    let mut legend_map: Map<String, Value> = Map::new();
    let mut symbols: Vec<(String, Vec<String>)> = Vec::new();

    let mut distinct_symbols: Vec<char> = Vec::new();
    for row in rows {
        for symbol in row.chars() {
            if !distinct_symbols.contains(&symbol) {
                distinct_symbols.push(symbol);
            }
        }
    }
    for symbol in &distinct_symbols {
        let key = symbol.to_ascii_lowercase().to_string();
        let stack = match find_alias(&key, aliases) {
            Some(Alias::Stack(stack)) => stack.clone(),
            _ => {
                return Err(IngestError::Malformed(format!(
                    "level glyph \"{symbol}\" is not a stack legend entry"
                )))
            }
        };
        // Every used object must be assigned a collision layer; an object the
        // level uses but `COLLISIONLAYERS` omits is malformed PuzzleScript and
        // is rejected fail-closed rather than silently given a default layer.
        for object in &stack {
            if !layer_index.contains_key(object) {
                return Err(IngestError::Malformed(format!(
                    "object \"{object}\" is used by the level but absent from COLLISIONLAYERS"
                )));
            }
        }
        let mut ordered = stack.clone();
        ordered.sort_by_key(|object| layer_for(object, layer_index));
        for object in &ordered {
            if !used.contains(object) {
                used.push(object.clone());
            }
        }
        symbols.push((symbol.to_string(), ordered));
    }

    // Assign exactly one role per used object, fail-closed on conflicts.
    let mut roles: BTreeMap<String, String> = BTreeMap::new();
    for object in &used {
        let role = classify(object, pushables, targets)?;
        roles.insert(object.clone(), role);
    }
    if !roles.values().any(|role| role == "player") {
        return Err(IngestError::Malformed(
            "level declares no Player object".into(),
        ));
    }
    if !roles.values().any(|role| role == "target") {
        return Err(IngestError::Malformed(
            "level declares no target tile".into(),
        ));
    }

    // Confirm declared objects were the source of every used object (defensive).
    for object in &used {
        if !objects.contains(object) {
            return Err(IngestError::Malformed(format!(
                "level uses undeclared object \"{object}\""
            )));
        }
    }

    let mut objects_map: Map<String, Value> = Map::new();
    for (object, role) in &roles {
        objects_map.insert(object.clone(), json!({ "role": role }));
    }
    for (symbol, stack) in &symbols {
        legend_map.insert(symbol.clone(), Value::from(stack.clone()));
    }

    let id = sections
        .title
        .as_deref()
        .map(slug)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "imported-puzzlescript".to_string());

    let height = rows.len();
    let width = rows.first().map(|r| r.chars().count()).unwrap_or(0);

    Ok(json!({
        "schemaVersion": "ouroforge.grid-puzzle.v1",
        "id": id,
        "width": width,
        "height": height,
        "objects": Value::Object(objects_map),
        "legend": Value::Object(legend_map),
        "rows": rows.to_vec(),
        "win": { "type": "all-targets-covered" },
        "lose": { "type": "none" },
    }))
}

fn layer_for(object: &str, layer_index: &BTreeMap<String, usize>) -> usize {
    layer_index.get(object).copied().unwrap_or(usize::MAX)
}

/// Derive a game-class role for a used object from PuzzleScript semantics.
fn classify(object: &str, pushables: &[String], targets: &[String]) -> Result<String, IngestError> {
    let is_player = object == PLAYER_OBJECT;
    let is_background = object == BACKGROUND_OBJECT;
    let is_pushable = pushables.contains(&object.to_string());
    let is_target = targets.contains(&object.to_string());

    // An object cannot be two distinct game-class roles at once.
    let claims = [is_player, is_pushable, is_target]
        .iter()
        .filter(|c| **c)
        .count();
    if claims > 1 {
        return Err(IngestError::Malformed(format!(
            "object \"{object}\" maps to more than one role"
        )));
    }

    if is_player {
        Ok("player".into())
    } else if is_target {
        Ok("target".into())
    } else if is_pushable {
        Ok("pushable".into())
    } else if is_background {
        Ok("background".into())
    } else {
        // Everything else the level uses is a solid obstacle (walls).
        Ok("solid".into())
    }
}

/// Slugify a title into a stable spec id.
fn slug(title: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in title.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

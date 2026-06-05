use anyhow::{anyhow, Context, Result};
use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static LEDGER_APPEND_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub fn append_ledger_event(
    run_dir: impl AsRef<Path>,
    kind: &str,
    actor: &str,
    payload: serde_json::Value,
) -> Result<serde_json::Value> {
    require_text("ledger event kind", kind)?;
    require_text("ledger event actor", actor)?;

    let event = json!({
        "event": kind,
        "actor": actor,
        "payload": payload,
        "created_at_unix_ms": unix_millis()?,
    });
    let ledger_path = run_dir.as_ref().join("ledger.jsonl");
    let line = serde_json::to_string(&event).context("failed to serialize ledger event")?;
    let _guard = LEDGER_APPEND_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(|_| anyhow!("ledger append lock poisoned"))?;
    let mut file = OpenOptions::new()
        .create(false)
        .append(true)
        .open(&ledger_path)
        .with_context(|| format!("failed to open ledger for append {}", ledger_path.display()))?;
    writeln!(file, "{line}").context("failed to append ledger event")?;
    Ok(event)
}

pub fn read_ledger_events(run_dir: impl AsRef<Path>) -> Result<Vec<serde_json::Value>> {
    let ledger_path = run_dir.as_ref().join("ledger.jsonl");
    let file = File::open(&ledger_path)
        .with_context(|| format!("failed to read ledger {}", ledger_path.display()))?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.with_context(|| {
            format!(
                "failed to read ledger line {} from {}",
                line_number + 1,
                ledger_path.display()
            )
        })?;
        if line.trim().is_empty() {
            continue;
        }
        let event: serde_json::Value = serde_json::from_str(&line).with_context(|| {
            format!(
                "failed to parse ledger JSON on line {} from {}",
                line_number + 1,
                ledger_path.display()
            )
        })?;
        events.push(event);
    }

    Ok(events)
}

pub fn write_ledger_created(path: &Path, created_at_unix_ms: u128) -> Result<()> {
    let mut file =
        File::create(path).with_context(|| format!("failed to write {}", path.display()))?;
    let line = serde_json::to_string(&json!({
        "event": "run.created",
        "created_at_unix_ms": created_at_unix_ms,
    }))
    .context("failed to serialize ledger event")?;
    writeln!(file, "{line}").context("failed to write ledger event")
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}

fn unix_millis() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before UNIX_EPOCH")?
        .as_millis())
}

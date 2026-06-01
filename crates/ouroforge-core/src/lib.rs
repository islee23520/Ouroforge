use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tungstenite::client::IntoClientRequest;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub id: String,
    pub title: String,
    pub goal: String,
    pub constraints: Constraints,
    pub acceptance: Vec<String>,
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Constraints {
    pub target: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub id: String,
    pub description: String,
}

impl Seed {
    pub fn from_yaml_str(input: &str) -> Result<Self> {
        let seed: Seed = serde_yaml::from_str(input).context("failed to parse Seed YAML")?;
        seed.validate()?;
        Ok(seed)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let input = fs::read_to_string(path)
            .with_context(|| format!("failed to read Seed file {}", path.display()))?;
        Self::from_yaml_str(&input)
    }

    pub fn validate(&self) -> Result<()> {
        require_text("id", &self.id)?;
        require_text("title", &self.title)?;
        require_text("goal", &self.goal)?;
        require_text("constraints.target", &self.constraints.target)?;

        if self.acceptance.is_empty() {
            return Err(anyhow!("acceptance must contain at least one item"));
        }
        for (index, item) in self.acceptance.iter().enumerate() {
            require_text(&format!("acceptance[{index}]"), item)?;
        }

        if self.scenarios.is_empty() {
            return Err(anyhow!("scenarios must contain at least one item"));
        }
        for (index, scenario) in self.scenarios.iter().enumerate() {
            require_text(&format!("scenarios[{index}].id"), &scenario.id)?;
            require_text(
                &format!("scenarios[{index}].description"),
                &scenario.description,
            )?;
        }

        Ok(())
    }
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceArtifact {
    pub id: String,
    pub kind: String,
    pub path: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub added_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceIndex {
    pub artifacts: Vec<EvidenceArtifact>,
}

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
    let mut file = OpenOptions::new()
        .create(false)
        .append(true)
        .open(&ledger_path)
        .with_context(|| format!("failed to open ledger for append {}", ledger_path.display()))?;
    let line = serde_json::to_string(&event).context("failed to serialize ledger event")?;
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

pub fn add_evidence_artifact(
    run_dir: impl AsRef<Path>,
    id: &str,
    kind: &str,
    path: &str,
    metadata: serde_json::Value,
) -> Result<EvidenceArtifact> {
    require_text("evidence artifact id", id)?;
    require_text("evidence artifact kind", kind)?;
    require_text("evidence artifact path", path)?;

    let mut index = read_evidence_index(&run_dir)?;
    if index.artifacts.iter().any(|artifact| artifact.id == id) {
        return Err(anyhow!("evidence artifact id already exists: {id}"));
    }

    let artifact = EvidenceArtifact {
        id: id.to_string(),
        kind: kind.to_string(),
        path: path.to_string(),
        metadata,
        added_at_unix_ms: unix_millis()?,
    };
    index.artifacts.push(artifact.clone());
    write_evidence_index(run_dir, &index)?;
    Ok(artifact)
}

pub fn list_evidence_artifacts(run_dir: impl AsRef<Path>) -> Result<Vec<EvidenceArtifact>> {
    Ok(read_evidence_index(run_dir)?.artifacts)
}

fn read_evidence_index(run_dir: impl AsRef<Path>) -> Result<EvidenceIndex> {
    let index_path = run_dir.as_ref().join("evidence/index.json");
    let input = fs::read_to_string(&index_path)
        .with_context(|| format!("failed to read evidence index {}", index_path.display()))?;
    let index: EvidenceIndex = serde_json::from_str(&input)
        .with_context(|| format!("failed to parse evidence index {}", index_path.display()))?;
    Ok(index)
}

fn write_evidence_index(run_dir: impl AsRef<Path>, index: &EvidenceIndex) -> Result<()> {
    write_json_atomic(&run_dir.as_ref().join("evidence/index.json"), &json!(index))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdpConnectionConfig {
    pub target_ws_url: String,
    pub io_timeout: Duration,
}

impl CdpConnectionConfig {
    pub fn new(target_ws_url: impl Into<String>) -> Result<Self> {
        let target_ws_url = target_ws_url.into();
        require_text("CDP target WebSocket URL", &target_ws_url)?;
        Ok(Self {
            target_ws_url,
            io_timeout: default_cdp_io_timeout(),
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CdpTargetSelection {
    pub target_id: Option<String>,
}

impl CdpTargetSelection {
    pub fn first_page() -> Self {
        Self::default()
    }

    pub fn target_id(target_id: impl Into<String>) -> Result<Self> {
        let target_id = target_id.into();
        require_text("CDP target id", &target_id)?;
        Ok(Self {
            target_id: Some(target_id),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdpNavigateResult {
    pub frame_id: Option<String>,
    pub loader_id: Option<String>,
}

pub trait CdpTransport {
    fn send_command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value>;
}

pub struct CdpClient<T> {
    transport: T,
}

impl<T: CdpTransport> CdpClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn navigate(&mut self, url: &str) -> Result<CdpNavigateResult> {
        require_text("navigation URL", url)?;
        let result = self
            .transport
            .send_command("Page.navigate", json!({ "url": url }))?;
        if let Some(error_text) = result.get("errorText").and_then(|value| value.as_str()) {
            return Err(anyhow!("CDP navigation failed: {error_text}"));
        }
        Ok(CdpNavigateResult {
            frame_id: result
                .get("frameId")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            loader_id: result
                .get("loaderId")
                .and_then(|value| value.as_str())
                .map(str::to_string),
        })
    }
}

pub struct WebSocketCdpTransport {
    socket: tungstenite::WebSocket<std::net::TcpStream>,
    next_id: u64,
}

impl WebSocketCdpTransport {
    pub fn connect(config: &CdpConnectionConfig) -> Result<Self> {
        let request = config
            .target_ws_url
            .as_str()
            .into_client_request()
            .context("failed to build CDP WebSocket request")?;
        let endpoint = CdpWebSocketEndpoint::parse(&config.target_ws_url)?;
        let stream = endpoint.connect(config.io_timeout)?;
        let (mut socket, _) = tungstenite::client(request, stream)
            .with_context(|| format!("failed to connect to CDP target {}", config.target_ws_url))?;
        set_tcp_stream_timeouts(socket.get_mut(), config.io_timeout)?;
        Ok(Self { socket, next_id: 1 })
    }
}

impl CdpTransport for WebSocketCdpTransport {
    fn send_command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        require_text("CDP method", method)?;
        let id = self.next_id;
        self.next_id += 1;
        let request = json!({
            "id": id,
            "method": method,
            "params": params,
        });
        let request_body =
            serde_json::to_string(&request).context("failed to serialize CDP request")?;
        self.socket
            .send(tungstenite::Message::Text(request_body))
            .context("failed to send CDP request")?;

        loop {
            let message = self.socket.read().context("failed to read CDP response")?;
            let tungstenite::Message::Text(body) = message else {
                continue;
            };
            let response: serde_json::Value =
                serde_json::from_str(&body).context("failed to parse CDP response")?;
            if response.get("id").and_then(|value| value.as_u64()) != Some(id) {
                continue;
            }
            if let Some(error) = response.get("error") {
                return Err(anyhow!("CDP command {method} failed: {error}"));
            }
            return Ok(response.get("result").cloned().unwrap_or_else(|| json!({})));
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CdpTargetInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub target_type: String,
    pub url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: String,
}

pub fn read_cdp_targets(debugging_http_url: &str) -> Result<Vec<CdpTargetInfo>> {
    require_text("CDP debugging HTTP URL", debugging_http_url)?;
    let endpoint = CdpHttpEndpoint::parse(debugging_http_url)?;
    let body = endpoint.get("/json/list")?;
    parse_cdp_targets(&body)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CdpHttpEndpoint {
    host: IpAddr,
    port: u16,
    timeout: Duration,
}

impl CdpHttpEndpoint {
    fn parse(input: &str) -> Result<Self> {
        let without_scheme = input
            .trim()
            .strip_prefix("http://")
            .ok_or_else(|| anyhow!("CDP debugging URL must start with http://"))?
            .trim_end_matches('/');
        let (host, port) = parse_host_port("CDP debugging URL", without_scheme)?;
        Ok(Self {
            host,
            port,
            timeout: default_cdp_io_timeout(),
        })
    }

    fn get(&self, path: &str) -> Result<String> {
        let mut stream =
            connect_with_timeout(self.host, self.port, self.timeout).with_context(|| {
                format!(
                    "failed to connect to CDP HTTP endpoint {}:{}",
                    self.host, self.port
                )
            })?;
        set_tcp_stream_timeouts(&stream, self.timeout)?;
        write!(
            stream,
            "GET {path} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            format_host_authority(self.host, self.port)
        )
        .context("failed to write CDP HTTP request")?;

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .context("failed to read CDP HTTP response")?;
        let (headers, body) = response
            .split_once("\r\n\r\n")
            .ok_or_else(|| anyhow!("invalid CDP HTTP response"))?;
        if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
            return Err(anyhow!("CDP HTTP request failed: {headers}"));
        }
        Ok(body.to_string())
    }
}

pub fn select_page_target(
    targets: &[CdpTargetInfo],
    selection: &CdpTargetSelection,
) -> Result<CdpConnectionConfig> {
    let target = targets
        .iter()
        .find(|target| {
            let id_matches = selection
                .target_id
                .as_ref()
                .is_none_or(|target_id| target.id == *target_id);
            id_matches && target.target_type == "page" && !target.web_socket_debugger_url.is_empty()
        })
        .ok_or_else(|| {
            anyhow!("no matching page CDP target with a WebSocket debugger URL found")
        })?;
    CdpConnectionConfig::new(target.web_socket_debugger_url.clone())
}

pub fn first_page_target(targets: &[CdpTargetInfo]) -> Result<CdpConnectionConfig> {
    select_page_target(targets, &CdpTargetSelection::first_page())
}

fn default_cdp_io_timeout() -> Duration {
    Duration::from_secs(10)
}

fn set_tcp_stream_timeouts(stream: &std::net::TcpStream, timeout: Duration) -> Result<()> {
    stream
        .set_read_timeout(Some(timeout))
        .context("failed to set CDP read timeout")?;
    stream
        .set_write_timeout(Some(timeout))
        .context("failed to set CDP write timeout")
}

fn format_host_authority(host: IpAddr, port: u16) -> String {
    match host {
        IpAddr::V4(addr) => format!("{addr}:{port}"),
        IpAddr::V6(addr) => format!("[{addr}]:{port}"),
    }
}

fn connect_with_timeout(host: IpAddr, port: u16, timeout: Duration) -> Result<std::net::TcpStream> {
    let addr = SocketAddr::new(host, port);
    std::net::TcpStream::connect_timeout(&addr, timeout)
        .with_context(|| format!("failed to connect to {addr} within {timeout:?}"))
}

fn parse_host_port(label: &str, authority: &str) -> Result<(IpAddr, u16)> {
    if let Some(rest) = authority.strip_prefix('[') {
        let (host, port_part) = rest
            .split_once(']')
            .ok_or_else(|| anyhow!("{label} has an unterminated IPv6 host"))?;
        let port = port_part
            .strip_prefix(':')
            .ok_or_else(|| anyhow!("{label} must include host:port"))?;
        return Ok((
            parse_loopback_ip(label, host)?,
            port.parse::<u16>()
                .with_context(|| format!("invalid {label} port: {port}"))?,
        ));
    }

    let (host, port) = authority
        .rsplit_once(':')
        .ok_or_else(|| anyhow!("{label} must include host:port"))?;
    if host.contains(':') {
        return Err(anyhow!("{label} IPv6 hosts must be bracketed"));
    }
    Ok((
        parse_loopback_ip(label, host)?,
        port.parse::<u16>()
            .with_context(|| format!("invalid {label} port: {port}"))?,
    ))
}

fn parse_loopback_ip(field: &str, value: &str) -> Result<IpAddr> {
    require_text(field, value)?;
    let ip = value
        .parse::<IpAddr>()
        .with_context(|| format!("{field} must be a numeric loopback IP address"))?;
    if !ip.is_loopback() {
        return Err(anyhow!("{field} must be a loopback IP address"));
    }
    Ok(ip)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CdpWebSocketEndpoint {
    host: IpAddr,
    port: u16,
    _path: String,
}

impl CdpWebSocketEndpoint {
    fn parse(input: &str) -> Result<Self> {
        let without_scheme = input
            .trim()
            .strip_prefix("ws://")
            .ok_or_else(|| anyhow!("CDP WebSocket URL must start with ws://"))?;
        let (authority, path) = without_scheme
            .split_once('/')
            .unwrap_or((without_scheme, ""));
        let (host, port) = parse_host_port("CDP WebSocket URL", authority)?;
        Ok(Self {
            host,
            port,
            _path: format!("/{path}"),
        })
    }

    fn connect(&self, timeout: Duration) -> Result<std::net::TcpStream> {
        let stream = connect_with_timeout(self.host, self.port, timeout)?;
        set_tcp_stream_timeouts(&stream, timeout)?;
        Ok(stream)
    }
}

fn parse_cdp_targets(input: &str) -> Result<Vec<CdpTargetInfo>> {
    serde_json::from_str(input).context("failed to parse CDP target list")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunArtifacts {
    pub run_dir: PathBuf,
}

pub fn create_run(
    seed_path: impl AsRef<Path>,
    runs_root: impl AsRef<Path>,
) -> Result<RunArtifacts> {
    let seed_path = seed_path.as_ref();
    let runs_root = runs_root.as_ref();
    let seed_yaml = fs::read_to_string(seed_path)
        .with_context(|| format!("failed to read Seed file {}", seed_path.display()))?;
    let seed = Seed::from_yaml_str(&seed_yaml)?;

    fs::create_dir_all(runs_root)
        .with_context(|| format!("failed to create runs root {}", runs_root.display()))?;

    let created_at_unix_ms = unix_millis()?;
    let run_id = format!("run-{created_at_unix_ms}-{}", std::process::id());
    let run_dir = runs_root.join(&run_id);
    fs::create_dir(&run_dir)
        .with_context(|| format!("failed to create run directory {}", run_dir.display()))?;
    fs::create_dir(run_dir.join("evidence")).context("failed to create evidence directory")?;

    write_json(
        &run_dir.join("run.json"),
        &json!({
            "id": run_id,
            "seed_id": seed.id,
            "seed_title": seed.title,
            "status": "created",
            "created_at_unix_ms": created_at_unix_ms,
        }),
    )?;
    fs::write(run_dir.join("seed.snapshot.yaml"), seed_yaml)
        .context("failed to write seed snapshot")?;
    write_ledger_created(&run_dir.join("ledger.jsonl"), created_at_unix_ms)?;
    fs::write(run_dir.join("journal.md"), initial_journal()).context("failed to write journal")?;
    write_json(
        &run_dir.join("verdict.json"),
        &json!({ "status": "pending" }),
    )?;
    write_evidence_index(
        &run_dir,
        &EvidenceIndex {
            artifacts: Vec::new(),
        },
    )?;

    Ok(RunArtifacts { run_dir })
}

fn unix_millis() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before UNIX_EPOCH")?
        .as_millis())
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<()> {
    let body = serde_json::to_string_pretty(value).context("failed to serialize JSON")?;
    fs::write(path, format!("{body}\n"))
        .with_context(|| format!("failed to write {}", path.display()))
}

fn write_json_atomic(path: &Path, value: &serde_json::Value) -> Result<()> {
    let body = serde_json::to_string_pretty(value).context("failed to serialize JSON")?;
    let temp_path = path.with_extension(format!(
        "json.tmp-{}-{}",
        std::process::id(),
        unix_millis()?
    ));
    fs::write(&temp_path, format!("{body}\n"))
        .with_context(|| format!("failed to write {}", temp_path.display()))?;
    fs::rename(&temp_path, path).with_context(|| {
        format!(
            "failed to replace {} with {}",
            path.display(),
            temp_path.display()
        )
    })
}

fn write_ledger_created(path: &Path, created_at_unix_ms: u128) -> Result<()> {
    let mut file =
        File::create(path).with_context(|| format!("failed to write {}", path.display()))?;
    let line = serde_json::to_string(&json!({
        "event": "run.created",
        "created_at_unix_ms": created_at_unix_ms,
    }))
    .context("failed to serialize ledger event")?;
    writeln!(file, "{line}").context("failed to write ledger event")
}

fn initial_journal() -> &'static str {
    "# Ouroforge Run Journal\n\n## Seed\n\n## Hypothesis\n\n## Observations\n\n## Evidence\n\n## Verdict\n\n## Next Mutation\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_SEED: &str = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

    #[test]
    fn parses_valid_seed() {
        let seed = Seed::from_yaml_str(VALID_SEED).expect("valid seed parses");
        assert_eq!(seed.id, "platformer.v0");
        assert_eq!(seed.constraints.target, "file-harness");
    }

    #[test]
    fn rejects_seed_missing_required_target() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints: {}
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("missing target fails");
        assert!(error.to_string().contains("failed to parse Seed YAML"));
    }

    #[test]
    fn rejects_seed_with_unknown_fields() {
        let invalid = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
future_scope: should-not-be-accepted
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

        let error = Seed::from_yaml_str(invalid).expect_err("unknown fields fail");
        assert!(error.to_string().contains("failed to parse Seed YAML"));
    }

    #[test]
    fn creates_required_run_artifacts() {
        let root = unique_temp_dir("ouroforge-core-test");
        fs::create_dir_all(&root).expect("temp root exists");
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, VALID_SEED).expect("seed written");
        let runs_root = root.join("runs");

        let artifacts = create_run(&seed_path, &runs_root).expect("run artifacts created");

        assert!(artifacts.run_dir.join("run.json").is_file());
        assert!(artifacts.run_dir.join("seed.snapshot.yaml").is_file());
        assert!(artifacts.run_dir.join("ledger.jsonl").is_file());
        assert!(artifacts.run_dir.join("journal.md").is_file());
        assert!(artifacts.run_dir.join("verdict.json").is_file());
        assert!(artifacts.run_dir.join("evidence/index.json").is_file());

        let ledger = fs::read_to_string(artifacts.run_dir.join("ledger.jsonl")).unwrap();
        let first_event: serde_json::Value =
            serde_json::from_str(ledger.lines().next().unwrap()).unwrap();
        assert_eq!(first_event["event"], "run.created");

        let evidence = fs::read_to_string(artifacts.run_dir.join("evidence/index.json")).unwrap();
        let evidence_index: serde_json::Value = serde_json::from_str(&evidence).unwrap();
        assert_eq!(evidence_index["artifacts"].as_array().unwrap().len(), 0);

        let journal = fs::read_to_string(artifacts.run_dir.join("journal.md")).unwrap();
        for heading in [
            "## Seed",
            "## Hypothesis",
            "## Observations",
            "## Evidence",
            "## Verdict",
            "## Next Mutation",
        ] {
            assert!(journal.contains(heading), "journal missing {heading}");
        }

        let verdict = fs::read_to_string(artifacts.run_dir.join("verdict.json")).unwrap();
        let verdict_json: serde_json::Value = serde_json::from_str(&verdict).unwrap();
        assert_eq!(verdict_json["status"], "pending");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn appends_and_reads_ledger_events() {
        let (root, artifacts) = create_test_run("ouroforge-ledger-test");

        append_ledger_event(
            &artifacts.run_dir,
            "test.event",
            "test",
            json!({ "ok": true }),
        )
        .expect("first event appended");
        append_ledger_event(&artifacts.run_dir, "test.second", "test", json!({}))
            .expect("second event appended");

        let events = read_ledger_events(&artifacts.run_dir).expect("ledger reads");
        assert_eq!(events.len(), 3);
        assert_eq!(events[0]["event"], "run.created");
        assert_eq!(events[1]["event"], "test.event");
        assert_eq!(events[1]["actor"], "test");
        assert_eq!(events[1]["payload"], json!({ "ok": true }));
        assert_eq!(events[2]["event"], "test.second");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_malformed_ledger_events() {
        let (root, artifacts) = create_test_run("ouroforge-bad-ledger-test");
        fs::write(artifacts.run_dir.join("ledger.jsonl"), "not-json\n")
            .expect("bad ledger written");

        let error = read_ledger_events(&artifacts.run_dir).expect_err("bad ledger fails");
        assert!(error.to_string().contains("failed to parse ledger JSON"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn adds_and_lists_evidence_artifacts() {
        let (root, artifacts) = create_test_run("ouroforge-evidence-test");

        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/artifact-1.txt",
            json!({ "source": "unit-test" }),
        )
        .expect("first evidence added");
        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-2",
            "application/json",
            "evidence/artifact-2.json",
            json!({}),
        )
        .expect("second evidence added");

        let artifacts_list = list_evidence_artifacts(&artifacts.run_dir).expect("evidence lists");
        assert_eq!(artifacts_list.len(), 2);
        assert_eq!(artifacts_list[0].id, "artifact-1");
        assert_eq!(artifacts_list[0].metadata, json!({ "source": "unit-test" }));
        assert_eq!(artifacts_list[1].kind, "application/json");

        let index = fs::read_to_string(artifacts.run_dir.join("evidence/index.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&index).unwrap();
        assert_eq!(parsed["artifacts"].as_array().unwrap().len(), 2);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_malformed_evidence_index_and_duplicate_ids() {
        let (root, artifacts) = create_test_run("ouroforge-bad-evidence-test");

        add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/artifact-1.txt",
            json!({}),
        )
        .expect("evidence added");
        let duplicate = add_evidence_artifact(
            &artifacts.run_dir,
            "artifact-1",
            "text/plain",
            "evidence/duplicate.txt",
            json!({}),
        )
        .expect_err("duplicate id fails");
        assert!(duplicate.to_string().contains("already exists"));

        fs::write(
            artifacts.run_dir.join("evidence/index.json"),
            r#"{"artifacts":"not-an-array"}"#,
        )
        .expect("bad evidence index written");
        let error = list_evidence_artifacts(&artifacts.run_dir).expect_err("bad index fails");
        assert!(error.to_string().contains("failed to parse evidence index"));

        fs::remove_dir_all(root).ok();
    }

    #[derive(Default)]
    struct MockCdpTransport {
        calls: Vec<(String, serde_json::Value)>,
    }

    impl CdpTransport for MockCdpTransport {
        fn send_command(
            &mut self,
            method: &str,
            params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            self.calls.push((method.to_string(), params));
            Ok(json!({ "frameId": "frame-1", "loaderId": "loader-1" }))
        }
    }

    struct FailingNavigateTransport;

    impl CdpTransport for FailingNavigateTransport {
        fn send_command(
            &mut self,
            _method: &str,
            _params: serde_json::Value,
        ) -> Result<serde_json::Value> {
            Ok(json!({ "frameId": "frame-1", "errorText": "net::ERR_CONNECTION_REFUSED" }))
        }
    }

    #[test]
    fn cdp_client_navigates_through_transport_boundary() {
        let transport = MockCdpTransport::default();
        let mut client = CdpClient::new(transport);

        let result = client
            .navigate("http://localhost:8000")
            .expect("navigation command succeeds");

        assert_eq!(result.frame_id.as_deref(), Some("frame-1"));
        assert_eq!(result.loader_id.as_deref(), Some("loader-1"));
        assert_eq!(client.transport.calls.len(), 1);
        assert_eq!(client.transport.calls[0].0, "Page.navigate");
        assert_eq!(
            client.transport.calls[0].1,
            json!({ "url": "http://localhost:8000" })
        );
    }

    #[test]
    fn cdp_client_reports_navigation_error_text() {
        let mut client = CdpClient::new(FailingNavigateTransport);

        let error = client
            .navigate("http://localhost:9")
            .expect_err("navigation errorText fails");

        assert!(error.to_string().contains("net::ERR_CONNECTION_REFUSED"));
    }

    #[test]
    fn parses_cdp_websocket_endpoint() {
        let endpoint = CdpWebSocketEndpoint::parse("ws://127.0.0.1:9222/devtools/page/page-1")
            .expect("websocket endpoint parses");
        assert_eq!(endpoint.host, "127.0.0.1".parse::<IpAddr>().unwrap());
        assert_eq!(endpoint.port, 9222);
    }

    #[test]
    fn formats_ipv6_host_authority_with_brackets() {
        assert_eq!(
            format_host_authority("::1".parse::<IpAddr>().unwrap(), 9222),
            "[::1]:9222"
        );
        assert_eq!(
            format_host_authority("127.0.0.1".parse::<IpAddr>().unwrap(), 9222),
            "127.0.0.1:9222"
        );
    }

    #[test]
    fn parses_ipv6_cdp_endpoints() {
        let http = CdpHttpEndpoint::parse("http://[::1]:9222/").expect("ipv6 http parses");
        assert_eq!(http.host, "::1".parse::<IpAddr>().unwrap());
        assert_eq!(http.port, 9222);

        let websocket = CdpWebSocketEndpoint::parse("ws://[::1]:9222/devtools/page/page-1")
            .expect("ipv6 websocket parses");
        assert_eq!(websocket.host, "::1".parse::<IpAddr>().unwrap());
        assert_eq!(websocket.port, 9222);
    }

    #[test]
    fn parses_cdp_http_endpoint() {
        let endpoint = CdpHttpEndpoint::parse("http://127.0.0.1:9222/").expect("endpoint parses");
        assert_eq!(endpoint.host, "127.0.0.1".parse::<IpAddr>().unwrap());
        assert_eq!(endpoint.port, 9222);
    }

    #[test]
    fn rejects_hostname_cdp_endpoint() {
        let error =
            CdpHttpEndpoint::parse("http://localhost:9222").expect_err("hostname endpoint fails");
        assert!(error
            .to_string()
            .contains("must be a numeric loopback IP address"));
    }

    #[test]
    fn rejects_non_http_cdp_endpoint() {
        let error =
            CdpHttpEndpoint::parse("https://127.0.0.1:9222").expect_err("https endpoint fails");
        assert!(error.to_string().contains("must start with http://"));
    }

    #[test]
    fn parses_first_page_cdp_target() {
        let targets = parse_cdp_targets(
            r#"[
              {
                "id":"browser",
                "type":"browser",
                "url":"",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/browser/abc"
              },
              {
                "id":"page-1",
                "type":"page",
                "url":"about:blank",
                "title":"New Tab",
                "description":"",
                "devtoolsFrontendUrl":"/devtools/inspector.html?ws=127.0.0.1:9222/devtools/page/page-1",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/page/page-1"
              }
            ]"#,
        )
        .expect("targets parse");

        let config = first_page_target(&targets).expect("page target selected");
        assert_eq!(
            config.target_ws_url,
            "ws://127.0.0.1:9222/devtools/page/page-1"
        );
    }

    #[test]
    fn selects_configured_page_cdp_target() {
        let targets = parse_cdp_targets(
            r#"[
              {
                "id":"page-1",
                "type":"page",
                "url":"http://wrong.example",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/page/page-1"
              },
              {
                "id":"page-2",
                "type":"page",
                "url":"http://right.example",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/page/page-2"
              }
            ]"#,
        )
        .expect("targets parse");
        let selection = CdpTargetSelection::target_id("page-2").expect("selection parses");

        let config = select_page_target(&targets, &selection).expect("configured target selected");

        assert_eq!(
            config.target_ws_url,
            "ws://127.0.0.1:9222/devtools/page/page-2"
        );
    }

    #[test]
    fn rejects_missing_page_cdp_target() {
        let targets = parse_cdp_targets(
            r#"[
              {
                "id":"browser",
                "type":"browser",
                "url":"",
                "webSocketDebuggerUrl":"ws://127.0.0.1:9222/devtools/browser/abc"
              }
            ]"#,
        )
        .expect("targets parse");

        let error = first_page_target(&targets).expect_err("missing page fails");
        assert!(error.to_string().contains("no matching page CDP target"));
    }

    fn create_test_run(prefix: &str) -> (PathBuf, RunArtifacts) {
        let root = unique_temp_dir(prefix);
        fs::create_dir_all(&root).expect("temp root exists");
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, VALID_SEED).expect("seed written");
        let runs_root = root.join("runs");
        let artifacts = create_run(&seed_path, &runs_root).expect("run artifacts created");
        (root, artifacts)
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{}",
            std::process::id(),
            unix_millis().expect("time works")
        ))
    }
}

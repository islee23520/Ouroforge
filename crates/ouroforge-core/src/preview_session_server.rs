//! Loopback HTTP server for Preview Session v1 (M131.1, Era X #2518).
//!
//! `ouroforge preview serve` wraps this module: a single-session,
//! single-client-at-a-time accept loop that validates manipulation intents
//! through [`crate::preview_session`] and answers with normalized deltas.
//! The server binds loopback only, holds all state in memory, and performs
//! no filesystem writes; shutdown is an explicit `POST /shutdown` or process
//! termination. Intents carry no file or shell authority - this is not a
//! command bridge.

use crate::preview_session::{
    apply_preview_intent, start_preview_session, PreviewIntent, PreviewSession,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tungstenite::handshake::derive_accept_key;
use tungstenite::protocol::Role;
use tungstenite::{Message, WebSocket};

pub const PREVIEW_SERVER_STATUS_SCHEMA_VERSION: &str = "ouroforge.preview-server-status.v1";

const MAX_REQUEST_BODY_BYTES: usize = 1024 * 1024;
const STREAM_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct PreviewServerConfig {
    pub scene_path: PathBuf,
    pub host: IpAddr,
    pub port: u16,
    pub session_id: String,
}

impl PreviewServerConfig {
    pub fn new(scene_path: impl Into<PathBuf>, session_id: &str) -> Self {
        Self {
            scene_path: scene_path.into(),
            host: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 0,
            session_id: session_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PreviewServerStatus {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub status: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "scenePath")]
    pub scene_path: String,
    pub sequence: u64,
    #[serde(rename = "currentSceneHash")]
    pub current_scene_hash: String,
    #[serde(rename = "baseSceneHash")]
    pub base_scene_hash: String,
}

/// Summary returned when the serve loop exits cleanly.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PreviewServerReport {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "intentsApplied")]
    pub intents_applied: u64,
    #[serde(rename = "intentsRejected")]
    pub intents_rejected: u64,
    #[serde(rename = "envelopeErrors")]
    pub envelope_errors: u64,
    #[serde(rename = "deltasBroadcast")]
    pub deltas_broadcast: u64,
    #[serde(rename = "acksReceived")]
    pub acks_received: u64,
    #[serde(rename = "shutdownReason")]
    pub shutdown_reason: String,
}

enum ConnectionOutcome {
    Continue,
    Shutdown,
    Subscribe(Box<WebSocket<TcpStream>>),
    Broadcast(String),
}

pub struct PreviewServer {
    listener: TcpListener,
    session: PreviewSession,
    subscribers: Vec<WebSocket<TcpStream>>,
    applied: u64,
    rejected: u64,
    envelope_errors: u64,
    deltas_broadcast: u64,
    acks_received: u64,
}

impl PreviewServer {
    /// Bind the loopback listener and start the in-memory session. Fails
    /// closed on non-loopback hosts and on invalid base scenes.
    pub fn bind(config: &PreviewServerConfig) -> Result<Self> {
        if !config.host.is_loopback() {
            return Err(anyhow!("preview server host must be a loopback address"));
        }
        let session = start_preview_session(&config.scene_path, &config.session_id)?;
        let listener = TcpListener::bind(SocketAddr::new(config.host, config.port))
            .context("failed to bind preview server loopback listener")?;
        Ok(Self {
            listener,
            session,
            subscribers: Vec::new(),
            applied: 0,
            rejected: 0,
            envelope_errors: 0,
            deltas_broadcast: 0,
            acks_received: 0,
        })
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.listener
            .local_addr()
            .context("preview server has no local address")
    }

    pub fn status(&self) -> PreviewServerStatus {
        PreviewServerStatus {
            schema_version: PREVIEW_SERVER_STATUS_SCHEMA_VERSION.to_string(),
            status: "serving".to_string(),
            session_id: self.session.session_id.clone(),
            scene_path: self.session.scene_path.to_string_lossy().to_string(),
            sequence: self.session.sequence,
            current_scene_hash: self.session.current_scene_hash.value.clone(),
            base_scene_hash: self.session.base_scene_hash.value.clone(),
        }
    }

    /// Serve requests until `POST /shutdown`. HTTP connections are handled
    /// one at a time with `Connection: close` JSON responses; `GET /channel`
    /// upgrades to a receive-only WebSocket subscription that gets every
    /// subsequent delta pushed as a JSON text frame.
    pub fn serve_until_shutdown(mut self) -> Result<PreviewServerReport> {
        loop {
            let (stream, _peer) = self
                .listener
                .accept()
                .context("preview server accept failed")?;
            match self.handle_connection(stream) {
                Ok(ConnectionOutcome::Shutdown) => {
                    self.drain_acks(Duration::from_millis(500));
                    for subscriber in &mut self.subscribers {
                        let _ = subscriber.close(None);
                    }
                    return Ok(PreviewServerReport {
                        session_id: self.session.session_id.clone(),
                        intents_applied: self.applied,
                        intents_rejected: self.rejected,
                        envelope_errors: self.envelope_errors,
                        deltas_broadcast: self.deltas_broadcast,
                        acks_received: self.acks_received,
                        shutdown_reason: "shutdown-requested".to_string(),
                    });
                }
                Ok(ConnectionOutcome::Subscribe(subscriber)) => {
                    self.subscribers.push(*subscriber);
                }
                Ok(ConnectionOutcome::Broadcast(delta_json)) => {
                    self.drain_acks(Duration::ZERO);
                    self.broadcast(&delta_json);
                }
                Ok(ConnectionOutcome::Continue) => {}
                // Per-connection IO errors (client vanished mid-request) must
                // not kill the long-lived session.
                Err(_) => {}
            }
        }
    }

    /// Push one delta JSON frame to every subscriber, pruning dead sockets.
    fn broadcast(&mut self, delta_json: &str) {
        if self.subscribers.is_empty() {
            return;
        }
        let mut alive = Vec::with_capacity(self.subscribers.len());
        for mut subscriber in self.subscribers.drain(..) {
            if subscriber
                .send(Message::Text(delta_json.to_string()))
                .is_ok()
            {
                alive.push(subscriber);
            }
        }
        if !alive.is_empty() {
            self.deltas_broadcast += 1;
        }
        self.subscribers = alive;
    }

    /// Best-effort, non-blocking drain of client acknowledgement frames.
    /// Acks are instrumentation evidence, not a delivery protocol: losing one
    /// never affects session state.
    fn drain_acks(&mut self, patience: Duration) {
        if self.subscribers.is_empty() {
            return;
        }
        let deadline = Instant::now() + patience;
        loop {
            let mut drained_any = false;
            for subscriber in &mut self.subscribers {
                if subscriber.get_ref().set_nonblocking(true).is_err() {
                    continue;
                }
                while let Ok(message) = subscriber.read() {
                    if let Message::Text(text) = message {
                        if text.contains("\"type\":\"ack\"") || text.contains("\"type\": \"ack\"") {
                            self.acks_received += 1;
                            drained_any = true;
                        }
                    }
                }
                let _ = subscriber.get_ref().set_nonblocking(false);
            }
            if drained_any || Instant::now() >= deadline {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn handle_connection(&mut self, stream: TcpStream) -> Result<ConnectionOutcome> {
        stream.set_read_timeout(Some(STREAM_TIMEOUT)).ok();
        stream.set_write_timeout(Some(STREAM_TIMEOUT)).ok();
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .context("failed to read request line")?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or_default().to_string();
        let path = parts.next().unwrap_or_default().to_string();

        let mut content_length = 0usize;
        let mut websocket_key: Option<String> = None;
        loop {
            let mut header = String::new();
            reader
                .read_line(&mut header)
                .context("failed to read header")?;
            let header = header.trim();
            if header.is_empty() {
                break;
            }
            let lowered = header.to_ascii_lowercase();
            if let Some(value) = lowered
                .strip_prefix("content-length:")
                .map(str::trim)
                .map(str::to_string)
            {
                content_length = value
                    .parse::<usize>()
                    .context("invalid content-length header")?;
            }
            if lowered.starts_with("sec-websocket-key:") {
                websocket_key = header
                    .splitn(2, ':')
                    .nth(1)
                    .map(str::trim)
                    .map(str::to_string);
            }
        }
        if content_length > MAX_REQUEST_BODY_BYTES {
            let stream = reader.into_inner();
            respond_json(
                stream,
                413,
                &json!({"error": "request body exceeds preview server limit"}),
            )?;
            return Ok(ConnectionOutcome::Continue);
        }
        let mut body = vec![0u8; content_length];
        if content_length > 0 {
            reader
                .read_exact(&mut body)
                .context("failed to read request body")?;
        }
        let stream = reader.into_inner();

        match (method.as_str(), path.as_str()) {
            ("GET", "/healthz") | ("GET", "/session") => {
                respond_json(stream, 200, &json!(self.status()))?;
                Ok(ConnectionOutcome::Continue)
            }
            ("GET", "/channel") => {
                let Some(key) = websocket_key else {
                    respond_json(
                        stream,
                        400,
                        &json!({"error": "GET /channel requires a WebSocket upgrade request"}),
                    )?;
                    return Ok(ConnectionOutcome::Continue);
                };
                let accept = derive_accept_key(key.as_bytes());
                let mut stream = stream;
                stream
                    .write_all(
                        format!(
                            "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
                        )
                        .as_bytes(),
                    )
                    .context("failed to write WebSocket upgrade response")?;
                let websocket = WebSocket::from_raw_socket(stream, Role::Server, None);
                Ok(ConnectionOutcome::Subscribe(Box::new(websocket)))
            }
            ("POST", "/intent") => {
                let intent: PreviewIntent = match serde_json::from_slice(&body) {
                    Ok(intent) => intent,
                    Err(error) => {
                        self.envelope_errors += 1;
                        respond_json(
                            stream,
                            400,
                            &json!({"error": format!("invalid preview intent: {error}")}),
                        )?;
                        return Ok(ConnectionOutcome::Continue);
                    }
                };
                match apply_preview_intent(&mut self.session, &intent) {
                    Ok(delta) => {
                        match delta.status {
                            crate::preview_session::PreviewDeltaStatus::Applied => {
                                self.applied += 1
                            }
                            crate::preview_session::PreviewDeltaStatus::Rejected => {
                                self.rejected += 1
                            }
                        }
                        let delta_json = serde_json::to_string(&delta)
                            .context("failed to encode preview delta")?;
                        respond_json(stream, 200, &json!(delta))?;
                        Ok(ConnectionOutcome::Broadcast(delta_json))
                    }
                    Err(error) => {
                        self.envelope_errors += 1;
                        respond_json(stream, 400, &json!({"error": error.to_string()}))?;
                        Ok(ConnectionOutcome::Continue)
                    }
                }
            }
            ("POST", "/shutdown") => {
                respond_json(stream, 200, &json!({"status": "shutting-down"}))?;
                Ok(ConnectionOutcome::Shutdown)
            }
            _ => {
                respond_json(
                    stream,
                    404,
                    &json!({"error": format!("unknown preview endpoint {method} {path}")}),
                )?;
                Ok(ConnectionOutcome::Continue)
            }
        }
    }
}

fn respond_json(mut stream: TcpStream, status: u16, body: &serde_json::Value) -> Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        413 => "Payload Too Large",
        _ => "Error",
    };
    let payload = serde_json::to_vec(body).context("failed to encode preview response")?;
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    );
    stream
        .write_all(header.as_bytes())
        .and_then(|_| stream.write_all(&payload))
        .context("failed to write preview response")?;
    Ok(())
}

/// Minimal loopback HTTP client for `preview status` / `preview stop`.
pub fn preview_http_request(url: &str, method: &str, path: &str) -> Result<serde_json::Value> {
    let authority = url
        .trim()
        .strip_prefix("http://")
        .ok_or_else(|| anyhow!("preview server url must start with http://"))?
        .trim_end_matches('/');
    let (host, port) = crate::parse_host_port("preview server url", authority)?;
    let mut stream = crate::connect_with_timeout(host, port, STREAM_TIMEOUT)?;
    stream.set_read_timeout(Some(STREAM_TIMEOUT)).ok();
    stream.set_write_timeout(Some(STREAM_TIMEOUT)).ok();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .context("failed to send preview request")?;
    let mut response = String::new();
    BufReader::new(stream)
        .read_to_string(&mut response)
        .context("failed to read preview response")?;
    let body = response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .unwrap_or_default();
    serde_json::from_str(body).context("preview server returned non-JSON body")
}

/// Send one intent to a running preview server and return the raw JSON
/// response (a delta on 200, an error object otherwise).
pub fn preview_send_intent(url: &str, intent: &PreviewIntent) -> Result<serde_json::Value> {
    let authority = url
        .trim()
        .strip_prefix("http://")
        .ok_or_else(|| anyhow!("preview server url must start with http://"))?
        .trim_end_matches('/');
    let (host, port) = crate::parse_host_port("preview server url", authority)?;
    let mut stream = crate::connect_with_timeout(host, port, STREAM_TIMEOUT)?;
    stream.set_read_timeout(Some(STREAM_TIMEOUT)).ok();
    stream.set_write_timeout(Some(STREAM_TIMEOUT)).ok();
    let payload = serde_json::to_vec(intent).context("failed to encode preview intent")?;
    let request = format!(
        "POST /intent HTTP/1.1\r\nHost: {authority}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    );
    stream
        .write_all(request.as_bytes())
        .and_then(|_| stream.write_all(&payload))
        .context("failed to send preview intent")?;
    let mut response = String::new();
    BufReader::new(stream)
        .read_to_string(&mut response)
        .context("failed to read preview intent response")?;
    let body = response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .unwrap_or_default();
    serde_json::from_str(body).context("preview server returned non-JSON body")
}

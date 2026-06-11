//! M131.2 channel push contract (Era X #2519, PR 2).
//!
//! Locks the WebSocket subscription behavior of `ouroforge preview serve`:
//! `GET /channel` upgrades to a receive-only subscription, every validated
//! delta is pushed as a JSON text frame, client acks are drained as
//! best-effort instrumentation, and the shutdown report reconciles broadcast
//! and ack counts.

use ouroforge_core::preview_session::{
    PreviewIntent, PreviewIntentPayload, PREVIEW_INTENT_SCHEMA_VERSION,
};
use ouroforge_core::preview_session_server::{
    preview_http_request, preview_send_intent, PreviewServer, PreviewServerConfig,
};
use serde_json::json;
use std::path::PathBuf;

fn fixture_scene() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json")
}

#[test]
fn channel_subscriber_receives_pushed_deltas() {
    let config = PreviewServerConfig::new(fixture_scene(), "channel-session");
    let server = PreviewServer::bind(&config).expect("server binds");
    let addr = server.local_addr().unwrap();
    let url = format!("http://{addr}");
    let serve_thread = std::thread::spawn(move || server.serve_until_shutdown());

    let (mut subscriber, _response) =
        tungstenite::connect(format!("ws://{addr}/channel")).expect("WebSocket upgrade succeeds");

    let delta = preview_send_intent(
        &url,
        &PreviewIntent {
            schema_version: PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
            intent_id: "channel-intent".to_string(),
            session_id: "channel-session".to_string(),
            payload: PreviewIntentPayload::ParameterSet {
                entity_id: "player".to_string(),
                path: "components.input.moveSpeed".to_string(),
                value: json!(9),
            },
        },
    )
    .unwrap();
    assert_eq!(delta["status"], "applied");

    let frame = subscriber
        .read()
        .expect("subscriber receives the pushed delta frame");
    let text = frame.into_text().expect("delta frame is text");
    let pushed: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(pushed["schemaVersion"], "ouroforge.preview-delta.v1");
    assert_eq!(pushed["deltaId"], delta["deltaId"]);
    assert_eq!(pushed["status"], "applied");
    assert_eq!(pushed["edits"][0]["path"], "components.input.moveSpeed");

    subscriber
        .send(tungstenite::Message::Text(
            json!({
                "type": "ack",
                "schemaVersion": "ouroforge.preview-ack.v1",
                "deltaId": pushed["deltaId"],
                "receivedAt": 1,
                "appliedAt": 2
            })
            .to_string(),
        ))
        .expect("ack send succeeds");

    let stopping = preview_http_request(&url, "POST", "/shutdown").unwrap();
    assert_eq!(stopping["status"], "shutting-down");

    let report = serve_thread.join().unwrap().unwrap();
    assert_eq!(report.intents_applied, 1);
    assert_eq!(report.deltas_broadcast, 1);
    assert_eq!(
        report.acks_received, 1,
        "the shutdown drain must collect the subscriber ack"
    );
}

#[test]
fn channel_requires_websocket_upgrade_headers() {
    let config = PreviewServerConfig::new(fixture_scene(), "channel-session");
    let server = PreviewServer::bind(&config).expect("server binds");
    let addr = server.local_addr().unwrap();
    let url = format!("http://{addr}");
    let serve_thread = std::thread::spawn(move || server.serve_until_shutdown());

    let plain = preview_http_request(&url, "GET", "/channel").unwrap();
    assert!(plain["error"]
        .as_str()
        .unwrap()
        .contains("requires a WebSocket upgrade"));

    let stopping = preview_http_request(&url, "POST", "/shutdown").unwrap();
    assert_eq!(stopping["status"], "shutting-down");
    let report = serve_thread.join().unwrap().unwrap();
    assert_eq!(report.deltas_broadcast, 0);
}

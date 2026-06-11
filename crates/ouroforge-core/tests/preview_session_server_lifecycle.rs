//! M131.1 serve lifecycle contract (Era X #2518, PR 2).
//!
//! Locks the loopback server behavior: ephemeral bind, intent dispatch with
//! applied/rejected accounting, envelope errors as HTTP 400, clean shutdown
//! via POST /shutdown, and a report that reconciles every interaction. The
//! server holds session state in memory only and writes no files.

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

fn intent(payload: PreviewIntentPayload) -> PreviewIntent {
    PreviewIntent {
        schema_version: PREVIEW_INTENT_SCHEMA_VERSION.to_string(),
        intent_id: "lifecycle-intent".to_string(),
        session_id: "lifecycle-session".to_string(),
        payload,
    }
}

#[test]
fn serve_lifecycle_round_trip() {
    let config = PreviewServerConfig::new(fixture_scene(), "lifecycle-session");
    let server = PreviewServer::bind(&config).expect("server binds on ephemeral loopback port");
    let addr = server.local_addr().unwrap();
    assert!(addr.ip().is_loopback(), "server must bind loopback only");
    let url = format!("http://{addr}");

    let serve_thread = std::thread::spawn(move || server.serve_until_shutdown());

    let health = preview_http_request(&url, "GET", "/healthz").unwrap();
    assert_eq!(health["status"], "serving");
    assert_eq!(health["sessionId"], "lifecycle-session");
    assert_eq!(health["sequence"], 0);

    let applied = preview_send_intent(
        &url,
        &intent(PreviewIntentPayload::ParameterSet {
            entity_id: "player".to_string(),
            path: "components.input.moveSpeed".to_string(),
            value: json!(9),
        }),
    )
    .unwrap();
    assert_eq!(applied["status"], "applied");
    assert_eq!(applied["sequence"], 1);
    assert!(applied["afterSceneHash"]["value"].is_string());

    let rejected = preview_send_intent(
        &url,
        &intent(PreviewIntentPayload::ParameterSet {
            entity_id: "player".to_string(),
            path: "components.transform.rotation".to_string(),
            value: json!(1),
        }),
    )
    .unwrap();
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(
        rejected["sequence"], 1,
        "rejection must not advance sequence"
    );

    // Envelope error 1: a foreign session id is a hard 400, not a delta.
    let mut foreign = intent(PreviewIntentPayload::SceneReload {});
    foreign.session_id = "another-session".to_string();
    let error = preview_send_intent(&url, &foreign).unwrap();
    assert!(error["error"]
        .as_str()
        .unwrap()
        .contains("does not match active session"));

    // Envelope error 2: malformed JSON body.
    let garbage = preview_http_request(&url, "POST", "/intent").unwrap();
    assert!(garbage["error"]
        .as_str()
        .unwrap()
        .contains("invalid preview intent"));

    let session = preview_http_request(&url, "GET", "/session").unwrap();
    assert_eq!(session["sequence"], 1);
    assert_ne!(
        session["currentSceneHash"], session["baseSceneHash"],
        "applied intent must move the in-memory hash off the base hash"
    );

    let unknown = preview_http_request(&url, "GET", "/no-such-endpoint").unwrap();
    assert!(unknown["error"]
        .as_str()
        .unwrap()
        .contains("unknown preview endpoint"));

    let stopping = preview_http_request(&url, "POST", "/shutdown").unwrap();
    assert_eq!(stopping["status"], "shutting-down");

    let report = serve_thread
        .join()
        .expect("serve thread joins")
        .expect("serve loop exits cleanly");
    assert_eq!(report.session_id, "lifecycle-session");
    assert_eq!(report.intents_applied, 1);
    assert_eq!(report.intents_rejected, 1);
    assert_eq!(report.envelope_errors, 2);
    assert_eq!(report.shutdown_reason, "shutdown-requested");
}

#[test]
fn bind_rejects_invalid_base_scene() {
    let config = PreviewServerConfig::new("/no/such/scene.json", "lifecycle-session");
    assert!(PreviewServer::bind(&config).is_err());
}

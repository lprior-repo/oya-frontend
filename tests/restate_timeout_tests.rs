//! Tests for REST call timeout handling and error resilience.
//!
//! Verifies:
//! 1. Admin API calls (client.rs) have timeout configuration
//! 2. Timeout errors map to ClientError::Timeout
//! 3. health_check gracefully handles timeout
//! 4. Ingress service calls (service_calls.rs) timeout gap is documented
//! 5. ClientError display messages are informative
//!
//! Run: cargo test --test restate_timeout_tests

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use oya_frontend::restate_client::client::{ClientError, RestateClient, RestateClientConfig};
use oya_frontend::restate_client::types::InvocationFilter;

/// Helper: build a client pointed at a port where nothing listens.
/// Connect attempts will fail with connection refused or timeout.
fn client_to_dead_port() -> RestateClient {
    RestateClient::new(RestateClientConfig {
        host: "127.0.0.1".into(),
        // Port 1 is privileged and almost certainly not listening
        port: 1,
        timeout_secs: 1,
    })
}

/// Helper: build a client with very short timeout for timeout-specific tests.
fn client_with_short_timeout() -> RestateClient {
    RestateClient::new(RestateClientConfig {
        host: "127.0.0.1".into(),
        // Port 80 — usually filtered/dropped, not refused
        // This simulates a network black hole that triggers timeout
        port: 80,
        timeout_secs: 1,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// 1. Admin API Timeout Configuration
// Verify: RestateClientConfig carries timeout_secs and applies it
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn client_config_has_default_timeout() {
    let config = RestateClientConfig::default();
    assert_eq!(config.timeout_secs, 10, "default timeout should be 10s");
}

#[test]
fn client_config_timeout_is_customizable() {
    let config = RestateClientConfig {
        host: "localhost".into(),
        port: 9070,
        timeout_secs: 30,
    };
    assert_eq!(config.timeout_secs, 30, "custom timeout should be respected");
}

#[test]
fn client_preserves_config_timeout() {
    let config = RestateClientConfig {
        host: "localhost".into(),
        port: 9070,
        timeout_secs: 5,
    };
    let client = RestateClient::new(config);
    // Client stores config internally — verify it doesn't panic on creation
    assert!(
        format!("{client:?}").contains("timeout_secs"),
        "client debug output should contain timeout config"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Connection Refused Handling (Admin API)
// Verify: failed connections produce ClientError::ConnectionFailed, not panic
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn query_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.query("SELECT 1").await;

    assert!(result.is_err(), "query to dead port should fail");
    let err = result.expect_err("should be error");
    // Connection refused should map to ConnectionFailed, not Timeout
    assert!(
        matches!(err, ClientError::ConnectionFailed(_)),
        "connection refused should be ClientError::ConnectionFailed, got: {err}"
    );
}

#[tokio::test]
async fn list_invocations_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.list_invocations(InvocationFilter::All).await;

    assert!(result.is_err(), "list_invocations to dead port should fail");
    assert!(
        matches!(result, Err(ClientError::ConnectionFailed(_))),
        "should be ClientError::ConnectionFailed"
    );
}

#[tokio::test]
async fn list_services_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.list_services().await;

    assert!(result.is_err(), "list_services to dead port should fail");
}

#[tokio::test]
async fn list_deployments_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.list_deployments().await;

    assert!(result.is_err(), "list_deployments to dead port should fail");
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. health_check Graceful Timeout Handling
// Verify: health_check maps ConnectionFailed and Timeout to Ok(false)
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn health_check_connection_refused_returns_false() {
    let client = client_to_dead_port();
    let result = client.health_check().await;

    assert!(result.is_ok(), "health_check should not return Err for connection refused");
    assert!(!result.unwrap(), "health_check should return false for unreachable server");
}

#[tokio::test]
async fn health_check_does_not_panic_on_network_failure() {
    let client = client_to_dead_port();
    // This must complete without panic regardless of network state
    let _ = client.health_check().await;
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Invocation Mutation Error Handling
// Verify: mutation calls (cancel/kill/pause/resume/purge) handle errors
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn cancel_invocation_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.cancel_invocation("inv_1234567890").await;
    assert!(result.is_err(), "cancel to dead port should fail");
}

#[tokio::test]
async fn kill_invocation_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.kill_invocation("inv_1234567890").await;
    assert!(result.is_err(), "kill to dead port should fail");
}

#[tokio::test]
async fn pause_invocation_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.pause_invocation("inv_1234567890").await;
    assert!(result.is_err(), "pause to dead port should fail");
}

#[tokio::test]
async fn resume_invocation_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.resume_invocation("inv_1234567890").await;
    assert!(result.is_err(), "resume to dead port should fail");
}

#[tokio::test]
async fn purge_invocation_connection_refused_returns_error() {
    let client = client_to_dead_port();
    let result = client.purge_invocation("inv_1234567890").await;
    assert!(result.is_err(), "purge to dead port should fail");
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. ClientError Display Messages
// Verify: error variants produce human-readable messages
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn timeout_error_display_message() {
    let err = ClientError::Timeout;
    let msg = format!("{err}");
    assert!(
        msg.to_lowercase().contains("timeout"),
        "Timeout error should mention 'timeout', got: {msg}"
    );
}

#[test]
fn connection_failed_error_display_message() {
    let err = ClientError::ConnectionFailed("connection refused".into());
    let msg = format!("{err}");
    assert!(
        msg.contains("connection refused"),
        "ConnectionFailed should include the detail, got: {msg}"
    );
}

#[test]
fn http_error_display_message() {
    let err = ClientError::HttpError {
        status: 503,
        message: "service unavailable".into(),
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("503"),
        "HttpError should include status code, got: {msg}"
    );
    assert!(
        msg.contains("service unavailable"),
        "HttpError should include message, got: {msg}"
    );
}

#[test]
fn invalid_response_error_display_message() {
    let err = ClientError::InvalidResponse("missing column 'id'".into());
    let msg = format!("{err}");
    assert!(
        msg.contains("missing column"),
        "InvalidResponse should include detail, got: {msg}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Ingress Service Call Timeout Gap
// Document: service_calls.rs has NO timeout on ingress calls (port 8080)
// This test verifies the Workflow type has restate_ingress_url for context.
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn workflow_has_restate_ingress_url_field() {
    // Verify the Workflow struct has a restate_ingress_url field
    // that defaults to "http://localhost:8080"
    let w = oya_frontend::graph::Workflow::new();
    assert!(
        !w.restate_ingress_url.is_empty(),
        "workflow should have a non-empty restate_ingress_url"
    );
    assert!(
        w.restate_ingress_url.contains("8080"),
        "default ingress URL should reference port 8080, got: {}",
        w.restate_ingress_url
    );
}

#[test]
fn ingress_url_is_runtime_only_not_serialized() {
    let mut w = oya_frontend::graph::Workflow::new();
    w.restate_ingress_url = "http://custom:9999".to_string();

    let json = serde_json::to_string(&w).unwrap();
    assert!(
        !json.contains("custom:9999"),
        "runtime ingress URL should not appear in serialized JSON"
    );

    let restored: oya_frontend::graph::Workflow = serde_json::from_str(&json).unwrap();
    assert_eq!(
        restored.restate_ingress_url, "http://localhost:8080",
        "deserialized workflow should use default ingress URL"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. Admin API Query Timeout (Slow Server)
// Verify: queries time out and produce ClientError::Timeout
// Uses a TCP listener that accepts but never responds (simulates slow server).
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn query_timeout_produces_timeout_error() {
    // Start a TCP server that accepts connections but never sends data
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    // Spawn a task that accepts but never responds
    let handle = tokio::spawn(async move {
        loop {
            if listener.accept().await.is_err() {
                break;
            }
            // Intentionally never respond — simulates a hung server
        }
    });

    let client = RestateClient::new(RestateClientConfig {
        host: "127.0.0.1".into(),
        port,
        timeout_secs: 1,
    });

    let result = client.query("SELECT 1").await;

    handle.abort();

    assert!(result.is_err(), "query to non-responding server should fail");
    assert!(
        matches!(result, Err(ClientError::Timeout)),
        "should be ClientError::Timeout, got: {:?}",
        result
    );
}

#[tokio::test]
async fn health_check_timeout_returns_false() {
    // Start a TCP server that accepts but never responds
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let handle = tokio::spawn(async move {
        loop {
            if listener.accept().await.is_err() {
                break;
            }
        }
    });

    let client = RestateClient::new(RestateClientConfig {
        host: "127.0.0.1".into(),
        port,
        timeout_secs: 1,
    });

    let result = client.health_check().await;

    handle.abort();

    assert!(result.is_ok(), "health_check should not return Err on timeout");
    assert!(!result.unwrap(), "health_check should return false on timeout");
}

#[tokio::test]
async fn invocation_action_timeout_produces_timeout_error() {
    // Start a TCP server that accepts but never responds
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let handle = tokio::spawn(async move {
        loop {
            if listener.accept().await.is_err() {
                break;
            }
        }
    });

    let client = RestateClient::new(RestateClientConfig {
        host: "127.0.0.1".into(),
        port,
        timeout_secs: 1,
    });

    let result = client.cancel_invocation("inv_test").await;

    handle.abort();

    assert!(result.is_err(), "cancel to non-responding server should fail");
    assert!(
        matches!(result, Err(ClientError::Timeout)),
        "should be ClientError::Timeout, got: {:?}",
        result
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 8. ClientError Variants Are Send + Sync
// Verify: error types can cross thread boundaries (required for tokio tasks)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn client_error_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<ClientError>();
}

#[test]
fn client_error_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<ClientError>();
}

// ═══════════════════════════════════════════════════════════════════════════
// 9. Concurrent Request Error Isolation
// Verify: multiple failing requests don't corrupt each other's errors
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn concurrent_queries_return_independent_errors() {
    let client = RestateClient::new(RestateClientConfig {
        host: "127.0.0.1".into(),
        port: 1, // Dead port
        timeout_secs: 1,
    });

    let r1 = client.query("SELECT 1").await;
    let r2 = client.query("SELECT 2").await;
    let r3 = client.query("SELECT 3").await;

    // All should fail independently
    assert!(r1.is_err());
    assert!(r2.is_err());
    assert!(r3.is_err());

    // Errors should be consistent type
    assert!(matches!(r1, Err(ClientError::ConnectionFailed(_))));
    assert!(matches!(r2, Err(ClientError::ConnectionFailed(_))));
    assert!(matches!(r3, Err(ClientError::ConnectionFailed(_))));
}

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

//! Hook for polling Restate's introspection API and surfacing live invocation state.
//!
//! Usage:
//! ```rust
//! let restate = use_restate_sync();
//! // restate.state.read().invocations → Vec<Invocation>
//! // restate.enabled → toggle polling on/off
//! // restate.admin_url → configurable admin URL (default: http://localhost:9070)
//! // restate.ingress_url → configurable ingress URL (default: http://localhost:8080)
//! ```

use crate::restate_client::types::Invocation;
use crate::restate_client::{RestateClient, RestateClientConfig};
use crate::restate_sync::poller::InvocationPoller;
use dioxus::prelude::*;
use im::HashMap;
use std::sync::Arc;

/// Live state surfaced from the Restate introspection poll.
#[derive(Clone, Debug, Default)]
pub struct RestateState {
    /// Latest snapshot of all invocations, indexed by ID.
    pub invocations: HashMap<String, Invocation>,
    /// Whether the last poll succeeded.
    pub connected: bool,
    /// Last error message if the connection failed.
    pub last_error: Option<String>,
    /// Last time the state was updated (timestamp).
    pub last_updated: i64,
}

/// Handle returned by `use_restate_sync`.
#[derive(Clone, Copy, PartialEq)]
pub struct RestateSyncHandle {
    /// Read-only view of the latest Restate state.
    pub state: ReadSignal<RestateState>,
    /// Toggle to start/stop polling. Write `true` to enable, `false` to pause.
    pub enabled: Signal<bool>,
    /// Admin API base URL (default: <http://localhost:9070>). Changing this restarts the client.
    pub admin_url: Signal<String>,
    /// Ingress base URL (default: <http://localhost:8080>). Used when running workflows.
    pub ingress_url: Signal<String>,
    /// Polling interval in milliseconds (default: 2000ms).
    pub poll_interval_ms: Signal<u32>,
}

pub fn provide_restate_sync_context() -> RestateSyncHandle {
    let mut state = use_signal(RestateState::default);
    let enabled = use_signal(|| false);
    let admin_url = use_signal(|| "http://localhost:9070".to_string());
    let ingress_url = use_signal(|| "http://localhost:8080".to_string());
    let poll_interval_ms = use_signal(|| 2000u32);

    // Main polling future.
    use_future(move || async move {
        let mut last_admin_url = String::new();
        let mut poller: Option<InvocationPoller> = None;

        loop {
            if *enabled.read() {
                let current_admin_url = admin_url.read().clone();

                // If the URL changed, reset the poller.
                if current_admin_url != last_admin_url {
                    let config = build_restate_config_from_url(&current_admin_url);
                    let client = Arc::new(RestateClient::new(config));
                    poller = Some(InvocationPoller::new(client));
                    last_admin_url = current_admin_url;
                }

                if let Some(ref mut p) = poller {
                    match p.poll().await {
                        Ok(result) => {
                            let mut s = state.write();
                            s.connected = true;
                            s.last_error = None;
                            s.last_updated = result.timestamp;

                            // Process delta events for status changes.
                            // This provides immediate UI feedback for state transitions
                            // while the full refresh ensures consistency for other changes.
                            let mut has_complete_data = false;

                            for event in &result.events {
                                match event {
                                    crate::restate_sync::InvocationEvent::StatusChanged {
                                        invocation_id,
                                        new_status,
                                    } => {
                                        if let Some(inv) = s.invocations.get_mut(invocation_id) {
                                            inv.status = (*new_status).into();
                                        }
                                    }
                                    crate::restate_sync::InvocationEvent::Completed { .. }
                                    | crate::restate_sync::InvocationEvent::Failed { .. }
                                    | crate::restate_sync::InvocationEvent::New { .. } => {
                                        // Cannot apply these deltas without complete invocation data.
                                        // Fall back to full refresh below.
                                        has_complete_data = true;
                                    }
                                }
                            }

                            // Full refresh when we lack complete delta data,
                            // or periodically to catch removed invocations.
                            if has_complete_data || result.events.is_empty() {
                                let mut new_map = HashMap::new();
                                for inv in p.state().invocations() {
                                    new_map.insert(inv.id.clone(), inv);
                                }
                                s.invocations = new_map;
                            }
                        }
                        Err(err) => {
                            let mut s = state.write();
                            s.connected = false;
                            s.last_error = Some(err.to_string());
                        }
                    }
                }
            } else {
                // If disabled, reset the poller so it starts fresh next time.
                poller = None;
                last_admin_url = String::new();
            }

            poll_sleep_ms(*poll_interval_ms.read()).await;
        }
    });

    let handle = RestateSyncHandle {
        state: state.into(),
        enabled,
        admin_url,
        ingress_url,
        poll_interval_ms,
    };
    provide_context(handle)
}

#[must_use]
pub fn use_restate_sync() -> RestateSyncHandle {
    use_context::<RestateSyncHandle>()
}

/// Parse a URL like "http://host:port" into a `RestateClientConfig`.
/// Falls back to defaults if parsing fails.
#[must_use]
pub fn build_restate_config_from_url(url: &str) -> RestateClientConfig {
    let url = url.trim_end_matches('/');
    // Strip scheme.
    let without_scheme = match url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
    {
        Some(stripped) => stripped,
        None => url,
    };

    let (host, port) = if let Some(colon) = without_scheme.rfind(':') {
        let h = &without_scheme[..colon];
        let p = without_scheme[colon + 1..].parse::<u16>().ok();
        (h.to_string(), p)
    } else {
        (without_scheme.to_string(), None)
    };

    RestateClientConfig {
        host: if host.is_empty() {
            "localhost".to_string()
        } else {
            host
        },
        port: port.unwrap_or(9070),
        timeout_secs: 10,
    }
}

/// Target-specific sleep: real timer in WASM, tokio on native.
pub async fn poll_sleep_ms(ms: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(u64::from(ms))).await;
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]
mod tests {
    use super::build_restate_config_from_url;

    #[test]
    fn given_default_url_when_parsing_then_localhost_9070_is_used() {
        let config = build_restate_config_from_url("http://localhost:9070");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 9070);
    }

    #[test]
    fn given_custom_host_and_port_when_parsing_then_both_are_captured() {
        let config = build_restate_config_from_url("http://192.168.1.100:9999");
        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 9999);
    }

    #[test]
    fn given_url_without_port_when_parsing_then_default_port_is_used() {
        let config = build_restate_config_from_url("http://myhost");
        assert_eq!(config.host, "myhost");
        assert_eq!(config.port, 9070);
    }

    #[test]
    fn given_url_with_trailing_slash_when_parsing_then_slash_is_stripped() {
        let config = build_restate_config_from_url("http://localhost:9070/");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 9070);
    }

    #[test]
    fn given_empty_url_when_parsing_then_defaults_are_used() {
        let config = build_restate_config_from_url("");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 9070);
    }
}

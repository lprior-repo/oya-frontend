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

use dioxus::prelude::*;
use oya_frontend::restate_client::{RestateClient, RestateClientConfig};
use oya_frontend::restate_client::types::{Invocation, InvocationFilter};

/// Live state surfaced from the Restate introspection poll.
#[derive(Clone, Debug, Default)]
pub struct RestateState {
    /// Latest snapshot of all invocations.
    pub invocations: Vec<Invocation>,
    /// Whether the last poll succeeded.
    pub connected: bool,
    /// Last error message if the connection failed.
    pub last_error: Option<String>,
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
}

/// Dioxus hook that polls Restate every 5 seconds while `enabled`.
///
/// Polling uses `gloo-timers` in WASM and `tokio::time::sleep` on native so
/// the future never spin-loops on either target.
pub fn use_restate_sync() -> RestateSyncHandle {
    let mut state = use_signal(RestateState::default);
    let enabled = use_signal(|| false);
    let admin_url = use_signal(|| "http://localhost:9070".to_string());
    let ingress_url = use_signal(|| "http://localhost:8080".to_string());

    let _ = use_future(move || async move {
        loop {
            if *enabled.read() {
                let url = admin_url.read().clone();
                let config = build_restate_config_from_url(&url);
                let client = RestateClient::new(config);

                match client.list_invocations(InvocationFilter::All).await {
                    Ok(invocations) => {
                        state.set(RestateState {
                            invocations,
                            connected: true,
                            last_error: None,
                        });
                    }
                    Err(err) => {
                        let mut s = state.write();
                        s.connected = false;
                        s.last_error = Some(err.to_string());
                    }
                }
            }

            poll_sleep_ms(5000).await;
        }
    });

    RestateSyncHandle {
        state: state.into(),
        enabled,
        admin_url,
        ingress_url,
    }
}

/// Parse a URL like "http://host:port" into a `RestateClientConfig`.
/// Falls back to defaults if parsing fails.
pub fn build_restate_config_from_url(url: &str) -> RestateClientConfig {
    let url = url.trim_end_matches('/');
    // Strip scheme.
    let without_scheme = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

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
async fn poll_sleep_ms(ms: u32) {
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

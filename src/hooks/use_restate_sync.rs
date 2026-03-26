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
#[derive(Clone, Copy)]
pub struct RestateSyncHandle {
    /// Read-only view of the latest Restate state.
    pub state: ReadOnlySignal<RestateState>,
    /// Toggle to start/stop polling. Write `true` to enable, `false` to pause.
    pub enabled: Signal<bool>,
}

/// Dioxus hook that polls Restate every 5 seconds while `enabled`.
///
/// Polling uses `gloo-timers` in WASM and `tokio::time::sleep` on native so
/// the future never spin-loops on either target.
pub fn use_restate_sync() -> RestateSyncHandle {
    let mut state = use_signal(RestateState::default);
    let enabled = use_signal(|| false);

    let _ = use_future(move || async move {
        let client = RestateClient::new(RestateClientConfig::default());

        loop {
            if *enabled.read() {
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

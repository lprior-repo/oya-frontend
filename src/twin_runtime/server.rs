use std::collections::HashMap;
use crate::twin_runtime::{TwinDefinition, TwinError};

pub struct TwinServerState {
    pub port: u16,
    pub twins: HashMap<String, (TwinDefinition, HashMap<String, String>)>,
}

impl TwinServerState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            port: 9001,
            twins: HashMap::new(),
        }
    }

    pub const fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Register a twin.
    /// 
    /// # Errors
    /// Returns an error if registration fails (not possible in current mock).
    pub fn register_twin(
        &mut self,
        name: String,
        definition: TwinDefinition,
        config: HashMap<String, String>,
    ) -> Result<(), TwinError> {
        let _ = self.twins.insert(name, (definition, config));
        Ok(())
    }
}

impl Default for TwinServerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Start the twin server.
/// 
/// # Errors
/// Returns an error if server fails.
pub async fn start_twin_server(port: u16) -> Result<(), TwinError> {
    println!("🚀 Twin server listening on port {port}");
    // Minimal mock implementation
    loop {
        #[cfg(target_arch = "wasm32")]
        gloo_timers::future::sleep(std::time::Duration::from_secs(3600)).await;
        #[cfg(not(target_arch = "wasm32"))]
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

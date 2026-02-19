use std::collections::HashMap;
use crate::twin_runtime::{TwinDefinition, TwinError};

pub struct TwinServerState {
    pub port: u16,
    pub twins: HashMap<String, (TwinDefinition, HashMap<String, String>)>,
}

impl TwinServerState {
    pub async fn new() -> Self {
        Self {
            port: 9001,
            twins: HashMap::new(),
        }
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub async fn register_twin(
        &mut self,
        name: String,
        definition: TwinDefinition,
        config: HashMap<String, String>,
    ) -> Result<(), TwinError> {
        self.twins.insert(name, (definition, config));
        Ok(())
    }
}

pub async fn start_twin_server(port: u16) -> Result<(), TwinError> {
    println!("🚀 Twin server listening on port {}", port);
    // Minimal mock implementation
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}

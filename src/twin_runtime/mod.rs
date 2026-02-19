use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub mod server;
pub use self::server::{TwinServerState, start_twin_server};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinConfig {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSchema {
    #[serde(default)]
    pub schema: HashMap<String, FieldSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    #[serde(default)]
    pub field_type: String,
    #[serde(default)]
    pub generated: bool,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub auto: bool,
    #[serde(default)]
    pub nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerDefinition {
    pub description: Option<String>,
    pub action: String,
    #[serde(default)]
    pub collection: Option<String>,
    #[serde(default)]
    pub response: Option<ResponseDefinition>,
    #[serde(default)]
    pub not_found: Option<ErrorResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDefinition {
    #[serde(default = "default_status")]
    pub status: u16,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

fn default_status() -> u16 {
    200
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub body: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinDefinition {
    pub twin: TwinConfig,
    #[serde(default)]
    pub state: TwinState,
    #[serde(default)]
    pub handlers: HashMap<String, HandlerDefinition>,
    #[serde(default)]
    pub inspection: HashMap<String, HandlerDefinition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwinState {
    #[serde(default)]
    pub collections: HashMap<String, Vec<serde_json::Value>>,
}

pub struct TwinInstance {
    pub definition: TwinDefinition,
    pub state: Arc<RwLock<TwinState>>,
    pub config: HashMap<String, String>,
}

impl TwinInstance {
    pub fn new(definition: TwinDefinition, config: HashMap<String, String>) -> Self {
        Self {
            definition,
            state: Arc::new(RwLock::new(TwinState::default())),
            config,
        }
    }

    pub fn handle_request(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<Response, TwinError> {
        let key = format!("{} {}", method.to_uppercase(), path);

        if let Some(handler) = self.definition.handlers.get(&key) {
            return self.execute_handler(handler, body);
        }

        if let Some(handler) = self.definition.inspection.get(&key) {
            return self.execute_handler(handler, body);
        }

        Err(TwinError::NotFound(format!(
            "No handler for {} {}",
            method, path
        )))
    }

    fn execute_handler(
        &self,
        handler: &HandlerDefinition,
        body: Option<&str>,
    ) -> Result<Response, TwinError> {
        let action = &handler.action;

        match action.as_str() {
            "create" => self.handle_create(handler, body),
            "read" => self.handle_read(handler),
            "list" => self.handle_list(handler),
            "update" => self.handle_update(handler, body),
            "delete" => self.handle_delete(handler),
            "reset_all" => self.handle_reset(),
            "health" => Ok(Response::new(200, serde_json::json!({"status": "healthy"}))),
            _ => Err(TwinError::UnknownAction(action.clone())),
        }
    }

    fn handle_create(
        &self,
        handler: &HandlerDefinition,
        body: Option<&str>,
    ) -> Result<Response, TwinError> {
        let collection_name = handler.collection.as_ref().ok_or(TwinError::ConfigError(
            "No collection specified".to_string(),
        ))?;

        let mut data = serde_json::Map::new();
        if let Some(body_str) = body {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                if let Some(obj) = parsed.as_object() {
                    for (k, v) in obj {
                        data.insert(k.clone(), v.clone());
                    }
                }
            }
        }

        data.insert(
            "id".to_string(),
            serde_json::json!(Uuid::new_v4().to_string()),
        );
        data.insert(
            "created_at".to_string(),
            serde_json::json!("2026-02-19T08:00:00Z"),
        );

        let mut state = self.state.write().map_err(|_| TwinError::LockError)?;
        let collection = state
            .collections
            .entry(collection_name.clone())
            .or_insert_with(Vec::new);
        collection.push(serde_json::Value::Object(data));

        let response_status = handler.response.as_ref().map_or(200, |r| r.status);
        Ok(Response::new(
            response_status,
            serde_json::json!({"success": true}),
        ))
    }

    fn handle_read(&self, handler: &HandlerDefinition) -> Result<Response, TwinError> {
        let collection_name = handler.collection.as_ref().ok_or(TwinError::ConfigError(
            "No collection specified".to_string(),
        ))?;

        let state = self.state.read().map_err(|_| TwinError::LockError)?;
        let collection = state.collections.get(collection_name);

        match collection {
            Some(items) if !items.is_empty() => Ok(Response::new(
                200,
                items.first().cloned().unwrap_or(serde_json::Value::Null),
            )),
            _ => {
                let error = handler
                    .not_found
                    .as_ref()
                    .ok_or(TwinError::NotFound("Item not found".to_string()))?;
                Ok(Response::new(error.status, error.body.clone()))
            }
        }
    }

    fn handle_list(&self, handler: &HandlerDefinition) -> Result<Response, TwinError> {
        let collection_name = handler.collection.as_ref().ok_or(TwinError::ConfigError(
            "No collection specified".to_string(),
        ))?;

        let state = self.state.read().map_err(|_| TwinError::LockError)?;
        let collection = state.collections.get(collection_name);

        let items = collection.cloned().unwrap_or_default();
        Ok(Response::new(
            200,
            serde_json::json!({ "items": items, "total": items.len() }),
        ))
    }

    fn handle_update(
        &self,
        handler: &HandlerDefinition,
        body: Option<&str>,
    ) -> Result<Response, TwinError> {
        let collection_name = handler.collection.as_ref().ok_or(TwinError::ConfigError(
            "No collection specified".to_string(),
        ))?;

        let mut data = serde_json::Map::new();
        if let Some(body_str) = body {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                if let Some(obj) = parsed.as_object() {
                    for (k, v) in obj {
                        data.insert(k.clone(), v.clone());
                    }
                }
            }
        }

        let mut state = self.state.write().map_err(|_| TwinError::LockError)?;
        let collection = state
            .collections
            .entry(collection_name.clone())
            .or_insert_with(Vec::new);

        if collection.is_empty() {
            let error = handler
                .not_found
                .as_ref()
                .ok_or(TwinError::NotFound("Item not found".to_string()))?;
            return Ok(Response::new(error.status, error.body.clone()));
        }

        collection.push(serde_json::Value::Object(data));

        let response_status = handler.response.as_ref().map_or(200, |r| r.status);
        Ok(Response::new(
            response_status,
            serde_json::json!({"success": true}),
        ))
    }

    fn handle_delete(&self, handler: &HandlerDefinition) -> Result<Response, TwinError> {
        let collection_name = handler.collection.as_ref().ok_or(TwinError::ConfigError(
            "No collection specified".to_string(),
        ))?;

        let mut state = self.state.write().map_err(|_| TwinError::LockError)?;
        state.collections.remove(collection_name);

        let response_status = handler.response.as_ref().map_or(200, |r| r.status);
        Ok(Response::new(
            response_status,
            serde_json::json!({"success": true}),
        ))
    }

    fn handle_reset(&self) -> Result<Response, TwinError> {
        let mut state = self.state.write().map_err(|_| TwinError::LockError)?;
        *state = TwinState::default();
        Ok(Response::new(
            200,
            serde_json::json!({"success": true, "message": "State reset"}),
        ))
    }

    pub fn get_collection(&self, name: &str) -> Result<Vec<serde_json::Value>, TwinError> {
        let state = self.state.read().map_err(|_| TwinError::LockError)?;
        Ok(state.collections.get(name).cloned().unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub body: serde_json::Value,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(status: u16, body: serde_json::Value) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status,
            body,
            headers,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionEndpoint {
    pub collection: String,
    pub items: Vec<serde_json::Value>,
}

impl TwinInstance {
    pub fn inspection_api(&self) -> HashMap<String, InspectionEndpoint> {
        let state = match self.state.read() {
            Ok(s) => s,
            Err(_) => return HashMap::new(),
        };

        let mut endpoints = HashMap::new();
        for (name, items) in &state.collections {
            endpoints.insert(
                format!("/__twin/{}", name),
                InspectionEndpoint {
                    collection: name.clone(),
                    items: items.clone(),
                },
            );
        }
        endpoints
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TwinError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Unknown action: {0}")]
    UnknownAction(String),
    #[error("Lock error")]
    LockError,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn load_twin_definition(path: &str) -> Result<TwinDefinition, TwinError> {
    let content = std::fs::read_to_string(path)?;
    let definition: TwinDefinition =
        serde_yaml::from_str(&content).map_err(|e| TwinError::ConfigError(e.to_string()))?;
    Ok(definition)
}

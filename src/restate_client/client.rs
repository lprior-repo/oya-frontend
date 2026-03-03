#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Restate HTTP client for introspection API

use crate::restate_client::queries::SqlQueries;
use crate::restate_client::types::{
    DeploymentInfo, Invocation, InvocationDetail, JournalEntry, JournalEvent, KeyedServiceStatus,
    PromiseInfo, ServiceInfo, SqlQueryResponse, StateEntry,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Request timeout")]
    Timeout,

    #[error("HTTP {status}: {message}")]
    HttpError { status: u16, message: String },

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("SQL query failed: {0}")]
    QueryFailed(String),

    #[error("JSON parse error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

/// Configuration for Restate client
#[derive(Debug, Clone)]
pub struct RestateClientConfig {
    pub host: String,
    pub port: u16,
    pub timeout_secs: u64,
}

impl Default for RestateClientConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 9070,
            timeout_secs: 10,
        }
    }
}

/// Restate SQL API client
#[derive(Debug, Clone)]
pub struct RestateClient {
    base_url: String,
    timeout_secs: u64,
}

impl RestateClient {
    pub fn new(config: RestateClientConfig) -> Self {
        let base_url = format!("http://{}:{}", config.host, config.port);
        Self {
            base_url,
            timeout_secs: config.timeout_secs,
        }
    }

    pub fn local() -> Self {
        Self::new(RestateClientConfig::default())
    }

    /// Execute a raw SQL query
    pub async fn query(&self, sql: &str) -> Result<SqlQueryResponse, ClientError> {
        let url = format!("{}/query", self.base_url);
        let body = serde_json::json!({ "query": sql });

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(self.timeout_secs))
            .build()
            .map_err(|e| ClientError::ConnectionFailed(e.to_string()))?;

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e: reqwest::Error| {
                if e.is_timeout() {
                    ClientError::Timeout
                } else {
                    ClientError::ConnectionFailed(e.to_string())
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(ClientError::HttpError {
                status: status.as_u16(),
                message,
            });
        }

        let result: SqlQueryResponse = response.json().await?;
        Ok(result)
    }

    /// List invocations
    pub async fn list_invocations(
        &self,
        include_completed: bool,
    ) -> Result<Vec<Invocation>, ClientError> {
        let sql = SqlQueries::list_invocations(include_completed);
        let response = self.query(&sql).await?;

        let invocations: Vec<Invocation> = response
            .rows
            .iter()
            .filter_map(|row| self.row_to_invocation(&response.columns, row))
            .collect();

        Ok(invocations)
    }

    /// Get single invocation detail with journal
    pub async fn get_invocation(&self, id: &str) -> Result<InvocationDetail, ClientError> {
        let inv_sql = SqlQueries::invocation(id);
        let inv_response = self.query(&inv_sql).await?;

        let invocation = inv_response
            .rows
            .first()
            .and_then(|row| self.row_to_invocation(&inv_response.columns, row))
            .ok_or_else(|| ClientError::InvalidResponse("Invocation not found".to_string()))?;

        let journal_sql = SqlQueries::journal(id);
        let journal_response = self.query(&journal_sql).await?;

        let journal: Vec<JournalEntry> = journal_response
            .rows
            .iter()
            .filter_map(|row| self.row_to_journal_entry(&journal_response.columns, row))
            .collect();

        Ok(InvocationDetail {
            invocation,
            journal,
        })
    }

    /// Get journal entries for an invocation
    pub async fn get_journal(&self, id: &str) -> Result<Vec<JournalEntry>, ClientError> {
        let sql = SqlQueries::journal(id);
        let response = self.query(&sql).await?;

        let entries: Vec<JournalEntry> = response
            .rows
            .iter()
            .filter_map(|row| self.row_to_journal_entry(&response.columns, row))
            .collect();

        Ok(entries)
    }

    /// Get journal events since index
    pub async fn get_journal_events(
        &self,
        id: &str,
        since_index: u32,
    ) -> Result<Vec<JournalEvent>, ClientError> {
        let sql = SqlQueries::journal_events_since(id, since_index);
        let response = self.query(&sql).await?;

        let events: Vec<JournalEvent> = response
            .rows
            .iter()
            .filter_map(|row| self.row_to_journal_event(&response.columns, row))
            .collect();

        Ok(events)
    }

    /// Get service state
    pub async fn get_service_state(
        &self,
        service_name: &str,
    ) -> Result<Vec<StateEntry>, ClientError> {
        let sql = SqlQueries::service_state(service_name);
        let response = self.query(&sql).await?;

        let state: Vec<StateEntry> = response
            .rows
            .iter()
            .filter_map(|row| self.row_to_state_entry(&response.columns, row))
            .collect();

        Ok(state)
    }

    /// List services
    pub async fn list_services(&self) -> Result<Vec<ServiceInfo>, ClientError> {
        let sql = SqlQueries::services();
        let response = self.query(&sql).await?;

        let services: Vec<ServiceInfo> = response
            .rows
            .iter()
            .filter_map(|row| self.row_to_service_info(&response.columns, row))
            .collect();

        Ok(services)
    }

    /// List deployments
    pub async fn list_deployments(&self) -> Result<Vec<DeploymentInfo>, ClientError> {
        let sql = SqlQueries::deployments();
        let response = self.query(&sql).await?;

        let deployments: Vec<DeploymentInfo> = response
            .rows
            .iter()
            .filter_map(|row| self.row_to_deployment_info(&response.columns, row))
            .collect();

        Ok(deployments)
    }

    /// Get keyed service status (blocking invocations)
    pub async fn get_keyed_service_status(
        &self,
    ) -> Result<Vec<KeyedServiceStatus>, ClientError> {
        let sql = SqlQueries::keyed_service_status();
        let response = self.query(&sql).await?;

        let status: Vec<KeyedServiceStatus> = response
            .rows
            .iter()
            .filter_map(|row| self.row_to_keyed_status(&response.columns, row))
            .collect();

        Ok(status)
    }

    /// Health check - try to query the API
    pub async fn health_check(&self) -> Result<bool, ClientError> {
        let sql = "SELECT 1";
        match self.query(sql).await {
            Ok(_) => Ok(true),
            Err(ClientError::ConnectionFailed(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    // Helper: Convert row to Invocation
    fn row_to_invocation(
        &self,
        columns: &[String],
        row: &[serde_json::Value],
    ) -> Option<Invocation> {
        let get = |name: &str| -> Option<&serde_json::Value> {
            let idx = columns.iter().position(|c| c == name)?;
            row.get(idx)
        };

        Some(Invocation {
            id: get("id")?.as_str()?.to_string(),
            target: get("target").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            target_service_name: get("target_service_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            target_service_key: get("target_service_key")
                .and_then(|v| v.as_str())
                .map(String::from),
            target_handler_name: get("target_handler_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            target_service_ty: get("target_service_ty")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "service" => crate::restate_client::types::ServiceType::Service,
                    "virtual_object" => crate::restate_client::types::ServiceType::VirtualObject,
                    "workflow" => crate::restate_client::types::ServiceType::Workflow,
                    _ => crate::restate_client::types::ServiceType::Service,
                })
                .unwrap_or(crate::restate_client::types::ServiceType::Service),
            status: get("status")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "pending" => crate::restate_client::types::InvocationStatus::Pending,
                    "scheduled" => crate::restate_client::types::InvocationStatus::Scheduled,
                    "ready" => crate::restate_client::types::InvocationStatus::Ready,
                    "running" => crate::restate_client::types::InvocationStatus::Running,
                    "paused" => crate::restate_client::types::InvocationStatus::Paused,
                    "backing-off" => crate::restate_client::types::InvocationStatus::BackingOff,
                    "suspended" => crate::restate_client::types::InvocationStatus::Suspended,
                    "completed" => crate::restate_client::types::InvocationStatus::Completed,
                    _ => crate::restate_client::types::InvocationStatus::Pending,
                })
                .unwrap_or(crate::restate_client::types::InvocationStatus::Pending),
            created_at: get("created_at")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            modified_at: get("modified_at")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            completed_at: get("completed_at").and_then(|v| v.as_i64()),
            journal_size: get("journal_size")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32,
            retry_count: get("retry_count")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            invoked_by: get("invoked_by")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "ingress" => crate::restate_client::types::InvokedBy::Ingress,
                    "service" => crate::restate_client::types::InvokedBy::Service,
                    "subscription" => crate::restate_client::types::InvokedBy::Subscription,
                    "restart_as_new" => crate::restate_client::types::InvokedBy::RestartAsNew,
                    _ => crate::restate_client::types::InvokedBy::Ingress,
                })
                .unwrap_or(crate::restate_client::types::InvokedBy::Ingress),
            invoked_by_service_name: get("invoked_by_service_name")
                .and_then(|v| v.as_str())
                .map(String::from),
            invoked_by_id: get("invoked_by_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            trace_id: get("trace_id").and_then(|v| v.as_str()).map(String::from),
            last_failure: get("last_failure")
                .and_then(|v| v.as_str())
                .map(String::from),
            last_failure_error_code: get("last_failure_error_code")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }

    // Helper: Convert row to JournalEntry
    fn row_to_journal_entry(
        &self,
        columns: &[String],
        row: &[serde_json::Value],
    ) -> Option<JournalEntry> {
        let get = |name: &str| -> Option<&serde_json::Value> {
            let idx = columns.iter().position(|c| c == name)?;
            row.get(idx)
        };

        Some(JournalEntry {
            id: get("id")?.as_str()?.to_string(),
            index: get("index").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            entry_type: get("entry_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            name: get("name").and_then(|v| v.as_str()).map(String::from),
            completed: get("completed").and_then(|v| v.as_bool()).unwrap_or(false),
            invoked_id: get("invoked_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            invoked_target: get("invoked_target")
                .and_then(|v| v.as_str())
                .map(String::from),
            sleep_wakeup_at: get("sleep_wakeup_at").and_then(|v| v.as_i64()),
            promise_name: get("promise_name")
                .and_then(|v| v.as_str())
                .map(String::from),
            entry_json: get("entry_json")
                .and_then(|v| v.as_str())
                .map(String::from),
            entry_lite_json: get("entry_lite_json")
                .and_then(|v| v.as_str())
                .map(String::from),
            appended_at: get("appended_at").and_then(|v| v.as_i64()),
        })
    }

    // Helper: Convert row to JournalEvent
    fn row_to_journal_event(
        &self,
        columns: &[String],
        row: &[serde_json::Value],
    ) -> Option<JournalEvent> {
        let get = |name: &str| -> Option<&serde_json::Value> {
            let idx = columns.iter().position(|c| c == name)?;
            row.get(idx)
        };

        Some(JournalEvent {
            id: get("id")?.as_str()?.to_string(),
            after_journal_entry_index: get("after_journal_entry_index")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32,
            appended_at: get("appended_at")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            event_type: get("event_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            event_json: get("event_json")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }

    // Helper: Convert row to StateEntry
    fn row_to_state_entry(
        &self,
        columns: &[String],
        row: &[serde_json::Value],
    ) -> Option<StateEntry> {
        let get = |name: &str| -> Option<&serde_json::Value> {
            let idx = columns.iter().position(|c| c == name)?;
            row.get(idx)
        };

        Some(StateEntry {
            service_name: get("service_name")?
                .as_str()
                .unwrap_or("")
                .to_string(),
            service_key: get("service_key")
                .and_then(|v| v.as_str())
                .map(String::from),
            key: get("key")?.as_str()?.to_string(),
            value_utf8: get("value_utf8")
                .and_then(|v| v.as_str())
                .map(String::from),
            value: None,
        })
    }

    // Helper: Convert row to ServiceInfo
    fn row_to_service_info(
        &self,
        columns: &[String],
        row: &[serde_json::Value],
    ) -> Option<ServiceInfo> {
        let get = |name: &str| -> Option<&serde_json::Value> {
            let idx = columns.iter().position(|c| c == name)?;
            row.get(idx)
        };

        Some(ServiceInfo {
            name: get("name")?.as_str()?.to_string(),
            ty: get("ty")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "service" => crate::restate_client::types::ServiceType::Service,
                    "virtual_object" => crate::restate_client::types::ServiceType::VirtualObject,
                    "workflow" => crate::restate_client::types::ServiceType::Workflow,
                    _ => crate::restate_client::types::ServiceType::Service,
                })
                .unwrap_or(crate::restate_client::types::ServiceType::Service),
            revision: get("revision")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            public: get("public").and_then(|v| v.as_bool()).unwrap_or(false),
            deployment_id: get("deployment_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        })
    }

    // Helper: Convert row to DeploymentInfo
    fn row_to_deployment_info(
        &self,
        columns: &[String],
        row: &[serde_json::Value],
    ) -> Option<DeploymentInfo> {
        let get = |name: &str| -> Option<&serde_json::Value> {
            let idx = columns.iter().position(|c| c == name)?;
            row.get(idx)
        };

        Some(DeploymentInfo {
            id: get("id")?.as_str()?.to_string(),
            ty: get("ty")
                .and_then(|v| v.as_str())
                .unwrap_or("http")
                .to_string(),
            endpoint: get("endpoint")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            created_at: get("created_at")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
        })
    }

    // Helper: Convert row to KeyedServiceStatus
    fn row_to_keyed_status(
        &self,
        columns: &[String],
        row: &[serde_json::Value],
    ) -> Option<KeyedServiceStatus> {
        let get = |name: &str| -> Option<&serde_json::Value> {
            let idx = columns.iter().position(|c| c == name)?;
            row.get(idx)
        };

        Some(KeyedServiceStatus {
            service_name: get("service_name")?
                .as_str()
                .unwrap_or("")
                .to_string(),
            service_key: get("service_key")?
                .as_str()
                .unwrap_or("")
                .to_string(),
            invocation_id: get("invocation_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_config_default() {
        let config = RestateClientConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 9070);
        assert_eq!(config.timeout_secs, 10);
    }

    #[test]
    fn client_config_custom() {
        let config = RestateClientConfig {
            host: "restate.example.com".to_string(),
            port: 8080,
            timeout_secs: 60,
        };
        assert_eq!(config.host, "restate.example.com");
        assert_eq!(config.port, 8080);
        assert_eq!(config.timeout_secs, 60);
    }

    #[test]
    fn client_new_creates_correct_url() {
        let client = RestateClient::new(RestateClientConfig {
            host: "192.168.1.1".to_string(),
            port: 9999,
            timeout_secs: 30,
        });
        assert!(client.base_url.contains("192.168.1.1"));
        assert!(client.base_url.contains("9999"));
        assert!(client.base_url.starts_with("http://"));
    }

    #[test]
    fn client_local_uses_defaults() {
        let client = RestateClient::local();
        assert!(client.base_url.contains("localhost"));
        assert!(client.base_url.contains("9070"));
    }

    #[test]
    fn client_timeout_is_stored() {
        let client = RestateClient::new(RestateClientConfig {
            host: "localhost".to_string(),
            port: 9070,
            timeout_secs: 5,
        });
        assert_eq!(client.timeout_secs, 5);
    }

    #[test]
    fn client_zero_timeout() {
        let client = RestateClient::new(RestateClientConfig {
            host: "localhost".to_string(),
            port: 9070,
            timeout_secs: 0,
        });
        assert_eq!(client.timeout_secs, 0);
    }

    #[test]
    fn client_max_timeout() {
        let client = RestateClient::new(RestateClientConfig {
            host: "localhost".to_string(),
            port: 9070,
            timeout_secs: u64::MAX,
        });
        assert_eq!(client.timeout_secs, u64::MAX);
    }

    #[test]
    fn client_config_debug() {
        let config = RestateClientConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("RestateClientConfig"));
    }

    #[test]
    fn client_debug() {
        let client = RestateClient::local();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("RestateClient"));
    }

    #[test]
    fn client_clone_is_independent() {
        let client1 = RestateClient::local();
        let client2 = client1.clone();
        assert_eq!(client1.base_url, client2.base_url);
        assert_eq!(client1.timeout_secs, client2.timeout_secs);
    }

    #[test]
    fn client_error_display() {
        let err = ClientError::ConnectionFailed("connection refused".to_string());
        assert!(format!("{}", err).contains("Connection failed"));
        
        let err = ClientError::Timeout;
        assert!(format!("{}", err).contains("timeout"));
        
        let err = ClientError::HttpError { status: 404, message: "Not Found".to_string() };
        assert!(format!("{}", err).contains("404"));
        
        let err = ClientError::QueryFailed("invalid query".to_string());
        assert!(format!("{}", err).contains("query failed"));
    }

    #[test]
    fn client_error_debug() {
        let err = ClientError::ConnectionFailed("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("ConnectionFailed"));
    }

    #[test]
    fn client_error_variants() {
        let _ = ClientError::ConnectionFailed("test".to_string());
        let _ = ClientError::Timeout;
        let _ = ClientError::HttpError { status: 500, message: "error".to_string() };
        let _ = ClientError::InvalidResponse("invalid".to_string());
        let _ = ClientError::QueryFailed("failed".to_string());
    }

    #[test]
    fn client_error_display_all_variants() {
        let err = ClientError::ConnectionFailed("test".to_string());
        let _ = format!("{}", err);
        
        let err = ClientError::Timeout;
        let _ = format!("{}", err);
        
        let err = ClientError::HttpError { status: 500, message: "error".to_string() };
        let _ = format!("{}", err);
        
        let err = ClientError::InvalidResponse("invalid".to_string());
        let _ = format!("{}", err);
        
        let err = ClientError::QueryFailed("failed".to_string());
        let _ = format!("{}", err);
    }

    #[test]
    fn client_config_all_ports() {
        for port in [80, 443, 8080, 9070, 9999, 65535] {
            let config = RestateClientConfig {
                host: "localhost".to_string(),
                port,
                timeout_secs: 10,
            };
            let client = RestateClient::new(config);
            assert!(client.base_url.contains(&port.to_string()));
        }
    }

    #[test]
    fn client_config_all_hosts() {
        for host in ["localhost", "127.0.0.1", "restate.local", "192.168.1.100", "::1"] {
            let config = RestateClientConfig {
                host: host.to_string(),
                port: 9070,
                timeout_secs: 10,
            };
            let client = RestateClient::new(config);
            assert!(client.base_url.contains(host));
        }
    }
}

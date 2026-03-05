#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Restate HTTP client for introspection API.

use crate::restate_client::queries::SqlQueries;
use crate::restate_client::types::{
    DeploymentInfo, Invocation, InvocationDetail, JournalEntry, JournalEvent, KeyedServiceStatus,
    ServiceInfo, SqlQueryResponse, StateEntry,
};
use serde_json::Value;
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

/// Configuration for Restate client.
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

/// Restate SQL API client.
#[derive(Debug, Clone)]
pub struct RestateClient {
    http_client: reqwest::Client,
    base_url: String,
    timeout_secs: u64,
}

impl RestateClient {
    pub fn new(config: RestateClientConfig) -> Self {
        let host = if config.host.contains(':')
            && !config.host.starts_with('[')
            && !config.host.ends_with(']')
        {
            format!("[{}]", config.host)
        } else {
            config.host
        };
        let base_url = format!("http://{host}:{}", config.port);

        Self {
            http_client: reqwest::Client::new(),
            base_url,
            timeout_secs: config.timeout_secs,
        }
    }

    pub fn local() -> Self {
        Self::new(RestateClientConfig::default())
    }

    /// Execute a raw SQL query.
    pub async fn query(&self, sql: &str) -> Result<SqlQueryResponse, ClientError> {
        let url = format!("{}/query", self.base_url);
        let body = serde_json::json!({ "query": sql });

        let response = self
            .http_client
            .post(&url)
            .timeout(std::time::Duration::from_secs(self.timeout_secs))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|error: reqwest::Error| {
                if error.is_timeout() {
                    ClientError::Timeout
                } else {
                    ClientError::ConnectionFailed(error.to_string())
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

    /// List invocations.
    pub async fn list_invocations(
        &self,
        include_completed: bool,
    ) -> Result<Vec<Invocation>, ClientError> {
        let sql = SqlQueries::list_invocations(include_completed);
        let response = self.query(&sql).await?;

        self.map_rows(
            "invocation",
            &response.columns,
            &response.rows,
            |columns, row| self.row_to_invocation(columns, row),
        )
    }

    /// Get single invocation detail with journal.
    pub async fn get_invocation(&self, id: &str) -> Result<InvocationDetail, ClientError> {
        let inv_sql = SqlQueries::invocation(id);
        let inv_response = self.query(&inv_sql).await?;

        let invocation = if let Some(row) = inv_response.rows.first() {
            self.row_to_invocation(&inv_response.columns, row)
                .map_err(|error| ClientError::InvalidResponse(format!("invocation row 0: {error}")))?
        } else {
            return Err(ClientError::InvalidResponse("Invocation not found".to_string()));
        };

        let journal_sql = SqlQueries::journal(id);
        let journal_response = self.query(&journal_sql).await?;
        let journal = self.map_rows(
            "journal",
            &journal_response.columns,
            &journal_response.rows,
            |columns, row| self.row_to_journal_entry(columns, row),
        )?;

        Ok(InvocationDetail {
            invocation,
            journal,
        })
    }

    /// Get journal entries for an invocation.
    pub async fn get_journal(&self, id: &str) -> Result<Vec<JournalEntry>, ClientError> {
        let sql = SqlQueries::journal(id);
        let response = self.query(&sql).await?;

        self.map_rows("journal", &response.columns, &response.rows, |columns, row| {
            self.row_to_journal_entry(columns, row)
        })
    }

    /// Get journal events since index.
    pub async fn get_journal_events(
        &self,
        id: &str,
        since_index: u32,
    ) -> Result<Vec<JournalEvent>, ClientError> {
        let sql = SqlQueries::journal_events_since(id, since_index);
        let response = self.query(&sql).await?;

        self.map_rows(
            "journal_event",
            &response.columns,
            &response.rows,
            |columns, row| self.row_to_journal_event(columns, row),
        )
    }

    /// Get service state.
    pub async fn get_service_state(
        &self,
        service_name: &str,
    ) -> Result<Vec<StateEntry>, ClientError> {
        let sql = SqlQueries::service_state(service_name);
        let response = self.query(&sql).await?;

        self.map_rows("state", &response.columns, &response.rows, |columns, row| {
            self.row_to_state_entry(columns, row)
        })
    }

    /// List services.
    pub async fn list_services(&self) -> Result<Vec<ServiceInfo>, ClientError> {
        let sql = SqlQueries::services();
        let response = self.query(&sql).await?;

        self.map_rows("service", &response.columns, &response.rows, |columns, row| {
            self.row_to_service_info(columns, row)
        })
    }

    /// List deployments.
    pub async fn list_deployments(&self) -> Result<Vec<DeploymentInfo>, ClientError> {
        let sql = SqlQueries::deployments();
        let response = self.query(&sql).await?;

        self.map_rows(
            "deployment",
            &response.columns,
            &response.rows,
            |columns, row| self.row_to_deployment_info(columns, row),
        )
    }

    /// Get keyed service status (blocking invocations).
    pub async fn get_keyed_service_status(
        &self,
    ) -> Result<Vec<KeyedServiceStatus>, ClientError> {
        let sql = SqlQueries::keyed_service_status();
        let response = self.query(&sql).await?;

        self.map_rows(
            "keyed_status",
            &response.columns,
            &response.rows,
            |columns, row| self.row_to_keyed_status(columns, row),
        )
    }

    /// Health check - try to query the API.
    pub async fn health_check(&self) -> Result<bool, ClientError> {
        let sql = "SELECT 1";
        match self.query(sql).await {
            Ok(_) => Ok(true),
            Err(ClientError::ConnectionFailed(_) | ClientError::Timeout) => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn map_rows<T, F>(
        &self,
        entity: &str,
        columns: &[String],
        rows: &[Vec<Value>],
        mut mapper: F,
    ) -> Result<Vec<T>, ClientError>
    where
        F: FnMut(&[String], &[Value]) -> Result<T, String>,
    {
        rows.iter()
            .enumerate()
            .map(|(index, row)| {
                mapper(columns, row).map_err(|error| {
                    ClientError::InvalidResponse(format!("{entity} row {index}: {error}"))
                })
            })
            .collect()
    }

    // Helper: Convert row to Invocation.
    fn row_to_invocation(&self, columns: &[String], row: &[Value]) -> Result<Invocation, String> {
        let target_service_ty_raw = Self::required_string(columns, row, "target_service_ty")?;
        let status_raw = Self::required_string(columns, row, "status")?;
        let invoked_by_raw = Self::required_string(columns, row, "invoked_by")?;
        let journal_size_raw = Self::required_u64(columns, row, "journal_size")?;

        Ok(Invocation {
            id: Self::required_string(columns, row, "id")?,
            target: Self::required_string(columns, row, "target")?,
            target_service_name: Self::required_string(columns, row, "target_service_name")?,
            target_service_key: Self::optional_string(columns, row, "target_service_key")?,
            target_handler_name: Self::required_string(columns, row, "target_handler_name")?,
            target_service_ty: Self::parse_service_type(&target_service_ty_raw)?,
            status: Self::parse_invocation_status(&status_raw)?,
            created_at: Self::required_i64(columns, row, "created_at")?,
            modified_at: Self::required_i64(columns, row, "modified_at")?,
            completed_at: Self::optional_i64(columns, row, "completed_at")?,
            journal_size: u32::try_from(journal_size_raw)
                .map_err(|_| format!("journal_size out of range: {journal_size_raw}"))?,
            retry_count: Self::required_u64(columns, row, "retry_count")?,
            invoked_by: Self::parse_invoked_by(&invoked_by_raw)?,
            invoked_by_service_name: Self::optional_string(columns, row, "invoked_by_service_name")?,
            invoked_by_id: Self::optional_string(columns, row, "invoked_by_id")?,
            trace_id: Self::optional_string(columns, row, "trace_id")?,
            last_failure: Self::optional_string(columns, row, "last_failure")?,
            last_failure_error_code: Self::optional_string(columns, row, "last_failure_error_code")?,
        })
    }

    // Helper: Convert row to JournalEntry.
    fn row_to_journal_entry(
        &self,
        columns: &[String],
        row: &[Value],
    ) -> Result<JournalEntry, String> {
        let index_raw = Self::required_u64(columns, row, "index")?;

        Ok(JournalEntry {
            id: Self::required_string(columns, row, "id")?,
            index: u32::try_from(index_raw)
                .map_err(|_| format!("index out of range: {index_raw}"))?,
            entry_type: Self::required_string(columns, row, "entry_type")?,
            name: Self::optional_string(columns, row, "name")?,
            completed: Self::required_bool(columns, row, "completed")?,
            invoked_id: Self::optional_string(columns, row, "invoked_id")?,
            invoked_target: Self::optional_string(columns, row, "invoked_target")?,
            sleep_wakeup_at: Self::optional_i64(columns, row, "sleep_wakeup_at")?,
            promise_name: Self::optional_string(columns, row, "promise_name")?,
            entry_json: Self::optional_string(columns, row, "entry_json")?,
            entry_lite_json: Self::optional_string(columns, row, "entry_lite_json")?,
            appended_at: Self::optional_i64(columns, row, "appended_at")?,
        })
    }

    // Helper: Convert row to JournalEvent.
    fn row_to_journal_event(
        &self,
        columns: &[String],
        row: &[Value],
    ) -> Result<JournalEvent, String> {
        let index_raw = Self::required_u64(columns, row, "after_journal_entry_index")?;

        Ok(JournalEvent {
            id: Self::required_string(columns, row, "id")?,
            after_journal_entry_index: u32::try_from(index_raw)
                .map_err(|_| format!("after_journal_entry_index out of range: {index_raw}"))?,
            appended_at: Self::required_i64(columns, row, "appended_at")?,
            event_type: Self::required_string(columns, row, "event_type")?,
            event_json: Self::optional_string(columns, row, "event_json")?,
        })
    }

    // Helper: Convert row to StateEntry.
    fn row_to_state_entry(&self, columns: &[String], row: &[Value]) -> Result<StateEntry, String> {
        Ok(StateEntry {
            service_name: Self::required_string(columns, row, "service_name")?,
            service_key: Self::optional_string(columns, row, "service_key")?,
            key: Self::required_string(columns, row, "key")?,
            value_utf8: Self::optional_string(columns, row, "value_utf8")?,
            value: Self::optional_bytes(columns, row, "value")?,
        })
    }

    // Helper: Convert row to ServiceInfo.
    fn row_to_service_info(
        &self,
        columns: &[String],
        row: &[Value],
    ) -> Result<ServiceInfo, String> {
        let ty_raw = Self::required_string(columns, row, "ty")?;

        Ok(ServiceInfo {
            name: Self::required_string(columns, row, "name")?,
            ty: Self::parse_service_type(&ty_raw)?,
            revision: Self::required_u64(columns, row, "revision")?,
            public: Self::required_bool(columns, row, "public")?,
            deployment_id: Self::required_string(columns, row, "deployment_id")?,
        })
    }

    // Helper: Convert row to DeploymentInfo.
    fn row_to_deployment_info(
        &self,
        columns: &[String],
        row: &[Value],
    ) -> Result<DeploymentInfo, String> {
        Ok(DeploymentInfo {
            id: Self::required_string(columns, row, "id")?,
            ty: Self::required_string(columns, row, "ty")?,
            endpoint: Self::required_string(columns, row, "endpoint")?,
            created_at: Self::required_i64(columns, row, "created_at")?,
        })
    }

    // Helper: Convert row to KeyedServiceStatus.
    fn row_to_keyed_status(
        &self,
        columns: &[String],
        row: &[Value],
    ) -> Result<KeyedServiceStatus, String> {
        Ok(KeyedServiceStatus {
            service_name: Self::required_string(columns, row, "service_name")?,
            service_key: Self::required_string(columns, row, "service_key")?,
            invocation_id: Self::required_string(columns, row, "invocation_id")?,
        })
    }

    fn required_value<'a>(columns: &[String], row: &'a [Value], name: &str) -> Result<&'a Value, String> {
        let index = columns
            .iter()
            .position(|column| column == name)
            .ok_or_else(|| format!("missing column '{name}'"))?;

        let value = row
            .get(index)
            .ok_or_else(|| format!("missing value for column '{name}'"))?;

        if value.is_null() {
            Err(format!("column '{name}' is NULL but required"))
        } else {
            Ok(value)
        }
    }

    fn optional_value<'a>(
        columns: &[String],
        row: &'a [Value],
        name: &str,
    ) -> Result<Option<&'a Value>, String> {
        let Some(index) = columns.iter().position(|column| column == name) else {
            return Ok(None);
        };

        let value = row
            .get(index)
            .ok_or_else(|| format!("missing value for column '{name}'"))?;

        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(value))
        }
    }

    fn required_string(columns: &[String], row: &[Value], name: &str) -> Result<String, String> {
        Self::required_value(columns, row, name)?
            .as_str()
            .map(ToString::to_string)
            .ok_or_else(|| format!("column '{name}' is not a string"))
    }

    fn optional_string(columns: &[String], row: &[Value], name: &str) -> Result<Option<String>, String> {
        let Some(value) = Self::optional_value(columns, row, name)? else {
            return Ok(None);
        };

        value
            .as_str()
            .map(ToString::to_string)
            .map(Some)
            .ok_or_else(|| format!("column '{name}' is not a string"))
    }

    fn required_u64(columns: &[String], row: &[Value], name: &str) -> Result<u64, String> {
        Self::required_value(columns, row, name)?
            .as_u64()
            .ok_or_else(|| format!("column '{name}' is not a u64"))
    }

    fn required_i64(columns: &[String], row: &[Value], name: &str) -> Result<i64, String> {
        Self::required_value(columns, row, name)?
            .as_i64()
            .ok_or_else(|| format!("column '{name}' is not an i64"))
    }

    fn optional_i64(columns: &[String], row: &[Value], name: &str) -> Result<Option<i64>, String> {
        let Some(value) = Self::optional_value(columns, row, name)? else {
            return Ok(None);
        };

        value
            .as_i64()
            .map(Some)
            .ok_or_else(|| format!("column '{name}' is not an i64"))
    }

    fn required_bool(columns: &[String], row: &[Value], name: &str) -> Result<bool, String> {
        Self::required_value(columns, row, name)?
            .as_bool()
            .ok_or_else(|| format!("column '{name}' is not a bool"))
    }

    fn optional_bytes(
        columns: &[String],
        row: &[Value],
        name: &str,
    ) -> Result<Option<Vec<u8>>, String> {
        let Some(value) = Self::optional_value(columns, row, name)? else {
            return Ok(None);
        };

        if let Some(values) = value.as_array() {
            let mut bytes = Vec::with_capacity(values.len());
            for (index, item) in values.iter().enumerate() {
                let raw = item.as_u64().ok_or_else(|| {
                    format!("column '{name}' has non-integer byte at index {index}")
                })?;
                let parsed = u8::try_from(raw).map_err(|_| {
                    format!("column '{name}' has out-of-range byte at index {index}: {raw}")
                })?;
                bytes.push(parsed);
            }
            return Ok(Some(bytes));
        }

        if let Some(text) = value.as_str() {
            return Ok(Some(text.as_bytes().to_vec()));
        }

        Err(format!(
            "column '{name}' must be an array of bytes or string value"
        ))
    }

    fn parse_service_type(raw: &str) -> Result<crate::restate_client::types::ServiceType, String> {
        match raw {
            "service" => Ok(crate::restate_client::types::ServiceType::Service),
            "virtual_object" => Ok(crate::restate_client::types::ServiceType::VirtualObject),
            "workflow" => Ok(crate::restate_client::types::ServiceType::Workflow),
            _ => Err(format!("unknown service type '{raw}'")),
        }
    }

    fn parse_invocation_status(
        raw: &str,
    ) -> Result<crate::restate_client::types::InvocationStatus, String> {
        match raw {
            "pending" => Ok(crate::restate_client::types::InvocationStatus::Pending),
            "scheduled" => Ok(crate::restate_client::types::InvocationStatus::Scheduled),
            "ready" => Ok(crate::restate_client::types::InvocationStatus::Ready),
            "running" => Ok(crate::restate_client::types::InvocationStatus::Running),
            "paused" => Ok(crate::restate_client::types::InvocationStatus::Paused),
            "backing-off" => Ok(crate::restate_client::types::InvocationStatus::BackingOff),
            "suspended" => Ok(crate::restate_client::types::InvocationStatus::Suspended),
            "completed" => Ok(crate::restate_client::types::InvocationStatus::Completed),
            _ => Err(format!("unknown invocation status '{raw}'")),
        }
    }

    fn parse_invoked_by(raw: &str) -> Result<crate::restate_client::types::InvokedBy, String> {
        match raw {
            "ingress" => Ok(crate::restate_client::types::InvokedBy::Ingress),
            "service" => Ok(crate::restate_client::types::InvokedBy::Service),
            "subscription" => Ok(crate::restate_client::types::InvokedBy::Subscription),
            "restart_as_new" => Ok(crate::restate_client::types::InvokedBy::RestartAsNew),
            _ => Err(format!("unknown invoker '{raw}'")),
        }
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
    fn client_new_wraps_ipv6_host() {
        let client = RestateClient::new(RestateClientConfig {
            host: "::1".to_string(),
            port: 9070,
            timeout_secs: 10,
        });

        assert!(client.base_url.contains("[::1]"));
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
    fn client_clone_is_independent() {
        let client1 = RestateClient::local();
        let client2 = client1.clone();
        assert_eq!(client1.base_url, client2.base_url);
        assert_eq!(client1.timeout_secs, client2.timeout_secs);
    }

    #[test]
    fn client_error_display() {
        let err = ClientError::ConnectionFailed("connection refused".to_string());
        assert!(format!("{err}").contains("Connection failed"));

        let err = ClientError::Timeout;
        assert!(format!("{err}").contains("timeout"));

        let err = ClientError::HttpError {
            status: 404,
            message: "Not Found".to_string(),
        };
        assert!(format!("{err}").contains("404"));
    }

    fn invocation_columns() -> Vec<String> {
        [
            "id",
            "target",
            "target_service_name",
            "target_service_key",
            "target_handler_name",
            "target_service_ty",
            "status",
            "created_at",
            "modified_at",
            "completed_at",
            "journal_size",
            "retry_count",
            "invoked_by",
            "invoked_by_service_name",
            "invoked_by_id",
            "trace_id",
            "last_failure",
            "last_failure_error_code",
        ]
        .iter()
        .map(|name| (*name).to_string())
        .collect()
    }

    fn invocation_row(status: &str, journal_size: u64) -> Vec<Value> {
        vec![
            Value::String("inv_123".to_string()),
            Value::String("Workflow/handler".to_string()),
            Value::String("Workflow".to_string()),
            Value::Null,
            Value::String("handler".to_string()),
            Value::String("workflow".to_string()),
            Value::String(status.to_string()),
            Value::Number(serde_json::Number::from(1_700_000_000_000_i64)),
            Value::Number(serde_json::Number::from(1_700_000_001_000_i64)),
            Value::Null,
            Value::Number(serde_json::Number::from(journal_size)),
            Value::Number(serde_json::Number::from(2_u64)),
            Value::String("service".to_string()),
            Value::String("CallerService".to_string()),
            Value::String("inv_122".to_string()),
            Value::String("trace_1".to_string()),
            Value::Null,
            Value::Null,
        ]
    }

    #[test]
    fn row_to_invocation_rejects_unknown_status() {
        let client = RestateClient::local();
        let columns = invocation_columns();
        let row = invocation_row("unknown-status", 10);

        let parsed = client.row_to_invocation(&columns, &row);
        assert!(matches!(
            parsed,
            Err(message) if message.contains("unknown invocation status")
        ));
    }

    #[test]
    fn row_to_invocation_rejects_journal_size_overflow() {
        let client = RestateClient::local();
        let columns = invocation_columns();
        let row = invocation_row("running", u64::from(u32::MAX) + 1);

        let parsed = client.row_to_invocation(&columns, &row);
        assert!(matches!(
            parsed,
            Err(message) if message.contains("journal_size out of range")
        ));
    }

    #[test]
    fn row_to_state_entry_parses_binary_value() {
        let client = RestateClient::local();
        let columns = vec![
            "service_name".to_string(),
            "service_key".to_string(),
            "key".to_string(),
            "value_utf8".to_string(),
            "value".to_string(),
        ];
        let row = vec![
            Value::String("MyService".to_string()),
            Value::String("id-1".to_string()),
            Value::String("state-key".to_string()),
            Value::Null,
            Value::Array(vec![
                Value::Number(serde_json::Number::from(0_u64)),
                Value::Number(serde_json::Number::from(127_u64)),
                Value::Number(serde_json::Number::from(255_u64)),
            ]),
        ];

        let parsed = client.row_to_state_entry(&columns, &row);
        assert!(matches!(
            parsed,
            Ok(entry) if entry.value == Some(vec![0_u8, 127_u8, 255_u8])
        ));
    }

    #[test]
    fn map_rows_reports_invalid_row_index() {
        let client = RestateClient::local();
        let columns = invocation_columns();
        let rows = vec![invocation_row("running", 1), invocation_row("invalid", 1)];

        let parsed: Result<Vec<Invocation>, ClientError> =
            client.map_rows("invocation", &columns, &rows, |row_columns, row| {
                client.row_to_invocation(row_columns, row)
            });

        assert!(matches!(
            parsed,
            Err(ClientError::InvalidResponse(message)) if message.contains("invocation row 1")
        ));
    }
}

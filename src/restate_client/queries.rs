#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! SQL queries for Restate introspection API

/// Pre-built SQL queries for Restate introspection
pub struct SqlQueries;

impl SqlQueries {
    /// List all invocations (active only by default)
    pub fn list_invocations(include_completed: bool) -> String {
        if include_completed {
            "SELECT id, target, status, created_at, modified_at, completed_at, journal_size, retry_count, invoked_by, last_failure FROM sys_invocation ORDER BY created_at DESC".to_string()
        } else {
            "SELECT id, target, status, created_at, modified_at, completed_at, journal_size, retry_count, invoked_by, last_failure FROM sys_invocation WHERE status != 'completed' ORDER BY created_at DESC".to_string()
        }
    }

    /// Get single invocation by ID
    pub fn invocation(id: &str) -> String {
        format!(
            "SELECT * FROM sys_invocation WHERE id = '{}'",
            escape_sql(id)
        )
    }

    /// Get invocation by status
    pub fn invocations_by_status(status: &str) -> String {
        format!(
            "SELECT id, target, status, created_at, modified_at, journal_size FROM sys_invocation WHERE status = '{}' ORDER BY created_at DESC",
            status
        )
    }

    /// Get journal entries for an invocation (ordered by index)
    pub fn journal(invocation_id: &str) -> String {
        format!(
            "SELECT id, index, entry_type, name, completed, invoked_id, invoked_target, sleep_wakeup_at, promise_name, entry_json, appended_at FROM sys_journal WHERE id = '{}' ORDER BY index",
            escape_sql(invocation_id)
        )
    }

    /// Get journal events since a given index
    pub fn journal_events_since(invocation_id: &str, since_index: u32) -> String {
        format!(
            "SELECT id, after_journal_entry_index, appended_at, event_type, event_json FROM sys_journal_events WHERE id = '{}' AND after_journal_entry_index > {} ORDER BY appended_at",
            escape_sql(invocation_id),
            since_index
        )
    }

    /// Get all state for a service
    pub fn service_state(service_name: &str) -> String {
        format!(
            "SELECT service_name, service_key, key, value_utf8, value FROM state WHERE service_name = '{}'",
            escape_sql(service_name)
        )
    }

    /// Get state for a specific key
    pub fn keyed_state(service_name: &str, service_key: &str) -> String {
        format!(
            "SELECT service_name, service_key, key, value_utf8, value FROM state WHERE service_name = '{}' AND service_key = '{}'",
            escape_sql(service_name),
            escape_sql(service_key)
        )
    }

    /// List all services
    pub fn services() -> String {
        "SELECT name, ty, revision, public, deployment_id FROM sys_service ORDER BY name".to_string()
    }

    /// List all deployments
    pub fn deployments() -> String {
        "SELECT id, ty, endpoint, created_at FROM sys_deployment ORDER BY created_at DESC".to_string()
    }

    /// Get keyed service status (blocking invocations)
    pub fn keyed_service_status() -> String {
        "SELECT service_name, service_key, invocation_id FROM sys_keyed_service_status".to_string()
    }

    /// Get promises for a workflow
    pub fn promises(service_name: &str, service_key: &str) -> String {
        format!(
            "SELECT service_name, service_key, key, completed, completion_success_value, completion_failure FROM sys_promise WHERE service_name = '{}' AND service_key = '{}'",
            escape_sql(service_name),
            escape_sql(service_key)
        )
    }

    /// Get invocations for a specific service
    pub fn invocations_for_service(service_name: &str) -> String {
        format!(
            "SELECT id, target, status, created_at, modified_at, journal_size FROM sys_invocation WHERE target_service_name = '{}' ORDER BY created_at DESC",
            escape_sql(service_name)
        )
    }

    /// Get invocations in backing-off status (retries)
    pub fn retrying_invocations() -> String {
        "SELECT id, target, status, retry_count, last_failure, last_failure_error_code, next_retry_at FROM sys_invocation WHERE status = 'backing-off' ORDER BY modified_at".to_string()
    }

    /// Get oldest stuck invocations
    pub fn stuck_invocations(since_hours: i64) -> String {
        format!(
            "SELECT id, target, status, modified_at, journal_size FROM sys_invocation WHERE status IN ('pending', 'scheduled', 'suspended') AND modified_at <= {} ORDER BY modified_at",
            since_hours * 3600 * 1000
        )
    }
}

/// Escape single quotes in SQL string literals
fn escape_sql(s: &str) -> String {
    s.replace('\'', "''")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_invocations_query_contains_select() {
        let q = SqlQueries::list_invocations(false);
        assert!(q.contains("SELECT"));
        assert!(q.contains("sys_invocation"));
    }

    #[test]
    fn journal_query_escapes_id() {
        let q = SqlQueries::journal("inv_123");
        assert!(q.contains("inv_123"));
    }

    #[test]
    fn service_state_query_escapes_name() {
        let q = SqlQueries::service_state("MyService");
        assert!(q.contains("MyService"));
    }

    #[test]
    fn keyed_state_escapes_both_params() {
        let q = SqlQueries::keyed_state("MyService", "my-key");
        assert!(q.contains("MyService"));
        assert!(q.contains("my-key"));
    }

    #[test]
    fn escape_sql_handles_quotes() {
        assert_eq!(escape_sql("test"), "test");
        assert_eq!(escape_sql("it's"), "it''s");
        assert_eq!(escape_sql("a'b'c"), "a''b''c");
    }

    #[test]
    fn service_state_empty_name() {
        let q = SqlQueries::service_state("");
        assert!(q.contains("service_name = ''"));
    }

    #[test]
    fn service_state_sql_injection_attempt() {
        let q = SqlQueries::service_state("'; DROP TABLE state; --");
        assert!(q.contains("''; DROP TABLE state; --"));
        assert!(!q.contains("' DROP TABLE"));
    }

    #[test]
    fn service_state_special_characters() {
        let q = SqlQueries::service_state("service'name");
        assert!(q.contains("service''name"));
    }

    #[test]
    fn service_state_unicode() {
        let q = SqlQueries::service_state("服务");
        assert!(q.contains("服务"));
    }

    #[test]
    fn keyed_state_empty_key() {
        let q = SqlQueries::keyed_state("MyService", "");
        assert!(q.contains("MyService"));
        assert!(q.contains("service_key = ''"));
    }

    #[test]
    fn stuck_invocations_zero_hours() {
        let q = SqlQueries::stuck_invocations(0);
        assert!(q.contains("modified_at <= 0"));
    }

    #[test]
    fn stuck_invocations_negative_hours() {
        let q = SqlQueries::stuck_invocations(-1);
        assert!(q.contains("modified_at <= -3600000"));
    }

    #[test]
    fn journal_query_escapes_special_chars() {
        let q = SqlQueries::journal("inv'id");
        assert!(q.contains("inv''id"));
    }

    #[test]
    fn invocation_query_empty_id() {
        let q = SqlQueries::invocation("");
        assert!(q.contains("id = ''"));
    }

    #[test]
    fn invocations_for_service_empty() {
        let q = SqlQueries::invocations_for_service("");
        assert!(q.contains("target_service_name = ''"));
    }

    #[test]
    fn promises_empty_key() {
        let q = SqlQueries::promises("Service", "");
        assert!(q.contains("Service"));
        assert!(q.contains("service_key = ''"));
    }

    #[test]
    fn journal_events_since_zero_index() {
        let q = SqlQueries::journal_events_since("inv_123", 0);
        assert!(q.contains("after_journal_entry_index > 0"));
    }

    #[test]
    fn journal_events_since_max_index() {
        let q = SqlQueries::journal_events_since("inv_123", u32::MAX);
        assert!(q.contains(&format!("after_journal_entry_index > {}", u32::MAX)));
    }
}

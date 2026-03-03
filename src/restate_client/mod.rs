//! Restate Client Module
//! 
//! Integration with Restate's introspection API for OYA Frontend.
//! 
//! API endpoints:
//! - Admin API: localhost:9070 (PATCH /invocations/*)
//! - SQL Query: localhost:9070/query (POST with SQL body)

pub mod types;
pub mod client;
pub mod queries;

pub use types::*;
pub use client::{RestateClient, RestateClientConfig, ClientError};
pub use queries::SqlQueries;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attestation {
    pub id: String,
    pub issuer: String,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
}
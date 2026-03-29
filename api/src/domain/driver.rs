use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub id: String,
    pub name: String,
    pub version: String,
    pub protocol: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub author_name: String,
    #[serde(default)]
    pub author_email: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default = "default_zero")]
    pub downloads: i64,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub reviews: Option<i32>,
    pub license: String,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub documentation: Option<String>,
    #[serde(default)]
    pub platforms: Option<serde_json::Value>,
    #[serde(default)]
    pub requirements: Option<serde_json::Value>,
    pub updated_at: DateTime<Utc>,
}

fn default_zero() -> i64 {
    0
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("invalid field: {0}")]
    InvalidField(String),
}

impl Driver {
    pub fn validate(v: &serde_json::Value) -> Result<(), ValidationError> {
        let obj = v.as_object().ok_or_else(|| ValidationError::InvalidField("not an object".into()))?;

        let required = ["id", "name", "version", "protocol", "author_name", "license", "updated_at"];
        for field in required {
            if !obj.contains_key(field) {
                return Err(ValidationError::MissingField(field.into()));
            }
        }

        // Validate id format
        let id = obj.get("id").and_then(|v| v.as_str()).unwrap();
        if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ValidationError::InvalidField(format!("invalid id format: {}", id)));
        }

        // Validate updated_at is valid ISO 8601
        let updated_at = obj.get("updated_at").and_then(|v| v.as_str()).unwrap();
        DateTime::parse_from_rfc3339(updated_at)
            .map_err(|_| ValidationError::InvalidField(format!("invalid updated_at: {}", updated_at)))?;

        Ok(())
    }
}

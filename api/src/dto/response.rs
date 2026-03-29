use serde::{Deserialize, Serialize};
use crate::domain::{Template, Driver};

// Generic API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(result: T) -> Self {
        Self { code: 0, msg: String::new(), result: Some(result) }
    }

    pub fn error(code: i32, msg: impl Into<String>) -> ApiResponse<()> {
        ApiResponse { code, msg: msg.into(), result: None }
    }
}

// Paginated list response
#[derive(Debug, Serialize)]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

impl<T> PaginatedList<T> {
    pub fn new(items: Vec<T>, total: usize, page: usize, per_page: usize) -> Self {
        Self { items, total, page, per_page }
    }
}

// Template list item (used in list response)
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateListItem {
    pub id: String,
    pub name: String,
    pub version: String,
    pub category: String,
    pub protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub author_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub license: String,
    #[serde(default)]
    pub downloads: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    pub updated_at: String,
}

impl From<Template> for TemplateListItem {
    fn from(t: Template) -> Self {
        Self {
            id: t.id,
            name: t.name,
            version: t.version,
            category: t.category,
            protocol: t.protocol,
            manufacturer: t.manufacturer,
            description: t.description,
            tags: t.tags,
            author_name: t.author_name,
            icon: t.icon,
            license: t.license,
            downloads: t.downloads,
            rating: t.rating,
            size: t.size,
            updated_at: t.updated_at.to_rfc3339(),
        }
    }
}

// Template detail response
#[derive(Debug, Serialize)]
pub struct TemplateDetail {
    pub id: String,
    pub name: String,
    pub version: String,
    pub category: String,
    pub protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub author_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub license: String,
    pub file_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    pub updated_at: String,
}

impl From<Template> for TemplateDetail {
    fn from(t: Template) -> Self {
        Self {
            id: t.id,
            name: t.name,
            version: t.version,
            category: t.category,
            protocol: t.protocol,
            manufacturer: t.manufacturer,
            description: t.description,
            tags: t.tags,
            author_name: t.author_name,
            author_email: t.author_email,
            icon: t.icon,
            license: t.license,
            file_url: t.file_url,
            checksum: t.checksum,
            readme_url: t.readme_url,
            size: t.size,
            updated_at: t.updated_at.to_rfc3339(),
        }
    }
}

// Driver list item
#[derive(Debug, Serialize, Deserialize)]
pub struct DriverListItem {
    pub id: String,
    pub name: String,
    pub version: String,
    pub protocol: String,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub author_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub license: String,
    #[serde(default)]
    pub downloads: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    pub updated_at: String,
}

impl From<Driver> for DriverListItem {
    fn from(d: Driver) -> Self {
        Self {
            id: d.id,
            name: d.name,
            version: d.version,
            protocol: d.protocol,
            description: d.description,
            tags: d.tags,
            author_name: d.author_name,
            icon: d.icon,
            license: d.license,
            downloads: d.downloads,
            rating: d.rating,
            updated_at: d.updated_at.to_rfc3339(),
        }
    }
}

// Driver detail response
#[derive(Debug, Serialize)]
pub struct DriverDetail {
    pub id: String,
    pub name: String,
    pub version: String,
    pub protocol: String,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub author_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<serde_json::Value>,
    pub updated_at: String,
}

impl From<Driver> for DriverDetail {
    fn from(d: Driver) -> Self {
        Self {
            id: d.id,
            name: d.name,
            version: d.version,
            protocol: d.protocol,
            description: d.description,
            tags: d.tags,
            author_name: d.author_name,
            author_email: d.author_email,
            icon: d.icon,
            license: d.license,
            homepage: d.homepage,
            documentation: d.documentation,
            platforms: d.platforms,
            requirements: d.requirements,
            updated_at: d.updated_at.to_rfc3339(),
        }
    }
}

// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub last_sync: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

// Repository config
#[derive(Debug, Deserialize, Clone)]
pub struct RepositoryConfig {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub repo: String,
    pub path: String,
}

use serde::Serialize;

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

// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub last_sync: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

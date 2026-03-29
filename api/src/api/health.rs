use axum::{extract::State, Json, Router};
use crate::AppState;
use crate::dto::{ApiResponse, HealthResponse};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", axum::routing::get(health_check))
}

async fn health_check(State(state): State<AppState>) -> Json<ApiResponse<HealthResponse>> {
    let last_sync = state.cache.get_last_sync().ok().flatten();

    // Check cache status
    let is_degraded = state.cache.is_cold() || last_sync.is_none();

    let status = if is_degraded {
        "degraded"
    } else {
        // Check if last sync is older than 1 hour
        if let Some(ts) = last_sync {
            let now = chrono::Utc::now().timestamp();
            if now - ts > 3600 {
                "degraded"
            } else {
                "ok"
            }
        } else {
            "ok"
        }
    };

    let response = HealthResponse {
        status: status.to_string(),
        last_sync: last_sync.map(|ts| {
            chrono::DateTime::from_timestamp(ts, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default()
        }),
        reason: if status == "degraded" {
            Some("rate_limit_exhausted".to_string())
        } else {
            None
        },
    };

    Json(ApiResponse::success(response))
}
